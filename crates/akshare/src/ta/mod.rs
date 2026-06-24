//! Technical analysis indicators, generic over the [`Ohlcv`] trait.
//!
//! All functions accept any slice of types implementing [`crate::types::Ohlcv`].
//!
//! # Categories
//!
//! - **Moving averages**: [`moving_average::sma`], [`moving_average::ema`]
//! - **Momentum**: [`momentum::rsi`], [`momentum::kdj`], [`momentum::cci`], [`momentum::wr`]
//! - **Trend**: [`trend::macd`], [`trend::adx`]
//! - **Volatility**: [`volatility::atr`], [`volatility::bollinger`]
//! - **Volume**: [`volume::obv`], [`volume::vwma`], [`volume::vwap`]

pub mod momentum;
pub mod moving_average;
pub mod trend;
pub mod types;
pub mod volatility;
pub mod volume;

use crate::types::Ohlcv;
pub use types::*;

/// Dispatch indicator by well-known name. Returns the latest value.
///
/// Recognized names: `close_50_sma`, `close_200_sma`, `close_10_ema`, `rsi`,
/// `atr`, `vwma`, `vwap`, `boll`, `boll_ub`, `boll_lb`, `macd`, `macds`,
/// `macdh`, `adx`, `kdj_k`, `kdj_d`, `kdj_j`, `cci`, `wr`, `obv`.
pub fn compute_indicator<T: Ohlcv>(name: &str, candles: &[T]) -> Option<f64> {
    match name {
        "close_50_sma" | "sma_50" => moving_average::sma(candles, 50),
        "close_200_sma" | "sma_200" => moving_average::sma(candles, 200),
        "close_10_ema" | "ema_10" => moving_average::ema(candles, 10),
        "rsi" => momentum::rsi(candles, 14),
        "atr" => volatility::atr(candles, 14),
        "vwma" => volume::vwma(candles, 20),
        "vwap" => volume::vwap(candles, 20),
        "boll" => moving_average::sma(candles, 20),
        "boll_ub" => volatility::bollinger(candles, 20).map(|bb| bb.upper),
        "boll_lb" => volatility::bollinger(candles, 20).map(|bb| bb.lower),
        "macd" => trend::macd(candles).map(|v| v.macd),
        "macds" => trend::macd(candles).map(|v| v.signal),
        "macdh" => trend::macd(candles).map(|v| v.histogram),
        "adx" => trend::adx(candles, 14),
        "kdj_k" => momentum::kdj(candles, 9).map(|v| v.k),
        "kdj_d" => momentum::kdj(candles, 9).map(|v| v.d),
        "kdj_j" => momentum::kdj(candles, 9).map(|v| v.j),
        "cci" => momentum::cci(candles, 20),
        "wr" => momentum::wr(candles, 14),
        "obv" => volume::obv(candles).map(|v| v.obv),
        _ => None,
    }
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
            high: close + 1.0,
            low: close - 1.0,
            volume: 1000,
            amount: 0.0,
            amplitude_pct: 0.0,
            change_pct: 0.0,
            change_amount: 0.0,
            turnover_pct: 0.0,
        }
    }

    #[test]
    fn compute_sma() {
        let candles: Vec<_> = (0..60).map(|i| candle(i as f64)).collect();
        assert!(compute_indicator("close_50_sma", &candles).is_some());
    }

    #[test]
    fn compute_unknown() {
        let candles: Vec<_> = (0..60).map(|i| candle(i as f64)).collect();
        assert!(compute_indicator("unknown_indicator", &candles).is_none());
    }

    #[test]
    fn compute_rsi() {
        let candles: Vec<_> = (0..20).map(|i| candle(i as f64)).collect();
        assert!(compute_indicator("rsi", &candles).is_some());
    }

    #[test]
    fn compute_insufficient_data() {
        let candles: Vec<_> = (0..5).map(|i| candle(i as f64)).collect();
        assert!(compute_indicator("close_50_sma", &candles).is_none());
    }
}
