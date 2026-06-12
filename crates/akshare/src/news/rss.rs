//! RSS-based news search (Bing, Google News).

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::NewsItem;

/// Parse a single XML tag value from an RSS `<item>` block.
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

/// Normalize an RSS date string to `YYYY-MM-DD`.
fn normalize_rss_date(raw: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(raw) {
        return dt.format("%Y-%m-%d").to_string();
    }
    if raw.len() >= 10 && raw.as_bytes()[4] == b'-' && raw.as_bytes()[7] == b'-' {
        return raw[..10].to_string();
    }
    String::new()
}

/// Parse RSS XML body into `NewsItem`s, filtering out items whose title
/// contains `exclude_title_substrings`.
fn parse_rss_items(body: &str, source: &str, exclude_title_substrings: &[&str]) -> Vec<NewsItem> {
    let mut items = Vec::new();
    for item_xml in body.split("<item>").skip(1) {
        let end = item_xml.find("</item>").unwrap_or(item_xml.len());
        let xml = &item_xml[..end];
        let title = extract_rss_tag(xml, "title").filter(|t| {
            !exclude_title_substrings
                .iter()
                .any(|sub| t.contains(sub))
        });
        let link = extract_rss_tag(xml, "link");
        let desc = extract_rss_tag(xml, "description");
        let date = extract_rss_tag(xml, "pubDate")
            .map(|d| normalize_rss_date(&d))
            .unwrap_or_default();
        if let (Some(title), Some(url)) = (title, link) {
            let published_at = if date.is_empty() {
                chrono::Utc::now().format("%Y-%m-%d").to_string()
            } else {
                date
            };
            items.push(NewsItem {
                published_at,
                title,
                summary: desc.unwrap_or_default(),
                source: source.to_string(),
                url: Some(url),
            });
        }
    }
    items
}

impl AkShareClient {
    /// Fetch news from Bing RSS search.
    ///
    /// Queries `cn.bing.com/search?q=...&format=rss` and parses the RSS response.
    /// Filters out items whose title contains "必应" or "Bing".
    pub async fn bing_news_rss(&self, query: &str, timeout_secs: u64) -> Result<Vec<NewsItem>> {
        let rss_url = format!(
            "https://cn.bing.com/search?q={}&format=rss",
            query.replace(' ', "+")
        );
        let body = tokio::time::timeout(
            std::time::Duration::from_secs(timeout_secs),
            self.get(&rss_url).send(),
        )
        .await
        .map_err(|_| {
            crate::Error::upstream(format!("Bing RSS request timed out after {timeout_secs}s"))
        })?
        .map_err(crate::Error::from)?
        .text()
        .await
        .map_err(crate::Error::from)?;

        Ok(parse_rss_items(&body, "bing_rss", &["必应", "Bing"]))
    }

    /// Fetch news from Google News RSS search.
    ///
    /// Queries `news.google.com/rss/search?q=...&hl=en-US&gl=US&ceid=US:en`.
    pub async fn google_news_rss(&self, query: &str, timeout_secs: u64) -> Result<Vec<NewsItem>> {
        let gnews_url = format!(
            "https://news.google.com/rss/search?q={}&hl=en-US&gl=US&ceid=US:en",
            query.replace(' ', "+")
        );
        let body = tokio::time::timeout(
            std::time::Duration::from_secs(timeout_secs),
            self.get(&gnews_url).send(),
        )
        .await
        .map_err(|_| {
            crate::Error::upstream(format!(
                "Google News RSS request timed out after {timeout_secs}s"
            ))
        })?
        .map_err(crate::Error::from)?
        .text()
        .await
        .map_err(crate::Error::from)?;

        Ok(parse_rss_items(&body, "google_news_rss", &[]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rss_tag() {
        let xml = "<item><title>Hello World</title><link>https://example.com</link></item>";
        assert_eq!(
            extract_rss_tag(xml, "title"),
            Some("Hello World".to_string())
        );
        assert_eq!(
            extract_rss_tag(xml, "link"),
            Some("https://example.com".to_string())
        );
        assert_eq!(extract_rss_tag(xml, "missing"), None);
    }

    #[test]
    fn test_extract_rss_tag_cdata() {
        let xml = "<title><![CDATA[CDATA Title]]></title>";
        assert_eq!(
            extract_rss_tag(xml, "title"),
            Some("CDATA Title".to_string())
        );
    }

    #[test]
    fn test_normalize_rss_date_rfc2822() {
        assert_eq!(
            normalize_rss_date("Wed, 03 Jun 2026 00:36:00 GMT"),
            "2026-06-03"
        );
    }

    #[test]
    fn test_normalize_rss_date_iso() {
        assert_eq!(normalize_rss_date("2026-06-03"), "2026-06-03");
    }

    #[test]
    fn test_normalize_rss_date_unknown() {
        assert_eq!(normalize_rss_date("unknown format"), "");
    }

    #[test]
    fn test_parse_rss_items_basic() {
        let body = r#"
<channel>
<item><title>T1</title><link>http://a.com</link><description>D1</description><pubDate>Wed, 03 Jun 2026 00:00:00 GMT</pubDate></item>
<item><title>T2</title><link>http://b.com</link><description>D2</description></item>
</channel>"#;
        let items = parse_rss_items(body, "test", &[]);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "T1");
        assert_eq!(items[1].title, "T2");
        assert_eq!(items[0].source, "test");
    }

    #[test]
    fn test_parse_rss_items_excludes_filtered_titles() {
        let body = r#"
<channel>
<item><title>必应 News</title><link>http://a.com</link></item>
<item><title>Real News</title><link>http://b.com</link></item>
</channel>"#;
        let items = parse_rss_items(body, "test", &["必应", "Bing"]);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Real News");
    }
}
