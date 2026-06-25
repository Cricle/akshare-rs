//! Shenwan (申万) industry index data.
//!
//! Fetches Shenwan Level-1 industry index candles and listing from Eastmoney.
//!
//! Shenwan indices use Eastmoney secid format with market prefix 90,
//! e.g. "90.801010" for 农林牧渔 (Agriculture).
//!
//! Data source: Eastmoney push2 API (klines + clist).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::wire::ClistResp;
use crate::types::{CandlePoint, IndexSnapshot};
use crate::util::today_iso;

#[derive(Debug, Deserialize)]
struct ClistItem {
    /// f12 — index code
    #[serde(rename = "f12")]
    code: Option<String>,
    /// f14 — index name
    #[serde(rename = "f14")]
    name: Option<String>,
    /// f2 — latest close / index value
    #[serde(rename = "f2")]
    close: Option<f64>,
    /// f3 — change percentage
    #[serde(rename = "f3")]
    change_pct: Option<f64>,
    /// f5 — volume
    #[serde(rename = "f5")]
    volume: Option<f64>,
    /// f6 — amount (turnover)
    #[serde(rename = "f6")]
    amount: Option<f64>,
}

// ---------------------------------------------------------------------------
// Well-known Shenwan Level-1 index codes
// ---------------------------------------------------------------------------

/// Eastmoney secid prefix for Shenwan sector indices.
const SW_SECID_PREFIX: &str = "90";

/// Shenwan Level-1 industry index codes (801xxx series).
///
/// This list is stable and covers the standard 31 Level-1 industries.
const SW_LEVEL1_CODES: &[&str] = &[
    "801010", "801020", "801030", "801040", "801050", "801060", "801080", "801110", "801120",
    "801130", "801140", "801150", "801160", "801170", "801180", "801200", "801210", "801230",
    "801710", "801720", "801730", "801740", "801750", "801760", "801770", "801780", "801790",
    "801880", "801890", "801950", "801960", "801970", "801980",
];

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Shenwan (申万) industry index daily candles.
    ///
    /// `symbol` is a Shenwan index code such as `"801010"` (农林牧渔),
    /// `"801180"` (房地产), etc.  The `90.` prefix is added automatically.
    ///
    /// Returns the most recent `limit` daily candle points (forward-adjusted).
    pub async fn sw_index_candles(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        let code = symbol.trim().to_uppercase();
        // Validate: must be 6 digits
        if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
            return Err(Error::invalid_input(format!(
                "invalid Shenwan index code: {symbol} (expected 6-digit code like 801010)"
            )));
        }
        let secid = format!("{SW_SECID_PREFIX}.{code}");
        self.eastmoney_klines(&secid, "qfq", limit).await
    }

    /// Shenwan first-level index info (申万一级行业信息).
    ///
    /// Returns list of all Shenwan Level-1 industry indices with codes and names.
    pub async fn sw_index_first_info(&self) -> Result<Vec<crate::types::Row>> {
        let mut items = Vec::new();
        for code in SW_LEVEL1_CODES {
            let mut row = crate::types::Row::new();
            row.insert("code".into(), serde_json::json!(code));
            row.insert("level".into(), serde_json::json!("1"));
            items.push(row);
        }
        Ok(items)
    }

    /// Shenwan second-level index info (申万二级行业信息).
    ///
    /// Returns list of Shenwan Level-2 industry indices.
    pub async fn sw_index_second_info(&self) -> Result<Vec<crate::types::Row>> {
        let url = "https://push2.eastmoney.com/api/qt/clist/get";
        let response = self
            .get(url)
            .query(&crate::util::eastmoney_clist_params("500", &[
                ("fid", "f3"),
                ("fs", "m:90+t:3"),
                ("fields", "f12,f14,f2,f3"),
            ]))
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&response)?;
        let diff = data["data"]["diff"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for mut row in diff {
            let mut r = crate::types::Row::new();
            r.insert(
                "code".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f12"))
                    .unwrap_or_default(),
            );
            r.insert(
                "name".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f14"))
                    .unwrap_or_default(),
            );
            r.insert(
                "close".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f2"))
                    .unwrap_or_default(),
            );
            r.insert(
                "change_pct".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f3"))
                    .unwrap_or_default(),
            );
            r.insert("level".into(), serde_json::json!("2"));
            items.push(r);
        }
        Ok(items)
    }

    /// Shenwan third-level index constituents (申万三级行业成分股).
    ///
    /// `symbol`: Shenwan Level-3 index code
    pub async fn sw_index_third_cons(&self, symbol: &str) -> Result<Vec<crate::types::Row>> {
        let secid = format!("90.{symbol}");
        let url = "https://push2.eastmoney.com/api/qt/clist/get";
        let response = self
            .get(url)
            .query(&crate::util::eastmoney_clist_params("500", &[
                ("fid", "f3"),
                ("fs", "b:MK0901+f:!50".to_string().as_str()),
                ("fields", "f12,f14,f2,f3"),
                ("secid", secid.as_str()),
            ]))
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&response)?;
        let diff = data["data"]["diff"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for mut row in diff {
            let mut r = crate::types::Row::new();
            r.insert(
                "code".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f12"))
                    .unwrap_or_default(),
            );
            r.insert(
                "name".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f14"))
                    .unwrap_or_default(),
            );
            r.insert(
                "close".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f2"))
                    .unwrap_or_default(),
            );
            r.insert(
                "change_pct".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f3"))
                    .unwrap_or_default(),
            );
            items.push(r);
        }
        Ok(items)
    }

    /// Shenwan third-level index info (申万三级行业信息).
    ///
    /// `symbol`: Shenwan Level-2 parent code (optional filter)
    pub async fn sw_index_third_info(&self, _symbol: &str) -> Result<Vec<crate::types::Row>> {
        let url = "https://push2.eastmoney.com/api/qt/clist/get";
        let fs = "m:90+t:4".to_string();
        let response = self
            .get(url)
            .query(&crate::util::eastmoney_clist_params("500", &[
                ("fid", "f3"),
                ("fs", fs.as_str()),
                ("fields", "f12,f14,f2,f3"),
            ]))
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&response)?;
        let diff = data["data"]["diff"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for mut row in diff {
            let mut r = crate::types::Row::new();
            r.insert(
                "code".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f12"))
                    .unwrap_or_default(),
            );
            r.insert(
                "name".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f14"))
                    .unwrap_or_default(),
            );
            r.insert(
                "close".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f2"))
                    .unwrap_or_default(),
            );
            r.insert(
                "change_pct".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("f3"))
                    .unwrap_or_default(),
            );
            r.insert("level".into(), serde_json::json!("3"));
            items.push(r);
        }
        Ok(items)
    }

    /// List Shenwan Level-1 industry indices with latest prices.
    ///
    /// Returns an [`IndexSnapshot`] per Level-1 industry (the 801xxx series)
    /// with the latest index value, change percentage, volume and amount.
    pub async fn sw_index_list(&self) -> Result<Vec<IndexSnapshot>> {
        // Build a comma-separated secid list for batch query via clist.
        // Eastmoney's clist API does not support `secid` filtering, so we
        // use the sector filter `m:90+t:2` which covers Shenwan industry indices.
        let pz = "200".to_string();
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(pz.as_str(), &[
                ("fid", "f3"),
                ("fs", "m:90+t:2"),
                ("fields", "f12,f14,f2,f3,f5,f6"),
                ]))
        )
        .await?;

        let payload: ClistResp = response.json().await.map_err(Error::from)?;
        let today = today_iso();

        let items = payload
            .data
            .and_then(|d| d.diff)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| {
                let item: ClistItem = serde_json::from_value(v).ok()?;
                let code = item.code?;
                // Filter to Level-1 Shenwan indices (801xxx).
                if !SW_LEVEL1_CODES.contains(&code.as_str()) {
                    return None;
                }
                Some(IndexSnapshot {
                    symbol: code,
                    name: item.name.unwrap_or_else(|| "未知行业".to_string()),
                    date: today.clone(),
                    close: item.close.unwrap_or(0.0),
                    change_pct: item.change_pct.unwrap_or(0.0),
                    volume: item.volume.unwrap_or(0.0),
                    amount: item.amount.unwrap_or(0.0),
                })
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no Shenwan Level-1 index items",
            ));
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sw_level1_codes_not_empty() {
        assert!(!SW_LEVEL1_CODES.is_empty());
        // All codes should start with 801
        for code in SW_LEVEL1_CODES {
            assert!(code.starts_with("801"), "unexpected SW code: {code}");
            assert_eq!(code.len(), 6);
        }
    }

    #[test]
    fn test_sw_secid_prefix() {
        assert_eq!(format!("{SW_SECID_PREFIX}.801010"), "90.801010");
    }
}
