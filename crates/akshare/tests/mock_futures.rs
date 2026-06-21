mod common;

async fn mount_mocks(server: &wiremock::MockServer) {
    common::mount_em_mocks(server).await;
}

macro_rules! macro_test_arg1 {
    ($test_name:ident, $method:ident, $arg:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = wiremock::MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg).await;
            let _ = result;
        }
    };
}

macro_test_arg1!(test_mock_futures_contract_detail_em, futures_contract_detail_em, "rb2401");
macro_test_arg1!(test_mock_futures_foreign_commodity_realtime_str, futures_foreign_commodity_realtime_str, "CL");

#[tokio::test]
async fn test_mock_futures_main_sina_derivative() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.futures_main_sina_derivative("rb0", "20240101", "20240131").await;
    let _ = result;
}
