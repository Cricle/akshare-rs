use super::super::NewsItem;
use super::super::news_filter::normalize_news_text;

pub(super) fn is_direct_company_news_match(
    item: &NewsItem,
    symbol: &str,
    company_name: &str,
    company_title: &str,
) -> bool {
    if is_low_signal_us_company_news_item(item) {
        return false;
    }
    if item.source.eq_ignore_ascii_case("SEC EDGAR") {
        return is_high_signal_sec_company_filing(item);
    }
    let combined = normalize_news_text(&format!("{} {}", item.title, item.summary));
    let company_terms = [
        normalize_news_text(symbol),
        normalize_news_text(company_name),
        normalize_news_text(company_title),
    ];
    let has_company_term = company_terms
        .iter()
        .filter(|term| !term.is_empty())
        .any(|term| combined.contains(term))
        || mentions_important_company_alias(&combined, symbol, company_name, company_title);
    if !has_company_term {
        return false;
    }
    // Trusted financial news sources that always cover major companies with
    // editorial context — accept without requiring a specific event keyword.
    if is_trusted_financial_source(item) {
        return true;
    }
    has_meaningful_us_company_event_signal(item)
}

fn is_low_signal_us_company_news_item(item: &NewsItem) -> bool {
    let normalized_title = normalize_news_text(&item.title);
    let normalized_summary = normalize_news_text(&item.summary);
    let url = item.url.as_deref().unwrap_or_default().to_ascii_lowercase();
    (normalized_title == "investorrelationsapple"
        || normalized_title.ends_with("investorrelations")
        || url.contains("investor-relations/default.aspx"))
        || (item.source.eq_ignore_ascii_case("SEC EDGAR")
            && (normalized_title.contains("144filing")
                || normalized_title.contains("4filing")
                || normalized_title.contains("3filing")
                || normalized_title.contains("5filing"))
            && !normalized_summary.contains("8-k")
            && !normalized_summary.contains("10-q")
            && !normalized_summary.contains("10-k"))
}

fn is_high_signal_sec_company_filing(item: &NewsItem) -> bool {
    let normalized_title = normalize_news_text(&item.title);
    normalized_title.contains("8-k")
        || normalized_title.contains("10-q")
        || normalized_title.contains("10-k")
        || normalized_title.contains("6-k")
        || normalized_title.contains("20-f")
}

/// Major financial news wires / publications that always carry editorial
/// context when they mention a public company by name.
fn is_trusted_financial_source(item: &NewsItem) -> bool {
    let source = item.source.to_ascii_lowercase();
    [
        "reuters",
        "bloomberg",
        "cnbc",
        "wsj",
        "wall street journal",
        "financial times",
        "ft.com",
        "barron",
        "marketwatch",
        "yahoo finance",
        "seeking alpha",
        "business insider",
        "fortune",
        "forbes",
        "associated press",
        "ap news",
        "dow jones",
        "benzinga",
        "investor's business daily",
        "ibd",
        "thestreet",
        "zacks",
        "morningstar",
    ]
    .iter()
    .any(|trusted| source.contains(trusted))
}

fn mentions_important_company_alias(
    combined: &str,
    symbol: &str,
    company_name: &str,
    company_title: &str,
) -> bool {
    let alias_candidates = [
        symbol.to_string(),
        company_name.to_string(),
        company_title.to_string(),
        company_name
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_string(),
        company_title
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_string(),
    ];
    alias_candidates
        .into_iter()
        .map(|value| normalize_news_text(&value))
        .filter(|value| value.len() >= 3)
        .any(|alias| combined.contains(&alias))
}

pub(super) fn has_meaningful_us_company_event_signal(item: &NewsItem) -> bool {
    let normalized_title = normalize_news_text(&item.title);
    let normalized_summary = normalize_news_text(&item.summary);
    let combined = format!("{normalized_title} {normalized_summary}");
    let signals = [
        "earnings",
        "quarterlyresults",
        "annualresults",
        "financialresults",
        "revenue",
        "guidance",
        "forecast",
        "outlook",
        "buyback",
        "sharebuyback",
        "dividend",
        "productlaunch",
        "iphone",
        "ipad",
        "mac",
        "services",
        "ai",
        "tariff",
        "antitrust",
        "supplier",
        "demand",
        "sales",
        "shipment",
        "8-k",
        "10-q",
        "10-k",
        "6-k",
        "20-f",
        // Additional market / price-moving signals
        "stock",
        "shares",
        "price",
        "upgrade",
        "downgrade",
        "analyst",
        "target",
        "rating",
        "initiated",
        "coverage",
        "merger",
        "acquisition",
        "lawsuit",
        "investigation",
        "sec",
        "regulatory",
        "fda",
        "contract",
        "partnership",
        "ceo",
        "cfo",
        "layoff",
        "restructur",
        "profit",
        "loss",
        "margin",
        "growth",
        "decline",
        "market",
        "trading",
        "ipo",
        "spac",
        "offering",
    ];
    signals.iter().any(|marker| combined.contains(marker))
}

pub(super) fn normalize_company_query_name(company_name: &str) -> String {
    let trimmed = company_name
        .replace([',', '.'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let mut value = trimmed.trim().to_string();
    let suffixes = [
        "common stock",
        "class a",
        "class b",
        "inc",
        "corp",
        "corporation",
        "ltd",
        "plc",
        "holdings",
    ];

    loop {
        let lowercase = value.to_lowercase();
        let mut stripped_any = false;
        for suffix in suffixes {
            if let Some(stripped) = lowercase.strip_suffix(suffix) {
                let stripped_len = stripped.len();
                value = value[..stripped_len].trim().to_string();
                stripped_any = true;
                break;
            }
        }
        if !stripped_any {
            break;
        }
    }
    value
}
