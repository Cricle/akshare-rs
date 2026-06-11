//! Convertible bond (可转债) data from Eastmoney.
//!
//! - `bond_convertible_list`: real-time list via clist API (`b:MK0354`)
//! - `bond_convertible_hist`: daily klines via Eastmoney kline API

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{BondSnapshot, CandlePoint};
use crate::util::{parse_csv_line, parse_f64_safe};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ClistEnvelope {
    data: Option<ClistData>,
}

#[derive(Debug, Deserialize)]
struct ClistData {
    diff: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct KlineEnvelope {
    data: Option<KlineData>,
}

#[derive(Debug, Deserialize)]
struct KlineData {
    klines: Option<Vec<String>>,
}

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
            "11" => "1", // Shanghai
            "12" => "0", // Shenzhen
            _ => "1",
        }
    } else {
        "1"
    };
    Ok(format!("{market}.{code}"))
}

impl AkShareClient {
    /// List convertible bonds with real-time snapshot data.
    ///
    /// Uses Eastmoney clist API with `fs=b:MK0354` to retrieve all listed
    /// convertible bonds. Returns up to `limit` items with current price
    /// and change percentage.
    pub async fn bond_convertible_list(&self, limit: usize) -> Result<Vec<BondSnapshot>> {
        let pz = limit.clamp(1, 5000).to_string();
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
                ("fs", "b:MK0354"),
                ("fields", "f2,f3,f12,f14"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: ClistEnvelope = response.json().await.map_err(Error::from)?;
        let values = payload.data.and_then(|d| d.diff).unwrap_or_default();

        if values.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no convertible bond items",
            ));
        }

        let today = crate::util::today_iso();
        let items: Vec<BondSnapshot> = values
            .into_iter()
            .take(limit)
            .filter_map(|v| {
                let code = v.get("f12")?.as_str()?.to_string();
                let name = v
                    .get("f14")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string();
                let price = v.get("f2").and_then(|x| x.as_f64()).unwrap_or(0.0);
                let change_pct = v.get("f3").and_then(|x| x.as_f64()).unwrap_or(0.0);
                Some(BondSnapshot {
                    symbol: code,
                    name,
                    date: today.clone(),
                    close: price,
                    change_pct,
                    yield_rate: None,
                    credit_rating: None,
                })
            })
            .collect();

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
        let lmt = limit.max(5).to_string();

        let response = self
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", "101"),
                ("fqt", "1"),
                ("lmt", lmt.as_str()),
                ("end", "20500000"),
                ("iscca", "1"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: KlineEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload
            .data
            .ok_or_else(|| Error::upstream("eastmoney cb kline response missing data"))?;
        let klines = data
            .klines
            .ok_or_else(|| Error::upstream("eastmoney cb kline response missing klines"))?;

        let mut items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no convertible bond kline items",
            ));
        }

        items.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        if items.len() > limit {
            let start = items.len() - limit;
            items = items[start..].to_vec();
        }
        Ok(items)
    }
}

/// Parse a single Eastmoney kline CSV line into a `CandlePoint`.
///
/// Format: `date,open,close,high,low,volume,amount,amplitude_pct,change_pct,change_amount,turnover_pct`
fn parse_candle_line(line: &str) -> Result<CandlePoint> {
    let f = parse_csv_line(line);
    if f.len() < 11 {
        return Err(Error::decode(format!(
            "unexpected eastmoney candle format: {line}"
        )));
    }
    Ok(CandlePoint {
        trade_date: f[0].to_string(),
        open: parse_f64_safe(f[1]),
        close: parse_f64_safe(f[2]),
        high: parse_f64_safe(f[3]),
        low: parse_f64_safe(f[4]),
        volume: parse_f64_safe(f[5]).round() as i64,
        amount: parse_f64_safe(f[6]),
        amplitude_pct: parse_f64_safe(f[7]),
        change_pct: parse_f64_safe(f[8]),
        change_amount: parse_f64_safe(f[9]),
        turnover_pct: parse_f64_safe(f[10]),
    })
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
