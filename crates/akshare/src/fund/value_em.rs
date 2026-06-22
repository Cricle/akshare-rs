//! Fund value estimation from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundValueEstimationItem;

impl AkShareClient {
    /// Fetch fund value estimation (Python: fund_value_estimation).
    ///
    /// `symbol`: "全部", "股票型", "混合型", "债券型", "指数型", "QDII", "ETF联接", "LOF", "场内交易基金".
    pub async fn fund_value_estimation(
        &self,
        symbol: &str,
    ) -> Result<Vec<FundValueEstimationItem>> {
        let symbol_map: &[(&str, &str)] = &[
            ("全部", "1"),
            ("股票型", "2"),
            ("混合型", "3"),
            ("债券型", "4"),
            ("指数型", "5"),
            ("QDII", "6"),
            ("ETF联接", "7"),
            ("LOF", "8"),
            ("场内交易基金", "9"),
        ];
        let type_id = symbol_map
            .iter()
            .find(|(n, _)| *n == symbol)
            .map_or("1", |(_, c)| *c);

        let ts = chrono::Utc::now().timestamp_millis().to_string();
        let response = self
            .get("https://api.fund.eastmoney.com/FundGuZhi/GetFundGZList")
            .header("Referer", "https://fund.eastmoney.com/")
            .query(&[
                ("type", type_id),
                ("sort", "3"),
                ("orderType", "desc"),
                ("canbuy", "0"),
                ("pageIndex", "1"),
                ("pageSize", "20000"),
                ("_", ts.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("Data")
            .and_then(|d| d.get("list"))
            .and_then(|l| l.as_array())
            .ok_or_else(|| Error::not_found("no fund estimation data"))?;

        let gzrq = payload
            .get("Data")
            .and_then(|d| d.get("gzrq"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut result = Vec::new();
        for (i, item) in data.iter().enumerate() {
            let Some(arr) = item.as_array() else { continue };
            if arr.len() < 28 {
                continue;
            }
            result.push(FundValueEstimationItem {
                rank: (i + 1) as i32,
                fund_code: arr[0].as_str().unwrap_or("").to_string(),
                fund_name: arr[26].as_str().unwrap_or("").to_string(),
                estimated_value: arr[20].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                estimated_change_pct: arr[21].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                published_nav: arr[24].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                published_change_pct: arr[22].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                deviation: arr[19].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                nav_date: gzrq.to_string(),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found("no fund estimation data"));
        }
        Ok(result)
    }
}
