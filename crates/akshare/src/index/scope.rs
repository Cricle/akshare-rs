//! 数库 (Chinascope) A股新闻情绪指数.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::NewsSentimentPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ScopeRow {
    #[serde(default, rename = "tradeDate")]
    trade_date: String,
    #[serde(default, rename = "maIndex1")]
    ma_index1: Option<f64>,
    #[serde(default, rename = "marketClose")]
    market_close: Option<f64>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 数库 — A股新闻情绪指数.
    pub async fn index_news_sentiment_scope(&self) -> Result<Vec<NewsSentimentPoint>> {
        let response = crate::util::send_and_check(
            self.get("https://www.chinascope.com/inews/senti/index")
                .query(&[("period", "YEAR")])
        )
        .await?;

        let rows: Vec<ScopeRow> = response.json().await.map_err(Error::from)?;

        let points: Vec<NewsSentimentPoint> = rows
            .into_iter()
            .map(|r| NewsSentimentPoint {
                date: r.trade_date,
                sentiment_index: r.ma_index1.unwrap_or(0.0),
                csi300: r.market_close.unwrap_or(0.0),
            })
            .collect();

        if points.is_empty() {
            return Err(Error::not_found("chinascope returned no sentiment data"));
        }
        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Scope functions require network access.
    }
}
