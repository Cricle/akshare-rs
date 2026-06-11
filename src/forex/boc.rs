//! Forex rates from Bank of China (BOC).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::ForexRate;

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
    /// Fetch forex rates from Bank of China via Eastmoney datacenter.
    ///
    /// Returns the latest BOC forex rates for major currency pairs against CNY.
    pub async fn forex_boc_rates(&self) -> Result<Vec<ForexRate>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_FE_QUOTATION_BOCCN"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "50"),
                ("sortTypes", "-1"),
                ("sortColumns", "DATE"),
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
            let currency_pair = v
                .get("CURRENCY_NAME")
                .or_else(|| v.get("CURRENCY_CODE"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if currency_pair.is_empty() {
                continue;
            }

            let buy_rate = v
                .get("BUYING_RATE")
                .or_else(|| v.get("BUY_RATE"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);
            let sell_rate = v
                .get("SELLING_RATE")
                .or_else(|| v.get("SELL_RATE"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);
            let middle_rate = v
                .get("MIDDLE_RATE")
                .or_else(|| v.get("CENTRAL_RATE"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);
            let date = v
                .get("DATE")
                .or_else(|| v.get("REPORT_DATE"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();

            items.push(ForexRate {
                currency_pair,
                buy_rate,
                sell_rate,
                middle_rate,
                date: date.get(..10).unwrap_or(&date).to_string(),
                change_pct: None,
            });
        }
        Ok(items)
    }
}
