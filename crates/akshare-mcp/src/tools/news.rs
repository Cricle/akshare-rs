use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct NewsDateParams {
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct NewsSearchParams {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct NewsSymbolParams {
    pub symbol: String,
}

fn default_limit() -> usize {
    20
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_date_params() {
        let json = r#"{"date": "20240101"}"#;
        let params: NewsDateParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.date, "20240101");
    }

    #[test]
    fn test_news_search_params() {
        let json = r#"{"query": "stock market"}"#;
        let params: NewsSearchParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.query, "stock market");
        assert_eq!(params.limit, 20);
    }

    #[test]
    fn test_news_search_params_custom_limit() {
        let json = r#"{"query": "bond", "limit": 50}"#;
        let params: NewsSearchParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 50);
    }

    #[test]
    fn test_news_symbol_params() {
        let json = r#"{"symbol": "finance"}"#;
        let params: NewsSymbolParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "finance");
    }
}
