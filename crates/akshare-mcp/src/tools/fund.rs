use rmcp::schemars;

// ── Existing param types ──────────────────────────────────────────────

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

// ── New param types for fund MCP tools ──────────────────────────────────

/// Params for fund_info_index_em: index category, fund type, limit
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FundInfoIndexParams {
    /// Index category: "全部", "沪深指数", "行业主题", "大盘指数", "中盘指数", "小盘指数"
    #[serde(default = "default_info_index_symbol")]
    pub symbol: String,
    /// Fund type: "全部", "被动指数型", "增强指数型"
    #[serde(default = "default_info_index_indicator")]
    pub indicator: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_info_index_symbol() -> String {
    "全部".to_string()
}

fn default_info_index_indicator() -> String {
    "全部".to_string()
}

/// Params for fund_etf_hist_em, fund_etf_hist_min_em, fund_lof_hist_em, fund_lof_hist_min_em
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FundEtfHistFullParams {
    /// 6-digit fund code (e.g. "510300")
    pub symbol: String,
    /// Period: "daily", "weekly", "monthly" (or "1","5","15","30","60" for minute)
    #[serde(default = "default_period")]
    pub period: String,
    /// Start date, format "YYYYMMDD"
    #[serde(default)]
    pub start_date: String,
    /// End date, format "YYYYMMDD"
    #[serde(default)]
    pub end_date: String,
    /// Adjust type: "qfq", "hfq", or ""
    #[serde(default = "default_adjust")]
    pub adjust: String,
}

fn default_period() -> String {
    "daily".to_string()
}

fn default_adjust() -> String {
    "qfq".to_string()
}

/// Params for fund_open_fund_info_em
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FundOpenFundInfoParams {
    /// Fund code (e.g. "710001")
    pub symbol: String,
    /// Start date (YYYYMMDD), empty for all
    #[serde(default)]
    pub start_date: String,
    /// End date (YYYYMMDD), empty for all
    #[serde(default)]
    pub end_date: String,
    /// Indicator: "单位净值走势", "累计净值走势", etc.
    #[serde(default = "default_open_indicator")]
    pub indicator: String,
}

fn default_open_indicator() -> String {
    "单位净值走势".to_string()
}

/// Params for QDII fund methods (require cookie)
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FundQdiiCookieParams {
    /// Jisilu cookie for authentication (empty for unauthenticated)
    #[serde(default)]
    pub cookie: String,
}

/// Params for fund_etf_category_ths (symbol + date)
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FundEtfCategoryThsParams {
    /// Category: "ETF", "LOF", "股票型", "债券型", "混合型", "QDII", "保本型", "指数型", or "" for all
    #[serde(default = "default_etf_category")]
    pub symbol: String,
    /// Date format "YYYYMMDD" or "" for latest
    #[serde(default)]
    pub date: String,
}

fn default_etf_category() -> String {
    "ETF".to_string()
}

const fn default_limit() -> usize {
    60
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fund_info_index_params_default() {
        let json = r#"{}"#;
        let params: FundInfoIndexParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "全部");
        assert_eq!(params.indicator, "全部");
        assert_eq!(params.limit, 60);
    }

    #[test]
    fn test_fund_etf_hist_full_params() {
        let json = r#"{"symbol": "510300"}"#;
        let params: FundEtfHistFullParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "510300");
        assert_eq!(params.period, "daily");
        assert_eq!(params.adjust, "qfq");
    }

    #[test]
    fn test_fund_open_fund_info_params() {
        let json = r#"{"symbol": "710001"}"#;
        let params: FundOpenFundInfoParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "710001");
        assert_eq!(params.indicator, "单位净值走势");
    }

    #[test]
    fn test_fund_qdii_cookie_params_default() {
        let json = r#"{}"#;
        let params: FundQdiiCookieParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.cookie, "");
    }
}
