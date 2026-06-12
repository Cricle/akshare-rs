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
