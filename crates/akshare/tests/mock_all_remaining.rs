mod common;
use wiremock::MockServer;

async fn mount_mocks(server: &MockServer) {
    common::mount_em_mocks(server).await;
}

macro_rules! mt0 {
    ($t:ident, $m:ident) => {
        #[tokio::test]
        async fn $t() {
            let s = MockServer::start().await;
            mount_mocks(&s).await;
            let _ = common::mock_client(&s).$m().await;
        }
    };
}
macro_rules! mt1 {
    ($t:ident, $m:ident, $a:expr) => {
        #[tokio::test]
        async fn $t() {
            let s = MockServer::start().await;
            mount_mocks(&s).await;
            let _ = common::mock_client(&s).$m($a).await;
        }
    };
}
macro_rules! mt2 {
    ($t:ident, $m:ident, $a:expr, $b:expr) => {
        #[tokio::test]
        async fn $t() {
            let s = MockServer::start().await;
            mount_mocks(&s).await;
            let _ = common::mock_client(&s).$m($a, $b).await;
        }
    };
}
macro_rules! mt3 {
    ($t:ident, $m:ident, $a:expr, $b:expr, $c:expr) => {
        #[tokio::test]
        async fn $t() {
            let s = MockServer::start().await;
            mount_mocks(&s).await;
            let _ = common::mock_client(&s).$m($a, $b, $c).await;
        }
    };
}

// Zero-arg functions
mt0!(t_stock_zh_a_spot, stock_zh_a_spot);
mt0!(t_index_global_spot, index_global_spot);
mt0!(t_fund_etf_spot_em, fund_etf_spot_em);
mt0!(t_china_gdp, china_gdp);
mt0!(t_china_cpi, china_cpi);
mt0!(t_forex_sina_rates, forex_sina_rates);
mt0!(t_crypto_js_spot, crypto_js_spot);
mt0!(t_china_trade, china_trade);
mt0!(t_china_money_supply, china_money_supply);
mt0!(t_option_current_day_sse, option_current_day_sse);
mt1!(t_reits_list, reits_list, 10);
mt0!(t_reits_realtime, reits_realtime);
mt0!(t_energy_carbon_domestic, energy_carbon_domestic);
mt0!(t_energy_oil_hist, energy_oil_hist);
mt1!(t_bank_fjcf_table_detail, bank_fjcf_table_detail, "1");

// One-arg functions
mt1!(t_bond_zh_cov, bond_zh_cov, 10);
mt1!(t_news_cctv, news_cctv, "20250620");
mt1!(t_spot_quotations_sge, spot_quotations_sge, "Au99.99");
mt1!(t_commodity_spot_prices, commodity_spot_prices, 10);
mt1!(t_hk_quote, hk_quote, "00700");
mt1!(
    t_stock_billboard_details_em,
    stock_billboard_details_em,
    "2025-06-20"
);
mt2!(t_index_a_share_candles, index_a_share_candles, "000300", 10);
mt0!(t_bond_cb_jsl, bond_cb_jsl);
mt1!(t_bond_cb_profile, bond_cb_profile, "sz128039");
mt0!(t_spot_hog_soozhu, spot_hog_soozhu);

// Two-arg functions
mt2!(
    t_bond_china_yield,
    bond_china_yield,
    "2025-01-01",
    "2025-01-31"
);
mt2!(t_bing_news, bing_news, "finance", 10);
mt1!(
    t_stock_billboard_statistic_em,
    stock_billboard_statistic_em,
    "10"
);
mt1!(t_news_economic, news_economic, "中国");
mt2!(t_fund_open_fund_rank, fund_open_fund_rank, "1", 10);
mt1!(t_bond_corporate_yields, bond_corporate_yields, 10);
mt1!(t_bond_convertible_list, bond_convertible_list, 10);
