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

macro_rules! macro_test_arg5 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = wiremock::MockServer::start().await;
            mount_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg1, $arg2, $arg3, $arg4, $arg5).await;
            let _ = result;
        }
    };
}

// No-arg fund methods
macro_test!(test_mock_fund_aum, fund_aum);
macro_test!(test_mock_fund_aum_trend, fund_aum_trend);
macro_test!(test_mock_fund_balance_position_lg, fund_balance_position_lg);
macro_test!(test_mock_fund_exchange_rank, fund_exchange_rank);
macro_test!(test_mock_fund_fh_rank, fund_fh_rank);
macro_test!(test_mock_fund_financial_fund_daily, fund_financial_fund_daily);
macro_test!(test_mock_fund_graded_fund_daily, fund_graded_fund_daily);
macro_test!(test_mock_fund_hk_rank, fund_hk_rank);
macro_test!(test_mock_fund_hold_structure, fund_hold_structure);
macro_test!(test_mock_fund_lcx_rank, fund_lcx_rank);
macro_test!(test_mock_fund_linghuo_position_lg, fund_linghuo_position_lg);
macro_test!(test_mock_fund_lof_spot, fund_lof_spot);
macro_test!(test_mock_fund_manager, fund_manager);
macro_test!(test_mock_fund_money_fund_daily, fund_money_fund_daily);
macro_test!(test_mock_fund_money_rank, fund_money_rank);
macro_test!(test_mock_fund_name, fund_name);
macro_test!(test_mock_fund_new_found_em, fund_new_found_em);
macro_test!(test_mock_fund_open_fund_daily, fund_open_fund_daily);
macro_test!(test_mock_fund_position_est_lg, fund_position_est_lg);
macro_test!(test_mock_fund_rating_all, fund_rating_all);
macro_test!(test_mock_fund_rating_jiashi, fund_rating_jiashi);
macro_test!(test_mock_fund_rating_tiantian, fund_rating_tiantian);
macro_test!(test_mock_fund_rating_zs, fund_rating_zs);
macro_test!(test_mock_fund_report_asset_allocation, fund_report_asset_allocation);
macro_test!(test_mock_fund_scale_change, fund_scale_change);
macro_test!(test_mock_fund_scale_close, fund_scale_close);
macro_test!(test_mock_fund_scale_money, fund_scale_money);
macro_test!(test_mock_fund_scale_structured, fund_scale_structured);
macro_test!(test_mock_fund_stock_position_lg, fund_stock_position_lg);
macro_test!(test_mock_fund_etf_scale_szse, fund_etf_scale_szse);
macro_test!(test_mock_fund_etf_spot_em, fund_etf_spot_em);

// Single-arg fund methods
macro_test_arg1!(test_mock_fund_aum_hist, fund_aum_hist, "2024");
macro_test_arg1!(test_mock_fund_cf, fund_cf, "2024");
macro_test_arg1!(test_mock_fund_etf_category_sina, fund_etf_category_sina, "封闭式基金");
macro_test_arg1!(test_mock_fund_etf_dividend, fund_etf_dividend, "510300");
macro_test_arg1!(test_mock_fund_etf_fund_daily, fund_etf_fund_daily, 100usize);
macro_test_arg1!(test_mock_fund_etf_hist_sina, fund_etf_hist_sina, "510300");
macro_test_arg1!(test_mock_fund_etf_scale_sse, fund_etf_scale_sse, "2024-01-02");
macro_test_arg1!(test_mock_fund_etf_spot_ths, fund_etf_spot_ths, "2024-01-02");
macro_test_arg1!(test_mock_fund_fee, fund_fee, 100usize);
macro_test_arg1!(test_mock_fund_fh, fund_fh, "2024");
macro_test_arg1!(test_mock_fund_financial_fund_info, fund_financial_fund_info, "000001");
macro_test_arg1!(test_mock_fund_graded, fund_graded, 100usize);
macro_test_arg1!(test_mock_fund_graded_fund_info, fund_graded_fund_info, "000001");
macro_test_arg1!(test_mock_fund_individual_analysis_xq, fund_individual_analysis_xq, "000001");
macro_test_arg1!(test_mock_fund_individual_basic_info_xq, fund_individual_basic_info_xq, "000001");
macro_test_arg1!(test_mock_fund_individual_achievement_xq, fund_individual_achievement_xq, "000001");
macro_test_arg1!(test_mock_fund_individual_detail_info_xq, fund_individual_detail_info_xq, "000001");
macro_test_arg1!(test_mock_fund_individual_profit_probability_xq, fund_individual_profit_probability_xq, "000001");
macro_test_arg1!(test_mock_fund_info, fund_info, 100usize);
macro_test_arg1!(test_mock_fund_lof, fund_lof, 100usize);
macro_test_arg1!(test_mock_fund_lof_list, fund_lof_list, 100usize);
macro_test_arg1!(test_mock_fund_money_fund_info, fund_money_fund_info, "000001");
macro_test_arg1!(test_mock_fund_money_market, fund_money_market, 100usize);
macro_test_arg1!(test_mock_fund_new_found_ths, fund_new_found_ths, "000001");
macro_test_arg1!(test_mock_fund_open_end_daily, fund_open_end_daily, 100usize);
macro_test_arg1!(test_mock_fund_overview, fund_overview, 100usize);
macro_test_arg1!(test_mock_fund_portfolio_asset_allocation, fund_portfolio_asset_allocation, "000001");
macro_test_arg1!(test_mock_fund_portfolio_bond_hold, fund_portfolio_bond_hold, "000001");
macro_test_arg1!(test_mock_fund_portfolio_industry_allocation, fund_portfolio_industry_allocation, "000001");
macro_test_arg1!(test_mock_fund_position_hist_lg, fund_position_hist_lg, "000001");
macro_test_arg1!(test_mock_fund_position_lg, fund_position_lg, "000001");
macro_test_arg1!(test_mock_fund_purchase, fund_purchase, 100usize);
macro_test_arg1!(test_mock_fund_rating, fund_rating, 100usize);
macro_test_arg1!(test_mock_fund_rating_ja, fund_rating_ja, "2024-01-02");
macro_test_arg1!(test_mock_fund_rating_sh, fund_rating_sh, "2024-01-02");
macro_test_arg1!(test_mock_fund_report, fund_report, "000001");
macro_test_arg1!(test_mock_fund_report_half_year, fund_report_half_year, "000001");
macro_test_arg1!(test_mock_fund_report_industry_allocation, fund_report_industry_allocation, "000001");
macro_test_arg1!(test_mock_fund_report_quarter, fund_report_quarter, "000001");
macro_test_arg1!(test_mock_fund_report_stock, fund_report_stock, "2024-01-02");
macro_test_arg1!(test_mock_fund_scale_open, fund_scale_open, "股票型基金");
macro_test_arg1!(test_mock_fund_value_estimation, fund_value_estimation, "全部");
macro_test_arg1!(test_mock_fund_xueqiu_achievement, fund_xueqiu_achievement, "000001");
macro_test_arg1!(test_mock_fund_xueqiu_info, fund_xueqiu_info, "000001");
macro_test_arg1!(test_mock_fund_announcement_dividend, fund_announcement_dividend, "000001");
macro_test_arg1!(test_mock_fund_announcement_report, fund_announcement_report, "000001");
macro_test_arg1!(test_mock_fund_announcement_personnel, fund_announcement_personnel, "000001");
macro_test_arg1!(test_mock_qdii_a_index_jsl, qdii_a_index_jsl, "");
macro_test_arg1!(test_mock_qdii_e_index_jsl, qdii_e_index_jsl, "");
macro_test_arg1!(test_mock_qdii_e_comm_jsl, qdii_e_comm_jsl, "");

// Two-arg fund methods
macro_test_arg2!(test_mock_fund_etf_hist, fund_etf_hist, "510300", 100usize);
macro_test_arg2!(test_mock_fund_etf_category_ths, fund_etf_category_ths, "ETF", "2024-01-02");
macro_test_arg2!(test_mock_fund_hk_fund_hist, fund_hk_fund_hist, "000001", "分红送配详情");
macro_test_arg2!(test_mock_fund_individual_detail_hold_xq, fund_individual_detail_hold_xq, "000001", "20240101");
macro_test_arg2!(test_mock_fund_lof_hist, fund_lof_hist, "160001", 100usize);
macro_test_arg2!(test_mock_fund_open_end_nav, fund_open_end_nav, "000001", 100usize);
macro_test_arg2!(test_mock_fund_open_fund_rank, fund_open_fund_rank, "000001", 100usize);
macro_test_arg2!(test_mock_fund_portfolio_change, fund_portfolio_change, "000001", "累计收益率走势");
macro_test_arg2!(test_mock_fund_portfolio_hold, fund_portfolio_hold, "000001", "2024-01-02");

// Three-arg fund methods
macro_test_arg3!(test_mock_fund_etf_fund_info, fund_etf_fund_info, "510300", "20240101", "20241231");
macro_test_arg3!(test_mock_fund_info_index, fund_info_index, "000001", "单位净值走势", 100usize);
macro_test_arg3!(test_mock_fund_scale_daily, fund_scale_daily, "2024-01-01", "2024-12-31", "000001");

// Four-arg fund methods
macro_test_arg4!(test_mock_fund_open_fund_info, fund_open_fund_info, "000001", "2024-01-01", "2024-12-31", "单位净值走势");

// Five-arg fund methods
macro_test_arg5!(test_mock_fund_etf_hist_em, fund_etf_hist_em, "510300", "daily", "2024-01-01", "2024-12-31", "qfq");
macro_test_arg5!(test_mock_fund_etf_hist_min, fund_etf_hist_min, "510300", "5", "2024-01-01 09:30:00", "2024-01-01 15:00:00", "qfq");
macro_test_arg5!(test_mock_fund_lof_hist_em, fund_lof_hist_em, "160001", "daily", "2024-01-01", "2024-12-31", "qfq");
macro_test_arg5!(test_mock_fund_lof_hist_min, fund_lof_hist_min, "160001", "5", "2024-01-01 09:30:00", "2024-01-01 15:00:00", "qfq");
