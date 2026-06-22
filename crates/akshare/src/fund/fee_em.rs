//! Fund fee data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch fund fee data from Eastmoney.
    pub async fn fund_fee(&self, _limit: usize) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode("fund_fee not yet fully implemented"))
    }
}
