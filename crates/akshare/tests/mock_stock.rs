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

// === stock_other methods (previously uncovered) ===

// No-arg stock_other methods
macro_test!(test_mock_stock_a_all_pb, stock_a_all_pb);
macro_test!(test_mock_stock_a_below_net_asset_statistics, stock_a_below_net_asset_statistics);
macro_test!(test_mock_stock_account_statistics, stock_account_statistics);
macro_test!(test_mock_stock_a_congestion_lg, stock_a_congestion_lg);
macro_test!(test_mock_stock_a_gxl_lg, stock_a_gxl_lg);
macro_test!(test_mock_stock_a_high_low_statistics, stock_a_high_low_statistics);
macro_test!(test_mock_stock_a_ttm_lyr, stock_a_ttm_lyr);
macro_test!(test_mock_stock_buffett_index_lg, stock_buffett_index_lg);
macro_test_arg1!(test_mock_stock_cg_equity_mortgage, stock_cg_equity_mortgage, "600000");
macro_test_arg1!(test_mock_stock_cg_guarantee, stock_cg_guarantee, "600000");
macro_test_arg1!(test_mock_stock_cg_lawsuit, stock_cg_lawsuit, "600000");
macro_test_arg1!(test_mock_stock_classify, stock_classify, "600000");
macro_test_arg1!(test_mock_stock_concept_cons_futu, stock_concept_cons_futu, "600000");
macro_test_arg1!(test_mock_stock_dividend, stock_dividend, "600000");
macro_test!(test_mock_stock_ebs_lg, stock_ebs_lg);
macro_test_arg1!(test_mock_stock_esg_rate, stock_esg_rate, "600000");
macro_test!(test_mock_stock_gddh, stock_gddh);
macro_test_arg1!(test_mock_stock_index_pb_lg, stock_index_pb_lg, "000001");
macro_test_arg1!(test_mock_stock_index_pe_lg, stock_index_pe_lg, "000001");
macro_test!(test_mock_stock_ipo_benefit, stock_ipo_benefit);
macro_test!(test_mock_stock_ipo_summary, stock_ipo_summary);
macro_test!(test_mock_stock_market_activity_legu, stock_market_activity_legu);
macro_test!(test_mock_stock_market_pb_lg, stock_market_pb_lg);
macro_test!(test_mock_stock_market_pe_lg, stock_market_pe_lg);
macro_test!(test_mock_stock_new_gh, stock_new_gh);
macro_test!(test_mock_stock_new_ipo, stock_new_ipo);
macro_test_arg1!(test_mock_stock_news_main_cx, stock_news_main_cx, "600000");
macro_test_arg1!(test_mock_stock_profile, stock_profile, "600000");
macro_test_arg1!(test_mock_stock_qsjy, stock_qsjy, "2024-01-02");
macro_test_arg1!(test_mock_stock_report_disclosure, stock_report_disclosure, "600000");
macro_test_arg1!(test_mock_stock_report_fund_hold_detail, stock_report_fund_hold_detail, "600000");
macro_test_arg1!(test_mock_stock_research_report, stock_research_report, "600000");
macro_test_arg1!(test_mock_stock_sector_detail, stock_sector_detail, "600000");
macro_test!(test_mock_stock_sgt_reference_exchange_rate_sse, stock_sgt_reference_exchange_rate_sse);
macro_test!(test_mock_stock_sgt_reference_exchange_rate_szse, stock_sgt_reference_exchange_rate_szse);
macro_test!(test_mock_stock_sgt_settlement_exchange_rate_sse, stock_sgt_settlement_exchange_rate_sse);
macro_test!(test_mock_stock_sgt_settlement_exchange_rate_szse, stock_sgt_settlement_exchange_rate_szse);
macro_test_arg1!(test_mock_stock_share_change, stock_share_change, "600000");
macro_test_arg1!(test_mock_stock_sns_sseinfo, stock_sns_sseinfo, "600000");
macro_test_arg1!(test_mock_stock_sse_deal_daily, stock_sse_deal_daily, "2024-01-02");
macro_test!(test_mock_stock_sy_hy, stock_sy_hy);
macro_test_arg1!(test_mock_stock_szse_area_summary, stock_szse_area_summary, "2024-01-02");
macro_test_arg1!(test_mock_stock_szse_sector_summary, stock_szse_sector_summary, "2024-01-02");
macro_test_arg1!(test_mock_stock_szse_summary, stock_szse_summary, "2024-01-02");
macro_test_arg1!(test_mock_stock_tfp, stock_tfp, "2024-01-02");
macro_test_arg1!(test_mock_stock_value, stock_value, "600000");
macro_test_arg1!(test_mock_stock_xgsr, stock_xgsr, "600000");
macro_test_arg1!(test_mock_stock_yzxdr, stock_yzxdr, "2024-01-02");
macro_test_arg1!(test_mock_stock_zcfz_bj, stock_zcfz_bj, "600000");
macro_test_arg2!(test_mock_stock_zdhtmx, stock_zdhtmx, "2024-01-01", "2024-01-31");

// Single-arg stock_other methods
macro_test_arg1!(test_mock_stock_a_code_to_symbol, stock_a_code_to_symbol, "600000");
macro_test_arg1!(test_mock_stock_allotment, stock_allotment, "600000");
macro_test_arg2!(test_mock_stock_cyq, stock_cyq, "600000", "qfq");
macro_test_arg1!(test_mock_stock_fhps_detail_ths, stock_fhps_detail_ths, "600000");
macro_test_arg1!(test_mock_stock_price_js, stock_price_js, "600000");
macro_test_arg1!(test_mock_stock_share_hold_change_bse, stock_share_hold_change_bse, "2024-01-01");
macro_test_arg1!(test_mock_stock_share_hold_change_sse, stock_share_hold_change_sse, "2024-01-01");
macro_test_arg1!(test_mock_stock_share_hold_change_szse, stock_share_hold_change_szse, "2024-01-01");

// Single-arg stock_other methods (delisted reports)
macro_test_arg1!(test_mock_stock_balance_sheet_by_report_delisted, stock_balance_sheet_by_report_delisted, "600000");
macro_test_arg1!(test_mock_stock_cash_flow_sheet_by_report_delisted, stock_cash_flow_sheet_by_report_delisted, "600000");
macro_test_arg1!(test_mock_stock_profit_sheet_by_report_delisted, stock_profit_sheet_by_report_delisted, "600000");
