//! ETF historical data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::CandlePoint;

/// Map common ETF symbols to Eastmoney secid.
///
/// ETFs listed on Shanghai use market prefix "1", Shenzhen use "0".
fn etf_secid(symbol: &str) -> Result<String> {
    let s = symbol.trim();
    // Already a secid?
    if s.contains('.') && s.len() >= 3 {
        return Ok(s.to_string());
    }
    // Pure numeric: infer exchange
    if s.len() == 6 && s.chars().all(|c| c.is_ascii_digit()) {
        let prefix = if s.starts_with('5') || s.starts_with('5') {
            "1" // Shanghai ETFs: 510xxx, 512xxx, 513xxx, 515xxx, 518xxx
        } else if s.starts_with('1') {
            "0" // Shenzhen ETFs: 159xxx
        } else {
            "1" // default Shanghai
        };
        return Ok(format!("{prefix}.{s}"));
    }
    Err(Error::invalid_input(format!(
        "invalid ETF symbol: {symbol}"
    )))
}

impl AkShareClient {
    /// Get ETF historical candles.
    ///
    /// `symbol` can be a 6-digit code (e.g. "510300") or a raw secid like "1.510300".
    pub async fn fund_etf_hist(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        let secid = etf_secid(symbol)?;
        self.eastmoney_klines(&secid, "qfq", limit).await
    }
}
