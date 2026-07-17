mod announcements;
mod candles;
mod capital_flow;
mod sector;

use std::collections::HashSet;

use anyhow::{Context, bail};

use super::search::market_to_eastmoney_label;
use super::{
    CapitalFlowPoint, FundamentalsSnapshot, MarketDataClient, NewsItem, QuoteSnapshot,
    QuoteWithProvider, StockSearchResult,
    wire::{AkshareIndividualInfo, EastmoneyAnnouncementsEnvelope, EastmoneySearchEnvelope},
};

impl MarketDataClient {
    const EASTMONEY_SEARCH_TIMEOUT_SECS: u64 = 3;

    pub(super) async fn search_stocks_from_eastmoney(
        &self,
        query: &str,
        market: Option<&str>,
        limit: usize,
    ) -> anyhow::Result<Vec<StockSearchResult>> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            bail!("search query is empty");
        }

        let response = tokio::time::timeout(
            std::time::Duration::from_secs(Self::EASTMONEY_SEARCH_TIMEOUT_SECS),
            self.http
                .get("https://searchapi.eastmoney.com/api/suggest/get")
                .query(&[
                    ("input", trimmed),
                    ("type", "14"),
                    ("token", "D43BF722C8E33BDC906FB84D85E326E8"),
                    ("count", &limit.clamp(1, 20).to_string()),
                ])
                .send(),
        )
        .await
        .with_context(|| {
            format!(
                "eastmoney stock search timed out after {}s",
                Self::EASTMONEY_SEARCH_TIMEOUT_SECS
            )
        })?
        .context("failed to search stocks from Eastmoney")?
        .error_for_status()
        .context("eastmoney search request failed")?;

        let payload: EastmoneySearchEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney search response")?;

        let items = payload
            .quotation_code_table
            .and_then(|table| table.data)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|item| {
                let symbol = item.code?;
                let name = item.name?;
                let exchange = item.exchange.unwrap_or_default();
                let market_name =
                    match (item.classify.as_deref(), item.security_type_name.as_deref()) {
                        (Some("AStock"), _) => "A股",
                        (Some("Fund") | Some("OTCFUND"), _) => "A股",
                        (_, Some("基金")) => "A股",
                        (Some("Index"), _) => "指数",
                        (Some("BStock"), _) => "A股",
                        (Some("NEEQ"), _) => "A股",
                        (Some("UsStock"), _) => "美股",
                        (Some("HK"), _) => "港股",
                        _ => return None,
                    };
                if let Some(expected_market) = market
                    && market_to_eastmoney_label(expected_market) != market_name
                {
                    return None;
                }
                Some(StockSearchResult {
                    symbol,
                    name,
                    market: market_name.to_string(),
                    exchange,
                })
            })
            .take(limit)
            .collect::<Vec<_>>();

        Ok(items)
    }

    /// Fallback search: try to look up a code directly via the eastmoney kline API.
    /// This handles codes (like certain indices) that the suggest API doesn't cover.
    /// Tries market IDs 0 (深), 1 (沪), 2 (中证), 47 (北交所).
    pub(super) async fn search_eastmoney_direct_lookup(
        &self,
        code: &str,
    ) -> Option<StockSearchResult> {
        let trimmed = code.trim();
        if trimmed.is_empty() || !trimmed.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        let market_ids: &[u32] = &[1, 0, 2, 47];
        for &market_id in market_ids {
            let secid = format!("{}.{}", market_id, trimmed);
            let result = tokio::time::timeout(
                std::time::Duration::from_secs(2),
                self.http
                    .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
                    .query(&[
                        ("secid", secid.as_str()),
                        ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
                        ("fields1", "f1,f2,f3,f4,f5,f6"),
                        ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58"),
                        ("klt", "101"),
                        ("fqt", "0"),
                        ("beg", "0"),
                        ("end", "20500000"),
                    ])
                    .send(),
            )
            .await;
            let Ok(Ok(response)) = result else {
                continue;
            };
            let Ok(payload) = response.json::<serde_json::Value>().await else {
                continue;
            };
            let data = payload.get("data")?;
            let name = data.get("name")?.as_str()?;
            let code_str = data.get("code")?.as_str()?;
            if name.is_empty() || code_str.is_empty() {
                continue;
            }
            let market_name = match market_id {
                0 => "A股",
                1 => "A股",
                2 => "指数",
                47 => "A股",
                _ => "A股",
            };
            return Some(StockSearchResult {
                symbol: code_str.to_string(),
                name: name.to_string(),
                market: market_name.to_string(),
                exchange: String::new(),
            });
        }
        None
    }

    pub(super) async fn fetch_a_share_quote_with_provider(
        &self,
        symbol: &str,
        ts_code: &str,
    ) -> anyhow::Result<QuoteWithProvider> {
        match self.fetch_a_share_quote_from_eastmoney(symbol).await {
            Ok(quote) => Ok(QuoteWithProvider {
                quote,
                provider: "tencent_quote".to_string(),
            }),
            Err(eastmoney_error) => {
                tracing::info!(
                    "tencent quote unavailable for {} ({}), falling back to tushare",
                    symbol,
                    eastmoney_error
                );
                match self.fetch_a_share_quote_from_tushare(symbol, ts_code).await {
                    Ok(quote) => Ok(QuoteWithProvider {
                        quote,
                        provider: "tushare_daily".to_string(),
                    }),
                    Err(tushare_error) => Err(tushare_error),
                }
            }
        }
    }

    pub(crate) async fn fetch_a_share_quote_from_tushare(
        &self,
        symbol: &str,
        ts_code: &str,
    ) -> anyhow::Result<QuoteSnapshot> {
        let today = chrono::Utc::now().format("%Y%m%d").to_string();
        let start_date = (chrono::Utc::now() - chrono::Duration::days(14))
            .format("%Y%m%d")
            .to_string();
        let rows = self
            .tushare_query(
                "daily",
                serde_json::json!({
                    "ts_code": ts_code,
                    "start_date": start_date,
                    "end_date": today
                }),
                "ts_code,trade_date,open,high,low,close,vol,amount",
            )
            .await?;
        let row = rows.first().context("tushare daily returned no rows")?;
        Ok(QuoteSnapshot {
            symbol: symbol.trim().to_uppercase(),
            date: row.string("trade_date")?,
            open: row.f64("open")?,
            high: row.f64("high")?,
            low: row.f64("low")?,
            close: row.f64("close")?,
            volume: (row.f64("vol")? * 100.0).round() as i64,
        })
    }

    pub(crate) async fn fetch_a_share_quote_from_eastmoney(
        &self,
        symbol: &str,
    ) -> anyhow::Result<QuoteSnapshot> {
        let market_symbol = self.tencent_market_symbol(symbol)?;
        let response = self
            .http
            .get("https://qt.gtimg.cn/q")
            .query(&[("q", market_symbol.as_str())])
            .send()
            .await
            .context("failed to fetch A-share quote from Tencent")?
            .error_for_status()
            .context("tencent quote request failed")?
            .text()
            .await
            .context("failed to read tencent quote response")?;
        Self::parse_tencent_quote(symbol, &response)
    }

    fn parse_tencent_quote(_symbol: &str, raw: &str) -> anyhow::Result<QuoteSnapshot> {
        let fields = Self::parse_tencent_quote_fields(raw)?;
        if fields.len() < 38 {
            bail!("unexpected tencent quote field count");
        }
        let date = fields[30]
            .get(0..8)
            .context("missing tencent trade date")?
            .to_string();
        Ok(QuoteSnapshot {
            symbol: fields[2].trim().to_string(),
            date,
            open: fields[5].parse().context("invalid tencent quote open")?,
            high: fields[33].parse().context("invalid tencent quote high")?,
            low: fields[34].parse().context("invalid tencent quote low")?,
            close: fields[3].parse().context("invalid tencent quote close")?,
            volume: fields[6].parse().context("invalid tencent quote volume")?,
        })
    }

    fn parse_tencent_quote_market_cap(raw: &str) -> anyhow::Result<Option<f64>> {
        let fields = Self::parse_tencent_quote_fields(raw)?;
        Ok(fields
            .get(45)
            .and_then(|value| value.parse::<f64>().ok())
            .map(|value| value * 100_000_000.0))
    }

    fn parse_tencent_quote_fields(raw: &str) -> anyhow::Result<Vec<&str>> {
        let payload = raw
            .split_once('"')
            .and_then(|(_, rest)| rest.rsplit_once('"').map(|(body, _)| body))
            .context("unexpected tencent quote response format")?;
        Ok(payload.split('~').collect::<Vec<_>>())
    }
}
impl MarketDataClient {
    pub(crate) fn a_share_fiscal_year_end_candidate(value: Option<String>) -> Option<String> {
        let raw = value?;
        let digits = raw
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .collect::<String>();
        if digits.len() < 8 || &digits[4..8] != "1231" {
            return None;
        }
        Some(format!(
            "{}-{}-{}",
            &digits[0..4],
            &digits[4..6],
            &digits[6..8]
        ))
    }

    fn a_share_macro_reference_pages(curr_date: &str) -> Vec<NewsItem> {
        vec![
            NewsItem {
                published_at: curr_date.to_string(),
                title: "中国宏观与政策跟踪 - 国家统计局".to_string(),
                summary: "A股场景宏观参考页，覆盖经济数据、公报与统计发布。".to_string(),
                source: "stats.gov.cn".to_string(),
                url: Some("https://www.stats.gov.cn/".to_string()),
            },
            NewsItem {
                published_at: curr_date.to_string(),
                title: "中国货币政策与金融数据 - 中国人民银行".to_string(),
                summary: "A股场景宏观参考页，覆盖利率、流动性与金融统计。".to_string(),
                source: "pbc.gov.cn".to_string(),
                url: Some("http://www.pbc.gov.cn/".to_string()),
            },
            NewsItem {
                published_at: curr_date.to_string(),
                title: "A股市场总览 - 东方财富".to_string(),
                summary: "A股市场场景参考页，覆盖指数、板块、资金面与市场新闻入口。".to_string(),
                source: "eastmoney.com".to_string(),
                url: Some("https://www.eastmoney.com/".to_string()),
            },
        ]
    }

    async fn fetch_eastmoney_stock_profile(
        &self,
        secucode: &str,
    ) -> anyhow::Result<Option<String>> {
        let response = self
            .http
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_F10_ORG_BASICINFO"),
                ("columns", "SECUCODE,SECURITY_NAME_ABBR,INDUSTRYCSRC1"),
                ("filter", &format!("(SECUCODE=\"{secucode}\")")),
                ("pageNumber", "1"),
                ("pageSize", "1"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .context("failed to fetch Eastmoney stock profile")?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let payload: serde_json::Value = response
            .json()
            .await
            .context("failed to decode eastmoney stock profile response")?;

        let industry = payload
            .get("result")
            .and_then(|r| r.get("data"))
            .and_then(|d| d.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("INDUSTRYCSRC1"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(industry)
    }

    async fn fetch_eastmoney_main_finance_indicator(
        &self,
        secucode: &str,
    ) -> anyhow::Result<super::wire::EastmoneyMainFinanceIndicatorItem> {
        let response = self
            .http
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_F10_FINANCE_MAINFINADATA"),
                ("columns", "ALL"),
                ("filter", &format!("(SECUCODE=\"{secucode}\")")),
                ("pageNumber", "1"),
                ("pageSize", "1"),
                ("sortTypes", "-1"),
                ("sortColumns", "REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .context("failed to fetch Eastmoney main finance indicator")?
            .error_for_status()
            .context("eastmoney main finance indicator request failed")?;
        let payload: super::wire::EastmoneyDatacenterEnvelope<
            super::wire::EastmoneyMainFinanceIndicatorItem,
        > = response
            .json()
            .await
            .context("failed to decode eastmoney main finance indicator response")?;
        payload
            .result
            .and_then(|result| result.data)
            .and_then(|mut items| items.drain(..).next())
            .context("eastmoney main finance indicator returned no rows")
    }

    async fn fetch_eastmoney_balance_sheet(
        &self,
        secucode: &str,
    ) -> anyhow::Result<super::wire::EastmoneyBalanceSheetItem> {
        let response = self
            .http
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_DMSK_FN_BALANCE"),
                ("columns", "ALL"),
                ("filter", &format!("(SECUCODE=\"{secucode}\")")),
                ("pageNumber", "1"),
                ("pageSize", "1"),
                ("sortTypes", "-1"),
                ("sortColumns", "REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .context("failed to fetch Eastmoney balance sheet")?
            .error_for_status()
            .context("eastmoney balance sheet request failed")?;
        let payload: super::wire::EastmoneyDatacenterEnvelope<
            super::wire::EastmoneyBalanceSheetItem,
        > = response
            .json()
            .await
            .context("failed to decode eastmoney balance sheet response")?;
        payload
            .result
            .and_then(|result| result.data)
            .and_then(|mut items| items.drain(..).next())
            .context("eastmoney balance sheet returned no rows")
    }

    async fn fetch_eastmoney_cashflow(
        &self,
        secucode: &str,
    ) -> anyhow::Result<super::wire::EastmoneyCashflowItem> {
        let response = self
            .http
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_DMSK_FN_CASHFLOW"),
                ("columns", "ALL"),
                ("filter", &format!("(SECUCODE=\"{secucode}\")")),
                ("pageNumber", "1"),
                ("pageSize", "1"),
                ("sortTypes", "-1"),
                ("sortColumns", "REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .context("failed to fetch Eastmoney cashflow")?
            .error_for_status()
            .context("eastmoney cashflow request failed")?;
        let payload: super::wire::EastmoneyDatacenterEnvelope<super::wire::EastmoneyCashflowItem> =
            response
                .json()
                .await
                .context("failed to decode eastmoney cashflow response")?;
        payload
            .result
            .and_then(|result| result.data)
            .and_then(|mut items| items.drain(..).next())
            .context("eastmoney cashflow returned no rows")
    }

    pub(super) async fn fetch_a_share_fundamentals(
        &self,
        symbol: &str,
        ts_code: &str,
    ) -> anyhow::Result<FundamentalsSnapshot> {
        let tushare_available = self.tushare_token.is_some();
        if !tushare_available {
            tracing::warn!(
                symbol = %symbol,
                ts_code = %ts_code,
                "TUSHARE_TOKEN not set; tushare data sources will be unavailable for A-share fundamentals"
            );
        }

        let basic_rows = match self
            .tushare_query(
                "stock_basic",
                serde_json::json!({ "ts_code": ts_code, "list_status": "L" }),
                "ts_code,symbol,name,industry,list_date",
            )
            .await
        {
            Ok(rows) => rows,
            Err(e) => {
                tracing::warn!(symbol = %symbol, error = %e, "tushare stock_basic query failed");
                vec![]
            }
        };
        let basic = basic_rows.first();
        let mut search_items = self
            .search_stocks_from_eastmoney(symbol.trim(), Some("A股"), 8)
            .await
            .unwrap_or_default();
        let search_match = search_items
            .drain(..)
            .find(|item| item.symbol == symbol.trim());
        let info = self.fetch_a_share_individual_info(symbol).await.ok();
        let eastmoney_industry = self
            .fetch_eastmoney_stock_profile(ts_code)
            .await
            .inspect_err(|e| {
                tracing::warn!(symbol = %symbol, error = %e, "eastmoney stock profile failed");
            })
            .ok()
            .flatten();
        let quote = self.fetch_a_share_quote_from_eastmoney(symbol).await.ok();
        let quote_market_cap = self
            .fetch_a_share_tencent_market_cap(symbol)
            .await
            .ok()
            .flatten();
        let eastmoney_main = self
            .fetch_eastmoney_main_finance_indicator(ts_code)
            .await
            .inspect_err(|e| {
                tracing::warn!(symbol = %symbol, error = %e, "eastmoney main finance indicator failed");
            })
            .ok();
        let eastmoney_balance = self
            .fetch_eastmoney_balance_sheet(ts_code)
            .await
            .inspect_err(|e| {
                tracing::warn!(symbol = %symbol, error = %e, "eastmoney balance sheet failed");
            })
            .ok();
        let eastmoney_cashflow = self
            .fetch_eastmoney_cashflow(ts_code)
            .await
            .inspect_err(|e| {
                tracing::warn!(symbol = %symbol, error = %e, "eastmoney cashflow failed");
            })
            .ok();

        let today = chrono::Utc::now().format("%Y%m%d").to_string();
        let recent_start = (chrono::Utc::now() - chrono::Duration::days(30))
            .format("%Y%m%d")
            .to_string();
        let daily_basic_rows = match self
            .tushare_query(
                "daily_basic",
                serde_json::json!({
                    "ts_code": ts_code,
                    "start_date": recent_start,
                    "end_date": today
                }),
                "ts_code,trade_date,total_share,float_share,total_mv,circ_mv,pe,pb",
            )
            .await
        {
            Ok(rows) => rows,
            Err(e) => {
                tracing::warn!(symbol = %symbol, error = %e, "tushare daily_basic query failed");
                vec![]
            }
        };
        let daily_basic = daily_basic_rows.first();

        let income_rows = match self
            .tushare_query(
                "income",
                serde_json::json!({ "ts_code": ts_code }),
                "ts_code,ann_date,end_date,total_revenue,revenue,n_income,n_income_attr_p",
            )
            .await
        {
            Ok(rows) => rows,
            Err(e) => {
                tracing::warn!(symbol = %symbol, error = %e, "tushare income query failed");
                vec![]
            }
        };
        // Prefer annual reports (end_date ends in 1231) to avoid quarterly income skewing PE
        let income = income_rows
            .iter()
            .find(|row| {
                row.optional_string("end_date")
                    .map(|d| d.ends_with("1231"))
                    .unwrap_or(false)
            })
            .or_else(|| income_rows.first());

        let balance_rows = match self
            .tushare_query(
                "balancesheet",
                serde_json::json!({ "ts_code": ts_code }),
                "ts_code,ann_date,end_date,total_assets,total_liab,total_hldr_eqy_exc_min_int,total_share,money_cap,lt_borr,st_borr",
            )
            .await
        {
            Ok(rows) => rows,
            Err(e) => {
                tracing::warn!(symbol = %symbol, error = %e, "tushare balancesheet query failed");
                vec![]
            }
        };
        let balance = balance_rows
            .iter()
            .find(|row| {
                row.optional_string("end_date")
                    .map(|d| d.ends_with("1231"))
                    .unwrap_or(false)
            })
            .or_else(|| balance_rows.first());

        let cashflow_rows = match self
            .tushare_query(
                "cashflow",
                serde_json::json!({ "ts_code": ts_code }),
                "ts_code,ann_date,end_date,n_cashflow_act,free_cashflow",
            )
            .await
        {
            Ok(rows) => rows,
            Err(e) => {
                tracing::warn!(symbol = %symbol, error = %e, "tushare cashflow query failed");
                vec![]
            }
        };
        let cashflow = cashflow_rows
            .iter()
            .find(|row| {
                row.optional_string("end_date")
                    .map(|d| d.ends_with("1231"))
                    .unwrap_or(false)
            })
            .or_else(|| cashflow_rows.first());

        let fina_indicator_rows = match self
            .tushare_query(
                "fina_indicator",
                serde_json::json!({ "ts_code": ts_code }),
                "ts_code,ann_date,end_date,total_profit,op_of_gr,profit_dedt,ocfps,fcff",
            )
            .await
        {
            Ok(rows) => rows,
            Err(e) => {
                tracing::warn!(symbol = %symbol, error = %e, "tushare fina_indicator query failed");
                vec![]
            }
        };
        let fina_indicator = fina_indicator_rows
            .iter()
            .find(|row| {
                row.optional_string("end_date")
                    .map(|d| d.ends_with("1231"))
                    .unwrap_or(false)
            })
            .or_else(|| fina_indicator_rows.first());

        let fiscal_year_end = Self::a_share_fiscal_year_end_candidate(
            income
                .and_then(|row| row.optional_string("end_date"))
                .or_else(|| balance.and_then(|row| row.optional_string("end_date")))
                .or_else(|| {
                    eastmoney_main.as_ref().and_then(|row| {
                        row.report_date
                            .clone()
                            .or_else(|| row.std_report_date.clone())
                    })
                }),
        );
        let provisional_shares_outstanding = daily_basic
            .and_then(|row| row.optional_f64("total_share"))
            .map(|value| (value * 10_000.0).round() as i64)
            .or_else(|| {
                balance
                    .and_then(|row| row.optional_f64("total_share"))
                    .map(|value| (value * 10_000.0).round() as i64)
            })
            .or_else(|| {
                eastmoney_main
                    .as_ref()
                    .and_then(|item| item.total_share)
                    .map(|value| value.round() as i64)
            })
            .or_else(|| info.as_ref().and_then(|value| value.total_share));
        let provisional_market_cap: Option<f64> = daily_basic
            .and_then(|row| row.optional_f64("total_mv"))
            .map(|v| v * 10_000.0)
            .or_else(|| info.as_ref().and_then(|value| value.market_cap))
            .or(quote_market_cap)
            .or_else(|| {
                let shares = provisional_shares_outstanding?;
                let quote = quote.as_ref()?;
                Some(quote.close * shares as f64)
            });
        let shares_outstanding = provisional_shares_outstanding.or_else(|| {
            let market_cap = provisional_market_cap?;
            let price = quote.as_ref()?.close;
            if price > 0.0 {
                Some((market_cap / price).round() as i64)
            } else {
                None
            }
        });
        let eastmoney_equity: Option<f64> = eastmoney_balance
            .as_ref()
            .and_then(|item| item.total_equity)
            .or_else(|| {
                eastmoney_main.as_ref().and_then(|item| {
                    let shares = item.total_share?;
                    let bps = item.bps?;
                    Some(shares * bps)
                })
            });
        let eastmoney_assets: Option<f64> = eastmoney_balance
            .as_ref()
            .and_then(|item| item.total_assets)
            .or_else(|| {
                eastmoney_main.as_ref().and_then(|item| {
                    let equity = eastmoney_equity?;
                    let debt_ratio_pct = item.zcfzl?;
                    let equity_ratio = 1.0 - (debt_ratio_pct / 100.0);
                    (equity_ratio > 0.0).then_some(equity / equity_ratio)
                })
            });
        let eastmoney_liabilities: Option<f64> = eastmoney_balance
            .as_ref()
            .and_then(|item| item.total_liabilities)
            .or_else(|| {
                eastmoney_assets
                    .zip(eastmoney_equity)
                    .map(|(assets, equity)| assets - equity)
            });

        let snapshot = FundamentalsSnapshot {
            symbol: symbol.trim().to_uppercase(),
            company_name: info
                .as_ref()
                .and_then(|value| value.stock_name.clone())
                .or_else(|| basic.and_then(|row| row.optional_string("name")))
                .or_else(|| {
                    eastmoney_main
                        .as_ref()
                        .and_then(|item| item.security_name_abbr.clone())
                })
                .or_else(|| search_match.as_ref().map(|item| item.name.clone()))
                .unwrap_or_else(|| symbol.to_string()),
            cik: ts_code.to_string(),
            industry: info
                .as_ref()
                .and_then(|value| value.industry.clone())
                .or_else(|| basic.and_then(|row| row.optional_string("industry")))
                .or_else(|| eastmoney_industry.clone()),
            currency: eastmoney_main
                .as_ref()
                .and_then(|item| item.currency.clone())
                .unwrap_or_else(|| "CNY".to_string()),
            fiscal_year_end,
            shares_outstanding,
            market_cap: provisional_market_cap,
            net_income_usd: income
                .and_then(|row| {
                    row.optional_f64("n_income_attr_p")
                        .or_else(|| row.optional_f64("n_income"))
                })
                .or_else(|| {
                    eastmoney_main
                        .as_ref()
                        .and_then(|item| item.parent_net_profit.or(item.holder_profit))
                }),
            revenues_usd: income
                .and_then(|row| {
                    row.optional_f64("total_revenue")
                        .or_else(|| row.optional_f64("revenue"))
                })
                .or_else(|| {
                    eastmoney_main
                        .as_ref()
                        .and_then(|item| item.total_operate_reve.or(item.operate_income))
                }),
            assets_usd: balance
                .and_then(|row| row.optional_f64("total_assets"))
                .or_else(|| {
                    eastmoney_main
                        .as_ref()
                        .and_then(|item| item.totalassets.or(item.total_assets))
                })
                .or(eastmoney_assets),
            liabilities_usd: balance
                .and_then(|row| row.optional_f64("total_liab"))
                .or_else(|| {
                    eastmoney_main
                        .as_ref()
                        .and_then(|item| item.totliab.or(item.total_liabilities))
                })
                .or(eastmoney_liabilities),
            stockholders_equity_usd: balance
                .and_then(|row| row.optional_f64("total_hldr_eqy_exc_min_int"))
                .or_else(|| {
                    eastmoney_main
                        .as_ref()
                        .and_then(|item| item.total_parent_equity)
                })
                .or(eastmoney_equity),
            cash_and_equivalents_usd: balance
                .and_then(|row| row.optional_f64("money_cap"))
                .or_else(|| {
                    eastmoney_balance
                        .as_ref()
                        .and_then(|item| item.monetary_funds)
                })
                .or_else(|| eastmoney_cashflow.as_ref().and_then(|item| item.end_cce)),
            gross_profit_usd: eastmoney_main
                .as_ref()
                .and_then(|item| item.gross_profit.or(item.mlr)),
            operating_income_usd: fina_indicator.and_then(|row| row.optional_f64("op_of_gr")),
            operating_expenses_usd: None,
            operating_cash_flow_usd: cashflow
                .and_then(|row| row.optional_f64("n_cashflow_act"))
                .or_else(|| {
                    eastmoney_main.as_ref().and_then(|item| {
                        item.netcash_operate.or(item.mgjyxjje.map(|per_share| {
                            item.total_share
                                .map(|shares| per_share * shares)
                                .unwrap_or(per_share)
                        }))
                    })
                })
                .or_else(|| {
                    eastmoney_cashflow
                        .as_ref()
                        .and_then(|item| item.netcash_operate)
                }),
            capital_expenditure_usd: eastmoney_main
                .as_ref()
                .and_then(|item| item.capital_expenditure)
                .map(f64::abs)
                .or_else(|| {
                    eastmoney_cashflow
                        .as_ref()
                        .and_then(|item| item.construct_long_asset)
                        .map(f64::abs)
                }),
            free_cash_flow_usd: cashflow
                .and_then(|row| row.optional_f64("free_cashflow"))
                .or_else(|| fina_indicator.and_then(|row| row.optional_f64("fcff")))
                .or_else(|| {
                    eastmoney_main.as_ref().and_then(|item| {
                        match (item.netcash_operate, item.capital_expenditure.map(f64::abs)) {
                            (Some(ocf), Some(capex)) => Some(ocf - capex),
                            _ => None,
                        }
                    })
                })
                .or_else(|| {
                    eastmoney_cashflow.as_ref().and_then(|item| {
                        match (
                            item.netcash_operate,
                            item.construct_long_asset.map(f64::abs),
                        ) {
                            (Some(ocf), Some(capex)) => Some(ocf - capex),
                            _ => None,
                        }
                    })
                }),
            long_term_debt_usd: balance.and_then(|row| row.optional_f64("lt_borr")),
            current_debt_usd: balance
                .and_then(|row| row.optional_f64("st_borr"))
                .or_else(|| {
                    eastmoney_main
                        .as_ref()
                        .and_then(|item| item.current_liab.or(item.current_liability))
                })
                .or_else(|| {
                    eastmoney_balance
                        .as_ref()
                        .and_then(|item| item.current_liab)
                }),
            total_debt_usd: balance
                .and_then(|row| {
                    let current = row.optional_f64("st_borr");
                    let long_term = row.optional_f64("lt_borr");
                    match (current, long_term) {
                        (Some(current), Some(long_term)) => Some(current + long_term),
                        (Some(current), None) => Some(current),
                        (None, Some(long_term)) => Some(long_term),
                        (None, None) => None,
                    }
                })
                .or_else(|| {
                    eastmoney_main.as_ref().and_then(|item| {
                        match (
                            item.current_liab.or(item.current_liability),
                            item.totalnoncliab.or(item.noncurrent_liab_1year),
                        ) {
                            (Some(current), Some(noncurrent)) => Some(current + noncurrent),
                            (Some(current), None) => Some(current),
                            (None, Some(noncurrent)) => Some(noncurrent),
                            (None, None) => None,
                        }
                    })
                })
                .or_else(|| {
                    eastmoney_balance.as_ref().and_then(|item| {
                        match (item.current_liab, item.totalnoncliab) {
                            (Some(current), Some(noncurrent)) => Some(current + noncurrent),
                            (Some(current), None) => Some(current),
                            (None, Some(noncurrent)) => Some(noncurrent),
                            (None, None) => None,
                        }
                    })
                }),
            diluted_shares_outstanding: None,
        };

        // Log data completeness for diagnostics
        let filled = [
            snapshot.company_name != symbol,
            snapshot.industry.is_some(),
            snapshot.shares_outstanding.is_some(),
            snapshot.market_cap.is_some(),
            snapshot.net_income_usd.is_some(),
            snapshot.revenues_usd.is_some(),
            snapshot.assets_usd.is_some(),
            snapshot.liabilities_usd.is_some(),
            snapshot.stockholders_equity_usd.is_some(),
            snapshot.cash_and_equivalents_usd.is_some(),
            snapshot.gross_profit_usd.is_some(),
            snapshot.operating_cash_flow_usd.is_some(),
            snapshot.free_cash_flow_usd.is_some(),
        ];
        let filled_count = filled.iter().filter(|&&v| v).count();
        let total = filled.len();
        tracing::info!(
            symbol = %symbol,
            ts_code = %ts_code,
            tushare_available = tushare_available,
            data_filled = filled_count,
            data_total = total,
            completeness_pct = (filled_count * 100 / total),
            eastmoney_main = eastmoney_main.is_some(),
            eastmoney_balance = eastmoney_balance.is_some(),
            eastmoney_cashflow = eastmoney_cashflow.is_some(),
            eastmoney_industry = eastmoney_industry.is_some(),
            "A-share fundamentals fetch complete"
        );
        if filled_count < total / 2 {
            tracing::warn!(
                symbol = %symbol,
                data_filled = filled_count,
                data_total = total,
                "A-share fundamentals data is less than 50% complete; analysis quality will be degraded"
            );
        }

        Ok(snapshot)
    }
}
impl MarketDataClient {
    pub(super) async fn fetch_a_share_news_diagnostics(
        &self,
        ts_code: &str,
        limit: usize,
    ) -> anyhow::Result<super::NewsFetchResult> {
        let symbol = ts_code.split('.').next().unwrap_or(ts_code);
        let search_match = self
            .search_stocks_from_eastmoney(symbol, Some("A股"), 8)
            .await
            .ok()
            .and_then(|mut items| {
                items
                    .drain(..)
                    .find(|item| item.symbol == symbol)
                    .map(|item| item.name)
            })
            .unwrap_or_else(|| symbol.to_string());
        let query_terms = vec![
            symbol.to_string(),
            search_match.clone(),
            format!("{search_match} 公告"),
            format!("{search_match} 业绩"),
            format!("{search_match} 回购 分红"),
            format!("{search_match} 调研"),
        ];

        // Fetch eastmoney announcements first (fast, usually < 2s).
        // Then attempt web search with a bounded timeout so slow providers
        // don't block the entire pipeline.
        let eastmoney_result = self.fetch_a_share_eastmoney_news(ts_code, limit).await;
        let has_eastmoney = eastmoney_result
            .as_ref()
            .is_ok_and(|items| !items.is_empty());

        let search_timeout_secs = if has_eastmoney { 6 } else { 12 };
        let (google_items, google_attempts) = match tokio::time::timeout(
            std::time::Duration::from_secs(search_timeout_secs),
            self.fetch_news_search_queries_with_attempts(
                &query_terms,
                "zh-CN",
                Some("month"),
                None,
                None,
                super::GeneralSearchIntent::CompanyEvidence,
            ),
        )
        .await
        {
            Ok(result) => result,
            Err(_) => {
                tracing::info!(
                    symbol = %ts_code,
                    timeout_secs = search_timeout_secs,
                    "A-share web search timed out, using eastmoney results only"
                );
                (Vec::new(), Vec::new())
            }
        };
        let mut result = Self::merge_a_share_news(
            ts_code,
            limit,
            eastmoney_result,
            google_items,
            google_attempts,
            query_terms,
        )?;

        // If per-symbol news is sparse, fetch macro/industry context
        if result.items.len() < 10 {
            let company = &search_match;
            let macro_queries = vec![
                format!("{} 行业 政策", company),
                "中国 宏观经济 货币政策".to_string(),
                "A股 市场 资金面".to_string(),
            ];
            let existing_titles: std::collections::HashSet<String> = result
                .items
                .iter()
                .map(|i| i.title.to_lowercase())
                .collect();
            if let Ok((macro_items, macro_attempts)) = tokio::time::timeout(
                std::time::Duration::from_secs(8),
                self.fetch_news_search_queries_with_attempts(
                    &macro_queries,
                    "zh-CN",
                    Some("month"),
                    None,
                    None,
                    super::GeneralSearchIntent::MacroEvidence,
                ),
            )
            .await
            {
                for item in macro_items {
                    if !existing_titles.contains(&item.title.to_lowercase()) {
                        result.items.push(item);
                    }
                }
                result.attempts.extend(macro_attempts);
            }
        }

        Ok(result)
    }

    pub(super) async fn fetch_a_share_global_news_diagnostics(
        &self,
        curr_date: &str,
        _look_back_days: usize,
        limit: usize,
    ) -> anyhow::Result<super::NewsFetchResult> {
        // Fetch from akshare Chinese financial news sources in parallel.
        let (cls_res, ths_res, sina_res, futu_res) = tokio::join!(
            super::akshare_rust::a_share::fetch_global_news_cls(self),
            super::akshare_rust::a_share::fetch_global_news_ths(self),
            super::akshare_rust::a_share::fetch_global_news_sina(self),
            super::akshare_rust::a_share::fetch_global_news_futu(self),
        );
        let mut akshare_items = Vec::new();
        let mut attempts = Vec::new();
        for (name, result) in [
            ("CLS 财联社", cls_res),
            ("THS 同花顺", ths_res),
            ("Sina 新浪", sina_res),
            ("Futu 富途", futu_res),
        ] {
            match result {
                Ok(items) => {
                    let count = items.len();
                    akshare_items.extend(items);
                    attempts.push(super::NewsFetchAttempt {
                        source: name.to_string(),
                        query: None,
                        success: true,
                        item_count: count,
                        error: None,
                    });
                }
                Err(e) => {
                    attempts.push(super::NewsFetchAttempt {
                        source: name.to_string(),
                        query: None,
                        success: false,
                        item_count: 0,
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        let queries = vec![
            "A股 市场 宏观".to_string(),
            "中国 经济 政策".to_string(),
            "A股 资金面".to_string(),
            "A股 银行 券商 基金".to_string(),
            "China stock market economy policy".to_string(),
            "A股 央行 降息 利率".to_string(),
            "中国 GDP PMI 经济数据".to_string(),
            "沪深 北向资金 外资".to_string(),
        ];
        let (web_merged, web_attempts) = self
            .fetch_news_search_queries_with_attempts(
                &queries,
                "zh-CN",
                Some("month"),
                None,
                None,
                super::GeneralSearchIntent::MacroEvidence,
            )
            .await;
        attempts.extend(web_attempts);
        let mut merged = akshare_items;
        merged.extend(web_merged);
        let original_merged = merged.clone();
        let year = curr_date.get(0..4).unwrap_or_default();
        let filtered_by_year = merged
            .into_iter()
            .filter(|item| {
                super::news_filter::normalized_news_date(&item.published_at)
                    .is_some_and(|date| date.starts_with(year))
            })
            .collect::<Vec<_>>();
        let selected_items = if filtered_by_year.is_empty() {
            tracing::info!(
                curr_date = %curr_date,
                "a-share global news year filter removed all searxng results; keeping unfiltered recent items"
            );
            original_merged
        } else {
            filtered_by_year
        };
        let merged = super::news_filter::merge_ranked_news(
            selected_items,
            limit.max(8),
            None,
            None,
            &[
                "A股".to_string(),
                "中国经济".to_string(),
                "政策".to_string(),
                "资金面".to_string(),
            ],
        );
        let (merged, attempts, cacheable) = if merged.is_empty() {
            let fallback_items = Self::a_share_macro_reference_pages(curr_date);
            let mut attempts = attempts;
            attempts.push(super::NewsFetchAttempt {
                source: "A-share Macro Reference".to_string(),
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

    pub(super) async fn fetch_a_share_insider_transactions(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let ts_code = self
            .normalize_a_share_symbol(symbol)
            .context("invalid A-share symbol for insider transactions")?;
        let rows = self
            .tushare_query(
                "stk_holdertrade",
                serde_json::json!({ "ts_code": ts_code }),
                "ts_code,ann_date,holder_name,holder_type,in_de,change_vol,change_ratio,after_share,after_ratio,avg_price,total_share,begin_date,close_date",
            )
            .await?;
        let filtered = rows
            .into_iter()
            .map(|row| NewsItem {
                published_at: row.string("ann_date").unwrap_or_default(),
                title: format!(
                    "股东{} {}",
                    if row.string("in_de").unwrap_or_default() == "IN" {
                        "增持"
                    } else {
                        "减持"
                    },
                    row.string("holder_name")
                        .unwrap_or_else(|_| "未知股东".to_string())
                ),
                summary: format!(
                    "holder_type={}, change_vol={:?}, change_ratio={:?}, avg_price={:?}",
                    row.string("holder_type").unwrap_or_default(),
                    row.optional_f64("change_vol"),
                    row.optional_f64("change_ratio"),
                    row.optional_f64("avg_price")
                ),
                source: "Tushare stk_holdertrade".to_string(),
                url: None,
            })
            .collect::<Vec<_>>();
        if filtered.is_empty() {
            bail!(
                "no A-share insider transaction items available for {}",
                symbol
            );
        }
        Ok(filtered)
    }

    fn merge_a_share_news(
        ts_code: &str,
        limit: usize,
        eastmoney_result: anyhow::Result<Vec<NewsItem>>,
        google_items: Vec<NewsItem>,
        mut google_attempts: Vec<super::NewsFetchAttempt>,
        keywords: Vec<String>,
    ) -> anyhow::Result<super::NewsFetchResult> {
        let mut merged = Vec::new();
        let mut seen = HashSet::new();
        let mut errors = Vec::new();
        let mut attempts = Vec::new();

        match eastmoney_result {
            Ok(items) => {
                attempts.push(super::NewsFetchAttempt {
                    source: "Eastmoney 公告".to_string(),
                    query: Some(ts_code.to_string()),
                    success: true,
                    item_count: items.len(),
                    error: None,
                });
                Self::push_unique_news(&mut merged, &mut seen, items)
            }
            Err(error) => {
                errors.push(format!("eastmoney: {error:#}"));
                attempts.push(super::NewsFetchAttempt {
                    source: "Eastmoney 公告".to_string(),
                    query: Some(ts_code.to_string()),
                    success: false,
                    item_count: 0,
                    error: Some(error.to_string()),
                });
            }
        }
        Self::push_unique_news(&mut merged, &mut seen, google_items);
        attempts.append(&mut google_attempts);

        if merged.is_empty() {
            bail!(
                "no A-share news available for {} from merged upstreams: {}",
                ts_code,
                errors.join(" | ")
            );
        }

        let target_limit = limit.max(8);
        let ranked =
            super::news_filter::merge_ranked_news(merged, target_limit, None, None, &keywords);
        let has_official = ranked.iter().any(|item| item.source.contains("Eastmoney"));
        let web_quota = if has_official {
            ranked
                .iter()
                .filter(|item| !item.source.contains("Eastmoney"))
                .count()
                .min(if target_limit >= 6 { 2 } else { 1 })
        } else {
            0
        };
        let mut items = Vec::with_capacity(target_limit);
        let mut web_taken = 0usize;
        let mut seen = HashSet::new();
        for item in ranked
            .iter()
            .filter(|item| !item.source.contains("Eastmoney"))
            .take(web_quota)
        {
            let key = format!(
                "{}|{}|{}",
                item.title.trim(),
                item.published_at.trim(),
                item.url.as_deref().unwrap_or_default().trim()
            );
            if seen.insert(key) {
                items.push(item.clone());
                web_taken += 1;
            }
        }
        for item in ranked {
            if items.len() >= target_limit {
                break;
            }
            let key = format!(
                "{}|{}|{}",
                item.title.trim(),
                item.published_at.trim(),
                item.url.as_deref().unwrap_or_default().trim()
            );
            if seen.insert(key) {
                items.push(item);
            }
        }
        if web_taken == 0 && has_official {
            tracing::info!(symbol = %ts_code, "a-share news merge produced only official announcement items");
        }
        let cacheable = super::news_result_cacheable(&items, &attempts);
        Ok(super::NewsFetchResult {
            items,
            attempts,
            cacheable,
        })
    }

    fn push_unique_news(
        merged: &mut Vec<NewsItem>,
        seen: &mut HashSet<String>,
        items: Vec<NewsItem>,
    ) {
        for item in items {
            let dedupe_key = format!(
                "{}|{}|{}",
                item.title.trim(),
                item.published_at.trim(),
                item.url.as_deref().unwrap_or_default().trim()
            );
            if seen.insert(dedupe_key) {
                merged.push(item);
            }
        }
    }

    async fn fetch_a_share_individual_info(
        &self,
        symbol: &str,
    ) -> anyhow::Result<AkshareIndividualInfo> {
        let Some(token) = self.tushare_token.as_deref() else {
            return Ok(AkshareIndividualInfo {
                stock_name: None,
                total_share: None,
                market_cap: None,
                industry: None,
            });
        };
        let ts_code = self
            .normalize_a_share_symbol(symbol)
            .context("invalid A-share symbol for stock_basic")?;
        let response = self
            .http
            .post("https://api.tushare.pro")
            .json(&serde_json::json!({
                "api_name": "stock_basic",
                "token": token,
                "params": { "ts_code": ts_code, "list_status": "L" },
                "fields": "ts_code,name,industry"
            }))
            .send()
            .await
            .context("failed to call tushare stock_basic for A-share individual info")?
            .error_for_status()
            .context("tushare stock_basic http request failed")?;
        let payload: super::wire::TushareResponse = response
            .json()
            .await
            .context("failed to decode tushare stock_basic response")?;
        let basic_rows = if payload.code == 0 {
            payload
                .data
                .map(|data| {
                    data.items
                        .into_iter()
                        .map(|item| super::wire::TushareRow::new(&data.fields, item))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        } else {
            tracing::warn!(
                symbol = %symbol,
                code = payload.code,
                msg = %payload.msg.as_deref().unwrap_or(""),
                "tushare stock_basic returned error for A-share individual info"
            );
            Vec::new()
        };
        let basic = basic_rows.first();
        Ok(AkshareIndividualInfo {
            stock_name: basic.and_then(|row| row.optional_string("name")),
            total_share: None,
            market_cap: None,
            industry: basic.and_then(|row| row.optional_string("industry")),
        })
    }
}
impl MarketDataClient {
    fn tencent_market_symbol(&self, symbol: &str) -> anyhow::Result<String> {
        let normalized = self
            .normalize_a_share_symbol(symbol)
            .context("invalid A-share symbol")?;
        let (code, suffix) = normalized
            .split_once('.')
            .context("invalid normalized A-share symbol")?;
        let prefix = match suffix {
            "SH" => "sh",
            "SZ" | "BJ" => "sz",
            _ => bail!("unsupported A-share suffix {}", suffix),
        };
        Ok(format!("{prefix}{code}"))
    }
}
impl MarketDataClient {
    pub(super) async fn fetch_a_share_return_since(
        &self,
        symbol: &str,
        start_date: &str,
        holding_days: usize,
    ) -> anyhow::Result<Option<f64>> {
        let candles = self.fetch_a_share_candles(symbol, "qfq", 120).await?;
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
        Ok(Some((end_price - start_price) / start_price))
    }

    pub(super) fn parse_a_share_capital_flow_line(line: &str) -> anyhow::Result<CapitalFlowPoint> {
        let fields = line.split(',').collect::<Vec<_>>();
        if fields.len() < 13 {
            bail!("unexpected eastmoney capital flow format: {}", line);
        }

        Ok(CapitalFlowPoint {
            trade_date: fields[0].to_string(),
            main_net_inflow: fields[1].parse().context("invalid main_net_inflow")?,
            small_net_inflow: fields[2].parse().context("invalid small_net_inflow")?,
            medium_net_inflow: fields[3].parse().context("invalid medium_net_inflow")?,
            large_net_inflow: fields[4].parse().context("invalid large_net_inflow")?,
            super_large_net_inflow: fields[5]
                .parse()
                .context("invalid super_large_net_inflow")?,
            main_net_inflow_ratio_pct: fields[6]
                .parse()
                .context("invalid main_net_inflow_ratio_pct")?,
            small_net_inflow_ratio_pct: fields[7]
                .parse()
                .context("invalid small_net_inflow_ratio_pct")?,
            medium_net_inflow_ratio_pct: fields[8]
                .parse()
                .context("invalid medium_net_inflow_ratio_pct")?,
            large_net_inflow_ratio_pct: fields[9]
                .parse()
                .context("invalid large_net_inflow_ratio_pct")?,
            super_large_net_inflow_ratio_pct: fields[10]
                .parse()
                .context("invalid super_large_net_inflow_ratio_pct")?,
            close: fields[11].parse().context("invalid capital flow close")?,
            change_pct: fields[12]
                .parse()
                .context("invalid capital flow change_pct")?,
        })
    }

    async fn fetch_a_share_eastmoney_news(
        &self,
        ts_code: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let symbol = ts_code
            .split_once('.')
            .map(|(code, _)| code)
            .unwrap_or(ts_code);
        let page_size = limit.to_string();
        let response = self
            .http
            .get("https://np-anotice-stock.eastmoney.com/api/security/ann")
            .query(&[
                ("page_size", page_size.as_str()),
                ("page_index", "1"),
                ("ann_type", "A"),
                ("client_source", "web"),
                ("stock_list", symbol),
            ])
            .send()
            .await
            .context("failed to fetch Eastmoney announcements")?
            .error_for_status()
            .context("eastmoney announcements request failed")?;
        let payload: EastmoneyAnnouncementsEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney announcements response")?;
        let mut items = payload
            .data
            .and_then(|data| data.list)
            .unwrap_or_default()
            .into_iter()
            .map(|item| NewsItem {
                published_at: item.notice_date.unwrap_or_default(),
                title: item.title.clone().unwrap_or_else(|| "公司公告".to_string()),
                summary: item.title.unwrap_or_else(|| "公司公告".to_string()),
                source: "Eastmoney 公告".to_string(),
                url: item.art_code.map(|art_code| {
                    format!("https://data.eastmoney.com/notices/detail/{symbol}/{art_code}.html")
                }),
            })
            .collect::<Vec<_>>();
        if items.is_empty() {
            bail!("eastmoney returned no announcement items");
        }
        items.truncate(limit);
        Ok(items)
    }

    async fn fetch_a_share_tencent_market_cap(&self, symbol: &str) -> anyhow::Result<Option<f64>> {
        let market_symbol = self.tencent_market_symbol(symbol)?;
        let response = self
            .http
            .get("https://qt.gtimg.cn/q")
            .query(&[("q", market_symbol.as_str())])
            .send()
            .await
            .context("failed to fetch A-share Tencent market cap")?
            .error_for_status()
            .context("tencent market cap request failed")?
            .text()
            .await
            .context("failed to read tencent market cap response")?;
        Self::parse_tencent_quote_market_cap(&response)
    }
}
