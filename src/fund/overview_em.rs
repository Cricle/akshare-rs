//! Fund overview data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch fund overview from Eastmoney.
    pub async fn fund_overview_em(&self, _limit: usize) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode("fund_overview_em not yet fully implemented"))
    }
}
