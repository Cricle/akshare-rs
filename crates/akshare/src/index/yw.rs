//! 义乌小商品指数.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 义乌小商品指数.
    ///
    /// `symbol` is one of: "周价格指数", "月价格指数", "月景气指数".
    pub async fn index_yw(&self, symbol: &str) -> Result<Vec<YwIndexPoint>> {
        match symbol {
            "月景气指数" => self.index_yw_bi().await,
            "周价格指数" => self.index_yw_price("piweek").await,
            "月价格指数" => self.index_yw_price("month").await,
            _ => Err(Error::invalid_input(format!(
                "unsupported YW symbol: {symbol}"
            ))),
        }
    }

    async fn index_yw_bi(&self) -> Result<Vec<YwIndexPoint>> {
        #[derive(Deserialize)]
        struct Envelope {
            data: Option<Vec<YwBiRow>>,
        }
        #[derive(Deserialize)]
        struct YwBiRow {
            #[serde(default)]
            indextimeno: String,
            #[serde(default)]
            totalindex: Option<f64>,
            #[serde(default)]
            scopeindex: Option<f64>,
            #[serde(default)]
            benifitindex: Option<f64>,
            #[serde(default)]
            confidentindex: Option<f64>,
        }

        let response = crate::util::send_and_check(
            self.get(
                "https://apiserver.chinagoods.com/yiwuindex/v1/active/industry/class/history/bi",
            )
            .query(&[("gcCode", "")]),
        )
        .await?;

        let payload: Envelope = response.json().await.map_err(Error::from)?;
        let rows = payload.data.unwrap_or_default();

        let points: Vec<YwIndexPoint> = rows
            .into_iter()
            .map(|r| YwIndexPoint {
                period: r.indextimeno,
                index_type: "月景气指数".to_string(),
                value1: r.totalindex.unwrap_or(0.0),
                value2: r.scopeindex.unwrap_or(0.0),
                value3: r.benifitindex.unwrap_or(0.0),
                value4: r.confidentindex.unwrap_or(0.0),
                value5: 0.0,
            })
            .collect();

        if points.is_empty() {
            return Err(Error::not_found("chinagoods returned no YW bi data"));
        }
        Ok(points)
    }

    async fn index_yw_price(&self, path: &str) -> Result<Vec<YwIndexPoint>> {
        #[derive(Deserialize)]
        struct Envelope {
            data: Option<Vec<serde_json::Value>>,
        }

        let url = format!(
            "https://apiserver.chinagoods.com/yiwuindex/v1/active/industry/class/history/{path}"
        );
        let response = crate::util::send_and_check(self.get(&url).query(&[("gcCode", "")])).await?;

        let payload: Envelope = response.json().await.map_err(Error::from)?;
        let data = payload.data.unwrap_or_default();

        let points: Vec<YwIndexPoint> = data
            .into_iter()
            .filter_map(|row| {
                let obj = row.as_object()?;
                let period = obj.get("indextimeno")?.as_str()?.to_string();
                let v1 = obj.get("totalpriceindex")?.as_f64().unwrap_or(0.0);
                let v2 = obj.get("stockdealpriceindex")?.as_f64().unwrap_or(0.0);
                let v3 = obj.get("netdealpriceindex")?.as_f64().unwrap_or(0.0);
                let v4 = obj.get("orderdealpriceindex")?.as_f64().unwrap_or(0.0);
                let v5 = obj.get("outdealpriceindex")?.as_f64().unwrap_or(0.0);
                Some(YwIndexPoint {
                    period,
                    index_type: if path == "piweek" {
                        "周价格指数".to_string()
                    } else {
                        "月价格指数".to_string()
                    },
                    value1: v1,
                    value2: v2,
                    value3: v3,
                    value4: v4,
                    value5: v5,
                })
            })
            .collect();

        if points.is_empty() {
            return Err(Error::not_found("chinagoods returned no YW price data"));
        }
        Ok(points)
    }
}

/// YW (义乌) index point — generic structure for both price and prosperity.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct YwIndexPoint {
    pub period: String,
    pub index_type: String,
    /// For price: total price index; for bi: prosperity index.
    pub value1: f64,
    /// For price: spot price index; for bi: scale index.
    pub value2: f64,
    /// For price: online price index; for bi: benefit index.
    pub value3: f64,
    /// For price: order price index; for bi: confidence index.
    pub value4: f64,
    /// For price: export price index; for bi: unused (0).
    pub value5: f64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // YW functions require network access.
    }
}
