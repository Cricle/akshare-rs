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

macro_rules! macro_test_arg4 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = wiremock::MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg1, $arg2, $arg3, $arg4).await;
            let _ = result;
        }
    };
}

// No-arg bond methods
macro_test!(test_mock_bond_available_index_cbond, bond_available_index_cbond);
macro_test!(test_mock_bond_cb_adj_logs_jsl, bond_cb_adj_logs_jsl);
macro_test!(test_mock_bond_cb_index_jsl, bond_cb_index_jsl);
macro_test!(test_mock_bond_cb_jsl, bond_cb_jsl);
macro_test!(test_mock_bond_cb_redeem_jsl, bond_cb_redeem_jsl);
macro_test!(test_mock_bond_cov_stock_issue, bond_cov_stock_issue);
macro_test!(test_mock_bond_spot_deal, bond_spot_deal);
macro_test!(test_mock_bond_spot_quote, bond_spot_quote);
macro_test!(test_mock_bond_zh_cov_info_ths, bond_zh_cov_info_ths);

// Single-arg bond methods
macro_test_arg1!(test_mock_bond_cash_summary, bond_cash_summary, "20240102");
macro_test_arg1!(test_mock_bond_cb_profile, bond_cb_profile, "110000");
macro_test_arg1!(test_mock_bond_cb_summary, bond_cb_summary, "110000");
macro_test_arg1!(test_mock_bond_convertible_list, bond_convertible_list, 100usize);
macro_test_arg1!(test_mock_bond_corporate_yields, bond_corporate_yields, 100usize);
macro_test_arg1!(test_mock_bond_cov_comparison, bond_cov_comparison, 100usize);
macro_test_arg1!(test_mock_bond_deal_summary, bond_deal_summary, "20240102");
macro_test_arg1!(test_mock_bond_debt_nafmii, bond_debt_nafmii, 1u32);
macro_test_arg1!(test_mock_bond_gb_us, bond_gb_us, "110000");
macro_test_arg1!(test_mock_bond_gb_zh, bond_gb_zh, "110000");
macro_test_arg1!(test_mock_bond_sh_buy_back, bond_sh_buy_back, 100usize);
macro_test_arg1!(test_mock_bond_sz_buy_back, bond_sz_buy_back, 100usize);
macro_test_arg1!(test_mock_bond_spot_rates, bond_spot_rates, 100usize);
macro_test_arg1!(test_mock_bond_zh_cov, bond_zh_cov, 100usize);
macro_test_arg1!(test_mock_bond_zh_cov_value_analysis, bond_zh_cov_value_analysis, "110000");
macro_test_arg1!(test_mock_bond_zh_hs_cov_daily, bond_zh_hs_cov_daily, "110000");
macro_test_arg1!(test_mock_bond_zh_hs_cov_pre_min, bond_zh_hs_cov_pre_min, "110000");
macro_test_arg1!(test_mock_bond_zh_hs_cov_spot, bond_zh_hs_cov_spot, "110000");
macro_test_arg1!(test_mock_bond_zh_hs_daily, bond_zh_hs_daily, "110000");
macro_test_arg1!(test_mock_bond_zh_hs_spot, bond_zh_hs_spot, 100usize);
macro_test_arg1!(test_mock_bond_zh_us_rate, bond_zh_us_rate, "2024-01-01");
macro_test_arg1!(test_mock_macro_china_bond_public, macro_china_bond_public, 100u32);

// Two-arg bond methods
macro_test_arg2!(test_mock_bond_buy_back_hist, bond_buy_back_hist, "110000", 100usize);
macro_test_arg2!(test_mock_bond_china_yield, bond_china_yield, "2024-01-01", "2024-12-31");
macro_test_arg2!(test_mock_bond_composite_index_cbond, bond_composite_index_cbond, "中债综合指数", "day");
macro_test_arg2!(test_mock_bond_convertible_hist, bond_convertible_hist, "110000", 100usize);
macro_test_arg2!(test_mock_bond_corporate_issue, bond_corporate_issue, "2024-01-01", "2024-12-31");
macro_test_arg2!(test_mock_bond_cov_issue, bond_cov_issue, "2024-01-01", "2024-12-31");
macro_test_arg2!(test_mock_bond_index_general_cbond, bond_index_general_cbond, "中债综合指数", "day");
macro_test_arg2!(test_mock_bond_local_gov_issue, bond_local_gov_issue, "2024-01-01", "2024-12-31");
macro_test_arg2!(test_mock_bond_local_government_issue, bond_local_government_issue, "2024-01-01", "2024-12-31");
macro_test_arg2!(test_mock_bond_new_composite_index_cbond, bond_new_composite_index_cbond, "中债综合指数", "day");
macro_test_arg2!(test_mock_bond_treasure_issue, bond_treasure_issue, "2024-01-01", "2024-12-31");
macro_test_arg2!(test_mock_bond_treasury_index_cbond, bond_treasury_index_cbond, "国债总指数", "day");
macro_test_arg2!(test_mock_bond_zh_cov_info, bond_zh_cov_info, "110000", "基本信息");
macro_test_arg2!(test_mock_bond_zh_hs_cov_min, bond_zh_hs_cov_min, "110000", "5");
macro_test_arg2!(test_mock_macro_china_swap_rate, macro_china_swap_rate, "2024-01-01", "2024-12-31");

// Four-arg bond methods
macro_test_arg4!(test_mock_bond_china_close_return, bond_china_close_return, "110000", "1", "2024-01-01", "2024-12-31");
