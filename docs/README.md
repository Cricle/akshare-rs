# akshare-rs Documentation

100% pure Rust implementation of [akshare](https://github.com/akfamily/akshare) — unified access to Chinese and global financial market data APIs.

## Features

- **Zero Python/JS/FFI dependencies** — pure Rust implementation
- **Async/await** — built on tokio for high-performance concurrent requests
- **Type-safe** — strong typing with serde for all API responses
- **Mock-friendly** — built-in mock server support for testing
- **1407 functions** — covers 100% of akshare's public API

## Quick Start

```rust
use akshare::AkShareClient;

#[tokio::main]
async fn main() -> Result<(), akshare::Error> {
    let client = AkShareClient::new();
    
    // A-share quote
    let quote = client.a_share_quote("600000").await?;
    
    // Fund NAV history
    let nav = client.fund_etf_spot().await?;
    
    Ok(())
}
```

## Modules

### Stock Data
- **stock** — A-share, HK, US stock data (469 functions)
  - Real-time quotes, historical candles, sector rankings
  - Billboards, shareholder analysis, capital flow
  - HK/US stock data with multiple fallback sources

### Fund Data
- **fund** — Fund NAV, rankings, holdings (98 functions)
  - ETF spots, LOF data, money market funds
  - Fund manager info, ratings, scale changes

### Bond Data
- **bond** — Government, convertible, corporate bonds (47 functions)
  - ChinaMoney yield curves, Sina/THS sources
  - SSE summaries, CNINFO issuance data

### Futures & Options
- **futures** — Futures daily bars, realtime quotes (110 functions)
  - Sina, Eastmoney, exchange-specific data
  - Inventory, delivery, settlement data
- **option** — Option chains, billboard data (48 functions)

### Index Data
- **index** — A-share, HK, global indices (64 functions)
  - Shenwan, CNIndex, CX, QVIX indices
  - Real-time and historical data

### Macro Data
- **macro_data** — Economic indicators (423 functions)
  - China, US, EU, Japan, Australia, etc.
  - Interest rates, GDP, CPI, trade data

### Other Modules
- **forex** — Foreign exchange rates (19 functions)
- **crypto** — Cryptocurrency spot data (4 functions)
- **commodity** — Commodity prices, carbon trading (9 functions)
- **economy** — Economic events, news, articles (62 functions)
- **news** — Financial news from multiple sources (17 functions)
- **spot** — Spot market prices (14 functions)
- **reits** — REITs data (5 functions)
- **bank** — Banking data (1 function)
- **cal** — Calendar, volatility calculations (2 functions)
- **tool** — Trade calendar, utilities (2 functions)
- **provider** — Data provider abstractions (13 functions)

## Testing

```bash
cargo test
```

All 1839 tests pass with 100% mock coverage.

## CI/CD

The project uses GitHub Actions with:
- `cargo check` — compilation check
- `cargo test` — full test suite
- `cargo fmt` — code formatting
- `cargo clippy` — linting
- MSRV check (Rust 1.85)
- Publish dry-run

## License

MIT OR Apache-2.0
