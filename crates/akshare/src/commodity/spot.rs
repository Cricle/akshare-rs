//! Commodity spot prices from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;
use crate::types::wire::EmDatacenterResp;

impl AkShareClient {
    /// Fetch commodity spot prices from Eastmoney datacenter.
    ///
    /// Returns the most recent `limit` data points of major domestic commodity
    /// spot prices (gold, silver, copper, etc.) reported by the Shanghai Gold
    /// Exchange and other spot markets.
    pub async fn commodity_spot_prices(&self, limit: usize) -> Result<Vec<MacroDataPoint>> {
        if limit == 0 {
            return Ok(Vec::new());
        }

        let page_size = limit.min(500).to_string();
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";

        // Fetch spot commodity quotes from the Eastmoney datacenter.
        // RPT_SPOT_COMMODITY covers major domestic spot commodity prices.
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_SPOT_COMMODITY"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", &page_size),
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
        let mut items = Vec::with_capacity(data.len().min(limit));

        for v in &data {
            if items.len() >= limit {
                break;
            }
            // Try common date field names used across Eastmoney reports.
            let date = v.str_or(&["REPORT_DATE", "TRADE_DATE", "DATE"], "");
            if date.is_empty() {
                continue;
            }

            // Price may be stored under different column names depending on the
            // commodity report structure.
            let value = v.f64_or(
                &[
                    "CLOSE_PRICE",
                    "LATEST_PRICE",
                    "PRICE",
                    "INDICATOR_VALUE",
                    "VALUE",
                ],
                0.0,
            );

            // Use the commodity name if available, otherwise a generic label.
            let name = v.str_or(
                &["COMMODITY_NAME", "PRODUCT_NAME", "NAME", "INDICATOR_NAME"],
                "Commodity Spot",
            );

            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(date.as_str()).to_string(),
                value,
                name,
            });
        }

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_spot_response() {
        // Simulate Eastmoney datacenter response.
        let json_str = r#"{
            "result": {
                "data": [
                    {
                        "REPORT_DATE": "2025-06-01 00:00:00",
                        "CLOSE_PRICE": 580.50,
                        "COMMODITY_NAME": "Au99.99"
                    },
                    {
                        "REPORT_DATE": "2025-06-01 00:00:00",
                        "CLOSE_PRICE": 7650.0,
                        "COMMODITY_NAME": "Ag(T+D)"
                    }
                ]
            }
        }"#;
        let resp: EmDatacenterResp = serde_json::from_str(json_str).unwrap();
        let data = resp.result.unwrap().data;
        assert_eq!(data.len(), 2);

        let first = &data[0];
        let date = first.str_field(&["REPORT_DATE"]).unwrap();
        assert_eq!(&date[..10], "2025-06-01");
        assert_eq!(first.f64_field(&["CLOSE_PRICE"]), Some(580.50));
    }

    #[test]
    fn test_parse_empty_response() {
        let json_str = r#"{"result": null}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json_str).unwrap();
        let data = resp.result.map(|r| r.data).unwrap_or_default();
        assert!(data.is_empty());
    }

    #[test]
    fn test_parse_fallback_fields() {
        // Ensure we fall back to alternative field names.
        let json_str = r#"{
            "result": {
                "data": [
                    {
                        "TRADE_DATE": "2025-05-30",
                        "LATEST_PRICE": 1024.0,
                        "NAME": "Copper"
                    }
                ]
            }
        }"#;
        let resp: EmDatacenterResp = serde_json::from_str(json_str).unwrap();
        let data = resp.result.unwrap().data;
        let v = &data[0];

        let date = v.str_or(&["REPORT_DATE", "TRADE_DATE"], "");
        assert_eq!(&date[..10], "2025-05-30");

        let value = v.f64_or(&["CLOSE_PRICE", "LATEST_PRICE"], 0.0);
        assert_eq!(value, 1024.0);

        let name = v.str_or(&["COMMODITY_NAME", "NAME"], "Commodity Spot");
        assert_eq!(name, "Copper");
    }
}
