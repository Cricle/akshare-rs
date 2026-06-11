use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::*;

impl AkShareClient {
    /// Get US stock quote. Derived from the latest candle of `us_candles`.
    pub async fn us_quote(&self, symbol: &str) -> Result<QuoteSnapshot> {
        let mut candles = self.us_candles(symbol, 2).await?;
        let last = candles
            .pop()
            .ok_or_else(|| Error::upstream("no US quote data"))?;
        Ok(QuoteSnapshot {
            symbol: symbol.to_uppercase(),
            date: last.trade_date,
            open: last.open,
            high: last.high,
            low: last.low,
            close: last.close,
            volume: last.volume,
        })
    }

    /// Get US stock candles with fallback: Sina -> Yahoo -> Stooq
    pub async fn us_candles(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        // Try Sina first
        match self.sina_us_daily(symbol, limit).await {
            Ok(items) if !items.is_empty() => return Ok(items),
            _ => {}
        }

        // Try Yahoo
        match self.yahoo_candles(symbol, limit).await {
            Ok(items) if !items.is_empty() => return Ok(items),
            _ => {}
        }

        // Fallback to Stooq
        self.stooq_candles(symbol, limit).await
    }
}
