mod batch;
mod billboard;
mod fund_flow;
mod margin;

use anyhow::Context;
use opentelemetry::KeyValue;
use tracing::Instrument;

use super::cache::SingleflightResult;
use super::{
    AnnouncementDetail, AnnouncementItem, CANDLES_CACHE_TTL_SECS, CANDLES_CACHE_VERSION,
    CandlePoint, CandlesWithProvider, CapitalFlowPoint, FUNDAMENTALS_CACHE_TTL_SECS,
    FUNDAMENTALS_CACHE_VERSION, FundamentalsSnapshot, GLOBAL_NEWS_CACHE_VERSION,
    INSIDER_CACHE_TTL_SECS, MARKET_DATA_CACHE_PREFIX, MarketDataClient, MarketKind,
    NEWS_CACHE_TTL_SECS, NEWS_CACHE_VERSION, NewsFetchAttempt, NewsFetchResult, NewsItem,
    QUOTE_CACHE_TTL_SECS, QUOTE_CACHE_VERSION, QuoteSnapshot, QuoteWithProvider,
    SEARCH_CACHE_TTL_SECS, SEARCH_CACHE_VERSION, SectorConstituent, SectorSnapshot,
    StockSearchResult, TradeCalendarItem,
};

use super::{
    AnalystDetail, AnalystRank, BalanceSheet, BlockTradeActiveBranch, BlockTradeBranchRanking,
    CashFlowSheet, CommentDesireIndex, CommentFocusIndex, CommentHistScore,
    CommentOrgParticipation, DividendInfo, DzjyHygtj, DzjyMrtj, EarningsForecast,
    EarningsQuickReport, EarningsReport, EsgRating, GdfxHoldingAnalyse, GdfxHoldingChange,
    GdfxHoldingDetail, GdfxHoldingStatistic, GdfxTeamwork, GdfxTop10, Gdhs, GdhsDetail, Ggcg,
    GpzyDistributeEntry, GpzyIndustry, GpzyPledgeDetail, GpzyPledgeRatio, GpzyPledgeRatioDetail,
    GpzyProfile, HotStockXq, IndustryCategory, JgdyDetail, JgdyTj, PankouChange, ProfitSheet,
    StockComment, ZtPool, ZtPoolDtgc, ZtPoolPrevious, ZtPoolStrong, ZtPoolSubNew, ZtPoolZbgc,
};

use crate::stock::hk_extra::{
    HkFamousStock, HkFhpxDetailThs, HkGxlLg, HkHotRank, HkHotRankDetail, HkSpotQuote,
    HkValuationBaidu,
};
use crate::stock::us_extra::{UsFamousStock, UsPinkStock, UsSpotSina, UsValuationBaidu};
use crate::stock::xueqiu::XqStockSpot;

const ESG_CACHE_TTL_SECS: u64 = 24 * 60 * 60;
const INDUSTRY_CACHE_TTL_SECS: u64 = 24 * 60 * 60;

impl MarketDataClient {
    /// Fetch real-time quote snapshot for a stock symbol.
    pub async fn fetch_quote(&self, symbol: &str) -> anyhow::Result<QuoteSnapshot> {
        let start = std::time::Instant::now();
        let result = self
            .fetch_quote_with_provider(symbol)
            .await
            .map(|r| r.quote);
        let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
        let meter = opentelemetry::global::meter("stock-analyzer");
        let outcome = if result.is_ok() { "success" } else { "error" };
        let attrs = vec![
            KeyValue::new("market_data.type", "quote"),
            KeyValue::new("market_data.symbol", symbol.to_string()),
            KeyValue::new("market_data.outcome", outcome),
        ];
        meter
            .u64_counter("market_data_fetch_total")
            .build()
            .add(1, &attrs);
        meter
            .f64_histogram("market_data_fetch_duration_ms")
            .build()
            .record(dur_ms, &attrs);
        if result.is_err() {
            meter
                .u64_counter("market_data_fetch_errors_total")
                .build()
                .add(1, &attrs);
        }
        result
    }

    /// Fetch quote with provider attribution for diagnostics.
    pub async fn fetch_quote_with_provider(
        &self,
        symbol: &str,
    ) -> anyhow::Result<QuoteWithProvider> {
        let span = tracing::info_span!("market_data.fetch", data_type = "quote", symbol);
        async {
            let start = std::time::Instant::now();
            let market = self.detect_market(symbol);
            let normalized_symbol = self.cache_symbol(symbol, market);
            let cache_key = format!(
                "{MARKET_DATA_CACHE_PREFIX}:quote:{QUOTE_CACHE_VERSION}:{normalized_symbol}"
            );
            if let Some(cached) = self.cache_get_json::<QuoteSnapshot>(&cache_key).await {
                let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
                let meter = opentelemetry::global::meter("stock-analyzer");
                let attrs = vec![
                    KeyValue::new("market_data.type", "quote"),
                    KeyValue::new("market_data.symbol", symbol.to_string()),
                    KeyValue::new("market_data.outcome", "success"),
                ];
                meter
                    .u64_counter("market_data_fetch_total")
                    .build()
                    .add(1, &attrs);
                meter
                    .f64_histogram("market_data_fetch_duration_ms")
                    .build()
                    .record(dur_ms, &attrs);
                return Ok(QuoteWithProvider {
                    quote: cached,
                    provider: "redis_cache".to_string(),
                });
            }
            // Singleflight: prevent cache stampede when multiple users request the same stock
            let sf = self.singleflight.clone();
            let _sf_guard = match sf.enter(&cache_key).await {
                SingleflightResult::Leader(g) => Some(g),
                SingleflightResult::Waiting => {
                    if let Some(cached) = self.cache_get_json::<QuoteSnapshot>(&cache_key).await {
                        return Ok(QuoteWithProvider {
                            quote: cached,
                            provider: "redis_cache".to_string(),
                        });
                    }
                    None
                }
            };
            let result = super::akshare_rust::fetch_quote(self, symbol).await;
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "quote"),
                KeyValue::new("market_data.symbol", symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter
                .u64_counter("market_data_fetch_total")
                .build()
                .add(1, &attrs);
            meter
                .f64_histogram("market_data_fetch_duration_ms")
                .build()
                .record(dur_ms, &attrs);
            if !ok {
                meter
                    .u64_counter("market_data_fetch_errors_total")
                    .build()
                    .add(1, &attrs);
            }
            let qwp = result?;
            self.cache_set_json(&cache_key, QUOTE_CACHE_TTL_SECS, &qwp.quote)
                .await;
            Ok(qwp)
        }
        .instrument(span)
        .await
    }

    /// Fetch fundamental data (financials, valuation) for a stock.
    pub async fn fetch_fundamentals(&self, symbol: &str) -> anyhow::Result<FundamentalsSnapshot> {
        let span = tracing::info_span!("market_data.fetch", data_type = "fundamentals", symbol);
        async {
            let start = std::time::Instant::now();
            let market = self.detect_market(symbol);
            let normalized_symbol = self.cache_symbol(symbol, market);
            let cache_key = format!(
                "{MARKET_DATA_CACHE_PREFIX}:fundamentals:{FUNDAMENTALS_CACHE_VERSION}:{normalized_symbol}"
            );
            if let Some(cached) = self
                .cache_get_json::<FundamentalsSnapshot>(&cache_key)
                .await
            {
                let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
                let meter = opentelemetry::global::meter("stock-analyzer");
                let attrs = vec![
                    KeyValue::new("market_data.type", "fundamentals"),
                    KeyValue::new("market_data.symbol", symbol.to_string()),
                    KeyValue::new("market_data.outcome", "success"),
                ];
                meter.u64_counter("market_data_fetch_total").build().add(1, &attrs);
                meter.f64_histogram("market_data_fetch_duration_ms").build().record(dur_ms, &attrs);
                return Ok(cached);
            }
            // Singleflight: prevent cache stampede
            let sf = self.singleflight.clone();
            let _sf_guard = match sf.enter(&cache_key).await {
                SingleflightResult::Leader(g) => Some(g),
                SingleflightResult::Waiting => {
                    if let Some(cached) = self.cache_get_json::<FundamentalsSnapshot>(&cache_key).await {
                        return Ok(cached);
                    }
                    None
                }
            };
            let result = super::akshare_rust::fetch_fundamentals(self, symbol).await;
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "fundamentals"),
                KeyValue::new("market_data.symbol", symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter.u64_counter("market_data_fetch_total").build().add(1, &attrs);
            meter.f64_histogram("market_data_fetch_duration_ms").build().record(dur_ms, &attrs);
            if !ok {
                meter.u64_counter("market_data_fetch_errors_total").build().add(1, &attrs);
            }
            let snapshot = result?;
            self.cache_set_json(&cache_key, FUNDAMENTALS_CACHE_TTL_SECS, &snapshot)
                .await;
            Ok(snapshot)
        }.instrument(span).await
    }

    /// Fetch company-specific news articles.
    pub async fn fetch_news(
        &self,
        symbol: &str,
        limit: usize,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let span = tracing::info_span!("market_data.fetch", data_type = "news", symbol);
        async {
            let start = std::time::Instant::now();
            let result = self
                .fetch_news_with_diagnostics(symbol, limit, start_date, end_date)
                .await;
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "news"),
                KeyValue::new("market_data.symbol", symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter
                .u64_counter("market_data_fetch_total")
                .build()
                .add(1, &attrs);
            meter
                .f64_histogram("market_data_fetch_duration_ms")
                .build()
                .record(dur_ms, &attrs);
            if !ok {
                meter
                    .u64_counter("market_data_fetch_errors_total")
                    .build()
                    .add(1, &attrs);
            }
            Ok(result?.items)
        }
        .instrument(span)
        .await
    }

    /// Fetch news with fetch attempt diagnostics.
    pub async fn fetch_news_with_diagnostics(
        &self,
        symbol: &str,
        limit: usize,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> anyhow::Result<NewsFetchResult> {
        self.fetch_news_with_diagnostics_query(symbol, limit, start_date, end_date, None)
            .await
    }

    /// Fetch news with custom query and diagnostics.
    pub async fn fetch_news_with_diagnostics_query(
        &self,
        symbol: &str,
        limit: usize,
        start_date: Option<&str>,
        end_date: Option<&str>,
        query: Option<&str>,
    ) -> anyhow::Result<NewsFetchResult> {
        let span = tracing::info_span!("market_data.fetch", data_type = "news_detailed", symbol);
        async {
            let start = std::time::Instant::now();
            let market = self.detect_market(symbol);
            let normalized_symbol = self.cache_symbol(symbol, market);
            let query_cache_key = self.news_query_cache_component(query);
            let cache_key = format!(
                "{MARKET_DATA_CACHE_PREFIX}:news:{NEWS_CACHE_VERSION}:{normalized_symbol}:{limit}:{}:{}:{query_cache_key}",
                start_date.unwrap_or("-"),
                end_date.unwrap_or("-")
            );
            if let Some(mut cached) = self.cache_get_json::<NewsFetchResult>(&cache_key).await {
                if cached.attempts.is_empty() {
                    cached.attempts.push(NewsFetchAttempt {
                        source: "redis_cache".to_string(),
                        query: None,
                        success: true,
                        item_count: cached.items.len(),
                        error: None,
                    });
                }
                cached.cacheable = true;
                let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
                let meter = opentelemetry::global::meter("stock-analyzer");
                let attrs = vec![
                    KeyValue::new("market_data.type", "news_detailed"),
                    KeyValue::new("market_data.symbol", symbol.to_string()),
                    KeyValue::new("market_data.outcome", "success"),
                ];
                meter.u64_counter("market_data_fetch_total").build().add(1, &attrs);
                meter.f64_histogram("market_data_fetch_duration_ms").build().record(dur_ms, &attrs);
                return Ok(cached);
            }
            let result = self
                .fetch_market_news_diagnostics_query(symbol, market, limit, start_date, end_date, query)
                .await;
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "news_detailed"),
                KeyValue::new("market_data.symbol", symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter.u64_counter("market_data_fetch_total").build().add(1, &attrs);
            meter.f64_histogram("market_data_fetch_duration_ms").build().record(dur_ms, &attrs);
            if !ok {
                meter.u64_counter("market_data_fetch_errors_total").build().add(1, &attrs);
            }
            let result = result?;
            if result.cacheable && !result.items.is_empty() {
                self.cache_set_json(&cache_key, NEWS_CACHE_TTL_SECS, &result)
                    .await;
            }
            Ok(result)
        }.instrument(span).await
    }

    /// Fetch global/market news articles.
    pub async fn fetch_global_news(
        &self,
        market_hint_symbol: &str,
        curr_date: &str,
        look_back_days: usize,
        limit: usize,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let span = tracing::info_span!(
            "market_data.fetch",
            data_type = "global_news",
            symbol = market_hint_symbol
        );
        async {
            let start = std::time::Instant::now();
            let result = self
                .fetch_global_news_with_diagnostics(
                    market_hint_symbol,
                    curr_date,
                    look_back_days,
                    limit,
                )
                .await;
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "global_news"),
                KeyValue::new("market_data.symbol", market_hint_symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter
                .u64_counter("market_data_fetch_total")
                .build()
                .add(1, &attrs);
            meter
                .f64_histogram("market_data_fetch_duration_ms")
                .build()
                .record(dur_ms, &attrs);
            if !ok {
                meter
                    .u64_counter("market_data_fetch_errors_total")
                    .build()
                    .add(1, &attrs);
            }
            Ok(result?.items)
        }
        .instrument(span)
        .await
    }

    /// Fetch global news with fetch attempt diagnostics.
    pub async fn fetch_global_news_with_diagnostics(
        &self,
        market_hint_symbol: &str,
        curr_date: &str,
        look_back_days: usize,
        limit: usize,
    ) -> anyhow::Result<NewsFetchResult> {
        let market = self.detect_market(market_hint_symbol);
        let normalized_symbol = self.cache_symbol(market_hint_symbol, market);
        let cache_key = format!(
            "{MARKET_DATA_CACHE_PREFIX}:global-news:{GLOBAL_NEWS_CACHE_VERSION}:{normalized_symbol}:{curr_date}:{look_back_days}:{limit}"
        );
        if let Some(mut cached) = self.cache_get_json::<NewsFetchResult>(&cache_key).await {
            if cached.attempts.is_empty() {
                cached.attempts.push(NewsFetchAttempt {
                    source: "redis_cache".to_string(),
                    query: None,
                    success: true,
                    item_count: cached.items.len(),
                    error: None,
                });
            }
            cached.cacheable = true;
            return Ok(cached);
        }
        let result = self
            .fetch_market_global_news_diagnostics(
                market_hint_symbol,
                market,
                curr_date,
                look_back_days,
                limit,
            )
            .await?;
        if result.cacheable && !result.items.is_empty() {
            self.cache_set_json(&cache_key, super::GLOBAL_NEWS_CACHE_TTL_SECS, &result)
                .await;
        }
        Ok(result)
    }
}

impl MarketDataClient {
    /// Fetch insider transaction data for a stock.
    pub async fn fetch_insider_transactions(&self, symbol: &str) -> anyhow::Result<Vec<NewsItem>> {
        let span = tracing::info_span!("market_data.fetch", data_type = "insider", symbol);
        async {
            let start = std::time::Instant::now();
            let market = self.detect_market(symbol);
            let normalized_symbol = self.cache_symbol(symbol, market);
            let cache_key = format!("{MARKET_DATA_CACHE_PREFIX}:insider:{normalized_symbol}");
            if let Some(cached) = self.cache_get_json(&cache_key).await {
                let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
                let meter = opentelemetry::global::meter("stock-analyzer");
                let attrs = vec![
                    KeyValue::new("market_data.type", "insider"),
                    KeyValue::new("market_data.symbol", symbol.to_string()),
                    KeyValue::new("market_data.outcome", "success"),
                ];
                meter
                    .u64_counter("market_data_fetch_total")
                    .build()
                    .add(1, &attrs);
                meter
                    .f64_histogram("market_data_fetch_duration_ms")
                    .build()
                    .record(dur_ms, &attrs);
                return Ok(cached);
            }
            let result = super::akshare_rust::fetch_insider_transactions(self, symbol).await;
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "insider"),
                KeyValue::new("market_data.symbol", symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter
                .u64_counter("market_data_fetch_total")
                .build()
                .add(1, &attrs);
            meter
                .f64_histogram("market_data_fetch_duration_ms")
                .build()
                .record(dur_ms, &attrs);
            if !ok {
                meter
                    .u64_counter("market_data_fetch_errors_total")
                    .build()
                    .add(1, &attrs);
            }
            let items = result?;
            self.cache_set_json(&cache_key, INSIDER_CACHE_TTL_SECS, &items)
                .await;
            Ok(items)
        }
        .instrument(span)
        .await
    }

    /// Fetch historical candlestick (OHLCV) data.
    pub async fn fetch_candles(
        &self,
        symbol: &str,
        adjust: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CandlePoint>> {
        let span = tracing::info_span!("market_data.fetch", data_type = "candles", symbol);
        async {
            let start = std::time::Instant::now();
            let result = self
                .fetch_candles_with_provider(symbol, adjust, limit)
                .await
                .map(|r| r.candles);
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "candles"),
                KeyValue::new("market_data.symbol", symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter
                .u64_counter("market_data_fetch_total")
                .build()
                .add(1, &attrs);
            meter
                .f64_histogram("market_data_fetch_duration_ms")
                .build()
                .record(dur_ms, &attrs);
            if !ok {
                meter
                    .u64_counter("market_data_fetch_errors_total")
                    .build()
                    .add(1, &attrs);
            }
            result
        }
        .instrument(span)
        .await
    }

    /// Fetch candles with provider attribution for diagnostics.
    pub async fn fetch_candles_with_provider(
        &self,
        symbol: &str,
        adjust: &str,
        limit: usize,
    ) -> anyhow::Result<CandlesWithProvider> {
        let span = tracing::info_span!("market_data.fetch", data_type = "candles", symbol);
        async {
            let start = std::time::Instant::now();
            let market = self.detect_market(symbol);
            let normalized_symbol = self.cache_symbol(symbol, market);
            let cache_key = format!(
                "{MARKET_DATA_CACHE_PREFIX}:candles:{CANDLES_CACHE_VERSION}:{normalized_symbol}:{adjust}:{limit}"
            );
            if let Some(cached) = self.cache_get_json(&cache_key).await {
                let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
                let meter = opentelemetry::global::meter("stock-analyzer");
                let attrs = vec![
                    KeyValue::new("market_data.type", "candles"),
                    KeyValue::new("market_data.symbol", symbol.to_string()),
                    KeyValue::new("market_data.outcome", "success"),
                ];
                meter.u64_counter("market_data_fetch_total").build().add(1, &attrs);
                meter.f64_histogram("market_data_fetch_duration_ms").build().record(dur_ms, &attrs);
                return Ok(CandlesWithProvider { candles: cached, provider: "redis_cache".to_string() });
            }
            // Singleflight: prevent cache stampede
            let sf = self.singleflight.clone();
            let _sf_guard = match sf.enter(&cache_key).await {
                SingleflightResult::Leader(g) => Some(g),
                SingleflightResult::Waiting => {
                    if let Some(cached) = self.cache_get_json(&cache_key).await {
                        return Ok(CandlesWithProvider { candles: cached, provider: "redis_cache".to_string() });
                    }
                    None
                }
            };
            let result = super::akshare_rust::fetch_candles(self, symbol, adjust, limit).await;
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "candles"),
                KeyValue::new("market_data.symbol", symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter.u64_counter("market_data_fetch_total").build().add(1, &attrs);
            meter.f64_histogram("market_data_fetch_duration_ms").build().record(dur_ms, &attrs);
            if !ok {
                meter.u64_counter("market_data_fetch_errors_total").build().add(1, &attrs);
            }
            let cwp = result?;
            if !cwp.candles.is_empty() {
                self.cache_set_json(&cache_key, CANDLES_CACHE_TTL_SECS, &cwp.candles)
                    .await;
            }
            Ok(cwp)
        }.instrument(span).await
    }

    /// Fetch capital flow (money flow) data for a stock.
    pub async fn fetch_capital_flow(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CapitalFlowPoint>> {
        let span = tracing::info_span!("market_data.fetch", data_type = "capital_flow", symbol);
        async {
            let start = std::time::Instant::now();
            let market = self.detect_market(symbol);
            let normalized_symbol = self.cache_symbol(symbol, market);
            let cache_key =
                format!("{MARKET_DATA_CACHE_PREFIX}:capital-flow:{normalized_symbol}:{limit}");
            if let Some(cached) = self.cache_get_json(&cache_key).await {
                let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
                let meter = opentelemetry::global::meter("stock-analyzer");
                let attrs = vec![
                    KeyValue::new("market_data.type", "capital_flow"),
                    KeyValue::new("market_data.symbol", symbol.to_string()),
                    KeyValue::new("market_data.outcome", "success"),
                ];
                meter
                    .u64_counter("market_data_fetch_total")
                    .build()
                    .add(1, &attrs);
                meter
                    .f64_histogram("market_data_fetch_duration_ms")
                    .build()
                    .record(dur_ms, &attrs);
                return Ok(cached);
            }
            let fetch_result = match market {
                MarketKind::AShare => self.fetch_a_share_capital_flow(symbol, limit).await,
                MarketKind::HongKong => self.fetch_hk_capital_flow(symbol, limit).await,
                MarketKind::UsEquity => self.fetch_us_capital_flow(symbol, limit).await,
            };
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = fetch_result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "capital_flow"),
                KeyValue::new("market_data.symbol", symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter
                .u64_counter("market_data_fetch_total")
                .build()
                .add(1, &attrs);
            meter
                .f64_histogram("market_data_fetch_duration_ms")
                .build()
                .record(dur_ms, &attrs);
            if !ok {
                meter
                    .u64_counter("market_data_fetch_errors_total")
                    .build()
                    .add(1, &attrs);
            }
            let items = fetch_result?;
            self.cache_set_json(&cache_key, CANDLES_CACHE_TTL_SECS, &items)
                .await;
            Ok(items)
        }
        .instrument(span)
        .await
    }

    /// Fetch capital flow for an HK stock via Eastmoney (secid prefix 116).
    async fn fetch_hk_capital_flow(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CapitalFlowPoint>> {
        let code = symbol.trim().trim_start_matches('0');
        let code = if code.is_empty() { "0" } else { code };
        let secid = format!("116.{code}");
        let result = self.fetch_capital_flow_primary(&secid, limit).await;
        match result {
            Ok(items) => Ok(items),
            Err(_) => {
                // Fallback: use clist endpoint for current-day data
                self.fetch_capital_flow_from_clist(code, "m:128+t:3,m:128+t:4")
                    .await
            }
        }
    }

    /// Fetch capital flow for a US stock via Eastmoney (secid prefix 105).
    ///
    /// Uses the same Eastmoney push2his API as A-share/HK stocks.
    /// Market codes: 105 = NASDAQ, 106 = NYSE, 107 = AMEX.
    /// Defaults to 105 (NASDAQ) since most major US stocks trade there.
    async fn fetch_us_capital_flow(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CapitalFlowPoint>> {
        let code = symbol.trim().to_uppercase();
        let secid = format!("105.{code}");
        let result = self.fetch_capital_flow_primary(&secid, limit).await;
        match result {
            Ok(items) => Ok(items),
            Err(_) => {
                // Fallback: use clist endpoint for current-day data
                self.fetch_capital_flow_from_clist(&code, "m:105+t:3,m:106+t:3,m:107+t:3")
                    .await
            }
        }
    }

    /// Shared primary capital flow fetch via push2his fflow/daykline endpoint.
    async fn fetch_capital_flow_primary(
        &self,
        secid: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CapitalFlowPoint>> {
        let response = self
            .http
            .get("https://push2his.eastmoney.com/api/qt/stock/fflow/daykline/get")
            .query(&[
                ("secid", secid),
                ("klt", "101"),
                ("lmt", &limit.to_string()),
                ("fields1", "f1,f2,f3,f7"),
                (
                    "fields2",
                    "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64,f65",
                ),
            ])
            .send()
            .await
            .context("failed to fetch capital flow from Eastmoney")?
            .error_for_status()
            .context("eastmoney capital flow request failed")?;
        let payload: crate::provider::market_client::wire::EastmoneyKlineEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney capital flow response")?;
        let data = payload
            .data
            .context("eastmoney capital flow response missing data")?;
        let klines = data
            .klines
            .context("eastmoney capital flow response missing klines")?;
        let mut items = klines
            .into_iter()
            .map(|line| Self::parse_a_share_capital_flow_line(&line))
            .collect::<anyhow::Result<Vec<_>>>()?;
        if items.is_empty() {
            anyhow::bail!("eastmoney returned no capital flow items");
        }
        items.truncate(limit);
        Ok(items)
    }

    /// Fallback: fetch current-day capital flow from the clist endpoint.
    ///
    /// Uses `push2.eastmoney.com/api/qt/clist/get` which returns fund flow
    /// data for all stocks. Filters by `target_code` in the response.
    /// Returns at most 1 data point (today's flow).
    pub(crate) async fn fetch_capital_flow_from_clist(
        &self,
        target_code: &str,
        market_fs: &str,
    ) -> anyhow::Result<Vec<CapitalFlowPoint>> {
        #[derive(serde::Deserialize)]
        struct ClistEnv {
            data: Option<ClistData>,
        }
        #[derive(serde::Deserialize)]
        struct ClistData {
            diff: Option<Vec<serde_json::Value>>,
        }

        let response = self
            .http
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", "5000"),
                ("po", "1"),
                ("np", "1"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f62"),
                ("fs", market_fs),
                (
                    "fields",
                    "f12,f14,f2,f3,f62,f184,f66,f69,f72,f75,f78,f81,f84,f87",
                ),
            ])
            .send()
            .await
            .context("failed to fetch capital flow from clist")?
            .error_for_status()
            .context("clist capital flow request failed")?;

        let payload: ClistEnv = response
            .json()
            .await
            .context("failed to decode clist capital flow response")?;

        let diff = payload.data.and_then(|d| d.diff).unwrap_or_default();

        let normalized_target = target_code.trim().to_uppercase();
        for item in &diff {
            let code = item
                .get("f12")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_uppercase();
            if code != normalized_target {
                continue;
            }
            let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
            let close = item.get("f2").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let change_pct = item.get("f3").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let main_net_inflow = item.get("f62").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let main_pct = item.get("f184").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let super_large = item.get("f66").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let super_large_pct = item.get("f69").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let large = item.get("f72").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let large_pct = item.get("f75").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let medium = item.get("f78").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let medium_pct = item.get("f81").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let small = item.get("f84").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let small_pct = item.get("f87").and_then(|v| v.as_f64()).unwrap_or(0.0);

            return Ok(vec![CapitalFlowPoint {
                trade_date: today,
                main_net_inflow,
                small_net_inflow: small,
                medium_net_inflow: medium,
                large_net_inflow: large,
                super_large_net_inflow: super_large,
                main_net_inflow_ratio_pct: main_pct,
                small_net_inflow_ratio_pct: small_pct,
                medium_net_inflow_ratio_pct: medium_pct,
                large_net_inflow_ratio_pct: large_pct,
                super_large_net_inflow_ratio_pct: super_large_pct,
                close,
                change_pct,
            }]);
        }

        anyhow::bail!("stock {target_code} not found in clist capital flow response");
    }

    /// Fetch A-share sector/concept board rankings.
    pub async fn fetch_a_share_sector_rankings(
        &self,
        sector_type: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<SectorSnapshot>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:a-share-sector-rankings:{}:{}",
                sector_type.trim().to_lowercase(),
                limit
            ),
            CANDLES_CACHE_TTL_SECS,
            || self.fetch_a_share_sector_rankings_from_sina(sector_type, limit),
        )
        .await
    }

    /// Fetch constituent stocks of a sector/concept board.
    pub async fn fetch_a_share_sector_constituents(
        &self,
        sector_code: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<SectorConstituent>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:a-share-sector-constituents:{}:{}",
                sector_code.trim().to_uppercase(),
                limit
            ),
            CANDLES_CACHE_TTL_SECS,
            || self.fetch_a_share_sector_constituents_from_eastmoney(sector_code, limit),
        )
        .await
    }

    /// Fetch capital flow data for A-share sectors.
    pub async fn fetch_a_share_sector_capital_flow(
        &self,
        sector_code: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CapitalFlowPoint>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:a-share-sector-capital-flow:{}:{}",
                sector_code.trim().to_uppercase(),
                limit
            ),
            CANDLES_CACHE_TTL_SECS,
            || self.fetch_a_share_sector_capital_flow_from_eastmoney(sector_code, limit),
        )
        .await
    }

    /// Fetch detail of a specific announcement.
    pub async fn fetch_announcement_detail(
        &self,
        art_code: &str,
    ) -> anyhow::Result<AnnouncementDetail> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:announcement-detail:{}",
                art_code.trim()
            ),
            SEARCH_CACHE_TTL_SECS,
            || self.fetch_a_share_announcement_detail(art_code),
        )
        .await
    }

    /// Fetch company announcements/filings.
    pub async fn fetch_announcements(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<AnnouncementItem>> {
        let span = tracing::info_span!("market_data.fetch", data_type = "announcements", symbol);
        async {
            let start = std::time::Instant::now();
            let market = self.detect_market(symbol);
            let normalized_symbol = self.cache_symbol(symbol, market);
            let cache_key = format!(
                "{MARKET_DATA_CACHE_PREFIX}:announcements:{}:{}",
                normalized_symbol, limit
            );
            if let Some(cached) = self.cache_get_json(&cache_key).await {
                let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
                let meter = opentelemetry::global::meter("stock-analyzer");
                let attrs = vec![
                    KeyValue::new("market_data.type", "announcements"),
                    KeyValue::new("market_data.symbol", symbol.to_string()),
                    KeyValue::new("market_data.outcome", "success"),
                ];
                meter
                    .u64_counter("market_data_fetch_total")
                    .build()
                    .add(1, &attrs);
                meter
                    .f64_histogram("market_data_fetch_duration_ms")
                    .build()
                    .record(dur_ms, &attrs);
                return Ok(cached);
            }
            let fetch_result = match market {
                MarketKind::AShare => {
                    let ts_code = self
                        .normalize_a_share_symbol(symbol)
                        .unwrap_or_else(|| symbol.trim().to_uppercase());
                    self.fetch_a_share_announcements(&ts_code, limit).await
                }
                MarketKind::HongKong => self.fetch_hk_announcements(symbol, limit).await,
                MarketKind::UsEquity => self.fetch_us_announcements(symbol, limit).await,
            };
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = fetch_result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "announcements"),
                KeyValue::new("market_data.symbol", symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter
                .u64_counter("market_data_fetch_total")
                .build()
                .add(1, &attrs);
            meter
                .f64_histogram("market_data_fetch_duration_ms")
                .build()
                .record(dur_ms, &attrs);
            if !ok {
                meter
                    .u64_counter("market_data_fetch_errors_total")
                    .build()
                    .add(1, &attrs);
            }
            let items = fetch_result?;
            self.cache_set_json(&cache_key, NEWS_CACHE_TTL_SECS, &items)
                .await;
            Ok(items)
        }
        .instrument(span)
        .await
    }

    /// Fetch announcements for an HK stock via Eastmoney.
    async fn fetch_hk_announcements(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<AnnouncementItem>> {
        let code = symbol.trim().trim_start_matches('0');
        let code = if code.is_empty() { "0" } else { code };
        let page_size = limit.clamp(1, 100).to_string();

        let response = self
            .http
            .get("https://np-anotice-stock.eastmoney.com/api/security/ann")
            .query(&[
                ("page_size", page_size.as_str()),
                ("page_index", "1"),
                ("ann_type", "A"),
                ("client_source", "web"),
                ("stock_list", code),
            ])
            .send()
            .await
            .context("failed to fetch HK announcements from Eastmoney")?
            .error_for_status()
            .context("eastmoney HK announcements request failed")?;

        #[derive(serde::Deserialize)]
        struct Envelope {
            data: Option<Data>,
        }
        #[derive(serde::Deserialize)]
        struct Data {
            list: Option<Vec<WireItem>>,
        }
        #[derive(serde::Deserialize)]
        struct WireItem {
            #[serde(default)]
            art_code: Option<String>,
            #[serde(default)]
            title: Option<String>,
            #[serde(default)]
            notice_date: Option<String>,
        }

        let payload: Envelope = response
            .json()
            .await
            .context("failed to decode eastmoney HK announcements response")?;
        let mut items = payload
            .data
            .and_then(|d| d.list)
            .unwrap_or_default()
            .into_iter()
            .map(|item| {
                let art_code = item.art_code.unwrap_or_default();
                AnnouncementItem {
                    url: (!art_code.is_empty()).then(|| {
                        format!("https://data.eastmoney.com/notices/detail/{code}/{art_code}.html")
                    }),
                    art_code,
                    symbol: code.to_string(),
                    title: item.title.unwrap_or_else(|| "公司公告".to_string()),
                    published_at: item.notice_date.unwrap_or_default(),
                    source: "Eastmoney 公告".to_string(),
                }
            })
            .collect::<Vec<_>>();

        // Eastmoney np-anotice-stock API only supports A-shares.
        // HK stock announcements are not available through this endpoint.
        // Return empty gracefully rather than failing.
        items.truncate(limit);
        Ok(items)
    }

    /// Fetch announcements for a US stock via SEC EDGAR.
    ///
    /// Uses the SEC EDGAR full-text search API (no API key required).
    /// Returns 8-K, 10-K, 10-Q, 6-K, and other material filings.
    async fn fetch_us_announcements(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<AnnouncementItem>> {
        let ticker = symbol.trim().to_uppercase();

        // Step 1: Look up CIK from SEC EDGAR company_tickers.json
        let cik_response = self
            .http
            .get("https://www.sec.gov/files/company_tickers.json")
            .header("User-Agent", "akshare-rs research@example.com")
            .send()
            .await
            .context("failed to fetch SEC EDGAR company tickers")?
            .error_for_status()
            .context("SEC EDGAR company tickers request failed")?;

        let tickers: std::collections::HashMap<String, serde_json::Value> = cik_response
            .json()
            .await
            .context("failed to decode SEC EDGAR company tickers")?;

        let cik = tickers
            .values()
            .find_map(|v| {
                let t = v.get("ticker")?.as_str()?;
                if t == ticker {
                    let cik_num = v.get("cik_str")?.as_i64()?;
                    Some(format!("{:010}", cik_num))
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("ticker {ticker} not found in SEC EDGAR"))?;

        // Step 2: Fetch recent filings from SEC EDGAR submissions API
        let url = format!("https://data.sec.gov/submissions/CIK{cik}.json");
        let response = self
            .http
            .get(&url)
            .header("User-Agent", "akshare-rs research@example.com")
            .send()
            .await
            .context("failed to fetch SEC EDGAR submissions")?
            .error_for_status()
            .context("SEC EDGAR submissions request failed")?;

        #[derive(serde::Deserialize)]
        struct EdgarSubmissions {
            filings: EdgarFilings,
        }
        #[derive(serde::Deserialize)]
        struct EdgarFilings {
            recent: EdgarRecent,
        }
        #[derive(serde::Deserialize)]
        struct EdgarRecent {
            form: Vec<String>,
            #[serde(rename = "filingDate")]
            filing_date: Vec<String>,
            #[serde(rename = "primaryDocDescription")]
            primary_doc_description: Vec<String>,
            #[serde(rename = "accessionNumber")]
            accession_number: Vec<String>,
        }

        let submissions: EdgarSubmissions = response
            .json()
            .await
            .context("failed to decode SEC EDGAR submissions")?;

        // Filter for material filing types
        let material_forms: std::collections::HashSet<&str> = [
            "8-K", "10-K", "10-Q", "6-K", "20-F", "40-F", "S-1", "S-3", "DEF 14A",
        ]
        .into_iter()
        .collect();

        let mut items = Vec::new();
        let recent = &submissions.filings.recent;
        let count = recent.form.len().min(recent.filing_date.len());

        for i in 0..count {
            if items.len() >= limit {
                break;
            }
            let form = &recent.form[i];
            if !material_forms.contains(form.as_str()) {
                continue;
            }
            let date = &recent.filing_date[i];
            let desc = recent
                .primary_doc_description
                .get(i)
                .and_then(|d| if d.is_empty() { None } else { Some(d.as_str()) })
                .unwrap_or(form.as_str());
            let accession = &recent.accession_number[i];
            let accession_dashless = accession.replace('-', "");
            let url = format!(
                "https://www.sec.gov/Archives/edgar/data/{}/{}/{}",
                cik.trim_start_matches('0'),
                accession_dashless,
                accession
            );

            items.push(AnnouncementItem {
                url: Some(url),
                art_code: accession.clone(),
                symbol: ticker.clone(),
                title: format!("{form}: {desc}"),
                published_at: date.clone(),
                source: "SEC EDGAR".to_string(),
            });
        }

        if items.is_empty() {
            anyhow::bail!("SEC EDGAR returned no announcement items for {ticker}");
        }
        Ok(items)
    }

    /// Search stocks by keyword across markets.
    pub async fn search_stocks(
        &self,
        query: &str,
        market: Option<&str>,
        limit: usize,
    ) -> anyhow::Result<Vec<StockSearchResult>> {
        let span = tracing::info_span!(
            "market_data.fetch",
            data_type = "stock_search",
            symbol = query
        );
        async {
            let start = std::time::Instant::now();
            let normalized_query = query.trim().to_uppercase();
            let normalized_market = market.unwrap_or("all");
            let cache_key = format!(
                "{MARKET_DATA_CACHE_PREFIX}:search:{SEARCH_CACHE_VERSION}:{normalized_market}:{}:{limit}",
                normalized_query
            );
            if let Some(cached) = self.cache_get_json(&cache_key).await {
                let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
                let meter = opentelemetry::global::meter("stock-analyzer");
                let attrs = vec![
                    KeyValue::new("market_data.type", "stock_search"),
                    KeyValue::new("market_data.symbol", query.to_string()),
                    KeyValue::new("market_data.outcome", "success"),
                ];
                meter.u64_counter("market_data_fetch_total").build().add(1, &attrs);
                meter.f64_histogram("market_data_fetch_duration_ms").build().record(dur_ms, &attrs);
                return Ok(cached);
            }
            let result = self
                .search_stocks_with_fallbacks(query, market, limit)
                .await;
            let dur_ms = start.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "stock_search"),
                KeyValue::new("market_data.symbol", query.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter.u64_counter("market_data_fetch_total").build().add(1, &attrs);
            meter.f64_histogram("market_data_fetch_duration_ms").build().record(dur_ms, &attrs);
            if !ok {
                meter.u64_counter("market_data_fetch_errors_total").build().add(1, &attrs);
            }
            let items = result?;
            self.cache_set_json(&cache_key, SEARCH_CACHE_TTL_SECS, &items)
                .await;
            Ok(items)
        }.instrument(span).await
    }

    /// Fetch trade calendar (market open/close dates).
    pub async fn fetch_trade_calendar(
        &self,
        exchange: &str,
        start_date: &str,
        end_date: &str,
    ) -> anyhow::Result<Vec<TradeCalendarItem>> {
        let exchange = exchange.trim().to_uppercase();
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:trade-calendar:{}:{}:{}",
                exchange, start_date, end_date
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || async move {
                let rows = self
                    .tushare_query(
                        "trade_cal",
                        serde_json::json!({
                            "exchange": exchange,
                            "start_date": start_date,
                            "end_date": end_date
                        }),
                        "exchange,cal_date,is_open,pretrade_date",
                    )
                    .await?;
                rows.into_iter()
                    .map(|row| {
                        Ok(TradeCalendarItem {
                            exchange: row.optional_string("exchange").unwrap_or_default(),
                            calendar_date: row.string("cal_date")?,
                            is_open: row.optional_f64("is_open").unwrap_or_default() > 0.0,
                            previous_trade_date: row.optional_string("pretrade_date"),
                        })
                    })
                    .collect::<anyhow::Result<Vec<_>>>()
            },
        )
        .await
    }
}

impl MarketDataClient {
    /// Fetch price return since a given date.
    pub async fn fetch_return_since(
        &self,
        symbol: &str,
        start_date: &str,
        holding_days: usize,
    ) -> anyhow::Result<Option<f64>> {
        let span = tracing::info_span!("market_data.fetch", data_type = "return_since", symbol);
        async {
            let timer = std::time::Instant::now();
            let result =
                super::akshare_rust::fetch_return_since(self, symbol, start_date, holding_days)
                    .await;
            let dur_ms = timer.elapsed().as_secs_f64() * 1000.0;
            let meter = opentelemetry::global::meter("stock-analyzer");
            let ok = result.is_ok();
            let outcome = if ok { "success" } else { "error" };
            let attrs = vec![
                KeyValue::new("market_data.type", "return_since"),
                KeyValue::new("market_data.symbol", symbol.to_string()),
                KeyValue::new("market_data.outcome", outcome),
            ];
            meter
                .u64_counter("market_data_fetch_total")
                .build()
                .add(1, &attrs);
            meter
                .f64_histogram("market_data_fetch_duration_ms")
                .build()
                .record(dur_ms, &attrs);
            if !ok {
                meter
                    .u64_counter("market_data_fetch_errors_total")
                    .build()
                    .add(1, &attrs);
            }
            result
        }
        .instrument(span)
        .await
    }

    pub(super) async fn fetch_market_news_diagnostics_query(
        &self,
        symbol: &str,
        market: MarketKind,
        limit: usize,
        start_date: Option<&str>,
        end_date: Option<&str>,
        query: Option<&str>,
    ) -> anyhow::Result<NewsFetchResult> {
        match market {
            MarketKind::AShare => {
                let ts_code = self
                    .normalize_a_share_symbol(symbol)
                    .context("invalid A-share symbol for news")?;
                self.fetch_a_share_news_diagnostics(&ts_code, limit).await
            }
            MarketKind::HongKong => {
                self.fetch_hk_news_diagnostics_query(symbol, limit, start_date, end_date, query)
                    .await
            }
            MarketKind::UsEquity => {
                self.fetch_us_news_diagnostics(symbol, limit, start_date, end_date)
                    .await
            }
        }
    }

    pub(super) async fn fetch_market_global_news_diagnostics(
        &self,
        _market_hint_symbol: &str,
        market: MarketKind,
        curr_date: &str,
        look_back_days: usize,
        limit: usize,
    ) -> anyhow::Result<NewsFetchResult> {
        match market {
            MarketKind::AShare => {
                self.fetch_a_share_global_news_diagnostics(curr_date, look_back_days, limit)
                    .await
            }
            MarketKind::HongKong => {
                self.fetch_hk_global_news_diagnostics(curr_date, look_back_days, limit)
                    .await
            }
            MarketKind::UsEquity => {
                self.fetch_us_global_news_diagnostics(curr_date, look_back_days, limit)
                    .await
            }
        }
    }
}

impl MarketDataClient {
    // -----------------------------------------------------------------------
    // Limit-Up/Down Pools (涨停/跌停股池)
    // -----------------------------------------------------------------------

    /// Fetch limit-up (涨停) stock pool for a date.
    pub async fn fetch_zt_pool(&self, date: &str) -> anyhow::Result<Vec<ZtPool>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:zt-pool:{}", date),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_zt_pool(self, date),
        )
        .await
    }

    /// Fetch limit-up pool with first-board data.
    pub async fn fetch_zt_pool_dtgc(&self, date: &str) -> anyhow::Result<Vec<ZtPoolDtgc>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:zt-pool-dtgc:{}", date),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_zt_pool_dtgc(self, date),
        )
        .await
    }

    /// Fetch previous-day limit-up pool.
    pub async fn fetch_zt_pool_previous(&self, date: &str) -> anyhow::Result<Vec<ZtPoolPrevious>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:zt-pool-prev:{}", date),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_zt_pool_previous(self, date),
        )
        .await
    }

    /// Fetch strong limit-up pool.
    pub async fn fetch_zt_pool_strong(&self, date: &str) -> anyhow::Result<Vec<ZtPoolStrong>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:zt-pool-strong:{}", date),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_zt_pool_strong(self, date),
        )
        .await
    }

    /// Fetch sub-new (次新) limit-up pool.
    pub async fn fetch_zt_pool_sub_new(&self, date: &str) -> anyhow::Result<Vec<ZtPoolSubNew>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:zt-pool-subnew:{}", date),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_zt_pool_sub_new(self, date),
        )
        .await
    }

    /// Fetch limit-up pool with opened-board data.
    pub async fn fetch_zt_pool_zbgc(&self, date: &str) -> anyhow::Result<Vec<ZtPoolZbgc>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:zt-pool-zbgc:{}", date),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_zt_pool_zbgc(self, date),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Earnings (业绩)
    // -----------------------------------------------------------------------

    /// Fetch earnings forecast data.
    pub async fn fetch_earnings_forecast(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<EarningsForecast>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:earnings-forecast:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_earnings_forecast(self, date),
        )
        .await
    }

    /// Fetch earnings quick report data.
    pub async fn fetch_earnings_quick_report(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<EarningsQuickReport>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:earnings-quick:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_earnings_quick_report(self, date),
        )
        .await
    }

    /// Fetch full earnings report.
    pub async fn fetch_earnings_report(&self, date: &str) -> anyhow::Result<Vec<EarningsReport>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:earnings-report:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_earnings_report(self, date),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Analyst (分析师)
    // -----------------------------------------------------------------------

    /// Fetch analyst ranking data.
    pub async fn fetch_analyst_rank(&self, year: &str) -> anyhow::Result<Vec<AnalystRank>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:analyst-rank:{}", year),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_analyst_rank(self, year),
        )
        .await
    }

    /// Fetch detail for a specific analyst.
    pub async fn fetch_analyst_detail(
        &self,
        analyst_id: &str,
        indicator: &str,
    ) -> anyhow::Result<Vec<AnalystDetail>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:analyst-detail:{}:{}",
                analyst_id, indicator
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_analyst_detail(self, analyst_id, indicator),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Shareholder Analysis (股东分析)
    // -----------------------------------------------------------------------

    /// Fetch gdfx free holding statistics data.
    pub async fn fetch_gdfx_free_holding_statistics(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<GdfxHoldingStatistic>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:gdfx-free-stat:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_free_holding_statistics(self, date),
        )
        .await
    }

    /// Fetch gdfx holding statistics data.
    pub async fn fetch_gdfx_holding_statistics(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<GdfxHoldingStatistic>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:gdfx-stat:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_holding_statistics(self, date),
        )
        .await
    }

    /// Fetch gdfx free holding change data.
    pub async fn fetch_gdfx_free_holding_change(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<GdfxHoldingChange>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:gdfx-free-change:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_free_holding_change(self, date),
        )
        .await
    }

    /// Fetch shareholder holding changes.
    pub async fn fetch_gdfx_holding_change(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<GdfxHoldingChange>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:gdfx-change:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_holding_change(self, date),
        )
        .await
    }

    /// Fetch gdfx free top10 data.
    pub async fn fetch_gdfx_free_top10(
        &self,
        symbol: &str,
        date: &str,
    ) -> anyhow::Result<Vec<GdfxTop10>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:gdfx-free-top10:{}:{}",
                symbol.trim(),
                date
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_free_top10(self, symbol, date),
        )
        .await
    }

    /// Fetch top 10 shareholders data.
    pub async fn fetch_gdfx_top10(
        &self,
        symbol: &str,
        date: &str,
    ) -> anyhow::Result<Vec<GdfxTop10>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:gdfx-top10:{}:{}",
                symbol.trim(),
                date
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_top10(self, symbol, date),
        )
        .await
    }

    /// Fetch gdfx free holding detail data.
    pub async fn fetch_gdfx_free_holding_detail(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<GdfxHoldingDetail>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:gdfx-free-detail:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_free_holding_detail(self, date),
        )
        .await
    }

    /// Fetch shareholder holding details.
    pub async fn fetch_gdfx_holding_detail(
        &self,
        date: &str,
        indicator: &str,
        symbol: &str,
    ) -> anyhow::Result<Vec<GdfxHoldingDetail>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:gdfx-detail:{}:{}:{}",
                date, indicator, symbol
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || {
                super::akshare_rust::a_share::fetch_gdfx_holding_detail(
                    self, date, indicator, symbol,
                )
            },
        )
        .await
    }

    /// Fetch gdfx free holding analyse data.
    pub async fn fetch_gdfx_free_holding_analyse(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<GdfxHoldingAnalyse>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:gdfx-free-analyse:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_free_holding_analyse(self, date),
        )
        .await
    }

    /// Fetch gdfx holding analyse data.
    pub async fn fetch_gdfx_holding_analyse(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<GdfxHoldingAnalyse>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:gdfx-analyse:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_holding_analyse(self, date),
        )
        .await
    }

    /// Fetch gdfx free teamwork data.
    pub async fn fetch_gdfx_free_teamwork(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<GdfxTeamwork>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:gdfx-free-team:{}",
                symbol.trim()
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_free_teamwork(self, symbol),
        )
        .await
    }

    /// Fetch shareholder teamwork data.
    pub async fn fetch_gdfx_teamwork(&self, symbol: &str) -> anyhow::Result<Vec<GdfxTeamwork>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:gdfx-team:{}", symbol.trim()),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_gdfx_teamwork(self, symbol),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Block Trades (大宗交易)
    // -----------------------------------------------------------------------

    /// Fetch block trade daily data.
    pub async fn fetch_block_trade_daily(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> anyhow::Result<Vec<DzjyMrtj>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:dzjy-mrtj:{}:{}",
                start_date, end_date
            ),
            INSIDER_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_block_trade_daily(self, start_date, end_date),
        )
        .await
    }

    /// Fetch block trade industry data.
    pub async fn fetch_block_trade_industry(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> anyhow::Result<Vec<DzjyHygtj>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:dzjy-hygtj:{}:{}",
                start_date, end_date
            ),
            INSIDER_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_block_trade_industry(self, start_date, end_date),
        )
        .await
    }

    /// Fetch block trade industry daily data.
    pub async fn fetch_block_trade_industry_daily(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> anyhow::Result<Vec<BlockTradeActiveBranch>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:dzjy-hyyybtj:{}:{}",
                start_date, end_date
            ),
            INSIDER_CACHE_TTL_SECS,
            || {
                super::akshare_rust::a_share::fetch_block_trade_industry_daily(
                    self, start_date, end_date,
                )
            },
        )
        .await
    }

    /// Fetch block trade seat ranking data.
    pub async fn fetch_block_trade_seat_ranking(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> anyhow::Result<Vec<BlockTradeBranchRanking>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:dzjy-yybph:{}:{}",
                start_date, end_date
            ),
            INSIDER_CACHE_TTL_SECS,
            || {
                super::akshare_rust::a_share::fetch_block_trade_seat_ranking(
                    self, start_date, end_date,
                )
            },
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Hot Stocks (雪球热度)
    // -----------------------------------------------------------------------

    /// Fetch hot follow xq data.
    pub async fn fetch_hot_follow_xq(&self, symbol: &str) -> anyhow::Result<Vec<HotStockXq>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:hot-follow:{}", symbol.trim()),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_hot_follow_xq(self, symbol),
        )
        .await
    }

    /// Fetch hot tweet xq data.
    pub async fn fetch_hot_tweet_xq(&self, symbol: &str) -> anyhow::Result<Vec<HotStockXq>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:hot-tweet:{}", symbol.trim()),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_hot_tweet_xq(self, symbol),
        )
        .await
    }

    /// Fetch hot deal xq data.
    pub async fn fetch_hot_deal_xq(&self, symbol: &str) -> anyhow::Result<Vec<HotStockXq>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:hot-deal:{}", symbol.trim()),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_hot_deal_xq(self, symbol),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Order Book Changes (盘口异动)
    // -----------------------------------------------------------------------

    /// Fetch pankou changes data.
    pub async fn fetch_pankou_changes(&self, symbol: &str) -> anyhow::Result<Vec<PankouChange>> {
        super::akshare_rust::a_share::fetch_pankou_changes(self, symbol).await
    }

    // -----------------------------------------------------------------------
    // Dividends (分红送配)
    // -----------------------------------------------------------------------

    /// Fetch dividends data.
    pub async fn fetch_dividends(&self, date: &str) -> anyhow::Result<Vec<DividendInfo>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:dividends:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_dividends(self, date),
        )
        .await
    }

    /// Fetch dividend detail data.
    pub async fn fetch_dividend_detail(&self, symbol: &str) -> anyhow::Result<Vec<DividendInfo>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:dividend-detail:{}",
                symbol.trim()
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_dividend_detail(self, symbol),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Pledge Data (股权质押)
    // -----------------------------------------------------------------------

    /// Fetch pledge profile data.
    pub async fn fetch_pledge_profile(&self) -> anyhow::Result<Vec<GpzyProfile>> {
        let cache_key = format!("{MARKET_DATA_CACHE_PREFIX}:pledge-profile");
        if let Some(cached) = self.cache_get_json(&cache_key).await {
            return Ok(cached);
        }
        let items = super::akshare_rust::a_share::fetch_pledge_profile(self).await?;
        self.cache_set_json(&cache_key, FUNDAMENTALS_CACHE_TTL_SECS, &items)
            .await;
        Ok(items)
    }

    /// Fetch pledge ratio data.
    pub async fn fetch_pledge_ratio(&self) -> anyhow::Result<Vec<GpzyPledgeRatio>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:pledge-ratio"),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_pledge_ratio(self),
        )
        .await
    }

    /// Fetch pledge detail data.
    pub async fn fetch_pledge_detail(&self) -> anyhow::Result<Vec<GpzyPledgeDetail>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:pledge-detail"),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_pledge_detail(self),
        )
        .await
    }

    /// Fetch pledge ratio detail data.
    pub async fn fetch_pledge_ratio_detail(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<GpzyPledgeRatioDetail>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:pledge-ratio-detail:{}",
                symbol.trim()
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_pledge_ratio_detail(self, symbol),
        )
        .await
    }

    /// Fetch pledge distribute bank data.
    pub async fn fetch_pledge_distribute_bank(&self) -> anyhow::Result<Vec<GpzyDistributeEntry>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:pledge-dist-bank"),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_pledge_distribute_bank(self),
        )
        .await
    }

    /// Fetch pledge distribute company data.
    pub async fn fetch_pledge_distribute_company(
        &self,
    ) -> anyhow::Result<Vec<GpzyDistributeEntry>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:pledge-dist-company"),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_pledge_distribute_company(self),
        )
        .await
    }

    /// Fetch pledge industry data.
    pub async fn fetch_pledge_industry(&self) -> anyhow::Result<Vec<GpzyIndustry>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:pledge-industry"),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_pledge_industry(self),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Institutional Research (机构调研)
    // -----------------------------------------------------------------------

    /// Fetch institutional research data.
    pub async fn fetch_institutional_research(&self, date: &str) -> anyhow::Result<Vec<JgdyTj>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:jgdy-tj:{}", date),
            INSIDER_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_institutional_research(self, date),
        )
        .await
    }

    /// Fetch institutional research detail data.
    pub async fn fetch_institutional_research_detail(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<JgdyDetail>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:jgdy-detail:{}", date),
            INSIDER_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_institutional_research_detail(self, date),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // ESG Ratings
    // -----------------------------------------------------------------------

    /// Fetch esg msci data.
    pub async fn fetch_esg_msci(&self) -> anyhow::Result<Vec<EsgRating>> {
        let cache_key = format!("{MARKET_DATA_CACHE_PREFIX}:esg-msci");
        if let Some(cached) = self.cache_get_json(&cache_key).await {
            return Ok(cached);
        }
        let items = super::akshare_rust::a_share::fetch_esg_msci(self).await?;
        self.cache_set_json(&cache_key, ESG_CACHE_TTL_SECS, &items)
            .await;
        Ok(items)
    }

    /// Fetch esg rft data.
    pub async fn fetch_esg_rft(&self) -> anyhow::Result<Vec<EsgRating>> {
        let cache_key = format!("{MARKET_DATA_CACHE_PREFIX}:esg-rft");
        if let Some(cached) = self.cache_get_json(&cache_key).await {
            return Ok(cached);
        }
        let items = super::akshare_rust::a_share::fetch_esg_rft(self).await?;
        self.cache_set_json(&cache_key, ESG_CACHE_TTL_SECS, &items)
            .await;
        Ok(items)
    }

    /// Fetch esg zd data.
    pub async fn fetch_esg_zd(&self) -> anyhow::Result<Vec<EsgRating>> {
        let cache_key = format!("{MARKET_DATA_CACHE_PREFIX}:esg-zd");
        if let Some(cached) = self.cache_get_json(&cache_key).await {
            return Ok(cached);
        }
        let items = super::akshare_rust::a_share::fetch_esg_zd(self).await?;
        self.cache_set_json(&cache_key, ESG_CACHE_TTL_SECS, &items)
            .await;
        Ok(items)
    }

    /// Fetch esg hz data.
    pub async fn fetch_esg_hz(&self) -> anyhow::Result<Vec<EsgRating>> {
        let cache_key = format!("{MARKET_DATA_CACHE_PREFIX}:esg-hz");
        if let Some(cached) = self.cache_get_json(&cache_key).await {
            return Ok(cached);
        }
        let items = super::akshare_rust::a_share::fetch_esg_hz(self).await?;
        self.cache_set_json(&cache_key, ESG_CACHE_TTL_SECS, &items)
            .await;
        Ok(items)
    }

    // -----------------------------------------------------------------------
    // Financial Reports (三大报表)
    // -----------------------------------------------------------------------

    /// Fetch balance sheet data.
    pub async fn fetch_balance_sheet(&self, date: &str) -> anyhow::Result<Vec<BalanceSheet>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:balance-sheet:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_balance_sheet(self, date),
        )
        .await
    }

    /// Fetch profit/income statement data.
    pub async fn fetch_profit_sheet(&self, date: &str) -> anyhow::Result<Vec<ProfitSheet>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:profit-sheet:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_profit_sheet(self, date),
        )
        .await
    }

    /// Fetch cash flow statement data.
    pub async fn fetch_cash_flow_sheet(&self, date: &str) -> anyhow::Result<Vec<CashFlowSheet>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:cash-flow-sheet:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_cash_flow_sheet(self, date),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Stock Comments (千股千评)
    // -----------------------------------------------------------------------

    /// Fetch stock comments data.
    pub async fn fetch_stock_comments(&self) -> anyhow::Result<Vec<StockComment>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:stock-comments"),
            INSIDER_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_stock_comments(self),
        )
        .await
    }

    /// Fetch institutional participation in comments.
    pub async fn fetch_comment_org_participation(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<CommentOrgParticipation>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:comment-org:{}", symbol.trim()),
            INSIDER_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_comment_org_participation(self, symbol),
        )
        .await
    }

    /// Fetch historical comment scores.
    pub async fn fetch_comment_hist_score(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<CommentHistScore>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:comment-hist:{}", symbol.trim()),
            INSIDER_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_comment_hist_score(self, symbol),
        )
        .await
    }

    /// Fetch comment focus index data.
    pub async fn fetch_comment_focus_index(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<CommentFocusIndex>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:comment-focus:{}", symbol.trim()),
            INSIDER_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_comment_focus_index(self, symbol),
        )
        .await
    }

    /// Fetch comment desire index data.
    pub async fn fetch_comment_desire_index(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<CommentDesireIndex>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:comment-desire:{}",
                symbol.trim()
            ),
            INSIDER_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_comment_desire_index(self, symbol),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Shareholder Changes (高管持股变动)
    // -----------------------------------------------------------------------

    /// Fetch executive shareholding data.
    pub async fn fetch_executive_shareholding(&self, symbol: &str) -> anyhow::Result<Vec<Ggcg>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:exec-shareholding:{}",
                symbol.trim()
            ),
            INSIDER_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_executive_shareholding(self, symbol),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Shareholder Count (股东户数)
    // -----------------------------------------------------------------------

    /// Fetch shareholder count data.
    pub async fn fetch_shareholder_count(&self, date: &str) -> anyhow::Result<Vec<Gdhs>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:gdhs:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_shareholder_count(self, date),
        )
        .await
    }

    /// Fetch shareholder count detail data.
    pub async fn fetch_shareholder_count_detail(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<GdhsDetail>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:gdhs-detail:{}", symbol.trim()),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::a_share::fetch_shareholder_count_detail(self, symbol),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Industry Classification (行业分类)
    // -----------------------------------------------------------------------

    /// Fetch industry classification data.
    pub async fn fetch_industry_category(&self) -> anyhow::Result<Vec<IndustryCategory>> {
        let cache_key = format!("{MARKET_DATA_CACHE_PREFIX}:industry-category");
        if let Some(cached) = self.cache_get_json(&cache_key).await {
            return Ok(cached);
        }
        let items = super::akshare_rust::a_share::fetch_industry_category(self).await?;
        self.cache_set_json(&cache_key, INDUSTRY_CACHE_TTL_SECS, &items)
            .await;
        Ok(items)
    }
}

impl MarketDataClient {
    // -----------------------------------------------------------------------
    // HK Spot (Sina)
    // -----------------------------------------------------------------------

    /// Fetch hk spot data.
    pub async fn fetch_hk_spot(&self) -> anyhow::Result<Vec<HkSpotQuote>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:hk-spot"),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_spot(self),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // HK Famous Stocks (Eastmoney)
    // -----------------------------------------------------------------------

    /// Fetch hk famous spot data.
    pub async fn fetch_hk_famous_spot(&self) -> anyhow::Result<Vec<HkFamousStock>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:hk-famous-spot"),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_famous_spot(self),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // HK Hot Rank (Eastmoney)
    // -----------------------------------------------------------------------

    /// Fetch HK stock hot ranking.
    pub async fn fetch_hk_hot_rank(&self) -> anyhow::Result<Vec<HkHotRank>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:hk-hot-rank"),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_hot_rank(self),
        )
        .await
    }

    /// Fetch hk hot rank latest data.
    pub async fn fetch_hk_hot_rank_latest(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<HkHotRankDetail>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:hk-hot-rank-latest:{}",
                symbol.trim()
            ),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_hot_rank_latest(self, symbol),
        )
        .await
    }

    /// Fetch HK stock hot ranking details.
    pub async fn fetch_hk_hot_rank_detail(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<HkHotRankDetail>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:hk-hot-rank-detail:{}",
                symbol.trim()
            ),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_hot_rank_detail(self, symbol),
        )
        .await
    }

    /// Fetch hk hot rank realtime data.
    pub async fn fetch_hk_hot_rank_realtime(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<HkHotRankDetail>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:hk-hot-rank-rt:{}",
                symbol.trim()
            ),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_hot_rank_realtime(self, symbol),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // HK Dividends (Eastmoney + THS)
    // -----------------------------------------------------------------------

    /// Fetch hk dividend payout data.
    pub async fn fetch_hk_dividend_payout(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:hk-dividend-payout:{}",
                symbol.trim()
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_dividend_payout(self, symbol),
        )
        .await
    }

    /// Fetch hk fhpx detail data.
    pub async fn fetch_hk_fhpx_detail(&self, symbol: &str) -> anyhow::Result<Vec<HkFhpxDetailThs>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:hk-fhpx-detail:{}",
                symbol.trim()
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_fhpx_detail(self, symbol),
        )
        .await
    }

    /// Fetch hk dividend yield data.
    pub async fn fetch_hk_dividend_yield(&self) -> anyhow::Result<Vec<HkGxlLg>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:hk-dividend-yield"),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_dividend_yield(self),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // HK Financial Indicators (Eastmoney)
    // -----------------------------------------------------------------------

    /// Fetch hk financial indicators data.
    pub async fn fetch_hk_financial_indicators(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:hk-fin-indicators:{}",
                symbol.trim()
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_financial_indicators(self, symbol),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // HK Valuation (Baidu)
    // -----------------------------------------------------------------------

    /// Fetch HK stock valuation data.
    pub async fn fetch_hk_valuation(
        &self,
        symbol: &str,
        indicator: &str,
        period: &str,
    ) -> anyhow::Result<Vec<HkValuationBaidu>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:hk-valuation:{}:{}:{}",
                symbol.trim(),
                indicator,
                period
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::hk::fetch_hk_valuation(self, symbol, indicator, period),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // US Spot (Sina / Eastmoney)
    // -----------------------------------------------------------------------

    /// Fetch us spot data.
    pub async fn fetch_us_spot(&self) -> anyhow::Result<Vec<UsSpotSina>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:us-spot"),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::us::fetch_us_spot(self),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // US Famous Stocks (Eastmoney)
    // -----------------------------------------------------------------------

    /// Fetch us famous spot data.
    pub async fn fetch_us_famous_spot(&self, category: &str) -> anyhow::Result<Vec<UsFamousStock>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:us-famous-spot:{}",
                category.trim()
            ),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::us::fetch_us_famous_spot(self, category),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // US Pink Sheet Stocks (Eastmoney)
    // -----------------------------------------------------------------------

    /// Fetch us pink spot data.
    pub async fn fetch_us_pink_spot(&self) -> anyhow::Result<Vec<UsPinkStock>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:us-pink-spot"),
            CANDLES_CACHE_TTL_SECS,
            || super::akshare_rust::us::fetch_us_pink_spot(self),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // US Valuation (Baidu)
    // -----------------------------------------------------------------------

    /// Fetch US stock valuation data.
    pub async fn fetch_us_valuation(
        &self,
        symbol: &str,
        indicator: &str,
        period: &str,
    ) -> anyhow::Result<Vec<UsValuationBaidu>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:us-valuation:{}:{}:{}",
                symbol.trim(),
                indicator,
                period
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::akshare_rust::us::fetch_us_valuation(self, symbol, indicator, period),
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Xueqiu Spot (works for HK and US symbols)
    // -----------------------------------------------------------------------

    /// Fetch xq spot data.
    pub async fn fetch_xq_spot(&self, symbol: &str) -> anyhow::Result<XqStockSpot> {
        let cache_key = format!("{MARKET_DATA_CACHE_PREFIX}:xq-spot:{}", symbol.trim());
        if let Some(cached) = self.cache_get_json(&cache_key).await {
            return Ok(cached);
        }
        // Try HK first, then US -- both delegate to the same Xueqiu endpoint
        let market = self.detect_market(symbol);
        let items = match market {
            crate::MarketKind::HongKong => {
                super::akshare_rust::hk::fetch_xq_spot(self, symbol).await?
            }
            _ => super::akshare_rust::us::fetch_xq_spot(self, symbol).await?,
        };
        self.cache_set_json(&cache_key, CANDLES_CACHE_TTL_SECS, &items)
            .await;
        Ok(items)
    }

    // -----------------------------------------------------------------------
    // US Fundamentals Fallback (Yahoo Finance + Finnhub)
    // -----------------------------------------------------------------------

    /// Fetch US equity fundamentals from Yahoo Finance quoteSummary API.
    pub async fn fetch_us_fundamentals_yahoo(
        &self,
        symbol: &str,
    ) -> anyhow::Result<FundamentalsSnapshot> {
        let url = format!(
            "https://query1.finance.yahoo.com/v10/finance/quoteSummary/{}?modules=financialData,defaultKeyStatistics",
            symbol
        );
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .context("Yahoo Finance request failed")?;
        if !resp.status().is_success() {
            anyhow::bail!("Yahoo Finance returned status {}", resp.status());
        }
        let body: YahooQuoteSummaryResponse = resp
            .json()
            .await
            .context("failed to parse Yahoo Finance response")?;
        let result = body
            .quote_summary
            .result
            .into_iter()
            .next()
            .context("Yahoo Finance returned no results")?;
        let financial = &result.financial_data;
        let stats = &result.default_key_statistics;

        Ok(FundamentalsSnapshot {
            symbol: symbol.to_string(),
            company_name: String::new(),
            cik: String::new(),
            industry: None,
            currency: financial
                .financial_currency
                .clone()
                .unwrap_or_else(|| "USD".to_string()),
            fiscal_year_end: None,
            shares_outstanding: stats.shares_outstanding.map(|v| v as i64),
            market_cap: financial.market_cap.as_ref().and_then(|v| v.raw),
            net_income_usd: financial.net_income_to_common.as_ref().and_then(|v| v.raw),
            revenues_usd: financial.total_revenue.as_ref().and_then(|v| v.raw),
            assets_usd: financial.total_assets.as_ref().and_then(|v| v.raw),
            liabilities_usd: financial.total_liabilities.as_ref().and_then(|v| v.raw),
            stockholders_equity_usd: financial
                .total_stockholder_equity
                .as_ref()
                .and_then(|v| v.raw),
            cash_and_equivalents_usd: None,
            gross_profit_usd: financial.gross_profit.as_ref().and_then(|v| v.raw),
            operating_income_usd: financial.operating_income.as_ref().and_then(|v| v.raw),
            operating_expenses_usd: None,
            operating_cash_flow_usd: financial.operating_cashflow.as_ref().and_then(|v| v.raw),
            capital_expenditure_usd: None,
            free_cash_flow_usd: financial.free_cashflow.as_ref().and_then(|v| v.raw),
            long_term_debt_usd: None,
            current_debt_usd: None,
            total_debt_usd: None,
            diluted_shares_outstanding: None,
        })
    }

    /// Fetch US equity fundamentals from Finnhub (financials-reported + metrics + profile).
    /// Reads API keys from `FALLBACK_FINNHUB_API_KEYS` env var (comma-separated).
    pub async fn fetch_us_fundamentals_finnhub(
        &self,
        symbol: &str,
    ) -> anyhow::Result<FundamentalsSnapshot> {
        let api_keys = Self::finnhub_api_keys();
        if api_keys.is_empty() {
            anyhow::bail!("no Finnhub API keys configured (FALLBACK_FINNHUB_API_KEYS)");
        }

        // Fetch financials-reported and metrics concurrently
        let (financials, metrics, profile) = tokio::join!(
            self.fetch_finnhub_financials(symbol, &api_keys),
            self.fetch_finnhub_metrics(symbol, &api_keys),
            self.fetch_finnhub_profile(symbol, &api_keys),
        );

        let mut result = match (financials, metrics) {
            (Some(mut fin), Some(met)) => {
                fin.market_cap = met.market_cap;
                if met.net_income_usd.is_some() {
                    fin.net_income_usd = met.net_income_usd;
                }
                if met.revenues_usd.is_some() {
                    fin.revenues_usd = met.revenues_usd;
                }
                if met.gross_profit_usd.is_some() {
                    fin.gross_profit_usd = met.gross_profit_usd;
                }
                if met.operating_income_usd.is_some() {
                    fin.operating_income_usd = met.operating_income_usd;
                }
                if met.stockholders_equity_usd.is_some() {
                    fin.stockholders_equity_usd = met.stockholders_equity_usd;
                }
                if met.total_debt_usd.is_some() {
                    fin.total_debt_usd = met.total_debt_usd;
                }
                fin
            }
            (Some(fin), None) => fin,
            (None, Some(met)) => met,
            (None, None) => anyhow::bail!("all Finnhub fundamentals sources failed"),
        };

        if let Some(prof) = profile {
            if result.company_name.is_empty() || result.company_name == result.cik {
                result.company_name = prof.name;
            }
            if result.industry.is_none() {
                result.industry = Some(prof.industry);
            }
            if result.shares_outstanding.is_none() {
                result.shares_outstanding = prof.shares_outstanding;
            }
        }

        Ok(result)
    }

    /// Fetch US company news from Finnhub.
    /// Reads API keys from `FALLBACK_FINNHUB_API_KEYS` env var (comma-separated).
    pub async fn fetch_us_news_finnhub(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
    ) -> anyhow::Result<Vec<NewsItem>> {
        let api_keys = Self::finnhub_api_keys();
        if api_keys.is_empty() {
            anyhow::bail!("no Finnhub API keys configured");
        }
        for api_key in &api_keys {
            let url = format!(
                "https://finnhub.io/api/v1/company-news?symbol={}&from={}&to={}&token={}",
                symbol, from, to, api_key
            );
            let resp = match self.http.get(&url).send().await {
                Ok(r) => r,
                Err(e) => {
                    tracing::debug!(symbol, error = %e, "Finnhub news request error");
                    continue;
                }
            };
            let status = resp.status();
            if status.as_u16() == 429 || status.as_u16() == 401 {
                tracing::debug!(
                    symbol,
                    status = %status,
                    "Finnhub news rate limited, trying next key"
                );
                continue;
            }
            if !status.is_success() {
                anyhow::bail!("Finnhub news returned status {}", status);
            }
            let articles: Vec<serde_json::Value> = resp
                .json()
                .await
                .context("failed to parse Finnhub news response")?;
            let items: Vec<NewsItem> = articles
                .into_iter()
                .filter_map(|item| {
                    let headline = item.get("headline")?.as_str()?;
                    if headline.is_empty() {
                        return None;
                    }
                    Some(NewsItem {
                        published_at: item
                            .get("datetime")
                            .and_then(|v| v.as_i64())
                            .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
                            .map(|dt| dt.format("%Y-%m-%d").to_string())
                            .unwrap_or_default(),
                        title: headline.to_string(),
                        summary: item
                            .get("summary")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        source: item
                            .get("source")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Finnhub")
                            .to_string(),
                        url: item
                            .get("url")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                    })
                })
                .collect();
            return Ok(items);
        }
        anyhow::bail!("all Finnhub news API keys failed")
    }

    // ── Finnhub helpers ──────────────────────────────────────────────────

    fn finnhub_api_keys() -> Vec<String> {
        std::env::var("FALLBACK_FINNHUB_API_KEYS")
            .ok()
            .map(|s| {
                s.split(',')
                    .map(|k| k.trim().to_string())
                    .filter(|k| !k.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }

    async fn fetch_finnhub_financials(
        &self,
        symbol: &str,
        api_keys: &[String],
    ) -> Option<FundamentalsSnapshot> {
        for api_key in api_keys {
            let url = format!(
                "https://finnhub.io/api/v1/stock/financials-reported?symbol={}&token={}",
                symbol, api_key
            );
            let resp = match self.http.get(&url).send().await {
                Ok(r) => r,
                Err(e) => {
                    tracing::debug!(symbol, error = %e, "Finnhub financials-reported error");
                    continue;
                }
            };
            let status = resp.status();
            if status.as_u16() == 429 || status.as_u16() == 401 {
                continue;
            }
            if !status.is_success() {
                return None;
            }
            let body: FinnhubFinancialsReportedResponse = resp.json().await.ok()?;
            let report = body.data.first()?.report.as_ref()?;

            let assets_usd = find_finnhub_value(&report.bs, "us-gaap_Assets");
            let liabilities_usd = find_finnhub_value(&report.bs, "us-gaap_Liabilities");
            let stockholders_equity_usd =
                find_finnhub_value(&report.bs, "us-gaap_StockholdersEquity");
            let cash_and_equivalents_usd =
                find_finnhub_value(&report.bs, "us-gaap_CashAndCashEquivalentsAtCarryingValue");
            let long_term_debt_usd =
                find_finnhub_value(&report.bs, "us-gaap_LongTermDebtNoncurrent");
            let current_debt_usd = find_finnhub_value(&report.bs, "us-gaap_LongTermDebtCurrent");
            let total_debt_usd = match (long_term_debt_usd, current_debt_usd) {
                (Some(lt), Some(ct)) => Some(lt + ct),
                (Some(lt), None) => Some(lt),
                (None, Some(ct)) => Some(ct),
                _ => None,
            };
            let operating_cash_flow_usd = find_finnhub_value(
                &report.cf,
                "us-gaap_NetCashProvidedByUsedInOperatingActivities",
            );
            let capital_expenditure_usd = find_finnhub_value(
                &report.cf,
                "us-gaap_PaymentsToAcquirePropertyPlantAndEquipment",
            );
            let free_cash_flow_usd = match (operating_cash_flow_usd, capital_expenditure_usd) {
                (Some(ocf), Some(capex)) => Some(ocf - capex),
                _ => None,
            };
            let revenues_usd = find_finnhub_value(
                &report.ic,
                "us-gaap_RevenueFromContractWithCustomerExcludingAssessedTax",
            );
            let gross_profit_usd = find_finnhub_value(&report.ic, "us-gaap_GrossProfit");
            let operating_income_usd =
                find_finnhub_value(&report.ic, "us-gaap_OperatingIncomeLoss");
            let net_income_usd = find_finnhub_value(&report.ic, "us-gaap_NetIncomeLoss");

            return Some(FundamentalsSnapshot {
                symbol: symbol.to_string(),
                company_name: body.cik.clone().unwrap_or_default(),
                cik: body.cik.unwrap_or_default(),
                industry: None,
                currency: "USD".to_string(),
                fiscal_year_end: None,
                shares_outstanding: None,
                market_cap: None,
                net_income_usd,
                revenues_usd,
                assets_usd,
                liabilities_usd,
                stockholders_equity_usd,
                cash_and_equivalents_usd,
                gross_profit_usd,
                operating_income_usd,
                operating_expenses_usd: None,
                operating_cash_flow_usd,
                capital_expenditure_usd,
                free_cash_flow_usd,
                long_term_debt_usd,
                current_debt_usd,
                total_debt_usd,
                diluted_shares_outstanding: None,
            });
        }
        None
    }

    async fn fetch_finnhub_metrics(
        &self,
        symbol: &str,
        api_keys: &[String],
    ) -> Option<FundamentalsSnapshot> {
        for api_key in api_keys {
            let url = format!(
                "https://finnhub.io/api/v1/stock/metric?symbol={}&metric=all&token={}",
                symbol, api_key
            );
            let resp = match self.http.get(&url).send().await {
                Ok(r) => r,
                Err(e) => {
                    tracing::debug!(symbol, error = %e, "Finnhub metrics error");
                    continue;
                }
            };
            let status = resp.status();
            if status.as_u16() == 429 || status.as_u16() == 401 {
                continue;
            }
            if !status.is_success() {
                return None;
            }
            let body: FinnhubMetricResponse = resp.json().await.ok()?;
            let m = &body.metric;

            let market_cap = m.market_cap;
            let net_income_usd = match (market_cap, m.pe_ttm) {
                (Some(mc), Some(pe)) if pe > 0.0 => Some(mc / pe),
                _ => None,
            };
            let revenues_usd = match (market_cap, m.ps_ttm) {
                (Some(mc), Some(ps)) if ps > 0.0 => Some(mc / ps),
                _ => None,
            };
            let gross_profit_usd = match (revenues_usd, m.gross_margin_ttm) {
                (Some(rev), Some(margin)) => Some(rev * margin / 100.0),
                _ => None,
            };
            let operating_income_usd = match (revenues_usd, m.operating_margin_ttm) {
                (Some(rev), Some(margin)) => Some(rev * margin / 100.0),
                _ => None,
            };
            let stockholders_equity_usd = match (market_cap, m.pb) {
                (Some(mc), Some(pb)) if pb > 0.0 => Some(mc / pb),
                _ => None,
            };
            let total_debt_usd =
                match (stockholders_equity_usd, m.total_debt_total_equity_quarterly) {
                    (Some(eq), Some(ratio)) => Some(eq * ratio),
                    _ => None,
                };

            return Some(FundamentalsSnapshot {
                symbol: symbol.to_string(),
                company_name: String::new(),
                cik: String::new(),
                industry: None,
                currency: "USD".to_string(),
                fiscal_year_end: None,
                shares_outstanding: None,
                market_cap,
                net_income_usd,
                revenues_usd,
                assets_usd: None,
                liabilities_usd: None,
                stockholders_equity_usd,
                cash_and_equivalents_usd: None,
                gross_profit_usd,
                operating_income_usd,
                operating_expenses_usd: None,
                operating_cash_flow_usd: None,
                capital_expenditure_usd: None,
                free_cash_flow_usd: None,
                long_term_debt_usd: None,
                current_debt_usd: None,
                total_debt_usd,
                diluted_shares_outstanding: None,
            });
        }
        None
    }

    async fn fetch_finnhub_profile(
        &self,
        symbol: &str,
        api_keys: &[String],
    ) -> Option<FinnhubProfileData> {
        for api_key in api_keys {
            let url = format!(
                "https://finnhub.io/api/v1/stock/profile2?symbol={}&token={}",
                symbol, api_key
            );
            let resp = match self.http.get(&url).send().await {
                Ok(r) => r,
                Err(e) => {
                    tracing::debug!(symbol, error = %e, "Finnhub profile error");
                    continue;
                }
            };
            let status = resp.status();
            if status.as_u16() == 429 || status.as_u16() == 401 {
                continue;
            }
            if !status.is_success() {
                return None;
            }
            let profile: FinnhubProfileResponse = resp.json().await.ok()?;
            return Some(FinnhubProfileData {
                name: profile.name.unwrap_or_default(),
                industry: profile.industry.unwrap_or_default(),
                shares_outstanding: profile
                    .share_outstanding
                    .map(|v| (v * 1_000_000.0).round() as i64),
            });
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Yahoo Finance response types
// ---------------------------------------------------------------------------

#[derive(serde::Deserialize)]
struct YahooQuoteSummaryResponse {
    #[serde(rename = "quoteSummary")]
    quote_summary: YahooQuoteSummary,
}

#[derive(serde::Deserialize)]
struct YahooQuoteSummary {
    result: Vec<YahooQuoteSummaryResult>,
}

#[derive(serde::Deserialize)]
struct YahooQuoteSummaryResult {
    #[serde(rename = "financialData")]
    financial_data: YahooFinancialData,
    #[serde(rename = "defaultKeyStatistics")]
    default_key_statistics: YahooDefaultKeyStatistics,
}

#[derive(serde::Deserialize)]
struct YahooFinancialData {
    #[serde(rename = "financialCurrency")]
    financial_currency: Option<String>,
    #[serde(rename = "marketCap")]
    market_cap: Option<YahooRawValue>,
    #[serde(rename = "netIncomeToCommon")]
    net_income_to_common: Option<YahooRawValue>,
    #[serde(rename = "totalRevenue")]
    total_revenue: Option<YahooRawValue>,
    #[serde(rename = "totalAssets")]
    total_assets: Option<YahooRawValue>,
    #[serde(rename = "totalLiabilities")]
    total_liabilities: Option<YahooRawValue>,
    #[serde(rename = "totalStockholderEquity")]
    total_stockholder_equity: Option<YahooRawValue>,
    #[serde(rename = "grossProfit")]
    gross_profit: Option<YahooRawValue>,
    #[serde(rename = "operatingIncome")]
    operating_income: Option<YahooRawValue>,
    #[serde(rename = "operatingCashflow")]
    operating_cashflow: Option<YahooRawValue>,
    #[serde(rename = "freeCashflow")]
    free_cashflow: Option<YahooRawValue>,
}

#[derive(serde::Deserialize)]
struct YahooDefaultKeyStatistics {
    #[serde(rename = "sharesOutstanding")]
    shares_outstanding: Option<f64>,
}

#[derive(serde::Deserialize)]
struct YahooRawValue {
    raw: Option<f64>,
}

// ---------------------------------------------------------------------------
// Finnhub response types
// ---------------------------------------------------------------------------

#[derive(serde::Deserialize)]
struct FinnhubMetricResponse {
    metric: FinnhubMetric,
}

#[derive(serde::Deserialize)]
struct FinnhubMetric {
    #[serde(rename = "marketCapitalization")]
    market_cap: Option<f64>,
    #[serde(rename = "peTTM")]
    pe_ttm: Option<f64>,
    #[serde(rename = "psTTM")]
    ps_ttm: Option<f64>,
    #[serde(rename = "grossMarginTTM")]
    gross_margin_ttm: Option<f64>,
    #[serde(rename = "operatingMarginTTM")]
    operating_margin_ttm: Option<f64>,
    #[serde(rename = "pb")]
    pb: Option<f64>,
    #[serde(rename = "totalDebt/totalEquityQuarterly")]
    total_debt_total_equity_quarterly: Option<f64>,
}

#[derive(serde::Deserialize)]
struct FinnhubFinancialsReportedResponse {
    cik: Option<String>,
    data: Vec<FinnhubFinancialsReportedData>,
}

#[derive(serde::Deserialize)]
struct FinnhubFinancialsReportedData {
    report: Option<FinnhubFinancialsReport>,
}

#[derive(serde::Deserialize)]
struct FinnhubFinancialsReport {
    bs: Option<Vec<FinnhubReportEntry>>,
    cf: Option<Vec<FinnhubReportEntry>>,
    ic: Option<Vec<FinnhubReportEntry>>,
}

#[derive(serde::Deserialize)]
struct FinnhubReportEntry {
    concept: String,
    value: Option<f64>,
}

#[derive(serde::Deserialize)]
struct FinnhubProfileResponse {
    name: Option<String>,
    #[serde(rename = "finnhubIndustry")]
    industry: Option<String>,
    #[serde(rename = "shareOutstanding")]
    share_outstanding: Option<f64>,
}

struct FinnhubProfileData {
    name: String,
    industry: String,
    shares_outstanding: Option<i64>,
}

fn find_finnhub_value(entries: &Option<Vec<FinnhubReportEntry>>, concept: &str) -> Option<f64> {
    entries
        .as_ref()?
        .iter()
        .find(|e| e.concept == concept)?
        .value
}
