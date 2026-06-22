//! LOF fund data from THS.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch LOF fund data from THS.
    pub async fn fund_lof(&self, _limit: usize) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode("fund_lof not yet fully implemented"))
    }
}
