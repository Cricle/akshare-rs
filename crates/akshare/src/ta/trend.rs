use crate::types::Ohlcv;

use super::moving_average;
use super::types::MacdValues;

/// MACD (12, 26, 9). Returns (macd_line, signal_line, histogram).
pub fn macd<T: Ohlcv>(candles: &[T]) -> Option<MacdValues> {
    if candles.len() < 35 {
        return None;
    }
    let ema12_series = moving_average::ema_series(candles, 12)?;
    let ema26_series = moving_average::ema_series(candles, 26)?;
    // Align by candle index: EMA12 has (len-12) values, EMA26 has (len-26) values.
    // The last N values of each series correspond to the same candle indices.
    let offset = ema12_series.len() - ema26_series.len();
    let macd_series: Vec<f64> = ema12_series[offset..]
        .iter()
        .zip(ema26_series.iter())
        .map(|(fast, slow)| fast - slow)
        .collect();
    let signal_series = moving_average::ema_values(&macd_series, 9)?;
    let macd_val = *macd_series.last()?;
    let signal_val = *signal_series.last()?;
    Some(MacdValues {
        macd: macd_val,
        signal: signal_val,
        histogram: macd_val - signal_val,
    })
}

/// Average Directional Index over `period` bars (default 14).
pub fn adx<T: Ohlcv>(candles: &[T], period: usize) -> Option<f64> {
    if candles.len() <= period + 1 {
        return None;
    }
    let mut dx_values = Vec::new();
    for window in candles.windows(period + 1) {
        let mut plus_dm = 0.0f64;
        let mut minus_dm = 0.0f64;
        let mut tr_sum = 0.0f64;
        for pair in window.windows(2) {
            let prev = &pair[0];
            let current = &pair[1];
            let up_move = current.high() - prev.high();
            let down_move = prev.low() - current.low();
            if up_move > down_move && up_move > 0.0 {
                plus_dm += up_move;
            }
            if down_move > up_move && down_move > 0.0 {
                minus_dm += down_move;
            }
            let hl = current.high() - current.low();
            let hc = (current.high() - prev.close()).abs();
            let lc = (current.low() - prev.close()).abs();
            tr_sum += hl.max(hc).max(lc);
        }
        if tr_sum <= f64::EPSILON {
            continue;
        }
        let plus_di = 100.0 * plus_dm / tr_sum;
        let minus_di = 100.0 * minus_dm / tr_sum;
        let denom = plus_di + minus_di;
        if denom > f64::EPSILON {
            dx_values.push(((plus_di - minus_di).abs() / denom) * 100.0);
        }
    }
    let slice = &dx_values[dx_values.len().saturating_sub(period)..];
    (!slice.is_empty()).then_some(slice.iter().sum::<f64>() / slice.len() as f64)
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
            volume: 1000,
            amount: 0.0,
            amplitude_pct: 0.0,
            change_pct: 0.0,
            change_amount: 0.0,
            turnover_pct: 0.0,
        }
    }

    #[test]
    fn macd_insufficient() {
        let candles: Vec<_> = (0..10)
            .map(|i| make_candle(i as f64, 0.0, i as f64))
            .collect();
        assert!(macd(&candles).is_none());
    }

    #[test]
    fn macd_basic() {
        // Build a trending series so EMA12 != EMA26
        let candles: Vec<_> = (0..50)
            .map(|i| make_candle(i as f64 + 2.0, i as f64, i as f64 + 1.0))
            .collect();
        let result = macd(&candles).unwrap();
        // In a consistent uptrend, macd line should be positive
        assert!(result.macd > 0.0);
    }

    #[test]
    fn adx_insufficient() {
        let candles: Vec<_> = (0..5)
            .map(|i| make_candle(i as f64, 0.0, i as f64))
            .collect();
        assert!(adx(&candles, 14).is_none());
    }

    #[test]
    fn adx_basic() {
        let candles: Vec<_> = (0..30)
            .map(|i| {
                let v = (i as f64 * 0.5).sin() * 5.0 + 100.0;
                make_candle(v + 2.0, v - 2.0, v)
            })
            .collect();
        let result = adx(&candles, 14).unwrap();
        assert!((0.0..=100.0).contains(&result));
    }
}
