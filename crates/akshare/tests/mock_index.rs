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

// No-arg index methods
macro_test!(test_mock_index_all_cni, index_all_cni);
macro_test!(test_mock_index_code_id_map, index_code_id_map);
macro_test!(test_mock_index_csindex_all, index_csindex_all);
macro_test!(
    test_mock_index_global_name_table_yahoo,
    index_global_name_table_yahoo
);
macro_test!(test_mock_index_global_spot, index_global_spot);
macro_test!(test_mock_index_hk_spot_em, index_hk_spot_em);
macro_test!(test_mock_index_hk_spot_sina, index_hk_spot_sina);
macro_test!(test_mock_index_hog_spot_price, index_hog_spot_price);
macro_test!(
    test_mock_index_inner_quote_sugar_msweet,
    index_inner_quote_sugar_msweet
);
macro_test!(
    test_mock_index_news_sentiment_scope,
    index_news_sentiment_scope
);
macro_test!(
    test_mock_index_outer_quote_sugar_msweet,
    index_outer_quote_sugar_msweet
);
macro_test!(test_mock_index_stock_info, index_stock_info);
macro_test!(test_mock_index_stock_zh_spot_sina, index_stock_zh_spot_sina);
macro_test!(test_mock_index_sugar_msweet, index_sugar_msweet);
macro_test!(test_mock_sw_index_first_info, sw_index_first_info);
macro_test!(test_mock_sw_index_list, sw_index_list);
macro_test!(test_mock_sw_index_second_info, sw_index_second_info);

// Single-arg index methods
macro_test_arg1!(test_mock_drewry_wci_index, drewry_wci_index, "2024-01-02");
macro_test_arg1!(test_mock_hf_sp_500, hf_sp_500, "2024");
macro_test_arg1!(test_mock_index_component_sw, index_component_sw, "801010");
macro_test_arg1!(test_mock_index_detail_cni, index_detail_cni, "000001");
macro_test_arg1!(
    test_mock_index_detail_hist_adjust_cni,
    index_detail_hist_adjust_cni,
    "000001"
);
macro_test_arg1!(test_mock_index_eri, index_eri, "000001");
macro_test_arg1!(
    test_mock_index_global_hist_sina,
    index_global_hist_sina,
    ".INX"
);
macro_test_arg1!(test_mock_index_kq_fashion, index_kq_fashion, "柯桥纺织");
macro_test_arg1!(test_mock_index_kq_fz, index_kq_fz, "柯桥纺织");
macro_test_arg1!(test_mock_index_min_sw, index_min_sw, "800000");
macro_test_arg1!(test_mock_index_price_cflp, index_price_cflp, "2024-01-02");
macro_test_arg1!(
    test_mock_index_realtime_fund_sw,
    index_realtime_fund_sw,
    "801010"
);
macro_test_arg1!(test_mock_index_realtime_sw, index_realtime_sw, "801010");
macro_test_arg1!(test_mock_index_stock_cons, index_stock_cons, "000001");
macro_test_arg1!(
    test_mock_index_stock_cons_csindex,
    index_stock_cons_csindex,
    "000300"
);
macro_test_arg1!(
    test_mock_index_stock_cons_sina,
    index_stock_cons_sina,
    "000001"
);
macro_test_arg1!(
    test_mock_index_stock_cons_weight_csindex,
    index_stock_cons_weight_csindex,
    "000300"
);
macro_test_arg1!(
    test_mock_index_stock_zh_spot_em,
    index_stock_zh_spot_em,
    "上证系列指数"
);
macro_test_arg1!(test_mock_index_us_stock, index_us_stock, ".INX");
macro_test_arg1!(test_mock_index_volume_cflp, index_volume_cflp, "2024-01-02");
macro_test_arg1!(test_mock_index_yw, index_yw, "000001");
macro_test_arg1!(
    test_mock_index_analysis_week_month_sw,
    index_analysis_week_month_sw,
    "801010"
);
macro_test_arg1!(test_mock_spot_goods, spot_goods, "铜");
macro_test_arg1!(
    test_mock_stock_zh_index_daily,
    stock_zh_index_daily,
    "000001"
);
macro_test_arg1!(test_mock_sw_index_third_cons, sw_index_third_cons, "801010");
macro_test_arg1!(test_mock_sw_index_third_info, sw_index_third_info, "801010");

// Two-arg index methods
macro_test_arg2!(
    test_mock_index_zh_a_hist,
    index_zh_a_hist,
    "000001",
    "daily"
);
macro_test_arg2!(
    test_mock_index_zh_a_hist_min,
    index_zh_a_hist_min,
    "000001",
    "5"
);
macro_test_arg2!(
    test_mock_index_global_candles,
    index_global_candles,
    ".INX",
    100usize
);
macro_test_arg2!(
    test_mock_sw_index_candles,
    sw_index_candles,
    "801010",
    100usize
);
macro_test_arg2!(test_mock_index_hist_sw, index_hist_sw, "801010", "day");
macro_test_arg3!(
    test_mock_index_analysis_daily_sw,
    index_analysis_daily_sw,
    "801010",
    "20240101",
    "20240131"
);
macro_test_arg2!(
    test_mock_index_analysis_weekly_sw,
    index_analysis_weekly_sw,
    "801010",
    "20240101"
);
macro_test_arg2!(
    test_mock_index_analysis_monthly_sw,
    index_analysis_monthly_sw,
    "801010",
    "20240101"
);
macro_test_arg2!(
    test_mock_index_hist_fund_sw,
    index_hist_fund_sw,
    "801010",
    "day"
);

// Three-arg index methods
macro_test_arg3!(
    test_mock_index_hk_daily,
    index_hk_daily,
    "HSI",
    "1000001",
    100usize
);
macro_test_arg3!(
    test_mock_index_global_hist_em,
    index_global_hist_em,
    "INX",
    ".INX",
    100usize
);
macro_test_arg3!(
    test_mock_index_hist_cni,
    index_hist_cni,
    "000001",
    "2024-01-01",
    "2024-12-31"
);
macro_test_arg3!(
    test_mock_index_detail_hist_cni,
    index_detail_hist_cni,
    "000001",
    "2024-01-01",
    "2024-12-31"
);

// Five-arg index methods
macro_test_arg5!(
    test_mock_index_zh_a_hist_min_em,
    index_zh_a_hist_min_em,
    "000001",
    "5",
    "2024-01-01",
    "2024-01-02",
    "qfq"
);
