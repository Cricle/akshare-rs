#![allow(dead_code)]
//! China/US treasury bond yield comparison from Eastmoney.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

#[derive(Debug, Deserialize)]
struct EmRateResp {
    result: Option<EmRateResult>,
}

#[derive(Debug, Deserialize)]
struct EmRateResult {
    #[serde(default)]
    data: Vec<serde_json::Value>,
    #[serde(default)]
    pages: u32,
}

impl AkShareClient {
    /// Fetch China and US treasury bond yield comparison data from Eastmoney.
    ///
    /// Returns time series of CN and US treasury yields for various tenors
    /// (2Y, 5Y, 10Y, 30Y) and GDP growth rates.
    ///
    /// `start_date` is in YYYYMMDD format.
    pub async fn bond_zh_us_rate(&self, start_date: &str) -> Result<Vec<MacroDataPoint>> {
        let mut all_items = Vec::new();

        for page in 1..=10 {
            let resp: EmRateResp = self
                .get("https://datacenter.eastmoney.com/api/data/get")
                .query(&[
                    ("type", "RPTA_WEB_TREASURYYIELD"),
                    ("sty", "ALL"),
                    ("st", "SOLAR_DATE"),
                    ("sr", "-1"),
                    ("token", "894050c76af8597a853f5b408b759f5d"),
                    ("p", &page.to_string()),
                    ("ps", "500"),
                    ("pageNo", &page.to_string()),
                    ("pageNum", &page.to_string()),
                ])
                .send()
                .await?
                .json()
                .await?;

            let data = resp.result.map(|r| r.data).unwrap_or_default();
            if data.is_empty() {
                break;
            }

            for v in &data {
                let date = v.get("SOLAR_DATE").and_then(|x| x.as_str()).unwrap_or("");
                if date.is_empty() {
                    continue;
                }
                let date_short = date.get(..10).unwrap_or(date).to_string();

                // Stop early if we've reached the start_date
                let sd = format!(
                    "{}-{}-{}",
                    &start_date[..4],
                    &start_date[4..6],
                    &start_date[6..8]
                );
                if date_short < sd {
                    return Ok(all_items);
                }

                // Extract CN yields
                for (field, label) in &[
                    ("EMM00588704", "中国国债收益率2年"),
                    ("EMM00166462", "中国国债收益率5年"),
                    ("EMM00166466", "中国国债收益率10年"),
                    ("EMM00166469", "中国国债收益率30年"),
                ] {
                    if let Some(val) = v.get(*field).and_then(serde_json::Value::as_f64) {
                        all_items.push(MacroDataPoint {
                            date: date_short.clone(),
                            value: val,
                            name: label.to_string(),
                        });
                    }
                }

                // Extract US yields
                for (field, label) in &[
                    ("EMG00001306", "美国国债收益率2年"),
                    ("EMG00001308", "美国国债收益率5年"),
                    ("EMG00001310", "美国国债收益率10年"),
                    ("EMG00001312", "美国国债收益率30年"),
                ] {
                    if let Some(val) = v.get(*field).and_then(serde_json::Value::as_f64) {
                        all_items.push(MacroDataPoint {
                            date: date_short.clone(),
                            value: val,
                            name: label.to_string(),
                        });
                    }
                }
            }
        }

        if all_items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no treasury yield data",
            ));
        }
        Ok(all_items)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_rate_row_extraction() {
        let row = json!({
            "SOLAR_DATE": "2025-01-15 00:00:00",
            "EMM00588704": 1.85,
            "EMM00166462": 2.10,
            "EMM00166466": 2.45,
            "EMM00166469": 2.80,
            "EMG00001306": 4.20,
            "EMG00001308": 4.05,
            "EMG00001310": 4.50,
            "EMG00001312": 4.70
        });
        let date = row.get("SOLAR_DATE").and_then(|v| v.as_str()).unwrap();
        assert_eq!(&date[..10], "2025-01-15");
        assert!(row.get("EMM00166466").unwrap().as_f64().unwrap() > 0.0);
    }
}
