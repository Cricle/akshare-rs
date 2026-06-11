//! Germany macro-economic data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::fetch_em_indicator;

const REPORT: &str = "RPT_ECONOMICVALUE_GER";

impl AkShareClient {
    /// Germany IFO business climate index (IFO商业景气指数).
    pub async fn germany_ifo(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00179154", "Germany IFO").await
    }

    /// Germany CPI monthly (消费者物价指数月率终值).
    pub async fn germany_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00009758", "Germany CPI Monthly").await
    }

    /// Germany CPI yearly (消费者物价指数年率终值).
    pub async fn germany_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00009756", "Germany CPI Yearly").await
    }

    /// Germany trade balance adjusted (贸易帐季调后).
    pub async fn germany_trade_adjusted(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00009753", "Germany Trade Adjusted").await
    }

    /// Germany GDP.
    pub async fn germany_gdp(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00009720", "Germany GDP").await
    }

    /// Germany retail sales monthly (实际零售销售月率).
    pub async fn germany_retail_sale_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG01333186", "Germany Retail Monthly").await
    }

    /// Germany retail sales yearly (实际零售销售年率).
    pub async fn germany_retail_sale_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG01333192", "Germany Retail Yearly").await
    }

    /// Germany ZEW economic sentiment (ZEW经济景气指数).
    pub async fn germany_zew(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00172577", "Germany ZEW").await
    }
}

// Python-compatible aliases
impl AkShareClient {
    pub async fn macro_germany_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.germany_cpi_monthly().await
    }

    pub async fn macro_germany_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.germany_cpi_yearly().await
    }

    pub async fn macro_germany_gdp(&self) -> Result<Vec<MacroDataPoint>> {
        self.germany_gdp().await
    }

    pub async fn macro_germany_ifo(&self) -> Result<Vec<MacroDataPoint>> {
        self.germany_ifo().await
    }

    pub async fn macro_germany_retail_sale_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.germany_retail_sale_monthly().await
    }

    pub async fn macro_germany_retail_sale_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.germany_retail_sale_yearly().await
    }

    pub async fn macro_germany_trade_adjusted(&self) -> Result<Vec<MacroDataPoint>> {
        self.germany_trade_adjusted().await
    }

    pub async fn macro_germany_zew(&self) -> Result<Vec<MacroDataPoint>> {
        self.germany_zew().await
    }

}
