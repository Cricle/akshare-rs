//! Shared types used across all modules.
//!
//! Core data structures: [`QuoteSnapshot`], [`CandlePoint`], [`MacroDataPoint`],
//! [`Row`] (flexible key-value map), [`MarketKind`], and more.

pub mod value_ext;
pub mod wire;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarketKind {
    AShare,
    HongKong,
    UsEquity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteSnapshot {
    pub symbol: String,
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlePoint {
    pub trade_date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: i64,
    pub amount: f64,
    pub amplitude_pct: f64,
    pub change_pct: f64,
    pub change_amount: f64,
    pub turnover_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    pub published_at: String,
    pub title: String,
    pub summary: String,
    pub source: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalFlowPoint {
    pub trade_date: String,
    pub main_net_inflow: f64,
    pub small_net_inflow: f64,
    pub medium_net_inflow: f64,
    pub large_net_inflow: f64,
    pub super_large_net_inflow: f64,
    pub main_net_inflow_ratio_pct: f64,
    pub small_net_inflow_ratio_pct: f64,
    pub medium_net_inflow_ratio_pct: f64,
    pub large_net_inflow_ratio_pct: f64,
    pub super_large_net_inflow_ratio_pct: f64,
    pub close: f64,
    pub change_pct: f64,
}

/// Fundamentals snapshot for a stock.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FundamentalsSnapshot {
    pub symbol: String,
    pub company_name: String,
    pub cik: String,
    pub industry: Option<String>,
    pub currency: String,
    pub fiscal_year_end: Option<String>,
    pub shares_outstanding: Option<i64>,
    pub market_cap: Option<f64>,
    pub net_income_usd: Option<f64>,
    pub revenues_usd: Option<f64>,
    pub assets_usd: Option<f64>,
    pub liabilities_usd: Option<f64>,
    pub stockholders_equity_usd: Option<f64>,
    pub cash_and_equivalents_usd: Option<f64>,
    pub gross_profit_usd: Option<f64>,
    pub operating_income_usd: Option<f64>,
    pub operating_expenses_usd: Option<f64>,
    pub operating_cash_flow_usd: Option<f64>,
    pub capital_expenditure_usd: Option<f64>,
    pub free_cash_flow_usd: Option<f64>,
    pub long_term_debt_usd: Option<f64>,
    pub current_debt_usd: Option<f64>,
    pub total_debt_usd: Option<f64>,
    pub diluted_shares_outstanding: Option<i64>,
}

/// HK stock financial data from Tencent Finance API.
///
/// Fields are parsed from the `qt.gtimg.cn` real-time quote API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HkFinancialSnapshot {
    pub symbol: String,
    pub name: String,
    pub pe_ttm: Option<f64>,
    pub pb: Option<f64>,
    pub eps: Option<f64>,
    pub bvps: Option<f64>,
    pub market_cap_hkd: Option<f64>,
    /// Total trading amount in HKD.
    pub amount_hkd: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorSnapshot {
    pub sector_code: String,
    pub sector_name: String,
    pub latest_index: f64,
    pub change_pct: f64,
    pub main_net_inflow: f64,
    pub main_net_inflow_ratio_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorConstituent {
    pub symbol: String,
    pub name: String,
    pub latest_price: f64,
    pub change_pct: f64,
    pub main_net_inflow: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockSearchResult {
    pub symbol: String,
    pub name: String,
    pub market: String,
    pub exchange: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementDetail {
    pub art_code: String,
    pub title: String,
    pub published_at: String,
    pub content: String,
    pub pdf_url: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementItem {
    pub art_code: String,
    pub symbol: String,
    pub title: String,
    pub published_at: String,
    pub url: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillboardEntry {
    pub trade_date: String,
    pub symbol: String,
    pub name: String,
    pub close_price: f64,
    pub change_rate_pct: f64,
    pub turnover_rate_pct: Option<f64>,
    pub net_amount: Option<f64>,
    pub buy_amount: Option<f64>,
    pub sell_amount: Option<f64>,
    pub explanation: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillboardSeatDetail {
    pub trade_date: String,
    pub symbol: String,
    pub department_name: String,
    pub buy_amount: Option<f64>,
    pub sell_amount: Option<f64>,
    pub net_amount: Option<f64>,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeCalendarItem {
    pub exchange: String,
    pub calendar_date: String,
    pub is_open: bool,
    pub previous_trade_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroDataPoint {
    pub date: String,
    pub value: f64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSnapshot {
    pub symbol: String,
    pub name: String,
    pub date: String,
    pub close: f64,
    pub change_pct: f64,
    pub volume: f64,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundSnapshot {
    pub symbol: String,
    pub name: String,
    pub date: String,
    pub nav: f64,
    pub acc_nav: f64,
    pub change_pct: f64,
    #[serde(default)]
    pub fund_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondSnapshot {
    pub symbol: String,
    pub name: String,
    pub date: String,
    pub close: f64,
    pub change_pct: f64,
    pub yield_rate: Option<f64>,
    #[serde(default)]
    pub credit_rating: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesSnapshot {
    pub symbol: String,
    pub name: String,
    pub date: String,
    pub close: f64,
    pub change_pct: f64,
    pub volume: f64,
    pub open_interest: f64,
    #[serde(default)]
    pub settlement_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionSnapshot {
    pub symbol: String,
    pub name: String,
    pub date: String,
    pub close: f64,
    pub change_pct: f64,
    pub volume: f64,
    pub open_interest: f64,
    #[serde(default)]
    pub strike_price: Option<f64>,
    #[serde(default)]
    pub expiry_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForexRate {
    pub currency_pair: String,
    pub buy_rate: f64,
    pub sell_rate: f64,
    pub middle_rate: f64,
    pub date: String,
    #[serde(default)]
    pub change_pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoSpot {
    pub symbol: String,
    pub name: String,
    pub price_usd: f64,
    pub price_cny: f64,
    pub change_24h_pct: f64,
    pub volume_24h: f64,
    pub market_cap: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReitSnapshot {
    pub symbol: String,
    pub name: String,
    pub date: String,
    pub close: f64,
    pub change_pct: f64,
    pub volume: f64,
    #[serde(default)]
    pub nav: Option<f64>,
}

/// Generic row type for flexible tabular data returned by many APIs.
pub type Row = std::collections::HashMap<String, serde_json::Value>;

/// Futures daily bar (OHLCV + settlement) across exchanges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesDailyBar {
    pub symbol: String,
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub open_interest: f64,
    pub turnover: f64,
    pub settle: f64,
    pub pre_settle: f64,
    pub variety: String,
}

/// Futures position rank entry for a single member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesPositionRank {
    pub rank: i32,
    pub vol_party_name: String,
    pub vol: f64,
    pub vol_chg: f64,
    pub long_party_name: String,
    pub long_open_interest: f64,
    pub long_open_interest_chg: f64,
    pub short_party_name: String,
    pub short_open_interest: f64,
    pub short_open_interest_chg: f64,
    pub symbol: String,
    pub variety: String,
}

/// Futures realtime quote from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesRealtimeQuote {
    pub symbol: String,
    pub time: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub current_price: f64,
    pub bid_price: f64,
    pub ask_price: f64,
    pub buy_vol: f64,
    pub sell_vol: f64,
    pub hold: f64,
    pub volume: f64,
    pub avg_price: f64,
    pub last_close: f64,
    pub last_settle_price: f64,
}

/// Foreign commodity realtime quote from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignCommodityQuote {
    pub symbol: String,
    pub current_price: f64,
    pub current_price_rmb: f64,
    pub change_amount: f64,
    pub change_pct: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub last_settle_price: f64,
    pub hold: f64,
    pub bid: f64,
    pub ask: f64,
    pub time: String,
    pub date: String,
}

/// Futures kline point (minute or daily) from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesKlinePoint {
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub hold: f64,
}

/// Eastmoney futures historical kline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesHistKline {
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub change_amount: f64,
    pub change_pct: f64,
    pub volume: f64,
    pub amount: f64,
    pub open_interest: f64,
}

/// Global futures kline from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalFuturesKline {
    pub date: String,
    pub code: String,
    pub name: String,
    pub open: f64,
    pub latest_price: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub change_pct: f64,
    pub open_interest: f64,
    pub open_interest_chg: f64,
}

// ---------------------------------------------------------------------------
// Index-specific types
// ---------------------------------------------------------------------------

/// Generic index spot item (A-share / HK / Global indices).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSpotItem {
    pub code: String,
    pub name: String,
    pub close: f64,
    pub change_pct: f64,
    pub change_amount: f64,
    pub volume: f64,
    pub amount: f64,
    pub amplitude_pct: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub prev_close: f64,
}

/// HK index spot item with internal market id.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkIndexSpotItem {
    pub code: String,
    pub internal_id: String,
    pub name: String,
    pub close: f64,
    pub change_amount: f64,
    pub change_pct: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub prev_close: f64,
    pub volume: f64,
    pub amount: f64,
}

/// CX / Caixin index time-series point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CxIndexPoint {
    pub date: String,
    pub value: f64,
    pub change: f64,
}

/// QVIX daily OHLC point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QvixDailyPoint {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

/// QVIX intraday point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QvixMinPoint {
    pub time: String,
    pub qvix: f64,
}

/// CNIndex overview item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CniIndexItem {
    pub code: String,
    pub name: String,
    pub sample_count: f64,
    pub close: f64,
    pub change_pct: f64,
    pub pe_ttm: f64,
    pub volume: f64,
    pub amount: f64,
    pub total_mv: f64,
    pub free_circ_mv: f64,
}

/// CNIndex daily history point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CniHistPoint {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub change_pct: f64,
    pub volume: f64,
    pub amount: f64,
}

/// CSIndex history point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsindexHistPoint {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub change: f64,
    pub change_pct: f64,
    pub volume: f64,
    pub amount: f64,
    pub sample_count: f64,
    pub pe_ttm: f64,
}

/// Shenwan research index history point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwResearchHistPoint {
    pub code: String,
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub amount: f64,
}

/// Shenwan research index realtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwResearchRealtime {
    pub code: String,
    pub name: String,
    pub prev_close: f64,
    pub change_pct: f64,
    pub year_change_pct: f64,
}

/// Shenwan research component stock.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwResearchComponent {
    pub symbol: String,
    pub name: String,
    pub weight: f64,
    pub date: String,
}

/// Shenwan analysis daily point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwAnalysisPoint {
    pub code: String,
    pub name: String,
    pub date: String,
    pub close: f64,
    pub volume: f64,
    pub change_pct: f64,
    pub turnover_rate: f64,
    pub pe: f64,
    pub pb: f64,
    pub avg_price: f64,
    pub amount_ratio: f64,
    pub circ_mv: f64,
    pub avg_circ_mv: f64,
    pub dividend_yield: f64,
}

/// Shenwan fund index realtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwFundRealtime {
    pub code: String,
    pub name: String,
    pub prev_close: f64,
    pub change_pct: f64,
    pub year_change_pct: f64,
}

/// Shenwan fund index history point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwFundHistPoint {
    pub date: String,
    pub close: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub change_pct: f64,
}

/// CFLP logistics index point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CflpIndexPoint {
    pub date: String,
    pub base_index: f64,
    pub mom_index: f64,
    pub yoy_index: f64,
}

/// Sugar index point (msweet).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SugarIndexPoint {
    pub date: String,
    pub composite_price: f64,
    pub raw_sugar_price: f64,
    pub spot_price: f64,
}

/// Hog index point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HogIndexPoint {
    pub date: String,
    pub index: f64,
    pub ma4: f64,
    pub ma6: f64,
    pub ma12: f64,
    pub presale_avg: f64,
    pub deal_avg: f64,
    pub deal_avg_weight: f64,
}

/// ERI (emission rights) index point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EriIndexPoint {
    pub date: String,
    pub trade_index: f64,
    pub volume: f64,
    pub amount: f64,
}

/// Spot goods price index point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotGoodsPoint {
    pub date: String,
    pub index: f64,
    pub change_amount: f64,
    pub change_pct: f64,
}

/// News sentiment index point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsSentimentPoint {
    pub date: String,
    pub sentiment_index: f64,
    pub csi300: f64,
}

// ---------------------------------------------------------------------------
// Fund-specific types
// ---------------------------------------------------------------------------

/// Fund NAV history info point (from eastmoney lsjz API).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundNavHistory {
    pub date: String,
    pub nav: f64,
    pub acc_nav: f64,
    pub change_pct: f64,
    pub subscribe_status: String,
    pub redeem_status: String,
}

/// Fund position estimate point (from Legu).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundPositionPoint {
    pub date: String,
    pub close: f64,
    pub position: f64,
}

/// Fund announcement item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundAnnouncementItem {
    pub fund_code: String,
    pub title: String,
    pub fund_name: String,
    pub date: String,
    pub report_id: String,
}

/// Fund portfolio change item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundPortfolioChangeItem {
    pub rank: i32,
    pub stock_code: String,
    pub stock_name: String,
    pub amount: f64,
    pub ratio: f64,
    pub quarter: String,
}

/// Fund industry allocation item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundIndustryAllocItem {
    pub rank: i32,
    pub industry: String,
    pub ratio: f64,
    pub market_value: f64,
    pub date: String,
}

/// Fund rating item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundRatingItem {
    pub code: String,
    pub name: String,
    pub fund_manager: String,
    pub company: String,
    pub rating_count_5star: i32,
    pub rating_sh: i32,
    pub rating_zs: i32,
    pub rating_ja: i32,
    pub rating_morningstar: i32,
    pub fee: f64,
    pub fund_type: String,
}

/// Fund manager item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundManagerItem {
    pub rank: i32,
    pub name: String,
    pub company: String,
    pub fund_code: String,
    pub fund_name: String,
    pub career_days: i32,
    pub total_scale: f64,
    pub best_return: f64,
}

/// Fund new found item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundNewFoundItem {
    pub fund_code: String,
    pub fund_name: String,
    pub company: String,
    pub fund_type: String,
    pub subscribe_period: String,
    pub shares: f64,
    pub found_date: String,
    pub return_since_found: f64,
    pub manager: String,
    pub status: String,
    pub discount_fee: f64,
}

/// Fund scale change item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundScaleChangeItem {
    pub rank: i32,
    pub report_date: String,
    pub fund_count: i32,
    pub subscribe: f64,
    pub redeem: f64,
    pub end_shares: f64,
    pub end_net_assets: f64,
}

/// Fund holder structure item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundHolderStructureItem {
    pub rank: i32,
    pub report_date: String,
    pub fund_count: i32,
    pub institution_ratio: f64,
    pub individual_ratio: f64,
    pub internal_ratio: f64,
    pub total_shares: f64,
}

/// Fund dividend item (from fhsp_em).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundDividendItem {
    pub rank: i32,
    pub fund_code: String,
    pub fund_name: String,
    pub record_date: String,
    pub ex_date: String,
    pub dividend: f64,
    pub pay_date: String,
}

/// Fund dividend rank item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundDividendRankItem {
    pub rank: i32,
    pub fund_code: String,
    pub fund_name: String,
    pub cumulative_dividend: f64,
    pub cumulative_count: i32,
    pub found_date: String,
}

/// Fund split item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundSplitItem {
    pub rank: i32,
    pub fund_code: String,
    pub fund_name: String,
    pub split_date: String,
    pub split_type: String,
    pub split_ratio: f64,
}

/// ETF spot item from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtfSpotItem {
    pub code: String,
    pub name: String,
    pub latest_price: f64,
    pub change_pct: f64,
    pub change_amount: f64,
    pub volume: f64,
    pub amount: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub prev_close: f64,
    pub amplitude: f64,
    pub turnover_rate: f64,
    pub iopv: f64,
    pub discount_rate: f64,
    pub shares: f64,
    pub circ_mv: f64,
    pub total_mv: f64,
    pub data_date: String,
}

/// ETF scale item from SSE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtfScaleItem {
    pub rank: i32,
    pub fund_code: String,
    pub fund_name: String,
    pub etf_type: String,
    pub stat_date: String,
    pub shares: f64,
}

/// Fund AUM trend point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundAumTrendPoint {
    pub date: String,
    pub value: f64,
}

/// Fund AUM history item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundAumHistItem {
    pub rank: i32,
    pub company: String,
    pub total: f64,
    pub equity: f64,
    pub mixed: f64,
    pub bond: f64,
    pub index: f64,
    pub qdii: f64,
    pub money: f64,
}

/// Fund AUM company item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundAumCompanyItem {
    pub rank: i32,
    pub company: String,
    pub found_date: String,
    pub total_scale: f64,
    pub fund_count: i32,
    pub manager_count: i32,
    pub update_date: String,
}

/// Fund exchange rank item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundExchangeRankItem {
    pub rank: i32,
    pub fund_code: String,
    pub fund_name: String,
    pub fund_type: String,
    pub date: String,
    pub nav: f64,
    pub acc_nav: f64,
    pub week_1: f64,
    pub month_1: f64,
    pub month_3: f64,
    pub month_6: f64,
    pub year_1: f64,
    pub year_2: f64,
    pub year_3: f64,
    pub ytd: f64,
    pub since_found: f64,
    pub found_date: String,
}

/// Fund money rank item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundMoneyRankItem {
    pub rank: i32,
    pub fund_code: String,
    pub fund_name: String,
    pub date: String,
    pub yield_per_10k: f64,
    pub annualized_7d: f64,
    pub annualized_14d: f64,
    pub annualized_28d: f64,
    pub month_1: f64,
    pub month_3: f64,
    pub month_6: f64,
    pub year_1: f64,
    pub year_2: f64,
    pub year_3: f64,
    pub year_5: f64,
    pub ytd: f64,
    pub since_found: f64,
}

/// Fund HK rank item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundHkRankItem {
    pub rank: i32,
    pub fund_code: String,
    pub fund_name: String,
    pub currency: String,
    pub date: String,
    pub nav: f64,
    pub change_pct: f64,
    pub week_1: f64,
    pub month_1: f64,
    pub month_3: f64,
    pub month_6: f64,
    pub year_1: f64,
    pub year_2: f64,
    pub year_3: f64,
    pub ytd: f64,
    pub since_found: f64,
    pub can_buy: String,
    pub hk_fund_code: String,
}

/// Xueqiu fund basic info item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XqBasicInfo {
    pub item: String,
    pub value: String,
}

/// Xueqiu fund achievement item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XqAchievementItem {
    pub achievement_type: String,
    pub period: String,
    pub return_pct: f64,
    pub max_drawdown: f64,
    pub rank: String,
}

/// Xueqiu fund analysis item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XqAnalysisItem {
    pub period: String,
    pub risk_return_ratio: f64,
    pub risk_control: f64,
    pub volatility: f64,
    pub sharpe: f64,
    pub max_drawdown: f64,
}

/// Xueqiu fund profit probability item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XqProfitProbabilityItem {
    pub holding_period: String,
    pub profit_ratio: f64,
    pub avg_return: f64,
}

/// Xueqiu fund detail hold item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XqDetailHoldItem {
    pub asset_type: String,
    pub percent: f64,
}

/// Fund value estimation item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundValueEstimationItem {
    pub rank: i32,
    pub fund_code: String,
    pub fund_name: String,
    pub estimated_value: f64,
    pub estimated_change_pct: f64,
    pub published_nav: f64,
    pub published_change_pct: f64,
    pub deviation: f64,
    pub nav_date: String,
}
