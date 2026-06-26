//! Corporate bond issuance data from Eastmoney datacenter.
//!
//! Uses `RPT_BOND_ISSUE` report to fetch corporate bond issuance records.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::BondSnapshot;

impl AkShareClient {
    /// Fetch corporate bond issuance data.
    ///
    /// The Eastmoney datacenter report `RPT_BOND_ISSUE` has been retired.
    /// Returns an error indicating the data source is unavailable.
    pub async fn bond_corporate_yields(&self, _limit: usize) -> Result<Vec<BondSnapshot>> {
        Err(crate::error::Error::upstream(
            "bond_corporate_yields: Eastmoney report RPT_BOND_ISSUE has been retired.",
        ))
    }
}
