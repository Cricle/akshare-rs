//! CFLP (中国物流与采购联合会) logistics price and volume indices.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::CflpIndexPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct CflpEnvelope {
    chart1: Option<CflpChart>,
    chart2: Option<CflpChart>,
    chart3: Option<CflpChart>,
}

#[derive(Debug, Deserialize)]
struct CflpChart {
    #[serde(default, rename = "xLebal")]
    x_lebal: Vec<String>,
    #[serde(default, rename = "yLebal")]
    y_lebal: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 中国公路物流运价指数.
    ///
    /// `symbol` is one of: "周指数", "月指数", "季度指数", "年度指数".
    pub async fn index_price_cflp(&self, symbol: &str) -> Result<Vec<CflpIndexPoint>> {
        let exp_type = match symbol {
            "周指数" => "2",
            "月指数" => "3",
            "季度指数" => "4",
            "年度指数" => "5",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported CFLP symbol: {symbol}"
                )));
            }
        };

        let response = self
            .post("http://index.0256.cn/expcenter_trend.action")
            .header("Origin", "http://index.0256.cn")
            .header("Referer", "http://index.0256.cn/expx.htm")
            .form(&[
                ("marketId", "1"),
                ("attribute1", "5"),
                ("exponentTypeId", exp_type),
                ("cateId", "2"),
                ("attribute2", "华北"),
                ("city", ""),
                ("startLine", ""),
                ("endLine", ""),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        parse_cflp_response(response).await
    }

    /// 中国公路物流运量指数.
    ///
    /// `symbol` is one of: "月指数", "季度指数", "年度指数".
    pub async fn index_volume_cflp(&self, symbol: &str) -> Result<Vec<CflpIndexPoint>> {
        let exp_type = match symbol {
            "月指数" => "3",
            "季度指数" => "4",
            "年度指数" => "5",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported CFLP symbol: {symbol}"
                )));
            }
        };

        let response = self
            .post("http://index.0256.cn/volume_query.action")
            .header("Origin", "http://index.0256.cn")
            .header("Referer", "http://index.0256.cn/expx.htm")
            .form(&[
                ("type", "1"),
                ("marketId", "1"),
                ("expTypeId", exp_type),
                ("startDate1", ""),
                ("endDate1", ""),
                ("city", ""),
                ("startDate3", ""),
                ("endDate3", ""),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        parse_cflp_response(response).await
    }
}

async fn parse_cflp_response(response: reqwest::Response) -> Result<Vec<CflpIndexPoint>> {
    let payload: CflpEnvelope = response.json().await.map_err(Error::from)?;

    let chart1 = payload
        .chart1
        .ok_or_else(|| Error::upstream("cflp response missing chart1"))?;
    let chart2 = payload.chart2.unwrap_or(CflpChart {
        x_lebal: vec![],
        y_lebal: vec![],
    });
    let chart3 = payload.chart3.unwrap_or(CflpChart {
        x_lebal: vec![],
        y_lebal: vec![],
    });

    let len = chart1.x_lebal.len();
    let mut points = Vec::with_capacity(len);

    for i in 0..len {
        let date = chart1.x_lebal.get(i).cloned().unwrap_or_default();
        let base = chart1
            .y_lebal
            .get(i)
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.0);
        let mom = chart2
            .y_lebal
            .get(i)
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.0);
        let yoy = chart3
            .y_lebal
            .get(i)
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.0);
        points.push(CflpIndexPoint {
            date,
            base_index: base,
            mom_index: mom,
            yoy_index: yoy,
        });
    }

    if points.is_empty() {
        return Err(Error::not_found("cflp returned no data"));
    }
    Ok(points)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // CFLP functions require network access.
    }
}
