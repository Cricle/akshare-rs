//! Shared wire types for Eastmoney API responses.
//!
//! These types are used across multiple modules to deserialize common
//! response envelopes from Eastmoney datacenter, push2, and kline APIs.

use serde::Deserialize;

// ---------------------------------------------------------------------------
// Eastmoney datacenter response
// ---------------------------------------------------------------------------

/// Envelope for Eastmoney datacenter API responses.
///
/// Used by: macro_data, economy, bond, commodity, forex modules.
#[derive(Debug, Deserialize)]
pub struct EmDatacenterResp {
    pub success: Option<bool>,
    pub message: Option<String>,
    pub code: Option<i64>,
    pub result: Option<EmDatacenterResult>,
}

impl EmDatacenterResp {
    /// Check if the API returned an error and return a descriptive error message.
    pub fn check_error(&self, report_name: &str) -> Option<String> {
        if self.success == Some(false) {
            return Some(format!(
                "Eastmoney report '{}' unavailable: {}",
                report_name,
                self.message.as_deref().unwrap_or("unknown error")
            ));
        }
        None
    }
}

#[derive(Debug, Deserialize)]
pub struct EmDatacenterResult {
    #[serde(default)]
    pub data: Vec<serde_json::Value>,
    pub pages: Option<i64>,
}

// ---------------------------------------------------------------------------
// Eastmoney push2 clist response
// ---------------------------------------------------------------------------

/// Envelope for Eastmoney push2 clist API responses.
///
/// Used by: stock, futures, bond, forex, index, option, reits, provider modules.
/// The `diff` field is `Vec<serde_json::Value>` because each module extracts
/// different fields from the items.
#[derive(Debug, Deserialize)]
pub struct ClistResp {
    pub data: Option<ClistRespData>,
}

#[derive(Debug, Deserialize)]
pub struct ClistRespData {
    pub diff: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub total: Option<i64>,
}

// ---------------------------------------------------------------------------
// Eastmoney kline response
// ---------------------------------------------------------------------------

/// Envelope for Eastmoney push2his kline API responses.
///
/// Used by: stock, bond, forex, option, reits, provider modules.
#[derive(Debug, Deserialize)]
pub struct KlineResp {
    pub data: Option<KlineRespData>,
}

#[derive(Debug, Deserialize)]
pub struct KlineRespData {
    pub klines: Option<Vec<String>>,
}
