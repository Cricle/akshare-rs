//! SEC EDGAR API helpers (US company fundamentals).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundamentalsSnapshot;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CompanyFacts {
    #[serde(default)]
    facts: Option<serde_json::Value>,
}

impl AkShareClient {
    /// Fetch US company fundamentals from SEC EDGAR.
    #[allow(dead_code)]
    pub(crate) async fn sec_edgar_fundamentals(
        &self,
        symbol: &str,
    ) -> Result<FundamentalsSnapshot> {
        let facts_url = format!(
            "https://data.sec.gov/api/xbrl/companyfacts/CIK{:0>10}.json",
            symbol.trim_start_matches('0')
        );

        let facts_resp = self
            .get(&facts_url)
            .header("User-Agent", "akshare-rust/0.1 (contact@example.com)")
            .header("Accept", "application/json")
            .send()
            .await;

        match facts_resp {
            Ok(resp) if resp.status().is_success() => {
                let _facts: CompanyFacts = resp.json().await?;
                Ok(FundamentalsSnapshot {
                    symbol: symbol.to_uppercase(),
                    company_name: String::new(),
                    cik: String::new(),
                    industry: None,
                    currency: "USD".to_string(),
                    fiscal_year_end: None,
                    shares_outstanding: None,
                    market_cap: None,
                    net_income_usd: None,
                    revenues_usd: None,
                    assets_usd: None,
                    liabilities_usd: None,
                    stockholders_equity_usd: None,
                    cash_and_equivalents_usd: None,
                    gross_profit_usd: None,
                    operating_income_usd: None,
                    operating_expenses_usd: None,
                    operating_cash_flow_usd: None,
                    capital_expenditure_usd: None,
                    free_cash_flow_usd: None,
                    long_term_debt_usd: None,
                    current_debt_usd: None,
                    total_debt_usd: None,
                    diluted_shares_outstanding: None,
                })
            }
            _ => Err(Error::not_found(format!("SEC EDGAR: no data for {symbol}"))),
        }
    }
}
