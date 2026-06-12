#![allow(dead_code)]
//! CCTV news (央视新闻) and Baidu economic news data.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::Row;

#[derive(Debug, Deserialize)]
struct CctvNewsResponse {
    #[serde(default)]
    data: Vec<serde_json::Value>,
}

impl AkShareClient {
    /// CCTV news for a given date.
    ///
    /// `date`: format YYYYMMDD
    /// Returns news items from the CCTV finance channel.
    pub async fn news_cctv(&self, date: &str) -> Result<Vec<Row>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://api.cctv.cn/getNewsList";
        let body = self
            .get(url)
            .query(&[
                ("serviceId", "finance"),
                ("date", date_fmt.as_str()),
                ("type", "1"),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let data = resp["data"]["list"].as_array().cloned().unwrap_or_default();

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
                entry.get("time").cloned().unwrap_or_default(),
            );
            row.insert(
                "brief".into(),
                entry.get("brief").cloned().unwrap_or_default(),
            );
            items.push(row);
        }
        Ok(items)
    }

    /// Baidu economic calendar news.
    ///
    /// `symbol`: event name filter (e.g., "中国", "美国")
    pub async fn news_economic_baidu(&self, symbol: &str) -> Result<Vec<Row>> {
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
    pub async fn news_report_time_baidu(&self, symbol: &str) -> Result<Vec<Row>> {
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
    pub async fn news_trade_notify_dividend_baidu(&self, date: &str) -> Result<Vec<Row>> {
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
    pub async fn news_trade_notify_suspend_baidu(&self, date: &str) -> Result<Vec<Row>> {
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
