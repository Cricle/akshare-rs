//! REITs data from Eastmoney.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, ReitSnapshot};
use crate::util::*;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ClistEnvelope {
    data: Option<ClistData>,
}

#[derive(Debug, Deserialize)]
struct ClistData {
    diff: Option<Vec<ClistItem>>,
}

#[derive(Debug, Deserialize)]
struct ClistItem {
    #[serde(rename = "f12")]
    code: Option<String>,
    #[serde(rename = "f14")]
    name: Option<String>,
    #[serde(rename = "f2")]
    price: Option<f64>,
    #[serde(rename = "f3")]
    change_pct: Option<f64>,
    #[serde(rename = "f5")]
    volume: Option<f64>,
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

impl AkShareClient {
    /// Fetch a snapshot list of China REITs from Eastmoney.
    ///
    /// Returns up to `limit` REITs with latest pricing data from the
    /// Eastmoney clist API using `fs=b:MK0970` (REITs board).
    pub async fn reits_list(&self, limit: usize) -> Result<Vec<ReitSnapshot>> {
        let pz = limit.clamp(1, 200).to_string();
        let today = today_iso();

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
                ("fs", "b:MK0970"),
                ("fields", "f12,f14,f2,f3,f5"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: ClistEnvelope = response.json().await.map_err(Error::from)?;
        let items = payload
            .data
            .and_then(|d| d.diff)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|item| {
                let code = item.code?;
                if code.is_empty() {
                    return None;
                }
                Some(ReitSnapshot {
                    symbol: code,
                    name: item.name.unwrap_or_else(|| "未知REIT".to_string()),
                    date: today.clone(),
                    close: item.price.unwrap_or(0.0),
                    change_pct: item.change_pct.unwrap_or(0.0),
                    volume: item.volume.unwrap_or(0.0),
                    nav: None,
                })
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no REITs"));
        }
        Ok(items)
    }

    /// REITs historical data with full parameter support.
    ///
    /// `symbol`: REIT code, e.g. "508000"
    /// `period`: "daily", "weekly", "monthly"
    /// `start_date`: format YYYYMMDD (currently unused; returns recent data)
    /// `end_date`: format YYYYMMDD (currently unused)
    /// `adjust`: "qfq", "hfq", or ""
    pub async fn reits_hist_em(
        &self,
        symbol: &str,
        period: &str,
        _start_date: &str,
        _end_date: &str,
        _adjust: &str,
    ) -> Result<Vec<CandlePoint>> {
        let klt = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => "101",
        };

        let secid = reits_eastmoney_secid(symbol)?;
        let response = self
                        .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", klt),
                ("fqt", "1"),
                ("lmt", "500"),
                ("end", "20500000"),
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
            .ok_or_else(|| Error::upstream("eastmoney REIT kline response missing data"))?;
        let klines = data
            .klines
            .ok_or_else(|| Error::upstream("eastmoney REIT kline response missing klines"))?;

        let items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_reit_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no REIT kline items"));
        }
        Ok(items)
    }

    /// REITs minute-level data from Eastmoney.
    ///
    /// `symbol`: REIT code, e.g. "508000"
    /// `period`: "1", "5", "15", "30", "60"
    pub async fn reits_hist_min_em(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<CandlePoint>> {
        let secid = reits_eastmoney_secid(symbol)?;
        let response = self
                        .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", period),
                ("fqt", "1"),
                ("lmt", "500"),
                ("end", "20500000"),
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
            .ok_or_else(|| Error::upstream("eastmoney REIT min kline missing data"))?;
        let klines = data
            .klines
            .ok_or_else(|| Error::upstream("eastmoney REIT min kline missing klines"))?;

        let items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_reit_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no REIT min kline items"));
        }
        Ok(items)
    }

    /// REITs real-time data from Eastmoney.
    ///
    /// Returns real-time pricing for all listed REITs.
    pub async fn reits_realtime_em(&self) -> Result<Vec<ReitSnapshot>> {
        self.reits_list(200).await
    }

    /// Fetch historical kline data for a specific REIT from Eastmoney.
    ///
    /// `symbol` is the REIT code, e.g. `"508000"` or `"508000.SH"`.
    /// The function converts the symbol to Eastmoney's secid format
    /// (e.g. `"1.508000"` for SH, `"0.180201"` for SZ).
    pub async fn reits_hist(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        let secid = reits_eastmoney_secid(symbol)?;
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
            .ok_or_else(|| Error::upstream("eastmoney REIT kline response missing data"))?;
        let klines = data
            .klines
            .ok_or_else(|| Error::upstream("eastmoney REIT kline response missing klines"))?;

        let mut items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_reit_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no REIT kline items"));
        }

        items.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        if items.len() > limit {
            let start = items.len() - limit;
            items = items[start..].to_vec();
        }
        Ok(items)
    }
}

/// Convert a REIT symbol to Eastmoney secid format.
///
/// Accepts formats like "508000", "508000.SH", "180201.SZ".
/// Returns "1.{code}" for Shanghai (6/5-prefix codes) or "0.{code}" for Shenzhen.
fn reits_eastmoney_secid(symbol: &str) -> Result<String> {
    let trimmed = symbol.trim();
    if let Some((code, suffix)) = trimmed.split_once('.') {
        let suffix_upper = suffix.to_uppercase();
        let market = match suffix_upper.as_str() {
            "SH" => "1",
            "SZ" => "0",
            _ => return Err(Error::invalid_input(format!("unsupported REIT exchange: {suffix}"))),
        };
        return Ok(format!("{market}.{code}"));
    }
    // Pure numeric: infer exchange from code prefix
    if trimmed.is_empty() {
        return Err(Error::invalid_input("REIT symbol is empty"));
    }
    let market = if trimmed.starts_with('5') || trimmed.starts_with('6') {
        "1" // Shanghai
    } else {
        "0" // Shenzhen
    };
    Ok(format!("{market}.{trimmed}"))
}

/// Parse a single Eastmoney kline CSV line into a `CandlePoint`.
fn parse_reit_candle_line(line: &str) -> Result<CandlePoint> {
    let f = parse_csv_line(line);
    if f.len() < 11 {
        return Err(Error::decode(format!(
            "unexpected eastmoney REIT kline format: {line}"
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
    fn test_reits_eastmoney_secid_sh() {
        assert_eq!(reits_eastmoney_secid("508000").unwrap(), "1.508000");
        assert_eq!(reits_eastmoney_secid("508000.SH").unwrap(), "1.508000");
    }

    #[test]
    fn test_reits_eastmoney_secid_sz() {
        assert_eq!(reits_eastmoney_secid("180201").unwrap(), "0.180201");
        assert_eq!(reits_eastmoney_secid("180201.SZ").unwrap(), "0.180201");
    }

    #[test]
    fn test_reits_eastmoney_secid_empty() {
        assert!(reits_eastmoney_secid("").is_err());
    }

    #[test]
    fn test_parse_reit_candle_line() {
        let line = "2025-01-02,3.500,3.550,3.600,3.480,50000,177500.00,3.43,1.43,0.050,0.12";
        let point = parse_reit_candle_line(line).unwrap();
        assert_eq!(point.trade_date, "2025-01-02");
        assert!((point.open - 3.50).abs() < 0.001);
        assert!((point.close - 3.55).abs() < 0.001);
        assert!((point.high - 3.60).abs() < 0.001);
        assert!((point.low - 3.48).abs() < 0.001);
        assert_eq!(point.volume, 50000);
    }

    #[test]
    fn test_parse_reit_candle_line_insufficient_fields() {
        let line = "2025-01-02,3.500,3.550";
        assert!(parse_reit_candle_line(line).is_err());
    }
}
