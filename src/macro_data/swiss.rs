//! Switzerland macro-economic data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::fetch_em_indicator;

const REPORT: &str = "RPT_ECONOMICVALUE_CH";

impl AkShareClient {
    /// Switzerland SVME PMI (SVME采购经理人指数).
    pub async fn swiss_svme(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00341602", "Swiss SVME PMI").await
    }

    /// Switzerland trade balance (贸易帐).
    pub async fn swiss_trade(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00341603", "Swiss Trade").await
    }

    /// Switzerland CPI yearly (消费者物价指数年率).
    pub async fn swiss_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00341604", "Swiss CPI Yearly").await
    }

    /// Switzerland GDP quarterly (GDP季率).
    pub async fn swiss_gdp_quarterly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00341600", "Swiss GDP Quarterly").await
    }

    /// Switzerland GDP yearly (GDP年率).
    pub async fn swiss_gdp_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00341601", "Swiss GDP Yearly").await
    }

    /// Switzerland bank rate (央行公布利率决议).
    pub async fn swiss_bank_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00341606", "Swiss Bank Rate").await
    }
}

// Python-compatible aliases
impl AkShareClient {
    pub async fn macro_swiss_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.swiss_cpi_yearly().await
    }

    pub async fn macro_swiss_gdp_quarterly(&self) -> Result<Vec<MacroDataPoint>> {
        self.swiss_gdp_quarterly().await
    }

    pub async fn macro_swiss_svme(&self) -> Result<Vec<MacroDataPoint>> {
        self.swiss_svme().await
    }

    pub async fn macro_swiss_trade(&self) -> Result<Vec<MacroDataPoint>> {
        self.swiss_trade().await
    }

    pub async fn macro_swiss_gbd_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.swiss_gdp_yearly().await
    }

    pub async fn macro_swiss_gbd_bank_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.swiss_bank_rate().await
    }
}
