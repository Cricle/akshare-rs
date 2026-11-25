//! Convertible bond info from Sina Finance.
//!
//! Mirrors Python `bond_cb_profile_sina` and `bond_cb_summary_sina`. Sina
//! serves server-rendered HTML tables; they are parsed with the shared
//! module-level table helper.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

fn pair_rows(rows: &[Vec<String>]) -> Vec<serde_json::Value> {
    let mut items = Vec::new();
    for row in rows {
        if row.len() >= 2 && !row[0].is_empty() {
            items.push(serde_json::json!({
                "item": row[0].clone(),
                "value": row[1].clone(),
            }));
        }
    }
    items
}

impl AkShareClient {
    /// Fetch convertible bond profile data from Sina Finance.
    ///
    /// `symbol` is the market-prefixed code, e.g. "sz128039".
    /// Returns an array of `{"item", "value"}` objects, matching the Python
    /// DataFrame columns.
    pub async fn bond_cb_profile(&self, symbol: &str) -> Result<serde_json::Value> {
        let symbol = symbol.trim();
        let url = format!("https://money.finance.sina.com.cn/bond/info/{symbol}.html");
        let text =
            crate::util::send_and_check_text(self.get(&url), Some(encoding_rs::GB18030)).await?;

        let table = crate::bond::html_tables(&text)
            .into_iter()
            .next()
            .ok_or_else(|| Error::not_found(format!("no profile table for {symbol}")))?;
        let items = pair_rows(&table);
        if items.is_empty() {
            return Err(Error::not_found(format!(
                "sina returned no profile fields for {symbol}"
            )));
        }
        Ok(serde_json::Value::Array(items))
    }

    /// Fetch convertible bond summary data from Sina Finance.
    ///
    /// `symbol` is the market-prefixed code, e.g. "sh155255".
    /// Returns an array of `{"item", "value"}` objects.
    pub async fn bond_cb_summary(&self, symbol: &str) -> Result<serde_json::Value> {
        let symbol = symbol.trim();
        let url = format!("https://money.finance.sina.com.cn/bond/quotes/{symbol}.html");
        let text =
            crate::util::send_and_check_text(self.get(&url), Some(encoding_rs::GB18030)).await?;

        let tables = crate::bond::html_tables(&text);
        let table = tables
            .iter()
            .find(|table| {
                table
                    .iter()
                    .any(|row| row.len() >= 6 && row.iter().any(|cell| cell.contains("债券类型")))
            })
            .ok_or_else(|| Error::not_found(format!("no summary table for {symbol}")))?;

        let mut items = Vec::new();
        for row in table {
            for start in [0usize, 2, 4] {
                if row.len() >= start + 2 && !row[start].is_empty() {
                    items.push(serde_json::json!({
                        "item": row[start].clone(),
                        "value": row[start + 1].clone(),
                    }));
                }
            }
        }
        if items.is_empty() {
            return Err(Error::not_found(format!(
                "sina returned no summary fields for {symbol}"
            )));
        }
        Ok(serde_json::Value::Array(items))
    }
}
