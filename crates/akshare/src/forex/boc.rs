//! Forex rates from Bank of China (BOC).

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::ForexRate;

impl AkShareClient {
    /// Fetch forex rates (BOC rates).
    ///
    /// The Eastmoney datacenter report `RPT_FE_QUOTATION_BOCCN` has been retired.
    /// Falls back to Sina forex API which provides the same data.
    pub async fn forex_boc_rates(&self) -> Result<Vec<ForexRate>> {
        self.forex_sina_rates().await
    }
}
