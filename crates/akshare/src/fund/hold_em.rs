//! Fund holder structure data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundHolderStructureItem;

impl AkShareClient {
    /// Fetch fund holder structure (Python: fund_hold_structure).
    pub async fn fund_hold_structure(&self) -> Result<Vec<FundHolderStructureItem>> {
        let response = crate::util::send_and_check(
            self.get("https://fund.eastmoney.com/data/FundDataPortfolio_Interface.aspx")
                .query(&[
                    ("dt", "11"),
                    ("pi", "1"),
                    ("pn", "50"),
                    ("mc", "hypzDetail"),
                    ("st", "desc"),
                    ("sc", "reportdate"),
                ]),
        )
        .await?;

        let text = response.text().await.map_err(Error::from)?;
        let json_start = text.find('{').unwrap_or(0);
        let json_end = text.rfind('}').map_or(text.len(), |i| i + 1);
        let json_str = &text[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("holder structure JSON parse: {e}")))?;

        let data = root
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| Error::not_found("no holder structure data"))?;

        let mut result = Vec::new();
        for (i, item) in data.iter().enumerate() {
            let Some(arr) = item.as_array() else { continue };
            if arr.len() < 7 {
                continue;
            }
            result.push(FundHolderStructureItem {
                rank: (i + 1) as i32,
                report_date: arr[0].as_str().unwrap_or("").to_string(),
                fund_count: arr[1].as_str().unwrap_or("0").parse().unwrap_or(0),
                institution_ratio: arr[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                individual_ratio: arr[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                internal_ratio: arr[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                total_shares: arr[5]
                    .as_str()
                    .unwrap_or("0")
                    .replace(',', "")
                    .parse()
                    .unwrap_or(0.0),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found("no holder structure data"));
        }
        Ok(result)
    }
}
