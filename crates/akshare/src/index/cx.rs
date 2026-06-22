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
// Public API — macro-generated (19 functions)
// ---------------------------------------------------------------------------

macro_rules! cx_index {
    ($(#[$meta:meta])* $name:ident, $code:expr) => {
        $(#[$meta])*
        pub async fn $name(&self) -> Result<Vec<CxIndexPoint>> {
            fetch_cx_index(self, $code, &[]).await
        }
    };
    ($(#[$meta:meta])* $name:ident, $code:expr, $extra:expr) => {
        $(#[$meta])*
        pub async fn $name(&self) -> Result<Vec<CxIndexPoint>> {
            fetch_cx_index(self, $code, $extra).await
        }
    };
}

impl AkShareClient {
    cx_index!(/// 财新中国 PMI — 综合 PMI.
        index_pmi_com_cx, "com");
    cx_index!(/// 财新中国 PMI — 制造业 PMI.
        index_pmi_man_cx, "man");
    cx_index!(/// 财新中国 PMI — 服务业 PMI.
        index_pmi_ser_cx, "ser");
    cx_index!(/// 数字经济指数.
        index_dei_cx, "dei");
    cx_index!(/// 产业指数.
        index_ii_cx, "ii");
    cx_index!(/// 溢出指数.
        index_si_cx, "si");
    cx_index!(/// 融合指数.
        index_fi_cx, "fi");
    cx_index!(/// 基础指数.
        index_bi_cx, "bi");
    cx_index!(/// 中国新经济指数.
        index_nei_cx, "nei");
    cx_index!(/// 劳动力投入指数.
        index_li_cx, "li");
    cx_index!(/// 资本投入指数.
        index_ci_cx, "ci");
    cx_index!(/// 科技投入指数.
        index_ti_cx, "ti");
    cx_index!(/// 新经济行业入职平均工资水平.
        index_neaw_cx, "neaw");
    cx_index!(/// 新经济入职工资溢价水平.
        index_awpr_cx, "awpr");
    cx_index!(/// 大宗商品指数.
        index_cci_cx, "cci", &[("code", "1000050"), ("month", "-1")]);
    cx_index!(/// 高质量因子指数.
        index_qli_cx, "qli", &[("code", "1000050"), ("month", "-1")]);
    cx_index!(/// AI 策略指数.
        index_ai_cx, "ai", &[("code", "1000050"), ("month", "-1")]);
    cx_index!(/// 基石经济指数.
        index_bei_cx, "ind", &[("code", "930927"), ("month", "-1")]);
    cx_index!(/// 新动能指数.
        index_neei_cx, "ind", &[("code", "930928"), ("month", "1")]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // CX indices require network access; verify compilation only.
    }
}
