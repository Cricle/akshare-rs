use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct EmptyParams {}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SymbolParams {
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CandlesParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct StockHistParams {
    pub symbol: String,
    #[serde(default = "default_period")]
    pub period: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
    #[serde(default)]
    pub adjust: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FundFlowParams {
    pub symbol: String,
    /// Market code: "sh", "sz", "bj", "hk", "us". Auto-detected if omitted.
    #[serde(default)]
    pub market: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FundFlowRankParams {
    /// Indicator: "today", "3day", "5day", "10day"
    #[serde(default = "default_indicator")]
    pub indicator: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct StockDailyParams {
    pub symbol: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct ValuationParams {
    pub symbol: String,
    #[serde(default = "default_valuation_indicator")]
    pub indicator: String,
    #[serde(default = "default_valuation_period")]
    pub period: String,
}

const fn default_limit() -> usize {
    60
}

fn default_period() -> String {
    "daily".to_string()
}

fn default_indicator() -> String {
    "today".to_string()
}

fn default_valuation_indicator() -> String {
    "总市值".to_string()
}

fn default_valuation_period() -> String {
    "近一年".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct DateParams {
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SymbolDateParams {
    pub symbol: String,
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct DateRangeParams {
    pub start_date: String,
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SymbolDateRangeParams {
    pub symbol: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SymbolIndicatorParams {
    pub symbol: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct LimitParams {
    #[serde(default = "default_limit")]
    pub limit: usize,
}

// ── Additional param types for bulk tool wrappers ──────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SearchParams {
    pub query: String,
    #[serde(default)]
    pub market: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SectorRankParams {
    pub sector_type: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SectorCodeParams {
    pub sector_code: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BillboardSeatsParams {
    pub symbol: String,
    pub side: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct TradeCalendarParams {
    pub exchange: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BoardCodeParams {
    pub board_code: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BoardHistMinParams {
    pub symbol: String,
    #[serde(default = "default_board_hist_min_period")]
    pub period: String,
}

fn default_board_hist_min_period() -> String {
    "5".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SymbolStartYearParams {
    pub symbol: String,
    #[serde(default = "default_start_year")]
    pub start_year: String,
}

fn default_start_year() -> String {
    "2020".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct HkReportParams {
    pub stock: String,
    pub symbol: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct StockSymbolParams {
    pub stock: String,
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct GdfxDetailParams {
    pub date: String,
    pub indicator: String,
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct DzjyMrmxParams {
    pub asset_type: String,
    pub start_date: String,
    pub end_date: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct XqInfoParams {
    pub symbol: String,
    pub token: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CyqParams {
    pub symbol: String,
    #[serde(default)]
    pub adjust: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct HistoryDividendDetailParams {
    pub symbol: String,
    pub indicator: String,
    #[serde(default)]
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct NoticeReportParams {
    pub security: String,
    pub report_type: String,
    #[serde(default)]
    pub begin_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct DisclosureParams {
    pub symbol: String,
    pub category: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SymbolYearParams {
    pub symbol: String,
    #[serde(default = "default_year")]
    pub year: String,
}

fn default_year() -> String {
    "2024".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct DateIndicatorParams {
    pub date: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct DateMarketParams {
    pub date: String,
    #[serde(default)]
    pub market: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct DateRangeLimitParams {
    pub start_date: String,
    pub end_date: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SymbolSectorParams {
    pub symbol: String,
    #[serde(default)]
    pub sector: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct KeywordDateParams {
    #[serde(default)]
    pub keyword: String,
    #[serde(default)]
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_params_deserialize() {
        let json = r#"{"symbol": "600000"}"#;
        let params: SymbolParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "600000");
    }

    #[test]
    fn test_candles_params_defaults() {
        let json = r#"{"symbol": "600000"}"#;
        let params: CandlesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "600000");
        assert_eq!(params.limit, 60);
    }

    #[test]
    fn test_candles_params_custom_limit() {
        let json = r#"{"symbol": "600000", "limit": 30}"#;
        let params: CandlesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 30);
    }

    #[test]
    fn test_stock_hist_params_defaults() {
        let json = r#"{"symbol": "600000"}"#;
        let params: StockHistParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "600000");
        assert_eq!(params.period, "daily");
        assert_eq!(params.start_date, "");
        assert_eq!(params.end_date, "");
        assert_eq!(params.adjust, "");
    }

    #[test]
    fn test_stock_hist_params_full() {
        let json = r#"{"symbol": "600000", "period": "weekly", "start_date": "20240101", "end_date": "20240601", "adjust": "qfq"}"#;
        let params: StockHistParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.period, "weekly");
        assert_eq!(params.adjust, "qfq");
    }

    #[test]
    fn test_fund_flow_params() {
        let json = r#"{"symbol": "600000", "market": "sh"}"#;
        let params: FundFlowParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "600000");
        assert_eq!(params.market, "sh");
        assert_eq!(params.limit, 60);
    }

    #[test]
    fn test_default_limit_value() {
        assert_eq!(default_limit(), 60);
    }

    #[test]
    fn test_default_period_value() {
        assert_eq!(default_period(), "daily");
    }
}
