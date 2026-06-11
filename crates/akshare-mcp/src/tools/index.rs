use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct IndexSymbolParams {
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct IndexCandlesParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct IndexHistParams {
    pub symbol: String,
    pub start_date: String,
    pub end_date: String,
}

fn default_limit() -> usize {
    60
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_symbol_params() {
        let json = r#"{"symbol": "000300"}"#;
        let params: IndexSymbolParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "000300");
    }

    #[test]
    fn test_index_candles_params() {
        let json = r#"{"symbol": "000300"}"#;
        let params: IndexCandlesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 60);
    }

    #[test]
    fn test_index_hist_params() {
        let json = r#"{"symbol": "000300", "start_date": "20240101", "end_date": "20240601"}"#;
        let params: IndexHistParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "000300");
        assert_eq!(params.start_date, "20240101");
        assert_eq!(params.end_date, "20240601");
    }
}
