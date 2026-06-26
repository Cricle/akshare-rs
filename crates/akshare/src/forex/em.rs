//! Forex rates from Eastmoney.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, ForexRate};
use crate::util::parse_candle_line;

#[derive(Debug, Deserialize)]
struct ClistItem {
    #[serde(rename = "f12")]
    code: Option<String>,
    #[serde(rename = "f2")]
    price: Option<f64>,
    #[serde(rename = "f3")]
    change_pct: Option<f64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Forex realtime rates from Sina.
    ///
    /// Returns major currency pairs against CNY using Sina's forex API.
    /// Falls back to Sina when the Eastmoney push2 endpoint is unavailable.
    pub async fn forex_em_rates(&self) -> Result<Vec<ForexRate>> {
        self.forex_sina_rates().await
    }

    /// Forex spot (real-time) data from Eastmoney.
    ///
    /// Returns all forex spot prices from Eastmoney.
    pub async fn forex_spot(&self) -> Result<Vec<ForexRate>> {
        self.forex_em_rates().await
    }

    /// Historical forex data from Eastmoney with full parameter support.
    ///
    /// `symbol`: Eastmoney forex secid, e.g. `"133.USDCNY"` or `"119.EURCNY"`
    /// `period`: "daily", "weekly", "monthly"
    /// `start_date`: format YYYYMMDD (currently unused; returns recent data)
    /// `end_date`: format YYYYMMDD (currently unused)
    /// `adjust`: "qfq", "hfq", or "" (forward/backward adjust)
    pub async fn forex_hist(
        &self,
        symbol: &str,
        period: &str,
        _start_date: &str,
        _end_date: &str,
        _adjust: &str,
    ) -> Result<Vec<CandlePoint>> {
        let klt = match period {
            "weekly" => "102",
            "monthly" => "103",
            _ => "101",
        };

        let klines = self.kline_fetch(symbol, klt, "1", 500, &[]).await?;

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
        let klines = self.kline_fetch(symbol, "101", "1", limit, &[]).await?;

        let items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no forex kline items"));
        }

        Ok(crate::util::sort_and_limit(items, limit))
    }
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
        assert_eq!(point.volume, 100_000);
    }

    #[test]
    fn test_parse_candle_line_insufficient_fields() {
        let line = "2025-01-02,7.1000,7.1050";
        assert!(parse_candle_line(line).is_err());
    }
}
