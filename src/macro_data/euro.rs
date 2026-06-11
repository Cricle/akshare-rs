#![allow(dead_code)]
//! Euro area macro-economic data from Jin10 datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::fetch_jin10_report;

// ---------------------------------------------------------------------------
// Jin10 datacenter — Euro area macro indicators
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Euro area GDP YoY (欧元区季度GDP年率).
    pub async fn euro_gdp_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "84", "Euro GDP YoY").await
    }

    /// Euro area CPI MoM (欧元区CPI月率).
    pub async fn euro_cpi_mom(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "84", "Euro CPI MoM").await
    }

    /// Euro area CPI YoY (欧元区CPI年率).
    pub async fn euro_cpi_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "8", "Euro CPI YoY").await
    }

    /// Euro area PPI MoM (欧元区PPI月率).
    pub async fn euro_ppi_mom(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "36", "Euro PPI MoM").await
    }

    /// Euro area retail sales MoM (欧元区零售销售月率).
    pub async fn euro_retail_sales_mom(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "38", "Euro Retail Sales MoM").await
    }

    /// Euro area employment change QoQ (欧元区就业人数季率).
    pub async fn euro_employment_change_qoq(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "14", "Euro Employment Change QoQ").await
    }

    /// Euro area unemployment rate (欧元区失业率).
    pub async fn euro_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "46", "Euro Unemployment Rate").await
    }

    /// Euro area trade balance (欧元区未季调贸易帐).
    pub async fn euro_trade_balance(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "43", "Euro Trade Balance").await
    }

    /// Euro area current account MoM (欧元区经常帐).
    pub async fn euro_current_account_mom(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "11", "Euro Current Account MoM").await
    }

    /// Euro area industrial production MoM (欧元区工业产出月率).
    pub async fn euro_industrial_production_mom(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "19", "Euro Industrial Production MoM").await
    }

    /// Euro area manufacturing PMI (欧元区制造业PMI初值).
    pub async fn euro_manufacturing_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "30", "Euro Manufacturing PMI").await
    }

    /// Euro area services PMI (欧元区服务业PMI).
    pub async fn euro_services_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "41", "Euro Services PMI").await
    }

    /// Euro area ZEW economic sentiment (欧元区ZEW经济景气指数).
    pub async fn euro_zew_economic_sentiment(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "48", "Euro ZEW Economic Sentiment").await
    }

    /// Euro area Sentix investor confidence (欧元区Sentix投资者信心指数).
    pub async fn euro_sentix_investor_confidence(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "40", "Euro Sentix Investor Confidence").await
    }
}

// ---------------------------------------------------------------------------
// Jin10 CDN — LME data
// ---------------------------------------------------------------------------

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Jin10CdnResp {
    values: Option<serde_json::Value>,
    keys: Option<Vec<serde_json::Value>>,
}

impl AkShareClient {
    /// LME holdings report (伦敦金属交易所持仓报告).
    pub async fn euro_lme_holding(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/lme_position.json";
        let resp: Jin10CdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, row) in obj {
                if let Some(cols) = row.as_object() {
                    for (col_name, col_data) in cols {
                        if let Some(arr) = col_data.as_array() {
                            if let Some(val) = arr.first().and_then(|v| v.as_f64()) {
                                items.push(MacroDataPoint {
                                    date: date.clone(),
                                    value: val,
                                    name: format!("LME Holding {}", col_name),
                                });
                            }
                        }
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// LME stock report (伦敦金属交易所库存报告).
    pub async fn euro_lme_stock(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/lme_stock.json";
        let resp: Jin10CdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, row) in obj {
                if let Some(cols) = row.as_object() {
                    for (col_name, col_data) in cols {
                        if let Some(arr) = col_data.as_array() {
                            if let Some(val) = arr.first().and_then(|v| v.as_f64()) {
                                items.push(MacroDataPoint {
                                    date: date.clone(),
                                    value: val,
                                    name: format!("LME Stock {}", col_name),
                                });
                            }
                        }
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }
}

// Python-compatible aliases
impl AkShareClient {
    pub async fn macro_euro_cpi_mom(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_cpi_mom().await
    }

    pub async fn macro_euro_cpi_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_cpi_yoy().await
    }

    pub async fn macro_euro_current_account_mom(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_current_account_mom().await
    }

    pub async fn macro_euro_employment_change_qoq(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_employment_change_qoq().await
    }

    pub async fn macro_euro_gdp_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_gdp_yoy().await
    }

    pub async fn macro_euro_industrial_production_mom(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_industrial_production_mom().await
    }

    pub async fn macro_euro_lme_holding(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_lme_holding().await
    }

    pub async fn macro_euro_lme_stock(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_lme_stock().await
    }

    pub async fn macro_euro_manufacturing_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_manufacturing_pmi().await
    }

    pub async fn macro_euro_ppi_mom(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_ppi_mom().await
    }

    pub async fn macro_euro_retail_sales_mom(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_retail_sales_mom().await
    }

    pub async fn macro_euro_sentix_investor_confidence(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_sentix_investor_confidence().await
    }

    pub async fn macro_euro_services_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_services_pmi().await
    }

    pub async fn macro_euro_trade_balance(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_trade_balance().await
    }

    pub async fn macro_euro_zew_economic_sentiment(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_zew_economic_sentiment().await
    }

    pub async fn macro_euro_unemployment_rate_mom(&self) -> Result<Vec<MacroDataPoint>> {
        self.euro_unemployment_rate().await
    }
}
