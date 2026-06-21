//! China government bond yield data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::BondSnapshot;
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
            let date = v
                .get("SOLAR_DATE")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
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
                let yield_rate = v.get(*field).and_then(serde_json::Value::as_f64);
                if let Some(rate) = yield_rate {
                    items.push(BillSnapshotInner {
                        symbol: format!("CNGB{tenor_label}"),
                        name: format!("China Gov Bond {tenor_label}"),
                        date: date.get(..10).unwrap_or(&date).to_string(),
                        yield_rate: Some(rate),
                    });
                }
            }
        }

        Ok(items
            .into_iter()
            .map(|b| BondSnapshot {
                symbol: b.symbol,
                name: b.name,
                date: b.date,
                close: 0.0,
                change_pct: 0.0,
                yield_rate: b.yield_rate,
                credit_rating: None,
            })
            .collect())
    }
}

/// Intermediate struct for bond yield extraction.
struct BillSnapshotInner {
    symbol: String,
    name: String,
    date: String,
    yield_rate: Option<f64>,
}
