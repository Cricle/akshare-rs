use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CurrencyParams {
    pub symbol: String,
    pub start_date: String,
    pub end_date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_params() {
        let json = r#"{"symbol": "美元", "start_date": "20240101", "end_date": "20240601"}"#;
        let params: CurrencyParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "美元");
        assert_eq!(params.start_date, "20240101");
        assert_eq!(params.end_date, "20240601");
    }
}
