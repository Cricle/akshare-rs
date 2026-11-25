//! Fund overview data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch a fund's basic profile from Eastmoney (Python: `fund_overview_em`).
    ///
    /// `symbol`: fund code, e.g. "015641".
    /// Returns a single object whose keys are the field labels published on
    /// `https://fundf10.eastmoney.com/jbgk_{symbol}.html`.
    pub async fn fund_overview_em(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let symbol = symbol.trim();
        if symbol.is_empty() {
            return Err(Error::invalid_input("fund code must not be empty"));
        }
        let url = format!("https://fundf10.eastmoney.com/jbgk_{symbol}.html");
        let text =
            crate::util::send_and_check_text(self.get(&url), Some(encoding_rs::GB18030)).await?;

        let tables = crate::fund::html_tables(&text);
        let table = tables
            .last()
            .ok_or_else(|| Error::not_found(format!("no overview table for {symbol}")))?;

        let mut object = serde_json::Map::new();
        for row in table {
            if row.len() >= 2 && !row[0].is_empty() {
                object.insert(row[0].clone(), serde_json::Value::String(row[1].clone()));
            }
            if row.len() >= 4 && !row[2].is_empty() {
                object.insert(row[2].clone(), serde_json::Value::String(row[3].clone()));
            }
        }

        if object.is_empty() {
            return Err(Error::not_found(format!(
                "eastmoney returned no overview fields for {symbol}"
            )));
        }
        Ok(vec![serde_json::Value::Object(object)])
    }

    /// Legacy placeholder: the upstream API requires a fund code.
    ///
    /// Use [`fund_overview_em`](Self::fund_overview_em) instead.
    pub async fn fund_overview(&self, _limit: usize) -> Result<Vec<serde_json::Value>> {
        Err(Error::invalid_input(
            "fund_overview requires a fund code; use fund_overview_em(symbol)",
        ))
    }
}
