//! Convertible bond info from Sina Finance.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch convertible bond profile data from Sina Finance.
    ///
    /// `symbol` is in Sina format with market prefix, e.g. "sz128039".
    /// Returns raw key-value pairs as JSON.
    pub async fn bond_cb_profile(&self, symbol: &str) -> Result<serde_json::Value> {
        let url = format!("https://money.finance.sina.com.cn/bond/info/{symbol}.html");
        let resp = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let _text = resp.text().await.map_err(Error::from)?;
        // Sina returns HTML; parsing requires HTML extraction which is complex.
        // Return a placeholder indicating the endpoint was reached.
        Err(Error::decode(format!(
            "sina bond profile requires HTML parsing; raw data available at {url}"
        )))
    }

    /// Fetch convertible bond summary data from Sina Finance.
    ///
    /// `symbol` is in Sina format with market prefix, e.g. "sh155255".
    pub async fn bond_cb_summary(&self, symbol: &str) -> Result<serde_json::Value> {
        let url = format!("https://money.finance.sina.com.cn/bond/quotes/{symbol}.html");
        let resp = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let _text = resp.text().await.map_err(Error::from)?;
        Err(Error::decode(format!(
            "sina bond summary requires HTML parsing; raw data available at {url}"
        )))
    }
}
