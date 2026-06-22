//! SSE option risk indicators.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct SseQueryEnvelope {
    result: Option<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// SSE option risk indicator row.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionRiskIndicatorRow {
    /// Trade date.
    pub trade_date: String,
    /// Security ID.
    pub security_id: String,
    /// Contract ID.
    pub contract_id: String,
    /// Contract symbol.
    pub contract_symbol: String,
    /// Delta.
    pub delta: f64,
    /// Theta.
    pub theta: f64,
    /// Gamma.
    pub gamma: f64,
    /// Vega.
    pub vega: f64,
    /// Rho.
    pub rho: f64,
    /// Implied volatility.
    pub implied_volatility: f64,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// SSE option risk indicators.
    ///
    /// `date` is "YYYYMMDD" (data available from 20150209).
    pub async fn option_risk_indicator(&self, date: &str) -> Result<Vec<OptionRiskIndicatorRow>> {
        let url = "http://query.sse.com.cn/commonQuery.do";

        let resp: SseQueryEnvelope = self
            .get(url)
            .query(&[
                ("isPagination", "false"),
                ("trade_date", date),
                ("sqlId", "SSE_ZQPZ_YSP_GGQQZSXT_YSHQ_QQFXZB_DATE_L"),
                ("contractSymbol", ""),
            ])
            .header("Referer", "http://www.sse.com.cn/")
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let data = resp.result.unwrap_or_default();
        let mut rows = Vec::with_capacity(data.len());

        for item in &data {
            rows.push(OptionRiskIndicatorRow {
                trade_date: json_str(item, "TRADE_DATE"),
                security_id: json_str(item, "SECURITY_ID"),
                contract_id: json_str(item, "CONTRACT_ID"),
                contract_symbol: json_str(item, "CONTRACT_SYMBOL"),
                delta: json_f64(item, "DELTA_VALUE"),
                theta: json_f64(item, "THETA_VALUE"),
                gamma: json_f64(item, "GAMMA_VALUE"),
                vega: json_f64(item, "VEGA_VALUE"),
                rho: json_f64(item, "RHO_VALUE"),
                implied_volatility: json_f64(item, "IMPLC_VOLATLTY"),
            });
        }

        Ok(rows)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn json_str(v: &serde_json::Value, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string()
}

fn json_f64(v: &serde_json::Value, key: &str) -> f64 {
    match v.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(serde_json::Value::String(s)) => s.trim().parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}
