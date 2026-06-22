//! Baidu news search (HTML scraping).

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::NewsItem;

impl AkShareClient {
    /// Search news from Baidu News.
    ///
    /// Queries `www.baidu.com/s?wd=...&tn=news` and parses the HTML response.
    pub async fn baidu_news_search(&self, query: &str, timeout_secs: u64) -> Result<Vec<NewsItem>> {
        if query.is_empty() {
            return Err(Error::invalid_input("query must not be empty"));
        }
        let encoded_query = percent_encode(query);
        let search_url = format!(
            "https://www.baidu.com/s?wd={encoded_query}&tn=news&rtt=4&bsst=1&cl=2&medium=0"
        );
        let body = tokio::time::timeout(
            std::time::Duration::from_secs(timeout_secs),
            self.get(&search_url)
                .header(
                    "User-Agent",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
                )
                .send(),
        )
        .await
        .map_err(|_| Error::upstream(format!("Baidu News request timed out after {timeout_secs}s")))?
        .map_err(Error::from)?
        .text()
        .await
        .map_err(Error::from)?;

        let mut items = Vec::new();
        // Primary pattern: div.result blocks
        for block in body.split("class=\"result\"").skip(1) {
            let block_end = block.find("class=\"result\"").unwrap_or(block.len());
            let block = &block[..block_end.min(4000)];
            if let Some(item) = parse_baidu_block(block) {
                items.push(item);
            }
        }
        // Fallback: <h3> blocks
        if items.is_empty() {
            for chunk in body.split("<h3").skip(1) {
                let chunk_end = chunk.find("<h3").unwrap_or(chunk.len());
                let chunk = &chunk[..chunk_end.min(4000)];
                if let Some(item) = parse_baidu_block(chunk) {
                    items.push(item);
                }
            }
        }
        Ok(items)
    }
}

fn parse_baidu_block(html: &str) -> Option<NewsItem> {
    let (title, url) = extract_baidu_link(html)?;
    if title.trim().is_empty() {
        return None;
    }
    let summary =
        extract_baidu_text_between(html, &["c-abstract", "c-span-last", "content-right_8Zs40"])
            .or_else(|| extract_baidu_plain_text(html))
            .unwrap_or_default();
    let (source, published_at) =
        extract_baidu_source(html).unwrap_or_else(|| ("Baidu".to_string(), String::new()));
    Some(NewsItem {
        published_at,
        title: title.trim().to_string(),
        summary,
        source,
        url: Some(url),
    })
}

fn extract_baidu_link(html: &str) -> Option<(String, String)> {
    let a_start = html.find("<a ")?;
    let a_end_tag = html[a_start..].find('>')? + a_start;
    let a_tag = &html[a_start..a_end_tag];
    let href = a_tag
        .find("href=\"")
        .and_then(|i| {
            let rest = &a_tag[i + 6..];
            rest.find('"').map(|end| rest[..end].to_string())
        })
        .or_else(|| {
            a_tag.find("href='").and_then(|i| {
                let rest = &a_tag[i + 6..];
                rest.find('\'').map(|end| rest[..end].to_string())
            })
        })?;
    let after_a = &html[a_end_tag + 1..];
    let a_close = after_a.find("</a>")?;
    let title_html = &after_a[..a_close];
    let title = strip_html_tags(title_html);
    let title = decode_html_entities(&title);
    if title.trim().is_empty() || href.is_empty() {
        return None;
    }
    let url = if href.starts_with("http") {
        href
    } else {
        format!("https://www.baidu.com{href}")
    };
    Some((title, url))
}

fn extract_baidu_text_between(html: &str, class_names: &[&str]) -> Option<String> {
    for class_name in class_names {
        let marker = format!("class=\"{class_name}\"");
        if let Some(pos) = html.find(&marker) {
            let after = &html[pos..];
            let tag_end = after.find('>')? + 1;
            let content_start = &after[tag_end..];
            let close_div = content_start
                .find("</div>")
                .unwrap_or_else(|| content_start.len().min(800));
            let text = strip_html_tags(&content_start[..close_div]);
            let text = decode_html_entities(&text);
            if !text.trim().is_empty() {
                return Some(text.trim().to_string());
            }
        }
    }
    None
}

fn extract_baidu_source(html: &str) -> Option<(String, String)> {
    let source_markers = ["c-color-gray", "c-gap-right-small", "news-source", "source"];
    for marker in &source_markers {
        let class_attr = format!("class=\"{marker}\"");
        if let Some(pos) = html.find(&class_attr) {
            let after = &html[pos..];
            let tag_end = after.find('>')? + 1;
            let content = &after[tag_end..];
            let span_close = content
                .find("</span>")
                .or_else(|| content.find("</a>"))
                .unwrap_or_else(|| content.len().min(200));
            let text = strip_html_tags(&content[..span_close]);
            let text = decode_html_entities(&text);
            if !text.trim().is_empty() {
                let parts: Vec<&str> = text.split_whitespace().collect();
                if let Some(last) = parts.last().filter(|_| parts.len() >= 2)
                    && (last.contains('-') || last.contains(':'))
                {
                    let source = parts[..parts.len() - 1].join(" ");
                    return Some((source, last.to_string()));
                }
                return Some((text.trim().to_string(), String::new()));
            }
        }
    }
    None
}

fn extract_baidu_plain_text(html: &str) -> Option<String> {
    let text = strip_html_tags(html);
    let text = decode_html_entities(&text);
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
}

fn decode_html_entities(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

fn percent_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len() * 3);
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            b' ' => encoded.push('+'),
            _ => {
                use std::fmt::Write as _;
                let _ = write!(encoded, "%{byte:02X}");
            }
        }
    }
    encoded
}
