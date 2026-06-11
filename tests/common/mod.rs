//! Shared test infrastructure for akshare crate tests.
//!
//! Provides mock HTTP server setup and sample response data.

#![allow(dead_code)]

use akshare::AkShareClient;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Create an AkShareClient that talks to a local mock server.
pub fn mock_client(server: &MockServer) -> AkShareClient {
    let mut client = AkShareClient::new();
    client.mock_uri = Some(server.uri());
    client
}

/// Generic Eastmoney datacenter response wrapper.
pub fn em_datacenter_response(data: Vec<serde_json::Value>) -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": data,
            "count": data.len()
        }
    })
}

/// Generic Eastmoney push2 list response.
pub fn em_push2_response(rows: Vec<serde_json::Value>) -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": rows.len(),
            "diff": rows
        }
    })
}

/// Generic Eastmoney push2his kline response.
pub fn em_kline_response(klines: Vec<&str>) -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "code": "000001",
            "klines": klines
        }
    })
}

/// Register a mock for any GET request matching a path pattern.
pub async fn mock_any_get(server: &MockServer, path_pattern: &str, body: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path_regex(path_pattern))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(server)
        .await;
}

/// Register a mock for any POST request matching a path pattern.
pub async fn mock_any_post(server: &MockServer, path_pattern: &str, body: serde_json::Value) {
    Mock::given(method("POST"))
        .and(path_regex(path_pattern))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(server)
        .await;
}

/// Register a mock for any GET request returning raw text.
pub async fn mock_any_get_text(server: &MockServer, path_pattern: &str, body: &str) {
    Mock::given(method("GET"))
        .and(path_regex(path_pattern))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(server)
        .await;
}

/// Sample Eastmoney stock row (f2=price, f3=change%, f4=change, f5=volume, f6=amount,
/// f7=amplitude%, f8=turnover%, f9=PE, f10=volume_ratio, f12=code, f14=name, f15=high,
/// f16=low, f17=open, f18=prev_close).
pub fn sample_em_stock_row(code: &str, name: &str) -> serde_json::Value {
    serde_json::json!({
        "f2": 10.50, "f3": 1.5, "f4": 0.15, "f5": 1000000, "f6": 10500000.0,
        "f7": 2.0, "f8": 1.2, "f9": 15.0, "f10": 1.1, "f12": code, "f14": name,
        "f15": 10.80, "f16": 10.20, "f17": 10.30, "f18": 10.35
    })
}

/// Sample candlestick kline string: "2024-01-02,10.00,10.50,10.80,9.90,100000,10500000,2.0,1.5,0.15,1.2"
pub fn sample_kline_str(date: &str) -> String {
    format!(
        "{},10.00,10.50,10.80,9.90,100000,10500000.0,2.0,1.5,0.15,1.2",
        date
    )
}

/// Sample MacroDataPoint-style JSON from Eastmoney datacenter.
pub fn sample_macro_row(date: &str, value: f64, name: &str) -> serde_json::Value {
    serde_json::json!({
        "REPORT_DATE": date,
        "DATA_VALUE": value,
        "INDICATOR_NAME": name
    })
}
