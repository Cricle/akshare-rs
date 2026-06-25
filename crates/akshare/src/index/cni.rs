//! CNIndex (国证指数) data — index listing, history, and constituents.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CniHistPoint, CniIndexItem};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct CniListEnvelope {
    data: Option<CniListData>,
}

#[derive(Debug, Deserialize)]
struct CniListData {
    rows: Option<Vec<Vec<serde_json::Value>>>,
}

#[derive(Debug, Deserialize)]
struct CniHistEnvelope {
    data: Option<CniHistData>,
}

#[derive(Debug, Deserialize)]
struct CniHistData {
    data: Option<Vec<Vec<serde_json::Value>>>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 国证指数 — 最近交易日的所有指数.
    pub async fn index_all_cni(&self) -> Result<Vec<CniIndexItem>> {
        let response = crate::util::send_and_check(
            self.get("https://www.cnindex.com.cn/index/indexList")
                .query(&[("channelCode", "-1"), ("rows", "2000"), ("pageNum", "1")])
        )
        .await?;

        let payload: CniListEnvelope = response.json().await.map_err(Error::from)?;
        let rows = payload.data.and_then(|d| d.rows).unwrap_or_default();

        let items: Vec<CniIndexItem> = rows
            .into_iter()
            .filter_map(|row| {
                if row.len() < 25 {
                    return None;
                }
                let code = row[2].as_str()?.to_string();
                let name = row[8].as_str().unwrap_or("").to_string();
                let sample_count = row[12].as_f64().unwrap_or(0.0);
                let close = row[13].as_f64().unwrap_or(0.0);
                let change_pct = row[14].as_f64().unwrap_or(0.0);
                let pe_ttm = row[16].as_f64().unwrap_or(0.0);
                let volume = row[18].as_f64().unwrap_or(0.0) / 100_000.0;
                let amount = row[19].as_f64().unwrap_or(0.0) / 100_000_000.0;
                let total_mv = row[20].as_f64().unwrap_or(0.0) / 100_000_000.0;
                let free_circ_mv = row[21].as_f64().unwrap_or(0.0) / 100_000_000.0;
                Some(CniIndexItem {
                    code,
                    name,
                    sample_count,
                    close,
                    change_pct,
                    pe_ttm,
                    volume,
                    amount,
                    total_mv,
                    free_circ_mv,
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("cnindex returned no index items"));
        }
        Ok(items)
    }

    /// 国证指数 — 历史行情数据.
    pub async fn index_hist_cni(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<CniHistPoint>> {
        // Format: YYYYMMDD -> YYYY-MM-DD
        let start = format_date_hyphen(start_date);
        let end = format_date_hyphen(end_date);

        let response = crate::util::send_and_check(
            self.get("http://hq.cnindex.com.cn/market/market/getIndexDailyDataWithDataFormat")
                .query(&[
                ("indexCode", symbol),
                ("startDate", &start),
                ("endDate", &end),
                ("frequency", "day"),
                ])
        )
        .await?;

        let payload: CniHistEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload.data.and_then(|d| d.data).unwrap_or_default();

        let mut points: Vec<CniHistPoint> = data
            .into_iter()
            .filter_map(|row| {
                if row.len() < 11 {
                    return None;
                }
                let date = row[0].as_str()?.to_string();
                let high = row[2].as_f64().unwrap_or(0.0);
                let open = row[3].as_f64().unwrap_or(0.0);
                let low = row[4].as_f64().unwrap_or(0.0);
                let close = row[5].as_f64().unwrap_or(0.0);
                let change_pct_raw = row[7].as_str().unwrap_or("0");
                let change_pct = change_pct_raw
                    .trim_end_matches('%')
                    .parse::<f64>()
                    .unwrap_or(0.0)
                    / 100.0;
                let amount = row[8].as_f64().unwrap_or(0.0);
                let volume = row[9].as_f64().unwrap_or(0.0);
                Some(CniHistPoint {
                    date,
                    open,
                    high,
                    low,
                    close,
                    change_pct,
                    volume,
                    amount,
                })
            })
            .collect();

        points.sort_by(|a, b| a.date.cmp(&b.date));
        if points.is_empty() {
            return Err(Error::not_found("cnindex returned no history data"));
        }
        Ok(points)
    }

    /// 国证指数 — 样本详情 (constituents).
    ///
    /// Returns a JSON array of objects with fields: date, code, name, industry,
    /// total_mv, weight.  The upstream returns an XLS; this endpoint returns
    /// JSON when called with the right accept header.
    pub async fn index_detail_cni(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let response = crate::util::send_and_check(
            self.get("https://www.cnindex.com.cn/sample-detail/download-history")
                .query(&[("indexcode", symbol)])
        )
        .await?;

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        if content_type.contains("json") {
            let data: Vec<serde_json::Value> = response.json().await.map_err(Error::from)?;
            Ok(data)
        } else {
            // The endpoint returns XLS; we cannot parse it in pure Rust without
            // a spreadsheet library. Return an error.
            Err(Error::upstream(
                "cnindex returns XLS format — not supported in pure Rust; \
                 use the JSON API or a third-party XLS parser",
            ))
        }
    }

    /// 国证指数 — 详细历史行情数据 (with date range).
    ///
    /// Same as `index_hist_cni` but with the Python-compatible name.
    pub async fn index_detail_hist_cni(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<CniHistPoint>> {
        self.index_hist_cni(symbol, start_date, end_date).await
    }

    /// 国证指数 — 历史调样.
    ///
    /// Same as `index_detail_cni` but for adjustment records.
    pub async fn index_detail_hist_adjust_cni(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let response = crate::util::send_and_check(
            self.get("http://www.cnindex.com.cn/sample-detail/download-adjustment")
                .query(&[("indexcode", symbol)])
        )
        .await?;

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        if content_type.contains("json") {
            let data: Vec<serde_json::Value> = response.json().await.map_err(Error::from)?;
            Ok(data)
        } else {
            Err(Error::upstream(
                "cnindex adjustment returns XLS format — not supported in pure Rust",
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn format_date_hyphen(d: &str) -> String {
    let d = d.trim();
    if d.len() == 8 && d.chars().all(|c| c.is_ascii_digit()) {
        format!("{}-{}-{}", &d[..4], &d[4..6], &d[6..8])
    } else {
        d.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date_hyphen() {
        assert_eq!(format_date_hyphen("20230114"), "2023-01-14");
        assert_eq!(format_date_hyphen("2023-01-14"), "2023-01-14");
    }
}
