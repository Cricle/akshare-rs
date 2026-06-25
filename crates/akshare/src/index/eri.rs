//! 浙江省排污权交易指数 (ERI).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::EriIndexPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct EriEnvelope {
    data: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct EriStatsEnvelope {
    data: Option<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 浙江省排污权交易指数.
    ///
    /// `symbol` is "月度" or "季度".
    pub async fn index_eri(&self, symbol: &str) -> Result<Vec<EriIndexPoint>> {
        let cycle = match symbol {
            "月度" => "MONTH",
            "季度" => "QUARTER",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported ERI symbol: {symbol}"
                )));
            }
        };

        let params = [
            ("cycle", cycle),
            ("regionId", "1"),
            ("structId", "1"),
            ("pageSize", "5000"),
            ("indexId", "1"),
            ("orderBy", "stage.publishTime"),
        ];

        // Fetch index data
        let resp1 = crate::util::send_and_check(
            self.get("https://zs.zjpwq.net/pwq-index-webapi/indexData")
                .query(&params)
        )
        .await?;

        let payload1: EriEnvelope = resp1.json().await.map_err(Error::from)?;
        let data1 = payload1.data.unwrap_or_default();

        let mut dates = Vec::new();
        let mut values = Vec::new();
        for item in &data1 {
            let Some(obj) = item.as_object() else {
                continue;
            };
            let val = obj
                .get("indexValue")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0);
            let ts = obj
                .get("stage")
                .and_then(|s| s.get("publishTime"))
                .and_then(serde_json::Value::as_i64)
                .unwrap_or(0);
            let date = chrono::DateTime::from_timestamp_millis(ts)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();
            dates.push(date);
            values.push(val);
        }

        // Fetch statistics data
        let resp2 = crate::util::send_and_check(
            self.get("https://zs.zjpwq.net/pwq-index-webapi/dataStatistics")
                .query(&params)
        )
        .await?;

        let payload2: EriStatsEnvelope = resp2.json().await.map_err(Error::from)?;
        let data2 = payload2.data.unwrap_or_default();

        let mut volumes = Vec::new();
        let mut amounts = Vec::new();
        for item in &data2 {
            let Some(obj) = item.as_object() else {
                continue;
            };
            volumes.push(
                obj.get("totalQuantity")
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0),
            );
            amounts.push(
                obj.get("totalCost")
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0),
            );
        }

        let len = dates.len().min(volumes.len());
        let mut points = Vec::with_capacity(len);
        for i in 0..len {
            points.push(EriIndexPoint {
                date: dates[i].clone(),
                trade_index: values[i],
                volume: volumes[i],
                amount: amounts[i],
            });
        }

        if points.is_empty() {
            return Err(Error::not_found("zjpwq returned no ERI data"));
        }
        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // ERI functions require network access.
    }
}
