mod common;

use akshare::ErrorKind;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn unavailable_stubs_are_explicit() {
    let client = common::mock_client(&wiremock::MockServer::start().await);

    assert_eq!(
        client.stock_esg_rate("600000").await.unwrap_err().kind(),
        ErrorKind::NotFound
    );
    assert_eq!(
        client.stock_ipo_benefit().await.unwrap_err().kind(),
        ErrorKind::Unavailable
    );
    assert_eq!(
        client
            .stock_info_sz_name_code("A股列表")
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::NotFound
    );
}
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
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
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
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Restricted);
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
    let body = common::em_datacenter_response(&[]);
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
    assert!(result.is_err());
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
    let body = common::em_push2_response(&[]);
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
