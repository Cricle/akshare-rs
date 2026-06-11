//! SSE and SZSE current day option contracts.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Wire types — SSE
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct SseQueryEnvelope {
    result: Option<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// SSE current day option contract info.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionCurrentDaySse {
    /// Contract code.
    pub security_id: String,
    /// Contract trading code.
    pub contract_id: String,
    /// Contract short name.
    pub contract_symbol: String,
    /// Underlying name and code.
    pub underlying: String,
    /// Type (call/put).
    pub call_or_put: String,
    /// Exercise price.
    pub exercise_price: f64,
    /// Contract unit.
    pub contract_unit: f64,
    /// Exercise date.
    pub end_date: String,
    /// Delivery date.
    pub delivery_date: String,
    /// Expiry date.
    pub expire_date: String,
    /// Start date.
    pub start_date: String,
}

/// SZSE current day option contract info.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionCurrentDaySzse {
    /// Sequence number.
    pub index: i64,
    /// Contract code.
    pub contract_code: String,
    /// Contract trading code.
    pub contract_id: String,
    /// Contract short name.
    pub contract_name: String,
    /// Underlying name and code.
    pub underlying: String,
    /// Contract type.
    pub contract_type: String,
    /// Exercise price.
    pub exercise_price: f64,
    /// Contract unit.
    pub contract_unit: f64,
    /// Last trading day.
    pub last_trade_date: String,
    /// Exercise day.
    pub exercise_date: String,
    /// Expiry date.
    pub expire_date: String,
    /// Delivery date.
    pub delivery_date: String,
    /// Upper limit price.
    pub upper_limit_price: f64,
    /// Lower limit price.
    pub lower_limit_price: f64,
    /// Previous settlement price.
    pub prev_settlement: f64,
    /// Total open interest.
    pub total_open_interest: f64,
    /// Trading days until expiry.
    pub remaining_trade_days: i64,
    /// Calendar days until expiry.
    pub remaining_calendar_days: i64,
    /// Trade date.
    pub trade_date: String,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// SSE current day option contracts.
    ///
    /// Fetches all option contracts listed for the current trading day from SSE.
    pub async fn option_current_day_sse(&self) -> Result<Vec<OptionCurrentDaySse>> {
        let url = "http://query.sse.com.cn/commonQuery.do";

        let resp: SseQueryEnvelope = self
            .get(url)
            .query(&[
                ("isPagination", "false"),
                ("expireDate", ""),
                ("securityId", ""),
                ("sqlId", "SSE_ZQPZ_YSP_GGQQZSXT_XXPL_DRHY_SEARCH_L"),
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
            rows.push(OptionCurrentDaySse {
                security_id: json_str(item, "SECURITY_ID"),
                contract_id: json_str(item, "CONTRACT_ID"),
                contract_symbol: json_str(item, "CONTRACT_SYMBOL"),
                underlying: json_str(item, "SECURITYNAMEBYID"),
                call_or_put: json_str(item, "CALL_OR_PUT"),
                exercise_price: json_f64(item, "EXERCISE_PRICE"),
                contract_unit: json_f64(item, "CONTRACT_UNIT"),
                end_date: json_str(item, "END_DATE"),
                delivery_date: json_str(item, "DELIVERY_DATE"),
                expire_date: json_str(item, "EXPIRE_DATE"),
                start_date: json_str(item, "START_DATE"),
            });
        }

        Ok(rows)
    }

    /// SZSE current day option contracts.
    ///
    /// Fetches all option contracts listed for the current trading day from SZSE.
    pub async fn option_current_day_szse(&self) -> Result<Vec<OptionCurrentDaySzse>> {
        let url = "https://www.sse.org.cn/api/report/ShowReport";

        let body = self
            .get(url)
            .query(&[
                ("SHOWTYPE", "JSON"),
                ("CATALOGID", "option_drhy"),
                ("TABKEY", "tab1"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // SZSE returns a JSON array (sometimes wrapped in a specific format)
        let json_val: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| Error::decode(format!("szse option json: {e}")))?;

        // Try to extract data from the response
        let data = if let Some(arr) = json_val.as_array() {
            arr.first()
                .and_then(|v| v.get("data"))
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default()
        } else {
            vec![]
        };

        let mut rows = Vec::with_capacity(data.len());
        for item in &data {
            rows.push(OptionCurrentDaySzse {
                index: json_i64(item, "\u{5e8f}\u{53f7}"),
                contract_code: json_str(item, "\u{5408}\u{7ea6}\u{7f16}\u{7801}"),
                contract_id: json_str(item, "\u{5408}\u{7ea6}\u{4ee3}\u{7801}"),
                contract_name: json_str(item, "\u{5408}\u{7ea6}\u{7b80}\u{79f0}"),
                underlying: json_str(item, "\u{6807}\u{7684}\u{8bc1}\u{5238}\u{7b80}\u{79f0}(\u{4ee3}\u{7801})"),
                contract_type: json_str(item, "\u{5408}\u{7ea6}\u{7c7b}\u{578b}"),
                exercise_price: json_f64(item, "\u{884c}\u{6743}\u{4ef7}"),
                contract_unit: json_f64(item, "\u{5408}\u{7ea6}\u{5355}\u{4f4d}"),
                last_trade_date: json_str(item, "\u{6700}\u{540e}\u{4ea4}\u{6613}\u{65e5}"),
                exercise_date: json_str(item, "\u{884c}\u{6743}\u{65e5}"),
                expire_date: json_str(item, "\u{5230}\u{671f}\u{65e5}"),
                delivery_date: json_str(item, "\u{4ea4}\u{6536}\u{65e5}"),
                upper_limit_price: json_f64(item, "\u{6da8}\u{505c}\u{4ef7}\u{683c}"),
                lower_limit_price: json_f64(item, "\u{8dcc}\u{505c}\u{4ef7}\u{683c}"),
                prev_settlement: json_f64(item, "\u{524d}\u{7ed3}\u{7b97}\u{4ef7}"),
                total_open_interest: json_f64(item, "\u{5408}\u{7ea6}\u{603b}\u{6301}\u{4ed3}"),
                remaining_trade_days: json_i64(item, "\u{5408}\u{7ea6}\u{5230}\u{671f}\u{5269}\u{4f59}\u{4ea4}\u{6613}\u{5929}\u{6570}"),
                remaining_calendar_days: json_i64(item, "\u{5408}\u{7ea6}\u{5230}\u{671f}\u{5269}\u{4f59}\u{81ea}\u{7136}\u{5929}\u{6570}"),
                trade_date: json_str(item, "\u{4ea4}\u{6613}\u{65e5}\u{671f}"),
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

fn json_i64(v: &serde_json::Value, key: &str) -> i64 {
    match v.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0),
        Some(serde_json::Value::String(s)) => s.trim().parse::<i64>().unwrap_or(0),
        _ => 0,
    }
}
