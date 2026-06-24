mod news;
mod tencent;
mod test_helpers;

use super::news_filter::{build_dated_news_query, merge_ranked_news};
use super::{
    CandlesWithProvider, CompanySearchContext, FundamentalsSnapshot, GeneralSearchIntent,
    MarketDataClient, NewsItem, QuoteWithProvider, SearchProviderKind, StockSearchResult,
};
use anyhow::{Context, bail};
use chrono::{Days, NaiveDate};
use regex::Regex;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashSet;
use std::time::Duration;
impl MarketDataClient {
    const HK_TENCENT_MAX_CANDLE_LIMIT: usize = 300;
    const HKEX_REQUEST_TIMEOUT_SECS: u64 = 3;
    const HK_COMPANY_SEARCH_NEWS_QUERY_LIMIT: usize = 8;
    const HK_COMPANY_SEARCH_GENERAL_QUERY_LIMIT: usize = 3;
    const HK_COMPANY_SEARCH_BATCH_SIZE: usize = 4;

    fn hk_macro_reference_pages(curr_date: &str) -> Vec<NewsItem> {
        vec![
            NewsItem {
                published_at: curr_date.to_string(),
                title: "香港市场与上市公司披露 - HKEX".to_string(),
                summary: "港股场景宏观参考页，覆盖市场公告、交易所披露与上市公司信息入口。"
                    .to_string(),
                source: "hkex.com.hk".to_string(),
                url: Some("https://www.hkex.com.hk/?sc_lang=zh-HK".to_string()),
            },
            NewsItem {
                published_at: curr_date.to_string(),
                title: "香港金融管理局市场与货币资讯 - HKMA".to_string(),
                summary: "港股场景宏观参考页，覆盖汇率、流动性与金融稳定信息。".to_string(),
                source: "hkma.gov.hk".to_string(),
                url: Some("https://www.hkma.gov.hk/chi/".to_string()),
            },
            NewsItem {
                published_at: curr_date.to_string(),
                title: "AASTOCKS 港股与恒生科技市场概览".to_string(),
                summary: "港股场景市场参考页，覆盖指数、板块与市场新闻入口。".to_string(),
                source: "aastocks.com".to_string(),
                url: Some(
                    "https://www.aastocks.com/tc/stocks/market/index/hk-index-con.aspx".to_string(),
                ),
            },
        ]
    }

    pub(super) fn hk_standard_code(&self, symbol: &str) -> anyhow::Result<String> {
        let normalized = self
            .normalize_hk_symbol(symbol)
            .context("invalid HK symbol")?;
        Ok(normalized.trim_end_matches(".HK").to_string())
    }

    pub(super) fn hk_search_aliases(&self, company_name: &str) -> Vec<String> {
        let trimmed = company_name.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }

        let mut aliases = vec![trimmed.to_string()];
        let suffixes = [
            "-W", "-SW", "-B", "-S", "－W", "－SW", "－B", "－S", "(W)", "(SW)", "(B)",
        ];

        let mut normalized = trimmed.to_string();
        for suffix in suffixes {
            if normalized.ends_with(suffix) {
                normalized = normalized.trim_end_matches(suffix).trim().to_string();
            }
        }

        if normalized != trimmed && !normalized.is_empty() {
            aliases.push(normalized);
        }

        aliases.sort();
        aliases.dedup();
        aliases
    }

    async fn hk_company_search_context(&self, standard_code: &str) -> CompanySearchContext {
        let mut items = self
            .search_stocks_from_eastmoney(standard_code, Some("港股"), 8)
            .await
            .unwrap_or_default();
        let matched = items.drain(..).find(|item| item.symbol == standard_code);
        let matched = if matched.is_some() {
            matched
        } else {
            self.search_hk_directory(standard_code, 8)
                .await
                .ok()
                .and_then(|mut rows| rows.drain(..).find(|row| row.symbol == standard_code))
        };
        let company_name = matched
            .as_ref()
            .map(|item: &StockSearchResult| item.name.clone())
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| standard_code.to_string());
        let aliases = self.hk_search_aliases(&company_name);
        CompanySearchContext { company_name, aliases }
    }

    async fn fetch_hk_main_finance_indicator(
        &self,
        symbol: &str,
    ) -> anyhow::Result<super::wire::EastmoneyMainFinanceIndicatorItem> {
        let secucode = format!("{}.HK", self.hk_standard_code(symbol)?);
        let response = self
            .http
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_HKF10_FN_MAININDICATOR"),
                ("columns", "ALL"),
                ("filter", &format!("(SECUCODE=\"{secucode}\")")),
                ("pageNumber", "1"),
                ("pageSize", "1"),
                ("sortTypes", "-1"),
                ("sortColumns", "STD_REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .context("failed to fetch HK main finance indicator from Eastmoney")?
            .error_for_status()
            .context("eastmoney HK main finance indicator request failed")?;
        let payload: super::wire::EastmoneyDatacenterEnvelope<
            super::wire::EastmoneyMainFinanceIndicatorItem,
        > = response
            .json()
            .await
            .context("failed to decode eastmoney HK main finance indicator response")?;
        payload
            .result
            .and_then(|result| result.data)
            .and_then(|mut items| items.drain(..).next())
            .context("eastmoney HK main finance indicator returned no rows")
    }

    async fn fetch_hk_income_items(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<super::wire::EastmoneyFinancialStatementItem>> {
        let secucode = format!("{}.HK", self.hk_standard_code(symbol)?);
        let response = self
            .http
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_HKF10_FN_INCOME"),
                ("columns", "ALL"),
                ("filter", &format!("(SECUCODE=\"{secucode}\")")),
                ("pageNumber", "1"),
                ("pageSize", "200"),
                ("sortTypes", "-1"),
                ("sortColumns", "STD_REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .context("failed to fetch HK income items from Eastmoney")?
            .error_for_status()
            .context("eastmoney HK income items request failed")?;
        let payload: super::wire::EastmoneyDatacenterEnvelope<
            super::wire::EastmoneyFinancialStatementItem,
        > = response
            .json()
            .await
            .context("failed to decode eastmoney HK income items response")?;
        payload
            .result
            .and_then(|result| result.data)
            .context("eastmoney HK income items returned no rows")
    }

    async fn fetch_hk_balance_items(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<super::wire::EastmoneyFinancialStatementItem>> {
        let secucode = format!("{}.HK", self.hk_standard_code(symbol)?);
        let response = self
            .http
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_HKF10_FN_BALANCE"),
                ("columns", "ALL"),
                ("filter", &format!("(SECUCODE=\"{secucode}\")")),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "STD_REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .context("failed to fetch HK balance items from Eastmoney")?
            .error_for_status()
            .context("eastmoney HK balance items request failed")?;
        let payload: super::wire::EastmoneyDatacenterEnvelope<
            super::wire::EastmoneyFinancialStatementItem,
        > = response
            .json()
            .await
            .context("failed to decode eastmoney HK balance items response")?;
        payload
            .result
            .and_then(|result| result.data)
            .context("eastmoney HK balance items returned no rows")
    }

    fn latest_hk_statement_amount(
        items: &[super::wire::EastmoneyFinancialStatementItem],
        names: &[&str],
    ) -> Option<f64> {
        items
            .iter()
            .filter(|item| {
                item.item_name
                    .as_deref()
                    .is_some_and(|name| names.contains(&name))
            })
            .max_by(|left, right| left.std_report_date.cmp(&right.std_report_date))
            .and_then(|item| item.amount)
    }

    async fn fetch_hk_cashflow_items(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<super::wire::EastmoneyFinancialStatementItem>> {
        let secucode = format!("{}.HK", self.hk_standard_code(symbol)?);
        let response = self
            .http
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_HKF10_FN_CASHFLOW"),
                ("columns", "ALL"),
                ("filter", &format!("(SECUCODE=\"{secucode}\")")),
                ("pageNumber", "1"),
                ("pageSize", "200"),
                ("sortTypes", "-1"),
                ("sortColumns", "STD_REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .context("failed to fetch HK cashflow items from Eastmoney")?
            .error_for_status()
            .context("eastmoney HK cashflow items request failed")?;
        let payload: super::wire::EastmoneyDatacenterEnvelope<
            super::wire::EastmoneyFinancialStatementItem,
        > = response
            .json()
            .await
            .context("failed to decode eastmoney HK cashflow items response")?;
        payload
            .result
            .and_then(|result| result.data)
            .context("eastmoney HK cashflow items returned no rows")
    }

    pub(super) async fn fetch_hk_quote_with_provider(
        &self,
        symbol: &str,
    ) -> anyhow::Result<QuoteWithProvider> {
        let code = self.hk_standard_code(symbol)?;
        match self.fetch_hk_tencent_quote(symbol, &code).await {
            Ok(snapshot) => Ok(QuoteWithProvider { quote: snapshot, provider: "tencent_quote".to_string() }),
            Err(primary_error) => {
                tracing::info!(
                    symbol = %symbol,
                    error = ?primary_error,
                    "Tencent HK quote failed, falling back to Yahoo Finance"
                );
                Ok(QuoteWithProvider {
                    quote: self.fetch_hk_yahoo_quote(symbol).await?,
                    provider: "yahoo_finance_chart".to_string(),
                })
            }
        }
    }

    pub(super) async fn fetch_hk_fundamentals(
        &self,
        symbol: &str,
    ) -> anyhow::Result<FundamentalsSnapshot> {
        let search_code = self.hk_standard_code(symbol)?;
        let tencent = self.fetch_hk_tencent_snapshot(&search_code).await.ok();
        let eastmoney_main = self.fetch_hk_main_finance_indicator(symbol).await.ok();
        let hk_income_items = self.fetch_hk_income_items(symbol).await.ok();
        let hk_balance_items = self.fetch_hk_balance_items(symbol).await.ok();
        let hk_cashflow_items = self.fetch_hk_cashflow_items(symbol).await.ok();
        let mut items = self
            .search_stocks_from_eastmoney(&search_code, Some("港股"), 8)
            .await?;
        let matched = items
            .drain(..)
            .find(|item| item.symbol.eq_ignore_ascii_case(&search_code));

        let company_name = matched
            .as_ref()
            .map(|item| item.name.clone())
            .filter(|value| !value.trim().is_empty())
            .or_else(|| {
                tencent
                    .as_ref()
                    .map(|item| item.name.clone())
                    .filter(|value| !value.trim().is_empty())
            })
            .unwrap_or_else(|| symbol.to_uppercase());

        let cash_and_equivalents = hk_balance_items.as_ref().and_then(|items| {
            Self::latest_hk_statement_amount(items, &["现金及等价物", "现金及现金等价物"])
        });
        let long_term_debt = hk_balance_items
            .as_ref()
            .and_then(|items| Self::latest_hk_statement_amount(items, &["长期贷款"]));
        let short_term_debt = hk_balance_items
            .as_ref()
            .and_then(|items| Self::latest_hk_statement_amount(items, &["短期贷款"]));
        let noncurrent_liabilities = hk_balance_items
            .as_ref()
            .and_then(|items| Self::latest_hk_statement_amount(items, &["非流动负债合计"]));
        let operating_income = hk_income_items
            .as_ref()
            .and_then(|items| Self::latest_hk_statement_amount(items, &["经营溢利"]));
        let operating_expenses = hk_income_items.as_ref().and_then(|items| {
            let sales = Self::latest_hk_statement_amount(items, &["销售及分销费用"]);
            let rnd = Self::latest_hk_statement_amount(items, &["研发费用"]);
            let admin = Self::latest_hk_statement_amount(items, &["管理费用", "行政费用"]);
            let direct_sum = match (sales, rnd, admin) {
                (Some(s), Some(r), Some(a)) => Some(s + r + a),
                (Some(s), Some(r), None) => Some(s + r),
                (Some(s), None, Some(a)) => Some(s + a),
                (None, Some(r), Some(a)) => Some(r + a),
                (Some(s), None, None) => Some(s),
                (None, Some(r), None) => Some(r),
                (None, None, Some(a)) => Some(a),
                (None, None, None) => None,
            };
            // Cross-validate: if we have gross_profit and operating_income,
            // derive operating_expenses from the difference as a more reliable figure.
            let gp = eastmoney_main.as_ref().and_then(|item| item.gross_profit);
            if let (Some(gp), Some(oi)) = (gp, operating_income) {
                let derived = gp - oi;
                if derived > 0.0 {
                    return Some(derived);
                }
            }
            direct_sum
        });

        let snapshot = FundamentalsSnapshot {
            symbol: symbol.to_uppercase(),
            company_name,
            cik: String::new(),
            industry: None,
            currency: eastmoney_main
                .as_ref()
                .and_then(|item| item.currency.clone())
                .or_else(|| tencent.as_ref().and_then(|item| item.currency.clone()))
                .unwrap_or_else(|| "HKD".to_string()),
            fiscal_year_end: eastmoney_main.as_ref().and_then(|item| {
                item.report_date
                    .clone()
                    .or_else(|| item.std_report_date.clone())
            }),
            shares_outstanding: eastmoney_main
                .as_ref()
                .and_then(|item| item.total_share)
                .map(|value| (value * 10_000.0).round() as i64)
                .or_else(|| tencent.as_ref().and_then(|item| item.shares_outstanding)),
            market_cap: tencent.as_ref().and_then(|item| item.market_cap_hkd),
            net_income_usd: eastmoney_main.as_ref().and_then(|item| item.holder_profit),
            revenues_usd: eastmoney_main.as_ref().and_then(|item| item.operate_income),
            assets_usd: eastmoney_main.as_ref().and_then(|item| item.total_assets),
            liabilities_usd: eastmoney_main
                .as_ref()
                .and_then(|item| item.total_liabilities),
            stockholders_equity_usd: eastmoney_main
                .as_ref()
                .and_then(|item| item.total_parent_equity),
            cash_and_equivalents_usd: cash_and_equivalents,
            gross_profit_usd: eastmoney_main.as_ref().and_then(|item| item.gross_profit),
            operating_income_usd: operating_income,
            operating_expenses_usd: operating_expenses,
            operating_cash_flow_usd: eastmoney_main
                .as_ref()
                .and_then(|item| item.netcash_operate)
                .or_else(|| {
                    hk_cashflow_items.as_ref().and_then(|items| {
                        Self::latest_hk_statement_amount(
                            items,
                            &["经营活动产生的现金流量净额", "经营业务现金流量净额"],
                        )
                    })
                }),
            capital_expenditure_usd: eastmoney_main
                .as_ref()
                .and_then(|item| item.capital_expenditure)
                .map(f64::abs)
                .or_else(|| {
                    hk_cashflow_items.as_ref().and_then(|items| {
                        // "购建固定资产、无形资产和其他长期资产支付的现金" is the
                        // standard CAPEX line on the HK cashflow statement.
                        Self::latest_hk_statement_amount(
                            items,
                            &[
                                "购建固定资产、无形资产和其他长期资产支付的现金",
                                "购买固定资产、无形资产及其他长期资产的款项",
                            ],
                        )
                    })
                }),
            free_cash_flow_usd: {
                let ocf = eastmoney_main
                    .as_ref()
                    .and_then(|item| item.netcash_operate)
                    .or_else(|| {
                        hk_cashflow_items.as_ref().and_then(|items| {
                            Self::latest_hk_statement_amount(
                                items,
                                &["经营活动产生的现金流量净额", "经营业务现金流量净额"],
                            )
                        })
                    });
                let capex = eastmoney_main
                    .as_ref()
                    .and_then(|item| item.capital_expenditure)
                    .map(f64::abs)
                    .or_else(|| {
                        hk_cashflow_items.as_ref().and_then(|items| {
                            Self::latest_hk_statement_amount(
                                items,
                                &[
                                    "购建固定资产、无形资产和其他长期资产支付的现金",
                                    "购买固定资产、无形资产及其他长期资产的款项",
                                ],
                            )
                        })
                    });
                match (ocf, capex) {
                    (Some(o), Some(c)) => Some(o - c),
                    _ => None,
                }
            },
            long_term_debt_usd: long_term_debt,
            current_debt_usd: eastmoney_main
                .as_ref()
                .and_then(|item| item.current_liability)
                .or(short_term_debt),
            total_debt_usd: eastmoney_main
                .as_ref()
                .and_then(
                    |item| match (item.current_liability, item.noncurrent_liab_1year) {
                        (Some(current), Some(noncurrent)) => Some(current + noncurrent),
                        (Some(current), None) => Some(current),
                        (None, Some(noncurrent)) => Some(noncurrent),
                        (None, None) => None,
                    },
                )
                .or_else(
                    || match (short_term_debt, long_term_debt.or(noncurrent_liabilities)) {
                        (Some(current), Some(noncurrent)) => Some(current + noncurrent),
                        (Some(current), None) => Some(current),
                        (None, Some(noncurrent)) => Some(noncurrent),
                        (None, None) => None,
                    },
                ),
            diluted_shares_outstanding: eastmoney_main
                .as_ref()
                .and_then(|item| item.total_share)
                .map(|value| (value * 10_000.0).round() as i64)
                .or_else(|| tencent.as_ref().and_then(|item| item.shares_outstanding)),
        };
        Ok(snapshot)
    }
}
impl MarketDataClient {
    pub(super) async fn fetch_hk_news(
        &self,
        symbol: &str,
        limit: usize,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> anyhow::Result<Vec<NewsItem>> {
        Ok(self
            .fetch_hk_news_diagnostics(symbol, limit, start_date, end_date)
            .await?
            .items)
    }

    pub(super) async fn fetch_hk_news_diagnostics(
        &self,
        symbol: &str,
        limit: usize,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> anyhow::Result<super::NewsFetchResult> {
        self.fetch_hk_news_diagnostics_query(symbol, limit, start_date, end_date, None)
            .await
    }

    pub(super) async fn fetch_hk_news_diagnostics_query(
        &self,
        symbol: &str,
        limit: usize,
        start_date: Option<&str>,
        end_date: Option<&str>,
        query: Option<&str>,
    ) -> anyhow::Result<super::NewsFetchResult> {
        let standard_code = self.hk_standard_code(symbol)?;
        let code = standard_code.trim_start_matches('0');
        let CompanySearchContext { company_name, aliases } = self.hk_company_search_context(&standard_code).await;
        let primary_name = aliases
            .iter()
            .find(|alias| !alias.contains("-W") && !alias.contains("-SW") && !alias.contains('－'))
            .cloned()
            .or_else(|| aliases.first().cloned())
            .unwrap_or_else(|| company_name.clone());
        let english_alias = aliases
            .iter()
            .find(|alias| alias.is_ascii() && alias.chars().any(|ch| ch.is_ascii_alphabetic()))
            .cloned()
            .unwrap_or_else(|| primary_name.clone());
        let queries = self.hk_company_news_queries(
            &standard_code,
            code,
            &company_name,
            &primary_name,
            &english_alias,
            &aliases,
            query,
            start_date,
            end_date,
        );
        let (mut merged, mut attempts) = self
            .fetch_search_evidence_with_query_locales_and_scope_mix_strategy(
                &queries,
                Some("month"),
                start_date,
                end_date,
                GeneralSearchIntent::CompanyEvidence,
                Self::HK_COMPANY_SEARCH_GENERAL_QUERY_LIMIT,
                Some(SearchProviderKind::Uapis),
                Some(Self::HK_COMPANY_SEARCH_NEWS_QUERY_LIMIT),
                Some(Self::HK_COMPANY_SEARCH_GENERAL_QUERY_LIMIT),
                Self::HK_COMPANY_SEARCH_BATCH_SIZE,
            )
            .await;
        // Fallback: when Searxng returns too few items (e.g. from China where
        // searxng can't reach upstream engines), try Bing RSS directly.
        if merged.len() < 5 {
            let existing_titles: std::collections::HashSet<String> =
                merged.iter().map(|i| i.title.to_lowercase()).collect();
            let query_refs: Vec<&str> = queries.iter().map(|s| s.as_str()).collect();
            let bing_items = self.fetch_bing_rss_news(&query_refs, 2).await;
            let mut bing_added = 0;
            for item in bing_items {
                if !existing_titles.contains(&item.title.to_lowercase()) {
                    merged.push(item);
                    bing_added += 1;
                }
            }
            if bing_added > 0 {
                attempts.push(super::NewsFetchAttempt {
                    source: "bing_rss_fallback".to_string(),
                    query: None,
                    success: true,
                    item_count: bing_added,
                    error: None,
                });
            }
        }
        // If per-symbol news is sparse, fetch macro/industry context
        if merged.len() < 10 {
            let macro_queries = vec![
                format!("{} 行业 政策", company_name),
                "港股 市场 行业".to_string(),
                "香港 经济 政策".to_string(),
            ];
            let existing_titles: std::collections::HashSet<String> =
                merged.iter().map(|i| i.title.to_lowercase()).collect();
            if let Ok((macro_items, macro_attempts)) = tokio::time::timeout(
                Duration::from_secs(8),
                self.fetch_search_evidence_with_query_locales_and_scope_mix_strategy(
                    &macro_queries,
                    Some("month"),
                    start_date,
                    end_date,
                    GeneralSearchIntent::MacroEvidence,
                    Self::HK_COMPANY_SEARCH_GENERAL_QUERY_LIMIT,
                    Some(SearchProviderKind::Uapis),
                    Some(Self::HK_COMPANY_SEARCH_NEWS_QUERY_LIMIT),
                    Some(Self::HK_COMPANY_SEARCH_GENERAL_QUERY_LIMIT),
                    Self::HK_COMPANY_SEARCH_BATCH_SIZE,
                ),
            )
            .await
            {
                for item in macro_items {
                    if !existing_titles.contains(&item.title.to_lowercase()) {
                        merged.push(item);
                    }
                }
                attempts.extend(macro_attempts);
            }
        }

        let mut hkex_window_has_items = false;
        match tokio::time::timeout(
            Duration::from_secs(Self::HKEX_REQUEST_TIMEOUT_SECS),
            self.fetch_hkex_company_announcements(
                &standard_code,
                start_date,
                end_date,
                limit.max(8),
            ),
        )
        .await
        {
            Ok(Ok(items)) if !items.is_empty() => {
                hkex_window_has_items = true;
                attempts.push(super::NewsFetchAttempt {
                    source: "HKEX Title Search".to_string(),
                    query: Some(standard_code.clone()),
                    success: true,
                    item_count: items.len(),
                    error: None,
                });
                merged.extend(items);
            }
            Ok(Ok(_)) => attempts.push(super::NewsFetchAttempt {
                source: "HKEX Title Search".to_string(),
                query: Some(standard_code.clone()),
                success: false,
                item_count: 0,
                error: Some("HKEX Title Search returned no items".to_string()),
            }),
            Ok(Err(error)) => attempts.push(super::NewsFetchAttempt {
                source: "HKEX Title Search".to_string(),
                query: Some(standard_code.clone()),
                success: false,
                item_count: 0,
                error: Some(error.to_string()),
            }),
            Err(_) => attempts.push(super::NewsFetchAttempt {
                source: "HKEX Title Search".to_string(),
                query: Some(standard_code.clone()),
                success: false,
                item_count: 0,
                error: Some(format!(
                    "HKEX Title Search timed out after {}s",
                    Self::HKEX_REQUEST_TIMEOUT_SECS
                )),
            }),
        }
        if !hkex_window_has_items {
            match tokio::time::timeout(
                Duration::from_secs(Self::HKEX_REQUEST_TIMEOUT_SECS),
                self.fetch_hkex_recent_high_value_announcements(
                    &standard_code,
                    start_date,
                    end_date,
                    limit.max(8),
                ),
            )
            .await
            {
                Ok(Ok(items)) if !items.is_empty() => {
                    tracing::info!(
                        symbol = %standard_code,
                        item_count = items.len(),
                        start_date = ?start_date,
                        end_date = ?end_date,
                        "HKEX recent high-value fallback supplied company announcements"
                    );
                    attempts.push(super::NewsFetchAttempt {
                        source: "HKEX Recent High-Value".to_string(),
                        query: Some(standard_code.clone()),
                        success: true,
                        item_count: items.len(),
                        error: None,
                    });
                    merged.extend(items);
                }
                Ok(Ok(_)) => attempts.push(super::NewsFetchAttempt {
                    source: "HKEX Recent High-Value".to_string(),
                    query: Some(standard_code.clone()),
                    success: false,
                    item_count: 0,
                    error: Some("HKEX recent high-value fallback returned no items".to_string()),
                }),
                Ok(Err(error)) => attempts.push(super::NewsFetchAttempt {
                    source: "HKEX Recent High-Value".to_string(),
                    query: Some(standard_code.clone()),
                    success: false,
                    item_count: 0,
                    error: Some(error.to_string()),
                }),
                Err(_) => attempts.push(super::NewsFetchAttempt {
                    source: "HKEX Recent High-Value".to_string(),
                    query: Some(standard_code.clone()),
                    success: false,
                    item_count: 0,
                    error: Some(format!(
                        "HKEX Recent High-Value timed out after {}s",
                        Self::HKEX_REQUEST_TIMEOUT_SECS
                    )),
                }),
            }
        }
        match tokio::time::timeout(
            Duration::from_secs(Self::HKEX_REQUEST_TIMEOUT_SECS),
            self.fetch_hk_eastmoney_announcements(&standard_code, limit.max(8)),
        )
        .await
        {
            Ok(Ok(items)) if !items.is_empty() => {
                attempts.push(super::NewsFetchAttempt {
                    source: "Eastmoney HK Announcements".to_string(),
                    query: Some(standard_code.clone()),
                    success: true,
                    item_count: items.len(),
                    error: None,
                });
                merged.extend(items);
            }
            Ok(Ok(_)) => attempts.push(super::NewsFetchAttempt {
                source: "Eastmoney HK Announcements".to_string(),
                query: Some(standard_code.clone()),
                success: false,
                item_count: 0,
                error: Some("Eastmoney HK announcements returned no items".to_string()),
            }),
            Ok(Err(error)) => attempts.push(super::NewsFetchAttempt {
                source: "Eastmoney HK Announcements".to_string(),
                query: Some(standard_code.clone()),
                success: false,
                item_count: 0,
                error: Some(error.to_string()),
            }),
            Err(_) => attempts.push(super::NewsFetchAttempt {
                source: "Eastmoney HK Announcements".to_string(),
                query: Some(standard_code.clone()),
                success: false,
                item_count: 0,
                error: Some(format!(
                    "Eastmoney HK announcements timed out after {}s",
                    Self::HKEX_REQUEST_TIMEOUT_SECS
                )),
            }),
        }
        let ranking_keywords = [
            code.to_string(),
            standard_code.clone(),
            company_name,
            primary_name,
            "业绩".to_string(),
            "公告".to_string(),
            "hkex".to_string(),
            "earnings".to_string(),
            "investor relations".to_string(),
        ];
        let mut merged = merge_ranked_news(
            merged,
            limit.max(8),
            start_date,
            end_date,
            &ranking_keywords,
        );
        merged.sort_by(|left, right| {
            let left_high_value = news::hkex_item_is_high_value(left);
            let right_high_value = news::hkex_item_is_high_value(right);
            right_high_value
                .cmp(&left_high_value)
                .then_with(|| right.published_at.cmp(&left.published_at))
                .then_with(|| left.title.cmp(&right.title))
        });
        if merged.is_empty() {
            let fallback_items = if attempts
                .iter()
                .any(|attempt| attempt.source == "HKEX Recent High-Value" && attempt.success)
            {
                self.fetch_hkex_recent_high_value_announcements(
                    &standard_code,
                    start_date,
                    end_date,
                    limit.max(8),
                )
                .await?
            } else {
                Vec::new()
            };
            merged = merge_ranked_news(
                fallback_items,
                limit.max(8),
                None,
                end_date,
                &ranking_keywords,
            );
            merged.sort_by(|left, right| {
                let left_high_value = news::hkex_item_is_high_value(left);
                let right_high_value = news::hkex_item_is_high_value(right);
                right_high_value
                    .cmp(&left_high_value)
                    .then_with(|| right.published_at.cmp(&left.published_at))
                    .then_with(|| left.title.cmp(&right.title))
            });
        }
        if merged.is_empty() {
            bail!("no HK company news available from current upstreams");
        }
        let cacheable = super::news_result_cacheable(&merged, &attempts);
        Ok(super::NewsFetchResult {
            items: merged,
            attempts,
            cacheable,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn hk_company_news_queries(
        &self,
        standard_code: &str,
        short_code: &str,
        company_name: &str,
        primary_name: &str,
        english_alias: &str,
        aliases: &[String],
        query: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Vec<String> {
        let mut seen = HashSet::new();
        let mut queries = Vec::new();
        let mut push_query = |raw: String| {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                return;
            }
            let normalized = trimmed.to_ascii_lowercase();
            if seen.insert(normalized) {
                queries.push(build_dated_news_query(trimmed, start_date, end_date));
            }
        };

        let mut base_terms = vec![
            standard_code.to_string(),
            company_name.to_string(),
            primary_name.to_string(),
            english_alias.to_string(),
        ];
        if !short_code.trim().is_empty() {
            base_terms.push(short_code.to_string());
        }
        base_terms.extend(aliases.iter().cloned());
        base_terms.sort();
        base_terms.dedup();

        if let Some(extra_query) = news::sanitize_hk_company_news_query(
            super::MarketDataClient::normalize_optional_query(query).as_deref(),
            standard_code,
            short_code,
            company_name,
            primary_name,
            english_alias,
            aliases,
        ) {
            push_query(extra_query.clone());
            for base in [company_name, primary_name, english_alias] {
                push_query(format!("{base} {extra_query}"));
            }
        }

        let priority_queries = [
            format!("{standard_code} {primary_name} 财报 公告"),
            format!("{standard_code} {english_alias} earnings"),
            format!("{standard_code} {primary_name} site:hkexnews.hk 公告 业绩"),
            format!("{standard_code} {english_alias} site:hkexnews.hk results announcement"),
            format!("{english_alias} investor relations news release"),
            format!("{english_alias} press release earnings"),
            format!("{english_alias} Reuters BusinessWire PRNewswire"),
            format!("{primary_name} 交付 销量"),
            format!("{english_alias} deliveries"),
            format!("{english_alias} guidance launch"),
            format!("{english_alias} investor relations quarterly results"),
            format!("{english_alias} annual results investor relations"),
            format!("{primary_name} 公告"),
            format!("{primary_name} 港股 最新消息"),
            format!("{primary_name} 财经 新闻"),
            format!("{english_alias} investor relations announcement"),
            format!("{english_alias} Reuters Bloomberg"),
        ];
        for query in priority_queries {
            push_query(query);
        }

        let event_terms = [
            "财报 公告",
            "业绩 公告",
            "季度业绩 公告",
            "交付 销量",
            "月度交付",
            "site:hkexnews.hk 公告 业绩",
            "site:hkexnews.hk quarterly results announcement",
            "earnings results announcement",
            "quarterly results investor relations",
            "annual results investor relations",
            "deliveries orders guidance",
            "Reuters BusinessWire PRNewswire",
            "guidance launch",
            "press release investor relations",
            "investor relations news release",
        ];
        for base in &base_terms {
            for event in event_terms {
                push_query(format!("{base} {event}"));
            }
        }

        queries.truncate(18);
        queries
    }
}

impl MarketDataClient {
    async fn fetch_hkex_company_announcements(
        &self,
        standard_code: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
        limit: usize,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let stock_id = self.fetch_hkex_stock_id(standard_code).await?;
        let default_end = chrono::Utc::now().date_naive();
        let parsed_end = end_date
            .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
            .unwrap_or(default_end);
        let parsed_start = start_date
            .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
            .unwrap_or_else(|| parsed_end - Days::new(180));
        let to = parsed_end.format("%Y%m%d").to_string();
        let from = parsed_start.format("%Y%m%d").to_string();
        let response = self
            .http
            .get("https://www1.hkexnews.hk/search/titlesearch.xhtml")
            .query(&[
                ("lang", "EN"),
                ("category", "0"),
                ("market", "SEHK"),
                ("searchType", "0"),
                ("documentType", "-2"),
                ("t1code", "-2"),
                ("t2Gcode", "-2"),
                ("t2code", "-2"),
                ("stockId", &stock_id),
                ("from", from.as_str()),
                ("to", to.as_str()),
            ])
            .send()
            .await
            .context("failed to query HKEX title search")?
            .error_for_status()
            .context("HKEX title search query failed")?
            .text()
            .await
            .context("failed to read HKEX title search results")?;
        Ok(news::parse_hkex_title_search_results(&response)
            .into_iter()
            .filter(|item| {
                super::news_filter::within_date_window(&item.published_at, start_date, end_date)
            })
            .take(limit)
            .collect())
    }

    async fn fetch_hkex_recent_high_value_announcements(
        &self,
        standard_code: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
        limit: usize,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let Some(window_end) = end_date else {
            return Ok(Vec::new());
        };
        let end = NaiveDate::parse_from_str(window_end, "%Y-%m-%d")
            .context("invalid end_date for HKEX recent high-value fallback")?;
        let fallback_start = end - Days::new(180);
        let min_window_start =
            start_date.and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok());
        let items = self
            .fetch_hkex_company_announcements(
                standard_code,
                Some(&fallback_start.to_string()),
                end_date,
                limit.saturating_mul(4).max(24),
            )
            .await?;
        tracing::info!(
            symbol = %standard_code,
            start_date = ?start_date,
            end_date = ?end_date,
            fallback_start = %fallback_start,
            fetched_count = items.len(),
            "HKEX recent high-value fallback fetched raw announcements"
        );
        let selected = items
            .into_iter()
            .filter(news::hkex_item_is_high_value)
            .filter(|item| {
                let published = item
                    .published_at
                    .split_whitespace()
                    .next()
                    .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok());
                match (published, min_window_start) {
                    (Some(published), Some(start)) => published >= start - Days::new(45),
                    _ => true,
                }
            })
            .take(limit)
            .collect::<Vec<_>>();
        tracing::info!(
            symbol = %standard_code,
            selected_count = selected.len(),
            titles = ?selected.iter().map(|item| item.title.clone()).collect::<Vec<_>>(),
            "HKEX recent high-value fallback selected announcements"
        );
        Ok(selected)
    }

    async fn fetch_hkex_stock_id(&self, standard_code: &str) -> anyhow::Result<String> {
        let payload = self
            .http
            .get("https://www1.hkexnews.hk/search/prefix.do")
            .query(&[
                ("lang", "EN"),
                ("type", "A"),
                ("name", standard_code),
                ("market", "SEHK"),
                ("callback", "callback"),
            ])
            .send()
            .await
            .context("failed to fetch HKEX stock id")?
            .error_for_status()
            .context("HKEX stock id request failed")?
            .text()
            .await
            .context("failed to read HKEX stock id response")?;
        let regex =
            Regex::new(r#""stockId":\s*([0-9]+)\s*,\s*"code":"([0-9]{5})""#).expect("valid regex");
        let captures = regex
            .captures(&payload)
            .context("HKEX stock id response missing stock id")?;
        let code = captures
            .get(2)
            .map(|value| value.as_str())
            .unwrap_or_default();
        if code != standard_code {
            bail!("HKEX stock id response mismatched stock code");
        }
        captures
            .get(1)
            .map(|value| value.as_str().to_string())
            .context("HKEX stock id response missing stock id capture")
    }

    async fn fetch_hk_eastmoney_announcements(
        &self,
        standard_code: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let page_size = limit.to_string();
        let response = self
            .http
            .get("https://np-anotice-stock.eastmoney.com/api/security/ann")
            .query(&[
                ("page_size", page_size.as_str()),
                ("page_index", "1"),
                ("ann_type", "H"),
                ("client_source", "web"),
                ("stock_list", standard_code),
            ])
            .send()
            .await
            .context("failed to fetch Eastmoney HK announcements")?
            .error_for_status()
            .context("eastmoney HK announcements request failed")?;
        let payload: super::wire::EastmoneyAnnouncementsEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney HK announcements response")?;
        let items = payload
            .data
            .and_then(|data| data.list)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|item| {
                let title = item.title?;
                if title.trim().is_empty() {
                    return None;
                }
                Some(NewsItem {
                    published_at: item.notice_date.unwrap_or_default(),
                    title: title.clone(),
                    summary: title,
                    source: "Eastmoney 公告".to_string(),
                    url: item.art_code.map(|art_code| {
                        format!(
                            "https://data.eastmoney.com/notices/detail/{standard_code}/{art_code}.html"
                        )
                    }),
                })
            })
            .take(limit)
            .collect::<Vec<_>>();
        Ok(items)
    }

    pub(super) async fn fetch_hk_global_news_diagnostics(
        &self,
        curr_date: &str,
        look_back_days: usize,
        limit: usize,
    ) -> anyhow::Result<super::NewsFetchResult> {
        let end = NaiveDate::parse_from_str(curr_date, "%Y-%m-%d")
            .context("invalid curr_date for HK global news")?;
        let start = end - chrono::Days::new(look_back_days as u64);
        let queries = vec![
            build_dated_news_query(
                "香港市场 中国互联网 宏观",
                Some(&start.to_string()),
                Some(curr_date),
            ),
            build_dated_news_query(
                "Hang Seng Tech China policy liquidity",
                Some(&start.to_string()),
                Some(curr_date),
            ),
            build_dated_news_query(
                "Hong Kong stocks China policy outlook",
                Some(&start.to_string()),
                Some(curr_date),
            ),
            build_dated_news_query(
                "RMB Hong Kong equities liquidity",
                Some(&start.to_string()),
                Some(curr_date),
            ),
            build_dated_news_query(
                "China EV market policy sentiment",
                Some(&start.to_string()),
                Some(curr_date),
            ),
            build_dated_news_query(
                "港股 恒生指数 资金流 北水",
                Some(&start.to_string()),
                Some(curr_date),
            ),
            build_dated_news_query(
                "China tech regulation antitrust internet",
                Some(&start.to_string()),
                Some(curr_date),
            ),
            build_dated_news_query(
                "Hong Kong IPO market listing activity",
                Some(&start.to_string()),
                Some(curr_date),
            ),
        ];
        let (items, attempts) = self
            .fetch_search_evidence_with_query_locales_and_scope_mix(
                &queries,
                Some("month"),
                Some(&start.to_string()),
                Some(curr_date),
                GeneralSearchIntent::MacroEvidence,
                4,
            )
            .await;
        let merged = merge_ranked_news(
            items,
            limit,
            Some(&start.to_string()),
            Some(curr_date),
            &[
                "香港市场".to_string(),
                "恒生科技".to_string(),
                "中国互联网".to_string(),
                "人民币".to_string(),
            ],
        );
        let (merged, attempts, cacheable) = if merged.is_empty() {
            let fallback_items = Self::hk_macro_reference_pages(curr_date);
            let mut attempts = attempts;
            attempts.push(super::NewsFetchAttempt {
                source: "HK Macro Reference".to_string(),
                query: Some(curr_date.to_string()),
                success: true,
                item_count: fallback_items.len(),
                error: None,
            });
            (fallback_items, attempts, false)
        } else {
            let cacheable = super::news_result_cacheable(&merged, &attempts);
            (merged, attempts, cacheable)
        };
        Ok(super::NewsFetchResult {
            items: merged,
            attempts,
            cacheable,
        })
    }

    pub(super) async fn fetch_hk_candles(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<super::CandlePoint>> {
        self.fetch_hk_candles_with_provider(symbol, limit)
            .await
            .map(|r| r.candles)
    }

    pub(super) async fn fetch_hk_candles_with_provider(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<CandlesWithProvider> {
        match self.fetch_hk_tencent_candles(symbol, limit).await {
            Ok(items) => return Ok(CandlesWithProvider { candles: items, provider: "tencent_kline".to_string() }),
            Err(primary_error) => {
                tracing::info!(
                    symbol = %symbol,
                    error = ?primary_error,
                    "Tencent HK candles failed, falling back to Yahoo Finance"
                );
            }
        }
        Ok(CandlesWithProvider {
            candles: self.fetch_hk_yahoo_candles(symbol, limit).await?,
            provider: "yahoo_finance_chart".to_string(),
        })
    }
}
impl MarketDataClient {
    pub(super) async fn fetch_hk_return_since(
        &self,
        symbol: &str,
        start_date: &str,
        holding_days: usize,
    ) -> anyhow::Result<Option<f64>> {
        let candles = self.fetch_hk_candles(symbol, holding_days + 15).await?;
        let mut items = candles
            .into_iter()
            .map(|item| (item.trade_date, item.close))
            .collect::<Vec<_>>();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        let Some(start_index) = items.iter().position(|(date, _)| date == start_date) else {
            return Ok(None);
        };
        let end_index = (start_index + holding_days).min(items.len().saturating_sub(1));
        if end_index <= start_index {
            return Ok(None);
        }
        let start_price = items[start_index].1;
        let end_price = items[end_index].1;
        if start_price <= 0.0 {
            return Ok(None);
        }
        Ok(Some(
            ((end_price - start_price) / start_price)
                .to_f64()
                .unwrap_or_default(),
        ))
    }
}
