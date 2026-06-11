//! Fund info from THS (同花顺).

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch fund info from THS.
    pub async fn fund_info_ths(&self, _limit: usize) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode("fund_info_ths not yet fully implemented"))
    }
}
