# Forex Module

The `forex` module provides foreign exchange rate data.

## Functions (19 total)

### Exchange Rates
- **forex_spot_em** — Real-time forex spot from Eastmoney
- **forex_hist_em** — Historical forex data from Eastmoney
- **currency_boc_sina** — BOC exchange rates from Sina
- **currency_boc_safe** — SAFE exchange rates
- **currency_convert** — Currency conversion
- **currency_currencies** — Available currencies
- **currency_history** — Historical rates
- **currency_latest** — Latest rates
- **currency_pair_map** — Currency pair mapping
- **currency_time_series** — Time series data

### FX Data
- **fx_quote** — FX quotes
- **fx_quote_baidu** — FX quotes from Baidu
- **fx_c_swap_cm** — FX swap from ChinaMoney

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// Forex spot
let spot = client.forex_spot_em().await?;

// Historical forex
let hist = client.forex_hist_em("USD/CNY", "20240101", "20241231").await?;

// BOC rates
let boc = client.currency_boc_sina().await?;
```
