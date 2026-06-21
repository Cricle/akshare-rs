mod common;

use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn mount_mocks(server: &MockServer) {
    common::mount_em_mocks(server).await;
}

macro_rules! macro_test_arg1 {
    ($test_name:ident, $method:ident, $arg:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg).await;
            let _ = result;
        }
    };
}

macro_rules! macro_test_arg2 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg1, $arg2).await;
            let _ = result;
        }
    };
}

macro_test_arg2!(test_mock_baidu_news_search, baidu_news_search, "rust", 10);
macro_test_arg2!(test_mock_bing_news_rss, bing_news_rss, "rust", 10);
macro_test_arg1!(test_mock_seeking_alpha_news, seeking_alpha_news, "AAPL");
macro_test_arg2!(test_mock_sogou_news_search, sogou_news_search, "rust", 10);

#[tokio::test]
async fn test_mock_bing_news_rss_with_lang() {
    let server = MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.bing_news_rss_with_lang("rust", 10, Some("en")).await;
    let _ = result;
}

#[tokio::test]
async fn test_mock_finnhub_company_news() {
    let server = MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.finnhub_company_news("AAPL", "2024-01-01", "2024-01-31", "test_key").await;
    let _ = result;
}

#[tokio::test]
async fn test_mock_gdelt_news_search() {
    let server = MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.gdelt_news_search("rust", "https://api.gdeltproject.org/api/v2/doc/doc", Some("English"), None, 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_mock_gdelt_news_search_owned() {
    let server = MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.gdelt_news_search_owned("rust", "https://api.gdeltproject.org/api/v2/doc/doc", Some("English".to_string()), None, 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_mock_google_news_rss() {
    let server = MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.google_news_rss("rust", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_mock_marketaux_news() {
    let server = MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.marketaux_news("AAPL", "test_key", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_mock_news_search_with_scope() {
    let server = MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.news_search_with_scope("rust", 10, "1").await;
    let _ = result;
}
