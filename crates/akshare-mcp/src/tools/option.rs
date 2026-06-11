use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionSymbolParams {
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionHistParams {
    pub symbol: String,
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_symbol_params() {
        let json = r#"{"symbol": "10002431"}"#;
        let params: OptionSymbolParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "10002431");
    }

    #[test]
    fn test_option_hist_params() {
        let json = r#"{"symbol": "cu2401", "date": "20240115"}"#;
        let params: OptionHistParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "cu2401");
        assert_eq!(params.date, "20240115");
    }
}
