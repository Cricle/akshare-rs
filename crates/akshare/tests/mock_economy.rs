mod common;

use wiremock::MockServer;

async fn mount_em_mocks(server: &MockServer) {
    common::mount_em_mocks(server).await;
}

macro_rules! macro_test {
    ($test_name:ident, $method:ident) => {
        #[tokio::test]
        async fn $test_name() {
            let server = MockServer::start().await;
            mount_em_mocks(&server).await;
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
            mount_em_mocks(&server).await;
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
            mount_em_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg1, $arg2).await;
            let _ = result;
        }
    };
}

// No-arg functions
macro_test!(test_mock_air_city_table, air_city_table);
macro_test!(test_mock_air_quality_hebei, air_quality_hebei);
macro_test!(test_mock_air_quality_rank, air_quality_rank);
macro_test!(test_mock_amac_aoin_info, amac_aoin_info);
macro_test!(test_mock_amac_fund_abs, amac_fund_abs);
macro_test!(test_mock_amac_fund_account_info, amac_fund_account_info);
macro_test!(test_mock_amac_fund_info, amac_fund_info);
macro_test!(test_mock_amac_fund_sub_info, amac_fund_sub_info);
macro_test!(test_mock_amac_futures_info, amac_futures_info);
macro_test!(test_mock_amac_manager_cancelled_info, amac_manager_cancelled_info);
macro_test!(test_mock_amac_manager_classify_info, amac_manager_classify_info);
macro_test!(test_mock_amac_manager_info, amac_manager_info);
macro_test!(test_mock_amac_member_info, amac_member_info);
macro_test!(test_mock_amac_member_sub_info, amac_member_sub_info);
macro_test!(test_mock_amac_person_bond_org_list, amac_person_bond_org_list);
macro_test!(test_mock_amac_person_fund_org_list, amac_person_fund_org_list);
macro_test!(test_mock_amac_securities_info, amac_securities_info);
macro_test!(test_mock_article_ff_crr, article_ff_crr);
macro_test!(test_mock_business_value_artist, business_value_artist);
macro_test!(test_mock_economy_amac_stats, economy_amac_stats);
macro_test!(test_mock_economy_auto_sales, economy_auto_sales);
macro_test!(test_mock_economy_box_office, economy_box_office);
macro_test!(test_mock_economy_sentiment_index, economy_sentiment_index);
macro_test!(test_mock_index_bloomberg_billionaires, index_bloomberg_billionaires);
macro_test!(test_mock_movie_boxoffice_realtime, movie_boxoffice_realtime);
macro_test!(test_mock_online_value_artist, online_value_artist);
macro_test!(test_mock_video_tv, video_tv);
macro_test!(test_mock_video_variety_show, video_variety_show);

// Single-arg functions
macro_test_arg1!(test_mock_article_epu_index, article_epu_index, "US");
macro_test_arg1!(test_mock_article_oman_rv_short, article_oman_rv_short, "AAPL");
macro_test_arg1!(test_mock_article_rlab_rv, article_rlab_rv, "AAPL");
macro_test_arg1!(test_mock_car_market_country_cpca, car_market_country_cpca, "中国");
macro_test_arg1!(test_mock_car_market_fuel_cpca, car_market_fuel_cpca, "中国");
macro_test_arg1!(test_mock_economy_air_quality, economy_air_quality, "北京");
macro_test_arg1!(test_mock_forbes_rank, forbes_rank, "2024");
macro_test_arg1!(test_mock_fred_md, fred_md, "2024-01");
macro_test_arg1!(test_mock_fred_qd, fred_qd, "2024-01");
macro_test_arg1!(test_mock_game_hot_rank_taptap, game_hot_rank_taptap, "热门");
macro_test_arg1!(test_mock_index_bloomberg_billionaires_hist, index_bloomberg_billionaires_hist, "2024");
macro_test_arg1!(test_mock_movie_boxoffice_cinema_daily, movie_boxoffice_cinema_daily, "2024-01-01");
macro_test_arg1!(test_mock_movie_boxoffice_cinema_weekly, movie_boxoffice_cinema_weekly, "2024-01-01");
macro_test_arg1!(test_mock_movie_boxoffice_daily, movie_boxoffice_daily, "2024-01-01");
macro_test_arg1!(test_mock_movie_boxoffice_monthly, movie_boxoffice_monthly, "2024-01");
macro_test_arg1!(test_mock_movie_boxoffice_weekly, movie_boxoffice_weekly, "2024-01-01");
macro_test_arg1!(test_mock_movie_boxoffice_yearly, movie_boxoffice_yearly, "2024");
macro_test_arg1!(test_mock_movie_boxoffice_yearly_first_week, movie_boxoffice_yearly_first_week, "2024");
macro_test_arg1!(test_mock_xincaifu_rank, xincaifu_rank, "2024");

// Two-arg functions
#[tokio::test]
async fn test_mock_air_quality_hist() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.air_quality_hist("北京", "2024-01-01", "2024-01-31").await;
    let _ = result;
}

#[tokio::test]
async fn test_mock_air_quality_watch_point() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.air_quality_watch_point("北京", "2024-01-01", "2024-01-31").await;
    let _ = result;
}
macro_test_arg2!(test_mock_article_oman_rv, article_oman_rv, "AAPL", "1");
macro_test_arg2!(test_mock_car_market_cate_cpca, car_market_cate_cpca, "中国", "轿车");
macro_test_arg2!(test_mock_car_market_man_rank_cpca, car_market_man_rank_cpca, "中国", "1");
macro_test_arg2!(test_mock_car_market_segment_cpca, car_market_segment_cpca, "中国", "轿车");
macro_test_arg2!(test_mock_car_market_total_cpca, car_market_total_cpca, "中国", "1");
macro_test_arg2!(test_mock_car_sale_rank_gasgoo, car_sale_rank_gasgoo, "中国", "2024-01");
macro_test_arg2!(test_mock_hurun_rank, hurun_rank, "2024", "全部");
#[tokio::test]
async fn test_mock_migration_area_baidu() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.migration_area_baidu("北京", "迁入", "2024-01-01").await;
    let _ = result;
}
macro_test_arg2!(test_mock_migration_scale_baidu, migration_scale_baidu, "北京", "迁入");
macro_test_arg1!(test_mock_nlp_answer, nlp_answer, "你好");
macro_test_arg2!(test_mock_nlp_ownthink, nlp_ownthink, "你好", "你好");
macro_test_arg2!(test_mock_sunrise_daily, sunrise_daily, "2024-01-01", "北京");
macro_test_arg2!(test_mock_sunrise_monthly, sunrise_monthly, "2024-01", "北京");
