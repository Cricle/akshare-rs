//! QDII fund data from Jisilu (集思录).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct JisiluResponse {
    rows: Option<Vec<JisiluRow>>,
}

#[derive(Debug, Deserialize)]
struct JisiluRow {
    cell: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Jisilu T+0 QDII - Asian market index funds.
    ///
    /// Returns QDII fund data for Asian market index tracking funds.
    /// `cookie` is optional for authenticated access.
    pub async fn qdii_a_index_jsl(&self, cookie: &str) -> Result<Vec<MacroDataPoint>> {
        self.fetch_qdii_jsl("A", cookie).await
    }

    /// Jisilu T+0 QDII - European/American market index funds.
    ///
    /// Returns QDII fund data for European/American market index tracking funds.
    pub async fn qdii_e_index_jsl(&self, cookie: &str) -> Result<Vec<MacroDataPoint>> {
        self.fetch_qdii_jsl("E", cookie).await
    }

    /// Jisilu T+0 QDII - European/American commodity funds.
    ///
    /// Returns QDII fund data for European/American commodity tracking funds.
    pub async fn qdii_e_comm_jsl(&self, cookie: &str) -> Result<Vec<MacroDataPoint>> {
        self.fetch_qdii_jsl("E", cookie).await
    }

    // Internal helper
    async fn fetch_qdii_jsl(
        &self,
        market: &str,
        cookie: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = format!(
            "https://www.jisilu.cn/data/qdii/qdii_list/{}",
            market
        );

        let mut req = self
            .get(&url)
            .query(&[
                ("___jsl", "LST___t=1728207798534"),
                ("rp", "22"),
            ]);

        if !cookie.is_empty() {
            req = req.header("Cookie", cookie);
        }

        let resp: JisiluResponse = req.send().await?.json().await?;

        let rows = resp.rows.unwrap_or_default();
        let mut items = Vec::new();

        for row in &rows {
            if let Some(cell) = &row.cell {
                let fund_id = cell
                    .get("fund_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let fund_name = cell
                    .get("fund_nm")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let price = cell
                    .get("price")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let nav = cell
                    .get("fund_nav")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let discount_rt = cell
                    .get("discount_rt")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.trim_end_matches('%').parse::<f64>().ok())
                    .unwrap_or(0.0);

                if !fund_id.is_empty() {
                    items.push(MacroDataPoint {
                        date: fund_id,
                        value: price,
                        name: fund_name,
                    });
                    // Also add NAV as a separate data point
                    items.push(MacroDataPoint {
                        date: format!("{}_nav", items.last().unwrap().date),
                        value: nav,
                        name: format!("{} NAV", items.last().unwrap().name),
                    });
                    // Add discount rate
                    items.push(MacroDataPoint {
                        date: format!(
                            "{}_discount",
                            items[items.len() - 2].date
                        ),
                        value: discount_rt,
                        name: format!("{} Discount%", items[items.len() - 2].name),
                    });
                }
            }
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_qdii_jsl_response_structure() {
        let json = r#"{
            "rows": [
                {
                    "cell": {
                        "fund_id": "513100",
                        "fund_nm": "纳指ETF",
                        "price": 1.234,
                        "fund_nav": 1.230,
                        "discount_rt": "0.33%"
                    }
                }
            ]
        }"#;
        let resp: super::JisiluResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.rows.as_ref().unwrap().len(), 1);
        let rows = resp.rows.unwrap();
        let cell = rows[0].cell.as_ref().unwrap();
        assert_eq!(cell.get("fund_id").unwrap().as_str().unwrap(), "513100");
    }
}
