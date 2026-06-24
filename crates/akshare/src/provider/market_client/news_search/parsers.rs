//! HTML, XML, and URL parsing utilities for news search.

pub(super) fn extract_baidu_link(html: &str) -> Option<(String, String)> {
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
        format!("https://www.baidu.com{}", href)
    };

    Some((title, url))
}

pub(super) fn extract_baidu_text_between(html: &str, class_names: &[&str]) -> Option<String> {
    for class_name in class_names {
        let marker = format!("class=\"{}\"", class_name);
        if let Some(pos) = html.find(&marker) {
            let after = &html[pos..];
            let tag_end = after.find('>')? + 1;
            let content_start = &after[tag_end..];
            let close_div = content_start
                .find("</div>")
                .unwrap_or(content_start.len().min(800));
            let text = strip_html_tags(&content_start[..close_div]);
            let text = decode_html_entities(&text);
            if !text.trim().is_empty() {
                return Some(text.trim().to_string());
            }
        }
    }
    None
}

pub(super) fn extract_baidu_source(html: &str) -> Option<(String, String)> {
    let source_markers = ["c-color-gray", "c-gap-right-small", "news-source", "source"];
    for marker in &source_markers {
        let class_attr = format!("class=\"{}\"", marker);
        if let Some(pos) = html.find(&class_attr) {
            let after = &html[pos..];
            let tag_end = after.find('>')? + 1;
            let content = &after[tag_end..];
            let span_close = content
                .find("</span>")
                .or_else(|| content.find("</a>"))
                .unwrap_or(content.len().min(200));
            let text = strip_html_tags(&content[..span_close]);
            let text = decode_html_entities(&text);
            if !text.trim().is_empty() {
                let parts: Vec<&str> = text.split_whitespace().collect();
                if let Some(last) = parts.last().filter(|_| parts.len() >= 2) {
                    if last.contains('-') || last.contains(':') {
                        let source = parts[..parts.len() - 1].join(" ");
                        return Some((source, last.to_string()));
                    }
                }
                return Some((text.trim().to_string(), String::new()));
            }
        }
    }
    None
}

pub(super) fn extract_baidu_plain_text(html: &str) -> Option<String> {
    let text = strip_html_tags(html);
    let text = decode_html_entities(&text);
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

pub(super) fn strip_html_tags(html: &str) -> String {
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

pub(super) fn decode_html_entities(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

pub(super) fn percent_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len() * 3);
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            b' ' => encoded.push('+'),
            _ => encoded.push_str(&format!("%{:02X}", byte)),
        }
    }
    encoded
}

pub(super) fn extract_rss_tag(xml: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{tag}>");
    let end_tag = format!("</{tag}>");
    let start = xml.find(&start_tag)? + start_tag.len();
    let end = xml.find(&end_tag)?;
    let value = xml[start..end].trim();
    // Strip CDATA
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

pub(super) fn normalize_rss_date(raw: &str) -> String {
    // Try RFC 2822: "Wed, 03 Jun 2026 00:36:00 GMT"
    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(raw) {
        return dt.format("%Y-%m-%d").to_string();
    }
    // Try ISO: "2026-06-03"
    if raw.len() >= 10 && raw.as_bytes()[4] == b'-' && raw.as_bytes()[7] == b'-' {
        return raw[..10].to_string();
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- strip_html_tags ---

    #[test]
    fn strip_html_tags_plain_text() {
        assert_eq!(strip_html_tags("hello world"), "hello world");
    }

    #[test]
    fn strip_html_tags_simple_tag() {
        assert_eq!(strip_html_tags("<b>bold</b>"), "bold");
    }

    #[test]
    fn strip_html_tags_nested_tags() {
        assert_eq!(strip_html_tags("<div><p>text</p></div>"), "text");
    }

    #[test]
    fn strip_html_tags_empty() {
        assert_eq!(strip_html_tags(""), "");
    }

    #[test]
    fn strip_html_tags_only_tags() {
        assert_eq!(strip_html_tags("<br><hr>"), "");
    }

    #[test]
    fn strip_html_tags_attributes() {
        assert_eq!(strip_html_tags(r#"<a href="http://example.com">link</a>"#), "link");
    }

    // --- decode_html_entities ---

    #[test]
    fn decode_html_entities_no_entities() {
        assert_eq!(decode_html_entities("hello"), "hello");
    }

    #[test]
    fn decode_html_entities_all_entities() {
        assert_eq!(decode_html_entities("&amp;&lt;&gt;&quot;&#39;&nbsp;"), "&<>\"' ");
    }

    #[test]
    fn decode_html_entities_mixed() {
        assert_eq!(decode_html_entities("a &amp; b"), "a & b");
    }

    // --- percent_encode ---

    #[test]
    fn percent_encode_alphanumeric() {
        assert_eq!(percent_encode("abc123"), "abc123");
    }

    #[test]
    fn percent_encode_space() {
        assert_eq!(percent_encode("hello world"), "hello+world");
    }

    #[test]
    fn percent_encode_special_chars() {
        assert_eq!(percent_encode("a&b"), "a%26b");
    }

    #[test]
    fn percent_encode_safe_chars() {
        assert_eq!(percent_encode("-_.~"), "-_.~");
    }

    #[test]
    fn percent_encode_empty() {
        assert_eq!(percent_encode(""), "");
    }

    // --- extract_baidu_link ---

    #[test]
    fn extract_baidu_link_valid() {
        let html = r#"<a href="https://example.com/news">Breaking News</a>"#;
        let result = extract_baidu_link(html);
        assert_eq!(result, Some(("Breaking News".into(), "https://example.com/news".into())));
    }

    #[test]
    fn extract_baidu_link_relative_url() {
        let html = r#"<a href="/news/123">Title</a>"#;
        let result = extract_baidu_link(html);
        assert_eq!(result, Some(("Title".into(), "https://www.baidu.com/news/123".into())));
    }

    #[test]
    fn extract_baidu_link_no_href() {
        let html = "<a>no href</a>";
        assert_eq!(extract_baidu_link(html), None);
    }

    #[test]
    fn extract_baidu_link_empty_title() {
        let html = r#"<a href="https://example.com"></a>"#;
        assert_eq!(extract_baidu_link(html), None);
    }

    #[test]
    fn extract_baidu_link_no_anchor() {
        assert_eq!(extract_baidu_link("no tags"), None);
    }

    #[test]
    fn extract_baidu_link_single_quotes() {
        let html = "<a href='https://example.com'>Title</a>";
        let result = extract_baidu_link(html);
        assert_eq!(result, Some(("Title".into(), "https://example.com".into())));
    }

    // --- extract_baidu_text_between ---

    #[test]
    fn extract_baidu_text_between_found() {
        let html = r#"<div class="content"><p>Hello World</p></div>"#;
        let result = extract_baidu_text_between(html, &["content"]);
        assert_eq!(result, Some("Hello World".into()));
    }

    #[test]
    fn extract_baidu_text_between_not_found() {
        let html = r#"<div class="other">text</div>"#;
        let result = extract_baidu_text_between(html, &["content"]);
        assert_eq!(result, None);
    }

    #[test]
    fn extract_baidu_text_between_multiple_classes() {
        let html = r#"<div class="b">found</div>"#;
        let result = extract_baidu_text_between(html, &["a", "b", "c"]);
        assert_eq!(result, Some("found".into()));
    }

    // --- extract_baidu_source ---

    #[test]
    fn extract_baidu_source_with_time() {
        let html = r#"<span class="news-source">Reuters 14:30</span>"#;
        let result = extract_baidu_source(html);
        assert_eq!(result, Some(("Reuters".into(), "14:30".into())));
    }

    #[test]
    fn extract_baidu_source_no_marker() {
        assert_eq!(extract_baidu_source("<div>plain</div>"), None);
    }

    // --- extract_baidu_plain_text ---

    #[test]
    fn extract_baidu_plain_text_valid() {
        let html = "<p>Hello &amp; World</p>";
        assert_eq!(extract_baidu_plain_text(html), Some("Hello & World".into()));
    }

    #[test]
    fn extract_baidu_plain_text_empty() {
        assert_eq!(extract_baidu_plain_text("<br>"), None);
    }

    // --- extract_rss_tag ---

    #[test]
    fn extract_rss_tag_valid() {
        let xml = "<title>Breaking News</title>";
        assert_eq!(extract_rss_tag(xml, "title"), Some("Breaking News".into()));
    }

    #[test]
    fn extract_rss_tag_cdata() {
        let xml = "<title><![CDATA[News Title]]></title>";
        assert_eq!(extract_rss_tag(xml, "title"), Some("News Title".into()));
    }

    #[test]
    fn extract_rss_tag_missing() {
        let xml = "<description>desc</description>";
        assert_eq!(extract_rss_tag(xml, "title"), None);
    }

    #[test]
    fn extract_rss_tag_empty() {
        let xml = "<title></title>";
        assert_eq!(extract_rss_tag(xml, "title"), None);
    }

    // --- normalize_rss_date ---

    #[test]
    fn normalize_rss_date_rfc2822() {
        let result = normalize_rss_date("Wed, 03 Jun 2026 00:36:00 GMT");
        assert_eq!(result, "2026-06-03");
    }

    #[test]
    fn normalize_rss_date_iso() {
        assert_eq!(normalize_rss_date("2026-06-03"), "2026-06-03");
    }

    #[test]
    fn normalize_rss_date_invalid() {
        assert_eq!(normalize_rss_date("not a date"), "");
    }

    #[test]
    fn normalize_rss_date_empty() {
        assert_eq!(normalize_rss_date(""), "");
    }
}
