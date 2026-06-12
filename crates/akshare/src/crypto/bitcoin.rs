//! Bitcoin CME volume and hold report data from Jin10.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct Jin10ReportResponse {
    data: Option<Jin10ReportData>,
}

#[derive(Debug, Deserialize)]
struct Jin10ReportData {
    keys: Option<Vec<Jin10Key>>,
    values: Option<Vec<Vec<serde_json::Value>>>,
}

#[derive(Debug, Deserialize)]
struct Jin10Key {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Jin10HoldResponse {
    data: Option<Jin10HoldData>,
}

#[derive(Debug, Deserialize)]
struct Jin10HoldData {
    values: Option<Vec<Vec<serde_json::Value>>>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// CME Bitcoin volume report from Jin10.
    ///
    /// Fetches the CME Bitcoin futures volume report for a specific date.
    /// `date` is in "YYYYMMDD" format.
    pub async fn crypto_bitcoin_cme(&self, date: &str) -> Result<Vec<MacroDataPoint>> {
        let formatted_date = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);

        let url = "https://datacenter-api.jin10.com/reports/list";
        let resp: Jin10ReportResponse = self
            .get(url)
            .query(&[
                ("category", "cme"),
                ("date", &formatted_date),
                ("attr_id", "4"),
            ])
            .header("X-App-Id", "rU6QIu7JHe2gOUeR")
            .header("X-Version", "1.0.0")
            .send()
            .await?
            .json()
            .await?;

        let data = resp
            .data
            .ok_or_else(|| Error::decode("bitcoin cme: no data"))?;

        let keys: Vec<String> = data
            .keys
            .unwrap_or_default()
            .into_iter()
            .map(|k| k.name.unwrap_or_default())
            .collect();

        let values = data.values.unwrap_or_default();
        let mut items = Vec::new();

        for row in &values {
            // Each row corresponds to a column in the report
            // Typically: 电子交易合约, 场内成交合约, 场外成交合约, 成交量, 未平仓合约, 持仓变化
            for (i, val) in row.iter().enumerate() {
                let key = keys.get(i).cloned().unwrap_or_default();
                let value = val.as_f64().unwrap_or(0.0);
                items.push(MacroDataPoint {
                    date: formatted_date.clone(),
                    value,
                    name: format!("CME BTC {key}"),
                });
            }
        }
        Ok(items)
    }

    /// Bitcoin treasury holdings report from Jin10.
    ///
    /// Returns the list of companies/organizations holding Bitcoin.
    pub async fn crypto_bitcoin_hold_report(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-api.jin10.com/bitcoin_treasuries/list";
        let resp: Jin10HoldResponse = self
            .get(url)
            .header("X-App-Id", "lnFP5lxse24wPgtY")
            .header("X-Version", "1.0.0")
            .send()
            .await?
            .json()
            .await?;

        let values = resp.data.and_then(|d| d.values).unwrap_or_default();

        let mut items = Vec::new();
        for row in &values {
            if row.len() < 16 {
                continue;
            }
            // Column layout based on Python source:
            // [0] code, [1] company_en, [2] country, [3] market_cap,
            // [4] btc_ratio, [5] cost, [6] holding_ratio, [7] holding_amount,
            // [8] holding_value, [9] query_date, [10] announcement_link,
            // [11] _, [12] category, [13] multiplier, [14] _, [15] company_cn
            let company = row[1].as_str().unwrap_or("").to_string();
            let holding_amount = row[7].as_f64().unwrap_or(0.0);
            let query_date = row[9].as_str().unwrap_or("").to_string();

            if !company.is_empty() {
                items.push(MacroDataPoint {
                    date: query_date.get(..10).unwrap_or(&query_date).to_string(),
                    value: holding_amount,
                    name: company,
                });
            }
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_jin10_report_response_structure() {
        let json = r#"{
            "data": {
                "keys": [
                    {"name": "成交量"},
                    {"name": "未平仓合约"}
                ],
                "values": [
                    [1000.0, 5000.0]
                ]
            }
        }"#;
        let resp: super::Jin10ReportResponse = serde_json::from_str(json).unwrap();
        let data = resp.data.unwrap();
        assert_eq!(data.keys.unwrap().len(), 2);
        assert_eq!(data.values.unwrap().len(), 1);
    }

    #[test]
    fn test_jin10_hold_response_structure() {
        let json = r#"{
            "data": {
                "values": [
                    ["MSTR", "MicroStrategy", "US", 50000, 0.5, 30000, 0.02, 150000, 4500000, "2024-01-01", "http://example.com", "", "上市公司", 1.0, "", "微策略"]
                ]
            }
        }"#;
        let resp: super::Jin10HoldResponse = serde_json::from_str(json).unwrap();
        let values = resp.data.unwrap().values.unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0][1].as_str().unwrap(), "MicroStrategy");
    }
}
