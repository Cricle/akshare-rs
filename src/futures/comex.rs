//! COMEX inventory data from Eastmoney (东方财富网).

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;

impl AkShareClient {
    /// COMEX gold/silver inventory data from Eastmoney.
    ///
    /// `symbol`: "黄金" or "白银"
    pub async fn futures_comex_inventory(&self, symbol: &str) -> Result<Vec<Row>> {
        let indicator_id = match symbol {
            "黄金" => "EMI00069026",
            "白银" => "EMI00069027",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported COMEX symbol: {}",
                    symbol
                )));
            }
        };

        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let filter = format!(
            r#"(INDICATOR_ID1="{}")(@STORAGE_TON<>"NULL")"#,
            indicator_id
        );

        let mut all_items = Vec::new();
        let mut page = 1;

        loop {
            let body = self
                                .get(url)
                .query(&[
                    ("sortColumns", "REPORT_DATE"),
                    ("sortTypes", "-1"),
                    ("pageSize", "500"),
                    ("pageNumber", &page.to_string()),
                    ("reportName", "RPT_FUTUOPT_GOLDSIL"),
                    ("columns", "ALL"),
                    ("quoteColumns", ""),
                    ("source", "WEB"),
                    ("client", "WEB"),
                    ("filter", filter.as_str()),
                ])
                .send()
                .await?
                .text()
                .await?;

            let data: serde_json::Value = serde_json::from_str(&body)?;
            let total_pages = data["result"]["pages"].as_i64().unwrap_or(1);
            let rows = data["result"]["data"]
                .as_array()
                .cloned()
                .unwrap_or_default();

            for row in &rows {
                let mut r = Row::new();
                r.insert("date".into(), row["REPORT_DATE"].clone());
                r.insert(
                    format!("comex_{}_ton", symbol),
                    row["STORAGE_TON"].clone(),
                );
                r.insert(
                    format!("comex_{}_ounce", symbol),
                    row["STORAGE_OUNCE"].clone(),
                );
                all_items.push(r);
            }

            if page >= total_pages as usize {
                break;
            }
            page += 1;
        }

        Ok(all_items)
    }
}
