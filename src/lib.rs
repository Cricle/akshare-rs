//! # akshare-rust
//!
//! 100% pure Rust implementation of [akshare](https://github.com/akfamily/akshare) —
//! unified access to Chinese and global financial market data APIs.
//!
//! ## Supported data sources
//!
//! - **Tencent Finance** — A-share/HK real-time quotes and klines
//! - **Eastmoney** — A-share search, klines, sectors, billboard, announcements, capital flow, financials, macro data
//! - **Sina Finance** — A-share real-time quotes, US daily data
//! - **Yahoo Finance** — HK/US/global stock charts
//! - **Stooq** — US/global stock CSV data (fallback)
//! - **SEC EDGAR** — US company fundamentals and filings
//! - **Tushare Pro** — Chinese market daily data, financials, trade calendar
//!
//! ## Usage
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

pub mod bank;
pub mod cal;
mod client;
mod error;
pub mod market;
pub mod provider;
pub mod stock;
pub mod index;
pub mod fund;
pub mod bond;
pub mod futures;
pub mod option;
pub mod macro_data;
pub mod forex;
pub mod crypto;
pub mod commodity;
pub mod spot;
pub mod reits;
pub mod economy;
pub mod news;
pub mod tool;
pub mod types;
mod util;

pub use client::{AkShareClient, AkShareClientBuilder};
pub use error::{Error, ErrorKind, Result};
pub use types::*;
pub use market::{detect_market, normalize_a_share_symbol, normalize_hk_symbol};
