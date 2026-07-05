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
#[allow(unused_macros)]
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

// === Newly covered functions (previously untested) ===

// reits
#[tokio::test]
async fn t_reits_hist_em() {
    let s = wiremock::MockServer::start().await;
    mount_mocks(&s).await;
    let _ = common::mock_client(&s)
        .reits_hist_em("508018", "daily", "20250101", "20250601", "qfq")
        .await;
}

// hk/us search & capital flow
mt2!(t_hk_search, hk_search, "腾讯", 10);
mt2!(t_hk_capital_flow, hk_capital_flow, "00700", 10);
mt2!(t_us_search, us_search, "AAPL", 10);
mt2!(t_us_capital_flow, us_capital_flow, "105.MSFT", 10);

// hk/us sector
mt2!(t_hk_sector_rankings, hk_sector_rankings, "industry", 10);
mt2!(
    t_hk_sector_capital_flow,
    hk_sector_capital_flow,
    "BK0475",
    10
);
mt2!(
    t_hk_sector_constituents,
    hk_sector_constituents,
    "BK0475",
    10
);
mt2!(t_us_sector_rankings, us_sector_rankings, "industry", 10);
mt2!(
    t_us_sector_capital_flow,
    us_sector_capital_flow,
    "BK0475",
    10
);
mt2!(
    t_us_sector_constituents,
    us_sector_constituents,
    "BK0475",
    10
);

// macro_data
mt3!(
    t_macro_china_nbs_nation,
    macro_china_nbs_nation,
    "月度数据",
    "A01",
    "A0101"
);
#[tokio::test]
async fn t_macro_china_nbs_region() {
    let s = wiremock::MockServer::start().await;
    mount_mocks(&s).await;
    let _ = common::mock_client(&s)
        .macro_china_nbs_region("分省月度数据", "A01", "A0101", "202501")
        .await;
}
mt3!(t_rate_interbank, rate_interbank, "shibor", "隔夜", "利率");

// economy / air
mt3!(
    t_air_quality_hist,
    air_quality_hist,
    "北京",
    "20250101",
    "20250601"
);
mt3!(
    t_air_quality_watch_point,
    air_quality_watch_point,
    "北京",
    "20250101",
    "20250601"
);

// provider / eastmoney
mt2!(
    t_eastmoney_sector_rankings_by_fs,
    eastmoney_sector_rankings_by_fs,
    "m:90+t2",
    10
);
mt3!(
    t_eastmoney_sector_capital_flow_by_prefix,
    eastmoney_sector_capital_flow_by_prefix,
    "90",
    "BK0475",
    10
);

// news (complex signatures — inline)
#[tokio::test]
async fn t_finnhub_company_news() {
    let s = wiremock::MockServer::start().await;
    mount_mocks(&s).await;
    let _ = common::mock_client(&s)
        .finnhub_company_news("AAPL", "2025-01-01", "2025-01-07", "test_key")
        .await;
}

#[tokio::test]
async fn t_gdelt_news_search() {
    let s = wiremock::MockServer::start().await;
    mount_mocks(&s).await;
    let _ = common::mock_client(&s)
        .gdelt_news_search(
            "finance",
            "https://api.gdeltproject.org/api/v2/doc/doc",
            None,
            None,
            10,
        )
        .await;
}

#[tokio::test]
async fn t_gdelt_news_search_owned() {
    let s = wiremock::MockServer::start().await;
    mount_mocks(&s).await;
    let _ = common::mock_client(&s)
        .gdelt_news_search_owned(
            "finance",
            "https://api.gdeltproject.org/api/v2/doc/doc",
            None,
            None,
            10,
        )
        .await;
}
