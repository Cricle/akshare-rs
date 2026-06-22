//! Finnhub company news API (US stocks).

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::NewsItem;

impl AkShareClient {
    /// Fetch company-specific news from Finnhub.
    ///
    /// `symbol` is the stock ticker (e.g. "AAPL").
    /// `from` and `to` are dates in "YYYY-MM-DD" format.
    /// `api_key` is the Finnhub API key (free tier: 60 req/min).
    pub async fn finnhub_company_news(
        &self,
        symbol: &str,
        from: &str,
        to: &str,
        api_key: &str,
    ) -> Result<Vec<NewsItem>> {
        if symbol.is_empty() {
            return Err(Error::invalid_input("symbol must not be empty"));
        }
        if api_key.is_empty() {
            return Err(Error::invalid_input("api_key must not be empty"));
        }

        let resp: serde_json::Value = self
            .get("https://finnhub.io/api/v1/company-news")
            .query(&[
                ("symbol", symbol),
                ("from", from),
                ("to", to),
                ("token", api_key),
            ])
            .send()
            .await?
            .json()
            .await?;

        let arr = resp
            .as_array()
            .ok_or_else(|| Error::decode("finnhub: expected JSON array"))?;

        let items: Vec<NewsItem> = arr
            .iter()
            .filter_map(|item| {
                let headline = item.get("headline")?.as_str()?;
                if headline.is_empty() {
                    return None;
                }
                let summary = item
                    .get("summary")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let source = item
                    .get("source")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Finnhub")
                    .to_string();
                let url = item.get("url").and_then(|v| v.as_str()).map(str::to_string);
                let published_at = item
                    .get("datetime")
                    .and_then(serde_json::Value::as_i64)
                    .map(timestamp_to_date)
                    .unwrap_or_default();

                Some(NewsItem {
                    published_at,
                    title: headline.to_string(),
                    summary,
                    source,
                    url,
                })
            })
            .collect();

        Ok(items)
    }
}

/// Convert unix timestamp to `YYYY-MM-DD`.
fn timestamp_to_date(ts: i64) -> String {
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_to_date() {
        assert_eq!(timestamp_to_date(1_718_496_000), "2024-06-16");
        assert_eq!(timestamp_to_date(0), "1970-01-01");
    }
}
