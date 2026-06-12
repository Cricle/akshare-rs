//! GDELT news search.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::NewsItem;

impl AkShareClient {
    /// Search news from GDELT API.
    ///
    /// `base_url` is the GDELT API endpoint (e.g. `https://api.gdeltproject.org/api/v2/doc/doc`).
    /// `language_hint` optionally restricts to a source language ("zh-CN" → Chinese, "en-US" → English).
    pub async fn gdelt_news_search(
        &self,
        query: &str,
        base_url: &str,
        language_hint: Option<&str>,
        time_range: Option<&str>,
        timeout_secs: u64,
    ) -> Result<Vec<NewsItem>> {
        let final_query = match language_hint {
            Some("zh-CN") => format!("{query} sourceLang:Chinese"),
            Some("en-US") => format!("{query} sourceLang:English"),
            _ => query.to_string(),
        };
        let max_records = 25;
        let mut request = self.get(base_url).query(&[
            ("query", final_query.as_str()),
            ("mode", "ArtList"),
            ("format", "json"),
            ("maxrecords", &max_records.to_string()),
            ("sort", "DateDesc"),
        ]);
        if let Some(range) = time_range {
            let timespan = match range {
                "day" => "1day",
                "week" => "7days",
                "month" => "1month",
                other => other,
            };
            request = request.query(&[("timespan", timespan)]);
        } else {
            request = request.query(&[("timespan", "1month")]);
        }

        let response = tokio::time::timeout(std::time::Duration::from_secs(timeout_secs), request.send())
            .await
            .map_err(|_| {
                Error::upstream(format!("GDELT request timed out after {timeout_secs}s"))
            })?
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let items = payload
            .get("articles")
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
            .filter_map(|item| {
                let title = item.get("title").and_then(|v| v.as_str())?.trim();
                if title.is_empty() {
                    return None;
                }
                let url = item
                    .get("url")
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                let source = item
                    .get("sourceCommonName")
                    .and_then(|v| v.as_str())
                    .filter(|v| !v.trim().is_empty())
                    .unwrap_or("GDELT")
                    .to_string();
                let published_at = item
                    .get("seendate")
                    .and_then(|v| v.as_str())
                    .map(gdelt_timestamp_to_date)
                    .unwrap_or_default();
                Some(NewsItem {
                    published_at,
                    title: title.to_string(),
                    summary: String::new(),
                    source,
                    url,
                })
            })
            .collect();
        Ok(items)
    }
}

/// Convert GDELT seendate (`20260603120000`) to `YYYY-MM-DD`.
fn gdelt_timestamp_to_date(raw: &str) -> String {
    if raw.len() >= 8 {
        format!("{}-{}-{}", &raw[0..4], &raw[4..6], &raw[6..8])
    } else {
        String::new()
    }
}
