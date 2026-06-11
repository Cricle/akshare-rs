//! 行情宝 hog (生猪) price index.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::HogIndexPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct HogEnvelope {
    data: Option<Vec<HogRow>>,
}

#[derive(Debug, Deserialize)]
struct HogRow {
    #[serde(default)]
    date: Option<i64>,
    #[serde(default)]
    index: Option<f64>,
    #[serde(default)]
    ma4: Option<f64>,
    #[serde(default)]
    ma6: Option<f64>,
    #[serde(default)]
    ma12: Option<f64>,
    #[serde(default)]
    presale_avg: Option<f64>,
    #[serde(default)]
    deal_avg: Option<f64>,
    #[serde(default)]
    deal_avg_weight: Option<f64>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 行情宝 — 生猪市场价格指数.
    pub async fn index_hog_spot_price(&self) -> Result<Vec<HogIndexPoint>> {
        let response = self
            .get("https://hqb.nxin.com/pigindex/getPigIndexChart.shtml")
            .query(&[("regionId", "0")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: HogEnvelope = response.json().await.map_err(Error::from)?;
        let rows = payload.data.unwrap_or_default();

        let points: Vec<HogIndexPoint> = rows
            .into_iter()
            .filter_map(|r| {
                let ts = r.date?;
                // ms -> Asia/Shanghai
                let dt = chrono::DateTime::from_timestamp_millis(ts)?;
                let dt_cst = dt.with_timezone(&chrono::FixedOffset::east_opt(8 * 3600)?);
                let date = dt_cst.format("%Y-%m-%d").to_string();
                Some(HogIndexPoint {
                    date,
                    index: r.index.unwrap_or(0.0),
                    ma4: r.ma4.unwrap_or(0.0),
                    ma6: r.ma6.unwrap_or(0.0),
                    ma12: r.ma12.unwrap_or(0.0),
                    presale_avg: r.presale_avg.unwrap_or(0.0),
                    deal_avg: r.deal_avg.unwrap_or(0.0),
                    deal_avg_weight: r.deal_avg_weight.unwrap_or(0.0),
                })
            })
            .collect();

        if points.is_empty() {
            return Err(Error::not_found("nxin returned no hog index data"));
        }
        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Hog functions require network access.
    }
}
