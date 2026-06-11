#![allow(dead_code)]
//! Convertible bond data from JSL (集思录).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::Row;

#[derive(Debug, Deserialize)]
struct JslCbResponse {
    #[serde(default)]
    data: Vec<serde_json::Value>,
}

impl AkShareClient {
    /// JSL convertible bond list with pricing and premium data.
    ///
    /// Fetches all listed convertible bonds from jisilu.cn with current price,
    /// conversion value, premium ratio, remaining years, etc.
    pub async fn bond_cb_jsl(&self) -> Result<Vec<Row>> {
        let url = "https://www.jisilu.cn/data/cbnew/cb_list/?___jsl=LST___t=1630000000000";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .header("Referer", "https://www.jisilu.cn/data/cbnew/")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let rows = resp["rows"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let cell = row.get("cell").cloned().unwrap_or(row.clone());
            let mut r = Row::new();
            for (key, val) in cell.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(key.clone(), val.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        Ok(items)
    }

    /// JSL convertible bond index data.
    ///
    /// Returns the convertible bond market index from JSL.
    pub async fn bond_cb_index_jsl(&self) -> Result<Vec<Row>> {
        let url = "https://www.jisilu.cn/data/cbnew/cb_index/?___jsl=LST___t=1630000000000";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .header("Referer", "https://www.jisilu.cn/data/cbnew/")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let rows = resp["rows"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let cell = row.get("cell").cloned().unwrap_or(row.clone());
            let mut r = Row::new();
            for (key, val) in cell.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(key.clone(), val.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        Ok(items)
    }

    /// JSL convertible bond adjustment logs (下修记录).
    ///
    /// Returns the history of conversion price downward adjustments.
    pub async fn bond_cb_adj_logs_jsl(&self) -> Result<Vec<Row>> {
        let url = "https://www.jisilu.cn/data/cbnew/cb_adj/?___jsl=LST___t=1630000000000";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .header("Referer", "https://www.jisilu.cn/data/cbnew/")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let rows = resp["rows"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let cell = row.get("cell").cloned().unwrap_or(row.clone());
            let mut r = Row::new();
            for (key, val) in cell.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(key.clone(), val.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        Ok(items)
    }

    /// JSL convertible bond redemption data (强赎数据).
    ///
    /// Returns information about forced redemption of convertible bonds.
    pub async fn bond_cb_redeem_jsl(&self) -> Result<Vec<Row>> {
        let url = "https://www.jisilu.cn/data/cbnew/cb_redeem/?___jsl=LST___t=1630000000000";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .header("Referer", "https://www.jisilu.cn/data/cbnew/")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let rows = resp["rows"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let cell = row.get("cell").cloned().unwrap_or(row.clone());
            let mut r = Row::new();
            for (key, val) in cell.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(key.clone(), val.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        Ok(items)
    }
}
