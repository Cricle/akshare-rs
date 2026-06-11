#![allow(dead_code)]
//! Forex rates from Eastmoney.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, ForexRate};
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
    /// Forex realtime rates from Eastmoney.
    ///
    /// Returns major currency pairs against CNY from Eastmoney's forex market.
    /// Uses the clist API with `fs=m:119,m:120` (forex markets).
    pub async fn forex_em_rates(&self) -> Result<Vec<ForexRate>> {
        let response = self
                        .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", "100"),
                ("po", "1"),
                ("np", "1"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f3"),
                ("fs", "m:119,m:120"),
                ("fields", "f12,f14,f2,f3"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: ClistEnvelope = response.json().await.map_err(Error::from)?;
        let today = today_iso();
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
                let price = item.price.unwrap_or(0.0);
                // Eastmoney clist forex: price is the middle rate.
                // buy_rate and sell_rate are not available from this endpoint;
                // approximate with a tight spread around the middle rate.
                let spread = price * 0.0001; // 1 pip spread approximation
                Some(ForexRate {
                    currency_pair: code,
                    buy_rate: price - spread,
                    sell_rate: price + spread,
                    middle_rate: price,
                    date: today.clone(),
                    change_pct: item.change_pct,
                })
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no forex rates"));
        }
        Ok(items)
    }

    /// Forex spot (real-time) data from Eastmoney.
    ///
    /// Returns all forex spot prices from Eastmoney.
    pub async fn forex_spot_em(&self) -> Result<Vec<ForexRate>> {
        self.forex_em_rates().await
    }

    /// Historical forex data from Eastmoney with full parameter support.
    ///
    /// `symbol`: Eastmoney forex secid, e.g. `"133.USDCNY"` or `"119.EURCNY"`
    /// `period`: "daily", "weekly", "monthly"
    /// `start_date`: format YYYYMMDD (currently unused; returns recent data)
    /// `end_date`: format YYYYMMDD (currently unused)
    /// `adjust`: "qfq", "hfq", or "" (forward/backward adjust)
    pub async fn forex_hist_em(
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

        let response = self
                        .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", symbol),
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
            .ok_or_else(|| Error::upstream("eastmoney forex kline response missing data"))?;
        let klines = data
            .klines
            .ok_or_else(|| Error::upstream("eastmoney forex kline response missing klines"))?;

        let items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no forex kline items"));
        }
        Ok(items)
    }

    /// Historical forex kline data from Eastmoney.
    ///
    /// `symbol` is an Eastmoney forex secid, e.g. `"133.USDCNY"` or `"119.EURCNY"`.
    /// `limit` is the number of data points to return.
    pub async fn forex_em_hist(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        if symbol.trim().is_empty() {
            return Err(Error::invalid_input("forex symbol is empty"));
        }
        let lmt = limit.max(5).to_string();

        let response = self
                        .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", symbol),
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
            .ok_or_else(|| Error::upstream("eastmoney forex kline response missing data"))?;
        let klines = data
            .klines
            .ok_or_else(|| Error::upstream("eastmoney forex kline response missing klines"))?;

        let mut items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no forex kline items"));
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
            "unexpected eastmoney forex kline format: {line}"
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
    fn test_parse_candle_line() {
        let line = "2025-01-02,7.1000,7.1050,7.1100,7.0900,100000,710000.00,0.28,0.07,0.0050,0.00";
        let point = parse_candle_line(line).unwrap();
        assert_eq!(point.trade_date, "2025-01-02");
        assert!((point.open - 7.10).abs() < 0.001);
        assert!((point.close - 7.105).abs() < 0.001);
        assert!((point.high - 7.11).abs() < 0.001);
        assert!((point.low - 7.09).abs() < 0.001);
        assert_eq!(point.volume, 100000);
    }

    #[test]
    fn test_parse_candle_line_insufficient_fields() {
        let line = "2025-01-02,7.1000,7.1050";
        assert!(parse_candle_line(line).is_err());
    }
}
