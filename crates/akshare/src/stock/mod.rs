//! Stock data: A-share, HK, US equities from multiple providers.
//!
//! ## Sub-modules
//!
//! | Sub-module | Description |
//! |------------|-------------|
//! | [`feature`] | Real-time quotes, candles, billboard, capital flow, financials |
//! | [`fundamental`] | Financial statements, IPO data, restricted releases |
//! | [`a_share`] | A-share market segment data |
//! | [`hk`] / [`hk_extra`] | Hong Kong stock data |
//! | [`us`] / [`us_extra`] | US stock data with fallback sources |
//! | [`board_em`] / [`board_ths`] | Sector/concept board rankings |
//! | [`sina_stock`] | Sina Finance intraday ticks, sector spot |
//! | [`xueqiu`] | Xueqiu (Snowball) stock data |
//! | `eastmoney_*` | Various Eastmoney data endpoints |

pub mod a_share;
pub mod feature;
pub mod fundamental;
pub mod hk;
pub mod us;

// Eastmoney-based stock modules
pub mod eastmoney_detail;
pub mod eastmoney_fund_flow;
pub mod eastmoney_hot;
pub mod eastmoney_hsgt;
pub mod eastmoney_misc;
pub mod eastmoney_spot;

// Sina-based stock modules
pub mod sina_stock;

// Other provider stock modules
pub mod jin10;
pub mod xueqiu;

// New market-specific modules
pub mod board_em;
pub mod board_ths;
pub mod hk_extra;
pub mod us_extra;
pub mod us_profile;
pub mod zh_a;
pub mod zh_ah;
pub mod zh_b;
pub mod zh_comparison;
pub mod zh_index;
pub mod zh_kcb;
