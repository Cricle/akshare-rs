//! Fund report data from CNINFO (巨潮资讯).

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch fund annual report data from CNINFO.
    pub async fn fund_report(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode(format!(
            "fund report for {symbol} from CNINFO requires HTML/JS parsing"
        )))
    }

    /// Fetch fund semi-annual report data from CNINFO.
    pub async fn fund_report_half_year(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode(format!(
            "fund semi-annual report for {symbol} from CNINFO requires HTML/JS parsing"
        )))
    }

    /// Fetch fund quarterly report data from CNINFO.
    pub async fn fund_report_quarter(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode(format!(
            "fund quarterly report for {symbol} from CNINFO requires HTML/JS parsing"
        )))
    }

    /// Fetch fund heavy stock holdings from CNINFO (Python: fund_report_stock).
    pub async fn fund_report_stock(&self, _date: &str) -> Result<Vec<serde_json::Value>> {
        // CNINFO requires JS-based authentication (mcode from py_mini_racer)
        Err(Error::decode(
            "fund_report_stock requires JS authentication (CNINFO mcode)",
        ))
    }

    /// Fetch fund industry allocation from CNINFO (Python: fund_report_industry_allocation).
    pub async fn fund_report_industry_allocation(
        &self,
        _date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode(
            "fund_report_industry_allocation requires JS authentication",
        ))
    }

    /// Fetch fund asset allocation from CNINFO (Python: fund_report_asset_allocation).
    pub async fn fund_report_asset_allocation(&self) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode(
            "fund_report_asset_allocation requires JS authentication",
        ))
    }
}
