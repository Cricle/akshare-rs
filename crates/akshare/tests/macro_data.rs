mod common;

use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ---------------------------------------------------------------------------
// Helper: mount GET + POST catch-all returning em_datacenter_response
// ---------------------------------------------------------------------------

async fn mount_em_mocks(server: &MockServer) {
    let body =
        common::em_datacenter_response(vec![common::sample_macro_row("2024-01-01", 123.45, "GDP")]);
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
        .mount(server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(server)
        .await;
}

// ---------------------------------------------------------------------------
// Macro: generate a no-arg test that calls one method
// ---------------------------------------------------------------------------

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

// ===========================================================================
// China — Eastmoney datacenter report methods
// ===========================================================================

macro_test!(test_macro_china_gdp, macro_china_gdp);
macro_test!(test_macro_china_cpi, macro_china_cpi);
macro_test!(test_macro_china_ppi, macro_china_ppi);
macro_test!(test_macro_china_pmi, macro_china_pmi);
macro_test!(test_macro_china_money_supply, macro_china_money_supply);
macro_test!(test_macro_china_fdi, macro_china_fdi);
macro_test!(test_macro_china_lpr, macro_china_lpr);
macro_test!(
    test_macro_china_new_house_price,
    macro_china_new_house_price
);
macro_test!(
    test_macro_china_enterprise_boom_index,
    macro_china_enterprise_boom_index
);
macro_test!(
    test_macro_china_national_tax_receipts,
    macro_china_national_tax_receipts
);
macro_test!(
    test_macro_china_new_financial_credit,
    macro_china_new_financial_credit
);
macro_test!(test_macro_china_fx_gold, macro_china_fx_gold);
macro_test!(
    test_macro_china_stock_market_cap,
    macro_china_stock_market_cap
);
macro_test!(
    test_macro_china_reserve_requirement_ratio,
    macro_china_reserve_requirement_ratio
);
macro_test!(
    test_macro_china_consumer_goods_retail,
    macro_china_consumer_goods_retail
);

// Original china_* methods (non-alias)
macro_test!(test_china_gdp, china_gdp);
macro_test!(test_china_cpi, china_cpi);
macro_test!(test_china_ppi, china_ppi);
macro_test!(test_china_pmi, china_pmi);
macro_test!(test_china_money_supply, china_money_supply);
macro_test!(test_china_trade, china_trade);
macro_test!(test_china_goods_index, china_goods_index);
macro_test!(test_china_fdi, china_fdi);
macro_test!(test_china_lpr, china_lpr);
macro_test!(test_china_new_house_price, china_new_house_price);
macro_test!(
    test_china_enterprise_boom_index,
    china_enterprise_boom_index
);
macro_test!(
    test_china_national_tax_receipts,
    china_national_tax_receipts
);
macro_test!(test_china_new_financial_credit, china_new_financial_credit);
macro_test!(test_china_fx_gold, china_fx_gold);
macro_test!(test_china_stock_market_cap, china_stock_market_cap);
macro_test!(
    test_china_fixed_asset_investment,
    china_fixed_asset_investment
);
macro_test!(test_china_fiscal_revenue, china_fiscal_revenue);
macro_test!(test_china_fx_loans, china_fx_loans);
macro_test!(test_china_fx_deposits, china_fx_deposits);
macro_test!(test_china_consumer_confidence, china_consumer_confidence);
macro_test!(test_china_industrial_growth, china_industrial_growth);
macro_test!(
    test_china_reserve_requirement_ratio,
    china_reserve_requirement_ratio
);
macro_test!(
    test_china_consumer_goods_retail,
    china_consumer_goods_retail
);

// Chinese-name aliases
macro_test!(test_macro_china_czsr, macro_china_czsr);
macro_test!(test_macro_china_gdzctz, macro_china_gdzctz);
macro_test!(test_macro_china_gyzjz, macro_china_gyzjz);
macro_test!(test_macro_china_hgjck, macro_china_hgjck);
macro_test!(test_macro_china_whxd, macro_china_whxd);
macro_test!(test_macro_china_wbck, macro_china_wbck);
macro_test!(test_macro_china_xfzxx, macro_china_xfzxx);
macro_test!(test_macro_china_qyspjg, macro_china_qyspjg);

// ===========================================================================
// China — Eastmoney industry index methods
// ===========================================================================

macro_test!(test_macro_china_bank_financing, macro_china_bank_financing);
macro_test!(
    test_macro_china_insurance_income,
    macro_china_insurance_income
);
macro_test!(test_macro_china_mobile_number, macro_china_mobile_number);
macro_test!(
    test_macro_china_vegetable_basket,
    macro_china_vegetable_basket
);
macro_test!(
    test_macro_china_agricultural_product,
    macro_china_agricultural_product
);
macro_test!(
    test_macro_china_agricultural_index,
    macro_china_agricultural_index
);
macro_test!(test_macro_china_energy_index, macro_china_energy_index);
macro_test!(
    test_macro_china_commodity_price_index,
    macro_china_commodity_price_index
);
macro_test!(
    test_macro_china_yw_electronic_index,
    macro_china_yw_electronic_index
);
macro_test!(
    test_macro_china_construction_index,
    macro_china_construction_index
);
macro_test!(
    test_macro_china_construction_price_index,
    macro_china_construction_price_index
);
macro_test!(test_macro_china_lpi_index, macro_china_lpi_index);
macro_test!(test_macro_china_bdti_index, macro_china_bdti_index);
macro_test!(test_macro_china_bsi_index, macro_china_bsi_index);
macro_test!(test_macro_china_real_estate, macro_china_real_estate);

// Original china_* industry index methods (non-alias)
macro_test!(test_china_bank_financing, china_bank_financing);
macro_test!(test_china_insurance_income, china_insurance_income);
macro_test!(test_china_mobile_number, china_mobile_number);
macro_test!(test_china_vegetable_basket, china_vegetable_basket);
macro_test!(test_china_agricultural_product, china_agricultural_product);
macro_test!(test_china_agricultural_index, china_agricultural_index);
macro_test!(test_china_energy_index, china_energy_index);
macro_test!(
    test_china_commodity_price_index,
    china_commodity_price_index
);
macro_test!(test_china_yw_electronic_index, china_yw_electronic_index);
macro_test!(test_china_construction_index, china_construction_index);
macro_test!(
    test_china_construction_price_index,
    china_construction_price_index
);
macro_test!(test_china_lpi_index, china_lpi_index);
macro_test!(test_china_bdti_index, china_bdti_index);
macro_test!(test_china_bsi_index, china_bsi_index);
macro_test!(test_china_real_estate, china_real_estate);

// ===========================================================================
// China — Jin10 datacenter methods
// ===========================================================================

macro_test!(test_macro_china_gdp_yearly, macro_china_gdp_yearly);
macro_test!(test_macro_china_cpi_yearly, macro_china_cpi_yearly);
macro_test!(test_macro_china_cpi_monthly, macro_china_cpi_monthly);
macro_test!(test_macro_china_ppi_yearly, macro_china_ppi_yearly);
macro_test!(test_macro_china_exports_yoy, macro_china_exports_yoy);
macro_test!(test_macro_china_imports_yoy, macro_china_imports_yoy);
macro_test!(test_macro_china_trade_balance, macro_china_trade_balance);
macro_test!(
    test_macro_china_industrial_production_yoy,
    macro_china_industrial_production_yoy
);
macro_test!(test_macro_china_pmi_yearly, macro_china_pmi_yearly);
macro_test!(test_macro_china_cx_pmi_yearly, macro_china_cx_pmi_yearly);
macro_test!(
    test_macro_china_cx_services_pmi_yearly,
    macro_china_cx_services_pmi_yearly
);
macro_test!(test_macro_china_non_man_pmi, macro_china_non_man_pmi);
macro_test!(
    test_macro_china_fx_reserves_yearly,
    macro_china_fx_reserves_yearly
);
macro_test!(test_macro_china_m2_yearly, macro_china_m2_yearly);

// Original china_* Jin10 methods (non-alias)
macro_test!(test_china_gdp_yearly, china_gdp_yearly);
macro_test!(test_china_cpi_yearly, china_cpi_yearly);
macro_test!(test_china_cpi_monthly, china_cpi_monthly);
macro_test!(test_china_ppi_yearly, china_ppi_yearly);
macro_test!(test_china_exports_yoy, china_exports_yoy);
macro_test!(test_china_imports_yoy, china_imports_yoy);
macro_test!(test_china_trade_balance_jin10, china_trade_balance_jin10);
macro_test!(
    test_china_industrial_production_yoy,
    china_industrial_production_yoy
);
macro_test!(test_china_pmi_jin10, china_pmi_jin10);
macro_test!(test_china_caixin_pmi, china_caixin_pmi);
macro_test!(test_china_caixin_services_pmi, china_caixin_services_pmi);
macro_test!(test_china_non_man_pmi, china_non_man_pmi);
macro_test!(test_china_fx_reserves_yearly, china_fx_reserves_yearly);
macro_test!(test_china_m2_yearly, china_m2_yearly);

// ===========================================================================
// China — Jin10 CDN methods
// ===========================================================================

macro_test!(test_macro_china_shibor_all, macro_china_shibor_all);
macro_test!(test_macro_china_hk_market_info, macro_china_hk_market_info);
macro_test!(test_macro_china_rmb, macro_china_rmb);
macro_test!(
    test_macro_china_market_margin_sz,
    macro_china_market_margin_sz
);
macro_test!(
    test_macro_china_market_margin_sh,
    macro_china_market_margin_sh
);
macro_test!(test_macro_china_au_report, macro_china_au_report);
macro_test!(test_macro_china_daily_energy, macro_china_daily_energy);

// Original china_* Jin10 CDN methods (non-alias)
macro_test!(test_china_shibor, china_shibor);
macro_test!(test_china_hibor, china_hibor);
macro_test!(test_china_rmb_central_parity, china_rmb_central_parity);
macro_test!(test_china_margin_sz, china_margin_sz);
macro_test!(test_china_margin_sh, china_margin_sh);
macro_test!(test_china_sge_report, china_sge_report);

// ===========================================================================
// China — Sina Finance methods
// ===========================================================================

macro_test!(
    test_macro_china_central_bank_balance,
    macro_china_central_bank_balance
);
macro_test!(test_macro_china_insurance, macro_china_insurance);
macro_test!(
    test_macro_china_supply_of_money,
    macro_china_supply_of_money
);
macro_test!(
    test_macro_china_foreign_exchange_gold,
    macro_china_foreign_exchange_gold
);
macro_test!(
    test_macro_china_retail_price_index,
    macro_china_retail_price_index
);
macro_test!(
    test_macro_china_society_electricity,
    macro_china_society_electricity
);
macro_test!(
    test_macro_china_society_traffic_volume,
    macro_china_society_traffic_volume
);
macro_test!(
    test_macro_china_postal_telecommunicational,
    macro_china_postal_telecommunicational
);
macro_test!(
    test_macro_china_international_tourism_fx,
    macro_china_international_tourism_fx
);
macro_test!(
    test_macro_china_passenger_load_factor,
    macro_china_passenger_load_factor
);
macro_test!(test_macro_china_freight_index, macro_china_freight_index);

// ===========================================================================
// China — Other / special methods
// ===========================================================================

macro_test!(test_macro_cnbs, macro_cnbs);
macro_test!(test_macro_china_shrzgm, macro_china_shrzgm);
macro_test!(test_macro_rmb_loan, macro_rmb_loan);
macro_test!(test_macro_rmb_deposit, macro_rmb_deposit);
macro_test!(
    test_macro_china_urban_unemployment,
    macro_china_urban_unemployment
);

#[tokio::test]
async fn test_macro_china_nbs_nation() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client
        .macro_china_nbs_nation("月度数据", "A010101", "202401")
        .await;
    let _ = result;
}

#[tokio::test]
async fn test_macro_china_nbs_region() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client
        .macro_china_nbs_region("分省月度数据", "A010101", "110000", "202401")
        .await;
    let _ = result;
}

// ===========================================================================
// USA — Eastmoney indicator methods
// ===========================================================================

macro_test!(test_macro_usa_phs, macro_usa_phs);
macro_test!(test_macro_usa_cpi_yoy, macro_usa_cpi_yoy);

// Original us_* Eastmoney indicator methods (non-alias)
macro_test!(test_us_pending_home_sales, us_pending_home_sales);
macro_test!(test_us_cpi_yoy, us_cpi_yoy);

// ===========================================================================
// USA — Jin10 datacenter methods
// ===========================================================================

macro_test!(test_macro_usa_gdp_monthly, macro_usa_gdp_monthly);
macro_test!(test_macro_usa_cpi_monthly, macro_usa_cpi_monthly);
macro_test!(test_macro_usa_core_cpi_monthly, macro_usa_core_cpi_monthly);
macro_test!(
    test_macro_usa_personal_spending,
    macro_usa_personal_spending
);
macro_test!(test_macro_usa_retail_sales, macro_usa_retail_sales);
macro_test!(test_macro_usa_import_price, macro_usa_import_price);
macro_test!(test_macro_usa_export_price, macro_usa_export_price);
macro_test!(test_macro_usa_lmci, macro_usa_lmci);
macro_test!(
    test_macro_usa_unemployment_rate,
    macro_usa_unemployment_rate
);
macro_test!(test_macro_usa_job_cuts, macro_usa_job_cuts);
macro_test!(test_macro_usa_non_farm, macro_usa_non_farm);
macro_test!(test_macro_usa_adp_employment, macro_usa_adp_employment);
macro_test!(test_macro_usa_core_pce_price, macro_usa_core_pce_price);
macro_test!(
    test_macro_usa_real_consumer_spending,
    macro_usa_real_consumer_spending
);
macro_test!(test_macro_usa_trade_balance, macro_usa_trade_balance);
macro_test!(test_macro_usa_current_account, macro_usa_current_account);
macro_test!(test_macro_usa_ppi, macro_usa_ppi);
macro_test!(test_macro_usa_core_ppi, macro_usa_core_ppi);
macro_test!(test_macro_usa_api_crude_stock, macro_usa_api_crude_stock);
macro_test!(test_macro_usa_pmi, macro_usa_pmi);
macro_test!(test_macro_usa_ism_pmi, macro_usa_ism_pmi);
macro_test!(
    test_macro_usa_industrial_production,
    macro_usa_industrial_production
);
macro_test!(
    test_macro_usa_durable_goods_orders,
    macro_usa_durable_goods_orders
);
macro_test!(test_macro_usa_factory_orders, macro_usa_factory_orders);
macro_test!(test_macro_usa_services_pmi, macro_usa_services_pmi);
macro_test!(
    test_macro_usa_business_inventories,
    macro_usa_business_inventories
);
macro_test!(test_macro_usa_ism_non_pmi, macro_usa_ism_non_pmi);
macro_test!(
    test_macro_usa_nahb_house_market_index,
    macro_usa_nahb_house_market_index
);
macro_test!(test_macro_usa_house_starts, macro_usa_house_starts);
macro_test!(test_macro_usa_new_home_sales, macro_usa_new_home_sales);
macro_test!(test_macro_usa_building_permits, macro_usa_building_permits);
macro_test!(test_macro_usa_exist_home_sales, macro_usa_exist_home_sales);
macro_test!(
    test_macro_usa_house_price_index,
    macro_usa_house_price_index
);
macro_test!(test_macro_usa_spcs20, macro_usa_spcs20);
macro_test!(
    test_macro_usa_pending_home_sales,
    macro_usa_pending_home_sales
);
macro_test!(
    test_macro_usa_cb_consumer_confidence,
    macro_usa_cb_consumer_confidence
);
macro_test!(
    test_macro_usa_nfib_small_business,
    macro_usa_nfib_small_business
);
macro_test!(
    test_macro_usa_michigan_consumer_sentiment,
    macro_usa_michigan_consumer_sentiment
);
macro_test!(test_macro_usa_eia_crude_rate, macro_usa_eia_crude_rate);
macro_test!(test_macro_usa_initial_jobless, macro_usa_initial_jobless);

// Original us_* Jin10 methods (non-alias)
macro_test!(test_us_gdp_monthly, us_gdp_monthly);
macro_test!(test_us_cpi_monthly, us_cpi_monthly);
macro_test!(test_us_core_cpi_monthly, us_core_cpi_monthly);
macro_test!(test_us_personal_spending, us_personal_spending);
macro_test!(test_us_retail_sales, us_retail_sales);
macro_test!(test_us_import_price, us_import_price);
macro_test!(test_us_export_price, us_export_price);
macro_test!(test_us_lmci, us_lmci);
macro_test!(test_us_unemployment_rate, us_unemployment_rate);
macro_test!(test_us_job_cuts, us_job_cuts);
macro_test!(test_us_non_farm, us_non_farm);
macro_test!(test_us_adp_employment, us_adp_employment);
macro_test!(test_us_core_pce_price, us_core_pce_price);
macro_test!(test_us_real_consumer_spending, us_real_consumer_spending);
macro_test!(test_us_trade_balance, us_trade_balance);
macro_test!(test_us_current_account, us_current_account);
macro_test!(test_us_ppi, us_ppi);
macro_test!(test_us_core_ppi, us_core_ppi);
macro_test!(test_us_api_crude_stock, us_api_crude_stock);
macro_test!(test_us_pmi, us_pmi);
macro_test!(test_us_ism_pmi, us_ism_pmi);
macro_test!(test_us_industrial_production, us_industrial_production);
macro_test!(test_us_durable_goods_orders, us_durable_goods_orders);
macro_test!(test_us_factory_orders, us_factory_orders);
macro_test!(test_us_services_pmi, us_services_pmi);
macro_test!(test_us_business_inventories, us_business_inventories);
macro_test!(test_us_ism_non_pmi, us_ism_non_pmi);
macro_test!(test_us_nahb_house_market_index, us_nahb_house_market_index);
macro_test!(test_us_house_starts, us_house_starts);
macro_test!(test_us_new_home_sales, us_new_home_sales);
macro_test!(test_us_building_permits, us_building_permits);
macro_test!(test_us_exist_home_sales, us_exist_home_sales);
macro_test!(test_us_house_price_index, us_house_price_index);
macro_test!(test_us_spcs20, us_spcs20);
macro_test!(
    test_us_pending_home_sales_jin10,
    us_pending_home_sales_jin10
);
macro_test!(test_us_cb_consumer_confidence, us_cb_consumer_confidence);
macro_test!(test_us_nfib_small_business, us_nfib_small_business);
macro_test!(
    test_us_michigan_consumer_sentiment,
    us_michigan_consumer_sentiment
);
macro_test!(test_us_eia_crude_rate, us_eia_crude_rate);
macro_test!(test_us_initial_jobless, us_initial_jobless);

// ===========================================================================
// USA — Jin10 CDN methods
// ===========================================================================

macro_test!(test_macro_usa_rig_count, macro_usa_rig_count);
macro_test!(test_macro_usa_crude_inner, macro_usa_crude_inner);
macro_test!(
    test_macro_usa_cme_merchant_goods_holding,
    macro_usa_cme_merchant_goods_holding
);
macro_test!(test_macro_usa_cftc_nc_holding, macro_usa_cftc_nc_holding);
macro_test!(test_macro_usa_cftc_c_holding, macro_usa_cftc_c_holding);
macro_test!(
    test_macro_usa_cftc_merchant_currency_holding,
    macro_usa_cftc_merchant_currency_holding
);
macro_test!(
    test_macro_usa_cftc_merchant_goods_holding,
    macro_usa_cftc_merchant_goods_holding
);

// Original us_* Jin10 CDN methods (non-alias)
macro_test!(test_us_rig_count, us_rig_count);
macro_test!(test_us_crude_production, us_crude_production);
macro_test!(test_us_cftc_nc_holding, us_cftc_nc_holding);
macro_test!(test_us_cftc_c_holding, us_cftc_c_holding);
macro_test!(
    test_us_cftc_merchant_currency_holding,
    us_cftc_merchant_currency_holding
);
macro_test!(
    test_us_cftc_merchant_goods_holding,
    us_cftc_merchant_goods_holding
);

// ===========================================================================
// Euro — Jin10 datacenter methods
// ===========================================================================

macro_test!(test_macro_euro_gdp_yoy, macro_euro_gdp_yoy);
macro_test!(test_macro_euro_cpi_mom, macro_euro_cpi_mom);
macro_test!(test_macro_euro_cpi_yoy, macro_euro_cpi_yoy);
macro_test!(test_macro_euro_ppi_mom, macro_euro_ppi_mom);
macro_test!(
    test_macro_euro_retail_sales_mom,
    macro_euro_retail_sales_mom
);
macro_test!(
    test_macro_euro_employment_change_qoq,
    macro_euro_employment_change_qoq
);
macro_test!(
    test_macro_euro_unemployment_rate_mom,
    macro_euro_unemployment_rate_mom
);
macro_test!(test_macro_euro_trade_balance, macro_euro_trade_balance);
macro_test!(
    test_macro_euro_current_account_mom,
    macro_euro_current_account_mom
);
macro_test!(
    test_macro_euro_industrial_production_mom,
    macro_euro_industrial_production_mom
);
macro_test!(
    test_macro_euro_manufacturing_pmi,
    macro_euro_manufacturing_pmi
);
macro_test!(test_macro_euro_services_pmi, macro_euro_services_pmi);
macro_test!(
    test_macro_euro_zew_economic_sentiment,
    macro_euro_zew_economic_sentiment
);
macro_test!(
    test_macro_euro_sentix_investor_confidence,
    macro_euro_sentix_investor_confidence
);

// Euro — CDN (LME) methods
macro_test!(test_macro_euro_lme_holding, macro_euro_lme_holding);
macro_test!(test_macro_euro_lme_stock, macro_euro_lme_stock);

// Original euro_* methods (non-alias)
macro_test!(test_euro_gdp_yoy, euro_gdp_yoy);
macro_test!(test_euro_cpi_mom, euro_cpi_mom);
macro_test!(test_euro_cpi_yoy, euro_cpi_yoy);
macro_test!(test_euro_ppi_mom, euro_ppi_mom);
macro_test!(test_euro_retail_sales_mom, euro_retail_sales_mom);
macro_test!(test_euro_employment_change_qoq, euro_employment_change_qoq);
macro_test!(test_euro_unemployment_rate, euro_unemployment_rate);
macro_test!(test_euro_trade_balance, euro_trade_balance);
macro_test!(test_euro_current_account_mom, euro_current_account_mom);
macro_test!(
    test_euro_industrial_production_mom,
    euro_industrial_production_mom
);
macro_test!(test_euro_manufacturing_pmi, euro_manufacturing_pmi);
macro_test!(test_euro_services_pmi, euro_services_pmi);
macro_test!(
    test_euro_zew_economic_sentiment,
    euro_zew_economic_sentiment
);
macro_test!(
    test_euro_sentix_investor_confidence,
    euro_sentix_investor_confidence
);
macro_test!(test_euro_lme_holding, euro_lme_holding);
macro_test!(test_euro_lme_stock, euro_lme_stock);

// ===========================================================================
// UK — Eastmoney indicator methods
// ===========================================================================

macro_test!(test_macro_uk_halifax_monthly, macro_uk_halifax_monthly);
macro_test!(test_macro_uk_halifax_yearly, macro_uk_halifax_yearly);
macro_test!(test_macro_uk_trade, macro_uk_trade);
macro_test!(test_macro_uk_bank_rate, macro_uk_bank_rate);
macro_test!(test_macro_uk_core_cpi_yearly, macro_uk_core_cpi_yearly);
macro_test!(test_macro_uk_core_cpi_monthly, macro_uk_core_cpi_monthly);
macro_test!(test_macro_uk_cpi_yearly, macro_uk_cpi_yearly);
macro_test!(test_macro_uk_cpi_monthly, macro_uk_cpi_monthly);
macro_test!(test_macro_uk_retail_monthly, macro_uk_retail_monthly);
macro_test!(test_macro_uk_retail_yearly, macro_uk_retail_yearly);
macro_test!(test_macro_uk_rightmove_yearly, macro_uk_rightmove_yearly);
macro_test!(test_macro_uk_rightmove_monthly, macro_uk_rightmove_monthly);
macro_test!(test_macro_uk_gdp_quarterly, macro_uk_gdp_quarterly);
macro_test!(test_macro_uk_gdp_yearly, macro_uk_gdp_yearly);
macro_test!(test_macro_uk_unemployment_rate, macro_uk_unemployment_rate);

// Original uk_* methods (non-alias)
macro_test!(test_uk_halifax_monthly, uk_halifax_monthly);
macro_test!(test_uk_halifax_yearly, uk_halifax_yearly);
macro_test!(test_uk_trade, uk_trade);
macro_test!(test_uk_bank_rate, uk_bank_rate);
macro_test!(test_uk_core_cpi_yearly, uk_core_cpi_yearly);
macro_test!(test_uk_core_cpi_monthly, uk_core_cpi_monthly);
macro_test!(test_uk_cpi_yearly, uk_cpi_yearly);
macro_test!(test_uk_cpi_monthly, uk_cpi_monthly);
macro_test!(test_uk_retail_monthly, uk_retail_monthly);
macro_test!(test_uk_retail_yearly, uk_retail_yearly);
macro_test!(test_uk_rightmove_yearly, uk_rightmove_yearly);
macro_test!(test_uk_rightmove_monthly, uk_rightmove_monthly);
macro_test!(test_uk_gdp_quarterly, uk_gdp_quarterly);
macro_test!(test_uk_gdp_yearly, uk_gdp_yearly);
macro_test!(test_uk_unemployment_rate, uk_unemployment_rate);

// ===========================================================================
// Japan — Eastmoney indicator methods
// ===========================================================================

macro_test!(test_macro_japan_bank_rate, macro_japan_bank_rate);
macro_test!(test_macro_japan_cpi_yearly, macro_japan_cpi_yearly);
macro_test!(
    test_macro_japan_core_cpi_yearly,
    macro_japan_core_cpi_yearly
);
macro_test!(
    test_macro_japan_unemployment_rate,
    macro_japan_unemployment_rate
);
macro_test!(test_macro_japan_head_indicator, macro_japan_head_indicator);

// Original japan_* methods (non-alias)
macro_test!(test_japan_bank_rate, japan_bank_rate);
macro_test!(test_japan_cpi_yearly, japan_cpi_yearly);
macro_test!(test_japan_core_cpi_yearly, japan_core_cpi_yearly);
macro_test!(test_japan_unemployment_rate, japan_unemployment_rate);
macro_test!(test_japan_leading_indicator, japan_leading_indicator);

// ===========================================================================
// Australia — Eastmoney indicator methods
// ===========================================================================

macro_test!(
    test_macro_australia_retail_rate_monthly,
    macro_australia_retail_rate_monthly
);
macro_test!(test_macro_australia_trade, macro_australia_trade);
macro_test!(
    test_macro_australia_unemployment_rate,
    macro_australia_unemployment_rate
);
macro_test!(
    test_macro_australia_ppi_quarterly,
    macro_australia_ppi_quarterly
);
macro_test!(
    test_macro_australia_cpi_quarterly,
    macro_australia_cpi_quarterly
);
macro_test!(test_macro_australia_cpi_yearly, macro_australia_cpi_yearly);
macro_test!(test_macro_australia_bank_rate, macro_australia_bank_rate);

// Original australia_* methods (non-alias)
macro_test!(
    test_australia_retail_rate_monthly,
    australia_retail_rate_monthly
);
macro_test!(test_australia_trade, australia_trade);
macro_test!(
    test_australia_unemployment_rate,
    australia_unemployment_rate
);
macro_test!(test_australia_ppi_quarterly, australia_ppi_quarterly);
macro_test!(test_australia_cpi_quarterly, australia_cpi_quarterly);
macro_test!(test_australia_cpi_yearly, australia_cpi_yearly);
macro_test!(test_australia_bank_rate, australia_bank_rate);

// ===========================================================================
// Canada — Eastmoney indicator methods
// ===========================================================================

macro_test!(
    test_macro_canada_new_house_rate,
    macro_canada_new_house_rate
);
macro_test!(
    test_macro_canada_unemployment_rate,
    macro_canada_unemployment_rate
);
macro_test!(test_macro_canada_trade, macro_canada_trade);
macro_test!(
    test_macro_canada_retail_rate_monthly,
    macro_canada_retail_rate_monthly
);
macro_test!(test_macro_canada_bank_rate, macro_canada_bank_rate);
macro_test!(
    test_macro_canada_core_cpi_yearly,
    macro_canada_core_cpi_yearly
);
macro_test!(
    test_macro_canada_core_cpi_monthly,
    macro_canada_core_cpi_monthly
);
macro_test!(test_macro_canada_cpi_yearly, macro_canada_cpi_yearly);
macro_test!(test_macro_canada_cpi_monthly, macro_canada_cpi_monthly);
macro_test!(test_macro_canada_gdp_monthly, macro_canada_gdp_monthly);

// Original canada_* methods (non-alias)
macro_test!(test_canada_new_house_rate, canada_new_house_rate);
macro_test!(test_canada_unemployment_rate, canada_unemployment_rate);
macro_test!(test_canada_trade, canada_trade);
macro_test!(test_canada_retail_rate_monthly, canada_retail_rate_monthly);
macro_test!(test_canada_bank_rate, canada_bank_rate);
macro_test!(test_canada_core_cpi_yearly, canada_core_cpi_yearly);
macro_test!(test_canada_core_cpi_monthly, canada_core_cpi_monthly);
macro_test!(test_canada_cpi_yearly, canada_cpi_yearly);
macro_test!(test_canada_cpi_monthly, canada_cpi_monthly);
macro_test!(test_canada_gdp_monthly, canada_gdp_monthly);

// ===========================================================================
// Germany — Eastmoney indicator methods
// ===========================================================================

macro_test!(test_macro_germany_ifo, macro_germany_ifo);
macro_test!(test_macro_germany_cpi_monthly, macro_germany_cpi_monthly);
macro_test!(test_macro_germany_cpi_yearly, macro_germany_cpi_yearly);
macro_test!(
    test_macro_germany_trade_adjusted,
    macro_germany_trade_adjusted
);
macro_test!(test_macro_germany_gdp, macro_germany_gdp);
macro_test!(
    test_macro_germany_retail_sale_monthly,
    macro_germany_retail_sale_monthly
);
macro_test!(
    test_macro_germany_retail_sale_yearly,
    macro_germany_retail_sale_yearly
);
macro_test!(test_macro_germany_zew, macro_germany_zew);

// Original germany_* methods (non-alias)
macro_test!(test_germany_ifo, germany_ifo);
macro_test!(test_germany_cpi_monthly, germany_cpi_monthly);
macro_test!(test_germany_cpi_yearly, germany_cpi_yearly);
macro_test!(test_germany_trade_adjusted, germany_trade_adjusted);
macro_test!(test_germany_gdp, germany_gdp);
macro_test!(
    test_germany_retail_sale_monthly,
    germany_retail_sale_monthly
);
macro_test!(test_germany_retail_sale_yearly, germany_retail_sale_yearly);
macro_test!(test_germany_zew, germany_zew);

// ===========================================================================
// Switzerland — Eastmoney indicator methods
// ===========================================================================

macro_test!(test_macro_swiss_svme, macro_swiss_svme);
macro_test!(test_macro_swiss_trade, macro_swiss_trade);
macro_test!(test_macro_swiss_cpi_yearly, macro_swiss_cpi_yearly);
macro_test!(test_macro_swiss_gdp_quarterly, macro_swiss_gdp_quarterly);
macro_test!(test_macro_swiss_gbd_yearly, macro_swiss_gbd_yearly);
macro_test!(test_macro_swiss_gbd_bank_rate, macro_swiss_gbd_bank_rate);

// Original swiss_* methods (non-alias)
macro_test!(test_swiss_svme, swiss_svme);
macro_test!(test_swiss_trade, swiss_trade);
macro_test!(test_swiss_cpi_yearly, swiss_cpi_yearly);
macro_test!(test_swiss_gdp_quarterly, swiss_gdp_quarterly);
macro_test!(test_swiss_gdp_yearly, swiss_gdp_yearly);
macro_test!(test_swiss_bank_rate, swiss_bank_rate);

// ===========================================================================
// Hong Kong — Eastmoney indicator methods
// ===========================================================================

macro_test!(test_macro_china_hk_cpi, macro_china_hk_cpi);
macro_test!(test_macro_china_hk_cpi_ratio, macro_china_hk_cpi_ratio);
macro_test!(
    test_macro_china_hk_rate_of_unemployment,
    macro_china_hk_rate_of_unemployment
);
macro_test!(test_macro_china_hk_gbp, macro_china_hk_gbp);
macro_test!(test_macro_china_hk_gbp_ratio, macro_china_hk_gbp_ratio);
macro_test!(
    test_macro_china_hk_building_volume,
    macro_china_hk_building_volume
);
macro_test!(
    test_macro_china_hk_building_amount,
    macro_china_hk_building_amount
);
macro_test!(
    test_macro_china_hk_trade_diff_ratio,
    macro_china_hk_trade_diff_ratio
);
macro_test!(test_macro_china_hk_ppi, macro_china_hk_ppi);

// Original hk_* methods (non-alias)
macro_test!(test_hk_cpi, hk_cpi);
macro_test!(test_hk_cpi_ratio, hk_cpi_ratio);
macro_test!(test_hk_unemployment_rate, hk_unemployment_rate);
macro_test!(test_hk_gdp, hk_gdp);
macro_test!(test_hk_gdp_ratio, hk_gdp_ratio);
macro_test!(test_hk_building_volume, hk_building_volume);
macro_test!(test_hk_building_amount, hk_building_amount);
macro_test!(test_hk_trade_diff_ratio, hk_trade_diff_ratio);
macro_test!(test_hk_ppi, hk_ppi);

// ===========================================================================
// Rate — China Money repo rate methods
// ===========================================================================

#[tokio::test]
async fn test_repo_rate_query_fr() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.repo_rate_query("回购定盘利率").await;
    let _ = result;
}

#[tokio::test]
async fn test_repo_rate_query_fdr() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.repo_rate_query("银银间回购定盘利率").await;
    let _ = result;
}

#[tokio::test]
async fn test_repo_rate_hist() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.repo_rate_hist("20240101", "20240131").await;
    let _ = result;
}

// Interbank lending rate
#[tokio::test]
async fn test_rate_interbank() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client
        .rate_interbank("上海银行同业拆借市场", "Shibor人民币", "隔夜")
        .await;
    let _ = result;
}

// ===========================================================================
// Bank — Jin10 interest rate decision methods
// ===========================================================================

macro_test!(
    test_macro_bank_usa_interest_rate,
    macro_bank_usa_interest_rate
);
macro_test!(
    test_macro_bank_euro_interest_rate,
    macro_bank_euro_interest_rate
);
macro_test!(
    test_macro_bank_newzealand_interest_rate,
    macro_bank_newzealand_interest_rate
);
macro_test!(
    test_macro_bank_china_interest_rate,
    macro_bank_china_interest_rate
);
macro_test!(
    test_macro_bank_switzerland_interest_rate,
    macro_bank_switzerland_interest_rate
);
macro_test!(
    test_macro_bank_english_interest_rate,
    macro_bank_english_interest_rate
);
macro_test!(
    test_macro_bank_australia_interest_rate,
    macro_bank_australia_interest_rate
);
macro_test!(
    test_macro_bank_japan_interest_rate,
    macro_bank_japan_interest_rate
);
macro_test!(
    test_macro_bank_russia_interest_rate,
    macro_bank_russia_interest_rate
);
macro_test!(
    test_macro_bank_india_interest_rate,
    macro_bank_india_interest_rate
);
macro_test!(
    test_macro_bank_brazil_interest_rate,
    macro_bank_brazil_interest_rate
);

// Original bank_* methods (non-alias)
macro_test!(test_bank_usa_interest_rate, bank_usa_interest_rate);
macro_test!(test_bank_euro_interest_rate, bank_euro_interest_rate);
macro_test!(
    test_bank_newzealand_interest_rate,
    bank_newzealand_interest_rate
);
macro_test!(test_bank_china_interest_rate, bank_china_interest_rate);
macro_test!(
    test_bank_switzerland_interest_rate,
    bank_switzerland_interest_rate
);
macro_test!(test_bank_england_interest_rate, bank_england_interest_rate);
macro_test!(
    test_bank_australia_interest_rate,
    bank_australia_interest_rate
);
macro_test!(test_bank_japan_interest_rate, bank_japan_interest_rate);
macro_test!(test_bank_russia_interest_rate, bank_russia_interest_rate);
macro_test!(test_bank_india_interest_rate, bank_india_interest_rate);
macro_test!(test_bank_brazil_interest_rate, bank_brazil_interest_rate);

// ===========================================================================
// Global — Eastmoney industry index methods
// ===========================================================================

macro_test!(test_macro_global_sox_index, macro_global_sox_index);
macro_test!(test_macro_shipping_bci, macro_shipping_bci);
macro_test!(test_macro_shipping_bdi, macro_shipping_bdi);
macro_test!(test_macro_shipping_bpi, macro_shipping_bpi);
macro_test!(test_macro_shipping_bcti, macro_shipping_bcti);

// ===========================================================================
// Constitute — Jin10 ETF / OPEC methods
// ===========================================================================

macro_test!(test_macro_cons_gold, macro_cons_gold);
macro_test!(test_macro_cons_silver, macro_cons_silver);
macro_test!(test_macro_cons_opec_month, macro_cons_opec_month);

// Original constitute methods (non-alias)
macro_test!(test_gold_etf_holding, gold_etf_holding);
macro_test!(test_silver_etf_holding, silver_etf_holding);

// ===========================================================================
// Other — Jin10 / THS / Wall Street methods
// ===========================================================================

macro_test!(test_macro_crypto_spot, macro_crypto_spot);
macro_test!(test_macro_stock_finance, macro_stock_finance);

#[tokio::test]
async fn test_macro_fx_sentiment() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.macro_fx_sentiment("20240101", "20240131").await;
    let _ = result;
}

#[tokio::test]
async fn test_macro_info_ws() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.macro_info_ws("20240101").await;
    let _ = result;
}

// ===========================================================================
// China — china_money.rs: macro_china_bond_public and macro_china_swap_rate
// ===========================================================================

#[tokio::test]
async fn test_macro_china_bond_public() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "records": [
                { "bondType": "国债", "bondName": "TestBond", "issueAmount": 100.0 }
            ]
        })))
        .mount(&server)
        .await;
    let client = common::mock_client(&server);
    let result = client.macro_china_bond_public(1).await;
    let _ = result;
}

#[tokio::test]
async fn test_macro_china_swap_rate() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "records": [
                { "date": "2024-01-02", "tenor1M": 1.80, "tenor3M": 1.90, "tenor6M": 2.00 }
            ]
        })))
        .mount(&server)
        .await;
    let client = common::mock_client(&server);
    let result = client.macro_china_swap_rate("20240101", "20240131").await;
    let _ = result;
}
