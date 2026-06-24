pub(crate) mod a_share;
pub(crate) mod hk;
pub(crate) mod us;
pub(crate) mod us_sina;

use super::{
    CandlesWithProvider, FundamentalsSnapshot, MarketDataClient, MarketKind, NewsItem,
    QuoteWithProvider,
};

pub(crate) async fn fetch_quote(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<QuoteWithProvider> {
    match client.detect_market(symbol) {
        MarketKind::AShare => {
            let ts_code = client
                .normalize_a_share_symbol(symbol)
                .ok_or_else(|| anyhow::anyhow!("invalid A-share symbol"))?;
            a_share::fetch_quote(client, symbol, &ts_code).await
        }
        MarketKind::HongKong => hk::fetch_quote(client, symbol).await,
        MarketKind::UsEquity => us::fetch_quote(client, symbol).await,
    }
}

pub(crate) async fn fetch_fundamentals(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<FundamentalsSnapshot> {
    match client.detect_market(symbol) {
        MarketKind::AShare => {
            let ts_code = client
                .normalize_a_share_symbol(symbol)
                .ok_or_else(|| anyhow::anyhow!("invalid A-share symbol"))?;
            a_share::fetch_fundamentals(client, symbol, &ts_code).await
        }
        MarketKind::HongKong => hk::fetch_fundamentals(client, symbol).await,
        MarketKind::UsEquity => us::fetch_fundamentals(client, symbol).await,
    }
}

pub(crate) async fn fetch_insider_transactions(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<Vec<NewsItem>> {
    match client.detect_market(symbol) {
        MarketKind::AShare => a_share::fetch_insider_transactions(client, symbol).await,
        MarketKind::HongKong => hk::fetch_insider_transactions(client, symbol).await,
        MarketKind::UsEquity => us::fetch_insider_transactions(client, symbol).await,
    }
}

pub(crate) async fn fetch_candles(
    client: &MarketDataClient,
    symbol: &str,
    adjust: &str,
    limit: usize,
) -> anyhow::Result<CandlesWithProvider> {
    match client.detect_market(symbol) {
        MarketKind::AShare => a_share::fetch_candles(client, symbol, adjust, limit).await,
        MarketKind::HongKong => hk::fetch_candles(client, symbol, limit).await,
        MarketKind::UsEquity => us::fetch_candles(client, symbol, limit).await,
    }
}

pub(crate) async fn fetch_return_since(
    client: &MarketDataClient,
    symbol: &str,
    start_date: &str,
    holding_days: usize,
) -> anyhow::Result<Option<f64>> {
    match client.detect_market(symbol) {
        MarketKind::AShare => {
            a_share::fetch_return_since(client, symbol, start_date, holding_days).await
        }
        MarketKind::HongKong => {
            hk::fetch_return_since(client, symbol, start_date, holding_days).await
        }
        MarketKind::UsEquity => {
            us::fetch_return_since(client, symbol, start_date, holding_days).await
        }
    }
}
