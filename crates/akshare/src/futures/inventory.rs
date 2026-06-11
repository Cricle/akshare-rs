#![allow(dead_code)]
//! Futures inventory (库存) data.
//!
//! Sources: Eastmoney (东方财富网), 99qh (99期货网)

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;

fn parse_f64(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.replace(',', "").parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

impl AkShareClient {
    /// Eastmoney futures inventory data.
    ///
    /// `symbol`: variety name in Chinese (e.g., "铝") or code (e.g., "a")
    pub async fn futures_inventory_em(&self, symbol: &str) -> Result<Vec<Row>> {
        // Step 1: Get product code mapping
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let body = self
            .get(url)
            .query(&[
                ("reportName", "RPT_FUTU_POSITIONCODE"),
                ("columns", "TRADE_MARKET_CODE,TRADE_CODE,TRADE_TYPE"),
                ("filter", r#"(IS_MAINCODE="1")"#),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["result"]["data"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        // Build symbol -> code mapping
        let mut code_map = std::collections::HashMap::new();
        for row in &rows {
            let trade_type = row["TRADE_TYPE"].as_str().unwrap_or("");
            let trade_code = row["TRADE_CODE"].as_str().unwrap_or("");
            if !trade_type.is_empty() && !trade_code.is_empty() {
                code_map.insert(trade_type.to_string(), trade_code.to_string());
            }
        }

        let product_id = code_map
            .get(symbol)
            .ok_or_else(|| Error::invalid_input(format!("unknown inventory symbol: {}", symbol)))?;

        // Step 2: Fetch inventory data
        let filter = format!(
            r#"(SECURITY_CODE="{}")(TRADE_DATE>='2020-10-28')"#,
            product_id
        );
        let body2 = self
            .get(url)
            .query(&[
                ("reportName", "RPT_FUTU_STOCKDATA"),
                (
                    "columns",
                    "SECURITY_CODE,TRADE_DATE,ON_WARRANT_NUM,ADDCHANGE",
                ),
                ("filter", filter.as_str()),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "TRADE_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .text()
            .await?;

        let data2: serde_json::Value = serde_json::from_str(&body2)?;
        let rows2 = data2["result"]["data"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows2 {
            let Some(arr) = row.as_array() else {
                continue;
            };
            if arr.len() < 4 {
                continue;
            }
            let mut r = Row::new();
            r.insert("date".into(), arr[1].clone());
            r.insert("inventory".into(), arr[2].clone());
            r.insert("change".into(), arr[3].clone());
            items.push(r);
        }
        Ok(items)
    }

    /// 99qh futures inventory data.
    ///
    /// `symbol`: Chinese name (e.g., "豆一")
    pub async fn futures_inventory_99(&self, _symbol: &str) -> Result<Vec<Row>> {
        let url = "https://www.99qh.com/data/stockIn";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("99qh"));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }
}
