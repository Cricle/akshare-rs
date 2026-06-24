use super::super::NewsItem;
use std::collections::HashSet;

pub(crate) fn sanitize_hk_company_news_query(
    query: Option<&str>,
    standard_code: &str,
    short_code: &str,
    company_name: &str,
    primary_name: &str,
    english_alias: &str,
    aliases: &[String],
) -> Option<String> {
    let raw = query?.trim();
    if raw.is_empty() {
        return None;
    }

    let normalized_allowed = std::iter::once(standard_code.to_string())
        .chain((!short_code.trim().is_empty()).then_some(short_code.to_string()))
        .chain([
            company_name.to_string(),
            primary_name.to_string(),
            english_alias.to_string(),
        ])
        .chain(aliases.iter().cloned())
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let allowed_terms = normalized_allowed
        .iter()
        .flat_map(|value| hk_query_allowed_terms(value))
        .collect::<HashSet<_>>();
    let safe_scene_terms = [
        "or",
        "and",
        "company",
        "specific",
        "company-specific",
        "news",
        "investor",
        "discussion",
        "public",
        "market",
        "sentiment",
        "announcement",
        "announcements",
        "earnings",
        "results",
        "quarterly",
        "annual",
        "deliveries",
        "guidance",
        "ir",
        "hkex",
        "公司",
        "新闻",
        "公告",
        "业绩",
        "财报",
        "交付",
        "销量",
        "融资",
        "投资者",
        "讨论",
        "市场",
        "情绪",
    ]
    .into_iter()
    .collect::<HashSet<_>>();

    let kept = raw
        .split_whitespace()
        .filter(|token| {
            let trimmed = token.trim_matches(|ch: char| {
                !ch.is_alphanumeric() && !is_cjk(ch) && !matches!(ch, '-' | '_' | '.' | '/')
            });
            if trimmed.is_empty() {
                return false;
            }
            let lower = trimmed.to_ascii_lowercase();
            if safe_scene_terms.contains(lower.as_str()) {
                return true;
            }
            if trimmed.chars().all(|ch| ch.is_ascii_digit()) {
                return trimmed == standard_code || trimmed == short_code;
            }
            if trimmed.chars().all(|ch| ch.is_ascii_alphabetic()) && trimmed.len() <= 2 {
                return false;
            }
            if trimmed.chars().any(is_cjk) || trimmed.chars().any(|ch| ch.is_ascii_alphabetic()) {
                return allowed_terms.contains(&lower);
            }
            true
        })
        .collect::<Vec<_>>();

    let cleaned = kept.join(" ").trim().to_string();
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}

fn is_cjk(ch: char) -> bool {
    matches!(ch, '\u{4e00}'..='\u{9fff}' | '\u{3400}'..='\u{4dbf}' | '\u{f900}'..='\u{faff}')
}

fn hk_query_allowed_terms(value: &str) -> Vec<String> {
    let tokens: Vec<&str> = value.split_whitespace().collect();
    let mut kept = Vec::new();
    for token in tokens {
        if token.chars().any(is_cjk) {
            kept.push(token.to_string());
            continue;
        }
        let lower = token.to_ascii_lowercase();
        let is_noise = lower.len() < 2
            || lower.starts_with("site:")
            || lower.ends_with(".com")
            || lower.ends_with(".hk")
            || lower.ends_with(".hk)")
            || lower.contains("hkex")
            || lower.contains("aastocks")
            || lower.contains("eastmoney");
        if !is_noise {
            kept.push(token.to_string());
        }
    }
    kept
}

pub(crate) fn parse_hkex_title_search_results(html: &str) -> Vec<NewsItem> {
    let row_regex = regex::Regex::new(r#"(?s)<tr>(.*?)</tr>"#).expect("valid hkex row regex");
    let time_regex = regex::Regex::new(r#"([0-9]{2}/[0-9]{2}/[0-9]{4})\s+([0-9]{2}:[0-9]{2})"#)
        .expect("valid hkex time regex");
    let headline_regex = regex::Regex::new(r#"(?s)<div class="headline">(.*?)<br/>"#)
        .expect("valid hkex headline regex");
    let link_regex = regex::Regex::new(r#"(?s)<a href="([^"]+)"[^>]*>(.*?)</a>"#)
        .expect("valid hkex link regex");
    let tag_regex = regex::Regex::new(r"<[^>]+>").expect("valid html tag regex");

    row_regex
        .captures_iter(html)
        .filter_map(|captures| {
            let row = captures.get(1)?.as_str();
            let time_caps = time_regex.captures(row)?;
            let date = time_caps.get(1)?.as_str();
            let time = time_caps.get(2)?.as_str();
            let category = headline_regex.captures(row)?.get(1)?.as_str().trim();
            let link_caps = link_regex.captures(row)?;
            let relative_url = link_caps.get(1)?.as_str().trim();
            let raw_title = link_caps.get(2)?.as_str();
            let title = html_unescape(tag_regex.replace_all(raw_title, "").trim());
            if title.is_empty() {
                return None;
            }
            let normalized_date = normalize_hkex_release_date(date)?;
            Some(NewsItem {
                published_at: format!("{normalized_date} {time}"),
                title,
                summary: category.to_string(),
                source: "hkexnews.hk".to_string(),
                url: Some(format!("https://www1.hkexnews.hk{relative_url}")),
            })
        })
        .collect()
}

fn html_unescape(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

fn normalize_hkex_release_date(value: &str) -> Option<String> {
    let mut parts = value.split('/');
    let day = parts.next()?;
    let month = parts.next()?;
    let year = parts.next()?;
    Some(format!("{year}-{month}-{day}"))
}

pub(super) fn hkex_item_is_high_value(item: &NewsItem) -> bool {
    let combined =
        super::super::news_filter::normalize_news_text(&format!("{} {}", item.title, item.summary));
    let positive_markers = [
        "annualresults",
        "interimresults",
        "quarterlyresults",
        "fiscalyear",
        "earnings",
        "financialresults",
        "resultsannouncement",
        "voluntaryannouncement",
        "vehicledelivery",
        "vehicledeliveries",
        "deliveryresults",
        "sales",
        "deliveries",
        "businessupdate",
        "tradingupdate",
        "profitwarning",
        "dividend",
        "interimdividend",
        "finaldividend",
        "sharebuyback",
        "subscription",
        "placing",
        "acquisition",
        "disposal",
        "jointventure",
    ];
    let negative_markers = [
        "monthlyreturnofequityissuer",
        "nextdaydisclosurereturn",
        "proxyform",
        "formofproxy",
        "notificationletter",
        "circular",
        "pollresults",
        "changeofdirector",
        "listofdirectors",
        "closureofregisterofmembers",
        "arrangementintoelectronicdissemination",
        "annualgeneralmeeting",
    ];

    positive_markers
        .iter()
        .any(|marker| combined.contains(marker))
        && !negative_markers
            .iter()
            .any(|marker| combined.contains(marker))
}
