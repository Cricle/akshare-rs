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

macro_rules! macro_test_arg2 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = wiremock::MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg1, $arg2).await;
            let _ = result;
        }
    };
}

// 2-arg functions
macro_test_arg2!(test_mock_stooq_candles, stooq_candles, "AAPL.US", 100);
#[tokio::test]
async fn test_mock_eastmoney_klines() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.eastmoney_klines("1.600000", "qfq", 100).await;
    let _ = result;
}
macro_test_arg2!(test_mock_eastmoney_sector_rankings, eastmoney_sector_rankings, "industry", 10);
macro_test_arg2!(test_mock_eastmoney_sector_constituents, eastmoney_sector_constituents, "BK0475", 10);
macro_test_arg2!(test_mock_eastmoney_sector_capital_flow, eastmoney_sector_capital_flow, "BK0475", 10);
macro_test_arg2!(test_mock_eastmoney_capital_flow, eastmoney_capital_flow, "1.600000", 10);
macro_test_arg2!(test_mock_eastmoney_billboard, eastmoney_billboard, "600000", 10);
macro_test_arg2!(test_mock_eastmoney_announcements, eastmoney_announcements, "600000", 10);
macro_test_arg2!(test_mock_sina_us_daily, sina_us_daily, "AAPL", 100);

// 1-arg functions
macro_test_arg1!(test_mock_eastmoney_announcement_detail, eastmoney_announcement_detail, "test_art_code");
macro_test_arg1!(test_mock_sina_a_share_realtime, sina_a_share_realtime, "sh600000");

// 3-arg functions
#[tokio::test]
async fn test_mock_eastmoney_search() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.eastmoney_search("浦发银行", Some("沪A"), 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_mock_eastmoney_billboard_seats() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.eastmoney_billboard_seats("600000", "buy", 10).await;
    let _ = result;
}
