use rmcp::schemars;

// ── Existing param types ──────────────────────────────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesCandlesParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesLimitParams {
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct DateParams {
    pub date: String,
}

const fn default_limit() -> usize {
    60
}

// ── Futures-specific param types ────────────────────────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesDateRangeParams {
    pub start_date: String,
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesDateSymbolParams {
    pub date: String,
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesDateMarketParams {
    pub date: String,
    pub market: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesSymbolPeriodParams {
    pub symbol: String,
    #[serde(default = "default_period")]
    pub period: String,
}

fn default_period() -> String {
    "daily".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
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
pub struct FuturesSymbolMarketParams {
    pub symbols: String,
    pub market: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesSymbolPeriodKlineParams {
    pub symbol: String,
    pub period: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesMainSinaDerivativeParams {
    pub symbol: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesSymbolIndicatorParams {
    pub symbol: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesCategoryParams {
    pub category: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FuturesHoldPosParams {
    /// Data type: "成交量", "多单持仓", or "空单持仓"
    pub data_type: String,
    pub contract: String,
    pub date: String,
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
