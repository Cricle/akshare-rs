mod common;

use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mock_eastmoney_datacenter() {
    let server = MockServer::start().await;

    // Mock POST for eastmoney datacenter
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(common::em_datacenter_response(vec![
                common::sample_em_stock_row("000001", "平安银行"),
                common::sample_em_stock_row("600000", "浦发银行"),
            ])),
        )
        .mount(&server)
        .await;

    let client = common::mock_client(&server);
    // Test that mock client can be created
    let _ = client;
}

#[tokio::test]
async fn test_mock_em_push2() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(common::em_push2_response(vec![
                common::sample_em_stock_row("000001", "平安银行"),
            ])),
        )
        .mount(&server)
        .await;

    let _client = common::mock_client(&server);
}

#[tokio::test]
async fn test_mock_em_kline() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(common::em_kline_response(vec![
                &common::sample_kline_str("2024-01-02"),
                &common::sample_kline_str("2024-01-03"),
            ])),
        )
        .mount(&server)
        .await;

    let _client = common::mock_client(&server);
}

#[tokio::test]
async fn test_em_datacenter_response_format() {
    let resp = common::em_datacenter_response(vec![]);
    assert_eq!(resp["success"], true);
    assert_eq!(resp["code"], 0);
    assert_eq!(resp["result"]["count"], 0);
}

#[tokio::test]
async fn test_em_push2_response_format() {
    let resp = common::em_push2_response(vec![]);
    assert_eq!(resp["rc"], 0);
    assert_eq!(resp["data"]["total"], 0);
}

#[tokio::test]
async fn test_sample_em_stock_row() {
    let row = common::sample_em_stock_row("000001", "平安银行");
    assert_eq!(row["f12"], "000001");
    assert_eq!(row["f14"], "平安银行");
    assert_eq!(row["f2"], 10.50);
}

#[tokio::test]
async fn test_sample_kline_str() {
    let kline = common::sample_kline_str("2024-01-02");
    assert!(kline.starts_with("2024-01-02,"));
    assert!(kline.contains("10.00"));
}

#[tokio::test]
async fn test_sample_macro_row() {
    let row = common::sample_macro_row("2024-01-01", 123.45, "GDP");
    assert_eq!(row["REPORT_DATE"], "2024-01-01");
    assert_eq!(row["INDICATOR_VALUE"], 123.45);
    assert_eq!(row["INDICATOR_NAME"], "GDP");
}
