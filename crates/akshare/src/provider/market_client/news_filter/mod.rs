use std::collections::HashSet;

#[cfg(test)]
use anyhow::Context;
use chrono::{DateTime, Duration as ChronoDuration, NaiveDate, Utc};

use super::{NewsItem, wire};
mod classifiers;
pub(crate) use classifiers::*;

pub(crate) fn build_dated_news_query(
    base: &str,
    _start_date: Option<&str>,
    _end_date: Option<&str>,
) -> String {
    base.trim().to_string()
}

pub(crate) fn within_date_window(
    published_at: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> bool {
    if published_at.trim().is_empty() {
        return true;
    }
    let Some(normalized) = normalized_news_date(published_at).or_else(|| {
        published_at
            .get(0..10)
            .filter(|prefix| prefix.len() == 10)
            .map(str::to_string)
    }) else {
        return true;
    };
    start_date.is_none_or(|start| normalized.as_str() >= start)
        && end_date.is_none_or(|end| normalized.as_str() <= end)
}

pub(crate) fn news_search_dedup_key(item: &NewsItem) -> String {
    format!(
        "{}|{}|{}",
        item.title.trim().to_lowercase(),
        item.source.trim().to_lowercase(),
        item.url.clone().unwrap_or_default().trim().to_lowercase()
    )
}

pub(crate) fn merge_ranked_news(
    items: Vec<NewsItem>,
    limit: usize,
    start_date: Option<&str>,
    end_date: Option<&str>,
    keywords: &[String],
) -> Vec<NewsItem> {
    let mut deduped = Vec::new();
    let mut seen = HashSet::new();
    for item in items {
        if !within_date_window(&item.published_at, start_date, end_date) {
            continue;
        }
        let dedupe_key = format!(
            "{}|{}",
            normalize_news_text(&item.title),
            normalized_news_date(&item.published_at).unwrap_or_else(|| item
                .published_at
                .get(0..10)
                .unwrap_or_default()
                .to_string())
        );
        if seen.insert(dedupe_key) {
            deduped.push(item);
        }
    }
    deduped.sort_by(|left, right| {
        let left_score = news_item_rank(left, keywords);
        let right_score = news_item_rank(right, keywords);
        right_score
            .cmp(&left_score)
            .then_with(|| right.published_at.cmp(&left.published_at))
            .then_with(|| left.title.cmp(&right.title))
    });
    deduped.truncate(limit.max(8));
    deduped
}

pub(crate) fn news_item_rank(item: &NewsItem, keywords: &[String]) -> i32 {
    let mut score = source_priority(&item.source);
    let normalized_title = normalize_news_text(&item.title);
    let normalized_summary = normalize_news_text(&item.summary);
    let combined = format!("{normalized_title} {normalized_summary}");
    let title_primary_keyword_hits = keywords
        .iter()
        .map(|value| normalize_news_text(value))
        .filter(|value| value.len() >= 2)
        .filter(|keyword| normalized_title.contains(keyword))
        .count();
    let combined_primary_keyword_hits = keywords
        .iter()
        .map(|value| normalize_news_text(value))
        .filter(|value| value.len() >= 2)
        .filter(|keyword| combined.contains(keyword))
        .count();
    if is_sec_filing_item(item) {
        score -= 20;
    }
    if title_or_summary_has_high_value_company_event(&normalized_title, &normalized_summary) {
        score += 34;
    }
    if title_or_summary_has_low_value_corporate_filing_noise(&normalized_title, &normalized_summary)
    {
        score -= 42;
    }
    if url_is_ir_landing_page(item.url.as_deref().unwrap_or_default()) {
        score -= 32;
    }
    for keyword in keywords
        .iter()
        .map(|value| normalize_news_text(value))
        .filter(|value| value.len() >= 2)
    {
        if is_sec_filing_item(item) && is_sec_biasing_keyword(&keyword) {
            continue;
        }
        if normalized_title.contains(&keyword) {
            score += 18;
        } else if combined.contains(&keyword) {
            score += 8;
        }
    }
    if item
        .url
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
    {
        score += 2;
    }
    if normalized_news_date(&item.published_at).is_some() {
        score += 4;
    }
    if title_is_generic_market_wrap(&normalized_title) {
        score -= 10;
    }
    if title_is_reference_or_overview_page(&normalized_title, &normalized_summary) {
        score -= 18;
    }
    if url_is_quote_or_overview_page(item.url.as_deref().unwrap_or_default()) {
        score -= 28;
    }
    if mentions_competitor_without_primary_company_focus(&normalized_title, &combined, keywords) {
        score -= 8;
    }
    if title_primary_keyword_hits == 0 && combined_primary_keyword_hits > 0 {
        score -= 12;
    }
    if title_primary_keyword_hits == 0
        && mentions_secondary_reference_only(&normalized_title, &normalized_summary, keywords)
    {
        score -= 10;
    }
    score
}

pub(crate) fn source_priority(source: &str) -> i32 {
    let normalized = source.to_ascii_lowercase();
    if normalized.contains("ir.")
        || normalized.contains("investor")
        || normalized.contains("relations")
    {
        58
    } else if normalized.contains("hkex") || normalized.contains("hkexnews") {
        56
    } else if normalized.contains("reuters")
        || normalized.contains("bloomberg")
        || normalized.contains("ft")
        || normalized.contains("wsj")
        || normalized.contains("nikkei")
        || normalized.contains("cnbc")
    {
        52
    } else if normalized.contains("sec") {
        26
    } else if normalized.contains("aastocks") {
        46
    } else if normalized.contains("eastmoney") {
        44
    } else if normalized.contains("sse") {
        42
    } else if normalized.contains("etnet") {
        38
    } else if normalized.contains("futunn")
        || normalized.contains("hstong")
        || normalized.contains("xueqiu")
    {
        34
    } else if normalized.contains("google") {
        30
    } else if normalized.contains("tushare") {
        28
    } else {
        24
    }
}

pub(crate) fn extract_site_name_from_url(url: &str) -> Option<&str> {
    let trimmed = url.trim();
    let without_scheme = trimmed
        .strip_prefix("https://")
        .or_else(|| trimmed.strip_prefix("http://"))
        .unwrap_or(trimmed);
    let host = without_scheme
        .split('/')
        .next()?
        .trim()
        .trim_start_matches("www.");
    if host.is_empty() { None } else { Some(host) }
}

pub(crate) fn normalize_news_text(value: &str) -> String {
    value.split_whitespace().collect::<String>().to_lowercase()
}

pub fn normalized_news_date(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    [
        "%Y-%m-%d",
        "%Y%m%d",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y年%m月%d日",
        "%Y年%m月%d日 %H:%M",
        "%Y年%m月%d日 %H:%M:%S",
    ]
    .iter()
    .find_map(|format| {
        NaiveDate::parse_from_str(trimmed, format)
            .ok()
            .map(|date| date.format("%Y-%m-%d").to_string())
            .or_else(|| {
                chrono::NaiveDateTime::parse_from_str(trimmed, format)
                    .ok()
                    .map(|datetime| datetime.date().format("%Y-%m-%d").to_string())
            })
    })
    .or_else(|| {
        trimmed
            .get(0..10)
            .filter(|prefix| prefix.chars().nth(4) == Some('-'))
            .map(str::to_string)
    })
    .or_else(|| normalize_relative_news_date(trimmed, Utc::now()))
}

pub(crate) fn gdelt_timestamp_to_published_at(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() < 14 || !trimmed.chars().take(14).all(|ch| ch.is_ascii_digit()) {
        return trimmed.to_string();
    }
    format!(
        "{}-{}-{} {}:{}:{}",
        &trimmed[0..4],
        &trimmed[4..6],
        &trimmed[6..8],
        &trimmed[8..10],
        &trimmed[10..12],
        &trimmed[12..14]
    )
}

pub(crate) fn normalize_relative_news_date(value: &str, now: DateTime<Utc>) -> Option<String> {
    let lower = value.trim().to_ascii_lowercase();
    if lower.is_empty() {
        return None;
    }

    if let Some((amount, unit)) = lower
        .split_once(' ')
        .and_then(|(amount, rest)| amount.parse::<i64>().ok().map(|value| (value, rest.trim())))
    {
        let date = if unit.starts_with("minute") || unit.starts_with("min") {
            Some((now - ChronoDuration::minutes(amount)).date_naive())
        } else if unit.starts_with("hour") || unit.starts_with("hr") {
            Some((now - ChronoDuration::hours(amount)).date_naive())
        } else if unit.starts_with("day") {
            Some((now - ChronoDuration::days(amount)).date_naive())
        } else if unit.starts_with("week") {
            Some((now - ChronoDuration::weeks(amount)).date_naive())
        } else {
            None
        }?;
        return Some(date.format("%Y-%m-%d").to_string());
    }

    if lower == "yesterday" {
        return Some(
            (now.date_naive() - ChronoDuration::days(1))
                .format("%Y-%m-%d")
                .to_string(),
        );
    }

    if lower == "today" {
        return Some(now.date_naive().format("%Y-%m-%d").to_string());
    }

    None
}

pub(crate) fn latest_metric_value(units: &wire::MetricUnits) -> Option<f64> {
    latest_preferred_metric(units, MetricSelection::LatestFiled).map(|item| item.val)
}

pub(crate) fn latest_strict_annual_metric_value(units: &wire::MetricUnits) -> Option<f64> {
    latest_preferred_metric(units, MetricSelection::StrictAnnual).map(|item| item.val)
}

pub(crate) fn latest_annual_metric_value(units: &wire::MetricUnits) -> Option<f64> {
    latest_preferred_metric(units, MetricSelection::Annual).map(|item| item.val)
}

pub(crate) fn latest_instant_metric_value(units: &wire::MetricUnits) -> Option<f64> {
    latest_preferred_metric(units, MetricSelection::Instant).map(|item| item.val)
}

#[derive(Clone, Copy)]
pub(crate) enum MetricSelection {
    LatestFiled,
    StrictAnnual,
    Annual,
    Instant,
}

pub(crate) fn latest_preferred_metric(
    units: &wire::MetricUnits,
    selection: MetricSelection,
) -> Option<&wire::MetricValue> {
    units
        .usd
        .as_ref()
        .or(units.shares.as_ref())
        .and_then(|values| {
            values
                .iter()
                .filter(|item| matches!(item.form.as_deref(), Some("10-K") | Some("10-Q")))
                .filter(|item| match selection {
                    MetricSelection::LatestFiled => true,
                    MetricSelection::StrictAnnual | MetricSelection::Annual => {
                        item.start.is_some()
                            && item.end.is_some()
                            && matches!(item.form.as_deref(), Some("10-K"))
                            && matches!(item.fp.as_deref(), Some("FY"))
                    }
                    MetricSelection::Instant => item.start.is_none() && item.end.is_some(),
                })
                .max_by_key(|item| {
                    (
                        item.end.as_deref().unwrap_or_default(),
                        item.filed.as_str(),
                        item.fp.as_deref().unwrap_or_default(),
                    )
                })
        })
        .or_else(|| {
            matches!(selection, MetricSelection::Annual).then(|| {
                units
                    .usd
                    .as_ref()
                    .or(units.shares.as_ref())
                    .and_then(|values| {
                        values
                            .iter()
                            .filter(|item| {
                                matches!(item.form.as_deref(), Some("10-K") | Some("10-Q"))
                                    && item.start.is_some()
                                    && item.end.is_some()
                            })
                            .max_by_key(|item| {
                                (
                                    item.end.as_deref().unwrap_or_default(),
                                    item.filed.as_str(),
                                    item.fp.as_deref().unwrap_or_default(),
                                )
                            })
                    })
            })?
        })
}

#[cfg(test)]
pub(crate) fn eastmoney_price(value: Option<f64>) -> anyhow::Result<f64> {
    value.context("eastmoney price field missing")
}

#[cfg(test)]
pub(crate) fn format_eastmoney_trade_date(value: Option<i64>) -> String {
    value
        .and_then(|timestamp| chrono::DateTime::from_timestamp(timestamp, 0))
        .map(|datetime| datetime.format("%Y%m%d").to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod news_filter_tests {
    use super::*;

    // --- normalized_news_date ---

    #[test]
    fn normalized_news_date_iso() {
        assert_eq!(
            normalized_news_date("2026-06-21"),
            Some("2026-06-21".into())
        );
    }

    #[test]
    fn normalized_news_date_compact() {
        assert_eq!(normalized_news_date("20260621"), Some("2026-06-21".into()));
    }

    #[test]
    fn normalized_news_date_with_time() {
        assert_eq!(
            normalized_news_date("2026-06-21 14:30:00"),
            Some("2026-06-21".into())
        );
    }

    #[test]
    fn normalized_news_date_chinese() {
        assert_eq!(
            normalized_news_date("2026年06月21日"),
            Some("2026-06-21".into())
        );
    }

    #[test]
    fn normalized_news_date_empty() {
        assert_eq!(normalized_news_date(""), None);
        assert_eq!(normalized_news_date("  "), None);
    }

    #[test]
    fn normalized_news_date_relative_today() {
        let result = normalized_news_date("today");
        assert!(result.is_some());
    }

    #[test]
    fn normalized_news_date_relative_yesterday() {
        let result = normalized_news_date("yesterday");
        assert!(result.is_some());
    }

    #[test]
    fn normalized_news_date_relative_days() {
        let result = normalized_news_date("3 days ago");
        assert!(result.is_some());
    }

    // --- normalize_relative_news_date ---

    #[test]
    fn normalize_relative_today() {
        let now = Utc::now();
        let result = normalize_relative_news_date("today", now);
        assert_eq!(
            result,
            Some(now.date_naive().format("%Y-%m-%d").to_string())
        );
    }

    #[test]
    fn normalize_relative_yesterday() {
        let now = Utc::now();
        let result = normalize_relative_news_date("yesterday", now);
        let expected = (now.date_naive() - ChronoDuration::days(1))
            .format("%Y-%m-%d")
            .to_string();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn normalize_relative_days_ago() {
        let now = Utc::now();
        let result = normalize_relative_news_date("5 days ago", now);
        let expected = (now.date_naive() - ChronoDuration::days(5))
            .format("%Y-%m-%d")
            .to_string();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn normalize_relative_hours_ago() {
        let now = Utc::now();
        let result = normalize_relative_news_date("2 hours ago", now);
        let expected = (now - ChronoDuration::hours(2))
            .date_naive()
            .format("%Y-%m-%d")
            .to_string();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn normalize_relative_empty() {
        assert_eq!(normalize_relative_news_date("", Utc::now()), None);
    }

    // --- gdelt_timestamp_to_published_at ---

    #[test]
    fn gdelt_timestamp_basic() {
        assert_eq!(
            gdelt_timestamp_to_published_at("20260621143000"),
            "2026-06-21 14:30:00"
        );
    }

    #[test]
    fn gdelt_timestamp_short() {
        assert_eq!(gdelt_timestamp_to_published_at("short"), "short");
    }

    // --- normalize_news_text ---

    #[test]
    fn normalize_news_text_basic() {
        assert_eq!(normalize_news_text("Hello  World"), "helloworld");
    }

    #[test]
    fn normalize_news_text_whitespace() {
        assert_eq!(normalize_news_text("  a  b  c  "), "abc");
    }

    // --- build_dated_news_query ---

    #[test]
    fn build_dated_news_query_basic() {
        assert_eq!(build_dated_news_query("Apple Inc", None, None), "Apple Inc");
    }

    #[test]
    fn build_dated_news_query_trims() {
        assert_eq!(build_dated_news_query("  hello  ", None, None), "hello");
    }

    // --- within_date_window ---

    #[test]
    fn within_date_window_empty_date() {
        assert!(within_date_window(
            "",
            Some("2025-01-01"),
            Some("2025-12-31")
        ));
    }

    #[test]
    fn within_date_window_in_range() {
        assert!(within_date_window(
            "2025-06-15",
            Some("2025-01-01"),
            Some("2025-12-31")
        ));
    }

    #[test]
    fn within_date_window_before_start() {
        assert!(!within_date_window(
            "2024-06-15",
            Some("2025-01-01"),
            Some("2025-12-31")
        ));
    }

    #[test]
    fn within_date_window_after_end() {
        assert!(!within_date_window(
            "2026-06-15",
            Some("2025-01-01"),
            Some("2025-12-31")
        ));
    }

    #[test]
    fn within_date_window_no_bounds() {
        assert!(within_date_window("2025-06-15", None, None));
    }

    #[test]
    fn within_date_window_only_start() {
        assert!(within_date_window("2025-06-15", Some("2025-01-01"), None));
        assert!(!within_date_window("2024-06-15", Some("2025-01-01"), None));
    }

    #[test]
    fn within_date_window_only_end() {
        assert!(within_date_window("2025-06-15", None, Some("2025-12-31")));
        assert!(!within_date_window("2026-06-15", None, Some("2025-12-31")));
    }

    // --- news_search_dedup_key ---

    #[test]
    fn news_search_dedup_key_basic() {
        let item = NewsItem {
            title: "Apple Reports Earnings".to_string(),
            source: "Reuters".to_string(),
            url: Some("http://example.com".to_string()),
            published_at: "2025-01-15".to_string(),
            summary: "Summary".to_string(),
        };
        let key = news_search_dedup_key(&item);
        assert!(key.contains("apple reports earnings"));
        assert!(key.contains("reuters"));
    }

    // --- source_priority ---

    #[test]
    fn source_priority_reuters() {
        assert!(source_priority("Reuters") > 0);
    }

    #[test]
    fn source_priority_bloomberg() {
        assert!(source_priority("Bloomberg") > 0);
    }

    #[test]
    fn source_priority_unknown() {
        // Default priority is 24 for unrecognized sources
        assert_eq!(source_priority("unknown-blog"), 24);
    }

    // --- title_is_generic_market_wrap ---

    #[test]
    fn title_is_generic_market_wrap_true() {
        assert!(title_is_generic_market_wrap("marketswrap"));
        assert!(title_is_generic_market_wrap("dowjonesfuturesrise"));
        assert!(title_is_generic_market_wrap("stockmarkettoday"));
    }

    #[test]
    fn title_is_generic_market_wrap_false() {
        assert!(!title_is_generic_market_wrap("applerecordrevenue"));
    }

    // --- title_is_reference_or_overview_page ---

    #[test]
    fn title_is_reference_overview_true() {
        assert!(title_is_reference_or_overview_page("stockoverview", ""));
        assert!(title_is_reference_or_overview_page(
            "",
            "engagesinthedesigndevelopmentmanufactureandsale"
        ));
    }

    #[test]
    fn title_is_reference_overview_false() {
        assert!(!title_is_reference_or_overview_page(
            "earningsreport",
            "quarterlyresults"
        ));
    }

    // --- url_is_quote_or_overview_page ---

    #[test]
    fn url_is_quote_yahoo() {
        assert!(url_is_quote_or_overview_page(
            "https://finance.yahoo.com/quote/AAPL"
        ));
    }

    #[test]
    fn url_is_quote_nasdaq() {
        assert!(url_is_quote_or_overview_page(
            "https://www.nasdaq.com/market-activity/stocks/aapl"
        ));
    }

    #[test]
    fn url_is_quote_normal_article() {
        assert!(!url_is_quote_or_overview_page(
            "https://www.reuters.com/technology/apple-earnings"
        ));
    }

    #[test]
    fn url_is_quote_xueqiu() {
        assert!(url_is_quote_or_overview_page("https://xueqiu.com/s/AAPL"));
    }

    // --- url_is_ir_landing_page ---

    #[test]
    fn url_is_ir_landing_page_investor_subdomain() {
        assert!(url_is_ir_landing_page(
            "https://investor.apple.com/investor-relations/"
        ));
    }

    #[test]
    fn url_is_ir_landing_page_normal() {
        assert!(!url_is_ir_landing_page("https://www.apple.com/newsroom"));
    }

    // --- is_sec_filing_item ---

    #[test]
    fn is_sec_filing_item_8k() {
        let item = NewsItem {
            title: "Apple 8-K Filing".to_string(),
            source: "SEC".to_string(),
            url: Some("https://sec.gov/filing".to_string()),
            published_at: "2025-01-15".to_string(),
            summary: "".to_string(),
        };
        assert!(is_sec_filing_item(&item));
    }

    #[test]
    fn is_sec_filing_item_normal_news() {
        let item = NewsItem {
            title: "Apple Reports Record Revenue".to_string(),
            source: "Reuters".to_string(),
            url: Some("https://reuters.com/article".to_string()),
            published_at: "2025-01-15".to_string(),
            summary: "".to_string(),
        };
        assert!(!is_sec_filing_item(&item));
    }

    // --- title_or_summary_has_high_value_company_event ---

    #[test]
    fn high_value_event_earnings() {
        assert!(title_or_summary_has_high_value_company_event(
            "earningsreport",
            ""
        ));
    }

    #[test]
    fn high_value_event_buyback() {
        assert!(title_or_summary_has_high_value_company_event(
            "",
            "sharebuybackannouncement"
        ));
    }

    #[test]
    fn high_value_event_chinese() {
        assert!(title_or_summary_has_high_value_company_event("财报", ""));
    }

    #[test]
    fn high_value_event_false() {
        assert!(!title_or_summary_has_high_value_company_event(
            "regularnews",
            "nothinginteresting"
        ));
    }

    // --- title_or_summary_has_low_value_corporate_filing_noise ---

    #[test]
    fn low_value_noise_proxy_form() {
        assert!(title_or_summary_has_low_value_corporate_filing_noise(
            "proxyform",
            ""
        ));
    }

    #[test]
    fn low_value_noise_false() {
        assert!(!title_or_summary_has_low_value_corporate_filing_noise(
            "earningsreport",
            "quarterlyresults"
        ));
    }

    // --- merge_ranked_news ---

    #[test]
    fn merge_ranked_news_basic() {
        let items = vec![
            NewsItem {
                title: "Apple Earnings".to_string(),
                source: "Reuters".to_string(),
                url: Some("http://test1.com".to_string()),
                published_at: "2025-06-15".to_string(),
                summary: "".to_string(),
            },
            NewsItem {
                title: "Apple Dividend".to_string(),
                source: "Bloomberg".to_string(),
                url: Some("http://test2.com".to_string()),
                published_at: "2025-06-14".to_string(),
                summary: "".to_string(),
            },
        ];
        let result = merge_ranked_news(items, 10, None, None, &[]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn merge_ranked_news_dedup() {
        let items = vec![
            NewsItem {
                title: "Same Title".to_string(),
                source: "Reuters".to_string(),
                url: Some("http://test1.com".to_string()),
                published_at: "2025-06-15".to_string(),
                summary: "".to_string(),
            },
            NewsItem {
                title: "Same Title".to_string(),
                source: "Bloomberg".to_string(),
                url: Some("http://test2.com".to_string()),
                published_at: "2025-06-15".to_string(),
                summary: "".to_string(),
            },
        ];
        let result = merge_ranked_news(items, 10, None, None, &[]);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn merge_ranked_news_date_filter() {
        let items = vec![NewsItem {
            title: "Old News".to_string(),
            source: "Reuters".to_string(),
            url: Some("http://test.com".to_string()),
            published_at: "2020-01-01".to_string(),
            summary: "".to_string(),
        }];
        let result = merge_ranked_news(items, 10, Some("2025-01-01"), Some("2025-12-31"), &[]);
        assert_eq!(result.len(), 0);
    }

    // --- normalize_relative_news_date additional ---

    #[test]
    fn normalize_relative_minutes_ago() {
        let now = Utc::now();
        let result = normalize_relative_news_date("30 minutes ago", now);
        assert!(result.is_some());
    }

    #[test]
    fn normalize_relative_weeks_ago() {
        let now = Utc::now();
        let result = normalize_relative_news_date("2 weeks ago", now);
        let expected = (now.date_naive() - ChronoDuration::weeks(2))
            .format("%Y-%m-%d")
            .to_string();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn normalize_relative_unknown_unit() {
        assert_eq!(
            normalize_relative_news_date("5 months ago", Utc::now()),
            None
        );
    }

    // --- gdelt_timestamp additional ---

    #[test]
    fn gdelt_timestamp_exact_14_digits() {
        assert_eq!(
            gdelt_timestamp_to_published_at("20260101000000"),
            "2026-01-01 00:00:00"
        );
    }

    #[test]
    fn gdelt_timestamp_non_digit() {
        assert_eq!(
            gdelt_timestamp_to_published_at("2026010100000x"),
            "2026010100000x"
        );
    }

    // --- extract_site_name_from_url ---

    #[test]
    fn extract_site_name_https() {
        assert_eq!(
            extract_site_name_from_url("https://www.reuters.com/article/123"),
            Some("reuters.com")
        );
    }

    #[test]
    fn extract_site_name_http() {
        assert_eq!(
            extract_site_name_from_url("http://finance.yahoo.com/quote/AAPL"),
            Some("finance.yahoo.com")
        );
    }

    #[test]
    fn extract_site_name_no_scheme() {
        assert_eq!(
            extract_site_name_from_url("reuters.com/article"),
            Some("reuters.com")
        );
    }

    #[test]
    fn extract_site_name_strips_www() {
        assert_eq!(
            extract_site_name_from_url("https://www.bloomberg.com/news"),
            Some("bloomberg.com")
        );
    }

    #[test]
    fn extract_site_name_empty() {
        assert_eq!(extract_site_name_from_url(""), None);
    }

    #[test]
    fn extract_site_name_just_host() {
        assert_eq!(
            extract_site_name_from_url("https://apple.com"),
            Some("apple.com")
        );
    }

    // --- is_sec_biasing_keyword ---

    #[test]
    fn is_sec_biasing_keyword_filing() {
        assert!(is_sec_biasing_keyword("filing"));
    }

    #[test]
    fn is_sec_biasing_keyword_sec() {
        assert!(is_sec_biasing_keyword("sec"));
    }

    #[test]
    fn is_sec_biasing_keyword_form() {
        assert!(is_sec_biasing_keyword("form"));
    }

    #[test]
    fn is_sec_biasing_keyword_8k() {
        assert!(is_sec_biasing_keyword("8-k"));
    }

    #[test]
    fn is_sec_biasing_keyword_normal() {
        assert!(!is_sec_biasing_keyword("technology"));
    }

    #[test]
    fn is_sec_biasing_keyword_earnings() {
        assert!(!is_sec_biasing_keyword("earnings"));
    }

    // --- news_item_rank ---

    #[test]
    fn news_item_rank_high_value_event() {
        let item = NewsItem {
            title: "Apple Earnings Report".to_string(),
            source: "Reuters".to_string(),
            url: Some("http://test.com".to_string()),
            published_at: "2025-06-15".to_string(),
            summary: "".to_string(),
        };
        let rank = news_item_rank(&item, &[]);
        // Reuters has high source priority, earnings is high-value event
        assert!(rank > 0, "expected positive rank, got {}", rank);
    }

    #[test]
    fn news_item_rank_keyword_match() {
        let item = NewsItem {
            title: "Apple Revenue Growth".to_string(),
            source: "Reuters".to_string(),
            url: Some("http://test.com".to_string()),
            published_at: "2025-06-15".to_string(),
            summary: "".to_string(),
        };
        let with_kw = news_item_rank(&item, &["Apple".into()]);
        let without_kw = news_item_rank(&item, &[]);
        assert!(with_kw >= without_kw);
    }
}
