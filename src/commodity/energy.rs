//! Energy commodity data: oil prices and carbon trading from Eastmoney.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

#[derive(Debug, Deserialize)]
struct EmDatacenterResp {
    result: Option<EmResult>,
}

#[derive(Debug, Deserialize)]
struct EmResult {
    #[serde(default)]
    data: Vec<serde_json::Value>,
}

impl AkShareClient {
    /// Historical oil price data (Brent, WTI) from Eastmoney.
    pub async fn energy_oil_hist(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_ECONOMY_OIL_PRICE"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        let mut items = Vec::with_capacity(data.len());
        for v in &data {
            let date = v
                .get("REPORT_DATE")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if date.is_empty() {
                continue;
            }
            let value = v
                .get("INDICATOR_VALUE")
                .or_else(|| v.get("VALUE"))
                .or_else(|| v.get("BRENT_PRICE"))
                .or_else(|| v.get("WTI_PRICE"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value,
                name: "Oil Price".to_string(),
            });
        }
        Ok(items)
    }

    /// China domestic carbon trading prices from Eastmoney.
    pub async fn energy_carbon_domestic(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_CARBON_TRADING"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "TRADE_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        let mut items = Vec::with_capacity(data.len());
        for v in &data {
            let date = v
                .get("TRADE_DATE")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if date.is_empty() {
                continue;
            }
            let value = v
                .get("CLOSE_PRICE")
                .or_else(|| v.get("PRICE"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value,
                name: "Carbon Trading".to_string(),
            });
        }
        Ok(items)
    }
}
