mod common;

async fn mount_mocks(server: &wiremock::MockServer) {
    common::mount_em_mocks(server).await;
}

macro_rules! macro_test_arg2 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = wiremock::MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let _ = client.$method($arg1, $arg2).await;
        }
    };
}

macro_rules! macro_test_arg3 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr, $arg3:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = wiremock::MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let _ = client.$method($arg1, $arg2, $arg3).await;
        }
    };
}

macro_test_arg2!(
    test_index_extra_a_share_candles,
    index_a_share_candles,
    "000300",
    100usize
);
macro_test_arg3!(
    test_index_extra_csindex_hist,
    stock_zh_index_hist_csindex,
    "000300",
    "2024-01-01",
    "2024-12-31"
);
