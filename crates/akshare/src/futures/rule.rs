//! Futures trading rules and calendar data.
//!
//! Sources: Guotai Junan Futures (国泰君安期货), Eastmoney Futures

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;
use crate::util::{parse_html_tables, send_and_check_text};

impl AkShareClient {
    /// Guotai Junan Futures trading calendar and rules.
    pub async fn futures_rule_gtja(&self, date: &str) -> Result<Vec<Row>> {
        if date.len() != 8 || !date.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Error::invalid_input("date must use YYYYMMDD"));
        }
        let body = send_and_check_text(
            self.get("https://www.gtjaqh.com/pc/calendar")
                .query(&[("date", date)])
                .header("User-Agent", "Mozilla/5.0"),
            Some(encoding_rs::GB18030),
        )
        .await?;
        let tables = parse_html_tables(&body);
        let table = tables
            .iter()
            .find(|table| !table.is_empty())
            .ok_or_else(|| Error::not_found("GTJA futures rule table not found"))?;
        let headers = table.first().cloned().unwrap_or_default();
        let mut result = Vec::new();
        for row in table.iter().skip(1) {
            if row.iter().all(|cell| cell.trim().is_empty()) {
                continue;
            }
            let mut out = Row::new();
            for (index, value) in row.iter().enumerate() {
                let key = headers
                    .get(index)
                    .filter(|header| !header.trim().is_empty())
                    .cloned()
                    .unwrap_or_else(|| format!("column_{index}"));
                out.insert(key, serde_json::json!(value.trim()));
            }
            out.insert("date".into(), serde_json::json!(date));
            result.push(out);
        }
        if result.is_empty() {
            return Err(Error::not_found("GTJA futures rule returned no rows"));
        }
        Ok(result)
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
