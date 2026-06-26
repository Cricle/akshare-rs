//! Convertible bond (可转债) data from Eastmoney.
//!
//! - `bond_convertible_list`: real-time list via clist API (`b:MK0354`)
//! - `bond_convertible_hist`: daily klines via Eastmoney kline API

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::value_ext::ValueExt;
use crate::types::{BondSnapshot, CandlePoint};
use crate::util::parse_candle_line;

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

/// Convert a 6-digit convertible bond code to an Eastmoney secid.
///
/// Codes starting with `11` are Shanghai (market 1), codes starting with
/// `12` are Shenzhen (market 0).
fn cb_secid(symbol: &str) -> Result<String> {
    let code = symbol.trim();
    if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
        return Err(Error::invalid_input(format!(
            "invalid convertible bond code: {symbol}"
        )));
    }
    let market = if code.starts_with('1') && code.len() >= 2 {
        match &code[..2] {
            "12" => "0", // Shenzhen
            _ => "1",    // Shanghai
        }
    } else {
        "1"
    };
    Ok(format!("{market}.{code}"))
}

impl AkShareClient {
    /// List convertible bonds with snapshot data.
    ///
    /// Uses Eastmoney datacenter API (`RPT_BOND_CB_LIST`) to retrieve
    /// convertible bond listing data. Returns up to `limit` items.
    pub async fn bond_convertible_list(&self, limit: usize) -> Result<Vec<BondSnapshot>> {
        use crate::types::wire::EmDatacenterResp;

        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_BOND_CB_LIST"),
                ("columns", "SECURITY_CODE,SECURITY_NAME_ABBR,LISTING_DATE"),
                ("pageNumber", "1"),
                ("pageSize", &limit.min(5000).to_string()),
                ("sortTypes", "-1"),
                ("sortColumns", "LISTING_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        if let Some(msg) = resp.check_error("RPT_BOND_CB_LIST") {
            return Err(Error::upstream(msg));
        }

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        let today = crate::util::today_iso();
        let items: Vec<BondSnapshot> = data
            .iter()
            .filter_map(|v| {
                let code = v.str_or(&["SECURITY_CODE"], "");
                if code.is_empty() {
                    return None;
                }
                Some(BondSnapshot {
                    symbol: code,
                    name: v.str_or(&["SECURITY_NAME_ABBR"], ""),
                    date: today.clone(),
                    close: 0.0,
                    change_pct: 0.0,
                    yield_rate: None,
                    credit_rating: None,
                })
            })
            .take(limit)
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("no convertible bond data available"));
        }
        Ok(items)
    }

    /// Fetch historical daily klines for a convertible bond.
    ///
    /// `symbol` is a 6-digit convertible bond code (e.g. "113050" for SH,
    /// "128039" for SZ). The exchange is inferred from the code prefix.
    pub async fn bond_convertible_hist(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let secid = cb_secid(symbol)?;
        let klines = self
            .kline_fetch(&secid, "101", "1", limit, &[("iscca", "1")])
            .await?;

        let items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no convertible bond kline items",
            ));
        }

        Ok(crate::util::sort_and_limit(items, limit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cb_secid() {
        // Shanghai convertible bonds: 11xxxx
        assert_eq!(cb_secid("113050").unwrap(), "1.113050");
        assert_eq!(cb_secid("110059").unwrap(), "1.110059");
        // Shenzhen convertible bonds: 12xxxx
        assert_eq!(cb_secid("128039").unwrap(), "0.128039");
        assert_eq!(cb_secid("123121").unwrap(), "0.123121");
    }

    #[test]
    fn test_cb_secid_invalid() {
        assert!(cb_secid("abc").is_err());
        assert!(cb_secid("12345").is_err());
        assert!(cb_secid("1234567").is_err());
    }

    #[test]
    fn test_parse_candle_line() {
        let line = "2025-01-02,100.50,101.20,102.00,99.80,50000,5050000.00,2.20,0.70,0.70,1.50";
        let cp = parse_candle_line(line).unwrap();
        assert_eq!(cp.trade_date, "2025-01-02");
        assert!((cp.open - 100.50).abs() < 0.01);
        assert!((cp.close - 101.20).abs() < 0.01);
        assert!((cp.high - 102.00).abs() < 0.01);
        assert!((cp.low - 99.80).abs() < 0.01);
        assert_eq!(cp.volume, 50000);
        assert!((cp.change_pct - 0.70).abs() < 0.01);
    }

    #[test]
    fn test_parse_candle_line_short() {
        let line = "2025-01-02,100.50,101.20";
        assert!(parse_candle_line(line).is_err());
    }
}
