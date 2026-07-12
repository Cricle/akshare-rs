use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for empty queries.
pub struct EmptyParams {}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol queries.
pub struct SymbolParams {
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for candles queries.
pub struct CandlesParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for stock hist queries.
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
/// MCP tool parameters for fund flow queries.
pub struct FundFlowParams {
    pub symbol: String,
    /// Market code: "sh", "sz", "bj", "hk", "us". Auto-detected if omitted.
    #[serde(default)]
    pub market: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for fund flow rank queries.
pub struct FundFlowRankParams {
    /// Indicator: "today", "3day", "5day", "10day"
    #[serde(default = "default_indicator")]
    pub indicator: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for stock daily queries.
pub struct StockDailyParams {
    pub symbol: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for valuation queries.
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
/// MCP tool parameters for date queries.
pub struct DateParams {
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol date queries.
pub struct SymbolDateParams {
    pub symbol: String,
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for date range queries.
pub struct DateRangeParams {
    pub start_date: String,
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol date range queries.
pub struct SymbolDateRangeParams {
    pub symbol: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol indicator queries.
pub struct SymbolIndicatorParams {
    pub symbol: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for limit queries.
pub struct LimitParams {
    #[serde(default = "default_limit")]
    pub limit: usize,
}

// ── Additional param types for bulk tool wrappers ──────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for search queries.
pub struct SearchParams {
    pub query: String,
    #[serde(default)]
    pub market: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for sector rank queries.
pub struct SectorRankParams {
    pub sector_type: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for sector code queries.
pub struct SectorCodeParams {
    pub sector_code: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for billboard seats queries.
pub struct BillboardSeatsParams {
    pub symbol: String,
    pub side: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for trade calendar queries.
pub struct TradeCalendarParams {
    pub exchange: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for board code queries.
pub struct BoardCodeParams {
    pub board_code: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for board hist min queries.
pub struct BoardHistMinParams {
    pub symbol: String,
    #[serde(default = "default_board_hist_min_period")]
    pub period: String,
}

fn default_board_hist_min_period() -> String {
    "5".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol start year queries.
pub struct SymbolStartYearParams {
    pub symbol: String,
    #[serde(default = "default_start_year")]
    pub start_year: String,
}

fn default_start_year() -> String {
    "2020".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for hk report queries.
pub struct HkReportParams {
    pub stock: String,
    pub symbol: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for stock symbol queries.
pub struct StockSymbolParams {
    pub stock: String,
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for gdfx detail queries.
pub struct GdfxDetailParams {
    pub date: String,
    pub indicator: String,
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for dzjy mrmx queries.
pub struct DzjyMrmxParams {
    pub asset_type: String,
    pub start_date: String,
    pub end_date: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for xq info queries.
pub struct XqInfoParams {
    pub symbol: String,
    pub token: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for cyq queries.
pub struct CyqParams {
    pub symbol: String,
    #[serde(default)]
    pub adjust: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for history dividend detail queries.
pub struct HistoryDividendDetailParams {
    pub symbol: String,
    pub indicator: String,
    #[serde(default)]
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for notice report queries.
pub struct NoticeReportParams {
    pub security: String,
    pub report_type: String,
    #[serde(default)]
    pub begin_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for disclosure queries.
pub struct DisclosureParams {
    pub symbol: String,
    pub category: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol year queries.
pub struct SymbolYearParams {
    pub symbol: String,
    #[serde(default = "default_year")]
    pub year: String,
}

fn default_year() -> String {
    "2024".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for date indicator queries.
pub struct DateIndicatorParams {
    pub date: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for date market queries.
pub struct DateMarketParams {
    pub date: String,
    #[serde(default)]
    pub market: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for date range limit queries.
pub struct DateRangeLimitParams {
    pub start_date: String,
    pub end_date: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol sector queries.
pub struct SymbolSectorParams {
    pub symbol: String,
    #[serde(default)]
    pub sector: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for keyword date queries.
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
