//! China government bond yield data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::BondSnapshot;
use crate::types::value_ext::ValueExt;
use crate::types::wire::EmDatacenterResp;

impl AkShareClient {
    /// Fetch China government bond yield curve data.
    ///
    /// `start` and `end` are date strings in "YYYY-MM-DD" format.
    pub async fn bond_china_yield(&self, start: &str, end: &str) -> Result<Vec<BondSnapshot>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let filter = format!("(SOLAR_DATE>='{start}')(SOLAR_DATE<='{end}')");
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_BOND_GOV_CN_YIELD"),
                ("columns", "ALL"),
                ("filter", filter.as_str()),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "SOLAR_DATE"),
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
            let date = v.str_or(&["SOLAR_DATE"], "");
            if date.is_empty() {
                continue;
            }

            // China bond yields come in multiple tenor columns
            // Emit one BondSnapshot per tenor found
            let tenors = [
                ("EMG01446460", "1Y"),
                ("EMG01446461", "2Y"),
                ("EMG01446462", "3Y"),
                ("EMG01446463", "5Y"),
                ("EMG01446464", "7Y"),
                ("EMG01446465", "10Y"),
                ("EMG01446466", "30Y"),
            ];

            for (field, tenor_label) in &tenors {
                if let Some(rate) = v.f64_field(&[*field]) {
                    items.push(BondSnapshot {
                        symbol: format!("CNGB{tenor_label}"),
                        name: format!("China Gov Bond {tenor_label}"),
                        date: date.get(..10).unwrap_or(&date).to_string(),
                        close: 0.0,
                        change_pct: 0.0,
                        yield_rate: Some(rate),
                        credit_rating: None,
                    });
                }
            }
        }

        Ok(items)
    }
}
