use rmcp::schemars;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_candles_params() {
        let json = r#"{"symbol": "AU0"}"#;
        let params: FuturesCandlesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "AU0");
        assert_eq!(params.limit, 60);
    }

    #[test]
    fn test_futures_limit_params_default() {
        let json = r"{}";
        let params: FuturesLimitParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 60);
    }

    #[test]
    fn test_date_params() {
        let json = r#"{"date": "20240101"}"#;
        let params: DateParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.date, "20240101");
    }
}
