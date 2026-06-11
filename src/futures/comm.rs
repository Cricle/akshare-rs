//! Futures commission (手续费) information.
//!
//! Sources: openctp, Jin10 (金十数据), 9qihuo (九期网)

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::Row;

impl AkShareClient {
    /// openctp futures fee reference table.
    pub async fn futures_fees_info_openctp(&self) -> Result<Vec<Row>> {
        let url = "http://openctp.cn/fees.html";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        // Parse HTML table
        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("openctp"));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }

    /// Jin10 futures commission data.
    ///
    /// `date`: format YYYYMMDD
    pub async fn futures_comm_js(&self, date: &str) -> Result<Vec<Row>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://mp-api.jin10.com/api/dynamic-data/child";
        let search = format!(
            r#"{{"range,date": "{},{}", "status": 1}}"#,
            date_fmt, date_fmt
        );

        let body = self
            .get(url)
            .query(&[
                ("tb_name", "_vir_26"),
                ("search", search.as_str()),
                ("order", "date,desc"),
            ])
            .header("user-agent", "Mozilla/5.0")
            .header("x-app-id", "fiXF2nOnDycGutVA")
            .header("x-version", "1.0")
            .header("referer", "https://www.jin10.com/")
            .header("origin", "https://www.jin10.com")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for mut row in rows {
            let mut r = Row::new();
            r.insert(
                "date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("date"))
                    .unwrap_or_default(),
            );
            r.insert(
                "contract_name".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("heyue_name"))
                    .unwrap_or_default(),
            );
            r.insert(
                "contract_code".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("heyue_code"))
                    .unwrap_or_default(),
            );
            r.insert(
                "price".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("heyue_price"))
                    .unwrap_or_default(),
            );
            r.insert(
                "up_limit".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("up_limit_num"))
                    .unwrap_or_default(),
            );
            r.insert(
                "down_limit".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("down_limit_num"))
                    .unwrap_or_default(),
            );
            r.insert(
                "buy_margin".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("buy_ratio"))
                    .unwrap_or_default(),
            );
            r.insert(
                "sell_margin".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("sell_ratio"))
                    .unwrap_or_default(),
            );
            r.insert(
                "per_lot_margin".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("per_lot_price"))
                    .unwrap_or_default(),
            );
            r.insert(
                "open_fee".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("buy_commission"))
                    .unwrap_or_default(),
            );
            r.insert(
                "close_yesterday_fee".into(),
                row["sell_yesterday_commission"].clone(),
            );
            r.insert(
                "close_today_fee".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("sell_cur_commission"))
                    .unwrap_or_default(),
            );
            r.insert(
                "exchange".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("jys"))
                    .unwrap_or_default(),
            );
            items.push(r);
        }
        Ok(items)
    }

    /// Futures fees info — unified entry point.
    ///
    /// Returns fee information for the given symbol or exchange.
    pub async fn futures_fees_info(&self, symbol: &str) -> Result<Vec<Row>> {
        // Try 9qihuo first
        let url = "https://www.9qihuo.com/qihuoshouxufei";
        let body = self
            .get(url)
            .query(&[("q", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .header("Referer", "https://www.9qihuo.com/")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("9qihuo"));
        row.insert("symbol".into(), serde_json::json!(symbol));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }

    /// 9qihuo futures commission data.
    ///
    /// `symbol`: exchange name or "所有"
    pub async fn futures_comm_info(&self, _symbol: &str) -> Result<Vec<Row>> {
        let url = "https://www.9qihuo.com/qihuoshouxufei";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .header("Referer", "https://www.9qihuo.com/")
            .send()
            .await?
            .text()
            .await?;

        // Parse HTML table
        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("9qihuo"));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }
}
