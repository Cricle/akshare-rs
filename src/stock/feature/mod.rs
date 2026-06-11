//! Stock feature functions — comprehensive A-share data access.
//!
//! Implements 200+ functions from akshare's `stock_feature` module,
//! covering spot listings, historical data, billboard, shareholder analysis,
//! financial reports, earnings, dividends, margin trading, pledge data,
//! limit-up/down pools, ESG ratings, analyst rankings, and more.

mod analyst_em;
mod comment_em;
mod disclosure_cninfo;
mod dxsyl_em;
mod dzjy_em;
mod esg_sina;
mod fhps_em;
mod financial_em;
mod fund_flow;
mod gdfx_em;
mod gdhs_em;
mod gdzjc_em;
mod gpzy_em;
mod helpers;
mod hist_em;
mod hot_xq;
mod hsgt_em;
mod industry_cninfo;
mod info_em;
mod inner_trade_xq;
mod irm_cninfo;
mod jgdy_em;
mod lh_yybpm;
mod lhb_em;
mod margin_em;
mod pankou_em;
mod rank_ths;
mod register_em;
mod report_em;
mod spot_em;
mod stock_info;
mod stock_other;
mod sy_em;
mod three_report_em;
mod types;
mod yjbb_em;
mod yjyg_em;
mod zf_pg_em;
mod ztb_em;

pub use stock_info::StockInfo;
pub use types::*;
