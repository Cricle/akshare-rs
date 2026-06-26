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
            let _ = client.$method().await;
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
            let _ = client.$method($arg).await;
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
            let _ = client.$method($arg1, $arg2).await;
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
            let _ = client.$method($arg1, $arg2, $arg3, $arg4).await;
        }
    };
}

macro_test!(test_futures_extra_symbol_mark, futures_symbol_mark);
macro_test!(test_futures_extra_hist_table, futures_hist_table);
macro_test!(test_futures_extra_global_spot, futures_global_spot);
macro_test!(test_futures_extra_display_main, futures_display_main);
macro_test!(
    test_futures_extra_fees_info_openctp,
    futures_fees_info_openctp
);
macro_test!(test_futures_extra_qhkc_tool_foreign, qhkc_tool_foreign);
macro_test!(test_futures_extra_qhkc_tool_gdp, qhkc_tool_gdp);

macro_test_arg1!(test_futures_extra_fees_info, futures_fees_info, "rb");
macro_test_arg1!(
    test_futures_extra_warehouse_sge,
    futures_warehouse_sge,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_warehouse_dce,
    futures_warehouse_dce,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_warehouse_czce,
    futures_warehouse_czce,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_match_main_contract,
    match_main_contract,
    "rb"
);
macro_test_arg1!(
    test_futures_extra_get_cffex_daily,
    get_cffex_daily,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_czce_daily,
    get_czce_daily,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_gfex_daily,
    get_gfex_daily,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_ine_daily,
    get_ine_daily,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_shfe_daily,
    get_shfe_daily,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_dce_receipt,
    get_dce_receipt,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_shfe_receipt,
    get_shfe_receipt,
    "2024-01-02"
);
macro_test_arg1!(test_futures_extra_get_rank_sum, get_rank_sum, "2024-01-02");
macro_test_arg1!(
    test_futures_extra_get_rank_sum_daily,
    get_rank_sum_daily,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_qhkc_fund_bs,
    get_qhkc_fund_bs,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_qhkc_fund_money_change,
    get_qhkc_fund_money_change,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_qhkc_fund_position,
    get_qhkc_fund_position,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_qhkc_index,
    get_qhkc_index,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_qhkc_index_profit_loss,
    get_qhkc_index_profit_loss,
    "2024-01-02"
);
macro_test_arg1!(
    test_futures_extra_get_qhkc_index_trend,
    get_qhkc_index_trend,
    "2024-01-02"
);

macro_test_arg2!(
    test_futures_extra_get_dce_daily,
    get_dce_daily,
    "2024-01-02",
    None::<&str>
);
macro_test_arg2!(
    test_futures_extra_get_cffex_rank_table,
    get_cffex_rank_table,
    "2024-01-02",
    "rb"
);
macro_test_arg2!(
    test_futures_extra_get_dce_rank_table,
    get_dce_rank_table,
    "2024-01-02",
    "rb"
);
macro_test_arg2!(
    test_futures_extra_get_shfe_rank_table,
    get_shfe_rank_table,
    "2024-01-02",
    "rb"
);
macro_test_arg2!(
    test_futures_extra_get_rank_table_czce,
    get_rank_table_czce,
    "2024-01-02",
    None::<&str>
);
macro_test_arg2!(
    test_futures_extra_get_receipt,
    get_receipt,
    "2024-01-02",
    None::<&str>
);
macro_test_arg2!(
    test_futures_extra_get_futures_daily,
    get_futures_daily,
    "2024-01-02",
    "CFFEX"
);

macro_test_arg4!(
    test_futures_extra_get_roll_yield_bar,
    get_roll_yield_bar,
    "2024-01-02",
    None::<&str>,
    None::<&str>,
    None::<&str>
);

macro_test_arg4!(
    test_futures_extra_get_roll_yield,
    get_roll_yield,
    "2024-01-02",
    "rb",
    None::<&str>,
    None::<&str>
);

#[tokio::test]
async fn test_futures_extra_foreign_commodity_realtime() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let _ = client.futures_foreign_commodity_realtime(&["CL"]).await;
}
