//! Fund fee data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch fund fee data from Eastmoney.
    pub async fn fund_fee_em(&self, _limit: usize) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode("fund_fee_em not yet fully implemented"))
    }
}
