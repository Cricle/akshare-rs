//! Futures commission (手续费) information.
//!
//! Sources: openctp, Jin10 (金十数据), 9qihuo (九期网)

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;
use crate::util::{parse_html_tables, send_and_check_text};

impl AkShareClient {
    /// openctp futures fee reference table.
    pub async fn futures_fees_info_openctp(&self) -> Result<Vec<Row>> {
        let body = send_and_check_text(
            self.get("http://openctp.cn/fees.html")
                .header("User-Agent", "Mozilla/5.0"),
            None,
        )
        .await?;
        let table = parse_html_tables(&body)
            .into_iter()
            .find(|table| {
                table
                    .iter()
                    .any(|row| row.iter().any(|cell| cell.trim() == "合约代码"))
            })
            .ok_or_else(|| Error::decode("openctp fee table not found"))?;
        let header_index = table
            .iter()
            .position(|row| row.iter().any(|cell| cell.trim() == "合约代码"))
            .ok_or_else(|| Error::decode("openctp fee header not found"))?;
        let headers = &table[header_index];
        let mut result = Vec::new();
        for row in table.iter().skip(header_index + 1) {
            if row.iter().all(|cell| cell.trim().is_empty()) {
                continue;
            }
            let mut out = Row::new();
            for (index, value) in row.iter().enumerate() {
                let key = headers
                    .get(index)
                    .map(String::as_str)
                    .filter(|key| !key.trim().is_empty())
                    .unwrap_or("");
                if !key.is_empty() {
                    out.insert(key.to_string(), serde_json::json!(value.trim()));
                }
            }
            if !out.is_empty() {
                result.push(out);
            }
        }
        if result.is_empty() {
            return Err(Error::not_found("openctp fee table returned no rows"));
        }
        Ok(result)
    }

    /// Jin10 futures commission data.
    ///
    /// `date`: format YYYYMMDD
    pub async fn futures_comm_js(&self, date: &str) -> Result<Vec<Row>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://mp-api.jin10.com/api/dynamic-data/child";
        let search = format!(r#"{{"range,date": "{date_fmt},{date_fmt}", "status": 1}}"#);

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
        Err(Error::unavailable(format!(
            "9qihuo fee page requires HTML table parsing for {symbol}"
        )))
    }

    /// 9qihuo futures commission data.
    ///
    /// `symbol`: exchange name or "所有"
    pub async fn futures_comm_info(&self, _symbol: &str) -> Result<Vec<Row>> {
        Err(Error::unavailable(
            "9qihuo fee page requires HTML table parsing",
        ))
    }
}
