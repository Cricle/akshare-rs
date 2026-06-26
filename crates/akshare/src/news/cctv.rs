//! CCTV news (央视新闻) and Baidu economic news data.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;

impl AkShareClient {
    /// CCTV news (latest).
    ///
    /// The original `api.cctv.cn` endpoint is no longer available.
    /// Falls back to `news.cctv.com/data/index.json` which returns the
    /// latest CCTV news headlines. The `date` parameter is accepted for
    /// API compatibility but ignored by the upstream.
    pub async fn news_cctv(&self, _date: &str) -> Result<Vec<Row>> {
        let url = "https://news.cctv.com/data/index.json";
        let body: serde_json::Value = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .json()
            .await?;

        let data = body["rollData"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for entry in &data {
            let mut row = Row::new();
            row.insert(
                "title".into(),
                entry.get("title").cloned().unwrap_or_default(),
            );
            row.insert("url".into(), entry.get("url").cloned().unwrap_or_default());
            row.insert(
                "time".into(),
                entry.get("dateTime").cloned().unwrap_or_default(),
            );
            row.insert(
                "brief".into(),
                entry.get("brief").cloned().unwrap_or_default(),
            );
            items.push(row);
        }
        if items.is_empty() {
            return Err(Error::not_found("cctv news: no data returned"));
        }
        Ok(items)
    }

    /// Baidu economic calendar news.
    ///
    /// `symbol`: event name filter (e.g., "中国", "美国")
    pub async fn news_economic(&self, symbol: &str) -> Result<Vec<Row>> {
        let url = "https://gushitong.baidu.com/opendata";
        let body = self
            .get(url)
            .query(&[
                ("resource_id", "5352"),
                ("query", symbol),
                ("code", "type"),
                ("name", "economic_calendar"),
                ("pn", "0"),
                ("rn", "100"),
                ("finClientType", "pc"),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let data = resp["Result"]
            .as_array()
            .cloned()
            .or_else(|| resp["data"].as_array().cloned())
            .unwrap_or_default();

        let mut items = Vec::new();
        let empty_map = serde_json::Map::new();
        for entry in &data {
            let rows = entry["DisplayData"]["resultData"]["tplData"]["result"]["rows"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            for row_val in &rows {
                let mut row = Row::new();
                for (k, v) in row_val.as_object().unwrap_or(&empty_map) {
                    row.insert(k.clone(), v.clone());
                }
                if !row.is_empty() {
                    items.push(row);
                }
            }
        }
        Ok(items)
    }

    /// Baidu report time data for a given stock symbol.
    ///
    /// `symbol`: stock code (e.g., "600000")
    pub async fn news_report_time(&self, symbol: &str) -> Result<Vec<Row>> {
        let url = "https://gushitong.baidu.com/opendata";
        let body = self
            .get(url)
            .query(&[
                ("resource_id", "5352"),
                ("query", symbol),
                ("code", symbol),
                ("name", "report_time"),
                ("pn", "0"),
                ("rn", "100"),
                ("finClientType", "pc"),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let data = resp["Result"]
            .as_array()
            .cloned()
            .or_else(|| resp["data"].as_array().cloned())
            .unwrap_or_default();

        let mut items = Vec::new();
        let empty_map = serde_json::Map::new();
        for entry in &data {
            let rows = entry["DisplayData"]["resultData"]["tplData"]["result"]["rows"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            for row_val in &rows {
                let mut row = Row::new();
                for (k, v) in row_val.as_object().unwrap_or(&empty_map) {
                    row.insert(k.clone(), v.clone());
                }
                if !row.is_empty() {
                    items.push(row);
                }
            }
        }
        Ok(items)
    }

    /// Baidu dividend notification data.
    ///
    /// `date`: format YYYYMMDD
    pub async fn news_trade_notify_dividend(&self, date: &str) -> Result<Vec<Row>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://gushitong.baidu.com/opendata";
        let body = self
            .get(url)
            .query(&[
                ("resource_id", "5352"),
                ("query", "分红"),
                ("code", "type"),
                ("name", "trade_notify"),
                ("date", date_fmt.as_str()),
                ("pn", "0"),
                ("rn", "100"),
                ("finClientType", "pc"),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let data = resp["Result"]
            .as_array()
            .cloned()
            .or_else(|| resp["data"].as_array().cloned())
            .unwrap_or_default();

        let mut items = Vec::new();
        let empty_map = serde_json::Map::new();
        for entry in &data {
            let rows = entry["DisplayData"]["resultData"]["tplData"]["result"]["rows"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            for row_val in &rows {
                let mut row = Row::new();
                for (k, v) in row_val.as_object().unwrap_or(&empty_map) {
                    row.insert(k.clone(), v.clone());
                }
                if !row.is_empty() {
                    items.push(row);
                }
            }
        }
        Ok(items)
    }

    /// Baidu stock suspension notification data.
    ///
    /// `date`: format YYYYMMDD
    pub async fn news_trade_notify_suspend(&self, date: &str) -> Result<Vec<Row>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://gushitong.baidu.com/opendata";
        let body = self
            .get(url)
            .query(&[
                ("resource_id", "5352"),
                ("query", "停牌"),
                ("code", "type"),
                ("name", "trade_notify"),
                ("date", date_fmt.as_str()),
                ("pn", "0"),
                ("rn", "100"),
                ("finClientType", "pc"),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let data = resp["Result"]
            .as_array()
            .cloned()
            .or_else(|| resp["data"].as_array().cloned())
            .unwrap_or_default();

        let mut items = Vec::new();
        let empty_map = serde_json::Map::new();
        for entry in &data {
            let rows = entry["DisplayData"]["resultData"]["tplData"]["result"]["rows"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            for row_val in &rows {
                let mut row = Row::new();
                for (k, v) in row_val.as_object().unwrap_or(&empty_map) {
                    row.insert(k.clone(), v.clone());
                }
                if !row.is_empty() {
                    items.push(row);
                }
            }
        }
        Ok(items)
    }
}
