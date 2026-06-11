//! Repo rate data from China Money (中国外汇交易中心).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct FrrResponse {
    records: Option<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// China Money - repo fixing rate query.
    ///
    /// Returns the latest repo fixing rates (FR001, FR007, FR014) from
    /// China Foreign Exchange Trade System.
    ///
    /// `symbol`: "回购定盘利率" or "银银间回购定盘利率"
    pub async fn repo_rate_query(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let (url, col_names) = if symbol == "银银间回购定盘利率" {
            (
                "https://www.chinamoney.com.cn/r/cms/www/chinamoney/data/currency/fdr-chrt.csv",
                vec!["FDR001", "FDR007", "FDR014"],
            )
        } else {
            (
                "https://www.chinamoney.com.cn/r/cms/www/chinamoney/data/currency/frr-chrt.csv",
                vec!["FR001", "FR007", "FR014"],
            )
        };

        let body = self.get(url).send().await?.text().await?;
        let mut items = Vec::new();

        for line in body.lines() {
            let fields: Vec<&str> = line.split(',').filter(|s| !s.trim().is_empty()).collect();
            if fields.len() < 4 {
                continue;
            }
            let date = fields[0].trim().to_string();
            if date.is_empty() || !date.contains('-') {
                continue;
            }

            for (i, col_name) in col_names.iter().enumerate() {
                if let Some(val) = fields.get(i + 1).and_then(|s| s.trim().parse::<f64>().ok()) {
                    items.push(MacroDataPoint {
                        date: date.clone(),
                        value: val,
                        name: col_name.to_string(),
                    });
                }
            }
        }
        Ok(items)
    }

    /// China Money - historical repo fixing rates.
    ///
    /// Returns historical FR001/FR007/FR014/FDR001/FDR007/FDR014 rates
    /// for the given date range.
    ///
    /// `start_date` and `end_date` in "YYYYMMDD" format. Must be within
    /// one month of each other.
    pub async fn repo_rate_hist(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let formatted_start = format!(
            "{}-{}-{}",
            &start_date[..4],
            &start_date[4..6],
            &start_date[6..8]
        );
        let formatted_end = format!(
            "{}-{}-{}",
            &end_date[..4],
            &end_date[4..6],
            &end_date[6..8]
        );

        let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bk-currency/FrrHis";
        let resp: FrrResponse = self
                        .post(url)
            .query(&[
                ("lang", "CN"),
                ("startDate", &formatted_start),
                ("endDate", &formatted_end),
            ])
            .header(
                "User-Agent",
                "Mozilla/5.0 (compatible; akshare-rust/0.1)",
            )
            .send()
            .await?
            .json()
            .await?;

        let records = resp.records.unwrap_or_default();
        let mut items = Vec::new();

        for record in &records {
            if let Some(fr_map) = record.get("frValueMap") {
                let date = fr_map
                    .get("date")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                for key in &["FR001", "FR007", "FR014", "FDR001", "FDR007", "FDR014"] {
                    if let Some(val) = fr_map.get(*key).and_then(|v| v.as_f64()) {
                        items.push(MacroDataPoint {
                            date: date.clone(),
                            value: val,
                            name: key.to_string(),
                        });
                    }
                }
            }
        }

        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_frr_response_structure() {
        let json = r#"{
            "records": [
                {
                    "frValueMap": {
                        "date": "2024-01-02",
                        "FR001": 1.5,
                        "FR007": 1.8,
                        "FR014": 2.0,
                        "FDR001": 1.6,
                        "FDR007": 1.9,
                        "FDR014": 2.1
                    }
                }
            ]
        }"#;
        let resp: super::FrrResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.records.as_ref().unwrap().len(), 1);
    }
}
