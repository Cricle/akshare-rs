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

// No-arg fund methods not covered in mock_fund.rs
macro_test!(test_mock_fund_private_fund_nav_em, fund_private_fund_nav_em);
macro_test!(
    test_mock_fund_private_fund_rank_em,
    fund_private_fund_rank_em
);
macro_test!(
    test_mock_fund_private_fund_manager_em,
    fund_private_fund_manager_em
);
