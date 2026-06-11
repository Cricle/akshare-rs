//! Fund split data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundSplitItem;

impl AkShareClient {
    /// Fetch fund split data (Python: fund_cf_em).
    ///
    /// `year`: query year (e.g. "2025").
    pub async fn fund_cf_em(&self, year: &str) -> Result<Vec<FundSplitItem>> {
        let response = self
            .get("https://fund.eastmoney.com/Data/funddataIndex_Interface.aspx")
            .query(&[
                ("dt", "9"), ("page", "1"), ("rank", "FSRQ"), ("sort", "desc"),
                ("gs", ""), ("ftype", ""), ("year", year),
            ])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        // Parse JS response: [[...],[...]]
        let start = text.find("[[").unwrap_or(0);
        let end_marker = text.find(";var jjcf_jjgs").unwrap_or(text.len());
        if start >= end_marker {
            return Ok(Vec::new());
        }
        let json_str = &text[start..end_marker];

        let data: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("fund split JSON parse: {e}")))?;

        let mut result = Vec::new();
        for (i, row) in data.iter().enumerate() {
            if row.len() < 6 { continue; }
            result.push(FundSplitItem {
                rank: (i + 1) as i32,
                fund_code: row[0].as_str().unwrap_or("").to_string(),
                fund_name: row[1].as_str().unwrap_or("").to_string(),
                split_date: row[2].as_str().unwrap_or("").to_string(),
                split_type: row[3].as_str().unwrap_or("").to_string(),
                split_ratio: row[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            });
        }
        Ok(result)
    }
}
