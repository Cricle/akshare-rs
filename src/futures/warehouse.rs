#![allow(dead_code)]
//! Futures warehouse receipt (仓单) data from Chinese exchanges.
//!
//! Covers CZCE, DCE, SHFE, and GFEX warehouse receipt daily data.

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
    /// CZCE warehouse receipt data.
    pub async fn futures_warehouse_receipt_czce(&self, date: &str) -> Result<Vec<Row>> {
        let year = &date[..4];
        let ext = if date > "20251101" { "xlsx" } else { "xls" };
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataWhsheet.{}",
            year, date, ext
        );
        let _body = self
                        .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .bytes()
            .await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("czce"));
        row.insert("date".into(), serde_json::json!(date));
        row.insert("note".into(), serde_json::json!("XLS/XLSX binary - requires spreadsheet parser"));
        items.push(row);
        Ok(items)
    }

    /// DCE warehouse receipt data.
    pub async fn futures_warehouse_receipt_dce(&self, date: &str) -> Result<Vec<Row>> {
        let url = "http://www.dce.com.cn/dcereport/publicweb/dailystat/wbillWeeklyQuotes";
        let payload = serde_json::json!({
            "tradeDate": date,
            "varietyId": "all",
        });

        let body = self
                        .post(url)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let entities = data["data"]["entityList"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &entities {
            let variety = row["variety"].as_str().unwrap_or("");
            if variety.is_empty() {
                continue;
            }
            let mut r = Row::new();
            r.insert("variety_name".into(), serde_json::json!(variety));
            r.insert("variety_code".into(), row["varietyOrder"].clone());
            r.insert("warehouse".into(), row["whAbbr"].clone());
            r.insert("last_receipt_qty".into(), row["lastWbillQty"].clone());
            r.insert("receipt_qty".into(), row["wbillQty"].clone());
            r.insert("change".into(), row["diff"].clone());
            r.insert("date".into(), serde_json::json!(date));
            items.push(r);
        }
        Ok(items)
    }

    /// SHFE warehouse receipt data.
    pub async fn futures_shfe_warehouse_receipt(&self, date: &str) -> Result<Vec<Row>> {
        let url = format!(
            "https://www.shfe.com.cn/data/tradedata/future/dailydata/{}dailystock.dat",
            date
        );
        let body = self
                        .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["o_cursor"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let var_name = row["VARNAME"].as_str().unwrap_or("");
            if var_name.is_empty() {
                continue;
            }
            let mut r = Row::new();
            r.insert("variety_name".into(), serde_json::json!(var_name.split('$').next().unwrap_or("")));
            r.insert("registry_name".into(), serde_json::json!(row["REGNAME"].as_str().unwrap_or("").split('$').next().unwrap_or("")));
            r.insert("warehouse".into(), serde_json::json!(row["WHABBRNAME"].as_str().unwrap_or("").split('$').next().unwrap_or("")));
            r.insert("receipt_qty".into(), row["WRTWGHTS"].clone());
            r.insert("date".into(), serde_json::json!(date));
            items.push(r);
        }
        Ok(items)
    }

    /// GFEX warehouse receipt data.
    pub async fn futures_gfex_warehouse_receipt(&self, date: &str) -> Result<Vec<Row>> {
        let url = "http://www.gfex.com.cn/u/interfacesWebTdWbillWeeklyQuotes/loadList";
        let body = self
                        .post(url)
            .form(&[("gen_date", date)])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let variety_order = row["varietyOrder"].as_str().unwrap_or("");
            if variety_order.is_empty() {
                continue;
            }
            let mut r = Row::new();
            r.insert("symbol".into(), serde_json::json!(variety_order.to_uppercase()));
            r.insert("variety_name".into(), row["variety"].clone());
            r.insert("warehouse".into(), row["whAbbr"].clone());
            r.insert("last_receipt_qty".into(), row["lastWbillQty"].clone());
            r.insert("receipt_qty".into(), row["wbillQty"].clone());
            r.insert("change".into(), row["regWbillQty"].clone());
            r.insert("date".into(), serde_json::json!(date));
            items.push(r);
        }
        Ok(items)
    }
}
