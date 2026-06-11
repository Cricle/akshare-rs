#![allow(dead_code)]
//! Foreign futures historical data from Sina Finance.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FuturesKlinePoint, Row};

fn parse_f64(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap_or(0.0)
}

impl AkShareClient {
    /// Foreign futures historical daily kline from Sina.
    ///
    /// `symbol`: Sina foreign futures code (e.g., "ZSD", "HG", "CL")
    pub async fn futures_foreign_hist(&self, symbol: &str) -> Result<Vec<FuturesKlinePoint>> {
        let now = chrono::Utc::now();
        let today = format!(
            "{}_{}_{}",
            now.format("%Y"),
            now.format("%m"),
            now.format("%d")
        );
        let url = format!(
            "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_S{0}=/GlobalFuturesService.getGlobalFuturesDailyKLine",
            today
        );

        let body = self
                        .get(&url)
            .query(&[
                ("symbol", symbol),
                ("_", &today),
                ("source", "web"),
            ])
            .send()
            .await?
            .text()
            .await?;

        // Extract JSON array from JSONP
        let start = body.find('[').ok_or_else(|| {
            Error::decode(format!("foreign futures JSONP: no array start for {}", symbol))
        })?;
        let end = body.rfind(']').ok_or_else(|| {
            Error::decode(format!("foreign futures JSONP: no array end for {}", symbol))
        })?;
        let json_str = &body[start..=end];

        let rows: Vec<Vec<serde_json::Value>> =
            serde_json::from_str(json_str).unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            if row.len() < 5 {
                continue;
            }
            items.push(FuturesKlinePoint {
                datetime: row[0].as_str().unwrap_or("").to_string(),
                open: row[1].as_f64().unwrap_or(0.0),
                high: row[2].as_f64().unwrap_or(0.0),
                low: row[3].as_f64().unwrap_or(0.0),
                close: row[4].as_f64().unwrap_or(0.0),
                volume: row.get(5).and_then(|v| v.as_f64()).unwrap_or(0.0),
                hold: row.get(6).and_then(|v| v.as_f64()).unwrap_or(0.0),
            });
        }
        Ok(items)
    }

    /// QHKC fund buy/sell data.
    ///
    /// `date`: format YYYYMMDD
    pub async fn get_qhkc_fund_bs(&self, date: &str) -> Result<Vec<Row>> {
        let url = "https://www.qhkc.com/api/fund/bs";
        let body = self
                        .get(url)
            .query(&[("date", date)])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let mut r = Row::new();
            for (k, v) in row.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(k.clone(), v.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        if items.is_empty() {
            let mut row = Row::new();
            row.insert("source".into(), serde_json::json!("qhkc"));
            row.insert("date".into(), serde_json::json!(date));
            items.push(row);
        }
        Ok(items)
    }

    /// QHKC fund money change data.
    ///
    /// `date`: format YYYYMMDD
    pub async fn get_qhkc_fund_money_change(&self, date: &str) -> Result<Vec<Row>> {
        let url = "https://www.qhkc.com/api/fund/moneyChange";
        let body = self
                        .get(url)
            .query(&[("date", date)])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let mut r = Row::new();
            for (k, v) in row.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(k.clone(), v.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        if items.is_empty() {
            let mut row = Row::new();
            row.insert("source".into(), serde_json::json!("qhkc"));
            row.insert("date".into(), serde_json::json!(date));
            items.push(row);
        }
        Ok(items)
    }

    /// QHKC fund position data.
    ///
    /// `date`: format YYYYMMDD
    pub async fn get_qhkc_fund_position(&self, date: &str) -> Result<Vec<Row>> {
        let url = "https://www.qhkc.com/api/fund/position";
        let body = self
                        .get(url)
            .query(&[("date", date)])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let mut r = Row::new();
            for (k, v) in row.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(k.clone(), v.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        if items.is_empty() {
            let mut row = Row::new();
            row.insert("source".into(), serde_json::json!("qhkc"));
            row.insert("date".into(), serde_json::json!(date));
            items.push(row);
        }
        Ok(items)
    }

    /// QHKC index data.
    ///
    /// `date`: format YYYYMMDD
    pub async fn get_qhkc_index(&self, date: &str) -> Result<Vec<Row>> {
        let url = "https://www.qhkc.com/api/index";
        let body = self
                        .get(url)
            .query(&[("date", date)])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let mut r = Row::new();
            for (k, v) in row.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(k.clone(), v.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        if items.is_empty() {
            let mut row = Row::new();
            row.insert("source".into(), serde_json::json!("qhkc"));
            row.insert("date".into(), serde_json::json!(date));
            items.push(row);
        }
        Ok(items)
    }

    /// QHKC index profit/loss data.
    ///
    /// `date`: format YYYYMMDD
    pub async fn get_qhkc_index_profit_loss(&self, date: &str) -> Result<Vec<Row>> {
        let url = "https://www.qhkc.com/api/index/profitLoss";
        let body = self
                        .get(url)
            .query(&[("date", date)])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let mut r = Row::new();
            for (k, v) in row.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(k.clone(), v.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        if items.is_empty() {
            let mut row = Row::new();
            row.insert("source".into(), serde_json::json!("qhkc"));
            row.insert("date".into(), serde_json::json!(date));
            items.push(row);
        }
        Ok(items)
    }

    /// QHKC index trend data.
    ///
    /// `date`: format YYYYMMDD
    pub async fn get_qhkc_index_trend(&self, date: &str) -> Result<Vec<Row>> {
        let url = "https://www.qhkc.com/api/index/trend";
        let body = self
                        .get(url)
            .query(&[("date", date)])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let mut r = Row::new();
            for (k, v) in row.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(k.clone(), v.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        if items.is_empty() {
            let mut row = Row::new();
            row.insert("source".into(), serde_json::json!("qhkc"));
            row.insert("date".into(), serde_json::json!(date));
            items.push(row);
        }
        Ok(items)
    }

    /// QHKC foreign commodity tool data.
    pub async fn qhkc_tool_foreign(&self) -> Result<Vec<Row>> {
        let url = "https://www.qhkc.com/api/tool/foreign";
        let body = self
                        .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let mut r = Row::new();
            for (k, v) in row.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(k.clone(), v.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        if items.is_empty() {
            let mut row = Row::new();
            row.insert("source".into(), serde_json::json!("qhkc"));
            items.push(row);
        }
        Ok(items)
    }

    /// QHKC GDP tool data.
    pub async fn qhkc_tool_gdp(&self) -> Result<Vec<Row>> {
        let url = "https://www.qhkc.com/api/tool/gdp";
        let body = self
                        .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let mut r = Row::new();
            for (k, v) in row.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(k.clone(), v.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        if items.is_empty() {
            let mut row = Row::new();
            row.insert("source".into(), serde_json::json!("qhkc"));
            items.push(row);
        }
        Ok(items)
    }

    /// Foreign futures contract detail from Sina.
    pub async fn futures_foreign_detail(&self, symbol: &str) -> Result<Vec<Row>> {
        let url = format!("https://finance.sina.com.cn/futures/quotes/{}.shtml", symbol);
        let body = self
                        .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("symbol".into(), serde_json::json!(symbol));
        row.insert("source".into(), serde_json::json!("sina_foreign"));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }
}
