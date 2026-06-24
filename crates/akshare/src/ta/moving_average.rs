use crate::types::Ohlcv;

/// Simple Moving Average of close prices over `period` bars.
pub fn sma<T: Ohlcv>(candles: &[T], period: usize) -> Option<f64> {
    if candles.len() < period {
        return None;
    }
    let sum: f64 = candles[candles.len() - period..]
        .iter()
        .map(|c| c.close())
        .sum();
    Some(sum / period as f64)
}

/// Exponential Moving Average of close prices over `period` bars.
pub fn ema<T: Ohlcv>(candles: &[T], period: usize) -> Option<f64> {
    ema_series(candles, period).and_then(|s| s.last().copied())
}

/// Full EMA series from `candles`, starting after the first SMA seed.
pub fn ema_series<T: Ohlcv>(candles: &[T], period: usize) -> Option<Vec<f64>> {
    if candles.len() < period {
        return None;
    }
    let multiplier = 2.0 / (period as f64 + 1.0);
    let mut values = Vec::with_capacity(candles.len() - period + 1);
    let mut ema: f64 = candles[..period].iter().map(|c| c.close()).sum::<f64>() / period as f64;
    values.push(ema);
    for candle in &candles[period..] {
        ema = (candle.close() - ema) * multiplier + ema;
        values.push(ema);
    }
    Some(values)
}

/// EMA computed over a raw `f64` slice (used by MACD signal line).
pub fn ema_values(values: &[f64], period: usize) -> Option<Vec<f64>> {
    if values.len() < period {
        return None;
    }
    let multiplier = 2.0 / (period as f64 + 1.0);
    let mut out = Vec::with_capacity(values.len() - period + 1);
    let mut ema: f64 = values[..period].iter().sum::<f64>() / period as f64;
    out.push(ema);
    for value in &values[period..] {
        ema = (value - ema) * multiplier + ema;
        out.push(ema);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CandlePoint;

    fn candle(close: f64) -> CandlePoint {
        CandlePoint {
            trade_date: String::new(),
            open: close,
            close,
            high: close,
            low: close,
            volume: 0,
            amount: 0.0,
            amplitude_pct: 0.0,
            change_pct: 0.0,
            change_amount: 0.0,
            turnover_pct: 0.0,
        }
    }

    #[test]
    fn sma_basic() {
        let candles: Vec<_> = [10.0, 20.0, 30.0].into_iter().map(candle).collect();
        let result = sma(&candles, 3).unwrap();
        assert!((result - 20.0).abs() < 1e-10);
    }

    #[test]
    fn sma_insufficient() {
        let candles: Vec<_> = [10.0, 20.0].into_iter().map(candle).collect();
        assert!(sma(&candles, 3).is_none());
    }

    #[test]
    fn ema_basic() {
        let candles: Vec<_> = [10.0, 11.0, 12.0, 13.0, 14.0]
            .into_iter()
            .map(candle)
            .collect();
        let result = ema(&candles, 3).unwrap();
        assert!(result > 12.0 && result < 14.0);
    }

    #[test]
    fn ema_series_len() {
        let candles: Vec<_> = (0..10).map(|i| candle(i as f64)).collect();
        let series = ema_series(&candles, 3).unwrap();
        assert_eq!(series.len(), 8); // 10 - 3 + 1
    }

    #[test]
    fn ema_values_basic() {
        let values = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let result = ema_values(&values, 3).unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn ema_values_insufficient() {
        assert!(ema_values(&[1.0, 2.0], 3).is_none());
    }
}
