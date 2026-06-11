//! Caixin (财新) innovation indices — 19 time-series functions.
//!
//! All endpoints hit the same `cxIndexTrendInfo` API with a different `type`
//! parameter. We implement a shared helper and expose one `pub async fn` per
//! Python function.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::CxIndexPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct CxEnvelope {
    data: Option<Vec<CxRow>>,
}

#[derive(Debug, Deserialize)]
struct CxRow {
    #[serde(default)]
    v: Option<f64>, // value
    #[serde(default)]
    s: Option<f64>, // change
    #[serde(default)]
    t: Option<i64>, // timestamp ms
    // Some endpoints use different field names
    #[serde(default)]
    value: Option<f64>,
    #[serde(default)]
    change: Option<f64>,
    #[serde(default)]
    date: Option<i64>,
}

// ---------------------------------------------------------------------------
// Shared helper
// ---------------------------------------------------------------------------

async fn fetch_cx_index(
    client: &AkShareClient,
    cx_type: &str,
    extra: &[(&str, &str)],
) -> Result<Vec<CxIndexPoint>> {
    let mut params = vec![("type", cx_type)];
    params.extend(extra.iter().map(|(k, v)| (*k, *v)));

    let response = client
        .get("https://yun.ccxe.com.cn/api/index/pro/cxIndexTrendInfo")
        .query(&params)
        .send()
        .await
        .map_err(Error::from)?
        .error_for_status()
        .map_err(Error::from)?;

    let payload: CxEnvelope = response.json().await.map_err(Error::from)?;
    let rows = payload.data.unwrap_or_default();

    let points: Vec<CxIndexPoint> = rows
        .into_iter()
        .filter_map(|r| {
            let ts = r.t.or(r.date)?;
            let val = r.v.or(r.value)?;
            let chg = r.s.or(r.change).unwrap_or(0.0);
            // Convert ms timestamp to Asia/Shanghai date
            let dt = chrono::DateTime::from_timestamp_millis(ts)?;
            let date = dt
                .with_timezone(&chrono::FixedOffset::east_opt(8 * 3600)?)
                .format("%Y-%m-%d")
                .to_string();
            Some(CxIndexPoint {
                date,
                value: val,
                change: chg,
            })
        })
        .collect();

    if points.is_empty() {
        return Err(Error::not_found("cx returned no data points"));
    }
    Ok(points)
}

// ---------------------------------------------------------------------------
// Public API (19 functions)
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 财新中国 PMI — 综合 PMI.
    pub async fn index_pmi_com_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "com", &[]).await
    }

    /// 财新中国 PMI — 制造业 PMI.
    pub async fn index_pmi_man_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "man", &[]).await
    }

    /// 财新中国 PMI — 服务业 PMI.
    pub async fn index_pmi_ser_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "ser", &[]).await
    }

    /// 数字经济指数.
    pub async fn index_dei_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "dei", &[]).await
    }

    /// 产业指数.
    pub async fn index_ii_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "ii", &[]).await
    }

    /// 溢出指数.
    pub async fn index_si_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "si", &[]).await
    }

    /// 融合指数.
    pub async fn index_fi_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "fi", &[]).await
    }

    /// 基础指数.
    pub async fn index_bi_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "bi", &[]).await
    }

    /// 中国新经济指数.
    pub async fn index_nei_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "nei", &[]).await
    }

    /// 劳动力投入指数.
    pub async fn index_li_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "li", &[]).await
    }

    /// 资本投入指数.
    pub async fn index_ci_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "ci", &[]).await
    }

    /// 科技投入指数.
    pub async fn index_ti_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "ti", &[]).await
    }

    /// 新经济行业入职平均工资水平.
    pub async fn index_neaw_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "neaw", &[]).await
    }

    /// 新经济入职工资溢价水平.
    pub async fn index_awpr_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "awpr", &[]).await
    }

    /// 大宗商品指数.
    pub async fn index_cci_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "cci", &[("code", "1000050"), ("month", "-1")]).await
    }

    /// 高质量因子指数.
    pub async fn index_qli_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "qli", &[("code", "1000050"), ("month", "-1")]).await
    }

    /// AI 策略指数.
    pub async fn index_ai_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "ai", &[("code", "1000050"), ("month", "-1")]).await
    }

    /// 基石经济指数.
    pub async fn index_bei_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "ind", &[("code", "930927"), ("month", "-1")]).await
    }

    /// 新动能指数.
    pub async fn index_neei_cx(&self) -> Result<Vec<CxIndexPoint>> {
        fetch_cx_index(self, "ind", &[("code", "930928"), ("month", "1")]).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // CX indices require network access; verify compilation only.
    }
}
