//! Cache stubs for MarketDataClient.
//!
//! Without the `redis-cache` feature, all cache operations are no-ops.
//! These methods provide the same API as the stock-analyzer version so that
//! search.rs and news_search can compile without changes.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Notify;

use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

use super::{MARKET_DATA_CACHE_PREFIX, MarketDataClient, SearchProviderConfig, SearchScope};

impl MarketDataClient {
    // --- Symbol normalization ---

    pub(super) fn normalize_a_share_symbol(&self, symbol: &str) -> Option<String> {
        let normalized = symbol.trim().to_uppercase();
        if normalized.ends_with(".SH") || normalized.ends_with(".SZ") || normalized.ends_with(".BJ")
        {
            return Some(normalized);
        }
        if normalized.len() != 6 || !normalized.chars().all(|char| char.is_ascii_digit()) {
            return None;
        }

        let suffix = match normalized.chars().next()? {
            '6' | '5' | '9' => "SH",
            '0' | '1' | '2' | '3' => "SZ",
            '4' | '8' => "BJ",
            _ => return None,
        };
        Some(format!("{normalized}.{suffix}"))
    }

    pub(super) fn normalize_hk_symbol(&self, symbol: &str) -> Option<String> {
        let normalized = symbol.trim().to_uppercase();
        if normalized.ends_with(".HK") {
            let code = normalized.trim_end_matches(".HK");
            if code.len() == 4 || code.len() == 5 {
                return code
                    .chars()
                    .all(|char| char.is_ascii_digit())
                    .then_some(format!("{code:0>5}.HK"));
            }
        }
        if (normalized.len() == 4 || normalized.len() == 5)
            && normalized.chars().all(|char| char.is_ascii_digit())
        {
            return Some(format!("{normalized:0>5}.HK"));
        }
        None
    }

    pub(super) fn cache_symbol(&self, symbol: &str, market: super::MarketKind) -> String {
        match market {
            super::MarketKind::AShare => self
                .normalize_a_share_symbol(symbol)
                .unwrap_or_else(|| symbol.trim().to_uppercase()),
            super::MarketKind::HongKong => self
                .normalize_hk_symbol(symbol)
                .unwrap_or_else(|| symbol.trim().to_uppercase()),
            super::MarketKind::UsEquity => symbol.trim().to_uppercase(),
        }
    }

    // --- Cache read/write stubs (no-ops without Redis) ---

    pub(super) async fn cache_get_json<T>(&self, _key: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        None
    }

    pub(super) async fn cache_get_json_exact<T>(&self, _key: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        None
    }

    pub(super) async fn cache_mget_json<T>(&self, _keys: &[String]) -> Vec<Option<T>>
    where
        T: DeserializeOwned,
    {
        vec![]
    }

    pub(super) async fn cache_set_json<T>(&self, _key: &str, _ttl_secs: u64, _value: &T)
    where
        T: Serialize,
    {
    }

    // --- Cache key utilities ---

    pub(super) fn stale_cache_key(&self, key: &str) -> String {
        format!("{key}:stale")
    }

    pub(super) fn normalized_news_query(&self, query: &str) -> String {
        query
            .trim()
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub(super) fn news_query_cache_component(&self, query: Option<&str>) -> String {
        let normalized = query
            .map(|value| self.normalized_news_query(value))
            .unwrap_or_default();
        let digest = Sha256::digest(normalized.as_bytes());
        format!("{digest:x}")
    }

    pub(crate) fn normalize_optional_query(query: Option<&str>) -> Option<String> {
        query
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
    }

    pub(super) fn search_query_cache_key(
        &self,
        provider: &SearchProviderConfig,
        query: &str,
        language: &str,
        time_range: Option<&str>,
        scope: SearchScope,
    ) -> String {
        let digest = Sha256::digest(
            format!(
                "provider={}|q={}|language={}|time_range={}|scope={}",
                provider.cache_scope(),
                self.normalized_news_query(query),
                language.trim(),
                time_range.unwrap_or_default().trim(),
                scope.as_str(),
            )
            .as_bytes(),
        );
        format!("{MARKET_DATA_CACHE_PREFIX}:search:news_query:v2:{digest:x}")
    }

    pub(super) fn search_evidence_cache_key(
        &self,
        queries: &[&str],
        language: &str,
        time_range: Option<&str>,
        scope: SearchScope,
    ) -> String {
        let mut normalized_queries = queries
            .iter()
            .map(|query| self.normalized_news_query(query))
            .filter(|query| !query.is_empty())
            .collect::<Vec<_>>();
        normalized_queries.sort();
        normalized_queries.dedup();
        let mut provider_scopes = self
            .search_providers
            .iter()
            .map(SearchProviderConfig::cache_scope)
            .collect::<Vec<_>>();
        provider_scopes.sort();
        provider_scopes.dedup();
        let digest = Sha256::digest(
            format!(
                "providers={}|queries={}|language={}|time_range={}|scope={}",
                provider_scopes.join("|"),
                normalized_queries.join("|"),
                language.trim(),
                time_range.unwrap_or_default().trim(),
                scope.as_str(),
            )
            .as_bytes(),
        );
        format!("{MARKET_DATA_CACHE_PREFIX}:search:news_evidence:v2:{digest:x}")
    }
}

// ============================================================
// Cache Stampede Protection (Singleflight)
// ============================================================

pub enum SingleflightResult<'a> {
    Leader(SingleflightGuard<'a>),
    Waiting,
}

pub struct SingleflightGuard<'a> {
    singleflight: &'a Singleflight,
    key: String,
}

impl Drop for SingleflightGuard<'_> {
    fn drop(&mut self) {
        if let Ok(mut map) = self.singleflight.in_flight.lock()
            && let Some(notify) = map.remove(&self.key)
        {
            drop(map);
            notify.notify_waiters();
        }
    }
}

pub struct Singleflight {
    in_flight: Arc<std::sync::Mutex<HashMap<String, Arc<Notify>>>>,
}

impl Clone for Singleflight {
    fn clone(&self) -> Self {
        Self {
            in_flight: Arc::clone(&self.in_flight),
        }
    }
}

impl Default for Singleflight {
    fn default() -> Self {
        Self::new()
    }
}

impl Singleflight {
    pub fn new() -> Self {
        Self {
            in_flight: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    pub async fn enter(&self, key: &str) -> SingleflightResult<'_> {
        let (is_leader, notify) = {
            let mut map = self.in_flight.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(n) = map.get(key) {
                (false, n.clone())
            } else {
                let n = Arc::new(Notify::new());
                map.insert(key.to_string(), n.clone());
                (true, n)
            }
        };

        if is_leader {
            SingleflightResult::Leader(SingleflightGuard {
                singleflight: self,
                key: key.to_string(),
            })
        } else {
            notify.notified().await;
            SingleflightResult::Waiting
        }
    }

    pub async fn do_once<F, Fut, T>(&self, key: &str, compute: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let _guard = match self.enter(key).await {
            SingleflightResult::Leader(guard) => guard,
            SingleflightResult::Waiting => {
                return compute().await;
            }
        };
        compute().await
    }
}
