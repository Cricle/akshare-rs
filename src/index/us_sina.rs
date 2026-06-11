//! US index data from Sina.
//!
//! The Python implementation uses JS decoding which is not available in pure Rust.
//! We provide a stub.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// 新浪财经 — 美股指数行情.
    ///
    /// The upstream requires JS decoding. Returns an error.
    pub async fn index_us_stock_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        Err(Error::upstream(format!(
            "index_us_stock_sina({symbol}) requires JS decoding; \
             use yahoo_candles() or a JS-capable runtime"
        )))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // US Sina functions require JS decoding.
    }
}
