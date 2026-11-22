use crate::types::Ohlcv;

use super::moving_average;
use super::types::BollingerBands;

/// Average True Range over `period` bars (default 14), Wilder-smoothed as in
/// TA-Lib, stockstats and TradingView.
pub fn atr<T: Ohlcv>(candles: &[T], period: usize) -> Option<f64> {
    if candles.len() <= period {
        return None;
    }
    let high: Vec<f64> = candles.iter().map(|c| c.high()).collect();
    let low: Vec<f64> = candles.iter().map(|c| c.low()).collect();
    let close: Vec<f64> = candles.iter().map(|c| c.close()).collect();
    use fin_primitives::signals::batch_f64::{AtrSeed, atr as wilder_atr};
    let last = *wilder_atr(&high, &low, &close, period, AtrSeed::TaLib).last()?;
    (!last.is_nan()).then_some(last)
}

/// Bollinger Bands (20, 2). Returns (middle, upper, lower).
pub fn bollinger<T: Ohlcv>(candles: &[T], period: usize) -> Option<BollingerBands> {
    let mid = moving_average::sma(candles, period)?;
    let slice = &candles[candles.len() - period..];
    let variance: f64 = slice
        .iter()
        .map(|c| {
            let diff = c.close() - mid;
            diff * diff
        })
        .sum::<f64>()
        / period as f64;
    let stddev = variance.sqrt();
    Some(BollingerBands {
        middle: mid,
        upper: mid + stddev * 2.0,
        lower: mid - stddev * 2.0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CandlePoint;

    fn candle(high: f64, low: f64, close: f64) -> CandlePoint {
        CandlePoint {
            trade_date: String::new(),
            open: close,
            close,
            high,
            low,
            volume: 0,
            amount: 0.0,
            amplitude_pct: 0.0,
            change_pct: 0.0,
            change_amount: 0.0,
            turnover_pct: 0.0,
        }
    }

    #[test]
    fn atr_basic() {
        let candles: Vec<_> = (0..20)
            .map(|i| candle(i as f64 + 3.0, i as f64, i as f64 + 1.0))
            .collect();
        let result = atr(&candles, 14).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn atr_insufficient() {
        let candles: Vec<_> = (0..5).map(|i| candle(i as f64, 0.0, i as f64)).collect();
        assert!(atr(&candles, 14).is_none());
    }

    #[test]
    fn atr_is_wilder_smoothed() {
        // 14 bars with a true range of 2, then 14 with a range of 1. An SMA of the
        // last 14 true ranges reads 1.0; Wilder still carries the earlier volatility.
        let candles: Vec<_> = (0..29)
            .map(|i| {
                if i <= 14 {
                    candle(11.0, 9.0, 10.0)
                } else {
                    candle(10.5, 9.5, 10.0)
                }
            })
            .collect();
        let expected = 1.0 + (13.0f64 / 14.0).powi(14);
        let result = atr(&candles, 14).unwrap();
        assert!((result - expected).abs() < 1e-9, "atr = {result}");
    }

    #[test]
    fn bollinger_basic() {
        let candles: Vec<_> = (0..25)
            .map(|i| candle(i as f64 + 2.0, i as f64, i as f64 + 1.0))
            .collect();
        let bb = bollinger(&candles, 20).unwrap();
        assert!(bb.upper > bb.middle);
        assert!(bb.middle > bb.lower);
    }

    #[test]
    fn bollinger_insufficient() {
        let candles: Vec<_> = (0..5).map(|i| candle(i as f64, 0.0, i as f64)).collect();
        assert!(bollinger(&candles, 20).is_none());
    }
}
