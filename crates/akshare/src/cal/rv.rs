//! Realized volatility calculation: Yang-Zhang estimator.
//!
//! Provides functions to compute Yang-Zhang realized volatility from OHLC data.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Data structures
// ---------------------------------------------------------------------------

/// OHLC bar for volatility calculations.
#[derive(Debug, Clone)]
pub struct OhlcBar {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Yang-Zhang realized volatility from OHLC data.
    ///
    /// Computes the Yang-Zhang realized volatility estimator from a series
    /// of OHLC bars. The formula is:
    ///
    /// RV^2 = Vo + k*Vc + (1-k)*Vrs
    ///
    /// where:
    /// - Vo = overnight variance
    /// - Vc = close-to-open variance
    /// - Vrs = Rogers-Satchell variance
    /// - k = 0.34 / (1.34 + (n+1)/(n-1))
    ///
    /// Returns daily realized volatility values.
    pub fn volatility_yz_rv(&self, data: &[OhlcBar]) -> Result<Vec<MacroDataPoint>> {
        if data.len() < 2 {
            return Err(Error::invalid_input(
                "need at least 2 OHLC bars for YZ RV calculation",
            ));
        }

        let mut returns_oi: Vec<f64> = Vec::new(); // overnight returns (open_i / close_{i-1})
        // close-to-open returns (close_i / open_i)
        let mut rs_values: Vec<f64> = Vec::new(); // Rogers-Satchell values

        // Calculate returns
        for i in 1..data.len() {
            let prev_close = data[i - 1].close;
            let curr_open = data[i].open;
            let curr_high = data[i].high;
            let curr_low = data[i].low;
            let curr_close = data[i].close;

            if prev_close <= 0.0 || curr_open <= 0.0 {
                continue;
            }

            // Overnight return (log)
            let oi = (curr_open / prev_close).ln();
            returns_oi.push(oi);

            // Close-to-open return (log)
            let ci = (curr_close / curr_open).ln();

            // Rogers-Satchell components
            let ui = (curr_high / curr_open).ln();
            let di = (curr_low / curr_open).ln();

            // RS = ui*(ui-ci) + di*(di-ci)
            let rs = ui * (ui - ci) + di * (di - ci);
            rs_values.push(rs);
        }

        if returns_oi.is_empty() {
            return Ok(Vec::new());
        }

        let n = returns_oi.len() as f64;
        let k = 0.34 / (1.34 + (n + 1.0) / (n - 1.0));

        // Vo = variance of overnight returns
        let oi_mean: f64 = returns_oi.iter().sum::<f64>() / n;
        let vo: f64 = returns_oi
            .iter()
            .map(|x| (x - oi_mean).powi(2))
            .sum::<f64>()
            / (n - 1.0);

        // Vc = variance of close-to-open returns
        let returns_ci: Vec<f64> = data[1..]
            .iter()
            .map(|bar| (bar.close / bar.open).ln())
            .collect();
        let ci_mean: f64 = returns_ci.iter().sum::<f64>() / n;
        let vc: f64 = returns_ci
            .iter()
            .map(|x| (x - ci_mean).powi(2))
            .sum::<f64>()
            / (n - 1.0);

        // Vrs = mean of RS values
        let vrs: f64 = rs_values.iter().sum::<f64>() / n;

        // Yang-Zhang RV
        let yz_rv_squared = vo + k * vc + (1.0 - k) * vrs;
        let yz_rv = yz_rv_squared.sqrt();

        // Return the final volatility as a single data point
        // with the last date
        let last_date = data.last().map(|b| b.date.clone()).unwrap_or_default();

        Ok(vec![MacroDataPoint {
            date: last_date,
            value: yz_rv,
            name: "Yang-Zhang RV".to_string(),
        }])
    }

    /// Compute RV from Sina futures minute data.
    ///
    /// `symbol`: futures contract code (e.g., "AU2406")
    /// Fetches minute kline data and computes Yang-Zhang realized volatility.
    pub async fn rv_from_futures_zh_minute_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let klines = self.futures_zh_minute_sina(symbol, "1").await?;
        if klines.is_empty() {
            return Err(Error::not_found(format!("no minute data for {symbol}")));
        }

        fn row_f64(row: &crate::types::Row, key: &str) -> f64 {
            row.get(key)
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0)
        }

        let bars: Vec<OhlcBar> = klines
            .iter()
            .map(|k| OhlcBar {
                date: k
                    .get("datetime")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                open: row_f64(k, "open"),
                high: row_f64(k, "high"),
                low: row_f64(k, "low"),
                close: row_f64(k, "close"),
            })
            .collect();

        self.volatility_yz_rv(&bars)
    }

    /// Compute RV from Eastmoney stock minute data.
    ///
    /// `symbol`: stock code (e.g., "600000")
    /// Fetches minute kline data and computes Yang-Zhang realized volatility.
    pub async fn rv_from_stock_zh_a_hist_min_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let klines = self
            .stock_zh_a_hist_min_em(symbol, "5", "qfq", "", "")
            .await?;
        if klines.is_empty() {
            return Err(Error::not_found(format!("no minute data for {symbol}")));
        }

        let bars: Vec<OhlcBar> = klines
            .iter()
            .map(|k| OhlcBar {
                date: k.trade_date.clone(),
                open: k.open,
                high: k.high,
                low: k.low,
                close: k.close,
            })
            .collect();

        self.volatility_yz_rv(&bars)
    }

    /// Compute rolling Yang-Zhang realized volatility.
    ///
    /// Computes YZ RV over a rolling window of `window_size` bars.
    pub fn volatility_yz_rv_rolling(
        &self,
        data: &[OhlcBar],
        window_size: usize,
    ) -> Result<Vec<MacroDataPoint>> {
        if data.len() < window_size + 1 {
            return Err(Error::invalid_input(format!(
                "need at least {} bars for rolling YZ RV",
                window_size + 1
            )));
        }

        let mut items = Vec::new();
        for window_start in 0..(data.len() - window_size) {
            let window = &data[window_start..window_start + window_size + 1];
            if let Ok(mut result) = self.volatility_yz_rv(window)
                && let Some(point) = result.pop()
            {
                items.push(point);
            }
        }
        Ok(items)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Compute Yang-Zhang RV from raw OHLC arrays (convenience function).
pub fn yang_zhang_rv(
    dates: &[String],
    opens: &[f64],
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
) -> Result<f64> {
    let n = dates
        .len()
        .min(opens.len())
        .min(highs.len())
        .min(lows.len())
        .min(closes.len());
    if n < 2 {
        return Err(Error::invalid_input("need at least 2 data points"));
    }

    let bars: Vec<OhlcBar> = (0..n)
        .map(|i| OhlcBar {
            date: dates[i].clone(),
            open: opens[i],
            high: highs[i],
            low: lows[i],
            close: closes[i],
        })
        .collect();

    // Use a temporary client for the calculation
    let client = AkShareClient::new();
    let result = client.volatility_yz_rv(&bars)?;
    Ok(result.first().map_or(0.0, |p| p.value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yang_zhang_rv_basic() {
        let dates = vec![
            "2024-01-01".to_string(),
            "2024-01-02".to_string(),
            "2024-01-03".to_string(),
        ];
        let opens = vec![100.0, 101.0, 102.0];
        let highs = vec![105.0, 106.0, 107.0];
        let lows = vec![99.0, 100.0, 101.0];
        let closes = vec![103.0, 104.0, 105.0];

        let rv = yang_zhang_rv(&dates, &opens, &highs, &lows, &closes).unwrap();
        assert!(rv > 0.0);
    }

    #[test]
    fn test_volatility_yz_rv_insufficient_data() {
        let client = AkShareClient::new();
        let bars = vec![OhlcBar {
            date: "2024-01-01".to_string(),
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 103.0,
        }];
        assert!(client.volatility_yz_rv(&bars).is_err());
    }

    #[test]
    fn test_volatility_yz_rv_rolling() {
        let client = AkShareClient::new();
        let bars: Vec<OhlcBar> = (0..10)
            .map(|i| OhlcBar {
                date: format!("2024-01-{:02}", i + 1),
                open: 100.0 + i as f64,
                high: 105.0 + i as f64,
                low: 99.0 + i as f64,
                close: 103.0 + i as f64,
            })
            .collect();

        let result = client.volatility_yz_rv_rolling(&bars, 5).unwrap();
        assert!(!result.is_empty());
    }
}
