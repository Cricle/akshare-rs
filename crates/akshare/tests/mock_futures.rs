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

// No-arg futures methods
macro_test!(test_mock_futures_display_main, futures_display_main);
macro_test!(test_mock_futures_fees_info_openctp, futures_fees_info_openctp);
macro_test!(test_mock_futures_global_spot, futures_global_spot);
macro_test!(test_mock_futures_hist_table, futures_hist_table);
macro_test!(test_mock_futures_rule_em, futures_rule_em);
macro_test!(test_mock_futures_contract_info_dce, futures_contract_info_dce);
macro_test!(test_mock_futures_contract_info_gfex, futures_contract_info_gfex);
macro_test!(test_mock_futures_symbol_mark, futures_symbol_mark);

// Single-arg futures methods
macro_test_arg1!(test_mock_futures_comex_inventory, futures_comex_inventory, "CL");
macro_test_arg1!(test_mock_futures_comm_info, futures_comm_info, "rb");
macro_test_arg1!(test_mock_futures_comm_js, futures_comm_js, "2024-01-02");
macro_test_arg1!(test_mock_futures_cffex_position_rank, futures_cffex_position_rank, "2024-01-02");
macro_test_arg1!(test_mock_futures_contract_detail, futures_contract_detail, "rb2401");
macro_test_arg1!(test_mock_futures_contract_detail_em, futures_contract_detail_em, "rb2401");
macro_test_arg1!(test_mock_futures_contract_detail_sina, futures_contract_detail_sina, "rb2401");
macro_test_arg1!(test_mock_futures_contract_info_cffex, futures_contract_info_cffex, "2024-01-02");
macro_test_arg1!(test_mock_futures_contract_info_czce, futures_contract_info_czce, "2024-01-02");
macro_test_arg1!(test_mock_futures_contract_info_ine, futures_contract_info_ine, "2024-01-02");
macro_test_arg1!(test_mock_futures_contract_info_shfe, futures_contract_info_shfe, "2024-01-02");
macro_test_arg1!(test_mock_futures_czce_position_rank, futures_czce_position_rank, "2024-01-02");
macro_test_arg1!(test_mock_futures_daily_cffex, futures_daily_cffex, "2024-01-02");
macro_test_arg1!(test_mock_futures_daily_czce, futures_daily_czce, "2024-01-02");
macro_test_arg1!(test_mock_futures_daily_dce, futures_daily_dce, "2024-01-02");
macro_test_arg1!(test_mock_futures_daily_gfex, futures_daily_gfex, "2024-01-02");
macro_test_arg1!(test_mock_futures_daily_ine, futures_daily_ine, "2024-01-02");
macro_test_arg1!(test_mock_futures_daily_shfe, futures_daily_shfe, "2024-01-02");
macro_test_arg1!(test_mock_futures_dce_position_rank, futures_dce_position_rank, "2024-01-02");
macro_test_arg1!(test_mock_futures_delivery_czce, futures_delivery_czce, "2024-01-02");
macro_test_arg1!(test_mock_futures_delivery_dce, futures_delivery_dce, "2024-01-02");
macro_test_arg1!(test_mock_futures_delivery_match_czce, futures_delivery_match_czce, "2024-01-02");
macro_test_arg1!(test_mock_futures_delivery_match_dce, futures_delivery_match_dce, "2024-01-02");
macro_test_arg1!(test_mock_futures_delivery_shfe, futures_delivery_shfe, "2024-01-02");
macro_test_arg1!(test_mock_futures_foreign_commodity_realtime_str, futures_foreign_commodity_realtime_str, "CL");
macro_test_arg1!(test_mock_futures_foreign_detail, futures_foreign_detail, "CL");
macro_test_arg1!(test_mock_futures_foreign_hist, futures_foreign_hist, "CL");
macro_test_arg1!(test_mock_futures_gfex_position_rank, futures_gfex_position_rank, "2024-01-02");
macro_test_arg1!(test_mock_futures_gfex_warehouse_receipt, futures_gfex_warehouse_receipt, "2024-01-02");
macro_test_arg1!(test_mock_futures_global_hist, futures_global_hist, "CL");
macro_test_arg1!(test_mock_futures_hist_daily_cffex, futures_hist_daily_cffex, "2024-01-02");
macro_test_arg1!(test_mock_futures_hog_core, futures_hog_core, "LH");
macro_test_arg1!(test_mock_futures_hog_cost, futures_hog_cost, "LH");
macro_test_arg1!(test_mock_futures_hog_supply, futures_hog_supply, "LH");
macro_test_arg1!(test_mock_futures_index_ccidx, futures_index_ccidx, "IM");
macro_test_arg1!(test_mock_futures_inventory, futures_inventory, "rb");
macro_test_arg1!(test_mock_futures_inventory_99, futures_inventory_99, "rb");
macro_test_arg1!(test_mock_futures_news_shmet, futures_news_shmet, "铜");
macro_test_arg1!(test_mock_futures_roll_yield_bar, futures_roll_yield_bar, "2024-01-02");
macro_test_arg1!(test_mock_futures_rule, futures_rule, "2024-01-02");
macro_test_arg1!(test_mock_futures_rule_gtja, futures_rule_gtja, "2024-01-02");
macro_test_arg1!(test_mock_futures_settle_cffex, futures_settle_cffex, "2024-01-02");
macro_test_arg1!(test_mock_futures_settle_czce, futures_settle_czce, "2024-01-02");
macro_test_arg1!(test_mock_futures_settle_gfex, futures_settle_gfex, "2024-01-02");
macro_test_arg1!(test_mock_futures_settle_ine, futures_settle_ine, "2024-01-02");
macro_test_arg1!(test_mock_futures_settle_shfe, futures_settle_shfe, "2024-01-02");
macro_test_arg1!(test_mock_futures_settlement_price_sgx, futures_settlement_price_sgx, "2024-01-02");
macro_test_arg1!(test_mock_futures_shfe_position_rank, futures_shfe_position_rank, "2024-01-02");
macro_test_arg1!(test_mock_futures_shfe_warehouse_receipt, futures_shfe_warehouse_receipt, "2024-01-02");
macro_test_arg1!(test_mock_futures_spot_price, futures_spot_price, "2024-01-02");
macro_test_arg1!(test_mock_futures_spot_price_previous, futures_spot_price_previous, "2024-01-02");
macro_test_arg1!(test_mock_futures_spot_prices, futures_spot_prices, 100usize);
macro_test_arg1!(test_mock_futures_spot_stock, futures_spot_stock, "2024-01-02");
macro_test_arg1!(test_mock_futures_spot_stock_em, futures_spot_stock_em, "农产品");
macro_test_arg1!(test_mock_futures_stock_shfe_js, futures_stock_shfe_js, "rb");
macro_test_arg1!(test_mock_futures_to_spot_czce, futures_to_spot_czce, "2024-01-02");
macro_test_arg1!(test_mock_futures_to_spot_dce, futures_to_spot_dce, "2024-01-02");
macro_test_arg1!(test_mock_futures_to_spot_shfe, futures_to_spot_shfe, "2024-01-02");
macro_test_arg1!(test_mock_futures_warehouse_receipt_czce, futures_warehouse_receipt_czce, "2024-01-02");
macro_test_arg1!(test_mock_futures_warehouse_receipt_dce, futures_warehouse_receipt_dce, "2024-01-02");
macro_test_arg1!(test_mock_futures_zh_daily, futures_zh_daily, "rb2401");
macro_test_arg1!(test_mock_futures_zh_realtime, futures_zh_realtime, "rb2401");

// Two-arg futures methods
macro_test_arg2!(test_mock_futures_main, futures_main, "rb2401", 100usize);
macro_test_arg2!(test_mock_futures_settle, futures_settle, "2024-01-02", "cffex");
macro_test_arg2!(test_mock_futures_spot_price_daily, futures_spot_price_daily, "2024-01-01", "2024-01-02");
macro_test_arg2!(test_mock_futures_spot_sys, futures_spot_sys, "LH", "price");
macro_test_arg2!(test_mock_futures_zh_minute, futures_zh_minute, "rb2401", "5");
macro_test_arg2!(test_mock_futures_zh_spot, futures_zh_spot, "rb2401", "cffex");
macro_test_arg2!(test_mock_futures_dce_position_rank_other, futures_dce_position_rank_other, "2024-01-02", "rb2401");

// Three-arg futures methods
macro_test_arg3!(test_mock_futures_hold_pos, futures_hold_pos, "rb", "rb2401", "2024-01-02");
macro_test_arg3!(test_mock_futures_main_sina_derivative, futures_main_sina_derivative, "rb0", "20240101", "20240102");

// Four-arg futures methods
macro_test_arg4!(test_mock_futures_hist, futures_hist, "rb2401", "daily", "2024-01-01", "2024-12-31");

// Special: takes &[&str]
#[tokio::test]
async fn test_mock_futures_foreign_commodity_realtime() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.futures_foreign_commodity_realtime(&["CL"]).await;
    let _ = result;
}
