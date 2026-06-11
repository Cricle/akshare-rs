//! Comparison and valuation data — Dupont, scale, Baidu valuation, Baidu vote.
//!
//! Covers Python functions:
//! - `stock_zh_dupont_comparison_em` — Dupont comparison from Eastmoney
//! - `stock_zh_scale_comparison_em` — Scale comparison from Eastmoney
//! - `stock_zh_valuation_baidu` — Baidu valuation data
//! - `stock_zh_vote_baidu` — Baidu vote data

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Dupont comparison entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DupontComparison {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub roe: Option<f64>,
    #[serde(default)]
    pub net_profit_margin: Option<f64>,
    #[serde(default)]
    pub asset_turnover: Option<f64>,
    #[serde(default)]
    pub equity_multiplier: Option<f64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Scale comparison entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleComparison {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub total_market_cap: Option<f64>,
    #[serde(default)]
    pub circulating_market_cap: Option<f64>,
    #[serde(default)]
    pub total_shares: Option<f64>,
    #[serde(default)]
    pub circulating_shares: Option<f64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Baidu valuation data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaiduValuation {
    pub date: String,
    pub value: f64,
}

/// Baidu vote data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaiduVote {
    pub period: String,
    #[serde(default)]
    pub bullish: Option<i64>,
    #[serde(default)]
    pub bearish: Option<i64>,
    #[serde(default)]
    pub bullish_ratio: Option<f64>,
    #[serde(default)]
    pub bearish_ratio: Option<f64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get Dupont comparison data from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_dupont_comparison_em(symbol)`
    ///
    /// - `symbol`: stock code like "SZ000895"
    pub async fn stock_zh_dupont_comparison_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<DupontComparison>> {
        let filter = if symbol.len() >= 2 {
            let (prefix, code) = symbol.split_at(2);
            format!("(SECUCODE=\"{}.{}\")", code, prefix)
        } else {
            return Err(Error::invalid_input("symbol must be like SZ000895"));
        };

        let url = "https://datacenter.eastmoney.com/securities/api/data/v1/get";
        let response = self
                        .get(url)
            .query(&[
                ("reportName", "RPT_PCF10_INDUSTRY_DUPONT"),
                ("columns", "ALL"),
                ("quoteColumns", ""),
                ("filter", filter.as_str()),
                ("pageNumber", ""),
                ("pageSize", ""),
                ("sortTypes", "1"),
                ("sortColumns", "PAIMING"),
                ("source", "HSF10"),
                ("client", "PC"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        parse_datacenter_response(response).await
    }

    /// Get scale comparison data from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_scale_comparison_em(symbol)`
    ///
    /// - `symbol`: stock code like "SZ000895"
    pub async fn stock_zh_scale_comparison_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<ScaleComparison>> {
        let filter = if symbol.len() >= 2 {
            let (prefix, code) = symbol.split_at(2);
            format!("(SECUCODE=\"{}.{}\")", code, prefix)
        } else {
            return Err(Error::invalid_input("symbol must be like SZ000895"));
        };

        let url = "https://datacenter.eastmoney.com/securities/api/data/v1/get";
        let response = self
                        .get(url)
            .query(&[
                ("reportName", "RPT_PCF10_INDUSTRY_SCALE"),
                ("columns", "ALL"),
                ("quoteColumns", ""),
                ("filter", filter.as_str()),
                ("pageNumber", ""),
                ("pageSize", ""),
                ("sortTypes", "1"),
                ("sortColumns", "PAIMING"),
                ("source", "HSF10"),
                ("client", "PC"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        parse_datacenter_response(response).await
    }

    /// Get Baidu valuation data.
    ///
    /// Python equivalent: `stock_zh_valuation_baidu(symbol, indicator, period)`
    ///
    /// - `symbol`: stock code like "002044"
    /// - `indicator`: "总市值", "市盈率(TTM)", "市盈率(静)", "市净率", "市现率"
    /// - `period`: "近一年", "近三年", "近五年", "近十年", "全部"
    pub async fn stock_zh_valuation_baidu(
        &self,
        symbol: &str,
        indicator: &str,
        period: &str,
    ) -> Result<Vec<BaiduValuation>> {
        let url = "https://gushitong.baidu.com/opendata";
        let response = self
                        .get(url)
            .query(&[
                ("openapi", "1"),
                ("dspName", "iphone"),
                ("tn", "tangram"),
                ("client", "app"),
                ("query", indicator),
                ("code", symbol),
                ("word", ""),
                ("resource_id", "51171"),
                ("market", "ab"),
                ("tag", indicator),
                ("chart_select", period),
                ("industry_select", ""),
                ("skip_industry", "1"),
                ("finClientType", "pc"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        parse_baidu_valuation(response).await
    }

    /// Get Baidu vote data.
    ///
    /// Python equivalent: `stock_zh_vote_baidu(symbol, indicator)`
    ///
    /// - `symbol`: stock code like "000001"
    /// - `indicator`: "指数" or "股票"
    pub async fn stock_zh_vote_baidu(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<BaiduVote>> {
        let finance_type = match indicator {
            "股票" => "stock",
            "指数" => "index",
            _ => return Err(Error::invalid_input(format!("invalid indicator: {indicator}"))),
        };

        let url = "https://finance.pae.baidu.com/vapi/v1/stockvoterecords";
        let mut votes = Vec::new();

        for period in &["day", "week", "month", "year"] {
            let response = self
                                .get(url)
                .query(&[
                    ("code", symbol),
                    ("market", "ab"),
                    ("finance_type", finance_type),
                    ("select_type", period),
                    ("from_smart_app", "0"),
                    ("method", "query"),
                    ("finClientType", "pc"),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let data: serde_json::Value = response.json().await.map_err(Error::from)?;

            let vote_res = data
                .get("Result")
                .and_then(|r| r.get("voteRecords"))
                .and_then(|v| v.get("voteRes"))
                .and_then(|v| v.as_array());

            if let Some(arr) = vote_res {
                for item in arr {
                    let item_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    if item_type == *period {
                        votes.push(BaiduVote {
                            period: period.to_string(),
                            bullish: item.get("bullCount").and_then(|v| v.as_i64()),
                            bearish: item.get("bearCount").and_then(|v| v.as_i64()),
                            bullish_ratio: item.get("bullRatio").and_then(|v| v.as_f64()),
                            bearish_ratio: item.get("bearRatio").and_then(|v| v.as_f64()),
                        });
                    }
                }
            }
        }

        if votes.is_empty() {
            return Err(Error::not_found("baidu vote returned no data"));
        }
        Ok(votes)
    }
}

/// Parse Eastmoney datacenter generic response.
async fn parse_datacenter_response<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<Vec<T>> {
    let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
    let data_arr = payload
        .get("result")
        .and_then(|r| r.get("data"))
        .and_then(|d| d.as_array())
        .ok_or_else(|| Error::upstream("datacenter returned no data"))?;

    let mut items = Vec::new();
    for val in data_arr {
        match serde_json::from_value::<T>(val.clone()) {
            Ok(item) => items.push(item),
            Err(_) => continue,
        }
    }

    if items.is_empty() {
        return Err(Error::not_found("datacenter returned empty data"));
    }
    Ok(items)
}

/// Parse Baidu valuation response.
async fn parse_baidu_valuation(response: reqwest::Response) -> Result<Vec<BaiduValuation>> {
    let data: serde_json::Value = response.json().await.map_err(Error::from)?;

    let body = data
        .get("Result")
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("DisplayData"))
        .and_then(|d| d.get("resultData"))
        .and_then(|r| r.get("tplData"))
        .and_then(|t| t.get("result"))
        .and_then(|r| r.get("chartInfo"))
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("body"))
        .and_then(|b| b.as_array())
        .ok_or_else(|| Error::upstream("baidu valuation missing chart data"))?;

    let items: Vec<BaiduValuation> = body
        .iter()
        .filter_map(|item| {
            let arr = item.as_array()?;
            if arr.len() < 2 {
                return None;
            }
            let date = arr[0].as_str().unwrap_or("").to_string();
            let value = arr[1].as_f64().unwrap_or(0.0);
            Some(BaiduValuation { date, value })
        })
        .collect();

    if items.is_empty() {
        return Err(Error::not_found("baidu valuation returned no data"));
    }
    Ok(items)
}
