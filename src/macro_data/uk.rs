//! UK macro-economic data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::fetch_em_indicator;

const REPORT: &str = "RPT_ECONOMICVALUE_BRITAIN";

impl AkShareClient {
    /// UK Halifax house price index monthly (Halifax房价指数月率).
    pub async fn uk_halifax_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00342256", "UK Halifax Monthly").await
    }

    /// UK Halifax house price index yearly (Halifax房价指数年率).
    pub async fn uk_halifax_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00010370", "UK Halifax Yearly").await
    }

    /// UK trade balance (贸易帐).
    pub async fn uk_trade(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00158309", "UK Trade").await
    }

    /// UK bank rate (央行公布利率决议).
    pub async fn uk_bank_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00342253", "UK Bank Rate").await
    }

    /// UK core CPI yearly (核心消费者物价指数年率).
    pub async fn uk_core_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00010279", "UK Core CPI Yearly").await
    }

    /// UK core CPI monthly (核心消费者物价指数月率).
    pub async fn uk_core_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00010291", "UK Core CPI Monthly").await
    }

    /// UK CPI yearly (消费者物价指数年率).
    pub async fn uk_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00010267", "UK CPI Yearly").await
    }

    /// UK CPI monthly (消费者物价指数月率).
    pub async fn uk_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00010291", "UK CPI Monthly").await
    }

    /// UK retail sales monthly (零售销售月率).
    pub async fn uk_retail_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00158298", "UK Retail Monthly").await
    }

    /// UK retail sales yearly (零售销售年率).
    pub async fn uk_retail_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00158297", "UK Retail Yearly").await
    }

    /// UK Rightmove house price index yearly (Rightmove房价指数年率).
    pub async fn uk_rightmove_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00341608", "UK Rightmove Yearly").await
    }

    /// UK Rightmove house price index monthly (Rightmove房价指数月率).
    pub async fn uk_rightmove_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00341607", "UK Rightmove Monthly").await
    }

    /// UK GDP quarterly (GDP季率初值).
    pub async fn uk_gdp_quarterly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00158277", "UK GDP Quarterly").await
    }

    /// UK GDP yearly (GDP年率初值).
    pub async fn uk_gdp_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00158276", "UK GDP Yearly").await
    }

    /// UK unemployment rate (失业率).
    pub async fn uk_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00010348", "UK Unemployment Rate").await
    }
}

// Python-compatible aliases
impl AkShareClient {
    pub async fn macro_uk_bank_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_bank_rate().await
    }

    pub async fn macro_uk_core_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_core_cpi_monthly().await
    }

    pub async fn macro_uk_core_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_core_cpi_yearly().await
    }

    pub async fn macro_uk_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_cpi_monthly().await
    }

    pub async fn macro_uk_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_cpi_yearly().await
    }

    pub async fn macro_uk_gdp_quarterly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_gdp_quarterly().await
    }

    pub async fn macro_uk_gdp_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_gdp_yearly().await
    }

    pub async fn macro_uk_halifax_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_halifax_monthly().await
    }

    pub async fn macro_uk_halifax_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_halifax_yearly().await
    }

    pub async fn macro_uk_retail_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_retail_monthly().await
    }

    pub async fn macro_uk_retail_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_retail_yearly().await
    }

    pub async fn macro_uk_rightmove_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_rightmove_monthly().await
    }

    pub async fn macro_uk_rightmove_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_rightmove_yearly().await
    }

    pub async fn macro_uk_trade(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_trade().await
    }

    pub async fn macro_uk_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.uk_unemployment_rate().await
    }

}
