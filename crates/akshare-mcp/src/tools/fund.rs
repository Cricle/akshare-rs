use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FundHkHistParams {
    /// HK fund code (from fund_hk_rank_em)
    pub code: String,
    /// Query type: "历史净值明细" or "分红送配详情"
    #[serde(default = "default_query_type")]
    pub query_type: String,
}

fn default_query_type() -> String {
    "历史净值明细".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FundHistParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FundRankParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

const fn default_limit() -> usize {
    60
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fund_hist_params() {
        let json = r#"{"symbol": "510300"}"#;
        let params: FundHistParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "510300");
        assert_eq!(params.limit, 60);
    }

    #[test]
    fn test_fund_hist_params_custom_limit() {
        let json = r#"{"symbol": "510300", "limit": 120}"#;
        let params: FundHistParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 120);
    }

    #[test]
    fn test_fund_rank_params() {
        let json = r#"{"symbol": "stock"}"#;
        let params: FundRankParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "stock");
        assert_eq!(params.limit, 60);
    }
}
