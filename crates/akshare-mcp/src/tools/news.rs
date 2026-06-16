use rmcp::schemars;

// ── Existing param types ──────────────────────────────────────────────

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

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct StockNewsSearchParams {
    pub symbol: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

const fn default_limit() -> usize {
    20
}

// ── NEW param types needed ──────────────────────────────────────────────────

/// Parameters for news search with a specific scope.
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct NewsSearchWithScopeParams {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    /// Search scope: "default" for A-share focused, "global" for broader HK/US coverage.
    #[serde(default = "default_scope")]
    pub scope: String,
}

/// Parameters for simple query + timeout news endpoints (Baidu, Sogou, Google, Bing).
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct NewsQueryTimeoutParams {
    pub query: String,
    /// Timeout in seconds for the upstream request.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

/// Parameters for GDELT global news search.
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct GdeltNewsSearchParams {
    pub query: String,
    /// GDELT API endpoint URL.
    /// Default: "https://api.gdeltproject.org/api/v2/doc/doc"
    #[serde(default = "default_gdelt_base_url")]
    pub base_url: String,
    /// Optional language hint: "zh-CN" for Chinese, "en-US" for English.
    #[serde(default)]
    pub language_hint: Option<String>,
    /// Optional time range: "day", "week", "month", or raw timespan.
    /// Default: "month"
    #[serde(default = "default_time_range")]
    pub time_range: String,
    /// Timeout in seconds for the upstream request.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

/// Parameters for Bing RSS news search with optional language.
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BingNewsRssParams {
    pub query: String,
    /// Timeout in seconds for the upstream request.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    /// Optional language: "en" for English (www.bing.com), omit for Chinese (cn.bing.com).
    #[serde(default)]
    pub lang: Option<String>,
}

// ── Finnhub / Marketaux param types ─────────────────────────────────────────

/// Parameters for Finnhub company news search.
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FinnhubNewsParams {
    /// Stock ticker symbol, e.g. "AAPL"
    pub symbol: String,
    /// Start date in YYYY-MM-DD format
    pub from: String,
    /// End date in YYYY-MM-DD format
    pub to: String,
    /// Finnhub API key (free tier: 60 requests/minute)
    #[serde(default)]
    pub api_key: String,
}

/// Parameters for Marketaux global/HK news search.
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct MarketauxNewsParams {
    /// Stock ticker symbol, e.g. "AAPL" or "0700.HK"
    pub symbol: String,
    /// Marketaux API key (free tier: 100 requests/day)
    #[serde(default)]
    pub api_key: String,
    /// Max number of articles (API max: 3 on free tier)
    #[serde(default = "default_marketaux_limit")]
    pub limit: usize,
}

fn default_marketaux_limit() -> usize {
    3
}

// ── Default value functions ─────────────────────────────────────────────────
// NOTE: default_limit() already exists in news.rs (value: 20). Do not re-define.

fn default_scope() -> String {
    "default".to_string()
}

const fn default_timeout() -> u64 {
    30
}

fn default_gdelt_base_url() -> String {
    "https://api.gdeltproject.org/api/v2/doc/doc".to_string()
}

fn default_time_range() -> String {
    "month".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_search_with_scope_params_defaults() {
        let json = r#"{"query": "stock"}"#;
        let params: NewsSearchWithScopeParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.query, "stock");
        assert_eq!(params.limit, 20);
        assert_eq!(params.scope, "default");
    }

    #[test]
    fn test_news_search_with_scope_params_custom() {
        let json = r#"{"query": "bond", "limit": 50, "scope": "global"}"#;
        let params: NewsSearchWithScopeParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 50);
        assert_eq!(params.scope, "global");
    }

    #[test]
    fn test_news_query_timeout_params_defaults() {
        let json = r#"{"query": "finance"}"#;
        let params: NewsQueryTimeoutParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.query, "finance");
        assert_eq!(params.timeout_secs, 30);
    }

    #[test]
    fn test_gdelt_news_search_params_defaults() {
        let json = r#"{"query": "economy"}"#;
        let params: GdeltNewsSearchParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.query, "economy");
        assert_eq!(
            params.base_url,
            "https://api.gdeltproject.org/api/v2/doc/doc"
        );
        assert_eq!(params.language_hint, None);
        assert_eq!(params.time_range, "month");
        assert_eq!(params.timeout_secs, 30);
    }

    #[test]
    fn test_bing_news_rss_params_defaults() {
        let json = r#"{"query": "market"}"#;
        let params: BingNewsRssParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.query, "market");
        assert_eq!(params.timeout_secs, 30);
        assert_eq!(params.lang, None);
    }
}
