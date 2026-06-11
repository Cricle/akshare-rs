//! Financial news search via Eastmoney search API.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::NewsItem;

/// Strip `<em>` / `</em>` highlight tags and other HTML tags from text.
fn strip_em_tags(text: &str) -> String {
    text.replace("<em>", "").replace("</em>", "")
}

/// Parse the JSONP wrapper around Eastmoney search response.
///
/// Response format: `jQuery...({...})`  -- we strip the callback prefix and
/// trailing `)` to extract the inner JSON object.
fn strip_jsonp(raw: &str) -> Result<&str> {
    // Find the first `(` which marks the start of the JSON payload.
    let start = raw
        .find('(')
        .ok_or_else(|| Error::decode("JSONP: opening '(' not found"))?;
    let inner = &raw[start + 1..];
    // Remove the trailing `);` or `)`
    let end = inner
        .rfind(')')
        .ok_or_else(|| Error::decode("JSONP: closing ')' not found"))?;
    Ok(inner[..end].trim())
}

impl AkShareClient {
    /// Search financial news from Eastmoney.
    ///
    /// Queries `https://search-api-web.eastmoney.com/search/jsonp` with the
    /// given keyword and returns up to `limit` news items.
    pub async fn news_search(&self, query: &str, limit: usize) -> Result<Vec<NewsItem>> {
        if query.is_empty() {
            return Err(Error::invalid_input("query must not be empty"));
        }
        if limit == 0 {
            return Ok(Vec::new());
        }

        let page_size = limit.min(100); // Eastmoney caps at 100 per page

        let inner_param = serde_json::json!({
            "uid": "",
            "keyword": query,
            "type": ["cmsArticleWebOld"],
            "client": "web",
            "clientType": "web",
            "clientVersion": "curr",
            "param": {
                "cmsArticleWebOld": {
                    "searchScope": "default",
                    "sort": "default",
                    "pageIndex": 1,
                    "pageSize": page_size,
                    "preTag": "<em>",
                    "postTag": "</em>",
                }
            }
        });

        let referer = format!("https://so.eastmoney.com/news/s?keyword={query}");

        let raw_text = self
            .get("https://search-api-web.eastmoney.com/search/jsonp")
            .query(&[
                ("cb", "jQuery_callback"),
                ("param", &inner_param.to_string()),
            ])
            .header("Referer", &referer)
            .send()
            .await?
            .text()
            .await?;

        let json_str = strip_jsonp(&raw_text)?;
        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("JSON parse: {e}")))?;

        let list = root
            .pointer("/result/cmsArticleWebOld")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::with_capacity(list.len().min(limit));
        for entry in &list {
            if items.len() >= limit {
                break;
            }
            let title_raw = entry.get("title").and_then(|v| v.as_str()).unwrap_or("");
            let content_raw = entry.get("content").and_then(|v| v.as_str()).unwrap_or("");
            let date = entry
                .get("date")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let media = entry
                .get("mediaName")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // The API may return articleUrl directly or a code field.
            let url = entry
                .get("articleUrl")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| {
                    entry
                        .get("code")
                        .and_then(|v| v.as_str())
                        .map(|code| format!("http://finance.eastmoney.com/a/{code}.html"))
                });

            // Clean up the HTML-tagged summary: strip tags, whitespace artifacts
            let summary = strip_em_tags(content_raw)
                .replace("\u{3000}", "")
                .replace("\r\n", " ")
                .trim()
                .to_string();

            items.push(NewsItem {
                published_at: date,
                title: strip_em_tags(title_raw),
                summary,
                source: media,
                url,
            });
        }

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_em_tags() {
        assert_eq!(strip_em_tags("hello <em>world</em>"), "hello world");
        assert_eq!(strip_em_tags("<em>test</em>"), "test");
        assert_eq!(strip_em_tags("no tags"), "no tags");
    }

    #[test]
    fn test_strip_jsonp_valid() {
        let raw = r#"jQuery123({"key":"value"})"#;
        let result = strip_jsonp(raw).unwrap();
        assert_eq!(result, r#"{"key":"value"}"#);
    }

    #[test]
    fn test_strip_jsonp_with_semicolon() {
        let raw = r#"jQuery123({"a":1});"#;
        let result = strip_jsonp(raw).unwrap();
        assert_eq!(result, r#"{"a":1}"#);
    }

    #[test]
    fn test_strip_jsonp_no_opening() {
        assert!(strip_jsonp("no jsonp here").is_err());
    }

    #[test]
    fn test_strip_jsonp_no_closing() {
        assert!(strip_jsonp("jQuery123({\"a\":1}").is_err());
    }

    #[test]
    fn test_parse_news_response() {
        // Simulate the JSON portion of a JSONP response.
        let json_str = r#"{
            "result": {
                "cmsArticleWebOld": [
                    {
                        "title": "Test <em>News</em> Title",
                        "content": "Some <em>content</em> here　",
                        "date": "2025-06-01 12:00",
                        "mediaName": "TestMedia",
                        "articleUrl": "https://example.com/article"
                    }
                ]
            }
        }"#;
        let root: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let list = root
            .pointer("/result/cmsArticleWebOld")
            .and_then(|v| v.as_array())
            .unwrap();

        assert_eq!(list.len(), 1);
        let entry = &list[0];
        let title = strip_em_tags(entry.get("title").and_then(|v| v.as_str()).unwrap());
        assert_eq!(title, "Test News Title");

        let content = strip_em_tags(entry.get("content").and_then(|v| v.as_str()).unwrap());
        assert_eq!(content, "Some content here\u{3000}");
    }
}
