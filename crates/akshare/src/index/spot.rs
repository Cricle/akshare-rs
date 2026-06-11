//! 新浪财经商品现货价格指数.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::SpotGoodsPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct SpotEnvelope {
    result: Option<SpotResult>,
}

#[derive(Debug, Deserialize)]
struct SpotResult {
    data: Option<SpotData>,
}

#[derive(Debug, Deserialize)]
struct SpotData {
    data: Option<Vec<SpotRow>>,
}

#[derive(Debug, Deserialize)]
struct SpotRow {
    #[serde(default)]
    opendate: String,
    #[serde(default)]
    price: Option<f64>,
    #[serde(default)]
    zde: Option<f64>,
    #[serde(default)]
    zdf: Option<f64>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 新浪财经 — 商品现货价格指数.
    ///
    /// `symbol` is one of: "波罗的海干散货指数", "钢坯价格指数", "澳大利亚粉矿价格".
    pub async fn spot_goods(&self, symbol: &str) -> Result<Vec<SpotGoodsPoint>> {
        let code = match symbol {
            "波罗的海干散货指数" => "BDI",
            "钢坯价格指数" => "GP",
            "澳大利亚粉矿价格" => "PB",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported spot symbol: {symbol}"
                )));
            }
        };

        let response = self
                        .get("https://stock.finance.sina.com.cn/futures/api/openapi.php/GoodsIndexService.get_goods_index")
            .query(&[("symbol", code), ("table", "0")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: SpotEnvelope = response.json().await.map_err(Error::from)?;
        let rows = payload
            .result
            .and_then(|r| r.data)
            .and_then(|d| d.data)
            .unwrap_or_default();

        let points: Vec<SpotGoodsPoint> = rows
            .into_iter()
            .filter_map(|r| {
                if r.opendate.is_empty() {
                    return None;
                }
                Some(SpotGoodsPoint {
                    date: r.opendate,
                    index: r.price.unwrap_or(0.0),
                    change_amount: r.zde.unwrap_or(0.0),
                    change_pct: r.zdf.unwrap_or(0.0),
                })
            })
            .collect();

        if points.is_empty() {
            return Err(Error::not_found("sina returned no spot goods data"));
        }
        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Spot goods functions require network access.
    }
}
