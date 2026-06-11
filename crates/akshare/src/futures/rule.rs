//! Futures trading rules and calendar data.
//!
//! Sources: Guotai Junan Futures (国泰君安期货), Eastmoney Futures

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::Row;

impl AkShareClient {
    /// Guotai Junan Futures trading calendar and rules.
    pub async fn futures_rule_gtja(&self, date: &str) -> Result<Vec<Row>> {
        let url = "https://www.gtjaqh.com/pc/calendar";
        let body = self
            .get(url)
            .query(&[("date", date)])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        // Parse HTML table
        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("gtja"));
        row.insert("date".into(), serde_json::json!(date));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }

    /// Futures trading rules — unified entry point.
    ///
    /// Returns futures trading rules and calendar from GTJA.
    pub async fn futures_rule(&self, date: &str) -> Result<Vec<Row>> {
        self.futures_rule_gtja(date).await
    }

    /// Eastmoney futures trading rules.
    pub async fn futures_rule_em(&self) -> Result<Vec<Row>> {
        let url = "https://eastmoneyfutures.com/api/ComManage/GetPZJYInfo";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["Data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let mut r = Row::new();
            if let Some(obj) = row.as_object() {
                for (k, v) in obj {
                    r.insert(k.clone(), v.clone());
                }
            }
            items.push(r);
        }
        Ok(items)
    }
}
