//! Yahoo Finance chart API helpers.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::CandlePoint;
use crate::util::{amplitude_pct, apply_change_metrics};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct YfChartResponse {
    chart: YfChart,
}

#[derive(Debug, Deserialize)]
struct YfChart {
    #[serde(default)]
    result: Vec<YfChartResult>,
}

#[derive(Debug, Deserialize)]
struct YfChartResult {
    timestamp: Option<Vec<i64>>,
    indicators: YfIndicators,
}

#[derive(Debug, Deserialize)]
struct YfIndicators {
    quote: Vec<YfQuote>,
}

#[derive(Debug, Deserialize)]
struct YfQuote {
    open: Vec<Option<f64>>,
    high: Vec<Option<f64>>,
    low: Vec<Option<f64>>,
    close: Vec<Option<f64>>,
    volume: Vec<Option<i64>>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Fetch candlestick data from Yahoo Finance chart API.
    pub(crate) async fn yahoo_candles(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let (interval, range) = if limit <= 60 {
            ("1d", format!("{limit}d"))
        } else if limit <= 120 {
            ("1d", "6mo".to_string())
        } else {
            ("1d", "1y".to_string())
        };

        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{symbol}?interval={interval}&range={range}"
        );

        let resp: YfChartResponse = self
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?
            .json()
            .await?;

        let result = resp
            .chart
            .result
            .into_iter()
            .next()
            .ok_or_else(|| Error::upstream("empty yahoo chart response"))?;

        let timestamps = result.timestamp.unwrap_or_default();
        let quote = result
            .indicators
            .quote
            .into_iter()
            .next()
            .ok_or_else(|| Error::upstream("yahoo: no quote data"))?;

        let mut items = Vec::with_capacity(timestamps.len());
        for (i, ts) in timestamps.iter().enumerate() {
            let open = quote.open.get(i).and_then(|v| *v).unwrap_or(0.0);
            let high = quote.high.get(i).and_then(|v| *v).unwrap_or(0.0);
            let low = quote.low.get(i).and_then(|v| *v).unwrap_or(0.0);
            let close = quote.close.get(i).and_then(|v| *v).unwrap_or(0.0);
            let volume = quote.volume.get(i).and_then(|v| *v).unwrap_or(0);

            let date = chrono::DateTime::from_timestamp(*ts, 0)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();

            items.push(CandlePoint {
                trade_date: date,
                open,
                close,
                high,
                low,
                volume,
                amount: 0.0,
                amplitude_pct: amplitude_pct(high, low),
                change_pct: 0.0,
                change_amount: 0.0,
                turnover_pct: 0.0,
            });
        }
        apply_change_metrics(&mut items);

        if items.len() > limit {
            items = items[items.len() - limit..].to_vec();
        }
        Ok(items)
    }
}
