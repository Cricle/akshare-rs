//! Stock feature functions — comprehensive A-share data access.
//!
//! Implements 200+ functions from akshare's `stock_feature` module,
//! covering spot listings, historical data, billboard, shareholder analysis,
//! financial reports, earnings, dividends, margin trading, pledge data,
//! limit-up/down pools, ESG ratings, analyst rankings, and more.

mod types;
mod helpers;
mod spot_em;
mod hist_em;
mod hsgt_em;
mod lhb_em;
mod gdfx_em;
mod comment_em;
mod analyst_em;
mod financial_em;
mod margin_em;
mod gpzy_em;
mod sy_em;
mod gdhs_em;
mod gdzjc_em;
mod ztb_em;
mod pankou_em;
mod zf_pg_em;
mod dxsyl_em;
mod yjbb_em;
mod yjyg_em;
mod fhps_em;
mod jgdy_em;
mod hot_xq;
mod inner_trade_xq;
mod esg_sina;
mod irm_cninfo;
mod disclosure_cninfo;
mod info_em;
mod three_report_em;
mod report_em;
mod lh_yybpm;
mod stock_info;
mod rank_ths;
mod register_em;
mod fund_flow;
mod industry_cninfo;
mod stock_other;
mod dzjy_em;

pub use types::*;
pub use stock_info::StockInfo;
