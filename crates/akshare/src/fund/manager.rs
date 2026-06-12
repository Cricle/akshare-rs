//! Fund manager data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundManagerItem;

impl AkShareClient {
    /// Fetch fund manager list (Python: fund_manager_em).
    pub async fn fund_manager_em(&self) -> Result<Vec<FundManagerItem>> {
        let response = self
            .get("https://fund.eastmoney.com/Data/FundDataPortfolio_Interface.aspx")
            .query(&[
                ("dt", "14"),
                ("mc", "returnjson"),
                ("ft", "all"),
                ("pn", "500"),
                ("pi", "1"),
                ("sc", "abbname"),
                ("st", "asc"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        let json_str = text.strip_prefix("var returnjson= ").unwrap_or(&text);
        let json_start = json_str.find('{').unwrap_or(0);
        let json_end = json_str.rfind('}').map_or(json_str.len(), |i| i + 1);
        let json_body = &json_str[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_body)
            .map_err(|e| Error::decode(format!("manager JSON parse: {e}")))?;

        let data = root
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| Error::not_found("no manager data"))?;

        let mut result = Vec::new();
        for (i, item) in data.iter().enumerate() {
            let Some(arr) = item.as_array() else { continue };
            if arr.len() < 12 {
                continue;
            }

            let fund_codes_str = arr[5].as_str().unwrap_or("");
            let fund_names_str = arr[6].as_str().unwrap_or("");
            let fund_codes: Vec<&str> = fund_codes_str.split(',').collect();
            let fund_names: Vec<&str> = fund_names_str.split(',').collect();

            for (code, name) in fund_codes.iter().zip(fund_names.iter()) {
                result.push(FundManagerItem {
                    rank: (i + 1) as i32,
                    name: arr[2].as_str().unwrap_or("").to_string(),
                    company: arr[4].as_str().unwrap_or("").to_string(),
                    fund_code: code.to_string(),
                    fund_name: name.to_string(),
                    career_days: arr[7].as_str().unwrap_or("0").parse().unwrap_or(0),
                    total_scale: arr[11]
                        .as_str()
                        .unwrap_or("0")
                        .replace("亿元", "")
                        .parse()
                        .unwrap_or(0.0),
                    best_return: arr[8]
                        .as_str()
                        .unwrap_or("0")
                        .replace('%', "")
                        .parse()
                        .unwrap_or(0.0),
                });
            }
        }
        if result.is_empty() {
            return Err(Error::not_found("no manager data"));
        }
        Ok(result)
    }
}
