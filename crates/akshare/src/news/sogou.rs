//! Sogou news search (HTML scraping).

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::NewsItem;

impl AkShareClient {
    /// Search news from Sogou News.
    ///
    /// Queries `news.sogou.com/news?query=...&sort=1` and parses the HTML response
    /// to extract news titles and URLs from `<h3><a>` blocks.
    pub async fn sogou_news_search(&self, query: &str, timeout_secs: u64) -> Result<Vec<NewsItem>> {
        let sogou_url = format!(
            "https://news.sogou.com/news?query={}&sort=1",
            query.replace(' ', "+")
        );
        let body = tokio::time::timeout(
            std::time::Duration::from_secs(timeout_secs),
            self.get(&sogou_url).send(),
        )
        .await
        .map_err(|_| {
            crate::Error::upstream(format!(
                "Sogou News request timed out after {timeout_secs}s"
            ))
        })?
        .map_err(crate::Error::from)?
        .text()
        .await
        .map_err(crate::Error::from)?;

        let mut items = Vec::new();
        let mut remaining = body.as_str();
        while let Some(pos) = remaining.find("<h3") {
            let chunk = &remaining[pos..];
            if let Some(a_start) = chunk.find("<a href=\"") {
                let after_href = &chunk[a_start + 9..];
                if let Some(quote_end) = after_href.find('"') {
                    let url = after_href[..quote_end].to_string();
                    let title_start = after_href.find('>').map(|i| i + 1);
                    let title = title_start.and_then(|start| {
                        let title_chunk = &after_href[start..];
                        title_chunk.find("</a>").map(|end| {
                            let raw = &title_chunk[..end];
                            let clean = raw.split('<').next().unwrap_or(raw).to_string();
                            clean.trim().to_string()
                        })
                    });
                    if let (Some(title), true) = (title, !url.is_empty() && url.starts_with("http"))
                        && !title.is_empty()
                    {
                        items.push(NewsItem {
                            published_at: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                            title,
                            summary: String::new(),
                            source: "sogou_news".to_string(),
                            url: Some(url),
                        });
                    }
                }
            }
            remaining = &remaining[pos + 3..];
        }
        Ok(items)
    }
}
