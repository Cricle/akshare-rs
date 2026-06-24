use super::types::ProviderResult;
use super::super::{CandlePoint, FundamentalsSnapshot, MarketDataClient, NewsItem, QuoteSnapshot};

use crate::stock::hk_extra::{
    HkFamousStock, HkFhpxDetailThs, HkGxlLg, HkHotRank, HkHotRankDetail, HkSpotQuote,
    HkValuationBaidu,
};
use crate::stock::xueqiu::XqStockSpot;

// ---------------------------------------------------------------------------
// Core data
// ---------------------------------------------------------------------------

pub(crate) async fn fetch_quote(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<ProviderResult<QuoteSnapshot>> {
    client.fetch_hk_quote_with_provider(symbol).await
}

pub(crate) async fn fetch_fundamentals(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<FundamentalsSnapshot> {
    client.fetch_hk_fundamentals(symbol).await
}

pub(crate) async fn fetch_insider_transactions(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<Vec<NewsItem>> {
    client.fetch_hk_news(symbol, 8, None, None).await
}

pub(crate) async fn fetch_candles(
    client: &MarketDataClient,
    symbol: &str,
    limit: usize,
) -> anyhow::Result<ProviderResult<Vec<CandlePoint>>> {
    client.fetch_hk_candles_with_provider(symbol, limit).await
}

pub(crate) async fn fetch_return_since(
    client: &MarketDataClient,
    symbol: &str,
    start_date: &str,
    holding_days: usize,
) -> anyhow::Result<Option<f64>> {
    match client
        .fetch_hk_return_since(symbol, start_date, holding_days)
        .await
    {
        Ok(value) => Ok(value),
        Err(error) => {
            tracing::info!(
                symbol = %symbol,
                start_date = %start_date,
                holding_days,
                error = ?error,
                "HK return_since upstream unavailable, skipping outcome computation"
            );
            Ok(None)
        }
    }
}

// ---------------------------------------------------------------------------
// HK spot (Sina)
// ---------------------------------------------------------------------------

pub(crate) async fn fetch_hk_spot(client: &MarketDataClient) -> anyhow::Result<Vec<HkSpotQuote>> {
    client.ak.stock_hk_spot().await.map_err(Into::into)
}

// ---------------------------------------------------------------------------
// HK famous stocks (Eastmoney)
// ---------------------------------------------------------------------------

pub(crate) async fn fetch_hk_famous_spot(
    client: &MarketDataClient,
) -> anyhow::Result<Vec<HkFamousStock>> {
    client.ak.stock_hk_famous_spot().await.map_err(Into::into)
}

// ---------------------------------------------------------------------------
// HK hot rank (Eastmoney)
// ---------------------------------------------------------------------------

pub(crate) async fn fetch_hk_hot_rank(client: &MarketDataClient) -> anyhow::Result<Vec<HkHotRank>> {
    client.ak.stock_hk_hot_rank().await.map_err(Into::into)
}

pub(crate) async fn fetch_hk_hot_rank_latest(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<Vec<HkHotRankDetail>> {
    client
        .ak
        .stock_hk_hot_rank_latest(symbol)
        .await
        .map_err(Into::into)
}

pub(crate) async fn fetch_hk_hot_rank_detail(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<Vec<HkHotRankDetail>> {
    client
        .ak
        .stock_hk_hot_rank_detail(symbol)
        .await
        .map_err(Into::into)
}

pub(crate) async fn fetch_hk_hot_rank_realtime(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<Vec<HkHotRankDetail>> {
    client
        .ak
        .stock_hk_hot_rank_detail_realtime(symbol)
        .await
        .map_err(Into::into)
}

// ---------------------------------------------------------------------------
// HK dividends (Eastmoney + THS)
// ---------------------------------------------------------------------------

pub(crate) async fn fetch_hk_dividend_payout(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<Vec<serde_json::Value>> {
    client
        .ak
        .stock_hk_dividend_payout(symbol)
        .await
        .map_err(Into::into)
}

pub(crate) async fn fetch_hk_fhpx_detail(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<Vec<HkFhpxDetailThs>> {
    client
        .ak
        .stock_hk_fhpx_detail(symbol)
        .await
        .map_err(Into::into)
}

pub(crate) async fn fetch_hk_dividend_yield(
    client: &MarketDataClient,
) -> anyhow::Result<Vec<HkGxlLg>> {
    client.ak.stock_hk_gxl_lg().await.map_err(Into::into)
}

// ---------------------------------------------------------------------------
// HK financial indicators (Eastmoney)
// ---------------------------------------------------------------------------

pub(crate) async fn fetch_hk_financial_indicators(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<Vec<serde_json::Value>> {
    client
        .ak
        .stock_hk_financial_indicator(symbol)
        .await
        .map_err(Into::into)
}

// ---------------------------------------------------------------------------
// HK valuation (Baidu)
// ---------------------------------------------------------------------------

pub(crate) async fn fetch_hk_valuation(
    client: &MarketDataClient,
    symbol: &str,
    indicator: &str,
    period: &str,
) -> anyhow::Result<Vec<HkValuationBaidu>> {
    client
        .ak
        .stock_hk_valuation(symbol, indicator, period)
        .await
        .map_err(Into::into)
}

// ---------------------------------------------------------------------------
// Xueqiu spot (works for HK symbols)
// ---------------------------------------------------------------------------

pub(crate) async fn fetch_xq_spot(
    client: &MarketDataClient,
    symbol: &str,
) -> anyhow::Result<XqStockSpot> {
    client
        .ak
        .stock_individual_spot_xq(symbol)
        .await
        .map_err(Into::into)
}
