use crate::types::Ohlcv;

use super::types::KdjValues;

/// Relative Strength Index over `period` bars (default 14).
pub fn rsi<T: Ohlcv>(candles: &[T], period: usize) -> Option<f64> {
    if candles.len() <= period {
        return None;
    }
    let mut gains = 0.0f64;
    let mut losses = 0.0f64;
    for pair in candles[candles.len() - period - 1..].windows(2) {
        let change = pair[1].close() - pair[0].close();
        if change >= 0.0 {
            gains += change;
        } else {
            losses += change.abs();
        }
    }
    if losses == 0.0 {
        return Some(100.0);
    }
    let rs = gains / losses;
    Some(100.0 - 100.0 / (1.0 + rs))
}

/// Stochastic KDJ over `period` bars (default 9).
pub fn kdj<T: Ohlcv>(candles: &[T], period: usize) -> Option<KdjValues> {
    if candles.len() < period {
        return None;
    }
    let mut k = 50.0;
    let mut d = 50.0;
    for index in period - 1..candles.len() {
        let slice = &candles[index + 1 - period..=index];
        let high = slice
            .iter()
            .map(|c| c.high())
            .fold(f64::NEG_INFINITY, f64::max);
        let low = slice.iter().map(|c| c.low()).fold(f64::INFINITY, f64::min);
        let close = candles[index].close();
        let rsv = if high > low {
            ((close - low) / (high - low)) * 100.0
        } else {
            50.0
        };
        k = (2.0 / 3.0) * k + (1.0 / 3.0) * rsv;
        d = (2.0 / 3.0) * d + (1.0 / 3.0) * k;
    }
    let j = 3.0 * k - 2.0 * d;
    Some(KdjValues { k, d, j })
}

/// Commodity Channel Index over `period` bars (default 20).
pub fn cci<T: Ohlcv>(candles: &[T], period: usize) -> Option<f64> {
    if candles.len() < period {
        return None;
    }
    let slice = &candles[candles.len() - period..];
    let typical: Vec<f64> = slice
        .iter()
        .map(|c| (c.high() + c.low() + c.close()) / 3.0)
        .collect();
    let ma = typical.iter().sum::<f64>() / period as f64;
    let mean_deviation = typical.iter().map(|v| (v - ma).abs()).sum::<f64>() / period as f64;
    if mean_deviation <= f64::EPSILON {
        return None;
    }
    let last = *typical.last()?;
    Some((last - ma) / (0.015 * mean_deviation))
}

/// Williams %R over `period` bars (default 14).
pub fn wr<T: Ohlcv>(candles: &[T], period: usize) -> Option<f64> {
    if candles.len() < period {
        return None;
    }
    let slice = &candles[candles.len() - period..];
    let high = slice
        .iter()
        .map(|c| c.high())
        .fold(f64::NEG_INFINITY, f64::max);
    let low = slice.iter().map(|c| c.low()).fold(f64::INFINITY, f64::min);
    let close = slice.last()?.close();
    if high <= low {
        return None;
    }
    Some(((high - close) / (high - low)) * -100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CandlePoint;

    fn make_candle(high: f64, low: f64, close: f64) -> CandlePoint {
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
    fn rsi_all_gains() {
        let candles: Vec<_> = (0..20)
            .map(|i| make_candle(i as f64 + 1.0, i as f64, i as f64 + 1.0))
            .collect();
        let r = rsi(&candles, 14).unwrap();
        assert_eq!(r, 100.0);
    }

    #[test]
    fn rsi_insufficient() {
        let candles: Vec<_> = (0..5)
            .map(|i| make_candle(i as f64, 0.0, i as f64))
            .collect();
        assert!(rsi(&candles, 14).is_none());
    }

    #[test]
    fn kdj_basic() {
        let candles: Vec<_> = (0..20)
            .map(|i| make_candle(i as f64 + 2.0, i as f64, i as f64 + 1.0))
            .collect();
        let result = kdj(&candles, 9).unwrap();
        assert!(result.k >= 0.0 && result.k <= 100.0);
    }

    #[test]
    fn kdj_insufficient() {
        let candles: Vec<_> = (0..3)
            .map(|i| make_candle(i as f64, 0.0, i as f64))
            .collect();
        assert!(kdj(&candles, 9).is_none());
    }

    #[test]
    fn cci_basic() {
        let candles: Vec<_> = (0..25)
            .map(|i| make_candle(i as f64 + 2.0, i as f64, i as f64 + 1.0))
            .collect();
        assert!(cci(&candles, 20).is_some());
    }

    #[test]
    fn wr_basic() {
        let candles: Vec<_> = (0..20)
            .map(|i| make_candle(i as f64 + 5.0, i as f64, i as f64 + 2.0))
            .collect();
        let r = wr(&candles, 14).unwrap();
        assert!((-100.0..=0.0).contains(&r));
    }

    #[test]
    fn wr_flat() {
        let candles: Vec<_> = (0..20).map(|_| make_candle(10.0, 10.0, 10.0)).collect();
        assert!(wr(&candles, 14).is_none());
    }
}
