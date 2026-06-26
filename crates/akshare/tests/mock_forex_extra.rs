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

macro_rules! macro_test_arg3 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr, $arg3:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg1, $arg2, $arg3).await;
            let _ = result;
        }
    };
}

macro_rules! macro_test_arg4 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg1, $arg2, $arg3, $arg4).await;
            let _ = result;
        }
    };
}

macro_rules! macro_test_arg5 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg1, $arg2, $arg3, $arg4, $arg5).await;
            let _ = result;
        }
    };
}

// No-arg forex methods
macro_test!(test_mock_forex_boc_rates, forex_boc_rates);
macro_test!(test_mock_forex_em_rates, forex_em_rates);
macro_test!(test_mock_forex_spot, forex_spot);
macro_test!(test_mock_forex_sina_rates, forex_sina_rates);
macro_test!(test_mock_currency_pair_map, currency_pair_map);
macro_test!(test_mock_fx_c_swap_cm, fx_c_swap_cm);
macro_test!(test_mock_fx_spot_quote, fx_spot_quote);
macro_test!(test_mock_fx_swap_quote, fx_swap_quote);
macro_test!(test_mock_currency_boc_safe, currency_boc_safe);

// Single-arg forex methods
macro_test_arg1!(test_mock_fx_pair_quote, fx_pair_quote, "USD/CNY");
macro_test_arg1!(test_mock_fx_quote, fx_quote, "USD_CNY");

// Two-arg forex methods
macro_test_arg2!(
    test_mock_forex_em_hist,
    forex_em_hist,
    "133.USDCNY",
    100usize
);

// Three-arg forex methods
macro_test_arg3!(
    test_mock_currency_boc,
    currency_boc,
    "美元",
    "20240101",
    "20240131"
);
macro_test_arg3!(
    test_mock_currency_latest,
    currency_latest,
    "USD",
    "EUR,GBP",
    "test_key"
);

// Four-arg forex methods
macro_test_arg4!(
    test_mock_currency_history,
    currency_history,
    "USD",
    "2024-01-01",
    "EUR,GBP",
    "test_key"
);
macro_test_arg4!(
    test_mock_currency_convert,
    currency_convert,
    "USD",
    "CNY",
    100.0f64,
    "test_key"
);

// Five-arg forex methods
macro_test_arg5!(
    test_mock_forex_hist,
    forex_hist,
    "133.USDCNY",
    "daily",
    "20240101",
    "20241231",
    "qfq"
);
macro_test_arg5!(
    test_mock_currency_time_series,
    currency_time_series,
    "USD",
    "2024-01-01",
    "2024-01-31",
    "EUR,GBP",
    "test_key"
);
