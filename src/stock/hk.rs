use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::*;

/// Convert an HK stock symbol (e.g. "00593", "593") to Yahoo format ("00593.HK").
fn hk_yahoo_symbol(symbol: &str) -> Result<String> {
    let trimmed = symbol.trim();
    // Strip .HK suffix if present, then re-normalize
    let code = if let Some((c, suffix)) = trimmed.split_once('.') {
        if suffix.eq_ignore_ascii_case("HK") {
            c
        } else {
            trimmed
        }
    } else {
        trimmed
    };

    // Validate: 1-5 digit numeric
    let digits: String = code.trim_start_matches('0').to_string();
    let digits_str = if digits.is_empty() { "0" } else { &digits };
    if digits_str.len() > 5 || !digits_str.chars().all(|c| c.is_ascii_digit()) {
        return Err(Error::invalid_input(format!(
            "invalid HK symbol: {symbol}"
        )));
    }

    Ok(format!("{:0>5}.HK", digits_str))
}

impl AkShareClient {
    /// Get HK stock quote with fallback: Tencent -> Yahoo candles
    pub async fn hk_quote(&self, symbol: &str) -> Result<QuoteSnapshot> {
        // Try Tencent first
        if let Ok(quote) = self.tencent_hk_quote(symbol).await { return Ok(quote) }

        // Fallback: get from Yahoo candles
        let yahoo_symbol = hk_yahoo_symbol(symbol)?;
        let mut candles = self.yahoo_candles(&yahoo_symbol, 2).await?;
        let last = candles
            .pop()
            .ok_or_else(|| Error::upstream("no HK quote data"))?;
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

    /// Get HK stock candles with fallback: Tencent -> Yahoo
    pub async fn hk_candles(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        // Try Tencent first
        match self.tencent_hk_candles(symbol, limit).await {
            Ok(items) if !items.is_empty() => return Ok(items),
            _ => {}
        }

        // Fallback to Yahoo
        let yahoo_symbol = hk_yahoo_symbol(symbol)?;
        self.yahoo_candles(&yahoo_symbol, limit).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hk_yahoo_symbol() {
        assert_eq!(hk_yahoo_symbol("00593").unwrap(), "00593.HK");
        assert_eq!(hk_yahoo_symbol("593").unwrap(), "00593.HK");
        assert_eq!(hk_yahoo_symbol("00593.HK").unwrap(), "00593.HK");
        assert_eq!(hk_yahoo_symbol("1").unwrap(), "00001.HK");
        assert_eq!(hk_yahoo_symbol("9988").unwrap(), "09988.HK");
        assert!(hk_yahoo_symbol("AAPL").is_err());
    }
}
