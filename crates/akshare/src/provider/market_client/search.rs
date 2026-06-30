use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::Cursor;
use std::time::Duration;

use anyhow::Context;
use calamine::{Data, Reader, Xlsx};

use super::{
    HK_SECURITIES_LIST_CACHE_TTL_SECS, HkSecurityDirectoryEntry, MARKET_DATA_CACHE_PREFIX,
    MarketDataClient, MarketKind, StockSearchResult, UsSecurityDirectoryEntry,
};

pub(crate) fn search_market_kind(value: &str) -> MarketKind {
    match value.trim().to_ascii_lowercase().as_str() {
        "a股" | "a_share" | "a-share" | "ashare" | "cn" | "cn_stock" | "cn-stock" | "china" => {
            MarketKind::AShare
        }
        "港股" | "hk" | "hk_equity" | "hk-equity" | "hongkong" | "hong_kong" => {
            MarketKind::HongKong
        }
        _ => MarketKind::UsEquity,
    }
}

/// Map a market parameter to the Chinese market label used by Eastmoney results.
pub(crate) fn market_to_eastmoney_label(value: &str) -> &str {
    match search_market_kind(value) {
        MarketKind::AShare => "A股",
        MarketKind::HongKong => "港股",
        MarketKind::UsEquity => "美股",
    }
}

pub(crate) fn stock_market_key(value: &str) -> &'static str {
    match search_market_kind(value) {
        MarketKind::AShare => "a_share",
        MarketKind::HongKong => "hk_equity",
        MarketKind::UsEquity => "us_equity",
    }
}

pub fn normalize_search_text(value: &str) -> String {
    value
        .chars()
        .filter(|char| !char.is_whitespace() && *char != '-' && *char != '_' && *char != '.')
        .flat_map(|char| char.to_lowercase())
        .collect::<String>()
}

pub fn preferred_search_language_for_query(query: &str) -> &'static str {
    let has_cjk = query.chars().any(is_cjk_character);
    let has_ascii_alpha = query.chars().any(|ch| ch.is_ascii_alphabetic());
    match (has_cjk, has_ascii_alpha) {
        (true, _) => "zh-CN",
        (false, true) => "en-US",
        (false, false) => "all",
    }
}

pub(crate) fn is_cjk_character(ch: char) -> bool {
    matches!(ch as u32, 0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF)
}

pub(crate) fn excel_cell_string(cell: Option<&Data>) -> String {
    match cell {
        Some(Data::String(value)) => value.trim().to_string(),
        Some(Data::Float(value)) => {
            if value.fract() == 0.0 {
                format!("{value:.0}")
            } else {
                value.to_string()
            }
        }
        Some(Data::Int(value)) => value.to_string(),
        Some(Data::Bool(value)) => {
            if *value {
                "Y".to_string()
            } else {
                String::new()
            }
        }
        Some(Data::DateTime(value)) => value.to_string(),
        Some(other) => other.to_string().trim().to_string(),
        None => String::new(),
    }
}

pub(crate) fn is_preferred_equity_listing(item: &StockSearchResult) -> bool {
    let upper_name = item.name.to_ascii_uppercase();

    let market = search_market_kind(&item.market);
    if market == MarketKind::HongKong
        && (upper_name.contains(" WR")
            || upper_name.ends_with("-WR")
            || upper_name.contains(" BULL")
            || upper_name.contains(" BEAR")
            || upper_name.contains(" CBBC")
            || upper_name.contains("WARRANT")
            || upper_name.contains(" INLINE")
            || upper_name.contains(" BT ")
            || upper_name.ends_with(" BT")
            || upper_name.contains(" N2")
            || upper_name.contains(" N3")
            || upper_name.contains(" N4")
            || upper_name.contains(" N5"))
    {
        return false;
    }
    true
}

pub fn stock_search_score(query: &str, item: &StockSearchResult) -> i32 {
    let symbol = normalize_search_text(&item.symbol);
    let name = normalize_search_text(&item.name);
    let mut score = 0i32;
    let mut matched = false;
    let query_is_short_alpha =
        query.len() <= 4 && query.chars().all(|char| char.is_ascii_alphabetic());

    if query.is_empty() {
        return i32::MIN;
    }
    if symbol == query {
        score += 500;
        matched = true;
    }
    if name == query {
        score += 460;
        matched = true;
    }
    if symbol.starts_with(query) {
        score += 260;
        matched = true;
    }
    if name.starts_with(query) {
        score += 240;
        matched = true;
    }
    if symbol.contains(query) {
        score += 200;
        matched = true;
    }
    if name.contains(query) {
        score += 180;
        matched = true;
    }
    if item
        .name
        .split(|char: char| !char.is_alphanumeric())
        .any(|part| normalize_search_text(part) == query)
    {
        score += 120;
        matched = true;
    }

    if !matched {
        return i32::MIN;
    }

    if query_is_short_alpha && !symbol.contains(query) && !name.starts_with(query) {
        return i32::MIN;
    }

    if is_preferred_equity_listing(item) {
        score += 80;
    } else {
        score -= 140;
    }
    score
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

fn normalized_news_date(published_at: &str) -> Option<String> {
    let trimmed = published_at.trim();
    if trimmed.len() >= 10 && trimmed.as_bytes()[4] == b'-' && trimmed.as_bytes()[7] == b'-' {
        return Some(trimmed[0..10].to_string());
    }
    None
}
impl MarketDataClient {
    pub(super) async fn search_stocks_with_fallbacks(
        &self,
        query: &str,
        market: Option<&str>,
        limit: usize,
    ) -> anyhow::Result<Vec<StockSearchResult>> {
        // TODO: Port akshare_rust::search_stocks to akshare-rs
        let primary: Vec<StockSearchResult> = Vec::new();
        let normalized_market = market.map(search_market_kind);

        let mut merged = self.rank_search_results(query, normalized_market, primary);
        if merged.len() >= limit {
            merged.truncate(limit);
            return Ok(merged);
        }

        match normalized_market {
            Some(MarketKind::HongKong) => {
                let hk_fallback = self
                    .search_hk_directory(query, limit.saturating_mul(4))
                    .await?;
                merged =
                    self.merge_ranked_stock_results(query, normalized_market, merged, hk_fallback);
            }
            Some(MarketKind::UsEquity) => {
                let us_fallback = self
                    .search_us_directory(query, limit.saturating_mul(4))
                    .await?;
                merged =
                    self.merge_ranked_stock_results(query, normalized_market, merged, us_fallback);
            }
            Some(MarketKind::AShare) => {
                let a_share_fallback = self
                    .search_stocks_from_eastmoney(query, market, limit.saturating_mul(4))
                    .await
                    .unwrap_or_default();
                merged = self.merge_ranked_stock_results(
                    query,
                    normalized_market,
                    merged,
                    a_share_fallback,
                );
            }
            _ => {}
        }

        // Direct lookup fallback for codes not found in suggest API (e.g. certain indices)
        if merged.is_empty() {
            let trimmed_query = query.trim();
            if !trimmed_query.is_empty()
                && trimmed_query.chars().all(|c| c.is_ascii_digit())
                && let Some(direct) = self.search_eastmoney_direct_lookup(trimmed_query).await
            {
                merged.push(direct);
            }
        }

        if merged.is_empty() {
            return Ok(Vec::new());
        }
        merged.truncate(limit);
        Ok(merged)
    }

    fn merge_ranked_stock_results(
        &self,
        query: &str,
        market: Option<MarketKind>,
        primary: Vec<StockSearchResult>,
        fallback: Vec<StockSearchResult>,
    ) -> Vec<StockSearchResult> {
        let mut deduped = HashMap::<String, StockSearchResult>::new();
        for item in primary.into_iter().chain(fallback) {
            deduped
                .entry(format!(
                    "{}:{}",
                    stock_market_key(&item.market),
                    item.symbol.to_uppercase()
                ))
                .or_insert(item);
        }
        self.rank_search_results(query, market, deduped.into_values().collect())
    }

    pub(crate) fn rank_search_results(
        &self,
        query: &str,
        market: Option<MarketKind>,
        items: Vec<StockSearchResult>,
    ) -> Vec<StockSearchResult> {
        let normalized_query = normalize_search_text(query);
        let expected_market = market;
        let mut scored = items
            .into_iter()
            .filter(|item| {
                expected_market.is_none_or(|value| search_market_kind(&item.market) == value)
            })
            .map(|item| {
                let score = stock_search_score(&normalized_query, &item);
                (score, item)
            })
            .filter(|(score, _)| *score > i32::MIN / 2)
            .collect::<Vec<_>>();
        scored.sort_by_key(|(score, item)| {
            (
                Reverse(*score),
                Reverse(is_preferred_equity_listing(item) as u8),
                item.symbol.len(),
                item.symbol.clone(),
            )
        });
        let mut ranked = scored.into_iter().map(|(_, item)| item).collect::<Vec<_>>();
        if expected_market == Some(MarketKind::HongKong)
            && ranked.iter().any(is_preferred_equity_listing)
        {
            ranked.retain(is_preferred_equity_listing);
        }
        ranked
    }

    pub(super) async fn search_hk_directory(
        &self,
        query: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<StockSearchResult>> {
        let normalized_query = normalize_search_text(query);
        if normalized_query.is_empty() {
            return Ok(Vec::new());
        }
        let entries = self.fetch_hk_security_directory().await?;
        let mut scored = entries
            .into_iter()
            .filter_map(|entry| {
                let item = StockSearchResult {
                    symbol: entry.symbol,
                    name: entry.name,
                    market: entry.market,
                    exchange: entry.exchange,
                };
                let score = stock_search_score(&normalized_query, &item);
                (score > i32::MIN / 2).then_some((score, item))
            })
            .collect::<Vec<_>>();
        scored.sort_by_key(|(score, item)| {
            (
                Reverse(*score),
                Reverse(is_preferred_equity_listing(item) as u8),
                item.symbol.len(),
                item.symbol.clone(),
            )
        });
        Ok(scored
            .into_iter()
            .take(limit)
            .map(|(_, item)| item)
            .collect())
    }

    async fn search_us_directory(
        &self,
        query: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<StockSearchResult>> {
        let normalized_query = normalize_search_text(query);
        if normalized_query.is_empty() {
            return Ok(Vec::new());
        }
        let entries = self.fetch_us_security_directory().await?;
        let mut scored = entries
            .into_iter()
            .filter_map(|entry| {
                let item = StockSearchResult {
                    symbol: entry.symbol,
                    name: entry.name,
                    market: entry.market,
                    exchange: entry.exchange,
                };
                let score = stock_search_score(&normalized_query, &item);
                (score > i32::MIN / 2).then_some((score, item))
            })
            .collect::<Vec<_>>();
        scored
            .sort_by_key(|(score, item)| (Reverse(*score), item.symbol.len(), item.symbol.clone()));
        Ok(scored
            .into_iter()
            .take(limit)
            .map(|(_, item)| item)
            .collect())
    }

    async fn fetch_hk_security_directory(&self) -> anyhow::Result<Vec<HkSecurityDirectoryEntry>> {
        let cache_key = format!("{MARKET_DATA_CACHE_PREFIX}:search:hk_directory:v1");
        if let Some(cached) = self
            .cache_get_json::<Vec<HkSecurityDirectoryEntry>>(&cache_key)
            .await
        {
            return Ok(cached);
        }

        let response = tokio::time::timeout(
            Duration::from_secs(3),
            self
                .http
                .get("https://www.hkex.com.hk/eng/services/trading/securities/securitieslists/ListOfSecurities.xlsx")
                .send(),
        )
        .await
            .context("HKEX securities list timed out after 3s")?
            .context("failed to fetch HKEX securities list")?
            .error_for_status()
            .context("HKEX securities list request failed")?;
        let bytes = response
            .bytes()
            .await
            .context("failed to read HKEX securities list body")?;
        let entries = Self::parse_hk_security_directory(bytes.as_ref())?;
        self.cache_set_json(&cache_key, HK_SECURITIES_LIST_CACHE_TTL_SECS, &entries)
            .await;
        Ok(entries)
    }

    fn parse_hk_security_directory(
        payload: &[u8],
    ) -> anyhow::Result<Vec<HkSecurityDirectoryEntry>> {
        let mut workbook: Xlsx<Cursor<Vec<u8>>> = Xlsx::new(Cursor::new(payload.to_vec()))
            .context("failed to open HKEX securities workbook")?;
        let range = workbook
            .worksheet_range("ListOfSecurities")
            .context("HKEX workbook missing ListOfSecurities sheet")?;

        let mut rows = range.rows();
        let header = rows
            .next()
            .context("HKEX securities workbook missing title row")?;
        let _updated = rows.next().unwrap_or(header);
        let columns = rows
            .next()
            .context("HKEX securities workbook missing header columns")?;
        let column_map = columns
            .iter()
            .enumerate()
            .map(|(index, cell)| (cell.to_string().trim().to_string(), index))
            .collect::<HashMap<_, _>>();

        let stock_code_index = *column_map
            .get("Stock Code")
            .context("HKEX securities workbook missing Stock Code column")?;
        let name_index = *column_map
            .get("Name of Securities")
            .context("HKEX securities workbook missing Name of Securities column")?;
        let category_index = *column_map
            .get("Category")
            .context("HKEX securities workbook missing Category column")?;
        let sub_category_index = *column_map
            .get("Sub-Category")
            .context("HKEX securities workbook missing Sub-Category column")?;
        let currency_index = *column_map
            .get("Trading Currency")
            .context("HKEX securities workbook missing Trading Currency column")?;

        let mut entries = Vec::new();
        for row in rows {
            let symbol = excel_cell_string(row.get(stock_code_index));
            let name = excel_cell_string(row.get(name_index));
            let category = excel_cell_string(row.get(category_index));
            let sub_category = excel_cell_string(row.get(sub_category_index));
            let trading_currency = excel_cell_string(row.get(currency_index));
            if symbol.is_empty() || name.is_empty() {
                continue;
            }
            if category != "Equity" {
                continue;
            }
            if !sub_category.contains("Equity Securities") {
                continue;
            }
            entries.push(HkSecurityDirectoryEntry {
                symbol: format!("{symbol:0>5}"),
                name,
                market: "港股".to_string(),
                exchange: "HK".to_string(),
                category,
                sub_category,
                trading_currency,
            });
        }
        Ok(entries)
    }

    async fn fetch_us_security_directory(&self) -> anyhow::Result<Vec<UsSecurityDirectoryEntry>> {
        let cache_key = format!("{MARKET_DATA_CACHE_PREFIX}:search:us_directory:v1");
        if let Some(cached) = self
            .cache_get_json::<Vec<UsSecurityDirectoryEntry>>(&cache_key)
            .await
        {
            return Ok(cached);
        }
        let entries = self.fetch_us_security_directory_from_sec().await?;
        self.cache_set_json(&cache_key, HK_SECURITIES_LIST_CACHE_TTL_SECS, &entries)
            .await;
        Ok(entries)
    }

    async fn fetch_us_security_directory_from_sec(
        &self,
    ) -> anyhow::Result<Vec<UsSecurityDirectoryEntry>> {
        tracing::info!("fetching SEC ticker map for US stock search");
        let map: super::wire::SecTickerLookup = self
            .http
            .get("https://www.sec.gov/files/company_tickers.json")
            .send()
            .await
            .context("failed to fetch SEC ticker map for US stock search")?
            .error_for_status()
            .context("SEC ticker map request for US stock search failed")?
            .json()
            .await
            .context("failed to decode SEC ticker map for US stock search")?;
        Ok(map
            .values()
            .map(|entry| UsSecurityDirectoryEntry {
                symbol: entry.ticker.clone(),
                name: entry.title.clone(),
                market: "美股".to_string(),
                exchange: "US".to_string(),
            })
            .collect())
    }
}

#[cfg(test)]
mod search_tests {
    use super::*;

    // --- search_market_kind ---

    #[test]
    fn search_market_kind_a_share() {
        assert_eq!(search_market_kind("A股"), MarketKind::AShare);
        assert_eq!(search_market_kind("a_share"), MarketKind::AShare);
        assert_eq!(search_market_kind("cn"), MarketKind::AShare);
    }

    #[test]
    fn search_market_kind_hong_kong() {
        assert_eq!(search_market_kind("港股"), MarketKind::HongKong);
        assert_eq!(search_market_kind("hk"), MarketKind::HongKong);
        assert_eq!(search_market_kind("hong_kong"), MarketKind::HongKong);
    }

    #[test]
    fn search_market_kind_us() {
        assert_eq!(search_market_kind("美股"), MarketKind::UsEquity);
        assert_eq!(search_market_kind("us"), MarketKind::UsEquity);
        assert_eq!(search_market_kind("unknown"), MarketKind::UsEquity);
    }

    // --- market_to_eastmoney_label ---

    #[test]
    fn market_to_eastmoney_label_all() {
        assert_eq!(market_to_eastmoney_label("A股"), "A股");
        assert_eq!(market_to_eastmoney_label("港股"), "港股");
        assert_eq!(market_to_eastmoney_label("美股"), "美股");
    }

    // --- stock_market_key ---

    #[test]
    fn stock_market_key_all() {
        assert_eq!(stock_market_key("A股"), "a_share");
        assert_eq!(stock_market_key("港股"), "hk_equity");
        assert_eq!(stock_market_key("美股"), "us_equity");
    }

    // --- normalize_search_text ---

    #[test]
    fn normalize_search_text_basic() {
        assert_eq!(normalize_search_text("AAPL"), "aapl");
    }

    #[test]
    fn normalize_search_text_strips_whitespace() {
        assert_eq!(normalize_search_text("AA PL"), "aapl");
    }

    #[test]
    fn normalize_search_text_strips_special() {
        assert_eq!(normalize_search_text("AA-PL_US.A"), "aaplusa");
    }

    #[test]
    fn normalize_search_text_chinese() {
        assert_eq!(normalize_search_text("贵州茅台"), "贵州茅台");
    }

    // --- preferred_search_language_for_query ---

    #[test]
    fn preferred_search_language_chinese() {
        assert_eq!(preferred_search_language_for_query("贵州茅台"), "zh-CN");
    }

    #[test]
    fn preferred_search_language_english() {
        assert_eq!(preferred_search_language_for_query("AAPL"), "en-US");
    }

    #[test]
    fn preferred_search_language_mixed() {
        // Chinese takes priority
        assert_eq!(preferred_search_language_for_query("AAPL贵州"), "zh-CN");
    }

    #[test]
    fn preferred_search_language_numeric() {
        assert_eq!(preferred_search_language_for_query("600519"), "all");
    }

    // --- is_cjk_character ---

    #[test]
    fn is_cjk_character_basic() {
        assert!(is_cjk_character('中'));
        assert!(is_cjk_character('文'));
        assert!(!is_cjk_character('A'));
        assert!(!is_cjk_character('1'));
    }

    // --- excel_cell_string ---

    #[test]
    fn excel_cell_string_string() {
        assert_eq!(
            excel_cell_string(Some(&Data::String("hello".into()))),
            "hello"
        );
    }

    #[test]
    fn excel_cell_string_float_integer() {
        assert_eq!(excel_cell_string(Some(&Data::Float(42.0))), "42");
    }

    #[test]
    fn excel_cell_string_float_decimal() {
        let result = excel_cell_string(Some(&Data::Float(3.15)));
        assert!(result.contains("3.15"));
    }

    #[test]
    fn excel_cell_string_int() {
        assert_eq!(excel_cell_string(Some(&Data::Int(42))), "42");
    }

    #[test]
    fn excel_cell_string_bool() {
        assert_eq!(excel_cell_string(Some(&Data::Bool(true))), "Y");
        assert_eq!(excel_cell_string(Some(&Data::Bool(false))), "");
    }

    #[test]
    fn excel_cell_string_none() {
        assert_eq!(excel_cell_string(None), "");
    }

    // --- is_preferred_equity_listing ---

    #[test]
    fn is_preferred_equity_listing_normal() {
        let item = StockSearchResult {
            symbol: "0700".into(),
            name: "TENCENT".into(),
            market: "港股".into(),
            exchange: "HKEX".into(),
        };
        assert!(is_preferred_equity_listing(&item));
    }

    #[test]
    fn is_preferred_equity_listing_warrant() {
        let item = StockSearchResult {
            symbol: "12345".into(),
            name: "TEST WR".into(),
            market: "港股".into(),
            exchange: "HKEX".into(),
        };
        assert!(!is_preferred_equity_listing(&item));
    }

    #[test]
    fn is_preferred_equity_listing_bull() {
        let item = StockSearchResult {
            symbol: "12345".into(),
            name: "TEST BULL".into(),
            market: "港股".into(),
            exchange: "HKEX".into(),
        };
        assert!(!is_preferred_equity_listing(&item));
    }

    #[test]
    fn is_preferred_equity_listing_us_always_preferred() {
        let item = StockSearchResult {
            symbol: "AAPL".into(),
            name: "Apple Inc".into(),
            market: "美股".into(),
            exchange: "NASDAQ".into(),
        };
        assert!(is_preferred_equity_listing(&item));
    }
}
