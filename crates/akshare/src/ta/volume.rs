use crate::types::Ohlcv;

use super::types::ObvValues;

/// On-Balance Volume. Returns (current_obv, obv_change).
pub fn obv<T: Ohlcv>(candles: &[T]) -> Option<ObvValues> {
    if candles.len() < 2 {
        return None;
    }
    let mut obv = 0.0;
    let mut prev_obv = 0.0;
    for pair in candles.windows(2) {
        prev_obv = obv;
        let prev = &pair[0];
        let current = &pair[1];
        if current.close() > prev.close() {
            obv += current.volume();
        } else if current.close() < prev.close() {
            obv -= current.volume();
        }
    }
    Some(ObvValues {
        obv,
        change: obv - prev_obv,
    })
}

/// Volume Weighted Moving Average over `period` bars.
pub fn vwma<T: Ohlcv>(candles: &[T], period: usize) -> Option<f64> {
    if candles.len() < period {
        return None;
    }
    let slice = &candles[candles.len() - period..];
    let volume_sum: f64 = slice.iter().map(|c| c.volume()).sum();
    if volume_sum <= 0.0 {
        return None;
    }
    Some(slice.iter().map(|c| c.close() * c.volume()).sum::<f64>() / volume_sum)
}

/// Volume Weighted Average Price over `period` bars.
pub fn vwap<T: Ohlcv>(candles: &[T], period: usize) -> Option<f64> {
    if candles.len() < period {
        return None;
    }
    let slice = &candles[candles.len() - period..];
    let volume_sum: f64 = slice.iter().map(|c| c.volume()).sum();
    if volume_sum <= 0.0 {
        return None;
    }
    Some(
        slice
            .iter()
            .map(|c| {
                let typical = (c.high() + c.low() + c.close()) / 3.0;
                typical * c.volume()
            })
            .sum::<f64>()
            / volume_sum,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CandlePoint;

    fn make_candle(high: f64, low: f64, close: f64, volume: i64) -> CandlePoint {
        CandlePoint {
            trade_date: String::new(),
            open: close,
            close,
            high,
            low,
            volume,
            amount: 0.0,
            amplitude_pct: 0.0,
            change_pct: 0.0,
            change_amount: 0.0,
            turnover_pct: 0.0,
        }
    }

    #[test]
    fn obv_basic() {
        let candles = vec![
            make_candle(10.0, 8.0, 9.0, 1000),
            make_candle(11.0, 9.0, 10.5, 1500),
            make_candle(10.0, 8.0, 9.0, 800),
        ];
        let result = obv(&candles).unwrap();
        assert_eq!(result.obv, 1500.0 - 800.0);
    }

    #[test]
    fn obv_insufficient() {
        let candles = vec![make_candle(10.0, 8.0, 9.0, 1000)];
        assert!(obv(&candles).is_none());
    }

    #[test]
    fn vwma_basic() {
        let candles = vec![
            make_candle(10.0, 8.0, 10.0, 100),
            make_candle(10.0, 8.0, 20.0, 200),
        ];
        let result = vwma(&candles, 2).unwrap();
        // (10*100 + 20*200) / (100+200) = 5000/300
        assert!((result - 5000.0 / 300.0).abs() < 1e-10);
    }

    #[test]
    fn vwma_zero_volume() {
        let candles = vec![
            make_candle(10.0, 8.0, 10.0, 0),
            make_candle(10.0, 8.0, 20.0, 0),
        ];
        assert!(vwma(&candles, 2).is_none());
    }

    #[test]
    fn vwap_basic() {
        let candles = vec![
            make_candle(12.0, 8.0, 10.0, 100),
            make_candle(22.0, 18.0, 20.0, 200),
        ];
        let result = vwap(&candles, 2).unwrap();
        let tp1 = (12.0 + 8.0 + 10.0) / 3.0;
        let tp2 = (22.0 + 18.0 + 20.0) / 3.0;
        let expected = (tp1 * 100.0 + tp2 * 200.0) / 300.0;
        assert!((result - expected).abs() < 1e-10);
    }
}
