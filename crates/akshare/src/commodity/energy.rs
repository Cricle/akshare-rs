//! Energy commodity data: oil prices and carbon trading from Eastmoney.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;
use crate::types::wire::EmDatacenterResp;

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
            let date = v.str_or(&["REPORT_DATE"], "");
            if date.is_empty() {
                continue;
            }
            let value = v.f64_or(
                &["INDICATOR_VALUE", "VALUE", "BRENT_PRICE", "WTI_PRICE"],
                0.0,
            );
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
            let date = v.str_or(&["TRADE_DATE"], "");
            if date.is_empty() {
                continue;
            }
            let value = v.f64_or(&["CLOSE_PRICE", "PRICE"], 0.0);
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value,
                name: "Carbon Trading".to_string(),
            });
        }
        Ok(items)
    }
}
