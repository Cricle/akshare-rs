mod news_filter;
mod yahoo_chart;

use anyhow::{Context, bail};
use chrono::NaiveDate;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, REFERER};
use rust_decimal::prelude::ToPrimitive;
use std::time::Duration;
use tokio::sync::OnceCell;

pub(crate) static TICKER_CACHE: OnceCell<SecTickerLookup> = OnceCell::const_new();

use super::news_filter::{
    build_dated_news_query, latest_metric_value, merge_ranked_news, within_date_window,
};
use super::{
    CandlesWithProvider, FundamentalsSnapshot, MarketDataClient, NewsItem, QuoteSnapshot,
    QuoteWithProvider, SearchProviderKind,
    akshare_rust::us_sina,
    wire::{CompanyFactsResponse, CompanySubmissionsResponse, SecTickerEntry, SecTickerLookup},
};
use news_filter::{is_direct_company_news_match, normalize_company_query_name};
use yahoo_chart::YahooChartResponse;
impl MarketDataClient {
    const US_SEC_SUBMISSIONS_TIMEOUT_SECS: u64 = 4;
    const US_COMPANY_SEARCH_NEWS_QUERY_LIMIT: usize = 6;
    const US_COMPANY_SEARCH_GENERAL_QUERY_LIMIT: usize = 2;
    const US_COMPANY_SEARCH_BATCH_SIZE: usize = 4;

    fn company_news_primary_queries(
        symbol: &str,
        company_name: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Vec<String> {
        vec![
            build_dated_news_query(symbol, start_date, end_date),
            build_dated_news_query(&format!("{symbol} earnings"), start_date, end_date),
            build_dated_news_query(&format!("{company_name} {symbol}"), start_date, end_date),
            build_dated_news_query(&format!("{company_name} earnings"), start_date, end_date),
            build_dated_news_query(
                &format!("{symbol} {company_name} revenue guidance Reuters"),
                start_date,
                end_date,
            ),
            build_dated_news_query(
                &format!("{company_name} press release Reuters"),
                start_date,
                end_date,
            ),
            build_dated_news_query(
                &format!("{company_name} investor relations BusinessWire PRNewswire"),
                start_date,
                end_date,
            ),
        ]
    }

    fn company_news_supplemental_queries(
        symbol: &str,
        company_name: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Vec<String> {
        vec![
            build_dated_news_query(&format!("{symbol} guidance"), start_date, end_date),
            build_dated_news_query(&format!("{company_name} guidance"), start_date, end_date),
            build_dated_news_query(&format!("{symbol} analyst"), start_date, end_date),
            build_dated_news_query(&format!("{company_name} filing"), start_date, end_date),
            build_dated_news_query(
                &format!("{symbol} Reuters Bloomberg CNBC"),
                start_date,
                end_date,
            ),
            build_dated_news_query(
                &format!("{company_name} product launch supplier demand"),
                start_date,
                end_date,
            ),
            build_dated_news_query(
                &format!("{company_name} revenue forecast services iphone"),
                start_date,
                end_date,
            ),
            build_dated_news_query(
                &format!("{symbol} buyback tariff antitrust"),
                start_date,
                end_date,
            ),
        ]
    }

    fn us_global_news_primary_queries(start: &str, curr_date: &str) -> Vec<String> {
        vec![
            build_dated_news_query("stock market economy", Some(start), Some(curr_date)),
            build_dated_news_query(
                "Federal Reserve interest rates",
                Some(start),
                Some(curr_date),
            ),
            build_dated_news_query("inflation economic outlook", Some(start), Some(curr_date)),
            build_dated_news_query("global markets trading", Some(start), Some(curr_date)),
            build_dated_news_query(
                "AI semiconductor megacap market outlook",
                Some(start),
                Some(curr_date),
            ),
        ]
    }

    fn us_global_news_supplemental_queries(start: &str, curr_date: &str) -> Vec<String> {
        vec![
            build_dated_news_query(
                "Treasury yields dollar risk appetite",
                Some(start),
                Some(curr_date),
            ),
            build_dated_news_query(
                "semiconductor demand export restrictions",
                Some(start),
                Some(curr_date),
            ),
            build_dated_news_query("cloud AI capex hyperscalers", Some(start), Some(curr_date)),
        ]
    }

    pub(super) async fn fetch_us_quote_with_provider(
        &self,
        symbol: &str,
    ) -> anyhow::Result<QuoteWithProvider> {
        match us_sina::fetch_quote(self, symbol).await {
            Ok(snapshot) => {
                return Ok(QuoteWithProvider {
                    quote: snapshot,
                    provider: "sina_us_daily".to_string(),
                });
            }
            Err(error) => {
                tracing::info!(
                    symbol = %symbol,
                    error = ?error,
                    "Sina US quote failed, falling back to Eastmoney quote"
                );
            }
        }
        match self.fetch_us_quote_from_eastmoney(symbol).await {
            Ok(snapshot) => {
                return Ok(QuoteWithProvider {
                    quote: snapshot,
                    provider: "eastmoney_quote".to_string(),
                });
            }
            Err(error) => {
                tracing::info!(
                    symbol = %symbol,
                    error = ?error,
                    "Eastmoney US quote failed, falling back to Yahoo Finance quote"
                );
            }
        }
        let end = chrono::Utc::now().date_naive() + chrono::Days::new(1);
        let start = end - chrono::Days::new(10);
        match self.fetch_us_chart_candles(symbol, start, end).await {
            Ok(mut items) => {
                if let Some(last) = items.pop() {
                    return Ok(QuoteWithProvider {
                        quote: QuoteSnapshot {
                            symbol: symbol.trim().to_uppercase(),
                            date: last.trade_date,
                            open: last.open,
                            high: last.high,
                            low: last.low,
                            close: last.close,
                            volume: last.volume,
                        },
                        provider: "yahoo_finance_chart".to_string(),
                    });
                }
            }
            Err(error) => {
                tracing::info!(
                    symbol = %symbol,
                    error = ?error,
                    "Yahoo Finance quote failed, falling back to Stooq quote"
                );
            }
        }
        Ok(QuoteWithProvider {
            quote: self.fetch_us_stooq_quote(symbol).await?,
            provider: "stooq".to_string(),
        })
    }

    pub(super) async fn fetch_us_fundamentals(
        &self,
        symbol: &str,
    ) -> anyhow::Result<FundamentalsSnapshot> {
        let t0 = std::time::Instant::now();
        let company = self.lookup_company(symbol).await?;
        tracing::debug!(
            "fetch_us_fundamentals: lookup_company for {symbol} took {}ms",
            t0.elapsed().as_millis()
        );

        let t1 = std::time::Instant::now();
        let company_facts: CompanyFactsResponse = self
            .http
            .get(format!(
                "https://data.sec.gov/api/xbrl/companyfacts/CIK{}.json",
                company.cik
            ))
            .send()
            .await
            .context("failed to fetch company facts from SEC")?
            .error_for_status()
            .context("SEC company facts request failed")?
            .json()
            .await
            .context("failed to decode SEC company facts")?;
        tracing::debug!(
            "fetch_us_fundamentals: companyfacts for {symbol} took {}ms",
            t1.elapsed().as_millis()
        );

        let t2 = std::time::Instant::now();
        let submissions: CompanySubmissionsResponse = self
            .http
            .get(format!(
                "https://data.sec.gov/submissions/CIK{}.json",
                company.cik
            ))
            .send()
            .await
            .context("failed to fetch SEC submissions")?
            .error_for_status()
            .context("SEC submissions request failed")?
            .json()
            .await
            .context("failed to decode SEC submissions")?;
        tracing::debug!(
            "fetch_us_fundamentals: submissions for {symbol} took {}ms",
            t2.elapsed().as_millis()
        );
        let industry = submissions.sic_description.filter(|s| !s.is_empty());

        let fiscal_year_end = company_facts.fiscal_year_end.clone();
        let shares_outstanding = company_facts
            .facts
            .dei
            .as_ref()
            .and_then(|dei| dei.entity_common_stock_shares_outstanding.as_ref())
            .and_then(|metric| latest_metric_value(&metric.units))
            .map(|value| value.round() as i64);
        let annual_snapshot = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(super::wire::UsGaapFacts::latest_annual_snapshot);
        let net_income_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(|facts| facts.annual_net_income_aligned(annual_snapshot.as_ref()));
        let revenues_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(|facts| facts.annual_revenue_aligned(annual_snapshot.as_ref()));
        let assets_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(super::wire::UsGaapFacts::latest_assets);
        let liabilities_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(super::wire::UsGaapFacts::latest_liabilities);
        let stockholders_equity_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(super::wire::UsGaapFacts::latest_stockholders_equity);
        let cash_and_equivalents_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(super::wire::UsGaapFacts::latest_cash_and_equivalents);
        let mut gross_profit_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(|facts| facts.annual_gross_profit_aligned(annual_snapshot.as_ref()));
        let mut operating_income_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(|facts| facts.annual_operating_income_aligned(annual_snapshot.as_ref()));
        let mut operating_expenses_usd =
            company_facts.facts.us_gaap.as_ref().and_then(|facts| {
                facts.annual_operating_expenses_aligned(annual_snapshot.as_ref())
            });
        let operating_cash_flow_usd =
            company_facts.facts.us_gaap.as_ref().and_then(|facts| {
                facts.annual_operating_cash_flow_aligned(annual_snapshot.as_ref())
            });
        let mut capital_expenditure_usd =
            company_facts.facts.us_gaap.as_ref().and_then(|facts| {
                facts.annual_capital_expenditure_aligned(annual_snapshot.as_ref())
            });
        if let (Some(revenue), Some(gross_profit)) = (revenues_usd, gross_profit_usd)
            && gross_profit > revenue
        {
            tracing::warn!(
                symbol = %symbol,
                revenue,
                gross_profit,
                "rejecting gross profit because it exceeds revenue for aligned annual period"
            );
            gross_profit_usd = None;
        }
        if let (Some(gross_profit), Some(operating_income)) =
            (gross_profit_usd, operating_income_usd)
            && operating_income > gross_profit
        {
            tracing::warn!(
                symbol = %symbol,
                gross_profit,
                operating_income,
                "rejecting operating income because it exceeds gross profit for aligned annual period"
            );
            operating_income_usd = None;
        }
        if let (Some(revenue), Some(operating_expenses)) = (revenues_usd, operating_expenses_usd)
            && operating_expenses > revenue
        {
            tracing::warn!(
                symbol = %symbol,
                revenue,
                operating_expenses,
                "rejecting operating expenses because they exceed revenue for aligned annual period"
            );
            operating_expenses_usd = None;
        }
        if let (Some(revenue), Some(capex)) = (revenues_usd, capital_expenditure_usd)
            && capex > revenue
        {
            tracing::warn!(
                symbol = %symbol,
                revenue,
                capex,
                "rejecting capital expenditure because it exceeds revenue for aligned annual period"
            );
            capital_expenditure_usd = None;
        }
        let free_cash_flow_usd = match (operating_cash_flow_usd, capital_expenditure_usd) {
            (Some(ocf), Some(capex)) => Some(ocf - capex),
            _ => None,
        };
        let long_term_debt_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(super::wire::UsGaapFacts::latest_long_term_debt);
        let current_debt_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(super::wire::UsGaapFacts::latest_current_debt);
        let total_debt_usd = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(super::wire::UsGaapFacts::latest_total_debt);
        let diluted_shares_outstanding = company_facts
            .facts
            .us_gaap
            .as_ref()
            .and_then(super::wire::UsGaapFacts::latest_diluted_shares_outstanding);
        let quote = self.fetch_quote(symbol).await.ok();
        let market_cap = quote
            .as_ref()
            .and_then(|quote| shares_outstanding.map(|shares| quote.close * shares as f64));

        Ok(FundamentalsSnapshot {
            symbol: symbol.to_uppercase(),
            company_name: company.title,
            cik: company.cik,
            industry,
            currency: "USD".to_string(),
            fiscal_year_end,
            shares_outstanding,
            market_cap,
            net_income_usd,
            revenues_usd,
            assets_usd,
            liabilities_usd,
            stockholders_equity_usd,
            cash_and_equivalents_usd,
            gross_profit_usd,
            operating_income_usd,
            operating_expenses_usd,
            operating_cash_flow_usd,
            capital_expenditure_usd,
            free_cash_flow_usd,
            long_term_debt_usd,
            current_debt_usd,
            total_debt_usd,
            diluted_shares_outstanding,
        })
    }
}
impl MarketDataClient {
    pub(super) async fn fetch_us_news_diagnostics(
        &self,
        symbol: &str,
        limit: usize,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> anyhow::Result<super::NewsFetchResult> {
        let company = self.lookup_company(symbol).await?;
        let mut attempts = Vec::new();
        let company_name = normalize_company_query_name(&company.title);
        let company_queries =
            Self::company_news_primary_queries(symbol, &company_name, start_date, end_date);
        let (search_items, mut search_attempts) = self
            .fetch_search_evidence_with_query_locales_and_scope_mix_strategy(
                &company_queries,
                Some("month"),
                start_date,
                end_date,
                super::GeneralSearchIntent::CompanyEvidence,
                Self::US_COMPANY_SEARCH_GENERAL_QUERY_LIMIT,
                None, // Try all available providers, not just Uapis
                Some(Self::US_COMPANY_SEARCH_NEWS_QUERY_LIMIT),
                Some(Self::US_COMPANY_SEARCH_GENERAL_QUERY_LIMIT),
                Self::US_COMPANY_SEARCH_BATCH_SIZE,
            )
            .await;
        let mut items = search_items;
        attempts.append(&mut search_attempts);

        let sec_items = match tokio::time::timeout(
            Duration::from_secs(Self::US_SEC_SUBMISSIONS_TIMEOUT_SECS),
            self.fetch_us_sec_filing_news_items(&company, limit, start_date, end_date),
        )
        .await
        {
            Ok(Ok(items)) => {
                attempts.push(super::NewsFetchAttempt {
                    source: "SEC EDGAR".to_string(),
                    query: Some(company.cik.clone()),
                    success: !items.is_empty(),
                    item_count: items.len(),
                    error: items.is_empty().then_some(
                        "SEC submissions contained no in-range filing items".to_string(),
                    ),
                });
                items
            }
            Ok(Err(error)) => {
                attempts.push(super::NewsFetchAttempt {
                    source: "SEC EDGAR".to_string(),
                    query: Some(company.cik.clone()),
                    success: false,
                    item_count: 0,
                    error: Some(error.to_string()),
                });
                Vec::new()
            }
            Err(_) => {
                attempts.push(super::NewsFetchAttempt {
                    source: "SEC EDGAR".to_string(),
                    query: Some(company.cik.clone()),
                    success: false,
                    item_count: 0,
                    error: Some(format!(
                        "SEC submissions timed out after {}s",
                        Self::US_SEC_SUBMISSIONS_TIMEOUT_SECS
                    )),
                });
                Vec::new()
            }
        };
        items.extend(sec_items);
        let keywords = vec![
            symbol.to_string(),
            company_name.clone(),
            company.title.clone(),
            "earnings".to_string(),
            "guidance".to_string(),
            "filing".to_string(),
            "analyst".to_string(),
        ];
        let items = merge_ranked_news(items, limit.max(8) * 3, start_date, end_date, &keywords)
            .into_iter()
            .filter(|item| {
                is_direct_company_news_match(item, symbol, &company_name, &company.title)
            })
            .collect::<Vec<_>>();
        let distinct_sources = items
            .iter()
            .map(|item| item.source.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect::<std::collections::HashSet<_>>()
            .len();
        let mostly_sec_only = distinct_sources <= 1
            && items
                .iter()
                .all(|item| item.source.eq_ignore_ascii_case("SEC EDGAR"));
        let should_retry_with_supplemental = items.len() < limit.min(4) || mostly_sec_only;
        let items = if should_retry_with_supplemental {
            let supplemental_queries = Self::company_news_supplemental_queries(
                symbol,
                &company_name,
                start_date,
                end_date,
            );
            let (supplemental_items, mut supplemental_attempts) = self
                .fetch_search_evidence_with_query_locales_and_scope_mix_strategy(
                    &supplemental_queries,
                    Some("month"),
                    start_date,
                    end_date,
                    super::GeneralSearchIntent::CompanyEvidence,
                    2,
                    None, // Try all available providers
                    Some(6),
                    Some(2),
                    Self::US_COMPANY_SEARCH_BATCH_SIZE,
                )
                .await;
            attempts.append(&mut supplemental_attempts);
            merge_ranked_news(
                items
                    .into_iter()
                    .chain(supplemental_items)
                    .collect::<Vec<_>>(),
                limit.max(8) * 3,
                start_date,
                end_date,
                &keywords,
            )
            .into_iter()
            .filter(|item| {
                is_direct_company_news_match(item, symbol, &company_name, &company.title)
            })
            .collect::<Vec<_>>()
        } else {
            items
        };
        // If per-symbol news is sparse, fetch macro/industry context
        let mut items = if items.len() < 10 {
            let macro_queries = vec![
                format!("{} industry policy", company_name),
                "US market macro economy".to_string(),
                "Fed interest rate".to_string(),
            ];
            let existing_titles: std::collections::HashSet<String> =
                items.iter().map(|i| i.title.to_lowercase()).collect();
            let mut items = items;
            if let Ok((macro_items, macro_attempts)) = tokio::time::timeout(
                Duration::from_secs(8),
                self.fetch_search_evidence_with_query_locales_and_scope_mix_strategy(
                    &macro_queries,
                    Some("month"),
                    start_date,
                    end_date,
                    super::GeneralSearchIntent::MacroEvidence,
                    Self::US_COMPANY_SEARCH_GENERAL_QUERY_LIMIT,
                    Some(SearchProviderKind::Uapis),
                    Some(Self::US_COMPANY_SEARCH_NEWS_QUERY_LIMIT),
                    Some(Self::US_COMPANY_SEARCH_GENERAL_QUERY_LIMIT),
                    Self::US_COMPANY_SEARCH_BATCH_SIZE,
                ),
            )
            .await
            {
                for item in macro_items {
                    if !existing_titles.contains(&item.title.to_lowercase()) {
                        items.push(item);
                    }
                }
                attempts.extend(macro_attempts);
            }
            items
        } else {
            items
        };
        // Bing RSS fallback when search providers are unavailable
        if items.len() < limit.min(4) {
            let existing_titles: std::collections::HashSet<String> =
                items.iter().map(|i| i.title.to_lowercase()).collect();
            let bing_queries = [
                format!("{} stock news", symbol),
                format!("{} stock", company_name),
            ];
            let query_refs: Vec<&str> = bing_queries.iter().map(|s| s.as_str()).collect();
            let bing_items = self.fetch_bing_rss_news(&query_refs, 2).await;
            let mut bing_added = 0usize;
            for item in bing_items {
                if !existing_titles.contains(&item.title.to_lowercase()) {
                    items.push(item);
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
        if items.is_empty() {
            bail!("no US company news available from current upstreams");
        }
        let cacheable = super::news_result_cacheable(&items, &attempts);
        Ok(super::NewsFetchResult {
            items,
            attempts,
            cacheable,
        })
    }

    async fn fetch_us_sec_filing_news_items(
        &self,
        company: &SecTickerEntry,
        limit: usize,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let filings: CompanySubmissionsResponse = self
            .http
            .get(format!(
                "https://data.sec.gov/submissions/CIK{}.json",
                company.cik
            ))
            .send()
            .await
            .context("failed to fetch SEC submissions")?
            .error_for_status()
            .context("SEC submissions request failed")?
            .json()
            .await
            .context("failed to decode SEC submissions")?;

        let recent = filings.filings.recent;
        let total = recent
            .filing_date
            .len()
            .min(recent.form.len())
            .min(recent.accession_number.len())
            .min(recent.primary_document.len());
        let mut items = Vec::new();
        for index in 0..total.min(limit) {
            let filing_date = &recent.filing_date[index];
            if !within_date_window(filing_date, start_date, end_date) {
                continue;
            }
            items.push(NewsItem {
                published_at: filing_date.clone(),
                title: format!(
                    "SEC {} filing {}",
                    recent.form[index], recent.accession_number[index]
                ),
                summary: format!("Primary document: {}", recent.primary_document[index]),
                source: "SEC EDGAR".to_string(),
                url: Some(format!(
                    "https://www.sec.gov/Archives/edgar/data/{}/{}/{}",
                    company.cik.trim_start_matches('0'),
                    recent.accession_number[index].replace('-', ""),
                    recent.primary_document[index]
                )),
            });
        }
        Ok(items)
    }

    pub(super) async fn fetch_us_global_news_diagnostics(
        &self,
        curr_date: &str,
        look_back_days: usize,
        limit: usize,
    ) -> anyhow::Result<super::NewsFetchResult> {
        let end = NaiveDate::parse_from_str(curr_date, "%Y-%m-%d")
            .context("invalid curr_date for global news")?;
        let start = end - chrono::Days::new(look_back_days as u64);
        let start_text = start.to_string();
        let queries = Self::us_global_news_primary_queries(&start_text, curr_date);
        let (items, attempts) = self
            .fetch_news_search_queries_with_attempts(
                &queries,
                "en-US",
                Some("month"),
                Some(&start_text),
                Some(curr_date),
                super::GeneralSearchIntent::MacroEvidence,
            )
            .await;
        let mut merged = merge_ranked_news(
            items,
            limit,
            Some(&start_text),
            Some(curr_date),
            &[
                "stock market".to_string(),
                "Federal Reserve".to_string(),
                "inflation".to_string(),
                "economy".to_string(),
                "AI".to_string(),
                "semiconductor".to_string(),
            ],
        );
        if merged.len() < limit.min(3) {
            let supplemental_queries =
                Self::us_global_news_supplemental_queries(&start_text, curr_date);
            let (supplemental_items, mut supplemental_attempts) = self
                .fetch_news_search_queries_with_attempts(
                    &supplemental_queries,
                    "en-US",
                    Some("month"),
                    Some(&start_text),
                    Some(curr_date),
                    super::GeneralSearchIntent::MacroEvidence,
                )
                .await;
            let mut attempts = attempts;
            attempts.append(&mut supplemental_attempts);
            merged = merge_ranked_news(
                merged
                    .into_iter()
                    .chain(supplemental_items)
                    .collect::<Vec<_>>(),
                limit,
                Some(&start_text),
                Some(curr_date),
                &[
                    "stock market".to_string(),
                    "Federal Reserve".to_string(),
                    "inflation".to_string(),
                    "economy".to_string(),
                    "AI".to_string(),
                    "semiconductor".to_string(),
                    "Treasury".to_string(),
                    "cloud".to_string(),
                ],
            );
            if merged.is_empty() {
                bail!("no US global/macro news available from current upstreams");
            }
            let cacheable = super::news_result_cacheable(&merged, &attempts);
            return Ok(super::NewsFetchResult {
                items: merged,
                attempts,
                cacheable,
            });
        }
        if merged.is_empty() {
            bail!("no US global/macro news available from current upstreams");
        }
        let cacheable = super::news_result_cacheable(&merged, &attempts);
        Ok(super::NewsFetchResult {
            items: merged,
            attempts,
            cacheable,
        })
    }

    pub(super) async fn fetch_us_insider_transactions(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let company = self.lookup_company(symbol).await?;
        let filings: CompanySubmissionsResponse = self
            .http
            .get(format!(
                "https://data.sec.gov/submissions/CIK{}.json",
                company.cik
            ))
            .send()
            .await
            .context("failed to fetch SEC submissions for insider transactions")?
            .error_for_status()
            .context("SEC submissions request failed for insider transactions")?
            .json()
            .await
            .context("failed to decode SEC submissions for insider transactions")?;

        let recent = filings.filings.recent;
        let total = recent
            .filing_date
            .len()
            .min(recent.form.len())
            .min(recent.accession_number.len())
            .min(recent.primary_document.len());
        let mut items = Vec::new();
        for index in 0..total {
            let form = &recent.form[index];
            if !matches!(form.as_str(), "3" | "4" | "5" | "SC 13D" | "SC 13G") {
                continue;
            }
            items.push(NewsItem {
                published_at: recent.filing_date[index].clone(),
                title: format!("SEC insider-related filing {form}"),
                summary: format!(
                    "Accession {} | primary document {}",
                    recent.accession_number[index], recent.primary_document[index]
                ),
                source: "SEC EDGAR".to_string(),
                url: Some(format!(
                    "https://www.sec.gov/Archives/edgar/data/{}/{}/{}",
                    company.cik.trim_start_matches('0'),
                    recent.accession_number[index].replace('-', ""),
                    recent.primary_document[index]
                )),
            });
        }
        if items.is_empty() {
            bail!(
                "no insider transaction filings available from SEC for {}",
                symbol
            );
        }
        Ok(items)
    }
}
impl MarketDataClient {
    pub(super) async fn fetch_us_candles_with_provider(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<CandlesWithProvider> {
        let (mut items, provider) = match us_sina::fetch_candles(self, symbol, limit).await {
            Ok(items) => (items, "sina_us_daily".to_string()),
            Err(error) => {
                tracing::info!(
                    symbol = %symbol,
                    error = ?error,
                    "Sina US candles failed, falling back to Eastmoney"
                );
                match self.fetch_us_candles_from_eastmoney(symbol, limit).await {
                    Ok(items) => (items, "eastmoney_kline".to_string()),
                    Err(error) => {
                        tracing::info!(
                            symbol = %symbol,
                            error = ?error,
                            "Eastmoney US candles failed, falling back to Yahoo Finance chart"
                        );
                        let end = chrono::Utc::now().date_naive() + chrono::Days::new(1);
                        let start = end - chrono::Days::new((limit.max(260) + 30) as u64);
                        match self.fetch_us_chart_candles(symbol, start, end).await {
                            Ok(items) => (items, "yahoo_finance_chart".to_string()),
                            Err(error) => {
                                tracing::info!(
                                    symbol = %symbol,
                                    error = ?error,
                                    "Yahoo Finance chart candles failed, falling back to Stooq daily history"
                                );
                                (
                                    self.fetch_us_stooq_candles(symbol, limit)
                                        .await
                                        .with_context(|| {
                                            format!(
                                                "failed to fetch US candles for {symbol} from Yahoo Finance and Stooq fallback"
                                            )
                                        })?,
                                    "stooq".to_string(),
                                )
                            }
                        }
                    }
                }
            }
        };
        if items.is_empty() {
            bail!("historical quote request returned no rows for {}", symbol);
        }
        // Always apply limit trim regardless of provider
        if items.len() > limit {
            let start = items.len() - limit;
            items = items[start..].to_vec();
        }
        Ok(CandlesWithProvider {
            candles: items,
            provider,
        })
    }

    pub(crate) async fn fetch_us_quote_from_eastmoney(
        &self,
        symbol: &str,
    ) -> anyhow::Result<QuoteSnapshot> {
        let secid = self.eastmoney_us_secid(symbol).await?;
        let response = self
            .http
            .get("https://72.push2.eastmoney.com/api/qt/clist/get")
            .query(&crate::util::eastmoney_clist_params(
                "200",
                &[
                    ("fid", "f12"),
                    ("fs", secid.as_str()),
                    (
                        "fields",
                        "f2,f3,f4,f5,f6,f7,f8,f12,f13,f14,f15,f16,f17,f18,f20,f115",
                    ),
                ],
            ))
            .send()
            .await
            .context("failed to fetch US quote from Eastmoney")?
            .error_for_status()
            .context("eastmoney US quote request failed")?;

        let payload: serde_json::Value = response
            .json()
            .await
            .context("failed to decode eastmoney US quote response")?;
        let data = payload
            .get("data")
            .and_then(|value| value.get("diff"))
            .and_then(|value| value.as_array())
            .and_then(|items| items.first())
            .context("eastmoney US quote response missing diff item")?;
        let price = Self::eastmoney_quote_price(data.get("f2").and_then(|value| value.as_f64()))?;
        let high = Self::eastmoney_quote_price(data.get("f15").and_then(|value| value.as_f64()))?;
        let low = Self::eastmoney_quote_price(data.get("f16").and_then(|value| value.as_f64()))?;
        let open = Self::eastmoney_quote_price(data.get("f17").and_then(|value| value.as_f64()))?;
        let volume = data
            .get("f5")
            .and_then(|value| value.as_f64())
            .unwrap_or_default()
            .round() as i64;
        Ok(QuoteSnapshot {
            symbol: data
                .get("f12")
                .and_then(|value| value.as_str())
                .unwrap_or(symbol)
                .trim()
                .to_uppercase(),
            date: chrono::Utc::now().format("%Y%m%d").to_string(),
            open,
            high,
            low,
            close: price,
            volume,
        })
    }

    pub(crate) async fn fetch_us_candles_from_eastmoney(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<super::CandlePoint>> {
        let secid = self.eastmoney_us_secid(symbol).await?;
        let response = self
            .http
            .get("https://63.push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", "101"),
                ("fqt", "1"),
                ("lmt", &limit.max(5).to_string()),
                ("end", "20500000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ])
            .send()
            .await
            .context("failed to fetch US candles from Eastmoney")?
            .error_for_status()
            .context("eastmoney US candle request failed")?;

        let payload: super::wire::EastmoneyKlineEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney US candle response")?;
        let data = payload
            .data
            .context("eastmoney US candle response missing data")?;
        let klines = data
            .klines
            .context("eastmoney US candle response missing klines")?;
        let items = klines
            .into_iter()
            .map(|line| Self::parse_eastmoney_us_candle_line(&line))
            .collect::<anyhow::Result<Vec<_>>>()?;
        if items.is_empty() {
            bail!("eastmoney US candle response returned no rows");
        }
        Ok(crate::util::sort_and_limit(items, limit))
    }

    pub(crate) async fn fetch_us_stooq_quote(&self, symbol: &str) -> anyhow::Result<QuoteSnapshot> {
        let stooq_symbol = format!("{}.us", symbol.to_lowercase());
        let response = self
            .http
            .get("https://stooq.com/q/l/")
            .query(&[("s", stooq_symbol.as_str()), ("i", "d")])
            .send()
            .await
            .context("failed to fetch quote from stooq")?;

        if !response.status().is_success() {
            bail!("quote request failed with {}", response.status());
        }

        let csv = response
            .text()
            .await
            .context("failed to read stooq response")?;
        Self::parse_quote_csv(symbol, &csv)
    }

    fn parse_eastmoney_us_candle_line(line: &str) -> anyhow::Result<super::CandlePoint> {
        let fields = line.split(',').collect::<Vec<_>>();
        if fields.len() < 11 {
            bail!("unexpected eastmoney US candle format: {}", line);
        }
        Ok(super::CandlePoint {
            trade_date: fields[0].to_string(),
            open: fields[1]
                .parse()
                .context("invalid eastmoney US candle open")?,
            close: fields[2]
                .parse()
                .context("invalid eastmoney US candle close")?,
            high: fields[3]
                .parse()
                .context("invalid eastmoney US candle high")?,
            low: fields[4]
                .parse()
                .context("invalid eastmoney US candle low")?,
            volume: fields[5].parse::<f64>().unwrap_or_default().round() as i64,
            amount: fields[6].parse().unwrap_or_default(),
            amplitude_pct: fields[7].parse().unwrap_or_default(),
            change_pct: fields[8].parse().unwrap_or_default(),
            change_amount: fields[9].parse().unwrap_or_default(),
            turnover_pct: fields[10].parse().unwrap_or_default(),
        })
    }

    fn eastmoney_quote_price(value: Option<f64>) -> anyhow::Result<f64> {
        value.context("eastmoney quote price field missing")
    }

    pub(super) async fn fetch_us_chart_candles(
        &self,
        symbol: &str,
        start: NaiveDate,
        end: NaiveDate,
    ) -> anyhow::Result<Vec<super::CandlePoint>> {
        let period1 = start
            .and_hms_opt(0, 0, 0)
            .context("invalid Yahoo Finance chart start datetime")?
            .and_utc()
            .timestamp();
        let period2 = end
            .and_hms_opt(0, 0, 0)
            .context("invalid Yahoo Finance chart end datetime")?
            .and_utc()
            .timestamp();
        let response = self
            .http
            .get(format!(
                "https://query1.finance.yahoo.com/v8/finance/chart/{}",
                symbol.to_uppercase()
            ))
            .headers(Self::yahoo_chart_headers(symbol))
            .query(&[
                ("period1", period1.to_string()),
                ("period2", period2.to_string()),
                ("interval", "1d".to_string()),
                ("includePrePost", "false".to_string()),
                ("events", "div,splits".to_string()),
            ])
            .send()
            .await
            .context("failed to fetch historical quote from Yahoo Finance chart")?;

        if !response.status().is_success() {
            bail!(
                "Yahoo Finance chart historical quote request failed with {}",
                response.status()
            );
        }

        let payload: YahooChartResponse = response
            .json()
            .await
            .context("failed to decode Yahoo Finance chart response")?;
        let result = payload
            .chart
            .result
            .and_then(|mut items| items.drain(..).next())
            .context("Yahoo Finance chart response missing result")?;
        let quote = result
            .indicators
            .quote
            .and_then(|mut items| items.drain(..).next())
            .context("Yahoo Finance chart response missing quote series")?;
        let timestamps = result.timestamp.unwrap_or_default();

        let opens = quote.open.unwrap_or_default();
        let highs = quote.high.unwrap_or_default();
        let lows = quote.low.unwrap_or_default();
        let closes = quote.close.unwrap_or_default();
        let volumes = quote.volume.unwrap_or_default();

        let total = timestamps
            .len()
            .min(opens.len())
            .min(highs.len())
            .min(lows.len())
            .min(closes.len())
            .min(volumes.len());
        let mut items = Vec::new();
        for index in 0..total {
            let (Some(open), Some(high), Some(low), Some(close), Some(volume)) = (
                opens[index],
                highs[index],
                lows[index],
                closes[index],
                volumes[index],
            ) else {
                continue;
            };
            let trade_date = chrono::DateTime::from_timestamp(timestamps[index], 0)
                .context("invalid Yahoo Finance chart timestamp")?
                .date_naive()
                .format("%Y-%m-%d")
                .to_string();
            items.push(super::CandlePoint {
                trade_date,
                open,
                close,
                high,
                low,
                volume,
                amount: 0.0,
                amplitude_pct: if low > 0.0 {
                    ((high - low) / low) * 100.0
                } else {
                    0.0
                },
                change_pct: 0.0,
                change_amount: 0.0,
                turnover_pct: 0.0,
            });
        }

        items.sort_by_key(|item| NaiveDate::parse_from_str(&item.trade_date, "%Y-%m-%d").ok());

        for index in 1..items.len() {
            let previous_close = items[index - 1].close;
            if previous_close > 0.0 {
                items[index].change_amount = items[index].close - previous_close;
                items[index].change_pct = (items[index].change_amount / previous_close * 100.0)
                    .to_f64()
                    .unwrap_or_default();
            }
        }

        Ok(items)
    }
}
impl MarketDataClient {
    pub(crate) async fn fetch_us_stooq_candles(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<super::CandlePoint>> {
        let stooq_symbol = format!("{}.us", symbol.to_lowercase());
        let response = self
            .http
            .get("https://stooq.com/q/d/l/")
            .query(&[("s", stooq_symbol.as_str()), ("i", "d")])
            .send()
            .await
            .context("failed to fetch candle history from stooq")?;

        if !response.status().is_success() {
            bail!("stooq candle request failed with {}", response.status());
        }

        let csv = response
            .text()
            .await
            .context("failed to read stooq candle response")?;
        if csv.contains("Get your apikey:") {
            bail!("stooq candle request requires captcha/api key");
        }

        let mut items = Self::parse_us_stooq_candles_csv(symbol, &csv)?;
        if limit < items.len() {
            let start = items.len() - limit;
            items = items[start..].to_vec();
        }
        Ok(items)
    }

    fn yahoo_chart_headers(symbol: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json,text/plain,*/*"),
        );
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        headers.insert(
            REFERER,
            HeaderValue::from_static("https://finance.yahoo.com/"),
        );
        if let Ok(value) = HeaderValue::from_str(&format!(
            "https://finance.yahoo.com/quote/{}",
            symbol.to_uppercase()
        )) {
            headers.insert("x-referer-symbol", value);
        }
        headers
    }

    fn parse_us_stooq_candles_csv(
        symbol: &str,
        csv: &str,
    ) -> anyhow::Result<Vec<super::CandlePoint>> {
        let mut items = Vec::new();
        for line in csv.lines().skip(1) {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let parts = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
            if parts.len() < 6 {
                continue;
            }
            if parts[1].eq_ignore_ascii_case("N/D") || parts[4].eq_ignore_ascii_case("N/D") {
                continue;
            }
            let trade_date = parts[0].to_string();
            let open = parts[1]
                .parse::<f64>()
                .with_context(|| format!("invalid stooq open for {symbol} on {trade_date}"))?;
            let high = parts[2]
                .parse::<f64>()
                .with_context(|| format!("invalid stooq high for {symbol} on {trade_date}"))?;
            let low = parts[3]
                .parse::<f64>()
                .with_context(|| format!("invalid stooq low for {symbol} on {trade_date}"))?;
            let close = parts[4]
                .parse::<f64>()
                .with_context(|| format!("invalid stooq close for {symbol} on {trade_date}"))?;
            let volume = parts[5].parse::<i64>().unwrap_or_default();
            items.push(super::CandlePoint {
                trade_date,
                open,
                close,
                high,
                low,
                volume,
                amount: 0.0,
                amplitude_pct: if low > 0.0 {
                    ((high - low) / low) * 100.0
                } else {
                    0.0
                },
                change_pct: 0.0,
                change_amount: 0.0,
                turnover_pct: 0.0,
            });
        }

        items.sort_by_key(|item| NaiveDate::parse_from_str(&item.trade_date, "%Y-%m-%d").ok());
        for index in 1..items.len() {
            let previous_close = items[index - 1].close;
            if previous_close > 0.0 {
                items[index].change_amount = items[index].close - previous_close;
                items[index].change_pct = (items[index].change_amount / previous_close * 100.0)
                    .to_f64()
                    .unwrap_or_default();
            }
        }
        if items.is_empty() {
            bail!("stooq candle history returned no rows");
        }
        Ok(items)
    }

    async fn lookup_company(&self, symbol: &str) -> anyhow::Result<SecTickerEntry> {
        let entries = TICKER_CACHE
            .get_or_try_init(|| async {
                tracing::info!("fetching SEC ticker map (one-time cache miss)");
                let map: SecTickerLookup = self
                    .http
                    .get("https://www.sec.gov/files/company_tickers.json")
                    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
                    .send()
                    .await
                    .context("failed to fetch SEC ticker map")?
                    .error_for_status()
                    .context("SEC ticker map request failed")?
                    .json()
                    .await
                    .context("failed to decode SEC ticker map")?;
                tracing::info!("SEC ticker map cached: {} entries", map.len());
                Ok::<_, anyhow::Error>(map)
            })
            .await?;

        let normalized = symbol.trim().to_uppercase();
        entries
            .values()
            .find(|entry| entry.ticker.eq_ignore_ascii_case(&normalized))
            .map(|entry| SecTickerEntry {
                cik: format!("{:0>10}", entry.cik_str),
                title: entry.title.clone(),
            })
            .context("symbol not found in SEC ticker map")
    }

    pub(super) async fn fetch_us_return_since(
        &self,
        symbol: &str,
        start_date: &str,
        holding_days: usize,
    ) -> anyhow::Result<Option<f64>> {
        NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .with_context(|| format!("invalid start_date for return_since: {start_date}"))?;
        let items = self
            .fetch_us_candles_with_provider(symbol, holding_days + 15)
            .await?
            .candles
            .into_iter()
            .map(|item| (item.trade_date, item.close))
            .collect::<Vec<_>>();
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

    async fn eastmoney_us_secid(&self, symbol: &str) -> anyhow::Result<String> {
        self.fetch_eastmoney_us_quote_id(symbol.trim()).await
    }

    async fn fetch_eastmoney_us_quote_id(&self, symbol: &str) -> anyhow::Result<String> {
        let response = self
            .http
            .get("https://searchapi.eastmoney.com/api/suggest/get")
            .query(&[
                ("input", symbol),
                ("type", "14"),
                ("token", "D43BF722C8E33BDC906FB84D85E326E8"),
                ("count", "8"),
            ])
            .send()
            .await
            .context("failed to fetch eastmoney US quote id search")?
            .error_for_status()
            .context("eastmoney US quote id search request failed")?;
        let payload: serde_json::Value = response
            .json()
            .await
            .context("failed to decode eastmoney US quote id search response")?;
        let items = payload
            .get("QuotationCodeTable")
            .and_then(|value| value.get("Data"))
            .and_then(|value| value.as_array())
            .context("eastmoney US quote id search response missing data")?;
        let quote_id = items.iter().find_map(|item| {
            let code = item.get("Code")?.as_str()?;
            if !code.eq_ignore_ascii_case(symbol) {
                return None;
            }
            item.get("QuoteID")
                .and_then(|value| value.as_str())
                .map(str::to_string)
        });
        quote_id.context("eastmoney US quote id not found")
    }
}

#[cfg(test)]
mod tests {
    use super::super::NewsItem;
    use super::super::news_filter::{build_dated_news_query, within_date_window};
    use super::news_filter::{
        has_meaningful_us_company_event_signal, is_direct_company_news_match,
        normalize_company_query_name,
    };

    #[test]
    fn normalizes_company_query_name_for_search() {
        assert_eq!(
            normalize_company_query_name("Example Holdings Inc. Common Stock"),
            "Example"
        );
        assert_eq!(normalize_company_query_name("Alphabet Inc."), "Alphabet");
    }

    #[test]
    fn builds_dated_news_query_with_inclusive_end_date() {
        assert_eq!(
            build_dated_news_query(
                "TEST Example earnings",
                Some("2025-04-30"),
                Some("2025-05-07")
            ),
            "TEST Example earnings"
        );
    }

    #[test]
    fn checks_date_window_from_normalized_news_date() {
        assert!(within_date_window(
            "2025-05-07",
            Some("2025-04-30"),
            Some("2025-05-07")
        ));
        assert!(!within_date_window(
            "2025-05-08",
            Some("2025-04-30"),
            Some("2025-05-07")
        ));
    }

    #[test]
    fn us_company_news_match_accepts_strong_alias_plus_event_signal() {
        let item = NewsItem {
            published_at: "2026-05-25".to_string(),
            title: "Example supplier signals stronger device demand".to_string(),
            summary: "Reuters says Example demand outlook improved ahead of the next cycle."
                .to_string(),
            source: "Reuters".to_string(),
            url: Some("https://example.com/example-supplier".to_string()),
        };
        assert!(is_direct_company_news_match(
            &item,
            "TEST",
            "Example",
            "Example Inc. Common Stock"
        ));
    }

    #[test]
    fn us_company_news_match_rejects_name_only_without_event_signal() {
        let item = NewsItem {
            published_at: "2026-05-25".to_string(),
            title: "Example investor relations".to_string(),
            summary: "Corporate profile and overview".to_string(),
            source: "Example".to_string(),
            url: Some("https://example.com/example-ir".to_string()),
        };
        assert!(!has_meaningful_us_company_event_signal(&item));
        assert!(!is_direct_company_news_match(
            &item,
            "TEST",
            "Example",
            "Example Inc. Common Stock"
        ));
    }
}
