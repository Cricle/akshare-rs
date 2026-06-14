use rmcp::schemars;

// ── Existing (kept as-is) ───────────────────────────────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CurrencyParams {
    pub symbol: String,
    pub start_date: String,
    pub end_date: String,
}

// ── New param types ────────────────────────────────────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct ForexHistEmParams {
    /// Eastmoney forex secid, e.g. "133.USDCNY" or "119.EURCNY"
    pub symbol: String,
    /// Kline period: "daily", "weekly", or "monthly"
    #[serde(default = "default_period")]
    pub period: String,
    /// Start date in YYYYMMDD format (currently unused by API but forwarded)
    #[serde(default)]
    pub start_date: String,
    /// End date in YYYYMMDD format (currently unused by API but forwarded)
    #[serde(default)]
    pub end_date: String,
    /// Price adjustment: "qfq" (forward), "hfq" (backward), or "" (none)
    #[serde(default)]
    pub adjust: String,
}

fn default_period() -> String {
    "daily".to_string()
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct ForexEmHistParams {
    /// Eastmoney forex secid, e.g. "133.USDCNY" or "119.EURCNY"
    pub symbol: String,
    /// Number of data points to return
    #[serde(default = "default_forex_limit")]
    pub limit: usize,
}

fn default_forex_limit() -> usize {
    60
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CurrencyLatestParams {
    /// Base currency code, e.g. "USD"
    pub base: String,
    /// Comma-separated target currency codes, e.g. "EUR,GBP,CNY"
    pub symbols: String,
    /// CurrencyBeacon API key
    #[serde(default)]
    pub api_key: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CurrencyHistoryParams {
    /// Base currency code, e.g. "USD"
    pub base: String,
    /// Date in YYYY-MM-DD format
    pub date: String,
    /// Comma-separated target currency codes
    pub symbols: String,
    /// CurrencyBeacon API key
    #[serde(default)]
    pub api_key: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CurrencyTimeSeriesParams {
    /// Base currency code, e.g. "USD"
    pub base: String,
    /// Start date in YYYY-MM-DD format
    pub start_date: String,
    /// End date in YYYY-MM-DD format
    pub end_date: String,
    /// Comma-separated target currency codes
    pub symbols: String,
    /// CurrencyBeacon API key
    #[serde(default)]
    pub api_key: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CurrencyCurrenciesParams {
    /// Currency type filter (e.g. "fiat")
    #[serde(default)]
    pub c_type: String,
    /// CurrencyBeacon API key
    #[serde(default)]
    pub api_key: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CurrencyConvertParams {
    /// Source currency code, e.g. "USD"
    pub from: String,
    /// Target currency code, e.g. "CNY"
    pub to: String,
    /// Amount to convert
    pub amount: f64,
    /// CurrencyBeacon API key
    #[serde(default)]
    pub api_key: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CurrencyPairParams {
    /// FX currency pair code, e.g. "USDCNY"
    pub pair: String,
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

    #[test]
    fn test_forex_hist_em_params_defaults() {
        let json = r#"{"symbol": "133.USDCNY"}"#;
        let params: ForexHistEmParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "133.USDCNY");
        assert_eq!(params.period, "daily");
        assert_eq!(params.start_date, "");
        assert_eq!(params.end_date, "");
        assert_eq!(params.adjust, "");
    }

    #[test]
    fn test_forex_em_hist_params_defaults() {
        let json = r#"{"symbol": "133.USDCNY"}"#;
        let params: ForexEmHistParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "133.USDCNY");
        assert_eq!(params.limit, 60);
    }

    #[test]
    fn test_currency_convert_params() {
        let json = r#"{"from": "USD", "to": "CNY", "amount": 100.0, "api_key": "test"}"#;
        let params: CurrencyConvertParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.from, "USD");
        assert_eq!(params.to, "CNY");
        assert!((params.amount - 100.0).abs() < f64::EPSILON);
        assert_eq!(params.api_key, "test");
    }
}
