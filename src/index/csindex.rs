//! CSIndex (中证指数) data — historical performance.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::CsindexHistPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct CsindexEnvelope {
    data: Option<Vec<Vec<serde_json::Value>>>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// List all CSIndex indices.
    ///
    /// Returns the list of all available CSIndex indices with their codes and names.
    pub async fn index_csindex_all(&self) -> Result<Vec<crate::types::Row>> {
        let url = "https://www.csindex.com.cn/csindex-home/index/list";
        let body = self
            .get(url)
            .query(&[("page", "1"), ("pageSize", "5000")])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let rows = resp["data"]["list"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let mut r = crate::types::Row::new();
            for (k, v) in row.as_object().unwrap_or(&serde_json::Map::new()) {
                r.insert(k.clone(), v.clone());
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        Ok(items)
    }

    /// 中证指数 — 具体指数历史行情数据.
    pub async fn stock_zh_index_hist_csindex(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<CsindexHistPoint>> {
        let response = self
            .get("https://www.csindex.com.cn/csindex-home/perf/index-perf")
            .query(&[
                ("indexCode", symbol),
                ("startDate", start_date),
                ("endDate", end_date),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: CsindexEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload.data.unwrap_or_default();

        let points: Vec<CsindexHistPoint> = data
            .into_iter()
            .filter_map(|row| {
                if row.len() < 16 {
                    return None;
                }
                Some(CsindexHistPoint {
                    date: row[0].as_str()?.to_string(),
                    open: row[6].as_f64().unwrap_or(0.0),
                    high: row[7].as_f64().unwrap_or(0.0),
                    low: row[8].as_f64().unwrap_or(0.0),
                    close: row[9].as_f64().unwrap_or(0.0),
                    change: row[10].as_f64().unwrap_or(0.0),
                    change_pct: row[11].as_f64().unwrap_or(0.0),
                    volume: row[12].as_f64().unwrap_or(0.0),
                    amount: row[13].as_f64().unwrap_or(0.0),
                    sample_count: row[14].as_f64().unwrap_or(0.0),
                    pe_ttm: row[15].as_f64().unwrap_or(0.0),
                })
            })
            .collect();

        if points.is_empty() {
            return Err(Error::not_found("csindex returned no history data"));
        }
        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // CSIndex functions require network access.
    }
}
