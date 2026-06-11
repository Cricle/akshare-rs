//! # akshare-rs
//!
//! 100% pure Rust implementation of [akshare](https://github.com/akfamily/akshare) —
//! unified access to Chinese and global financial market data APIs.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use akshare::AkShareClient;
//!
//! # async fn example() -> Result<(), akshare::Error> {
//! let client = AkShareClient::new();
//!
//! // A-share quote
//! let quote = client.a_share_quote("600000").await?;
//!
//! // A-share candles
//! let candles = client.a_share_candles("600000", "qfq", 60).await?;
//!
//! // US stock candles
//! let us_candles = client.us_candles("AAPL", 30).await?;
//!
//! // HK stock quote
//! let hk_quote = client.hk_quote("00593").await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Data Sources
//!
//! | Provider | Coverage |
//! |----------|----------|
//! | **Eastmoney** | A-share search, quotes, klines, sectors, billboard, capital flow |
//! | **Sina Finance** | A-share real-time, US daily, bonds, forex |
//! | **Tencent Finance** | A-share/HK real-time quotes and klines |
//! | **Yahoo Finance** | HK/US/global stock charts |
//! | **Stooq** | US/global stock CSV data (fallback) |
//! | **SEC EDGAR** | US company fundamentals and filings |
//! | **Tushare Pro** | Chinese market daily data, financials, trade calendar |
//!
//! ## Module Overview
//!
//! ### Equity Markets
//!
//! | Module | Description | Functions |
//! |--------|-------------|-----------|
//! | [`stock`] | A-share, HK, US stock data | 433 |
//! | [`index`] | A-share, HK, global indices | 97 |
//!
//! ### Derivatives
//!
//! | Module | Description | Functions |
//! |--------|-------------|-----------|
//! | [`futures`] | Domestic exchanges, spot prices, warehouse stocks | 109 |
//! | [`option`] | SSE, CZCE, CFFEX, commodity options | 48 |
//!
//! ### Funds & Fixed Income
//!
//! | Module | Description | Functions |
//! |--------|-------------|-----------|
//! | [`fund`] | ETF, LOF, ranked lists, holdings | 98 |
//! | [`bond`] | Government, corporate, convertible bonds | 51 |
//! | [`reits`] | REITs data from Eastmoney | 5 |
//!
//! ### Macro & Economy
//!
//! | Module | Description | Functions |
//! |--------|-------------|-----------|
//! | [`macro_data`] | China, US, EU, UK, Japan GDP/CPI/PMI | 423 |
//! | [`economy`] | Events, articles, NLP sentiment | 62 |
//!
//! ### FX, Crypto & Commodities
//!
//! | Module | Description | Functions |
//! |--------|-------------|-----------|
//! | [`forex`] | BOC rates, cross rates, real-time | 19 |
//! | [`crypto`] | Bitcoin and major crypto data | 4 |
//! | [`commodity`] | Commodity prices, carbon trading | 9 |
//! | [`spot`] | Spot market prices (SGE, hog, futures) | 14 |
//!
//! ### Other
//!
//! | Module | Description | Functions |
//! |--------|-------------|-----------|
//! | [`news`] | Financial news from multiple sources | 6 |
//! | [`bank`] | Banking regulatory data | 1 |
//! | [`cal`] | Calendar, volatility calculations | 2 |
//! | [`tool`] | Trade calendar, utilities | 2 |
//! | [`provider`] | Data provider abstractions | 12 |
//!
//! ## MSRV
//!
//! Rust **1.85** (edition 2024)

// Equity Markets
pub mod index;
pub mod stock;

// Derivatives
pub mod futures;
pub mod option;

// Funds & Fixed Income
pub mod bond;
pub mod fund;
pub mod reits;

// Macro & Economy
pub mod economy;
pub mod macro_data;

// FX, Crypto & Commodities
pub mod commodity;
pub mod crypto;
pub mod forex;
pub mod spot;

// Other
pub mod bank;
pub mod cal;
pub mod news;
pub mod provider;
pub mod tool;

// Internal
mod client;
mod error;
pub mod market;
pub mod types;
mod util;

pub use client::{AkShareClient, AkShareClientBuilder};
pub use error::{Error, ErrorKind, Result};
pub use market::{detect_market, normalize_a_share_symbol, normalize_hk_symbol};
pub use types::*;
