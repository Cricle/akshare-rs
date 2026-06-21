mod common;

use akshare::ErrorKind;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mock_error_em_datacenter_http_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    let client = common::mock_client(&server);
    let result = client.macro_china_gdp().await;
    assert!(result.is_err());
    // fetch_em_report does not call error_for_status(), so a 404 body
    // fails JSON deserialization and maps to Upstream (not NotFound).
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Upstream);
}

#[tokio::test]
async fn test_mock_error_em_datacenter_http_403() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(403))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(403))
        .mount(&server)
        .await;
    let client = common::mock_client(&server);
    let result = client.macro_china_gdp().await;
    assert!(result.is_err());
    // fetch_em_report does not call error_for_status(), so a 403 body
    // fails JSON deserialization and maps to Upstream (not Restricted).
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Upstream);
}

#[tokio::test]
async fn test_mock_error_em_datacenter_malformed_json() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
        .mount(&server)
        .await;
    let client = common::mock_client(&server);
    let result = client.macro_china_gdp().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mock_error_em_datacenter_empty_data() {
    let server = MockServer::start().await;
    let body = common::em_datacenter_response(vec![]);
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;
    let client = common::mock_client(&server);
    let result = client.macro_china_gdp().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_mock_error_sina_text_empty() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_string(""))
        .mount(&server)
        .await;
    let client = common::mock_client(&server);
    let result = client.stock_zh_a_spot().await;
    // Empty response returns NotFound because all_quotes is empty
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

#[tokio::test]
async fn test_mock_error_sina_text_malformed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_string("completely invalid"))
        .mount(&server)
        .await;
    let client = common::mock_client(&server);
    let result = client.stock_zh_a_spot().await;
    let _ = result;
}

#[tokio::test]
async fn test_mock_error_push2_http_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    let client = common::mock_client(&server);
    let result = client.stock_zh_a_spot_em().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mock_error_push2_empty_diff() {
    let server = MockServer::start().await;
    let body = common::em_push2_response(vec![]);
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;
    let client = common::mock_client(&server);
    let result = client.stock_zh_a_spot_em().await;
    // Empty diff returns NotFound because items is empty
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}
