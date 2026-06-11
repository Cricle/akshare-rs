//! Australia macro-economic data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::fetch_em_indicator;

const REPORT: &str = "RPT_ECONOMICVALUE_AUSTRALIA";

impl AkShareClient {
    /// Australia retail sales monthly (零售销售月率).
    pub async fn australia_retail_rate_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00152903", "Australia Retail Monthly").await
    }

    /// Australia trade balance (贸易帐).
    pub async fn australia_trade(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00152793", "Australia Trade").await
    }

    /// Australia unemployment rate (失业率).
    pub async fn australia_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00101141", "Australia Unemployment Rate").await
    }

    /// Australia PPI quarterly (生产者物价指数季率).
    pub async fn australia_ppi_quarterly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00152722", "Australia PPI Quarterly").await
    }

    /// Australia CPI quarterly (消费者物价指数季率).
    pub async fn australia_cpi_quarterly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00101104", "Australia CPI Quarterly").await
    }

    /// Australia CPI yearly (消费者物价指数年率).
    pub async fn australia_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00101093", "Australia CPI Yearly").await
    }

    /// Australia bank rate (央行公布利率决议).
    pub async fn australia_bank_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00342255", "Australia Bank Rate").await
    }
}

// Python-compatible aliases
impl AkShareClient {
    pub async fn macro_australia_bank_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.australia_bank_rate().await
    }

    pub async fn macro_australia_cpi_quarterly(&self) -> Result<Vec<MacroDataPoint>> {
        self.australia_cpi_quarterly().await
    }

    pub async fn macro_australia_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.australia_cpi_yearly().await
    }

    pub async fn macro_australia_ppi_quarterly(&self) -> Result<Vec<MacroDataPoint>> {
        self.australia_ppi_quarterly().await
    }

    pub async fn macro_australia_retail_rate_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.australia_retail_rate_monthly().await
    }

    pub async fn macro_australia_trade(&self) -> Result<Vec<MacroDataPoint>> {
        self.australia_trade().await
    }

    pub async fn macro_australia_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.australia_unemployment_rate().await
    }

}
