//! Hong Kong macro-economic data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::fetch_em_indicator;

const REPORT: &str = "RPT_ECONOMICVALUE_HK";

impl AkShareClient {
    /// Hong Kong CPI (消费者物价指数).
    pub async fn hk_cpi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG01336996", "HK CPI").await
    }

    /// Hong Kong CPI YoY (消费者物价指数年率).
    pub async fn hk_cpi_ratio(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00059282", "HK CPI Ratio").await
    }

    /// Hong Kong unemployment rate (失业率).
    pub async fn hk_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00059647", "HK Unemployment Rate").await
    }

    /// Hong Kong GDP.
    pub async fn hk_gdp(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG01337008", "HK GDP").await
    }

    /// Hong Kong GDP YoY (香港GDP同比).
    pub async fn hk_gdp_ratio(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG01337009", "HK GDP Ratio").await
    }

    /// Hong Kong building transaction volume (楼宇买卖合约数量).
    pub async fn hk_building_volume(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00158055", "HK Building Volume").await
    }

    /// Hong Kong building transaction amount (楼宇买卖合约成交金额).
    pub async fn hk_building_amount(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00158066", "HK Building Amount").await
    }

    /// Hong Kong trade balance YoY (商品贸易差额年率).
    pub async fn hk_trade_diff_ratio(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00157898", "HK Trade Diff Ratio").await
    }

    /// Hong Kong manufacturing PPI YoY (制造业PPI年率).
    pub async fn hk_ppi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, REPORT, "EMG00157818", "HK PPI").await
    }
}

// Python-compatible aliases
impl AkShareClient {
    pub async fn macro_china_hk_building_amount(&self) -> Result<Vec<MacroDataPoint>> {
        self.hk_building_amount().await
    }

    pub async fn macro_china_hk_building_volume(&self) -> Result<Vec<MacroDataPoint>> {
        self.hk_building_volume().await
    }

    pub async fn macro_china_hk_cpi(&self) -> Result<Vec<MacroDataPoint>> {
        self.hk_cpi().await
    }

    pub async fn macro_china_hk_cpi_ratio(&self) -> Result<Vec<MacroDataPoint>> {
        self.hk_cpi_ratio().await
    }

    pub async fn macro_china_hk_ppi(&self) -> Result<Vec<MacroDataPoint>> {
        self.hk_ppi().await
    }

    pub async fn macro_china_hk_trade_diff_ratio(&self) -> Result<Vec<MacroDataPoint>> {
        self.hk_trade_diff_ratio().await
    }

    pub async fn macro_china_hk_gbp(&self) -> Result<Vec<MacroDataPoint>> {
        self.hk_gdp().await
    }

    pub async fn macro_china_hk_gbp_ratio(&self) -> Result<Vec<MacroDataPoint>> {
        self.hk_gdp_ratio().await
    }

    pub async fn macro_china_hk_rate_of_unemployment(&self) -> Result<Vec<MacroDataPoint>> {
        self.hk_unemployment_rate().await
    }
}
