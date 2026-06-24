//! Generic JSON parsing helpers for Eastmoney response data.
//!
//! The shared Eastmoney API methods (`kline_fetch`, `dc_fetch_all`,
//! `clist_spot_fetch`, `push2ex_fetch`, `emweb_financial_fetch`) have been
//! moved to `crate::provider::eastmoney` so they are always compiled.

/// Get a string field from a JSON value, returning default if missing.
pub(crate) fn json_str(v: &serde_json::Value, key: &str) -> String {
    v.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// Get a string field as Option from a JSON value.
pub(crate) fn json_str_opt(v: &serde_json::Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|v| v.as_str())
        .map(std::string::ToString::to_string)
}

/// Get a f64 field from a JSON value, returning 0.0 if missing.
pub(crate) fn json_f64(v: &serde_json::Value, key: &str) -> f64 {
    v.get(key)
        .and_then(serde_json::Value::as_f64)
        .unwrap_or(0.0)
}

/// Get a f64 field as Option from a JSON value.
pub(crate) fn json_f64_opt(v: &serde_json::Value, key: &str) -> Option<f64> {
    v.get(key).and_then(serde_json::Value::as_f64)
}

/// Get an i64 field from a JSON value, returning 0 if missing.
pub(crate) fn json_i64(v: &serde_json::Value, key: &str) -> i64 {
    v.get(key).and_then(serde_json::Value::as_i64).unwrap_or(0)
}

/// Get an i64 field as Option from a JSON value.
pub(crate) fn json_i64_opt(v: &serde_json::Value, key: &str) -> Option<i64> {
    v.get(key).and_then(serde_json::Value::as_i64)
}
