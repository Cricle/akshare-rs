//! Option data — comprehensive option market data from multiple sources.
//!
//! ## Data sources
//!
//! - **Eastmoney** — Current-day options, minute data, analysis (premium/value/risk), billboard
//! - **Sina Finance** — CFFEX index options (SZ50, HS300, ZZ1000), SSE ETF options
//! - **SSE/SZSE** — Current day contracts, daily statistics, risk indicators, finance board
//! - **Commodity exchanges** — DCE, CZCE, SHFE, GFEX daily option data
//! - **Other** — Commission info (9qihuo), margin data (iweiai), CTP contract info (openctp)

pub mod em;
pub mod cffex_sina;
pub mod sse_sina;
pub mod commodity;
pub mod commodity_sina;
pub mod current_sse;
pub mod daily_stats;
pub mod finance;
pub mod lhb_em;
pub mod margin;
pub mod risk_indicator;
pub mod analysis_em;
pub mod comm_qihuo;
pub mod contract_info_ctp;
pub mod czce;
