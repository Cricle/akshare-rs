use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondRateParams {
    pub start_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondLimitParams {
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondYieldParams {
    pub start_date: String,
    pub end_date: String,
}

const fn default_limit() -> usize {
    100
}

const fn default_page() -> usize {
    1
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondSymbolParams {
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondSymbolLimitParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondIndicatorPeriodParams {
    pub indicator: String,
    pub period: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondCloseReturnParams {
    pub symbol: String,
    pub period: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondSymbolIndicatorParams {
    pub symbol: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondSymbolPeriodParams {
    pub symbol: String,
    pub period: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondPageParams {
    #[serde(default = "default_page")]
    pub page: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BondDateParams {
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_rate_params() {
        let json = r#"{"start_date": "20200101"}"#;
        let params: BondRateParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.start_date, "20200101");
    }

    #[test]
    fn test_bond_limit_params() {
        let json = r"{}";
        let params: BondLimitParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 100);
    }

    #[test]
    fn test_bond_limit_params_custom() {
        let json = r#"{"limit": 50}"#;
        let params: BondLimitParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 50);
    }

    #[test]
    fn test_bond_yield_params() {
        let json = r#"{"start_date": "20200101", "end_date": "20240101"}"#;
        let params: BondYieldParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.start_date, "20200101");
        assert_eq!(params.end_date, "20240101");
    }
}
