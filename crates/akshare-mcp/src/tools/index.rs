use rmcp::schemars;

// ── Existing param types ──────────────────────────────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for index hk daily queries.
pub struct IndexHkDailyParams {
    pub symbol: String,
    /// Eastmoney internal market id (e.g. "100", "124"). Get from index_hk_spot_em.
    pub internal_id: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for index symbol queries.
pub struct IndexSymbolParams {
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for index candles queries.
pub struct IndexCandlesParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for index hist queries.
pub struct IndexHistParams {
    pub symbol: String,
    pub start_date: String,
    pub end_date: String,
}

const fn default_limit() -> usize {
    60
}

// ── New param types ──────────────────────────────────────────────────

/// Params for methods taking symbol + period (e.g. "daily"/"weekly"/"monthly").
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct IndexPeriodParams {
    pub symbol: String,
    /// Period: "daily", "weekly", or "monthly".
    pub period: String,
}

/// Params for Eastmoney A-share index intraday with full arguments.
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct IndexZhAHistMinEmParams {
    pub symbol: String,
    /// Period: "1", "5", "15", "30", or "60".
    pub period: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
    /// Adjust: "qfq", "hfq", or "".
    #[serde(default)]
    pub adjust: String,
}

/// Params for Shenwan research analysis reports (date is a single date).
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SwAnalysisDateParams {
    /// Index type: "市场表征", "一级行业", "二级行业", "风格指数".
    pub symbol: String,
    /// Report date in YYYYMMDD or YYYY-MM-DD format.
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_period_params() {
        let json = r#"{"symbol": "000300", "period": "daily"}"#;
        let params: IndexPeriodParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "000300");
        assert_eq!(params.period, "daily");
    }

    #[test]
    fn test_index_zh_a_hist_min_em_params() {
        let json = r#"{"symbol": "000300", "period": "5"}"#;
        let params: IndexZhAHistMinEmParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "000300");
        assert_eq!(params.period, "5");
        assert_eq!(params.start_date, "");
        assert_eq!(params.adjust, "");
    }

    #[test]
    fn test_sw_analysis_date_params() {
        let json = r#"{"symbol": "一级行业", "date": "20241025"}"#;
        let params: SwAnalysisDateParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "一级行业");
        assert_eq!(params.date, "20241025");
    }
}
