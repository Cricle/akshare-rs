//! Banking regulatory penalty data from NFRA (国家金融监督管理总局).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct CbircResponse {
    data: Option<CbircData>,
}

#[derive(Debug, Deserialize)]
struct CbircData {
    #[serde(default)]
    total: u64,
    #[serde(default)]
    rows: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// NFRA banking penalty table details.
    ///
    /// Fetches administrative penalty records from the National Financial
    /// Regulatory Administration (NFRA, formerly CBIRC).
    ///
    /// `item`: "机关" (headquarters), "本级" (directly under), "分局本级" (branch level)
    pub async fn bank_fjcf_table_detail(&self, item: &str) -> Result<Vec<MacroDataPoint>> {
        let item_id = match item {
            "机关" => "4113",
            "本级" => "4114",
            "分局本级" => "4115",
            _ => return Err(Error::invalid_input(format!("unknown item type: {}", item))),
        };

        let url = "https://www.nfra.gov.cn/cbircweb/DocInfo/SelectDocByItemIdAndChild";
        let mut all_items = Vec::new();
        let mut page = 1;
        let page_size = 18;

        loop {
            let resp: CbircResponse = self
                .get(url)
                .query(&[
                    ("itemId", item_id),
                    ("pageSize", &page_size.to_string()),
                    ("pageIndex", &page.to_string()),
                ])
                .header("User-Agent", "Mozilla/5.0 (compatible; akshare-rust/0.1)")
                .send()
                .await?
                .json()
                .await?;

            let data = resp.data.unwrap_or(CbircData {
                total: 0,
                rows: Vec::new(),
            });

            let total = data.total;
            let rows = data.rows;

            if rows.is_empty() {
                break;
            }

            for row in &rows {
                let title = row
                    .get("docTitle")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let pub_date = row
                    .get("publishDate")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let _doc_id = row
                    .get("docId")
                    .and_then(|v| v.as_str())
                    .or_else(|| row.get("docId").and_then(|v| v.as_i64()).map(|_| ""))
                    .unwrap_or("")
                    .to_string();

                if !title.is_empty() {
                    all_items.push(MacroDataPoint {
                        date: pub_date.get(..10).unwrap_or(&pub_date).to_string(),
                        value: 0.0,
                        name: title,
                    });
                }
            }

            let total_pages = (total as f64 / page_size as f64).ceil() as u64;
            if page as u64 >= total_pages {
                break;
            }
            page += 1;

            // Safety limit
            if page > 100 {
                break;
            }
        }

        Ok(all_items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cbirc_response_structure() {
        let json = r#"{
            "data": {
                "total": 2,
                "rows": [
                    {
                        "docTitle": "Test Penalty 1",
                        "publishDate": "2024-01-01 00:00:00",
                        "docId": "12345"
                    },
                    {
                        "docTitle": "Test Penalty 2",
                        "publishDate": "2024-01-02 00:00:00",
                        "docId": "12346"
                    }
                ]
            }
        }"#;
        let resp: CbircResponse = serde_json::from_str(json).unwrap();
        let data = resp.data.unwrap();
        assert_eq!(data.total, 2);
        assert_eq!(data.rows.len(), 2);
        assert_eq!(
            data.rows[0].get("docTitle").and_then(|v| v.as_str()),
            Some("Test Penalty 1")
        );
    }
}
