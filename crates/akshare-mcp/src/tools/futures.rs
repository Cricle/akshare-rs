use rmcp::schemars;

// ── Existing param types ──────────────────────────────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures candles queries.
pub struct FuturesCandlesParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures limit queries.
pub struct FuturesLimitParams {
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for date queries.
pub struct DateParams {
    pub date: String,
}

const fn default_limit() -> usize {
    60
}

// ── Futures-specific param types ────────────────────────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures date range queries.
pub struct FuturesDateRangeParams {
    pub start_date: String,
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures date symbol queries.
pub struct FuturesDateSymbolParams {
    pub date: String,
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures date market queries.
pub struct FuturesDateMarketParams {
    pub date: String,
    pub market: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures symbol period queries.
pub struct FuturesSymbolPeriodParams {
    pub symbol: String,
    #[serde(default = "default_period")]
    pub period: String,
}

fn default_period() -> String {
    "daily".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures hist em queries.
pub struct FuturesHistEmParams {
    pub symbol: String,
    #[serde(default = "default_period")]
    pub period: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures symbol market queries.
pub struct FuturesSymbolMarketParams {
    pub symbols: String,
    pub market: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures symbol period kline queries.
pub struct FuturesSymbolPeriodKlineParams {
    pub symbol: String,
    pub period: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures main sina derivative queries.
pub struct FuturesMainSinaDerivativeParams {
    pub symbol: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures symbol indicator queries.
pub struct FuturesSymbolIndicatorParams {
    pub symbol: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures category queries.
pub struct FuturesCategoryParams {
    pub category: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures hold pos queries.
pub struct FuturesHoldPosParams {
    /// Data type: "成交量", "多单持仓", or "空单持仓"
    pub data_type: String,
    pub contract: String,
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for futures foreign commodity queries.
pub struct FuturesForeignCommodityParams {
    /// Comma-separated Sina codes (e.g., "CL,GC,XAU").
    pub symbols: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_date_range_params() {
        let json = r#"{"start_date": "20240101", "end_date": "20240131"}"#;
        let params: FuturesDateRangeParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.start_date, "20240101");
        assert_eq!(params.end_date, "20240131");
    }

    #[test]
    fn test_futures_symbol_period_default() {
        let json = r#"{"symbol": "AU0"}"#;
        let params: FuturesSymbolPeriodParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "AU0");
        assert_eq!(params.period, "daily");
    }

    #[test]
    fn test_futures_hist_em_params() {
        let json = r#"{"symbol": "rb2505"}"#;
        let params: FuturesHistEmParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "rb2505");
        assert_eq!(params.period, "daily");
        assert!(params.start_date.is_empty());
    }
}
