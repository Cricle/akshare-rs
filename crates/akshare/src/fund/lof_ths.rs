//! LOF fund data from THS.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch LOF fund data from THS.
    pub async fn fund_lof(&self, _limit: usize) -> Result<Vec<serde_json::Value>> {
        Err(Error::unavailable(
            "THS fund_lof has no one-to-one upstream function for the legacy limit-only signature",
        ))
    }
}
