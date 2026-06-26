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

// No-arg commodity methods
macro_test!(test_mock_energy_oil_hist, energy_oil_hist);
macro_test!(test_mock_energy_carbon_domestic, energy_carbon_domestic);
macro_test!(test_mock_energy_carbon_bj, energy_carbon_bj);
macro_test!(test_mock_energy_carbon_sz, energy_carbon_sz);
macro_test!(test_mock_energy_carbon_eu, energy_carbon_eu);
macro_test!(test_mock_energy_carbon_hb, energy_carbon_hb);
macro_test!(test_mock_energy_carbon_gz, energy_carbon_gz);

// Single-arg commodity methods
macro_test_arg1!(
    test_mock_commodity_spot_prices,
    commodity_spot_prices,
    100usize
);
macro_test_arg1!(test_mock_energy_oil_detail, energy_oil_detail, "20240102");
