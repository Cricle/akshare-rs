//! SSE and SZSE option daily statistics.

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

/// SSE option daily statistics.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionDailyStatsSse {
    /// Underlying security code.
    pub security_code: String,
    /// Underlying security name.
    pub security_name: String,
    /// Number of contracts.
    pub contract_count: f64,
    /// Total turnover amount.
    pub total_amount: f64,
    /// Total volume.
    pub total_volume: f64,
    /// Call volume.
    pub call_volume: f64,
    /// Put volume.
    pub put_volume: f64,
    /// Put/call ratio.
    pub put_call_ratio: f64,
    /// Total open interest.
    pub total_open_interest: f64,
    /// Call open interest.
    pub call_open_interest: f64,
    /// Put open interest.
    pub put_open_interest: f64,
    /// Trade date.
    pub trade_date: String,
}

/// SZSE option daily statistics.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionDailyStatsSzse {
    /// Underlying security code.
    pub security_code: String,
    /// Underlying security name.
    pub security_name: String,
    /// Volume.
    pub volume: f64,
    /// Call volume.
    pub call_volume: f64,
    /// Put volume.
    pub put_volume: f64,
    /// Put/call position ratio.
    pub put_call_position_ratio: f64,
    /// Total open interest.
    pub total_open_interest: f64,
    /// Call open interest.
    pub call_open_interest: f64,
    /// Put open interest.
    pub put_open_interest: f64,
    /// Trade date.
    pub trade_date: String,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// SSE option daily statistics.
    ///
    /// `date` is "YYYYMMDD".
    pub async fn option_daily_stats_sse(&self, date: &str) -> Result<Vec<OptionDailyStatsSse>> {
        let url = "http://query.sse.com.cn/commonQuery.do";

        let resp: SseQueryEnvelope = self
            .get(url)
            .query(&[
                ("isPagination", "false"),
                ("sqlId", "COMMON_SSE_ZQPZ_YSP_QQ_SJTJ_MRTJ_CX"),
                ("tradeDate", date),
            ])
            .header("Referer", "https://www.sse.com.cn/")
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let data = resp.result.unwrap_or_default();
        let mut rows = Vec::with_capacity(data.len());

        for item in &data {
            rows.push(OptionDailyStatsSse {
                security_code: json_str(item, "SECURITY_CODE"),
                security_name: json_str(item, "SECURITY_ABBR"),
                contract_count: json_f64_clean(item, "CONTRACT_VOLUME"),
                total_amount: json_f64_clean(item, "TOTAL_MONEY"),
                total_volume: json_f64_clean(item, "TOTAL_VOLUME"),
                call_volume: json_f64_clean(item, "CALL_VOLUME"),
                put_volume: json_f64_clean(item, "PUT_VOLUME"),
                put_call_ratio: json_f64_clean(item, "CP_RATE"),
                total_open_interest: json_f64_clean(item, "LEAVES_QTY"),
                call_open_interest: json_f64_clean(item, "LEAVES_CALL_QTY"),
                put_open_interest: json_f64_clean(item, "LEAVES_PUT_QTY"),
                trade_date: json_str(item, "TRADE_DATE"),
            });
        }

        Ok(rows)
    }

    /// SZSE option daily statistics.
    ///
    /// `date` is "YYYYMMDD".
    pub async fn option_daily_stats_szse(&self, date: &str) -> Result<Vec<OptionDailyStatsSzse>> {
        let date_formatted = if date.len() >= 8 {
            format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8])
        } else {
            date.to_string()
        };

        let url = "https://investor.szse.cn/api/report/ShowReport/data";

        let body = self
            .get(url)
            .query(&[
                ("SHOWTYPE", "JSON"),
                ("CATALOGID", "ysprdzb"),
                ("TABKEY", "tab1"),
                ("txtQueryDate", date_formatted.as_str()),
                ("random", "0.0652692406565949"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        let json_val: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| Error::decode(format!("szse daily stats json: {e}")))?;

        let data = json_val
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|v| v.get("data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut rows = Vec::with_capacity(data.len());
        for item in &data {
            rows.push(OptionDailyStatsSzse {
                security_code: json_str(item, "bddm"),
                security_name: json_str(item, "bdmc"),
                volume: json_f64_clean(item, "cjl"),
                call_volume: json_f64_clean(item, "rccjl"),
                put_volume: json_f64_clean(item, "rpcjl"),
                put_call_position_ratio: json_f64_clean(item, "rcrpccb"),
                total_open_interest: json_f64_clean(item, "wpchyzs"),
                call_open_interest: json_f64_clean(item, "wpcrchys"),
                put_open_interest: json_f64_clean(item, "wpcrphys"),
                trade_date: date_formatted.clone(),
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

/// Parse a JSON value that may contain commas in numeric strings.
fn json_f64_clean(v: &serde_json::Value, key: &str) -> f64 {
    match v.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(serde_json::Value::String(s)) => {
            s.trim().replace(',', "").parse::<f64>().unwrap_or(0.0)
        }
        _ => 0.0,
    }
}
