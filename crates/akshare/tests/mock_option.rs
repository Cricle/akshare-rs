mod common;

async fn mount_mocks(server: &wiremock::MockServer) {
    common::mount_em_mocks(server).await;
}

macro_rules! macro_test {
    ($test_name:ident, $method:ident) => {
        #[tokio::test]
        async fn $test_name() {
            let server = wiremock::MockServer::start().await;
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

macro_rules! macro_test_arg3 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr, $arg3:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = wiremock::MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg1, $arg2, $arg3).await;
            let _ = result;
        }
    };
}

// No-arg option methods
macro_test!(test_mock_option_comm_symbol, option_comm_symbol);
macro_test!(test_mock_option_contract_info_ctp, option_contract_info_ctp);
macro_test!(test_mock_option_current, option_current);
macro_test!(test_mock_option_current_cffex, option_current_cffex);
macro_test!(test_mock_option_current_day_sse, option_current_day_sse);
macro_test!(test_mock_option_current_day_szse, option_current_day_szse);
macro_test!(test_mock_option_margin_symbol, option_margin_symbol);
macro_test!(test_mock_option_premium_analysis, option_premium_analysis);
macro_test!(test_mock_option_risk_analysis, option_risk_analysis);
macro_test!(test_mock_option_value_analysis, option_value_analysis);
macro_test!(test_mock_option_cffex_hs300_list, option_cffex_hs300_list);
macro_test!(test_mock_option_cffex_sz50_list, option_cffex_sz50_list);
macro_test!(test_mock_option_cffex_zz1000_list, option_cffex_zz1000_list);

// Single-arg option methods
macro_test_arg1!(test_mock_option_comm_info, option_comm_info, "SR");
macro_test_arg1!(test_mock_option_commodity_contract, option_commodity_contract, "SR");
macro_test_arg1!(test_mock_option_commodity_hist, option_commodity_hist, "SR");
macro_test_arg1!(test_mock_option_cffex_hs300_daily, option_cffex_hs300_daily, "2401");
macro_test_arg1!(test_mock_option_cffex_hs300_spot, option_cffex_hs300_spot, "2401");
macro_test_arg1!(test_mock_option_cffex_sz50_daily, option_cffex_sz50_daily, "2401");
macro_test_arg1!(test_mock_option_cffex_sz50_spot, option_cffex_sz50_spot, "2401");
macro_test_arg1!(test_mock_option_cffex_zz1000_spot, option_cffex_zz1000_spot, "2401");
macro_test_arg1!(test_mock_option_cffex_zz1000_daily, option_cffex_zz1000_daily, "2401");
macro_test_arg1!(test_mock_option_daily_stats_sse, option_daily_stats_sse, "2024-01-02");
macro_test_arg1!(test_mock_option_daily_stats_szse, option_daily_stats_szse, "2024-01-02");
macro_test_arg1!(test_mock_option_finance_minute, option_finance_minute, "10000001");
macro_test_arg1!(test_mock_option_finance_sse_underlying, option_finance_sse_underlying, "华夏上证50ETF期权");
macro_test_arg1!(test_mock_option_margin, option_margin, "SR");
macro_test_arg1!(test_mock_option_minute, option_minute, "10000001");
macro_test_arg1!(test_mock_option_risk_indicator, option_risk_indicator, "2024-01-02");
macro_test_arg1!(test_mock_option_sse_daily, option_sse_daily, "10000001");
macro_test_arg1!(test_mock_option_sse_greeks, option_sse_greeks, "10000001");
macro_test_arg1!(test_mock_option_sse_minute, option_sse_minute, "10000001");
macro_test_arg1!(test_mock_option_sse_spot_price, option_sse_spot_price, "10000001");
macro_test_arg1!(test_mock_option_sse_underlying_spot_price, option_sse_underlying_spot_price, "10000001");

// Two-arg option methods
macro_test_arg2!(test_mock_option_chain, option_chain, "SR", 100usize);
macro_test_arg2!(test_mock_option_commodity_contract_table, option_commodity_contract_table, "SR", "SR2401");
macro_test_arg2!(test_mock_option_finance_board, option_finance_board, "10000001", "202401");
macro_test_arg2!(test_mock_option_hist_czce, option_hist_czce, "SR2401", "2024-01-02");
macro_test_arg2!(test_mock_option_hist_dce, option_hist_dce, "m2401", "2024-01-02");
macro_test_arg2!(test_mock_option_hist_gfex, option_hist_gfex, "si2401", "2024-01-02");
macro_test_arg2!(test_mock_option_hist_shfe, option_hist_shfe, "cu2401", "2024-01-02");
macro_test_arg2!(test_mock_option_hist_yearly_czce, option_hist_yearly_czce, "SR2401", "2024");
macro_test_arg2!(test_mock_option_sse_list, option_sse_list, "10000001", "null");
macro_test_arg2!(test_mock_option_vol_gfex, option_vol_gfex, "si2401", "2024-01-02");
macro_test_arg2!(test_mock_option_vol_shfe, option_vol_shfe, "cu2401", "2024-01-02");

// Three-arg option methods
macro_test_arg3!(test_mock_option_lhb, option_lhb, "10000001", "认购", "20240102");
macro_test_arg3!(test_mock_option_sse_codes, option_sse_codes, "10000001", "2024-01-02", "510050");
macro_test_arg3!(test_mock_option_sse_expire_day, option_sse_expire_day, "2024-01-02", "10000001", "null");
