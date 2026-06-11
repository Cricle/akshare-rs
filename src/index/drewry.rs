//! Drewry World Container Index.
//!
//! The upstream scrapes Infogram HTML which is not feasible in pure Rust.
//! We provide a stub.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Drewry 集装箱指数.
    ///
    /// The upstream scrapes Infogram JS/HTML. Returns an error.
    pub async fn drewry_wci_index(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        Err(Error::upstream(format!(
            "drewry_wci_index({symbol}) requires Infogram HTML scraping; \
             not supported in pure Rust"
        )))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Drewry functions require HTML scraping.
    }
}
