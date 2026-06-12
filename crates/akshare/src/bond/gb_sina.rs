//! Government bond yield data from Sina Finance.
//!
//! Provides China and US government bond yield historical data.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

// Wire types

#[derive(Debug, Deserialize)]
struct SinaGbEnvelope {
    result: Option<SinaGbResult>,
}

#[derive(Debug, Deserialize)]
struct SinaGbResult {
    data: Option<Vec<Vec<String>>>,
}

/// Map of China government bond tenor names to Sina symbol codes.
const ZH_GB_MAP: &[(&str, &str)] = &[
    ("中国1年期国债", "CN1YT"),
    ("中国2年期国债", "CN2YT"),
    ("中国3年期国债", "CN3YT"),
    ("中国5年期国债", "CN5YT"),
    ("中国7年期国债", "CN7YT"),
    ("中国10年期国债", "CN10YT"),
    ("中国15年期国债", "CN15YT"),
    ("中国20年期国债", "CN20YT"),
    ("中国30年期国债", "CN30YT"),
];

/// Map of US government bond tenor names to Sina symbol codes.
const US_GB_MAP: &[(&str, &str)] = &[
    ("美国1月期国债", "US1MT"),
    ("美国2月期国债", "US2MT"),
    ("美国3月期国债", "US3MT"),
    ("美国4月期国债", "US4MT"),
    ("美国6月期国债", "US6MT"),
    ("美国1年期国债", "US1YT"),
    ("美国2年期国债", "US2YT"),
    ("美国3年期国债", "US3YT"),
    ("美国5年期国债", "US5YT"),
    ("美国7年期国债", "US7YT"),
    ("美国10年期国债", "US10YT"),
    ("美国20年期国债", "US20YT"),
    ("美国30年期国债", "US30YT"),
];

/// List all available China government bond tenor names.
#[must_use]
pub fn bond_gb_zh_symbols() -> Vec<&'static str> {
    ZH_GB_MAP.iter().map(|(name, _)| *name).collect()
}

/// List all available US government bond tenor names.
#[must_use]
pub fn bond_gb_us_symbols() -> Vec<&'static str> {
    US_GB_MAP.iter().map(|(name, _)| *name).collect()
}

fn resolve_sina_code(map: &[(&str, &str)], name: &str) -> Result<String> {
    map.iter()
        .find(|(n, _)| *n == name)
        .map(|(_, code)| (*code).to_string())
        .ok_or_else(|| Error::invalid_input(format!("unknown bond tenor: {name}")))
}

impl AkShareClient {
    /// Fetch China government bond yield data from Sina Finance.
    ///
    /// `symbol` is one of the China bond tenor names (e.g. "中国10年期国债").
    /// Returns daily open/high/low/close yield data.
    pub async fn bond_gb_zh_sina(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let code = resolve_sina_code(ZH_GB_MAP, symbol)?;
        self.fetch_sina_bond(&code, symbol).await
    }

    /// Fetch US government bond yield data from Sina Finance.
    ///
    /// `symbol` is one of the US bond tenor names (e.g. "美国10年期国债").
    /// Returns daily open/high/low/close yield data.
    pub async fn bond_gb_us_sina(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let code = resolve_sina_code(US_GB_MAP, symbol)?;
        self.fetch_sina_bond(&code, symbol).await
    }

    async fn fetch_sina_bond(&self, code: &str, label: &str) -> Result<Vec<MacroDataPoint>> {
        let url = format!("https://bond.finance.sina.com.cn/hq/gb/daily?symbol={code}");
        let resp: SinaGbEnvelope = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let rows = resp.result.and_then(|r| r.data).unwrap_or_default();

        if rows.is_empty() {
            return Err(Error::not_found(format!(
                "sina returned no bond data for {label}"
            )));
        }

        let items: Vec<MacroDataPoint> = rows
            .into_iter()
            .filter_map(|row| {
                if row.len() < 5 {
                    return None;
                }
                let date = row[0].clone();
                // fields: date, open, high, low, close, volume
                let close: f64 = row[4].parse().ok()?;
                Some(MacroDataPoint {
                    date: date.get(..10).unwrap_or(&date).to_string(),
                    value: close,
                    name: label.to_string(),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(format!("no valid bond data for {label}")));
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zh_gb_map() {
        assert_eq!(ZH_GB_MAP.len(), 9);
        assert_eq!(
            resolve_sina_code(ZH_GB_MAP, "中国10年期国债").unwrap(),
            "CN10YT"
        );
    }

    #[test]
    fn test_us_gb_map() {
        assert_eq!(US_GB_MAP.len(), 13);
        assert_eq!(
            resolve_sina_code(US_GB_MAP, "美国10年期国债").unwrap(),
            "US10YT"
        );
    }

    #[test]
    fn test_invalid_symbol() {
        assert!(resolve_sina_code(ZH_GB_MAP, "不存在的").is_err());
    }

    #[test]
    fn test_bond_gb_zh_symbols() {
        let syms = bond_gb_zh_symbols();
        assert!(syms.contains(&"中国10年期国债"));
    }

    #[test]
    fn test_bond_gb_us_symbols() {
        let syms = bond_gb_us_symbols();
        assert!(syms.contains(&"美国10年期国债"));
    }
}
