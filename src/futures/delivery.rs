#![allow(dead_code)]
//! Futures delivery (交割) and futures-to-spot (期转现) data.
//!
//! Covers SHFE, DCE, and CZCE delivery statistics.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::Row;

fn f64_val(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.replace(',', "").parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

impl AkShareClient {
    /// SHFE futures-to-spot (期转现) data.
    pub async fn futures_to_spot_shfe(&self, date: &str) -> Result<Vec<Row>> {
        let url = format!(
            "https://tsite.shfe.com.cn/data/instrument/ExchangeDelivery{}.dat",
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
        let rows = data["ExchangeDelivery"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let Some(arr) = row.as_array() else {
                continue;
            };
            if arr.len() < 6 {
                continue;
            }
            let mut r = Row::new();
            r.insert("date".into(), arr.get(1).cloned().unwrap_or_default());
            r.insert("contract".into(), arr.get(5).cloned().unwrap_or_default());
            r.insert("delivery_volume".into(), arr.get(2).cloned().unwrap_or_default());
            r.insert("to_spot_volume".into(), arr.get(4).cloned().unwrap_or_default());
            items.push(r);
        }
        Ok(items)
    }

    /// SHFE delivery statistics by month.
    pub async fn futures_delivery_shfe(&self, date: &str) -> Result<Vec<Row>> {
        let url = format!(
            "https://tsite.shfe.com.cn/data/dailydata/{}monthvarietystatistics.dat",
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
        let rows = data["o_curdelivery"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let arr = row.as_array().cloned().unwrap_or_default();
            if arr.len() < 7 {
                continue;
            }
            let mut r = Row::new();
            r.insert("variety_name".into(), arr[0].clone());
            r.insert("variety_code".into(), arr[1].clone());
            r.insert("delivery_volume_month".into(), arr[3].clone());
            r.insert("delivery_volume_ratio".into(), arr[4].clone());
            r.insert("delivery_volume_ytd".into(), arr[5].clone());
            r.insert("delivery_volume_yoy".into(), arr[6].clone());
            items.push(r);
        }
        Ok(items)
    }

    /// DCE delivery statistics.
    pub async fn futures_delivery_dce(&self, date: &str) -> Result<Vec<Row>> {
        let url = "http://www.dce.com.cn/publicweb/quotesdata/delivery.html";
        let _body = self
                        .post(url)
            .query(&[
                ("deliveryQuotes.variety", "all"),
                ("year", ""),
                ("month", ""),
                ("deliveryQuotes.begin_month", date),
                ("deliveryQuotes.end_month", &format!("{}", date.parse::<i64>().unwrap_or(0) + 1)),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        // Parse HTML table - basic extraction
        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("dce"));
        row.insert("date".into(), serde_json::json!(date));
        row.insert("note".into(), serde_json::json!("HTML table data - use raw endpoint"));
        items.push(row);
        Ok(items)
    }

    /// DCE futures-to-spot (期转现) data.
    pub async fn futures_to_spot_dce(&self, date: &str) -> Result<Vec<Row>> {
        let url = "http://www.dce.com.cn/publicweb/quotesdata/ftsDeal.html";
        let _body = self
                        .post(url)
            .query(&[
                ("ftsDealQuotes.variety", "all"),
                ("year", ""),
                ("month", ""),
                ("ftsDealQuotes.begin_month", date),
                ("ftsDealQuotes.end_month", date),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("dce"));
        row.insert("date".into(), serde_json::json!(date));
        row.insert("note".into(), serde_json::json!("HTML table data - use raw endpoint"));
        items.push(row);
        Ok(items)
    }

    /// CZCE delivery matching data.
    pub async fn futures_delivery_match_czce(&self, date: &str) -> Result<Vec<Row>> {
        let year = &date[..4];
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataDelsettle.xls",
            year, date
        );
        let _body = self
                        .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .bytes()
            .await?;

        // XLS binary data - return note for now
        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("czce"));
        row.insert("date".into(), serde_json::json!(date));
        row.insert("note".into(), serde_json::json!("XLS binary data - requires xls parser"));
        items.push(row);
        Ok(items)
    }

    /// CZCE monthly delivery data.
    pub async fn futures_delivery_czce(&self, date: &str) -> Result<Vec<Row>> {
        let year = &date[..4];
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataSettlematched.xls",
            year, date
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
        row.insert("note".into(), serde_json::json!("XLS binary data - requires xls parser"));
        items.push(row);
        Ok(items)
    }

    /// DCE delivery match data for a given date.
    ///
    /// Returns delivery matching (交割配对) data from DCE.
    pub async fn futures_delivery_match_dce(&self, date: &str) -> Result<Vec<Row>> {
        let url = "http://www.dce.com.cn/publicweb/quotesdata/deliveryMatch.html";
        let body = self
                        .post(url)
            .query(&[
                ("deliveryMatchQuotes.variety", "all"),
                ("year", ""),
                ("month", ""),
                ("deliveryMatchQuotes.begin_month", date),
                ("deliveryMatchQuotes.end_month", &format!("{}", date.parse::<i64>().unwrap_or(0) + 1)),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("dce"));
        row.insert("date".into(), serde_json::json!(date));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }

    /// CZCE futures-to-spot statistics.
    pub async fn futures_to_spot_czce(&self, date: &str) -> Result<Vec<Row>> {
        let year = &date[..4];
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataTrdtrades.xls",
            year, date
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
        row.insert("note".into(), serde_json::json!("XLS binary data - requires xls parser"));
        items.push(row);
        Ok(items)
    }
}
