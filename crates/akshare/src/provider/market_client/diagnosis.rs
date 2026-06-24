use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

use super::{CandlePoint, MarketDataClient, MarketKind, QuoteSnapshot};

/// Records a single provider attempt during a data fetch with rotation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataFetchAttempt {
    pub provider: String,
    pub success: bool,
    pub error: Option<String>,
    pub duration_ms: u64,
}

/// Full diagnosis of a data fetch operation across multiple providers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataFetchDiagnosis {
    pub data_type: String,
    pub symbol: String,
    pub attempts: Vec<DataFetchAttempt>,
    pub final_status: String, // "success", "degraded", "failed"
    pub used_stale_cache: bool,
}

impl DataFetchDiagnosis {
    pub fn new(data_type: &str, symbol: &str) -> Self {
        Self {
            data_type: data_type.to_string(),
            symbol: symbol.to_string(),
            attempts: Vec::new(),
            final_status: "failed".to_string(),
            used_stale_cache: false,
        }
    }

    /// Returns a summary string for logging.
    pub fn summary(&self) -> String {
        let providers: Vec<String> = self
            .attempts
            .iter()
            .map(|a| {
                if a.success {
                    format!("{}:ok({}ms)", a.provider, a.duration_ms)
                } else {
                    format!("{}:err({}ms)", a.provider, a.duration_ms)
                }
            })
            .collect();
        format!(
            "[{}:{}:{}] {}",
            self.data_type,
            self.symbol,
            self.final_status,
            providers.join(" -> ")
        )
    }
}

type FetcherFn<T> =
    Box<dyn Fn() -> Pin<Box<dyn Future<Output = anyhow::Result<T>> + Send>> + Send + Sync>;

/// A named provider that can attempt to fetch data.
pub struct NamedProvider<T> {
    pub name: String,
    pub fetcher: FetcherFn<T>,
}

impl<T> NamedProvider<T> {
    pub fn new<F, Fut>(name: impl Into<String>, fetcher: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = anyhow::Result<T>> + Send + 'static,
    {
        let name = name.into();
        Self {
            name,
            fetcher: Box::new(move || Box::pin(fetcher())),
        }
    }
}

impl super::MarketDataClient {
    /// Check if fetched data is effectively empty (empty vec, empty string, etc.)
    fn is_data_empty<T: serde::Serialize>(data: &T) -> bool {
        match serde_json::to_value(data) {
            Ok(serde_json::Value::Array(arr)) => arr.is_empty(),
            Ok(serde_json::Value::String(s)) => s.is_empty(),
            Ok(serde_json::Value::Null) => true,
            _ => false,
        }
    }

    /// Try fetching data from multiple providers in order.
    /// Returns the first successful result, or the last failure.
    /// On success, caches the result. On all-failure, tries stale cache.
    pub async fn fetch_with_rotation<T>(
        &self,
        symbol: &str,
        data_type: &str,
        providers: &[NamedProvider<T>],
        cache_key: &str,
        cache_ttl_secs: u64,
    ) -> (Option<T>, DataFetchDiagnosis)
    where
        T: serde::Serialize + serde::de::DeserializeOwned + Clone,
    {
        let mut diagnosis = DataFetchDiagnosis::new(data_type, symbol);

        // First, check fresh cache
        if let Some(cached) = self.cache_get_json::<T>(cache_key).await {
            diagnosis.final_status = "success".to_string();
            diagnosis.used_stale_cache = false;
            diagnosis.attempts.push(DataFetchAttempt {
                provider: "redis_cache".to_string(),
                success: true,
                error: None,
                duration_ms: 0,
            });
            return (Some(cached), diagnosis);
        }

        // Try each provider in order
        for provider in providers {
            let start = std::time::Instant::now();
            let result = (provider.fetcher)().await;
            let duration_ms = start.elapsed().as_millis() as u64;

            match result {
                Ok(data) => {
                    // Check for empty collections — treat as failure to try next provider
                    if Self::is_data_empty(&data) {
                        diagnosis.attempts.push(DataFetchAttempt {
                            provider: provider.name.clone(),
                            success: false,
                            error: Some("provider returned empty data".to_string()),
                            duration_ms,
                        });
                        tracing::warn!(
                            provider = %provider.name,
                            symbol = %symbol,
                            data_type = %data_type,
                            "provider returned empty data, trying next"
                        );
                        continue;
                    }
                    diagnosis.attempts.push(DataFetchAttempt {
                        provider: provider.name.clone(),
                        success: true,
                        error: None,
                        duration_ms,
                    });
                    diagnosis.final_status = "success".to_string();
                    // Cache the successful result
                    self.cache_set_json(cache_key, cache_ttl_secs, &data).await;
                    return (Some(data), diagnosis);
                }
                Err(e) => {
                    diagnosis.attempts.push(DataFetchAttempt {
                        provider: provider.name.clone(),
                        success: false,
                        error: Some(e.to_string()),
                        duration_ms,
                    });
                    tracing::warn!(
                        provider = %provider.name,
                        symbol = %symbol,
                        data_type = %data_type,
                        error = %e,
                        "data fetch failed, trying next provider"
                    );
                }
            }
        }

        // All providers failed — try stale cache
        let stale_key = self.stale_cache_key(cache_key);
        if let Some(cached) = self.cache_get_json::<T>(&stale_key).await {
            diagnosis.final_status = "degraded".to_string();
            diagnosis.used_stale_cache = true;
            diagnosis.attempts.push(DataFetchAttempt {
                provider: "stale_cache".to_string(),
                success: true,
                error: None,
                duration_ms: 0,
            });
            tracing::info!(
                symbol = %symbol,
                data_type = %data_type,
                "all providers failed, using stale cache"
            );
            return (Some(cached), diagnosis);
        }

        diagnosis.final_status = "failed".to_string();
        (None, diagnosis)
    }
}

// ============================================================
// Provider chain definitions
// ============================================================

impl MarketDataClient {
    /// Fetch quote with provider rotation and diagnosis.
    /// A-share: tencent -> tushare | HK: tencent -> yahoo | US: sina -> eastmoney -> yahoo -> stooq
    pub async fn fetch_quote_with_rotation(
        &self,
        symbol: &str,
    ) -> (Option<QuoteSnapshot>, DataFetchDiagnosis) {
        let market = self.detect_market(symbol);
        let normalized = self.cache_symbol(symbol, market);
        let cache_key = format!(
            "{}:quote:{}:{}",
            super::MARKET_DATA_CACHE_PREFIX,
            super::QUOTE_CACHE_VERSION,
            normalized
        );
        match market {
            MarketKind::AShare => {
                let ts_code = self.normalize_a_share_symbol(symbol).unwrap_or_default();
                let symbol_owned = symbol.to_string();
                let ts_owned = ts_code.clone();
                let providers: Vec<NamedProvider<QuoteSnapshot>> = vec![
                    NamedProvider::new("tencent_quote", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move { client.fetch_a_share_quote_from_eastmoney(&sym).await }
                        }
                    }),
                    NamedProvider::new("tushare_daily", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        let ts = ts_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            let ts = ts.clone();
                            async move { client.fetch_a_share_quote_from_tushare(&sym, &ts).await }
                        }
                    }),
                ];
                self.fetch_with_rotation(
                    symbol,
                    "quote",
                    &providers,
                    &cache_key,
                    super::QUOTE_CACHE_TTL_SECS,
                )
                .await
            }
            MarketKind::HongKong => {
                let symbol_owned = symbol.to_string();
                let providers: Vec<NamedProvider<QuoteSnapshot>> = vec![
                    NamedProvider::new("tencent_quote", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move {
                                let code = client.hk_standard_code(&sym)?;
                                client.fetch_hk_tencent_quote(&sym, &code).await
                            }
                        }
                    }),
                    NamedProvider::new("yahoo_finance_chart", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move { client.fetch_hk_yahoo_quote(&sym).await }
                        }
                    }),
                ];
                self.fetch_with_rotation(
                    symbol,
                    "quote",
                    &providers,
                    &cache_key,
                    super::QUOTE_CACHE_TTL_SECS,
                )
                .await
            }
            MarketKind::UsEquity => {
                let symbol_owned = symbol.to_string();
                let providers: Vec<NamedProvider<QuoteSnapshot>> = vec![
                    NamedProvider::new("sina_us_daily", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move { super::akshare_rust::us_sina::fetch_quote(&client, &sym).await }
                        }
                    }),
                    NamedProvider::new("eastmoney_quote", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move { client.fetch_us_quote_from_eastmoney(&sym).await }
                        }
                    }),
                    NamedProvider::new("yahoo_finance_chart", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move {
                                let end = chrono::Utc::now().date_naive() + chrono::Days::new(1);
                                let start = end - chrono::Days::new(10);
                                let mut items =
                                    client.fetch_us_chart_candles(&sym, start, end).await?;
                                items
                                    .pop()
                                    .map(|last| QuoteSnapshot {
                                        symbol: sym.trim().to_uppercase(),
                                        date: last.trade_date,
                                        open: last.open,
                                        high: last.high,
                                        low: last.low,
                                        close: last.close,
                                        volume: last.volume,
                                    })
                                    .ok_or_else(|| {
                                        anyhow::anyhow!("yahoo chart returned no candles")
                                    })
                            }
                        }
                    }),
                    NamedProvider::new("stooq", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move { client.fetch_us_stooq_quote(&sym).await }
                        }
                    }),
                ];
                self.fetch_with_rotation(
                    symbol,
                    "quote",
                    &providers,
                    &cache_key,
                    super::QUOTE_CACHE_TTL_SECS,
                )
                .await
            }
        }
    }

    /// Fetch candles with provider rotation and diagnosis.
    /// A-share: tencent -> eastmoney | HK: tencent -> yahoo | US: sina -> eastmoney -> yahoo -> stooq
    pub async fn fetch_candles_with_rotation(
        &self,
        symbol: &str,
        adjust: &str,
        limit: usize,
    ) -> (Option<Vec<CandlePoint>>, DataFetchDiagnosis) {
        let market = self.detect_market(symbol);
        let normalized = self.cache_symbol(symbol, market);
        let cache_key = format!(
            "{}:candles:{}:{}:{}:{}",
            super::MARKET_DATA_CACHE_PREFIX,
            super::CANDLES_CACHE_VERSION,
            normalized,
            adjust,
            limit
        );
        match market {
            MarketKind::AShare => {
                let symbol_owned = symbol.to_string();
                let adjust_owned = adjust.to_string();
                let providers: Vec<NamedProvider<Vec<CandlePoint>>> = vec![
                    NamedProvider::new("tencent_kline", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        let adj = adjust_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            let adj = adj.clone();
                            async move {
                                client
                                    .fetch_a_share_tencent_candles(&sym, &adj, limit)
                                    .await
                            }
                        }
                    }),
                    NamedProvider::new("eastmoney_kline", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        let adj = adjust_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            let adj = adj.clone();
                            async move {
                                client
                                    .fetch_a_share_eastmoney_candles(&sym, &adj, limit)
                                    .await
                            }
                        }
                    }),
                ];
                self.fetch_with_rotation(
                    symbol,
                    "candles",
                    &providers,
                    &cache_key,
                    super::CANDLES_CACHE_TTL_SECS,
                )
                .await
            }
            MarketKind::HongKong => {
                let symbol_owned = symbol.to_string();
                let providers: Vec<NamedProvider<Vec<CandlePoint>>> = vec![
                    NamedProvider::new("tencent_kline", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move { client.fetch_hk_tencent_candles(&sym, limit).await }
                        }
                    }),
                    NamedProvider::new("yahoo_finance_chart", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move { client.fetch_hk_yahoo_candles(&sym, limit).await }
                        }
                    }),
                ];
                self.fetch_with_rotation(
                    symbol,
                    "candles",
                    &providers,
                    &cache_key,
                    super::CANDLES_CACHE_TTL_SECS,
                )
                .await
            }
            MarketKind::UsEquity => {
                let symbol_owned = symbol.to_string();
                let providers: Vec<NamedProvider<Vec<CandlePoint>>> = vec![
                    NamedProvider::new("sina_us_daily", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move {
                                super::akshare_rust::us_sina::fetch_candles(&client, &sym, limit)
                                    .await
                            }
                        }
                    }),
                    NamedProvider::new("eastmoney_kline", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move { client.fetch_us_candles_from_eastmoney(&sym, limit).await }
                        }
                    }),
                    NamedProvider::new("yahoo_finance_chart", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move {
                                let end = chrono::Utc::now().date_naive() + chrono::Days::new(1);
                                let start = end - chrono::Days::new((limit.max(260) + 30) as u64);
                                client.fetch_us_chart_candles(&sym, start, end).await
                            }
                        }
                    }),
                    NamedProvider::new("stooq", {
                        let client = self.clone();
                        let sym = symbol_owned.clone();
                        move || {
                            let client = client.clone();
                            let sym = sym.clone();
                            async move { client.fetch_us_stooq_candles(&sym, limit).await }
                        }
                    }),
                ];
                self.fetch_with_rotation(
                    symbol,
                    "candles",
                    &providers,
                    &cache_key,
                    super::CANDLES_CACHE_TTL_SECS,
                )
                .await
            }
        }
    }
}

#[cfg(test)]
mod diagnosis_tests {
    use super::*;

    #[test]
    fn data_fetch_diagnosis_new() {
        let d = DataFetchDiagnosis::new("quote", "AAPL");
        assert_eq!(d.data_type, "quote");
        assert_eq!(d.symbol, "AAPL");
        assert!(d.attempts.is_empty());
        assert_eq!(d.final_status, "failed");
        assert!(!d.used_stale_cache);
    }

    #[test]
    fn data_fetch_diagnosis_summary_success() {
        let mut d = DataFetchDiagnosis::new("quote", "AAPL");
        d.final_status = "success".into();
        d.attempts.push(DataFetchAttempt {
            provider: "yahoo".into(),
            success: true,
            error: None,
            duration_ms: 150,
        });
        let summary = d.summary();
        assert!(summary.contains("quote"));
        assert!(summary.contains("AAPL"));
        assert!(summary.contains("success"));
        assert!(summary.contains("yahoo:ok(150ms)"));
    }

    #[test]
    fn data_fetch_diagnosis_summary_failed() {
        let mut d = DataFetchDiagnosis::new("candles", "000001.SZ");
        d.final_status = "failed".into();
        d.attempts.push(DataFetchAttempt {
            provider: "tencent".into(),
            success: false,
            error: Some("timeout".into()),
            duration_ms: 5000,
        });
        let summary = d.summary();
        assert!(summary.contains("failed"));
        assert!(summary.contains("tencent:err(5000ms)"));
    }

    #[test]
    fn data_fetch_diagnosis_summary_degraded() {
        let mut d = DataFetchDiagnosis::new("news", "TSLA");
        d.final_status = "degraded".into();
        d.used_stale_cache = true;
        let summary = d.summary();
        assert!(summary.contains("degraded"));
    }
}
