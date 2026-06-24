# Technical Analysis Module

Generic technical analysis indicators. All functions accept any slice of types implementing `Ohlcv`.

## Sub-modules

### ta::moving_average
- **sma** — Simple Moving Average
- **ema** — Exponential Moving Average
- **sma_series** — SMA series over a window
- **ema_series** — EMA series (used by MACD)
- **ema_values** — EMA over raw f64 values

### ta::momentum
- **rsi** — Relative Strength Index (default 14)
- **kdj** — KDJ indicator (default 9)
- **cci** — Commodity Channel Index (default 20)
- **wr** — Williams %R (default 14)

### ta::trend
- **macd** — MACD (12, 26, 9) with signal line and histogram
- **adx** — Average Directional Index (default 14)

### ta::volatility
- **atr** — Average True Range (default 14)
- **bollinger** — Bollinger Bands (default 20, 2 std dev)

### ta::volume
- **obv** — On-Balance Volume
- **vwma** — Volume Weighted Moving Average
- **vwap** — Volume Weighted Average Price

### ta::compute_indicator
Dispatch by well-known name (e.g. `"close_50_sma"`, `"rsi"`, `"macd"`).

## Usage

```rust,ignore
use akshare::ta::{self, CandlePoint};

fn make_candle(close: f64) -> CandlePoint {
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

let candles: Vec<_> = (0..60).map(|i| make_candle(i as f64)).collect();

// RSI
let rsi = ta::momentum::rsi(&candles, 14);

// MACD
let macd = ta::trend::macd(&candles);

// Bollinger Bands
let bb = ta::volatility::bollinger(&candles, 20);

// Dispatch by name
let value = ta::compute_indicator("rsi", &candles);
```
