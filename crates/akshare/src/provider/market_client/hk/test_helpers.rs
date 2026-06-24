#![allow(dead_code)]

#[allow(unused_imports)]
use super::super::{MarketDataClient, NewsItem};
#[allow(unused_imports)]
use rust_decimal::prelude::ToPrimitive;

#[cfg(test)]
pub(crate) fn test_hk_yahoo_symbol(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<String> {
    client.hk_yahoo_symbol(symbol)
}

#[cfg(test)]
pub(crate) fn test_hk_standard_code(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<String> {
    client.hk_standard_code(symbol)
}

#[cfg(test)]
pub(crate) fn test_hk_search_aliases(client: &MarketDataClient, company_name: &str) -> Vec<String> {
    client.hk_search_aliases(company_name)
}

#[cfg(test)]
pub(crate) fn test_hk_company_news_queries(
    client: &MarketDataClient,
    standard_code: &str,
    short_code: &str,
    company_name: &str,
    primary_name: &str,
    english_alias: &str,
    aliases: &[String],
    query: Option<&str>,
) -> Vec<String> {
    client.hk_company_news_queries(
        standard_code,
        short_code,
        company_name,
        primary_name,
        english_alias,
        aliases,
        query,
        None,
        None,
    )
}

#[cfg(test)]
pub(crate) fn test_parse_hkex_title_search_results(html: &str) -> Vec<NewsItem> {
    super::news::parse_hkex_title_search_results(html)
}

#[cfg(test)]
pub(crate) fn test_hkex_item_is_high_value(item: &NewsItem) -> bool {
    super::news::hkex_item_is_high_value(item)
}

#[cfg(test)]
pub(crate) fn test_parse_hk_tencent_snapshot(
    raw: &str,
) -> anyhow::Result<TencentSnapshotTestResult> {
    let item = MarketDataClient::parse_hk_tencent_snapshot(raw)?;
    Ok(TencentSnapshotTestResult {
        name: item.name,
        market_cap_hkd: item.market_cap_hkd.and_then(|d| d.to_f64()),
        shares_outstanding: item.shares_outstanding,
        currency: item.currency,
    })
}
