//! China government bond yield data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::BondSnapshot;

impl AkShareClient {
    /// Fetch China government bond yield curve data.
    ///
    /// The Eastmoney datacenter report `RPT_BOND_GOV_CN_YIELD` has been retired.
    /// The original Python akshare uses ChinaBond website which requires JS rendering.
    /// Returns an error indicating the data source is unavailable.
    pub async fn bond_china_yield(&self, _start: &str, _end: &str) -> Result<Vec<BondSnapshot>> {
        Err(crate::error::Error::upstream(
            "bond_china_yield: Eastmoney report RPT_BOND_GOV_CN_YIELD has been retired. \
             ChinaBond website requires JavaScript rendering and cannot be accessed via simple HTTP.",
        ))
    }
}
