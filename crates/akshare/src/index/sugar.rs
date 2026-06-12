//! 沐甜科技 sugar indices — composite price, import inner/outer quotes.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 沐甜科技 — 中国食糖指数.
    pub async fn index_sugar_msweet(&self) -> Result<Vec<SugarMsweetPoint>> {
        #[derive(Deserialize)]
        struct Envelope {
            category: Option<Vec<String>>,
            data: Option<Vec<Vec<serde_json::Value>>>,
        }

        let response = self
            .get("https://www.msweet.com.cn/eportal/ui")
            .query(&[
                ("struts.portlet.action", "/portlet/price!getSTZSJson.action"),
                ("moduleId", "cb752447cfe24b44b18c7a7e9abab048"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Envelope = response.json().await.map_err(Error::from)?;
        let categories = payload.category.unwrap_or_default();
        let data = payload.data.unwrap_or_default();

        let mut points = Vec::new();
        for (i, date) in categories.iter().enumerate() {
            let row = data.get(i);
            let composite = row
                .and_then(|r| r.first())
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0);
            let raw = row
                .and_then(|r| r.get(1))
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0);
            let spot = row
                .and_then(|r| r.get(2))
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0);
            points.push(SugarMsweetPoint {
                date: date.clone(),
                composite_price: composite,
                raw_sugar_price: raw,
                spot_price: spot,
            });
        }

        if points.is_empty() {
            return Err(Error::not_found("msweet returned no sugar index data"));
        }
        Ok(points)
    }

    /// 沐甜科技 — 配额内进口糖估算指数.
    pub async fn index_inner_quote_sugar_msweet(&self) -> Result<Vec<SugarInnerQuotePoint>> {
        #[derive(Deserialize)]
        struct Envelope {
            category: Option<Vec<String>>,
            data: Option<Vec<Vec<serde_json::Value>>>,
        }

        let response = self
            .get("https://www.msweet.com.cn/datacenterapply/datacenter/json/JinKongTang.json")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Envelope = response.json().await.map_err(Error::from)?;
        let categories = payload.category.unwrap_or_default();
        let data = payload.data.unwrap_or_default();

        let mut points = Vec::new();
        for (i, date_raw) in categories.iter().enumerate() {
            let row = data.get(i);
            let date = date_raw.replace('/', "-");
            let get = |idx: usize| -> f64 {
                row.and_then(|r| r.get(idx))
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0)
            };
            points.push(SugarInnerQuotePoint {
                date,
                profit_space: get(0),
                thailand_sugar: get(1),
                thailand_ma5: get(2),
                brazil_ma5: get(3),
                profit_ma5: get(4),
                brazil_ma10: get(5),
                brazil_sugar: get(6),
                liuzhou_spot: get(7),
                guangzhou_spot: get(8),
                thailand_ma10: get(9),
                profit_ma30: get(10),
                profit_ma10: get(11),
            });
        }

        if points.is_empty() {
            return Err(Error::not_found(
                "msweet returned no inner quote sugar data",
            ));
        }
        Ok(points)
    }

    /// 沐甜科技 — 配额外进口糖估算指数.
    pub async fn index_outer_quote_sugar_msweet(&self) -> Result<Vec<SugarOuterQuotePoint>> {
        #[derive(Deserialize)]
        struct Envelope {
            category: Option<Vec<String>>,
            data: Option<Vec<Vec<serde_json::Value>>>,
        }

        let response = self
            .get("https://www.msweet.com.cn/datacenterapply/datacenter/json/Jkpewlr.json")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Envelope = response.json().await.map_err(Error::from)?;
        let categories = payload.category.unwrap_or_default();
        let data = payload.data.unwrap_or_default();

        let mut points = Vec::new();
        for (i, date_raw) in categories.iter().enumerate() {
            let row = data.get(i);
            let date = date_raw.replace('/', "-");
            let get = |idx: usize| -> f64 {
                row.and_then(|r| r.get(idx))
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0)
            };
            points.push(SugarOuterQuotePoint {
                date,
                brazil_import_cost: get(0),
                thailand_profit: get(1),
                brazil_profit: get(2),
                thailand_import_cost: get(3),
                rizhao_spot: get(4),
            });
        }

        if points.is_empty() {
            return Err(Error::not_found(
                "msweet returned no outer quote sugar data",
            ));
        }
        Ok(points)
    }
}

// ---------------------------------------------------------------------------
// Supporting types
// ---------------------------------------------------------------------------

/// Msweet sugar composite index point.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SugarMsweetPoint {
    pub date: String,
    pub composite_price: f64,
    pub raw_sugar_price: f64,
    pub spot_price: f64,
}

/// Msweet inner import quote point.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SugarInnerQuotePoint {
    pub date: String,
    pub profit_space: f64,
    pub thailand_sugar: f64,
    pub thailand_ma5: f64,
    pub brazil_ma5: f64,
    pub profit_ma5: f64,
    pub brazil_ma10: f64,
    pub brazil_sugar: f64,
    pub liuzhou_spot: f64,
    pub guangzhou_spot: f64,
    pub thailand_ma10: f64,
    pub profit_ma30: f64,
    pub profit_ma10: f64,
}

/// Msweet outer import quote point.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SugarOuterQuotePoint {
    pub date: String,
    pub brazil_import_cost: f64,
    pub thailand_profit: f64,
    pub brazil_profit: f64,
    pub thailand_import_cost: f64,
    pub rizhao_spot: f64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Sugar functions require network access.
    }
}
