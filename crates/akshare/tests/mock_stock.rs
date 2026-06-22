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

// No-arg functions
macro_test!(test_mock_get_us_stock_name, get_us_stock_name);
macro_test!(test_mock_stock_info_bj_name_code, stock_info_bj_name_code);

// Single-arg functions
macro_test_arg1!(test_mock_hk_financial, hk_financial, "00593");
macro_test_arg1!(
    test_mock_hk_market_cap_from_tencent,
    hk_market_cap_from_tencent,
    "00593"
);
macro_test_arg2!(
    test_mock_stock_financial_hk_analysis_indicator_em_typed,
    stock_financial_hk_analysis_indicator_em_typed,
    "00593",
    "年度"
);
macro_test_arg2!(
    test_mock_stock_financial_hk_balance_sheet_typed,
    stock_financial_hk_balance_sheet_typed,
    "00593",
    "年度"
);
macro_test_arg2!(
    test_mock_stock_financial_hk_cashflow_sheet_typed,
    stock_financial_hk_cashflow_sheet_typed,
    "00593",
    "年度"
);
macro_test_arg2!(
    test_mock_stock_financial_hk_income_sheet_typed,
    stock_financial_hk_income_sheet_typed,
    "00593",
    "年度"
);
macro_test_arg2!(
    test_mock_stock_financial_us_analysis_indicator_em_typed,
    stock_financial_us_analysis_indicator_em_typed,
    "AAPL",
    "年度"
);
macro_test_arg2!(
    test_mock_stock_financial_us_balance_sheet_typed,
    stock_financial_us_balance_sheet_typed,
    "AAPL",
    "年度"
);
macro_test_arg2!(
    test_mock_stock_financial_us_cashflow_sheet_typed,
    stock_financial_us_cashflow_sheet_typed,
    "AAPL",
    "年度"
);
macro_test_arg2!(
    test_mock_stock_financial_us_income_sheet_typed,
    stock_financial_us_income_sheet_typed,
    "AAPL",
    "年度"
);
macro_test_arg1!(
    test_mock_stock_individual_info_em_by_secid,
    stock_individual_info_em_by_secid,
    "1.600000"
);
macro_test_arg1!(
    test_mock_stock_info_by_secid,
    stock_info_by_secid,
    "1.600000"
);
macro_test_arg1!(
    test_mock_stock_info_sh_name_code,
    stock_info_sh_name_code,
    "600000"
);
macro_test_arg1!(
    test_mock_stock_info_sz_name_code,
    stock_info_sz_name_code,
    "000001"
);
macro_test_arg1!(
    test_mock_stock_news_em_by_name,
    stock_news_em_by_name,
    "浦发银行"
);
macro_test_arg1!(test_mock_stock_news_em_hk, stock_news_em_hk, "00593");
macro_test_arg1!(test_mock_stock_news_em_us, stock_news_em_us, "AAPL");
macro_test_arg1!(
    test_mock_stock_us_dividend_payout_em,
    stock_us_dividend_payout,
    "AAPL"
);
macro_test_arg1!(
    test_mock_stock_us_financial_indicator_em,
    stock_us_financial_indicator,
    "AAPL"
);
macro_test_arg1!(
    test_mock_stock_us_growth_comparison_em,
    stock_us_growth_comparison,
    "AAPL"
);
macro_test!(test_mock_stock_us_gxl_lg, stock_us_gxl_lg);
macro_test_arg1!(
    test_mock_stock_us_hot_keyword_em,
    stock_us_hot_keyword,
    "AAPL"
);
macro_test_arg1!(
    test_mock_stock_us_hot_rank_detail_em,
    stock_us_hot_rank_detail,
    "AAPL"
);
macro_test_arg1!(
    test_mock_stock_us_hot_rank_detail_realtime_em,
    stock_us_hot_rank_detail_realtime,
    "AAPL"
);
macro_test!(test_mock_stock_us_hot_rank_em, stock_us_hot_rank);
macro_test_arg1!(
    test_mock_stock_us_hot_rank_latest_em,
    stock_us_hot_rank_latest,
    "AAPL"
);
macro_test_arg1!(
    test_mock_stock_us_index_daily_em,
    stock_us_index_daily_em,
    ".INX"
);
macro_test_arg1!(
    test_mock_stock_us_index_daily_sina,
    stock_us_index_daily_sina,
    ".INX"
);
macro_test!(test_mock_stock_us_index_spot_em, stock_us_index_spot_em);
macro_test!(test_mock_stock_us_index_spot_sina, stock_us_index_spot_sina);
macro_test_arg1!(
    test_mock_stock_us_scale_comparison_em,
    stock_us_scale_comparison,
    "AAPL"
);
macro_test_arg1!(
    test_mock_stock_us_valuation_comparison_em,
    stock_us_valuation_comparison,
    "AAPL"
);
macro_test_arg1!(
    test_mock_stock_zh_a_dividend_payout_em,
    stock_zh_a_dividend_payout,
    "600000"
);
macro_test_arg1!(
    test_mock_stock_zh_a_financial_indicator_em,
    stock_zh_a_financial_indicator,
    "600000"
);
macro_test_arg1!(
    test_mock_us_market_cap_from_sina,
    us_market_cap_from,
    "AAPL"
);
// us_stock_industry returns a tuple, not a Result
#[tokio::test]
async fn test_mock_us_stock_industry() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let _result = client.us_stock_industry("AAPL").await;
}
macro_test_arg1!(test_mock_us_stock_key_stats, us_stock_key_stats, "AAPL");
macro_test_arg1!(test_mock_us_stock_profile, us_stock_profile, "AAPL");

// Multi-arg functions
#[tokio::test]
async fn test_mock_stock_zh_a_hist() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client
        .stock_zh_a_hist("600000", "daily", "qfq", "2024-01-01", "2024-01-31")
        .await;
    result.unwrap();
}

#[tokio::test]
async fn test_mock_stock_zh_a_hist_min_em() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client
        .stock_zh_a_hist_min("600000", "1", "qfq", "2024-01-01", "2024-01-31")
        .await;
    result.unwrap();
}
