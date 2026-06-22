//! Type definitions for stock_feature functions.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Spot market data (stock_zh_a_spot_em, stock_sh_a_spot, etc.)
// ---------------------------------------------------------------------------

/// Real-time A-share spot quote from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotQuote {
    pub code: String,
    pub name: String,
    pub latest_price: f64,
    pub change_pct: f64,
    pub change_amount: f64,
    pub volume: f64,
    pub amount: f64,
    pub amplitude_pct: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub prev_close: f64,
    pub volume_ratio: f64,
    pub turnover_rate: f64,
    pub pe_dynamic: f64,
    pub pb: f64,
    pub total_market_cap: f64,
    pub circulating_market_cap: f64,
    pub speed: f64,
    pub change_5min: f64,
    pub change_60d: f64,
    pub change_ytd: f64,
}

// ---------------------------------------------------------------------------
// Historical data
// ---------------------------------------------------------------------------

/// Historical candlestick data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistData {
    pub trade_date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub amount: f64,
    pub amplitude_pct: f64,
    pub change_pct: f64,
    pub change_amount: f64,
    pub turnover_rate: f64,
}

// ---------------------------------------------------------------------------
// Billboard (LHB - Dragon Tiger List)
// ---------------------------------------------------------------------------

/// Billboard detail entry (龙虎榜详情).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbDetail {
    pub code: String,
    pub name: String,
    pub trade_date: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub billboard_net_amount: f64,
    pub billboard_buy_amount: f64,
    pub billboard_sell_amount: f64,
    pub billboard_deal_amount: f64,
    pub total_amount: f64,
    pub net_ratio: f64,
    pub deal_ratio: f64,
    pub turnover_rate: f64,
    pub circulating_market_cap: f64,
    pub reason: String,
    pub explanation: Option<String>,
    pub d1_change: Option<f64>,
    pub d2_change: Option<f64>,
    pub d5_change: Option<f64>,
    pub d10_change: Option<f64>,
}

/// Billboard stock statistics (个股上榜统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbStockStatistic {
    pub code: String,
    pub name: String,
    pub latest_trade_date: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub billboard_times: i64,
    pub billboard_net_amount: f64,
    pub billboard_buy_amount: f64,
    pub billboard_sell_amount: f64,
    pub billboard_total_amount: f64,
    pub org_buy_times: i64,
    pub org_sell_times: i64,
    pub org_net_buy_amount: f64,
    pub org_buy_amount: f64,
    pub org_sell_amount: f64,
    pub m1_change: Option<f64>,
    pub m3_change: Option<f64>,
    pub m6_change: Option<f64>,
    pub y1_change: Option<f64>,
}

/// Billboard institutional daily statistics (机构买卖每日统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbJgmmtj {
    pub code: String,
    pub name: String,
    pub trade_date: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub buy_org_count: i64,
    pub sell_org_count: i64,
    pub org_buy_amount: f64,
    pub org_sell_amount: f64,
    pub org_net_buy_amount: f64,
    pub total_amount: f64,
    pub org_net_ratio: f64,
    pub turnover_rate: f64,
    pub circulating_market_cap: f64,
    pub reason: String,
}

/// Billboard institutional seat tracking (机构席位追踪).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbJgstatistic {
    pub code: String,
    pub name: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub billboard_amount: f64,
    pub billboard_times: i64,
    pub org_buy_amount: f64,
    pub org_buy_times: i64,
    pub org_sell_amount: f64,
    pub org_sell_times: i64,
    pub org_net_buy_amount: f64,
    pub m1_change: Option<f64>,
    pub m3_change: Option<f64>,
    pub m6_change: Option<f64>,
    pub y1_change: Option<f64>,
}

/// Billboard active trading department (每日活跃营业部).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbHyyyb {
    pub dept_name: String,
    pub trade_date: String,
    pub buy_stock_count: i64,
    pub sell_stock_count: i64,
    pub buy_amount: f64,
    pub sell_amount: f64,
    pub net_amount: f64,
    pub buy_stocks: Option<String>,
    pub dept_code: String,
}

/// Billboard trading department ranking (营业部排行).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbYybph {
    pub dept_name: String,
    pub dept_code: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub billboard_amount: f64,
    pub billboard_times: i64,
    pub buy_amount: f64,
    pub buy_times: i64,
    pub sell_amount: f64,
    pub sell_times: i64,
    pub net_buy_amount: f64,
    pub m1_change: Option<f64>,
    pub m3_change: Option<f64>,
    pub m6_change: Option<f64>,
    pub y1_change: Option<f64>,
}

/// Billboard trader statistics (游资统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbTraderStatistic {
    pub trader_name: String,
    pub trader_code: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub billboard_amount: f64,
    pub billboard_times: i64,
    pub buy_amount: f64,
    pub buy_times: i64,
    pub sell_amount: f64,
    pub sell_times: i64,
    pub net_buy_amount: f64,
    pub m1_change: Option<f64>,
    pub m3_change: Option<f64>,
    pub m6_change: Option<f64>,
    pub y1_change: Option<f64>,
}

/// Billboard stock detail date (个股上榜日期).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbStockDetailDate {
    pub trade_date: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub reason: String,
    pub billboard_net_amount: f64,
    pub billboard_buy_amount: f64,
    pub billboard_sell_amount: f64,
}

/// Billboard stock detail (个股龙虎榜详情).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbStockDetail {
    pub trade_date: String,
    pub code: String,
    pub name: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub dept_name: String,
    pub buy_amount: f64,
    pub sell_amount: f64,
    pub net_amount: f64,
    pub reason: String,
    pub side: String,
}

/// Billboard trading department detail (营业部龙虎榜详情).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbYybDetail {
    pub trade_date: String,
    pub code: String,
    pub name: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub buy_amount: f64,
    pub sell_amount: f64,
    pub net_amount: f64,
    pub reason: String,
}

// ---------------------------------------------------------------------------
// Shareholder Analysis (GDFX)
// ---------------------------------------------------------------------------

/// Shareholder holding statistics (股东持股统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdfxHoldingStatistic {
    pub holder_name: String,
    pub holder_type: Option<String>,
    pub statistics_count: Option<i64>,
    pub avg_change_10d: Option<f64>,
    pub max_change_10d: Option<f64>,
    pub min_change_10d: Option<f64>,
    pub avg_change_30d: Option<f64>,
    pub max_change_30d: Option<f64>,
    pub min_change_30d: Option<f64>,
    pub avg_change_60d: Option<f64>,
    pub max_change_60d: Option<f64>,
    pub min_change_60d: Option<f64>,
    pub holding_stocks: Option<String>,
}

/// Shareholder holding change statistics (股东持股变动统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdfxHoldingChange {
    pub holder_name: String,
    pub holder_type: Option<String>,
    pub total_holdings: i64,
    pub new_positions: i64,
    pub increased: i64,
    pub unchanged: i64,
    pub decreased: i64,
    pub circulating_market_cap: Option<f64>,
    pub holding_stocks: Option<String>,
}

/// Shareholder holding detail (股东持股明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdfxHoldingDetail {
    pub holder_name: String,
    pub holder_type: Option<String>,
    pub code: String,
    pub name: String,
    pub report_date: String,
    pub holding_count: f64,
    pub holding_change: Option<f64>,
    pub holding_change_ratio: Option<f64>,
    pub holding_change_name: Option<String>,
    pub circulating_market_cap: Option<f64>,
    pub notice_date: String,
}

/// Shareholder holding analysis (股东持股分析).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdfxHoldingAnalyse {
    pub code: String,
    pub name: String,
    pub report_date: String,
    pub holder_count: Option<i64>,
    pub holder_count_change: Option<i64>,
    pub holder_count_ratio: Option<f64>,
    pub avg_holding_amount: Option<f64>,
    pub avg_market_cap: Option<f64>,
    pub total_market_cap: Option<f64>,
    pub total_shares: Option<f64>,
    pub price: Option<f64>,
    pub change_pct: Option<f64>,
}

/// Top 10 shareholders (十大股东).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdfxTop10 {
    pub rank: i64,
    pub holder_name: String,
    pub holder_nature: Option<String>,
    pub share_type: Option<String>,
    pub holding_count: f64,
    pub holding_ratio: f64,
    pub change: Option<String>,
    pub change_ratio: Option<f64>,
}

// ---------------------------------------------------------------------------
// HSGT (沪深港通 - Northbound/Southbound)
// ---------------------------------------------------------------------------

/// HSGT fund flow summary (沪深港通资金流向).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsgtFundFlowSummary {
    pub trade_date: String,
    pub mutual_type: String,
    pub board_type: Option<String>,
    pub fund_direction: Option<String>,
    pub trade_status: Option<String>,
    pub net_buy_amount: Option<f64>,
    pub fund_net_inflow: Option<f64>,
    pub remaining_quota: Option<f64>,
    pub up_count: Option<i64>,
    pub flat_count: Option<i64>,
    pub down_count: Option<i64>,
    pub index_change_pct: Option<f64>,
}

/// HK Stock Connect components (港股通成份股).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkGgtComponent {
    pub code: String,
    pub name: String,
    pub latest_price: f64,
    pub change_amount: f64,
    pub change_pct: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub prev_close: f64,
    pub volume: f64,
    pub amount: f64,
}

/// HSGT stock holding (沪深港通持股-个股排行).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsgtHoldStock {
    pub code: String,
    pub name: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub holding_shares: f64,
    pub holding_market_cap: f64,
    pub holding_circulating_ratio: f64,
    pub holding_total_ratio: f64,
    pub add_shares: Option<f64>,
    pub add_market_cap: Option<f64>,
    pub add_market_cap_change: Option<f64>,
    pub add_circulating_ratio: Option<f64>,
    pub add_total_ratio: Option<f64>,
    pub sector: Option<String>,
    pub trade_date: String,
}

/// HSGT stock statistics (沪深港通持股-每日个股统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsgtStockStatistic {
    pub trade_date: String,
    pub name: String,
    pub code: String,
    pub holding_market_cap: f64,
    pub holding_shares: f64,
    pub holding_circulating_ratio: f64,
    pub holding_total_ratio: f64,
    pub close_price: f64,
    pub change_pct: f64,
}

/// HSGT institution statistics (沪深港通持股-机构统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsgtInstitutionStatistic {
    pub trade_date: String,
    pub institution_name: String,
    pub holding_market_cap: f64,
    pub holding_shares: f64,
    pub holding_count: i64,
}

/// HSGT historical data (沪深港通历史数据).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsgtHist {
    pub trade_date: String,
    pub fund_net_inflow: f64,
    pub fund_buy_amount: f64,
    pub fund_sell_amount: f64,
    pub index_close: f64,
    pub index_change_pct: f64,
}

/// HSGT board ranking (沪深港通板块排行).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsgtBoardRank {
    pub board_name: String,
    pub holding_market_cap: f64,
    pub holding_shares: f64,
    pub holding_count: i64,
    pub change_pct: Option<f64>,
}

/// HSGT individual stock detail (沪深港通个股详情).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsgtIndividualDetail {
    pub trade_date: String,
    pub holding_shares: f64,
    pub holding_market_cap: f64,
    pub holding_circulating_ratio: f64,
    pub holding_total_ratio: f64,
    pub close_price: f64,
    pub change_pct: f64,
}

// ---------------------------------------------------------------------------
// Comment (千股千评)
// ---------------------------------------------------------------------------

/// Stock comment (千股千评).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockComment {
    pub code: String,
    pub name: String,
    pub latest_price: f64,
    pub change_pct: f64,
    pub turnover_rate: f64,
    pub pe: f64,
    pub main_cost: f64,
    pub org_participation: f64,
    pub total_score: f64,
    pub rise: f64,
    pub current_rank: f64,
    pub focus_index: f64,
    pub trade_date: String,
}

/// Stock comment institution participation (机构参与度).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentOrgParticipation {
    pub trade_date: String,
    pub org_participation: f64,
}

/// Stock comment historical score (历史评分).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentHistScore {
    pub trade_date: String,
    pub score: f64,
}

/// Stock comment focus index (用户关注指数).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentFocusIndex {
    pub trade_date: String,
    pub focus_index: f64,
}

/// Stock comment desire index (市场渴望指数).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentDesireIndex {
    pub trade_date: String,
    pub desire_index: f64,
}

// ---------------------------------------------------------------------------
// Analyst (分析师)
// ---------------------------------------------------------------------------

/// Analyst ranking (分析师排行).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalystRank {
    pub analyst_name: String,
    pub analyst_org: String,
    pub annual_index: f64,
    pub annual_return: f64,
    pub return_3m: f64,
    pub return_6m: f64,
    pub return_12m: f64,
    pub constituent_count: i64,
    pub latest_stock_name: Option<String>,
    pub latest_stock_code: Option<String>,
    pub analyst_id: String,
    pub industry_code: Option<String>,
    pub industry: Option<String>,
    pub update_date: String,
    pub year: String,
}

/// Analyst detail (分析师详情).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalystDetail {
    pub code: String,
    pub name: String,
    pub latest_price: Option<f64>,
    pub change_pct: Option<f64>,
    pub target_price: Option<f64>,
    pub rating: Option<String>,
    pub report_title: Option<String>,
    pub publish_date: Option<String>,
}

// ---------------------------------------------------------------------------
// Financial Reports (三大报表)
// ---------------------------------------------------------------------------

/// Balance sheet entry (资产负债表).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub code: String,
    pub name: String,
    pub notice_date: Option<String>,
    pub total_assets: Option<f64>,
    pub total_liabilities: Option<f64>,
    pub equity: Option<f64>,
    pub cash: Option<f64>,
    pub accounts_receivable: Option<f64>,
    pub inventory: Option<f64>,
    pub accounts_payable: Option<f64>,
    pub advance_receipts: Option<f64>,
    pub total_assets_yoy: Option<f64>,
    pub total_liabilities_yoy: Option<f64>,
    pub debt_ratio: Option<f64>,
    /// 长期借款 (HK/US)
    pub long_term_debt: Option<f64>,
    /// 短期借款 (HK/US)
    pub short_term_debt: Option<f64>,
}

/// Profit sheet entry (利润表).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitSheet {
    pub code: String,
    pub name: String,
    pub notice_date: Option<String>,
    pub total_revenue: Option<f64>,
    pub operating_cost: Option<f64>,
    pub operating_profit: Option<f64>,
    pub total_profit: Option<f64>,
    pub net_profit: Option<f64>,
    pub net_profit_deducted: Option<f64>,
    pub total_revenue_yoy: Option<f64>,
    pub net_profit_yoy: Option<f64>,
    pub gross_margin: Option<f64>,
    pub net_margin: Option<f64>,
    pub roe: Option<f64>,
    pub eps: Option<f64>,
    /// 毛利 (HK/US)
    pub gross_profit: Option<f64>,
    /// 营业费用 (HK/US: selling + admin + R&D)
    pub operating_expenses: Option<f64>,
}

/// Cash flow sheet entry (现金流量表).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlowSheet {
    pub code: String,
    pub name: String,
    pub notice_date: Option<String>,
    pub operating_cash_flow: Option<f64>,
    pub investing_cash_flow: Option<f64>,
    pub financing_cash_flow: Option<f64>,
    pub cash_increase: Option<f64>,
    pub operating_cash_flow_yoy: Option<f64>,
    /// 购建固定资产、无形资产和其他长期资产支付的现金 (HK/US)
    pub capital_expenditure: Option<f64>,
}

// ---------------------------------------------------------------------------
// HK/US Main Financial Indicators
// ---------------------------------------------------------------------------

/// HK main financial indicator (港股主要指标).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkMainIndicator {
    pub report_date: Option<String>,
    pub std_report_date: Option<String>,
    pub currency: Option<String>,
    pub operate_income: Option<f64>,
    pub holder_profit: Option<f64>,
    pub gross_profit: Option<f64>,
    pub total_assets: Option<f64>,
    pub total_liabilities: Option<f64>,
    pub total_parent_equity: Option<f64>,
    pub netcash_operate: Option<f64>,
    pub capital_expenditure: Option<f64>,
    pub total_share: Option<f64>,
    pub current_liability: Option<f64>,
    pub noncurrent_liab_1year: Option<f64>,
}

/// US main financial indicator (美股主要指标).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsMainIndicator {
    pub report_date: Option<String>,
    pub std_report_date: Option<String>,
    pub currency: Option<String>,
    pub operate_income: Option<f64>,
    pub operate_income_yoy: Option<f64>,
    pub gross_profit: Option<f64>,
    pub gross_profit_yoy: Option<f64>,
    pub gross_profit_ratio: Option<f64>,
    pub net_profit_ratio: Option<f64>,
    pub holder_profit: Option<f64>,
    pub holder_profit_yoy: Option<f64>,
    pub basic_eps: Option<f64>,
    pub basic_eps_yoy: Option<f64>,
    pub diluted_eps: Option<f64>,
    pub roe_avg: Option<f64>,
    pub roe_avg_yoy: Option<f64>,
    pub roa: Option<f64>,
    pub roa_yoy: Option<f64>,
    pub debt_asset_ratio: Option<f64>,
    pub debt_asset_ratio_yoy: Option<f64>,
    pub current_ratio: Option<f64>,
    pub current_ratio_yoy: Option<f64>,
    pub speed_ratio: Option<f64>,
    pub equity_ratio: Option<f64>,
    pub total_assets_tr: Option<f64>,
    pub inventory_tr: Option<f64>,
    pub accounts_rece_tr: Option<f64>,
    // Always None for US (not in this API), kept for compatibility
    pub total_share: Option<f64>,
    pub bps: Option<f64>,
}

// ---------------------------------------------------------------------------
// Earnings (业绩报表/业绩快报/业绩预告)
// ---------------------------------------------------------------------------

/// Earnings report (业绩报表).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsReport {
    pub code: String,
    pub name: String,
    pub eps: f64,
    pub total_revenue: f64,
    pub total_revenue_yoy: f64,
    pub total_revenue_qoq: f64,
    pub net_profit: f64,
    pub net_profit_yoy: f64,
    pub net_profit_qoq: f64,
    pub bvps: f64,
    pub roe: f64,
    pub operating_cash_flow_per_share: f64,
    pub gross_margin: f64,
    pub industry: Option<String>,
    pub notice_date: String,
}

/// Earnings quick report (业绩快报).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsQuickReport {
    pub code: String,
    pub name: String,
    pub notice_date: String,
    pub eps: Option<f64>,
    pub bvps: Option<f64>,
    pub total_revenue: Option<f64>,
    pub net_profit: Option<f64>,
    pub revenue_yoy: Option<f64>,
    pub net_profit_yoy: Option<f64>,
    pub roe: Option<f64>,
    pub net_margin: Option<f64>,
}

/// Earnings forecast (业绩预告).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsForecast {
    pub code: String,
    pub name: String,
    pub notice_date: String,
    pub forecast_type: String,
    pub forecast_content: Option<String>,
    pub change_range: Option<String>,
    pub change_reason: Option<String>,
    pub previous_year_net_profit: Option<f64>,
}

// ---------------------------------------------------------------------------
// Dividends (分红送配)
// ---------------------------------------------------------------------------

/// Dividend/rights entry (分红送配).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividendInfo {
    pub code: String,
    pub name: String,
    pub bonus_shares_ratio: Option<f64>,
    pub transfer_ratio: Option<f64>,
    pub convert_ratio: Option<f64>,
    pub cash_dividend_ratio: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub eps: Option<f64>,
    pub bvps: Option<f64>,
    pub capital_reserve_per_share: Option<f64>,
    pub undistributed_profit_per_share: Option<f64>,
    pub net_profit_yoy: Option<f64>,
    pub total_shares: Option<f64>,
    pub plan_notice_date: Option<String>,
    pub record_date: Option<String>,
    pub ex_date: Option<String>,
    pub plan_progress: Option<String>,
    pub latest_notice_date: Option<String>,
}

// ---------------------------------------------------------------------------
// Margin Trading (融资融券)
// ---------------------------------------------------------------------------

/// Margin account info (融资融券账户统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginAccountInfo {
    pub date: String,
    pub fin_balance: f64,
    pub loan_balance: f64,
    pub fin_buy_amount: f64,
    pub loan_sell_amount: f64,
    pub security_org_count: Option<i64>,
    pub dept_count: Option<i64>,
    pub personal_investor_count: Option<i64>,
    pub org_investor_count: Option<i64>,
    pub active_investor_count: Option<i64>,
    pub margin_liability_investor_count: Option<i64>,
    pub total_guarantee: Option<f64>,
    pub avg_guarantee_ratio: Option<f64>,
}

// ---------------------------------------------------------------------------
// Pledge Data (股权质押)
// ---------------------------------------------------------------------------

/// Pledge market overview (股权质押市场概况).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpzyProfile {
    pub trade_date: String,
    pub pledge_ratio: f64,
    pub company_count: i64,
    pub pledge_count: i64,
    pub total_shares: f64,
    pub total_market_cap: f64,
    pub hs300_index: f64,
    pub change_pct: f64,
}

/// Pledge ratio (上市公司质押比例).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpzyPledgeRatio {
    pub code: String,
    pub name: String,
    pub trade_date: String,
    pub industry: Option<String>,
    pub pledge_ratio: f64,
    pub pledge_shares: f64,
    pub pledge_market_cap: f64,
    pub pledge_count: i64,
    pub untradable_pledge: f64,
    pub tradable_pledge: f64,
    pub y1_change: Option<f64>,
    pub industry_code: Option<String>,
}

/// Pledge detail (重要股东股权质押明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpzyPledgeDetail {
    pub notice_date: String,
    pub code: String,
    pub name: String,
    pub holder_name: String,
    pub pledge_shares: f64,
    pub pledge_ratio: f64,
    pub pledge_market_cap: f64,
    pub pledge_type: Option<String>,
    pub receiver: Option<String>,
}

/// Pledge distribution (质押机构分布统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpzyDistribute {
    pub org_name: String,
    pub pledge_count: i64,
    pub pledge_shares: f64,
    pub pledge_market_cap: f64,
}

/// Pledge industry data (行业数据).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpzyIndustry {
    pub industry: String,
    pub pledge_ratio: f64,
    pub company_count: i64,
    pub pledge_count: i64,
    pub total_shares: f64,
    pub total_market_cap: f64,
}

// ---------------------------------------------------------------------------
// Goodwill (商誉)
// ---------------------------------------------------------------------------

/// Goodwill market overview (A股商誉市场概况).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyProfile {
    pub report_date: String,
    pub goodwill: f64,
    pub goodwill_impairment: f64,
    pub net_assets: f64,
    pub goodwill_to_net_assets_ratio: f64,
    pub impairment_to_net_assets_ratio: f64,
    pub net_profit: f64,
    pub impairment_to_net_profit_ratio: f64,
}

/// Goodwill prediction detail (商誉减值预期明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyYqDetail {
    pub code: String,
    pub name: String,
    pub report_date: String,
    pub notice_date: String,
    pub goodwill: f64,
    pub predicted_impairment: f64,
    pub net_profit: f64,
    pub impairment_reason: Option<String>,
}

/// Goodwill impairment detail (个股商誉减值明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyJzDetail {
    pub code: String,
    pub name: String,
    pub report_date: String,
    pub notice_date: String,
    pub goodwill: f64,
    pub impairment_amount: f64,
    pub net_profit: f64,
}

/// Goodwill detail (个股商誉明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyDetail {
    pub code: String,
    pub name: String,
    pub report_date: String,
    pub notice_date: String,
    pub goodwill: f64,
    pub goodwill_change: f64,
    pub net_profit: f64,
    pub goodwill_to_net_assets_ratio: f64,
}

/// Goodwill industry data (行业商誉).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyIndustry {
    pub industry: String,
    pub report_date: String,
    pub company_count: i64,
    pub goodwill: f64,
    pub impairment: f64,
    pub net_profit: f64,
}

// ---------------------------------------------------------------------------
// Shareholder Count (股东户数)
// ---------------------------------------------------------------------------

/// Shareholder count (股东户数).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gdhs {
    pub code: String,
    pub name: String,
    pub latest_price: Option<f64>,
    pub change_pct: Option<f64>,
    pub holder_count: f64,
    pub prev_holder_count: f64,
    pub holder_change: f64,
    pub holder_change_ratio: f64,
    pub interval_change_pct: Option<f64>,
    pub current_end_date: String,
    pub prev_end_date: String,
    pub avg_market_cap: Option<f64>,
    pub avg_shares: Option<f64>,
    pub total_market_cap: Option<f64>,
    pub total_shares: Option<f64>,
    pub notice_date: String,
}

/// Shareholder count detail (股东户数详情).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdhsDetail {
    pub end_date: String,
    pub holder_count: f64,
    pub prev_holder_count: f64,
    pub holder_change: f64,
    pub holder_change_ratio: f64,
    pub avg_market_cap: Option<f64>,
    pub avg_shares: Option<f64>,
    pub total_market_cap: Option<f64>,
    pub total_shares: Option<f64>,
    pub notice_date: String,
    pub interval_change_pct: Option<f64>,
}

// ---------------------------------------------------------------------------
// Shareholder Changes (高管持股/股东增减持)
// ---------------------------------------------------------------------------

/// Executive shareholding changes (高管持股).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ggcg {
    pub code: String,
    pub name: String,
    pub latest_price: Option<f64>,
    pub change_pct: Option<f64>,
    pub holder_name: String,
    pub direction: String,
    pub change_amount: f64,
    pub change_total_ratio: f64,
    pub change_circulating_ratio: f64,
    pub holding_count: f64,
    pub holding_total_ratio: f64,
    pub holding_circulating_count: f64,
    pub holding_circulating_ratio: f64,
    pub start_date: String,
    pub end_date: String,
    pub notice_date: String,
}

// ---------------------------------------------------------------------------
// Limit-up/Down Pools (涨停/跌停股池)
// ---------------------------------------------------------------------------

/// Limit-up pool entry (涨停股池).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZtPool {
    pub code: String,
    pub name: String,
    pub change_pct: f64,
    pub latest_price: f64,
    pub amount: f64,
    pub circulating_market_cap: f64,
    pub total_market_cap: f64,
    pub turnover_rate: f64,
    pub seal_amount: f64,
    pub first_seal_time: String,
    pub last_seal_time: String,
    pub break_count: i64,
    pub zt_statistics: String,
    pub consecutive_count: i64,
    pub industry: String,
}

/// Previous limit-up pool entry (昨日涨停股池).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZtPoolPrevious {
    pub code: String,
    pub name: String,
    pub change_pct: f64,
    pub latest_price: f64,
    pub limit_price: f64,
    pub amount: f64,
    pub circulating_market_cap: f64,
    pub total_market_cap: f64,
    pub turnover_rate: f64,
    pub speed: f64,
    pub amplitude: f64,
    pub prev_seal_time: String,
    pub prev_consecutive_count: i64,
    pub zt_statistics: String,
    pub industry: String,
}

/// Strong stock pool (强势股池).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZtPoolStrong {
    pub code: String,
    pub name: String,
    pub change_pct: f64,
    pub latest_price: f64,
    pub amount: f64,
    pub circulating_market_cap: f64,
    pub total_market_cap: f64,
    pub turnover_rate: f64,
    pub seal_amount: f64,
    pub first_seal_time: String,
    pub last_seal_time: String,
    pub break_count: i64,
    pub zt_statistics: String,
    pub consecutive_count: i64,
    pub industry: String,
}

/// Sub-new stock pool (次新股池).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZtPoolSubNew {
    pub code: String,
    pub name: String,
    pub change_pct: f64,
    pub latest_price: f64,
    pub amount: f64,
    pub circulating_market_cap: f64,
    pub total_market_cap: f64,
    pub turnover_rate: f64,
    pub ipo_date: String,
    pub industry: String,
}

/// Broken seal pool (炸板股池).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZtPoolZbgc {
    pub code: String,
    pub name: String,
    pub change_pct: f64,
    pub latest_price: f64,
    pub amount: f64,
    pub circulating_market_cap: f64,
    pub total_market_cap: f64,
    pub turnover_rate: f64,
    pub seal_amount: f64,
    pub first_seal_time: String,
    pub last_seal_time: String,
    pub break_count: i64,
    pub zt_statistics: String,
    pub industry: String,
}

/// Limit-down pool (跌停股池).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZtPoolDtgc {
    pub code: String,
    pub name: String,
    pub change_pct: f64,
    pub latest_price: f64,
    pub amount: f64,
    pub circulating_market_cap: f64,
    pub total_market_cap: f64,
    pub turnover_rate: f64,
    pub seal_amount: f64,
    pub first_seal_time: String,
    pub last_seal_time: String,
    pub open_count: i64,
    pub industry: String,
}

// ---------------------------------------------------------------------------
// Order Book Changes (盘口异动)
// ---------------------------------------------------------------------------

/// Order book change entry (盘口异动).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PankouChange {
    pub time: String,
    pub code: String,
    pub name: String,
    pub board: String,
    pub info: Option<String>,
}

// ---------------------------------------------------------------------------
// IPO Subscription (打新收益率/新股申购)
// ---------------------------------------------------------------------------

/// IPO subscription yield (打新收益率).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dxsyl {
    pub code: String,
    pub name: String,
    pub issue_price: f64,
    pub latest_price: Option<f64>,
    pub online_lottery_rate: f64,
    pub online_valid_shares: f64,
    pub online_valid_accounts: f64,
    pub online_oversubscribe_ratio: f64,
    pub offline_lottery_rate: f64,
    pub offline_valid_shares: f64,
    pub offline_valid_accounts: f64,
    pub offline_oversubscribe_ratio: f64,
    pub total_issue_shares: f64,
    pub open_premium: f64,
    pub first_day_change: f64,
    pub listing_date: Option<String>,
}

/// New stock subscription list (新股申购与中签查询).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Xgsglb {
    pub code: String,
    pub name: String,
    pub apply_code: Option<String>,
    pub issue_price: Option<f64>,
    pub latest_price: Option<f64>,
    pub issue_pe_ratio: Option<f64>,
    pub apply_date: Option<String>,
    pub listing_date: Option<String>,
    pub online_lottery_rate: Option<f64>,
    pub first_day_close: Option<f64>,
    pub first_day_change: Option<f64>,
    pub first_day_turnover: Option<f64>,
    pub main_business: Option<String>,
}

// ---------------------------------------------------------------------------
// Rights Issue (增发/配股)
// ---------------------------------------------------------------------------

/// SEO (Secondary Equity Offering) entry (全部增发).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qbzf {
    pub code: String,
    pub name: String,
    pub seo_code: String,
    pub issue_type: String,
    pub issue_count: f64,
    pub online_issue: Option<f64>,
    pub issue_price: f64,
    pub latest_price: Option<f64>,
    pub issue_date: String,
    pub listing_date: Option<String>,
    pub lock_period: Option<String>,
}

/// Rights issue entry (配股).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pg {
    pub code: String,
    pub name: String,
    pub pg_code: String,
    pub issue_price: f64,
    pub latest_price: Option<f64>,
    pub issue_count: f64,
    pub plan_notice_date: Option<String>,
    pub record_date: Option<String>,
    pub pay_date: Option<String>,
    pub listing_date: Option<String>,
    pub issue_type: Option<String>,
}

// ---------------------------------------------------------------------------
// Institutional Research (机构调研)
// ---------------------------------------------------------------------------

/// Institutional research statistics (机构调研统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JgdyTj {
    pub code: String,
    pub name: String,
    pub latest_price: Option<f64>,
    pub change_pct: Option<f64>,
    pub org_count: i64,
    pub receive_method: Option<String>,
    pub receive_personnel: Option<String>,
    pub receive_location: Option<String>,
    pub receive_date: String,
    pub notice_date: String,
}

/// Institutional research detail (机构调研详细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JgdyDetail {
    pub code: String,
    pub name: String,
    pub notice_date: String,
    pub receive_date: String,
    pub receive_location: Option<String>,
    pub receive_method: Option<String>,
    pub receive_personnel: Option<String>,
    pub org_count: i64,
    pub content: Option<String>,
}

// ---------------------------------------------------------------------------
// Xueqiu Hot Stocks
// ---------------------------------------------------------------------------

/// Xueqiu hot stock entry (雪球热度排行).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotStockXq {
    pub code: String,
    pub name: String,
    pub value: f64,
    pub latest_price: f64,
}

// ---------------------------------------------------------------------------
// Insider Trading (内部交易)
// ---------------------------------------------------------------------------

/// Insider trading entry (内部交易).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerTradeXq {
    pub code: String,
    pub name: String,
    pub change_date: String,
    pub changer: String,
    pub change_shares: f64,
    pub avg_price: f64,
    pub holding_after_change: f64,
    pub relationship: String,
    pub position: String,
}

// ---------------------------------------------------------------------------
// ESG (ESG评级)
// ---------------------------------------------------------------------------

/// ESG rating entry (ESG评级).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsgRating {
    pub code: String,
    pub name: Option<String>,
    pub esg_score: Option<f64>,
    pub env_score: Option<f64>,
    pub social_score: Option<f64>,
    pub governance_score: Option<f64>,
    pub rating_date: Option<String>,
    pub market: Option<String>,
}

// ---------------------------------------------------------------------------
// Investor Relations (互动易)
// ---------------------------------------------------------------------------

/// IRM question (互动易-提问).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrmQuestion {
    pub code: String,
    pub company_name: String,
    pub question: String,
    pub answer: Option<String>,
    pub questioner: Option<String>,
    pub answerer: Option<String>,
    pub question_time: String,
    pub source: Option<String>,
}

// ---------------------------------------------------------------------------
// Disclosure (信息披露)
// ---------------------------------------------------------------------------

/// Disclosure report entry (信息披露公告).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisclosureReport {
    pub code: Option<String>,
    pub name: Option<String>,
    pub title: String,
    pub publish_date: String,
    pub url: Option<String>,
    pub category: Option<String>,
}

// ---------------------------------------------------------------------------
// News/Info
// ---------------------------------------------------------------------------

/// News entry (财经快讯).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsEntry {
    pub title: String,
    pub summary: Option<String>,
    pub time: String,
    pub url: Option<String>,
}

// ---------------------------------------------------------------------------
// Three Reports (个股三大报表)
// ---------------------------------------------------------------------------

/// Per-stock financial report entry (个股财务报表).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeReportEntry {
    pub report_date: String,
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Trading Department Rankings (营业部排名)
// ---------------------------------------------------------------------------

/// Trading department ranking entry (营业部排名).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhYybRanking {
    pub dept_name: String,
    pub dept_code: Option<String>,
    pub metric_value: f64,
    pub extra: Option<String>,
}

// ---------------------------------------------------------------------------
// Freeholding Teamwork (股东协同)
// ---------------------------------------------------------------------------

/// Shareholder teamwork entry (股东协同).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdfxTeamwork {
    pub holder_name: String,
    pub holder_type: Option<String>,
    pub holding_count: i64,
    pub total_market_cap: Option<f64>,
    pub holding_stocks: Option<String>,
}

// ---------------------------------------------------------------------------
// Margin Trading - SSE/SZSE detail
// ---------------------------------------------------------------------------

/// SSE margin summary (上交所融资融券汇总).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginSseSummary {
    pub trade_date: String,
    pub fin_balance: f64,
    pub fin_buy_amount: f64,
    pub loan_volume: f64,
    pub loan_balance: f64,
    pub loan_sell_amount: f64,
    pub margin_balance: f64,
}

/// SSE margin detail (上交所融资融券明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginSseDetail {
    pub trade_date: String,
    pub code: String,
    pub name: String,
    pub fin_balance: f64,
    pub fin_buy_amount: f64,
    pub fin_repay_amount: f64,
    pub loan_volume: f64,
    pub loan_sell_amount: f64,
    pub loan_repay_amount: f64,
}

/// SZSE margin summary (深交所融资融券汇总).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginSzseSummary {
    pub fin_buy_amount: f64,
    pub fin_balance: f64,
    pub loan_sell_amount: f64,
    pub loan_volume: f64,
    pub loan_balance: f64,
    pub margin_balance: f64,
}

/// SZSE margin detail (深交所融资融券明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginSzseDetail {
    pub code: String,
    pub name: String,
    pub fin_buy_amount: f64,
    pub fin_balance: f64,
    pub loan_sell_amount: f64,
    pub loan_volume: f64,
    pub loan_balance: f64,
    pub margin_balance: f64,
}

/// PA margin ratio (平安融资融券保证金比例).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginRatioPa {
    pub code: String,
    pub name: String,
    pub fin_ratio: f64,
    pub loan_ratio: f64,
}

/// SZSE underlying info (深交所标的证券信息).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginUnderlyingInfoSzse {
    pub code: String,
    pub name: String,
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Pledge distribution (质押机构分布统计)
// ---------------------------------------------------------------------------

/// Pledge distribution entry (质押机构分布统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpzyDistributeEntry {
    pub rank: i64,
    pub org_name: String,
    pub company_count: i64,
    pub pledge_count: i64,
    pub pledge_shares: f64,
    pub below_warning_ratio: Option<f64>,
    pub warning_not_sell_ratio: Option<f64>,
    pub sell_line_ratio: Option<f64>,
}

/// Pledge ratio detail (重要股东股权质押明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpzyPledgeRatioDetail {
    pub rank: i64,
    pub code: String,
    pub name: String,
    pub holder_name: String,
    pub pledge_shares: f64,
    pub holding_ratio: f64,
    pub total_ratio: f64,
    pub org_name: Option<String>,
    pub latest_price: Option<f64>,
    pub pledge_close_price: Option<f64>,
    pub estimated_sell_price: Option<f64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: Option<String>,
    pub notice_date: String,
}

// ---------------------------------------------------------------------------
// Block trade (大宗交易)
// ---------------------------------------------------------------------------

/// Block trade daily stats (大宗交易每日统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DzjyMrtj {
    pub code: String,
    pub name: String,
    pub trade_date: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub turnover_rate: f64,
    pub block_trade_count: i64,
    pub block_trade_volume: f64,
    pub block_trade_amount: f64,
    pub block_trade_premium_ratio: f64,
    pub circulating_market_cap: f64,
}

/// Block trade industry stats (大宗交易行业统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DzjyHygtj {
    pub industry: String,
    pub block_trade_count: i64,
    pub block_trade_volume: f64,
    pub block_trade_amount: f64,
    pub avg_premium_ratio: f64,
}

/// Block trade industry daily stats (大宗交易行业每日统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DzjyHyyybtj {
    pub industry: String,
    pub trade_date: String,
    pub block_trade_count: i64,
    pub block_trade_volume: f64,
    pub block_trade_amount: f64,
    pub avg_premium_ratio: f64,
}

/// Block trade seat ranking (大宗交易营业部排行).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DzjyYybph {
    pub dept_name: String,
    pub buy_count: i64,
    pub buy_amount: f64,
    pub sell_count: i64,
    pub sell_amount: f64,
    pub net_amount: f64,
}

// ---------------------------------------------------------------------------
// Account statistics (股票账户统计)
// ---------------------------------------------------------------------------

/// Account statistics (股票账户统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStatistics {
    pub date: String,
    pub new_investor_count: Option<f64>,
    pub new_investor_mom: Option<f64>,
    pub new_investor_yoy: Option<f64>,
    pub total_investor_count: Option<f64>,
    pub a_share_count: Option<f64>,
    pub b_share_count: Option<f64>,
    pub total_market_cap: Option<f64>,
    pub avg_market_cap: Option<f64>,
    pub sh_close: Option<f64>,
    pub sh_change_pct: Option<f64>,
}

// ---------------------------------------------------------------------------
// TFP (停复牌)
// ---------------------------------------------------------------------------

/// TFP info (停复牌信息).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TfpInfo {
    pub rank: i64,
    pub code: String,
    pub name: String,
    pub suspend_time: Option<String>,
    pub suspend_end: Option<String>,
    pub suspend_period: Option<String>,
    pub suspend_reason: Option<String>,
    pub market: Option<String>,
    pub expected_resume: Option<String>,
}

// ---------------------------------------------------------------------------
// Stock value (估值分析)
// ---------------------------------------------------------------------------

/// Stock value analysis (估值分析).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockValue {
    pub trade_date: String,
    pub close_price: f64,
    pub change_pct: f64,
    pub total_market_cap: f64,
    pub circulating_market_cap: f64,
    pub total_shares: f64,
    pub circulating_shares: f64,
    pub pe_ttm: f64,
    pub pe_static: f64,
    pub pb: f64,
    pub peg: f64,
    pub pcf: f64,
    pub ps: f64,
}

// ---------------------------------------------------------------------------
// QSJY (券商业绩月报)
// ---------------------------------------------------------------------------

/// QSJY data (券商业绩月报).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QsjyEntry {
    pub code: String,
    pub name: String,
    pub month_net_profit: Option<f64>,
    pub month_np_yoy: Option<f64>,
    pub month_np_qoq: Option<f64>,
    pub accum_net_profit: Option<f64>,
    pub accum_np_yoy: Option<f64>,
    pub month_revenue: Option<f64>,
    pub month_rev_yoy: Option<f64>,
    pub month_rev_qoq: Option<f64>,
    pub accum_revenue: Option<f64>,
    pub accum_rev_yoy: Option<f64>,
    pub net_assets: Option<f64>,
    pub na_yoy: Option<f64>,
}

// ---------------------------------------------------------------------------
// GDDH (股东大会)
// ---------------------------------------------------------------------------

/// Shareholder meeting (股东大会).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GddhEntry {
    pub code: String,
    pub name: String,
    pub meeting_title: String,
    pub start_date: Option<String>,
    pub equity_record_date: Option<String>,
    pub onsite_record_date: Option<String>,
    pub web_vote_start: Option<String>,
    pub web_vote_end: Option<String>,
    pub decision_notice_date: Option<String>,
    pub notice_date: String,
    pub serial_num: Option<String>,
    pub proposal: Option<String>,
}

// ---------------------------------------------------------------------------
// YZXDR (一致行动人)
// ---------------------------------------------------------------------------

/// YZXDR entry (一致行动人).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YzxdrEntry {
    pub rank: i64,
    pub code: String,
    pub name: String,
    pub actor: String,
    pub holder_rank: Option<i64>,
    pub holding_count: f64,
    pub holding_ratio: f64,
    pub holding_change: f64,
    pub industry: Option<String>,
    pub notice_date: String,
}

// ---------------------------------------------------------------------------
// ZDHTMX (重大合同明细)
// ---------------------------------------------------------------------------

/// Major contract detail (重大合同明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZdhtmxEntry {
    pub rank: i64,
    pub code: String,
    pub name: String,
    pub signatory: Option<String>,
    pub signatory_rel: Option<String>,
    pub counterparty: Option<String>,
    pub counterparty_rel: Option<String>,
    pub contract_type: Option<String>,
    pub contract_name: Option<String>,
    pub amount: f64,
    pub last_year_revenue: Option<f64>,
    pub revenue_ratio: Option<f64>,
    pub latest_revenue: Option<f64>,
    pub sign_date: Option<String>,
    pub notice_date: String,
}

// ---------------------------------------------------------------------------
// News/research (新闻/研报)
// ---------------------------------------------------------------------------

/// Stock news (个股新闻).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockNews {
    pub title: String,
    pub content: Option<String>,
    pub publish_time: String,
    pub source: Option<String>,
    pub url: Option<String>,
}

/// Research report (个股研报).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchReport {
    pub rank: i64,
    pub code: String,
    pub name: String,
    pub title: String,
    pub rating: Option<String>,
    pub org: String,
    pub month_count: Option<i64>,
    pub industry: Option<String>,
    pub date: String,
    pub pdf_url: Option<String>,
}

// ---------------------------------------------------------------------------
// Chip distribution (筹码分布)
// ---------------------------------------------------------------------------

/// Chip distribution data (筹码分布).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyqData {
    pub date: String,
    pub benefit_part: f64,
    pub avg_cost: f64,
    pub pct_90_low: f64,
    pub pct_90_high: f64,
    pub pct_90_concentration: f64,
    pub pct_70_low: f64,
    pub pct_70_high: f64,
    pub pct_70_concentration: f64,
}

// ---------------------------------------------------------------------------
// Rank THS (同花顺排名)
// ---------------------------------------------------------------------------

/// THS rank entry (同花顺排名).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankThsEntry {
    pub code: String,
    pub name: String,
    pub latest_price: Option<f64>,
    pub change_pct: Option<f64>,
    pub volume: Option<f64>,
    pub amount: Option<f64>,
    pub extra: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Register EM (注册制)
// ---------------------------------------------------------------------------

/// Registration info (注册制).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterEntry {
    pub code: String,
    pub name: String,
    pub industry: Option<String>,
    pub list_date: Option<String>,
    pub issue_price: Option<f64>,
    pub pe_ratio: Option<f64>,
    pub extra: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Fund flow (资金流向)
// ---------------------------------------------------------------------------

/// Fund flow entry (资金流向).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundFlowEntry {
    pub code: String,
    pub name: String,
    pub latest_price: Option<f64>,
    pub change_pct: Option<f64>,
    pub turnover_rate: Option<f64>,
    pub inflow: Option<f64>,
    pub outflow: Option<f64>,
    pub net_flow: Option<f64>,
    pub amount: Option<f64>,
    /// Main fund net flow as percentage of total volume (f184, 主力净占比).
    pub net_flow_pct: Option<f64>,
}

/// Sector fund flow rank (板块资金流向排名).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorFundFlowRank {
    pub name: String,
    pub change_pct: Option<f64>,
    pub inflow: Option<f64>,
    pub outflow: Option<f64>,
    pub net_flow: Option<f64>,
    pub leader: Option<String>,
    pub leader_change_pct: Option<f64>,
}

// ---------------------------------------------------------------------------
// Industry CNINFO (巨潮行业)
// ---------------------------------------------------------------------------

/// Industry category (巨潮行业分类).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryCategory {
    pub code: String,
    pub name: String,
    pub industry: Option<String>,
    pub extra: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// LHB Sina (新浪龙虎榜)
// ---------------------------------------------------------------------------

/// LHB Sina detail (新浪龙虎榜每日详情).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbSinaDetail {
    pub rank: i64,
    pub code: String,
    pub name: String,
    pub close_price: f64,
    pub metric_value: f64,
    pub volume: f64,
    pub amount: f64,
    pub indicator: String,
}

/// LHB Sina stock stats (新浪龙虎榜个股统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbSinaGgtj {
    pub code: String,
    pub name: String,
    pub billboard_count: i64,
    pub accum_buy: f64,
    pub accum_sell: f64,
    pub net_amount: f64,
    pub buy_seats: i64,
    pub sell_seats: i64,
}

/// LHB Sina seat stats (新浪龙虎榜营业部统计).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbSinaYytj {
    pub dept_name: String,
    pub billboard_count: i64,
    pub accum_buy: f64,
    pub buy_seats: i64,
    pub accum_sell: f64,
    pub sell_seats: i64,
    pub top_stocks: Option<String>,
}

/// LHB Sina institution tracking (新浪龙虎榜机构追踪).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbSinaJgzz {
    pub code: String,
    pub name: String,
    pub accum_buy: f64,
    pub buy_count: i64,
    pub accum_sell: f64,
    pub sell_count: i64,
    pub net_amount: f64,
}

/// LHB Sina institution detail (新浪龙虎榜机构明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LhbSinaJgmx {
    pub code: String,
    pub name: String,
    pub trade_date: String,
    pub inst_buy_amount: f64,
    pub inst_sell_amount: f64,
}

// ---------------------------------------------------------------------------
// CXINFO holder changes (巨潮股东变动)
// ---------------------------------------------------------------------------

/// Hold change entry (股东持股变动).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldChangeCninfo {
    pub data: serde_json::Value,
}

/// Hold control entry (实际控制人).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldControlCninfo {
    pub data: serde_json::Value,
}

/// Management detail (管理层明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagementDetail {
    pub data: serde_json::Value,
}

/// Share change entry (股本变动).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareChangeCninfo {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Delisted financials (退市财务报表)
// ---------------------------------------------------------------------------

/// Delisted financial report (退市公司财务报表).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelistedReport {
    pub code: String,
    pub name: Option<String>,
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// CG (公司治理) - cninfo
// ---------------------------------------------------------------------------

/// Equity mortgage (股权抵押).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CgEquityMortgage {
    pub data: serde_json::Value,
}

/// Guarantee (担保).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CgGuarantee {
    pub data: serde_json::Value,
}

/// Lawsuit (诉讼).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CgLawsuit {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Legulegu generic (乐咕乐股)
// ---------------------------------------------------------------------------

/// Legulegu data point (乐咕乐股数据点).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LgDataPoint {
    pub date: String,
    pub value: f64,
    pub extra: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// CNINFO generic (巨潮通用)
// ---------------------------------------------------------------------------

/// CNINFO dividend (巨潮分红).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividendCninfo {
    pub data: serde_json::Value,
}

/// CNINFO allotment (巨潮配股).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllotmentCninfo {
    pub data: serde_json::Value,
}

/// CNINFO profile (公司概况).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileCninfo {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// SGT exchange rate (港股通汇率)
// ---------------------------------------------------------------------------

/// Exchange rate entry (汇率).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRateEntry {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Share hold change (持股变动)
// ---------------------------------------------------------------------------

/// Share hold change entry (持股变动).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareHoldChangeEntry {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// SSE/SZSE summary (交易所汇总)
// ---------------------------------------------------------------------------

/// SZSE summary (深交所汇总).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SzseSummaryEntry {
    pub data: serde_json::Value,
}

/// SSE deal daily (上交所每日交易).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseDealDaily {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Stock classify Sina (新浪行业分类)
// ---------------------------------------------------------------------------

/// Sina classify (新浪行业分类).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifySina {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Stock news CX (财联社新闻)
// ---------------------------------------------------------------------------

/// CX main news (财联社重要新闻).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CxMainNews {
    pub title: String,
    pub content: Option<String>,
    pub publish_time: String,
    pub url: Option<String>,
}

// ---------------------------------------------------------------------------
// IPO/forecast (新股/业绩预告)
// ---------------------------------------------------------------------------

/// CNINFO forecast (巨潮业绩预告).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastCninfo {
    pub data: serde_json::Value,
}

/// IPO summary (新股概况).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpoSummaryCninfo {
    pub data: serde_json::Value,
}

/// IPO benefit THS (同花顺打新收益).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpoBenefitThs {
    pub data: serde_json::Value,
}

/// New GH CNINFO (新股过会).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGhCninfo {
    pub data: serde_json::Value,
}

/// New IPO CNINFO (新股IPO).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewIpoCninfo {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// SH/SZ delist (退市)
// ---------------------------------------------------------------------------

/// Delist info (退市信息).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelistInfo {
    pub data: serde_json::Value,
}

/// Name change info (股票更名).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameChangeInfo {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// SZSE area/sector summary
// ---------------------------------------------------------------------------

/// SZSE area summary (深交所地区汇总).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SzseAreaSummary {
    pub data: serde_json::Value,
}

/// SZSE sector summary (深交所板块汇总).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SzseSectorSummary {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// FHPS THS (同花顺分红配送)
// ---------------------------------------------------------------------------

/// FHPS THS (同花顺分红配送).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhpsThs {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// XGSR THS (同花顺新股上市)
// ---------------------------------------------------------------------------

/// XGSR THS (同花顺新股上市日).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XgsrThs {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// ESG Sina (新浪ESG)
// ---------------------------------------------------------------------------

/// ESG rate Sina (新浪ESG评级).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsgRateSina {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Price JS (JS价格)
// ---------------------------------------------------------------------------

/// JS price (JS实时价格).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceJs {
    pub code: String,
    pub name: String,
    pub price: f64,
    pub change_pct: f64,
    pub volume: f64,
    pub amount: f64,
}

// ---------------------------------------------------------------------------
// Industry change (行业变动)
// ---------------------------------------------------------------------------

/// Industry change entry (行业变动).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryChange {
    pub data: serde_json::Value,
}

/// Industry PE ratio (行业市盈率).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryPeRatio {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Congestion/GXL/Buffett/EBS Legulegu (乐咕乐股指标)
// ---------------------------------------------------------------------------

/// Congestion data (乐咕乐股拥堵度).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongestionLg {
    pub data: serde_json::Value,
}

/// Dividend yield (乐咕乐股股息率).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GxlLg {
    pub data: serde_json::Value,
}

/// Buffett index (乐咕乐股巴菲特指标).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffettIndexLg {
    pub data: serde_json::Value,
}

/// EBS data (乐咕乐股等权市净率).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbsLg {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Disclosure (信息披露)
// ---------------------------------------------------------------------------

/// Report disclosure (信息披露).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDisclosure {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// SSE info (上交所互动易)
// ---------------------------------------------------------------------------

/// SSE info (上交所互动易).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseInfo {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Market activity (市场活跃度)
// ---------------------------------------------------------------------------

/// Market activity (市场活跃度).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketActivityLegu {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Sector detail (板块详情)
// ---------------------------------------------------------------------------

/// Sector detail (板块详情).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorDetail {
    pub code: String,
    pub name: String,
    pub latest_price: Option<f64>,
    pub change_pct: Option<f64>,
    pub extra: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Industry CLF SH (申万行业历史)
// ---------------------------------------------------------------------------

/// Industry CLF history (申万行业分类历史).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryClfHistSw {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Futu concept constituents (富途概念成份股)
// ---------------------------------------------------------------------------

/// Futu concept constituent (富途概念成份股).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptConsFutu {
    pub code: String,
    pub name: String,
    pub extra: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Fund hold detail (基金持仓明细)
// ---------------------------------------------------------------------------

/// Fund hold detail (基金持仓明细).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundHoldDetail {
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Main fund flow (主力资金流向)
// ---------------------------------------------------------------------------

/// Main fund flow (主力资金流向).
///
/// Parsed from kline format: "date,main_in,main_out,main_net,super_large_net,large_net,ratio"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainFundFlow {
    pub date: String,
    /// Main fund inflow (主力流入, 元)
    pub main_in: f64,
    /// Main fund outflow (主力流出, 元)
    pub main_out: f64,
    /// Main fund net flow (主力净流入, 元)
    pub main_net: f64,
    /// Super large order net flow (超大单净流入, 元)
    pub super_large_net: f64,
    /// Large order net flow (大单净流入, 元)
    pub large_net: f64,
    /// Main fund net flow as percentage of total volume (主力净占比, %)
    pub net_ratio_pct: f64,
}

/// Market fund flow (市场资金流向).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketFundFlow {
    pub data: serde_json::Value,
}

/// Concept fund flow hist (概念资金流向历史).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptFundFlowHist {
    pub data: serde_json::Value,
}

/// Sector fund flow hist (板块资金流向历史).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorFundFlowHist {
    pub data: serde_json::Value,
}

/// Sector fund flow summary (板块资金流向汇总).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorFundFlowSummary {
    pub data: serde_json::Value,
}
