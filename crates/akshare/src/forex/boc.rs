//! Forex rates from Bank of China (BOC).

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::ForexRate;
use crate::types::value_ext::ValueExt;
use crate::types::wire::EmDatacenterResp;

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
            let currency_pair = v.str_or(&["CURRENCY_NAME", "CURRENCY_CODE"], "");
            if currency_pair.is_empty() {
                continue;
            }

            let buy_rate = v.f64_or(&["BUYING_RATE", "BUY_RATE"], 0.0);
            let sell_rate = v.f64_or(&["SELLING_RATE", "SELL_RATE"], 0.0);
            let middle_rate = v.f64_or(&["MIDDLE_RATE", "CENTRAL_RATE"], 0.0);
            let date = v.str_or(&["DATE", "REPORT_DATE"], "");

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
