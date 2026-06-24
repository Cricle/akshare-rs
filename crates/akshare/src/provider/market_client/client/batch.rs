use super::super::{
    BatchFundamentalsResult, BatchQuoteResult, FundamentalsSnapshot, MarketDataClient,
    QuoteSnapshot, FUNDAMENTALS_CACHE_TTL_SECS, FUNDAMENTALS_CACHE_VERSION,
    MARKET_DATA_CACHE_PREFIX, QUOTE_CACHE_TTL_SECS, QUOTE_CACHE_VERSION,
};

impl MarketDataClient {
    /// Batch fetch quotes for multiple symbols. Returns (symbol, `Option<QuoteSnapshot>`).
    /// Cache hits are returned from Redis MGET; misses are fetched concurrently via akshare.
    pub async fn fetch_quotes_batch(&self, symbols: &[&str]) -> Vec<BatchQuoteResult> {
        if symbols.is_empty() {
            return Vec::new();
        }
        // Build cache keys
        let keys: Vec<(String, String)> = symbols
            .iter()
            .map(|sym| {
                let market = self.detect_market(sym);
                let normalized = self.cache_symbol(sym, market);
                let cache_key =
                    format!("{MARKET_DATA_CACHE_PREFIX}:quote:{QUOTE_CACHE_VERSION}:{normalized}");
                (sym.to_string(), cache_key)
            })
            .collect();
        let cache_keys: Vec<String> = keys.iter().map(|(_, k)| k.clone()).collect();
        // Batch MGET
        let cached: Vec<Option<QuoteSnapshot>> = self.cache_mget_json(&cache_keys).await;
        // Collect misses
        let mut miss_indices = Vec::new();
        for (i, hit) in cached.iter().enumerate() {
            if hit.is_none() {
                miss_indices.push(i);
            }
        }
        // Fetch misses concurrently
        let miss_results: Vec<(usize, Option<QuoteSnapshot>)> = if miss_indices.is_empty() {
            Vec::new()
        } else {
            let futs: Vec<_> = miss_indices
                .iter()
                .map(|&i| {
                    let sym = keys[i].0.clone();
                    let client = self.clone();
                    async move {
                        let result = tokio::time::timeout(
                            std::time::Duration::from_secs(15),
                            super::super::akshare_rust::fetch_quote(&client, &sym),
                        )
                        .await;
                        let quote = match result {
                            Ok(Ok(qwp)) => {
                                // Write to cache
                                let market = client.detect_market(&sym);
                                let normalized = client.cache_symbol(&sym, market);
                                let cache_key = format!(
                                    "{MARKET_DATA_CACHE_PREFIX}:quote:{QUOTE_CACHE_VERSION}:{normalized}"
                                );
                                client
                                    .cache_set_json(&cache_key, QUOTE_CACHE_TTL_SECS, &qwp.quote)
                                    .await;
                                Some(qwp.quote)
                            }
                            _ => None,
                        };
                        (i, quote)
                    }
                })
                .collect();
            futures::future::join_all(futs).await
        };
        // Merge results
        let mut results: Vec<Option<QuoteSnapshot>> = cached;
        for (i, quote) in miss_results {
            results[i] = quote;
        }
        keys.into_iter()
            .zip(results)
            .map(|((sym, _), q)| BatchQuoteResult {
                symbol: sym,
                quote: q,
            })
            .collect()
    }

    /// Batch fetch fundamentals for multiple symbols.
    pub async fn fetch_fundamentals_batch(
        &self,
        symbols: &[&str],
    ) -> Vec<BatchFundamentalsResult> {
        if symbols.is_empty() {
            return Vec::new();
        }
        let keys: Vec<(String, String)> = symbols
            .iter()
            .map(|sym| {
                let market = self.detect_market(sym);
                let normalized = self.cache_symbol(sym, market);
                let cache_key = format!(
                    "{MARKET_DATA_CACHE_PREFIX}:fundamentals:{FUNDAMENTALS_CACHE_VERSION}:{normalized}"
                );
                (sym.to_string(), cache_key)
            })
            .collect();
        let cache_keys: Vec<String> = keys.iter().map(|(_, k)| k.clone()).collect();
        let cached: Vec<Option<FundamentalsSnapshot>> = self.cache_mget_json(&cache_keys).await;
        let mut miss_indices = Vec::new();
        for (i, hit) in cached.iter().enumerate() {
            if hit.is_none() {
                miss_indices.push(i);
            }
        }
        let miss_results: Vec<(usize, Option<FundamentalsSnapshot>)> = if miss_indices.is_empty()
        {
            Vec::new()
        } else {
            let futs: Vec<_> = miss_indices
                .iter()
                .map(|&i| {
                    let sym = keys[i].0.clone();
                    let client = self.clone();
                    async move {
                        let result = tokio::time::timeout(
                            std::time::Duration::from_secs(15),
                            super::super::akshare_rust::fetch_fundamentals(&client, &sym),
                        )
                        .await;
                        let fund = match result {
                            Ok(Ok(f)) => {
                                let market = client.detect_market(&sym);
                                let normalized = client.cache_symbol(&sym, market);
                                let cache_key = format!(
                                    "{MARKET_DATA_CACHE_PREFIX}:fundamentals:{FUNDAMENTALS_CACHE_VERSION}:{normalized}"
                                );
                                client
                                    .cache_set_json(&cache_key, FUNDAMENTALS_CACHE_TTL_SECS, &f)
                                    .await;
                                Some(f)
                            }
                            _ => None,
                        };
                        (i, fund)
                    }
                })
                .collect();
            futures::future::join_all(futs).await
        };
        let mut results: Vec<Option<FundamentalsSnapshot>> = cached;
        for (i, fund) in miss_results {
            results[i] = fund;
        }
        keys.into_iter()
            .zip(results)
            .map(|((sym, _), f)| BatchFundamentalsResult {
                symbol: sym,
                fundamentals: f,
            })
            .collect()
    }
}
