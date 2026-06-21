//! Futures spot (现货) price data from Eastmoney.
//!
//! Fetches real-time spot commodity futures prices using the Eastmoney
//! clist API.  Covers SHFE (上海期货), DCE (大连商品), CZCE (郑州商品),
//! CFFEX (中金所) and INE (上海能源) futures markets.
//!
//! Data source: `https://push2.eastmoney.com/api/qt/clist/get`

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FuturesSnapshot;
use crate::types::wire::ClistResp;
use crate::util::today_iso;

#[derive(Debug, Deserialize)]
struct ClistItem {
    /// f12 — symbol / contract code
    #[serde(rename = "f12")]
    code: Option<String>,
    /// f14 — name
    #[serde(rename = "f14")]
    name: Option<String>,
    /// f2 — latest price / close
    #[serde(rename = "f2")]
    close: Option<f64>,
    /// f3 — change percentage
    #[serde(rename = "f3")]
    change_pct: Option<f64>,
    /// f5 — volume
    #[serde(rename = "f5")]
    volume: Option<f64>,
    /// f8 — turnover rate (repurposed as open interest ratio; we don't use it)
    /// We use `f15` (high) and `f16` (low) to verify, but open interest is f10.
    /// f10 — open interest (持仓量)
    #[serde(rename = "f10")]
    open_interest: Option<f64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

/// Eastmoney market codes for Chinese commodity / financial futures:
///   m:113 — SHFE (上海期货交易所)
///   m:114 — DCE  (大连商品交易所)
///   m:115 — CZCE (郑州商品交易所)
///   m:8   — CFFEX (中国金融期货交易所)
///   m:142 — INE  (上海国际能源交易中心)
///   m:225 — GFEX (广州期货交易所)
const FUTURES_FS: &str = "m:113,m:114,m:115,m:8,m:142,m:225";

impl AkShareClient {
    /// Fetch current futures spot prices from Eastmoney.
    ///
    /// Returns up to `limit` futures contracts with latest price, change,
    /// volume and open interest across all major Chinese futures exchanges.
    pub async fn futures_spot_prices(&self, limit: usize) -> Result<Vec<FuturesSnapshot>> {
        let pz = limit.clamp(1, 500).to_string();
        let response = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", pz.as_str()),
                ("po", "1"),
                ("np", "1"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f3"),
                ("fs", FUTURES_FS),
                ("fields", "f12,f14,f2,f3,f5,f10"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

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
                if code.is_empty() {
                    return None;
                }
                Some(FuturesSnapshot {
                    symbol: code,
                    name: item.name.unwrap_or_else(|| "未知合约".to_string()),
                    date: today.clone(),
                    close: item.close.unwrap_or(0.0),
                    change_pct: item.change_pct.unwrap_or(0.0),
                    volume: item.volume.unwrap_or(0.0),
                    open_interest: item.open_interest.unwrap_or(0.0),
                    settlement_price: None,
                })
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no futures spot prices",
            ));
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_fs_constant() {
        // Verify the filter string covers the expected exchanges.
        assert!(FUTURES_FS.contains("m:113"));
        assert!(FUTURES_FS.contains("m:114"));
        assert!(FUTURES_FS.contains("m:115"));
        assert!(FUTURES_FS.contains("m:8"));
    }
}
