//! Canada macro-economic data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::fetch_em_indicator;

const REPORT: &str = "RPT_ECONOMICVALUE_CA";

impl AkShareClient {
    /// Canada new housing starts (新屋开工).
    pub async fn canada_new_house_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00342247", "Canada New House Rate").await
    }

    /// Canada unemployment rate (失业率).
    pub async fn canada_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00157746", "Canada Unemployment Rate").await
    }

    /// Canada trade balance (贸易帐).
    pub async fn canada_trade(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00102022", "Canada Trade").await
    }

    /// Canada retail sales monthly (零售销售月率).
    pub async fn canada_retail_rate_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG01337094", "Canada Retail Monthly").await
    }

    /// Canada bank rate (央行公布利率决议).
    pub async fn canada_bank_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00342248", "Canada Bank Rate").await
    }

    /// Canada core CPI yearly (核心消费者物价指数年率).
    pub async fn canada_core_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00102030", "Canada Core CPI Yearly").await
    }

    /// Canada core CPI monthly (核心消费者物价指数月率).
    pub async fn canada_core_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00102044", "Canada Core CPI Monthly").await
    }

    /// Canada CPI yearly (消费者物价指数年率).
    pub async fn canada_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00102029", "Canada CPI Yearly").await
    }

    /// Canada CPI monthly (消费者物价指数月率).
    pub async fn canada_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00158719", "Canada CPI Monthly").await
    }

    /// Canada GDP monthly (GDP月率).
    pub async fn canada_gdp_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00159259", "Canada GDP Monthly").await
    }
}

// Python-compatible aliases
impl AkShareClient {
    pub async fn macro_canada_bank_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.canada_bank_rate().await
    }

    pub async fn macro_canada_core_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.canada_core_cpi_monthly().await
    }

    pub async fn macro_canada_core_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.canada_core_cpi_yearly().await
    }

    pub async fn macro_canada_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.canada_cpi_monthly().await
    }

    pub async fn macro_canada_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.canada_cpi_yearly().await
    }

    pub async fn macro_canada_gdp_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.canada_gdp_monthly().await
    }

    pub async fn macro_canada_new_house_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.canada_new_house_rate().await
    }

    pub async fn macro_canada_retail_rate_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.canada_retail_rate_monthly().await
    }

    pub async fn macro_canada_trade(&self) -> Result<Vec<MacroDataPoint>> {
        self.canada_trade().await
    }

    pub async fn macro_canada_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.canada_unemployment_rate().await
    }
}
