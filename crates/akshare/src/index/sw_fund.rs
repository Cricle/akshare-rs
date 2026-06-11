//! Shenwan (申万宏源) fund index data — realtime and history.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{SwFundHistPoint, SwFundRealtime};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct SwFundPageEnvelope {
    data: Option<SwFundPageData>,
}

#[derive(Debug, Deserialize)]
struct SwFundPageData {
    list: Option<Vec<SwFundPageItem>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct SwFundPageItem {
    #[serde(default)]
    swIndexCode: String,
    #[serde(default)]
    swIndexName: String,
    #[serde(default)]
    lastCloseIndex: Option<f64>,
    #[serde(default)]
    lastMarkup: Option<f64>,
    #[serde(default)]
    yearMarkup: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct SwFundHistEnvelope {
    data: Option<Vec<SwFundHistItem>>,
}

#[derive(Debug, Deserialize)]
struct SwFundHistItem {
    #[serde(default)]
    bargaindate: String,
    #[serde(default)]
    closeindex: Option<f64>,
    #[serde(default)]
    maxindex: Option<f64>,
    #[serde(default)]
    minindex: Option<f64>,
    #[serde(default)]
    openindex: Option<f64>,
    #[serde(default)]
    markup: Option<f64>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 申万宏源研究 — 基金指数实时行情.
    ///
    /// `symbol` is one of: "基础一级", "基础二级", "基础三级", "特色指数".
    pub async fn index_realtime_fund_sw(&self, symbol: &str) -> Result<Vec<SwFundRealtime>> {
        let response = self
            .post("https://www.swsresearch.com/insWechatSw/fundIndex/pageList")
            .json(&serde_json::json!({
                "pageNo": 1,
                "pageSize": 50,
                "indexTypeName": symbol,
                "sortField": "",
                "rule": "",
                "indexType": 1,
            }))
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: SwFundPageEnvelope = response.json().await.map_err(Error::from)?;
        let items = payload.data.and_then(|d| d.list).unwrap_or_default();

        let result: Vec<SwFundRealtime> = items
            .into_iter()
            .map(|item| SwFundRealtime {
                code: item.swIndexCode,
                name: item.swIndexName,
                prev_close: item.lastCloseIndex.unwrap_or(0.0),
                change_pct: item.lastMarkup.unwrap_or(0.0),
                year_change_pct: item.yearMarkup.unwrap_or(0.0),
            })
            .collect();

        if result.is_empty() {
            return Err(Error::not_found(
                "swsresearch returned no fund realtime data",
            ));
        }
        Ok(result)
    }

    /// 申万宏源研究 — 基金指数历史行情.
    ///
    /// `period` is "day", "week", or "month".
    pub async fn index_hist_fund_sw(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<SwFundHistPoint>> {
        let period_upper = match period {
            "day" => "DAY",
            "week" => "WEEK",
            "month" => "MONTH",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported period: {period}"
                )));
            }
        };

        let response = self
            .post("https://www.swsresearch.com/insWechatSw/fundIndex/getFundKChartData")
            .json(&serde_json::json!({
                "swIndexCode": symbol,
                "type": period_upper,
            }))
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: SwFundHistEnvelope = response.json().await.map_err(Error::from)?;
        let items = payload.data.unwrap_or_default();

        let points: Vec<SwFundHistPoint> = items
            .into_iter()
            .map(|item| SwFundHistPoint {
                date: item.bargaindate,
                close: item.closeindex.unwrap_or(0.0),
                open: item.openindex.unwrap_or(0.0),
                high: item.maxindex.unwrap_or(0.0),
                low: item.minindex.unwrap_or(0.0),
                change_pct: item.markup.unwrap_or(0.0),
            })
            .collect();

        if points.is_empty() {
            return Err(Error::not_found(
                "swsresearch returned no fund history data",
            ));
        }
        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // SW fund functions require network access.
    }
}
