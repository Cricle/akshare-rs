//! Global macro-economic data.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::fetch_em_industry_index;

impl AkShareClient {
    /// Philadelphia Semiconductor Index (费城半导体指数).
    pub async fn macro_global_sox_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00055562", "Global SOX Index").await
    }

    /// BCI - Baltic Capesize Index (海岬型运费指数).
    pub async fn macro_shipping_bci(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00107666", "Shipping BCI").await
    }

    /// BDI - Baltic Dry Index (波罗的海干散货指数).
    pub async fn macro_shipping_bdi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00107664", "Shipping BDI").await
    }

    /// BPI - Baltic Panamax Index (巴拿马型运费指数).
    pub async fn macro_shipping_bpi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00107665", "Shipping BPI").await
    }

    /// BCTI - Baltic Clean Tanker Index (成品油运输指数).
    pub async fn macro_shipping_bcti(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00107669", "Shipping BCTI").await
    }
}
