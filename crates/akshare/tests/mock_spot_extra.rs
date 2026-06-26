mod common;

use wiremock::MockServer;

async fn mount_mocks(server: &MockServer) {
    common::mount_em_mocks(server).await;
}

macro_rules! macro_test {
    ($test_name:ident, $method:ident) => {
        #[tokio::test]
        async fn $test_name() {
            let server = MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method().await;
            let _ = result;
        }
    };
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

// Single-arg spot methods
macro_test_arg1!(
    test_mock_spot_quotations_sge,
    spot_quotations_sge,
    "Au99.99"
);
macro_test_arg1!(test_mock_spot_hist_sge, spot_hist_sge, "Au99.99");
macro_test_arg1!(test_mock_spot_price_qh, spot_price_qh, "螺纹钢");

// No-arg spot methods
macro_test!(
    test_mock_spot_golden_benchmark_sge,
    spot_golden_benchmark_sge
);
macro_test!(
    test_mock_spot_silver_benchmark_sge,
    spot_silver_benchmark_sge
);
macro_test!(test_mock_spot_price_table_qh, spot_price_table_qh);
macro_test!(test_mock_spot_hog_soozhu, spot_hog_soozhu);
macro_test!(
    test_mock_spot_hog_year_trend_soozhu,
    spot_hog_year_trend_soozhu
);
macro_test!(
    test_mock_spot_hog_lean_price_soozhu,
    spot_hog_lean_price_soozhu
);
macro_test!(
    test_mock_spot_hog_three_way_soozhu,
    spot_hog_three_way_soozhu
);
macro_test!(
    test_mock_spot_hog_crossbred_soozhu,
    spot_hog_crossbred_soozhu
);
macro_test!(test_mock_spot_corn_price_soozhu, spot_corn_price_soozhu);
macro_test!(
    test_mock_spot_soybean_price_soozhu,
    spot_soybean_price_soozhu
);
macro_test!(test_mock_spot_mixed_feed_soozhu, spot_mixed_feed_soozhu);
