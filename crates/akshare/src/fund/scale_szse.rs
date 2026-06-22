//! Fund scale data from SZSE.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch SZSE fund scale daily data (Python: fund_scale_daily).
    ///
    /// `start_date` / `end_date`: format "YYYYMMDD".
    /// `symbol`: "ETF", "LOF", or "REITS".
    pub async fn fund_scale_daily(
        &self,
        _start_date: &str,
        _end_date: &str,
        _symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        // SZSE returns xlsx content; cannot parse in pure Rust.
        Err(Error::decode(
            "SZSE fund scale daily data requires xlsx parsing",
        ))
    }
}
