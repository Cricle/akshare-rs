//! Futures registered warehouse receipt (注册仓单) data.
//!
//! Covers DCE, SHFE, CZCE, and GFEX receipt data.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::Row;

fn parse_f64(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.replace(',', "").parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

impl AkShareClient {
    /// DCE registered warehouse receipt data.
    pub async fn get_dce_receipt(&self, date: &str) -> Result<Vec<Row>> {
        let url = "http://www.dce.com.cn/dcereport/publicweb/dailystat/wbillWeeklyQuotes";
        let payload = serde_json::json!({
            "tradeDate": date,
            "varietyId": "all",
        });

        let body = self.post(url).json(&payload).send().await?.text().await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let entities = data["data"]["entityList"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for row in &entities {
            let variety = row["variety"].as_str().unwrap_or("");
            if variety.is_empty() {
                continue;
            }
            // Only include subtotals (品种小计)
            if !variety.ends_with("小计") {
                continue;
            }
            let var_name = &variety[..variety.len() - "小计".len()];
            let mut r = Row::new();
            r.insert("variety".into(), serde_json::json!(var_name));
            r.insert("receipt_qty".into(), row["wbillQty"].clone());
            r.insert("receipt_chg".into(), row["diff"].clone());
            r.insert("date".into(), serde_json::json!(date));
            items.push(r);
        }
        Ok(items)
    }

    /// SHFE registered warehouse receipt data.
    pub async fn get_shfe_receipt(&self, date: &str) -> Result<Vec<Row>> {
        let url =
            format!("https://www.shfe.com.cn/data/tradedata/future/dailydata/{date}dailystock.dat");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["o_cursor"].as_array().cloned().unwrap_or_default();

        // Aggregate by variety
        let mut variety_map: std::collections::HashMap<String, f64> =
            std::collections::HashMap::new();
        for row in &rows {
            let var_name = row["VARNAME"].as_str().unwrap_or("");
            let variety = var_name.split('$').next().unwrap_or("").to_string();
            if variety.is_empty() {
                continue;
            }
            let weight = parse_f64(&row["WRTWGHTS"]);
            *variety_map.entry(variety).or_insert(0.0) += weight;
        }

        let mut items = Vec::new();
        for (variety, total) in &variety_map {
            let mut r = Row::new();
            r.insert("variety".into(), serde_json::json!(variety));
            r.insert("receipt_qty".into(), serde_json::json!(total));
            r.insert("date".into(), serde_json::json!(date));
            items.push(r);
        }
        Ok(items)
    }
}
