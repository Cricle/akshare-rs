//! Content classifiers for news item filtering and relevance scoring.
//!
//! Each function classifies a news item or text snippet by content type
//! (e.g., investment research, SEC filing, market wrap, competitor mention).

use super::{normalize_news_text, NewsItem};

pub(crate) fn title_is_reference_or_overview_page(
    normalized_title: &str,
    normalized_summary: &str,
) -> bool {
    normalized_title.contains("stockoverview")
        || normalized_title.contains("marketdata")
        || normalized_title.contains("companyprofile")
        || normalized_title.contains("homepage")
        || normalized_title.contains("officialwebsite")
        || normalized_title.contains("latestnewsandupdates")
        || normalized_title.contains("realtimequotes")
        || normalized_title.contains("latestprice")
        || normalized_title.contains("stockprice")
        || normalized_title.contains("quote")
        || normalized_title.contains("homeoverview")
        || normalized_summary.contains("engagesinthedesigndevelopmentmanufactureandsale")
        || normalized_summary.contains("operatesthroughthe")
}

pub(crate) fn url_is_quote_or_overview_page(url: &str) -> bool {
    let normalized = url.trim().to_ascii_lowercase();
    let Ok(parsed) = reqwest::Url::parse(&normalized) else {
        return normalized.contains("quote.") || normalized.contains("stockpage.");
    };

    let host = parsed.host_str().unwrap_or_default();
    let path = parsed.path();

    host.starts_with("quote.")
        || host.starts_with("stockpage.")
        || host == "finance.yahoo.com" && path.starts_with("/quote/")
        || host == "hk.finance.yahoo.com" && path.starts_with("/quote/")
        || host == "tw.stock.yahoo.com" && path.starts_with("/quote/")
        || host == "www.nasdaq.com" && path.starts_with("/market-activity/stocks/")
        || host == "nasdaq.com" && path.starts_with("/market-activity/stocks/")
        || host == "finance.baidu.com" && path.starts_with("/stock/")
        || host == "stock.finance.sina.com.cn" && path.starts_with("/hkstock/quotes/")
        || host == "stock.finance.sina.com.cn" && path.starts_with("/usstock/quotes/")
        || host.ends_with("xueqiu.com") && path.starts_with("/s/")
        || host.ends_with("hstong.com") && path.starts_with("/quotes/")
        || host.ends_with("aastocks.com") && path.starts_with("/en/stocks/quote/")
        || host.ends_with("aastocks.com") && path.starts_with("/tc/stocks/quote/")
        || host.ends_with("etnet.com.hk") && path.ends_with("/quote.php")
}

pub(crate) fn url_is_ir_landing_page(url: &str) -> bool {
    let normalized = url.trim().to_ascii_lowercase();
    let Ok(parsed) = reqwest::Url::parse(&normalized) else {
        return normalized.contains("investor-relations/default")
            || normalized.ends_with("/investor-relations/")
            || normalized.ends_with("/investor-relations");
    };

    let host = parsed.host_str().unwrap_or_default();
    let path = parsed.path();

    (host.contains("investor.") || host.contains("ir."))
        && (path == "/"
            || path.ends_with("/default.aspx")
            || path.ends_with("/investor-relations/"))
        || path.ends_with("/investor-relations")
}

pub(crate) fn title_or_summary_has_high_value_company_event(
    normalized_title: &str,
    normalized_summary: &str,
) -> bool {
    let combined = format!("{normalized_title} {normalized_summary}");
    let markers = [
        "earnings",
        "quarterlyresults",
        "annualresults",
        "interimresults",
        "financialresults",
        "resultsannouncement",
        "businessupdate",
        "tradingupdate",
        "guidance",
        "buyback",
        "sharebuyback",
        "dividend",
        "delivery",
        "deliveries",
        "sales",
        "orders",
        "productlaunch",
        "业绩",
        "财报",
        "公告",
        "季报",
        "年报",
        "中报",
        "回购",
        "派息",
        "交付",
        "销量",
        "订单",
        "指引",
    ];
    markers
        .iter()
        .any(|marker| combined.contains(&normalize_news_text(marker)))
}

pub(crate) fn title_or_summary_has_low_value_corporate_filing_noise(
    normalized_title: &str,
    normalized_summary: &str,
) -> bool {
    let combined = format!("{normalized_title} {normalized_summary}");
    let markers = [
        "nextdaydisclosurereturn",
        "monthlyreturnofequityissuer",
        "pollresults",
        "annualgeneralmeeting",
        "proxyform",
        "formofproxy",
        "notificationletter",
        "circular",
        "changeofdirector",
        "listofdirectors",
        "closureofregisterofmembers",
        "independentdirectorcandidate",
        "statementandundertaking",
        "144filing",
    ];
    markers.iter().any(|marker| combined.contains(marker))
}

pub(crate) fn is_investment_research_evidence_page(item: &NewsItem) -> bool {
    let normalized_title = normalize_news_text(&item.title);
    let normalized_summary = normalize_news_text(&item.summary);
    let normalized_source = item.source.to_ascii_lowercase();
    let normalized_url = item.url.as_deref().unwrap_or_default().to_ascii_lowercase();
    let combined = format!("{normalized_title} {normalized_summary}");

    let finance_hub_markers = [
        "eastmoney",
        "xueqiu",
        "sina.com.cn",
        "10jqka",
        "futunn",
        "stockstar",
        "hstong",
        "investing.com",
        "aastocks",
        "etnet",
        "cnevpost",
        "carnewschina",
        "finance.",
        "stock.",
        "quote.",
    ];
    let finance_hub_event_markers = [
        "业绩",
        "公告",
        "财报",
        "业绩快报",
        "中报",
        "年报",
        "季报",
        "交付",
        "销量",
        "订单",
        "指引",
        "投资者关系",
        "数据报告",
        "新闻",
        "研报",
        "评级",
        "investor relations",
        "earnings",
        "results",
        "quarterly results",
        "interim results",
        "annual results",
        "delivery",
        "deliveries",
        "order",
        "orders",
        "guidance",
        "research",
        "report",
        "announcement",
    ];
    let has_finance_hub_signal = finance_hub_markers
        .iter()
        .any(|marker| normalized_source.contains(marker) || normalized_url.contains(marker));
    let has_finance_hub_event_signal = finance_hub_event_markers
        .iter()
        .any(|marker| combined.contains(&normalize_news_text(marker)));
    let has_quote_or_overview_url =
        url_is_quote_or_overview_page(item.url.as_deref().unwrap_or_default());

    if title_is_reference_or_overview_page(&normalized_title, &normalized_summary)
        || title_is_generic_market_wrap(&normalized_title)
    {
        return false;
    }

    let source_markers = [
        "reuters",
        "bloomberg",
        "yahoo",
        "marketwatch",
        "benzinga",
        "investing",
        "seekingalpha",
        "fool",
        "barron",
        "morningstar",
        "ft",
        "wsj",
        "nikkei",
        "cnbc",
        "eastmoney",
        "etnet",
        "aastocks",
        "hkex",
        "nasdaq",
        "sec",
        "sse",
        "szse",
    ];
    let strong_url_markers = [
        "news",
        "article",
        "press-release",
        "announcement",
        "investor",
        "ir.",
        "earnings",
        "filing",
        "research",
        "report",
        "finance",
        "quote",
        "hkexnews.hk",
        "aastocks.com",
        "etnet.com.hk",
        "eastmoney.com",
    ];
    let event_markers = [
        "业绩",
        "公告",
        "财报",
        "业绩快报",
        "中报",
        "年报",
        "季报",
        "交付",
        "销量",
        "订单",
        "指引",
        "回购",
        "派息",
        "增发",
        "配股",
        "融资",
        "投资者关系",
        "研报",
        "评级",
        "target price",
        "earnings",
        "results",
        "quarterly results",
        "interim results",
        "annual results",
        "delivery",
        "deliveries",
        "order",
        "orders",
        "guidance",
        "analyst",
        "downgrade",
        "upgrade",
        "filing",
        "press release",
        "investor relations",
        "annual report",
        "quarterly report",
    ];
    let weak_reference_markers = [
        "quote", "行情", "股价", "概览", "overview", "profile", "homepage", "官网", "home", "wiki",
        "百科",
    ];
    let entertainment_noise_markers = [
        "白玉兰",
        "杨幂",
        "杨紫",
        "娱乐",
        "douyin",
        "weibo",
        "celebrity",
        "entertainment",
        "sport",
        "足球",
        "live-ticker",
    ];

    if entertainment_noise_markers
        .iter()
        .any(|marker| combined.contains(&normalize_news_text(marker)))
    {
        return false;
    }

    if weak_reference_markers
        .iter()
        .any(|marker| normalized_title.contains(&normalize_news_text(marker)))
        && !finance_hub_markers
            .iter()
            .any(|marker| normalized_source.contains(marker) || normalized_url.contains(marker))
    {
        return false;
    }

    let has_source_signal = source_markers
        .iter()
        .any(|marker| normalized_source.contains(marker));
    let has_url_signal = strong_url_markers
        .iter()
        .any(|marker| normalized_url.contains(marker));
    let has_event_signal = event_markers
        .iter()
        .any(|marker| combined.contains(&normalize_news_text(marker)));
    let has_ir_results_signal = (normalized_url.contains("investor")
        || normalized_url.contains("/ir")
        || normalized_url.contains("relations"))
        && (combined.contains("results")
            || combined.contains("earnings")
            || combined.contains("report")
            || combined.contains("announcement")
            || combined.contains(&normalize_news_text("业绩"))
            || combined.contains(&normalize_news_text("公告"))
            || combined.contains(&normalize_news_text("财报")));
    let has_finance_hub_article_signal = has_finance_hub_signal
        && has_finance_hub_event_signal
        && !has_quote_or_overview_url
        && (normalized_url.contains("news")
            || normalized_url.contains("article")
            || normalized_url.contains("/a/")
            || normalized_url.contains("/n/")
            || normalized_url.contains("research")
            || normalized_url.contains("report"));

    if has_quote_or_overview_url {
        return has_ir_results_signal;
    }

    (has_source_signal || has_url_signal) && has_event_signal
        || (has_source_signal && has_url_signal)
        || has_finance_hub_article_signal
        || has_ir_results_signal
}

pub(crate) fn is_macro_research_evidence_page(item: &NewsItem) -> bool {
    let normalized_title = normalize_news_text(&item.title);
    let normalized_summary = normalize_news_text(&item.summary);
    let normalized_source = item.source.to_ascii_lowercase();
    let normalized_url = item.url.as_deref().unwrap_or_default().to_ascii_lowercase();
    let combined = format!("{normalized_title} {normalized_summary}");

    if title_is_reference_or_overview_page(&normalized_title, &normalized_summary) {
        return false;
    }

    let entertainment_noise_markers = [
        "白玉兰",
        "杨幂",
        "杨紫",
        "娱乐",
        "douyin",
        "weibo",
        "celebrity",
        "entertainment",
        "sport",
        "足球",
        "live-ticker",
    ];
    if entertainment_noise_markers
        .iter()
        .any(|marker| combined.contains(&normalize_news_text(marker)))
    {
        return false;
    }

    let macro_source_markers = [
        "reuters",
        "bloomberg",
        "ft",
        "wsj",
        "cnbc",
        "nikkei",
        "yahoo",
        "marketwatch",
        "investing",
        "aastocks",
        "etnet",
        "eastmoney",
        "stcn",
        "caixin",
        "finance",
        "cctv",
        "chinanews",
        "gov.cn",
        "china.com.cn",
    ];
    let macro_url_markers = [
        "news",
        "article",
        "markets",
        "economy",
        "macro",
        "policy",
        "finance",
        "business",
        "gov.cn",
        "cctv.com",
        "chinanews.com.cn",
        "china.com.cn",
    ];
    let macro_event_markers = [
        "宏观",
        "经济",
        "政策",
        "利率",
        "通胀",
        "流动性",
        "汇率",
        "人民币",
        "港股",
        "恒生",
        "科技股",
        "中国互联网",
        "risk sentiment",
        "market",
        "economy",
        "policy",
        "yield",
        "inflation",
        "liquidity",
        "hong kong",
        "china tech",
        "federal reserve",
        "tariff",
        "时政",
        "发布会",
        "刺激",
        "消费",
        "PMI",
        "manufacturing",
        "outlook",
        "equities",
        "新能源车",
        "电动车",
        "汽车",
        "智驾",
        "补贴",
        "以旧换新",
        "内需",
        "出口",
        "关税",
        "ev",
        "electric vehicle",
        "auto",
        "autos",
        "subsidy",
        "consumer",
        "stimulus",
    ];

    let has_source_signal = macro_source_markers
        .iter()
        .any(|marker| normalized_source.contains(marker));
    let has_url_signal = macro_url_markers
        .iter()
        .any(|marker| normalized_url.contains(marker));
    let has_event_signal = macro_event_markers
        .iter()
        .any(|marker| combined.contains(&normalize_news_text(marker)));

    has_event_signal && (has_source_signal || has_url_signal)
}

pub(crate) fn is_sec_filing_item(item: &NewsItem) -> bool {
    let normalized_title = normalize_news_text(&item.title);
    let normalized_source = item.source.to_ascii_lowercase();
    normalized_source.contains("sec")
        && (normalized_title.contains("filing")
            || normalized_title.contains("form")
            || normalized_title.contains("def14a")
            || normalized_title.contains("8-k")
            || normalized_title.contains("13d")
            || normalized_title.contains("proxy")
            || normalized_title.contains("ars")
            || normalized_title.contains("defa14a")
            || normalized_title.contains("px14a6g"))
}

pub(crate) fn is_sec_biasing_keyword(keyword: &str) -> bool {
    matches!(
        keyword,
        "filing" | "sec" | "form" | "proxy" | "def14a" | "8-k" | "13d"
    )
}

pub(crate) fn title_is_generic_market_wrap(normalized_title: &str) -> bool {
    normalized_title.contains("dowjones")
        || normalized_title.contains("s&p500")
        || normalized_title.contains("stockmarket")
        || normalized_title.contains("markets")
        || normalized_title.contains("sensex")
        || normalized_title.contains("nifty")
}

pub(crate) fn mentions_competitor_without_primary_company_focus(
    normalized_title: &str,
    combined: &str,
    keywords: &[String],
) -> bool {
    let mentions_primary = keywords
        .iter()
        .map(|value| normalize_news_text(value))
        .filter(|value| !value.is_empty())
        .any(|keyword| normalized_title.contains(&keyword) || combined.contains(&keyword));
    let competitor_markers = [
        "cerebras", "apple", "intel", "amd", "broadcom", "samsung", "micron", "groq",
    ];
    let mentions_competitor = competitor_markers
        .iter()
        .any(|marker| normalized_title.contains(marker));
    mentions_competitor && !mentions_primary
}

pub(crate) fn mentions_secondary_reference_only(
    normalized_title: &str,
    normalized_summary: &str,
    keywords: &[String],
) -> bool {
    if !keywords
        .iter()
        .map(|value| normalize_news_text(value))
        .filter(|value| !value.is_empty())
        .any(|keyword| normalized_summary.contains(&keyword))
    {
        return false;
    }
    let secondary_markers = [
        "dominated by",
        "compared with",
        "competes with",
        "versus",
        "rival",
        "peer",
        "challenge to",
        "take on",
    ];
    secondary_markers
        .iter()
        .any(|marker| normalized_title.contains(marker) || normalized_summary.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(title: &str, summary: &str, source: &str, url: &str) -> NewsItem {
        NewsItem {
            published_at: "2026-01-01".to_string(),
            title: title.to_string(),
            summary: summary.to_string(),
            source: source.to_string(),
            url: if url.is_empty() { None } else { Some(url.to_string()) },
        }
    }

    // --- title_is_reference_or_overview_page ---

    #[test]
    fn title_ref_stockoverview() {
        assert!(title_is_reference_or_overview_page("stockoverview page", ""));
    }

    #[test]
    fn title_ref_quote() {
        assert!(title_is_reference_or_overview_page("quote for AAPL", ""));
    }

    #[test]
    fn title_ref_normal_title() {
        assert!(!title_is_reference_or_overview_page("Apple beats earnings", ""));
    }

    #[test]
    fn title_ref_summary_engages() {
        assert!(title_is_reference_or_overview_page("", "engagesinthedesigndevelopmentmanufactureandsale"));
    }

    // --- url_is_quote_or_overview_page ---

    #[test]
    fn url_quote_yahoo() {
        assert!(url_is_quote_or_overview_page("https://finance.yahoo.com/quote/AAPL"));
    }

    #[test]
    fn url_quote_nasdaq() {
        assert!(url_is_quote_or_overview_page("https://www.nasdaq.com/market-activity/stocks/AAPL"));
    }

    #[test]
    fn url_quote_baidu() {
        assert!(url_is_quote_or_overview_page("https://finance.baidu.com/stock/sh600519"));
    }

    #[test]
    fn url_quote_xueqiu() {
        assert!(url_is_quote_or_overview_page("https://xueqiu.com/s/AAPL"));
    }

    #[test]
    fn url_quote_news_article() {
        assert!(!url_is_quote_or_overview_page("https://reuters.com/news/article123"));
    }

    // --- url_is_ir_landing_page ---

    #[test]
    fn url_ir_investor_subdomain() {
        assert!(url_is_ir_landing_page("https://investor.apple.com/"));
    }

    #[test]
    fn url_ir_default_aspx() {
        assert!(url_is_ir_landing_page("https://ir.company.com/default.aspx"));
    }

    #[test]
    fn url_ir_normal_url() {
        assert!(!url_is_ir_landing_page("https://apple.com/products"));
    }

    // --- title_or_summary_has_high_value_company_event ---

    #[test]
    fn high_value_earnings() {
        assert!(title_or_summary_has_high_value_company_event("earnings report", ""));
    }

    #[test]
    fn high_value_chinese_earnings() {
        assert!(title_or_summary_has_high_value_company_event("", "业绩公告"));
    }

    #[test]
    fn high_value_delivery() {
        assert!(title_or_summary_has_high_value_company_event("Q2 deliveries", ""));
    }

    #[test]
    fn high_value_none() {
        assert!(!title_or_summary_has_high_value_company_event("random news", "nothing special"));
    }

    // --- title_or_summary_has_low_value_corporate_filing_noise ---

    #[test]
    fn low_value_proxy_form() {
        assert!(title_or_summary_has_low_value_corporate_filing_noise("proxy form", ""));
    }

    #[test]
    fn low_value_agm() {
        assert!(title_or_summary_has_low_value_corporate_filing_noise("", "annualgeneralmeeting"));
    }

    #[test]
    fn low_value_none() {
        assert!(!title_or_summary_has_low_value_corporate_filing_noise("earnings beat", "strong results"));
    }

    // --- is_sec_filing_item ---

    #[test]
    fn sec_filing_form() {
        let item = make_item("Form 8-K Filing", "", "SEC", "");
        assert!(is_sec_filing_item(&item));
    }

    #[test]
    fn sec_filing_def14a() {
        let item = make_item("DEF14A Proxy Statement", "", "SEC", "");
        assert!(is_sec_filing_item(&item));
    }

    #[test]
    fn sec_filing_non_sec_source() {
        let item = make_item("Form 8-K Filing", "", "Reuters", "");
        assert!(!is_sec_filing_item(&item));
    }

    // --- is_sec_biasing_keyword ---

    #[test]
    fn sec_bias_filing() {
        assert!(is_sec_biasing_keyword("filing"));
    }

    #[test]
    fn sec_bias_sec() {
        assert!(is_sec_biasing_keyword("sec"));
    }

    #[test]
    fn sec_bias_normal() {
        assert!(!is_sec_biasing_keyword("earnings"));
    }

    // --- title_is_generic_market_wrap ---

    #[test]
    fn market_wrap_dowjones() {
        assert!(title_is_generic_market_wrap("dowjones rally today"));
    }

    #[test]
    fn market_wrap_sp500() {
        assert!(title_is_generic_market_wrap("s&p500 hits new high"));
    }

    #[test]
    fn market_wrap_normal() {
        assert!(!title_is_generic_market_wrap("Apple reports strong earnings"));
    }

    // --- mentions_competitor_without_primary_company_focus ---

    #[test]
    fn competitor_without_primary() {
        let keywords = vec!["NVDA".to_string()];
        assert!(mentions_competitor_without_primary_company_focus(
            "Apple launches new chip", "Apple competes with nvidia", &keywords
        ));
    }

    #[test]
    fn competitor_with_primary() {
        let keywords = vec!["Apple".to_string()];
        assert!(!mentions_competitor_without_primary_company_focus(
            "Apple launches new chip", "Apple and nvidia compete", &keywords
        ));
    }

    #[test]
    fn no_competitor_mention() {
        let keywords = vec!["NVDA".to_string()];
        assert!(!mentions_competitor_without_primary_company_focus(
            "NVDA reports earnings", "NVDA strong results", &keywords
        ));
    }

    // --- mentions_secondary_reference_only ---

    #[test]
    fn secondary_reference_versus() {
        let keywords = vec!["NVDA".to_string()];
        assert!(mentions_secondary_reference_only(
            "NVDA versus AMD", "NVDA compared with AMD", &keywords
        ));
    }

    #[test]
    fn secondary_reference_no_keyword_in_summary() {
        let keywords = vec!["NVDA".to_string()];
        assert!(!mentions_secondary_reference_only(
            "NVDA versus AMD", "random text", &keywords
        ));
    }

    #[test]
    fn secondary_reference_no_marker() {
        let keywords = vec!["NVDA".to_string()];
        assert!(!mentions_secondary_reference_only(
            "NVDA reports earnings", "NVDA strong results", &keywords
        ));
    }

    // --- is_investment_research_evidence_page ---

    #[test]
    fn research_evidence_reuters_earnings() {
        let item = make_item("Apple earnings beat", "strong quarterly results", "Reuters", "https://reuters.com/news/123");
        assert!(is_investment_research_evidence_page(&item));
    }

    #[test]
    fn research_evidence_overview_page() {
        let item = make_item("stockoverview AAPL", "overview page", "Yahoo", "https://finance.yahoo.com/quote/AAPL");
        assert!(!is_investment_research_evidence_page(&item));
    }

    #[test]
    fn research_evidence_entertainment() {
        let item = make_item("杨幂 new movie", "entertainment news", "Weibo", "https://weibo.com/123");
        assert!(!is_investment_research_evidence_page(&item));
    }

    #[test]
    fn research_evidence_eastmoney_article() {
        let item = make_item("业绩快报", "公告内容", "eastmoney", "https://eastmoney.com/a/123.html");
        assert!(is_investment_research_evidence_page(&item));
    }

    // --- is_macro_research_evidence_page ---

    #[test]
    fn macro_revidence_reuters_macro() {
        let item = make_item("Fed rate decision", "federal reserve policy", "Reuters", "https://reuters.com/markets/123");
        assert!(is_macro_research_evidence_page(&item));
    }

    #[test]
    fn macro_revidence_entertainment() {
        let item = make_item("杨幂 news", "entertainment", "Weibo", "");
        assert!(!is_macro_research_evidence_page(&item));
    }

    #[test]
    fn macro_revidence_no_source() {
        let item = make_item("economy strong", "market outlook", "RandomBlog", "");
        assert!(!is_macro_research_evidence_page(&item));
    }
}
