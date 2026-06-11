# akshare-rs

[![Crates.io](https://img.shields.io/crates/v/akshare)](https://crates.io/crates/akshare)
[![docs.rs](https://img.shields.io/docsrs/akshare)](https://docs.rs/akshare)
[![CI](https://github.com/Cricle/akshare-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Cricle/akshare-rs/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](#license)

100% pure Rust implementation of [akshare](https://github.com/akfamily/akshare) — unified access to Chinese and global financial market data APIs.

## Features

- **Zero Python dependency** — native Rust HTTP client, no FFI or subprocess calls
- **Async-first** — built on `tokio` + `reqwest`
- **Comprehensive coverage** — A-share, HK, US stocks, funds, bonds, futures, options, forex, crypto, macro data, and more
- **Multiple data sources** — Eastmoney, Sina, Tencent, Yahoo, Stooq, SEC EDGAR, Tushare
- **Typed responses** — all data returned as strongly-typed Rust structs

## Quick Start

```toml
[dependencies]
akshare = "0.1"
```

```rust,no_run
use akshare::AkShareClient;

# async fn example() -> Result<(), akshare::Error> {
let client = AkShareClient::new();

// A-share real-time quote
let quote = client.a_share_quote("600000").await?;

// A-share K-line candles
let candles = client.a_share_candles("600000", "qfq", 60).await?;

// HK stock quote
let hk_quote = client.hk_quote("00593").await?;

// US stock candles
let us_candles = client.us_candles("AAPL", 30).await?;
# Ok(())
# }
```

## Supported Data

| Category | Examples |
|----------|---------|
| **A-share** | Quotes, candles, sectors, billboard, capital flow, financials, fundamentals |
| **HK** | Quotes, candles, news |
| **US** | Daily candles, quotes |
| **Funds** | ETF, LOF, ranked lists, holdings, manager info |
| **Bonds** | Government, corporate, convertible, yield curves |
| **Futures** | Domestic exchanges, spot prices, warehouse stocks, COT |
| **Options** | SSE, CZCE, CFFEX, commodity options |
| **Forex** | BOC rates, cross rates, real-time |
| **Crypto** | Bitcoin and major crypto data |
| **Macro** | China, US, EU, UK, Japan, Australia, Canada GDP/CPI/PMI |
| **Index** | CSI, SW, CNI, global indices, VIX |
| **Economy** | Events, articles, NLP sentiment, auto sales, box office |
| **News** | CCTV news, search aggregation |

## Data Sources

| Provider | Coverage |
|----------|----------|
| **Eastmoney** | A-share search, quotes, klines, sectors, billboard, announcements, capital flow |
| **Sina Finance** | A-share real-time, US daily, bonds, forex |
| **Tencent Finance** | A-share/HK real-time quotes and klines |
| **Yahoo Finance** | HK/US/global stock charts |
| **Stooq** | US/global stock CSV data (fallback) |
| **SEC EDGAR** | US company fundamentals and filings |
| **Tushare Pro** | Chinese market daily data, financials, trade calendar |

## MSRV

Rust **1.85** (edition 2024)

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.
