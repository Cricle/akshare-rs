//! MarketDataClient — high-level client with caching and multi-provider news search.
//!
//! Wraps [`AkShareClient`](crate::AkShareClient) with optional Redis caching,
//! singleflight deduplication, and multi-provider news aggregation.

pub(crate) mod akshare_rust;
pub(crate) mod cache;
pub(crate) mod client;
pub(crate) mod diagnosis;
pub(crate) mod news_filter;
pub(crate) mod news_search;
pub(crate) mod search;
pub(crate) mod tushare;
pub(crate) mod wire;

pub(crate) mod a_share;
pub(crate) mod hk;
pub(crate) mod us;

pub mod tools;

use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::types::{
    AnnouncementDetail, AnnouncementItem, BatchFundamentalsResult, BatchQuoteResult,
    BillboardEntry, BillboardSeatDetail, CandlePoint, CandlesWithProvider, CapitalFlowPoint,
    CompanySearchContext, FundamentalsSnapshot, MarketKind, NewsFetchAttempt, NewsFetchResult,
    NewsItem, QuoteSnapshot, QuoteWithProvider, SectorConstituent, SectorSnapshot,
    StockSearchResult, TradeCalendarItem,
};

// Re-export types from stock modules used by fetch methods
pub use crate::stock::feature::{
    AnalystDetail, AnalystRank, BalanceSheet, BillboardActiveBranch, BillboardBranchDetail,
    BillboardBranchRanking, BillboardDetail, BillboardOrgSeatTracking, BillboardOrgTradeSummary,
    BillboardStockDetail, BillboardStockDetailDate, BillboardStockStatistic,
    BillboardTraderStatistic, BlockTradeActiveBranch, BlockTradeBranchRanking, CashFlowSheet,
    CommentDesireIndex, CommentFocusIndex, CommentHistScore, CommentOrgParticipation, DividendInfo,
    DzjyHygtj, DzjyMrtj, EarningsForecast, EarningsQuickReport, EarningsReport, EsgRating,
    FundFlowEntry, GdfxHoldingAnalyse, GdfxHoldingChange, GdfxHoldingDetail, GdfxHoldingStatistic,
    GdfxTeamwork, GdfxTop10, Gdhs, GdhsDetail, Ggcg, GpzyDistributeEntry, GpzyIndustry,
    GpzyPledgeDetail, GpzyPledgeRatio, GpzyPledgeRatioDetail, GpzyProfile, HotStockXq,
    IndustryCategory, JgdyDetail, JgdyTj, MainFundFlow, MarginAccountInfo, MarginRatioPa,
    MarginSseDetail, MarginSseSummary, MarginSzseDetail, MarginSzseSummary, PankouChange,
    ProfitSheet, SectorFundFlowRank, StockComment, ZtPool, ZtPoolDtgc, ZtPoolPrevious,
    ZtPoolStrong, ZtPoolSubNew, ZtPoolZbgc,
};

pub use crate::stock::hk_extra::{
    HkFamousStock, HkFhpxDetailThs, HkGxlLg, HkHotRank, HkHotRankDetail, HkSpotQuote,
    HkValuationBaidu,
};

pub use crate::stock::us_extra::{UsFamousStock, UsPinkStock, UsSpotSina, UsValuationBaidu};

pub use crate::stock::xueqiu::XqStockSpot;

// Re-export news filter utilities
pub use news_filter::normalized_news_date;

// Re-export diagnosis types
pub use diagnosis::DataFetchDiagnosis;

// Re-export singleflight types
pub use cache::{Singleflight, SingleflightGuard, SingleflightResult};

// cache, diagnosis, and qdrant stay in stock-analyzer (not data fetching concerns)

/// Configuration for constructing a `MarketDataClient`.
pub struct DataConfig {
    /// When set, all HTTP requests through the inner AkShareClient are redirected to this base URL (for testing).
    pub mock_uri: Option<String>,
    pub tushare_token: Option<String>,
    pub search_providers: Vec<SearchProviderConfig>,
}

/// High-level market data client with caching and multi-provider news search.
///
/// Wraps [`AkShareClient`](crate::AkShareClient) with singleflight deduplication
/// and multi-provider news aggregation. Use [`MarketDataClient::new`] to construct.
#[derive(Clone)]
pub struct MarketDataClient {
    http: reqwest_middleware::ClientWithMiddleware,
    tushare_token: Option<String>,
    search_providers: Vec<SearchProviderConfig>,
    ak: crate::AkShareClient,
    pub(crate) singleflight: cache::Singleflight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SearchProviderKind {
    Searxng,
    Gdelt,
    Baidu,
    Uapis,
}

/// Configuration for a news/web search provider.
#[derive(Debug, Clone)]
pub struct SearchProviderConfig {
    kind: SearchProviderKind,
    name: String,
    base_url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SearchScope {
    News,
    General,
}

/// Intent of a general web search (company-specific or macro evidence).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneralSearchIntent {
    CompanyEvidence,
    MacroEvidence,
}

/// Classification of data-fetching errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataErrorKind {
    UnsupportedMarket,
    PermissionDenied,
    Restricted,
    MissingCredentials,
    NotFound,
    Upstream,
}

impl DataErrorKind {
    /// Return this error kind as a static string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedMarket => "unsupported_market",
            Self::PermissionDenied => "permission_denied",
            Self::Restricted => "restricted",
            Self::MissingCredentials => "missing_credentials",
            Self::NotFound => "not_found",
            Self::Upstream => "upstream_error",
        }
    }
}

/// Error returned by [`MarketDataClient`] fetch methods.
#[derive(Debug)]
pub struct DataError {
    kind: DataErrorKind,
    message: String,
}

impl DataError {
    pub(crate) fn new(kind: DataErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for DataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for DataError {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct SearxngNewsQueryCacheEntry {
    #[serde(default)]
    items: Vec<NewsItem>,
    #[serde(default)]
    cached_error: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct SearxngNewsEvidenceCacheEntry {
    #[serde(default)]
    items: Vec<NewsItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct HkSecurityDirectoryEntry {
    pub symbol: String,
    pub name: String,
    pub market: String,
    pub exchange: String,
    pub category: String,
    pub sub_category: String,
    pub trading_currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UsSecurityDirectoryEntry {
    pub symbol: String,
    pub name: String,
    pub market: String,
    pub exchange: String,
}

impl SearchProviderConfig {
    /// Create a SearXNG search provider config.
    pub fn searxng(name: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            kind: SearchProviderKind::Searxng,
            name: name.into(),
            base_url: base_url.into(),
        }
    }

    /// Create a GDELT search provider config.
    pub fn gdelt(name: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            kind: SearchProviderKind::Gdelt,
            name: name.into(),
            base_url: base_url.into(),
        }
    }

    /// Create a Baidu search provider config.
    pub fn baidu(name: impl Into<String>) -> Self {
        Self {
            kind: SearchProviderKind::Baidu,
            name: name.into(),
            base_url: String::new(),
        }
    }

    /// Create a UAPI search provider config.
    pub fn uapis(name: impl Into<String>) -> Self {
        Self {
            kind: SearchProviderKind::Uapis,
            name: name.into(),
            base_url: String::new(),
        }
    }

    pub(crate) fn cache_scope(&self) -> String {
        format!(
            "{}:{}",
            self.name.trim().to_ascii_lowercase(),
            self.base_url.trim().trim_end_matches('/')
        )
    }

    pub(crate) fn display_name(&self) -> String {
        match self.kind {
            SearchProviderKind::Searxng => {
                if self.name.eq_ignore_ascii_case("searxng") {
                    "SearXNG News".to_string()
                } else {
                    format!("{} News", self.name)
                }
            }
            SearchProviderKind::Gdelt => {
                if self.name.eq_ignore_ascii_case("gdelt") {
                    "GDELT News".to_string()
                } else {
                    format!("{} News", self.name)
                }
            }
            SearchProviderKind::Baidu => "Baidu News".to_string(),
            SearchProviderKind::Uapis => "Uapis News".to_string(),
        }
    }

    pub(crate) fn supports_scope(&self, scope: SearchScope) -> bool {
        match self.kind {
            SearchProviderKind::Searxng => true,
            SearchProviderKind::Gdelt => scope == SearchScope::News,
            SearchProviderKind::Baidu => scope == SearchScope::News,
            SearchProviderKind::Uapis => true,
        }
    }

    pub(crate) fn query_budget(&self, scope: SearchScope) -> usize {
        match self.kind {
            SearchProviderKind::Searxng => usize::MAX,
            SearchProviderKind::Gdelt => match scope {
                SearchScope::News => 2,
                SearchScope::General => 0,
            },
            SearchProviderKind::Baidu => match scope {
                SearchScope::News => 3,
                SearchScope::General => 0,
            },
            SearchProviderKind::Uapis => usize::MAX,
        }
    }

    pub(crate) fn rewrite_query(&self, query: &str, language: &str) -> String {
        match self.kind {
            SearchProviderKind::Searxng => query.trim().to_string(),
            SearchProviderKind::Gdelt => rewrite_query_for_gdelt(query, language),
            SearchProviderKind::Baidu => query.trim().to_string(),
            SearchProviderKind::Uapis => query.trim().to_string(),
        }
    }

    pub(crate) fn cache_ttl_secs(&self) -> u64 {
        match self.kind {
            SearchProviderKind::Uapis => UAPIS_QUERY_CACHE_TTL_SECS,
            _ => SEARXNG_QUERY_CACHE_TTL_SECS,
        }
    }

    pub(crate) fn negative_cache_ttl_secs(&self) -> u64 {
        match self.kind {
            SearchProviderKind::Uapis => UAPIS_QUERY_NEGATIVE_CACHE_TTL_SECS,
            _ => SEARXNG_QUERY_NEGATIVE_CACHE_TTL_SECS,
        }
    }
}

impl SearchScope {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::News => "news",
            Self::General => "general",
        }
    }
}

pub(crate) const MARKET_DATA_CACHE_PREFIX: &str = "akshare:marketdata";
pub(crate) const QUOTE_CACHE_VERSION: &str = "v5";
pub(crate) const FUNDAMENTALS_CACHE_VERSION: &str = "v6";
pub(crate) const CANDLES_CACHE_VERSION: &str = "v5";
pub(crate) const NEWS_CACHE_VERSION: &str = "v5";
pub(crate) const GLOBAL_NEWS_CACHE_VERSION: &str = "v2";
pub(crate) const QUOTE_CACHE_TTL_SECS: u64 = 120;
pub(crate) const FUNDAMENTALS_CACHE_TTL_SECS: u64 = 6 * 60 * 60;
pub(crate) const NEWS_CACHE_TTL_SECS: u64 = 10 * 60;
pub(crate) const GLOBAL_NEWS_CACHE_TTL_SECS: u64 = 10 * 60;
pub(crate) const SEARXNG_QUERY_CACHE_TTL_SECS: u64 = 5 * 60;
pub(crate) const SEARXNG_QUERY_NEGATIVE_CACHE_TTL_SECS: u64 = 30;
pub(crate) const UAPIS_QUERY_CACHE_TTL_SECS: u64 = 15 * 60;
pub(crate) const UAPIS_QUERY_NEGATIVE_CACHE_TTL_SECS: u64 = 2 * 60;
pub(crate) const INSIDER_CACHE_TTL_SECS: u64 = 15 * 60;
pub(crate) const CANDLES_CACHE_TTL_SECS: u64 = 5 * 60;
pub(crate) const SEARCH_CACHE_TTL_SECS: u64 = 60 * 60;
pub(crate) const SEARCH_CACHE_VERSION: &str = "v4";
pub(crate) const HK_SECURITIES_LIST_CACHE_TTL_SECS: u64 = 24 * 60 * 60;
pub(crate) const GENERAL_SEARCH_FALLBACK_QUERY_LIMIT: usize = 6;
pub(crate) const NEWS_SEARCH_PROVIDER_TIMEOUT_SECS: u64 = 6;
pub(crate) const NEWS_SEARCH_EVIDENCE_QUERY_LIMIT_PER_PROVIDER: usize = 2;

pub(crate) fn news_result_cacheable(items: &[NewsItem], attempts: &[NewsFetchAttempt]) -> bool {
    !items.is_empty() && !attempts.is_empty() && attempts.iter().all(|attempt| attempt.success)
}

fn rewrite_query_for_gdelt(query: &str, language: &str) -> String {
    let mut tokens = query
        .split_whitespace()
        .filter(|token| {
            let normalized =
                token.trim_matches(|ch: char| ch == '"' || ch == '\'' || ch == '(' || ch == ')');
            !normalized.is_empty()
                && !normalized.eq_ignore_ascii_case("or")
                && !normalized.eq_ignore_ascii_case("and")
                && !normalized.eq_ignore_ascii_case("not")
                && !normalized.starts_with("site:")
        })
        .map(|token| {
            token.trim_matches(|ch: char| ch == '"' || ch == '\'' || ch == '(' || ch == ')')
        })
        .filter(|token| !token.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();

    if language.eq_ignore_ascii_case("en-US") && tokens.len() > 6 {
        tokens.truncate(6);
    }
    if language.eq_ignore_ascii_case("zh-CN") && tokens.len() > 8 {
        tokens.truncate(8);
    }
    tokens.join(" ")
}

// ---------------------------------------------------------------------------
// MarketDataClient — construction
// ---------------------------------------------------------------------------

impl MarketDataClient {
    pub async fn new() -> anyhow::Result<Self> {
        Self::from_config(&DataConfig {
            mock_uri: None,
            tushare_token: std::env::var("TUSHARE_TOKEN")
                .ok()
                .filter(|v| !v.is_empty()),
            search_providers: Self::load_search_providers(),
        })
        .await
    }

    /// Load search provider configurations from environment variables.
    pub fn load_search_providers() -> Vec<SearchProviderConfig> {
        let provider_names = std::env::var("SEARCH_PROVIDERS")
            .ok()
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .filter(|values| !values.is_empty())
            .unwrap_or_else(|| vec!["baidu".to_string(), "gdelt".to_string()]);

        let searxng_urls = std::env::var("SEARXNG_BASE_URLS")
            .or_else(|_| std::env::var("SEARCH_SEARXNG_URLS"))
            .ok()
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(|value| value.trim_end_matches('/').to_string())
                    .collect::<Vec<_>>()
            })
            .filter(|values| !values.is_empty())
            .unwrap_or_else(|| {
                vec![
                    std::env::var("SEARXNG_BASE_URL")
                        .ok()
                        .map(|value| value.trim().trim_end_matches('/').to_string())
                        .filter(|value| !value.is_empty())
                        .unwrap_or_else(|| "http://127.0.0.1:8080".to_string()),
                ]
            });
        let mut providers = Vec::new();
        for provider_name in provider_names {
            match provider_name.trim().to_ascii_lowercase().as_str() {
                "searxng" => {
                    for (index, base_url) in searxng_urls.iter().enumerate() {
                        let name = if searxng_urls.len() == 1 {
                            "searxng".to_string()
                        } else {
                            format!("searxng-{}", index + 1)
                        };
                        providers.push(SearchProviderConfig::searxng(name, base_url.clone()));
                    }
                }
                "gdelt" => {
                    providers.push(SearchProviderConfig::gdelt(
                        "gdelt",
                        "https://api.gdeltproject.org/api/v2/doc/doc",
                    ));
                }
                "baidu" => {
                    providers.push(SearchProviderConfig::baidu("baidu"));
                }
                "uapis" => {
                    providers.push(SearchProviderConfig::uapis("uapis"));
                }
                unsupported => {
                    tracing::warn!(
                        provider = unsupported,
                        "unsupported search provider configured; ignoring"
                    );
                }
            }
        }

        if providers.is_empty() {
            providers.push(SearchProviderConfig::searxng(
                "searxng",
                "http://127.0.0.1:8080",
            ));
        }
        providers
    }

    pub async fn from_config(config: &DataConfig) -> anyhow::Result<Self> {
        let mut http_builder = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .http1_only()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(8);
        let outbound_proxy_url = std::env::var("OUTBOUND_PROXY_URL")
            .ok()
            .or_else(|| std::env::var("HTTP_PROXY").ok())
            .or_else(|| std::env::var("http_proxy").ok())
            .or_else(|| std::env::var("HTTPS_PROXY").ok())
            .or_else(|| std::env::var("https_proxy").ok())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        if let Some(proxy_url) = outbound_proxy_url.as_deref()
            && let Ok(proxy) = reqwest::Proxy::all(proxy_url)
        {
            http_builder = http_builder.proxy(proxy);
        }
        let base_client = http_builder
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        let http = reqwest_middleware::ClientBuilder::new(base_client)
            .with(reqwest_tracing::TracingMiddleware::default())
            .build();

        let mut ak_builder = crate::AkShareClient::builder();
        if let Some(token) = &config.tushare_token {
            ak_builder = ak_builder.tushare_token(token);
        }
        if let Some(proxy_url) = outbound_proxy_url.as_deref() {
            ak_builder = ak_builder.proxy(proxy_url);
        }
        let mut ak = ak_builder.build();
        if let Some(ref mock_uri) = config.mock_uri {
            ak.mock_uri = Some(mock_uri.clone());
        }

        Ok(Self {
            http,
            tushare_token: config.tushare_token.clone(),
            search_providers: config.search_providers.clone(),
            ak,
            singleflight: cache::Singleflight::new(),
        })
    }

    /// Access the underlying [`AkShareClient`](crate::AkShareClient).
    pub fn ak(&self) -> &crate::AkShareClient {
        &self.ak
    }

    /// Access the HTTP client for custom requests.
    pub fn http(&self) -> &reqwest_middleware::ClientWithMiddleware {
        &self.http
    }

    /// Detect market from symbol.
    pub fn detect_market(&self, symbol: &str) -> MarketKind {
        crate::market::detect_market(symbol)
    }

    /// Data source label for quotes.
    pub fn quote_source(&self, symbol: &str) -> &'static str {
        match self.detect_market(symbol) {
            MarketKind::AShare => "akshare:tencent_quote+tushare_daily",
            MarketKind::HongKong => "akshare:tencent_quote+yahoo_finance_chart",
            MarketKind::UsEquity => "akshare:sina_us_daily+yahoo_finance_chart+stooq",
        }
    }

    /// Data source label for fundamentals.
    pub fn fundamentals_source(&self, symbol: &str) -> &'static str {
        match self.detect_market(symbol) {
            MarketKind::AShare => "akshare:eastmoney+tushare",
            MarketKind::HongKong => "akshare:tencent_quote+eastmoney_search",
            MarketKind::UsEquity => "akshare:sec_edgar",
        }
    }

    /// Data source label for news.
    pub fn news_source(&self, symbol: &str) -> &'static str {
        match self.detect_market(symbol) {
            MarketKind::AShare => "akshare:eastmoney+searxng_news",
            MarketKind::HongKong => "akshare:searxng_news",
            MarketKind::UsEquity => "akshare:sec_edgar+searxng_news",
        }
    }

    /// Data source label for candles.
    pub fn candles_source(&self, symbol: &str) -> &'static str {
        match self.detect_market(symbol) {
            MarketKind::AShare => "akshare:tencent_kline+eastmoney_kline",
            MarketKind::HongKong => "akshare:tencent_kline+yahoo_finance_chart",
            MarketKind::UsEquity => "akshare:sina_us_daily+yahoo_finance_chart+stooq",
        }
    }

    /// Data source label for capital flow.
    pub fn capital_flow_source(&self, symbol: &str) -> &'static str {
        match self.detect_market(symbol) {
            MarketKind::AShare => "akshare:eastmoney_fund_flow",
            _ => "akshare:unsupported",
        }
    }

    /// Classify an error by kind.
    pub fn error_kind(&self, error: &anyhow::Error) -> &'static str {
        if let Some(de) = error.downcast_ref::<DataError>() {
            return de.kind.as_str();
        }
        if let Some(re) = error.downcast_ref::<reqwest::Error>() {
            if re.is_timeout() {
                return "timeout";
            }
            if re.is_connect() {
                return "connect_error";
            }
            if re.is_status() {
                return match re.status() {
                    Some(s)
                        if s == reqwest::StatusCode::FORBIDDEN
                            || s == reqwest::StatusCode::UNAUTHORIZED =>
                    {
                        "permission_denied"
                    }
                    Some(s) if s == reqwest::StatusCode::NOT_FOUND => "not_found",
                    _ => "upstream_error",
                };
            }
        }
        "upstream_error"
    }
}

/// Extract text content from an XML tag (handles CDATA).
fn extract_rss_tag(xml: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{tag}>");
    let end_tag = format!("</{tag}>");
    let start = xml.find(&start_tag)? + start_tag.len();
    let end = xml.find(&end_tag)?;
    let value = xml[start..end].trim();
    let value = value
        .strip_prefix("<![CDATA[")
        .and_then(|s| s.strip_suffix("]]>"))
        .unwrap_or(value);
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

/// Normalize RSS date string to YYYY-MM-DD format.
fn normalize_rss_date(raw: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(raw) {
        return dt.format("%Y-%m-%d").to_string();
    }
    if raw.len() >= 10 && raw.as_bytes()[4] == b'-' && raw.as_bytes()[7] == b'-' {
        return raw[..10].to_string();
    }
    String::new()
}

impl MarketDataClient {
    /// Fetch news items from Bing RSS for the given queries.
    ///
    /// Returns parsed `NewsItem`s. Callers are responsible for deduplication.
    pub(crate) async fn fetch_bing_rss_news(
        &self,
        queries: &[&str],
        max_queries: usize,
    ) -> Vec<NewsItem> {
        let mut items = Vec::new();
        for query in queries.iter().take(max_queries) {
            let rss_url = format!(
                "https://cn.bing.com/search?q={}&format=rss",
                query.replace(' ', "+")
            );
            if let Ok(Ok(response)) = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                self.http.get(&rss_url).send(),
            )
            .await
                && let Ok(body) = response.text().await
            {
                for item_xml in body.split("<item>").skip(1) {
                    let end = item_xml.find("</item>").unwrap_or(item_xml.len());
                    let xml = &item_xml[..end];
                    let title = extract_rss_tag(xml, "title")
                        .filter(|t| !t.contains("必应") && !t.contains("Bing"));
                    let link = extract_rss_tag(xml, "link");
                    let desc = extract_rss_tag(xml, "description");
                    let date = extract_rss_tag(xml, "pubDate")
                        .map(|d| normalize_rss_date(&d))
                        .unwrap_or_default();
                    if let (Some(title), Some(url)) = (title, link) {
                        items.push(NewsItem {
                            published_at: date,
                            title,
                            summary: desc.unwrap_or_default(),
                            source: "bing_rss".to_string(),
                            url: Some(url),
                        });
                    }
                }
            }
        }
        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_error_kind_as_str() {
        assert_eq!(
            DataErrorKind::UnsupportedMarket.as_str(),
            "unsupported_market"
        );
        assert_eq!(
            DataErrorKind::PermissionDenied.as_str(),
            "permission_denied"
        );
        assert_eq!(DataErrorKind::Restricted.as_str(), "restricted");
        assert_eq!(
            DataErrorKind::MissingCredentials.as_str(),
            "missing_credentials"
        );
        assert_eq!(DataErrorKind::NotFound.as_str(), "not_found");
        assert_eq!(DataErrorKind::Upstream.as_str(), "upstream_error");
    }

    #[test]
    fn search_provider_display_names() {
        assert_eq!(
            SearchProviderConfig::searxng("searxng", "http://localhost").display_name(),
            "SearXNG News"
        );
        assert_eq!(
            SearchProviderConfig::gdelt("gdelt", "http://gdelt").display_name(),
            "GDELT News"
        );
        assert_eq!(
            SearchProviderConfig::baidu("baidu").display_name(),
            "Baidu News"
        );
    }

    #[test]
    fn rewrite_query_for_gdelt_basic() {
        assert_eq!(
            rewrite_query_for_gdelt("Apple Inc stock price", "en-US"),
            "Apple Inc stock price"
        );
    }
}
