use crate::types::Ohlcv;

use super::moving_average;
use super::types::BollingerBands;

/// Average True Range over `period` bars (default 14).
pub fn atr<T: Ohlcv>(candles: &[T], period: usize) -> Option<f64> {
    if candles.len() <= period {
        return None;
    }
    let ranges: Vec<f64> = candles
        .windows(2)
        .map(|pair| {
            let current = &pair[1];
            let prev = &pair[0];
            let high_low = current.high() - current.low();
            let high_close = (current.high() - prev.close()).abs();
            let low_close = (current.low() - prev.close()).abs();
            high_low.max(high_close).max(low_close)
        })
        .collect();
    let slice = &ranges[ranges.len().saturating_sub(period)..];
    Some(slice.iter().sum::<f64>() / slice.len() as f64)
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
