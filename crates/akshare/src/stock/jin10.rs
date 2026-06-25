//! Jin10 data center — Weibo NLP sentiment report.
//!
//! Covers Python functions:
//! - `stock_js_weibo_nlp_time` — Available time periods
//! - `stock_js_weibo_report` — Weibo sentiment report

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ConfigEnvelope {
    data: Option<ConfigData>,
}

#[derive(Debug, Deserialize)]
struct ConfigData {
    timescale: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct WeiboEnvelope {
    data: Option<Vec<WeiboItem>>,
}

#[derive(Debug, Deserialize)]
struct WeiboItem {
    symbol: Option<String>,
    name: Option<String>,
    rate: Option<f64>,
    #[serde(rename = "closePx")]
    close_px: Option<f64>,
    #[serde(rename = "changePx")]
    change_px: Option<f64>,
    #[serde(rename = "changeRate")]
    change_rate: Option<f64>,
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Weibo sentiment report entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeiboReportEntry {
    pub symbol: String,
    pub name: String,
    #[serde(default)]
    pub sentiment_rate: Option<f64>,
    #[serde(default)]
    pub close_price: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get available time periods for Weibo NLP reports.
    ///
    /// Python equivalent: `stock_js_weibo_nlp_time()`
    pub async fn stock_js_weibo_nlp_time(&self) -> Result<HashMap<String, String>> {
        let response = crate::util::send_and_check(
            self.get("https://datacenter-api.jin10.com/weibo/config")
                .header("x-app-id", "rU6QIu7JHe2gOUeR")
                .header("x-version", "1.0.0")
                .header("origin", "https://datacenter.jin10.com")
                .header("referer", "https://datacenter.jin10.com/market"),
        )
        .await?;

        let payload: ConfigEnvelope = response.json().await.map_err(Error::from)?;
        Ok(payload.data.and_then(|d| d.timescale).unwrap_or_default())
    }

    /// Get Weibo NLP sentiment report.
    ///
    /// Python equivalent: `stock_js_weibo_report(time_period)`
    ///
    /// `time_period` is one of: "CNHOUR2", "CNHOUR6", "CNHOUR12", "CNHOUR24", "CNDAY7", "CNDAY30".
    pub async fn stock_js_weibo_report(&self, time_period: &str) -> Result<Vec<WeiboReportEntry>> {
        let response = crate::util::send_and_check(
            self.get("https://datacenter-api.jin10.com/weibo/list")
                .query(&[("timescale", time_period)])
                .header("x-app-id", "rU6QIu7JHe2gOUeR")
                .header("x-version", "1.0.0")
                .header("origin", "https://datacenter.jin10.com")
                .header("referer", "https://datacenter.jin10.com/market"),
        )
        .await?;

        let payload: WeiboEnvelope = response.json().await.map_err(Error::from)?;

        Ok(payload
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|item| WeiboReportEntry {
                symbol: item.symbol.unwrap_or_default(),
                name: item.name.unwrap_or_default(),
                sentiment_rate: item.rate,
                close_price: item.close_px,
                change_amount: item.change_px,
                change_pct: item.change_rate,
            })
            .collect())
    }
}
