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

// No-arg bond methods not covered in mock_bond.rs
macro_test!(test_mock_bond_zh_us_rate_latest, bond_zh_us_rate_latest);
macro_test!(test_mock_bond_yield_curve, bond_yield_curve);

// Single-arg bond methods not covered in mock_bond.rs
macro_test_arg1!(test_mock_bond_repo_sse, bond_repo_sse, "20240102");
macro_test_arg1!(test_mock_bond_repo_szse, bond_repo_szse, "20240102");
macro_test_arg1!(
    test_mock_bond_treasury_issue_em,
    bond_treasury_issue_em,
    "20240102"
);
