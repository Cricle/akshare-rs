//! Japan macro-economic data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::fetch_em_indicator;

const REPORT: &str = "RPT_ECONOMICVALUE_JPAN";

impl AkShareClient {
    /// Japan bank rate (央行公布利率决议).
    pub async fn japan_bank_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00342252", "Japan Bank Rate").await
    }

    /// Japan CPI yearly (全国消费者物价指数年率).
    pub async fn japan_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00005004", "Japan CPI Yearly").await
    }

    /// Japan core CPI yearly (全国核心消费者物价指数年率).
    pub async fn japan_core_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00158099", "Japan Core CPI Yearly").await
    }

    /// Japan unemployment rate (失业率).
    pub async fn japan_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00005047", "Japan Unemployment Rate").await
    }

    /// Japan leading indicator (领先指标终值).
    pub async fn japan_leading_indicator(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00005117", "Japan Leading Indicator").await
    }
}

// Python-compatible aliases
impl AkShareClient {
    pub async fn macro_japan_bank_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.japan_bank_rate().await
    }

    pub async fn macro_japan_core_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.japan_core_cpi_yearly().await
    }

    pub async fn macro_japan_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.japan_cpi_yearly().await
    }

    pub async fn macro_japan_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.japan_unemployment_rate().await
    }

    pub async fn macro_japan_head_indicator(&self) -> Result<Vec<MacroDataPoint>> {
        self.japan_leading_indicator().await
    }
}
