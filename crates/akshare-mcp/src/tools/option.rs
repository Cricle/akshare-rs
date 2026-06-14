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

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionChainParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionSymbolDateParams {
    pub symbol: String,
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionSymbolMonthParams {
    pub symbol: String,
    pub end_month: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionSymbolYearParams {
    pub symbol: String,
    pub year: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionSymbolContractParams {
    pub symbol: String,
    pub contract: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionSymbolExchangeParams {
    pub symbol: String,
    pub exchange: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionLhbParams {
    pub symbol: String,
    pub indicator: String,
    pub trade_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionSseCodesParams {
    pub symbol: String,
    pub trade_date: String,
    pub underlying: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct OptionSseExpireDayParams {
    pub trade_date: String,
    pub symbol: String,
    pub exchange: String,
}

const fn default_limit() -> usize {
    200
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_chain_params() {
        let json = r#"{"symbol": "510050"}"#;
        let params: OptionChainParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "510050");
        assert_eq!(params.limit, 200);
    }

    #[test]
    fn test_option_chain_params_with_limit() {
        let json = r#"{"symbol": "510050", "limit": 50}"#;
        let params: OptionChainParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 50);
    }

    #[test]
    fn test_option_symbol_date_params() {
        let json = r#"{"symbol": "玉米期权", "date": "20240115"}"#;
        let params: OptionSymbolDateParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "玉米期权");
        assert_eq!(params.date, "20240115");
    }

    #[test]
    fn test_option_symbol_month_params() {
        let json = r#"{"symbol": "华夏上证50ETF期权", "end_month": "2306"}"#;
        let params: OptionSymbolMonthParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.end_month, "2306");
    }

    #[test]
    fn test_option_symbol_year_params() {
        let json = r#"{"symbol": "SR", "year": "2021"}"#;
        let params: OptionSymbolYearParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.year, "2021");
    }

    #[test]
    fn test_option_symbol_contract_params() {
        let json = r#"{"symbol": "豆粕期权", "contract": "m2306"}"#;
        let params: OptionSymbolContractParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.contract, "m2306");
    }

    #[test]
    fn test_option_symbol_exchange_params() {
        let json = r#"{"symbol": "50ETF", "exchange": "SSE"}"#;
        let params: OptionSymbolExchangeParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.exchange, "SSE");
    }

    #[test]
    fn test_option_lhb_params() {
        let json = r#"{"symbol": "510050", "indicator": "期权交易情况-认沽交易量", "trade_date": "20240115"}"#;
        let params: OptionLhbParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "510050");
    }

    #[test]
    fn test_option_sse_codes_params() {
        let json = r#"{"symbol": "看涨期权", "trade_date": "202202", "underlying": "510050"}"#;
        let params: OptionSseCodesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.underlying, "510050");
    }

    #[test]
    fn test_option_sse_expire_day_params() {
        let json = r#"{"trade_date": "202401", "symbol": "50ETF", "exchange": "SSE"}"#;
        let params: OptionSseExpireDayParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "50ETF");
    }
}
