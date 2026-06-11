//! Fund position data from Legu (乐咕).

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundPositionPoint;

impl AkShareClient {
    /// Fetch fund position estimates from Legu.
    pub async fn fund_position_lg(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode(format!(
            "Legu fund position data for {symbol} requires API authentication"
        )))
    }

    /// Fetch fund position estimate history from Legu.
    pub async fn fund_position_hist_lg(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode(format!(
            "Legu fund position history for {symbol} requires API authentication"
        )))
    }

    /// Fetch fund position estimate summary from Legu.
    pub async fn fund_position_est_lg(&self) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode(
            "Legu fund position estimate requires API authentication",
        ))
    }

    /// Fetch stock-type fund position from Legu (Python: fund_stock_position_lg).
    pub async fn fund_stock_position_lg(&self) -> Result<Vec<FundPositionPoint>> {
        Err(Error::decode(
            "Legu fund stock position requires API authentication and cookie",
        ))
    }

    /// Fetch balanced fund position from Legu (Python: fund_balance_position_lg).
    pub async fn fund_balance_position_lg(&self) -> Result<Vec<FundPositionPoint>> {
        Err(Error::decode(
            "Legu fund balance position requires API authentication and cookie",
        ))
    }

    /// Fetch flexible allocation fund position from Legu (Python: fund_linghuo_position_lg).
    pub async fn fund_linghuo_position_lg(&self) -> Result<Vec<FundPositionPoint>> {
        Err(Error::decode(
            "Legu fund linghuo position requires API authentication and cookie",
        ))
    }
}
