//! Marketaux news API (global/HK stocks).

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::NewsItem;

impl AkShareClient {
    /// Fetch stock news from Marketaux.
    ///
    /// `symbol` is the stock ticker (e.g. "AAPL", "0700.HK").
    /// `api_key` is the Marketaux API key (free tier: 100 req/day).
    /// `limit` is the max number of articles to return (API max: 3 on free tier).
    pub async fn marketaux_news(
        &self,
        symbol: &str,
        api_key: &str,
        limit: usize,
    ) -> Result<Vec<NewsItem>> {
        if symbol.is_empty() {
            return Err(Error::invalid_input("symbol must not be empty"));
        }
        if api_key.is_empty() {
            return Err(Error::invalid_input("api_key must not be empty"));
        }

        let limit_str = limit.min(3).to_string();
        let resp: serde_json::Value = self
            .get("https://api.marketaux.com/v1/news/all")
            .query(&[
                ("entities", symbol),
                ("api_token", api_key),
                ("limit", &limit_str),
            ])
            .send()
            .await?
            .json()
            .await?;

        let arr = resp
            .get("data")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("marketaux: missing 'data' array"))?;

        let items: Vec<NewsItem> = arr
            .iter()
            .filter_map(|item| {
                let title = item.get("title")?.as_str()?;
                if title.is_empty() {
                    return None;
                }
                let description = item
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let url = item.get("url").and_then(|v| v.as_str()).map(str::to_string);
                let source = item
                    .get("source")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Marketaux")
                    .to_string();
                let published_at = item
                    .get("published_at")
                    .and_then(|v| v.as_str())
                    .map(|s| s[..10.min(s.len())].to_string())
                    .unwrap_or_default();

                Some(NewsItem {
                    published_at,
                    title: title.to_string(),
                    summary: description,
                    source,
                    url,
                })
            })
            .collect();

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_published_at_truncation() {
        // Marketaux returns ISO 8601 like "2026-06-15T12:00:00+00:00"
        let raw = "2026-06-15T12:00:00+00:00";
        assert_eq!(&raw[..10.min(raw.len())], "2026-06-15");
    }
}
