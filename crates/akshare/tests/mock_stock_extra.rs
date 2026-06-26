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

// === No-arg functions ===
macro_test!(
    test_extra_stock_board_concept_name_ths,
    stock_board_concept_name_ths
);
macro_test!(
    test_extra_stock_board_industry_name_ths,
    stock_board_industry_name_ths
);
macro_test!(test_extra_stock_hk_spot, stock_hk_spot);
macro_test!(test_extra_stock_hk_famous_spot, stock_hk_famous_spot);
macro_test!(test_extra_stock_hk_index_spot_em, stock_hk_index_spot_em);
macro_test!(
    test_extra_stock_hk_index_spot_sina,
    stock_hk_index_spot_sina
);
macro_test!(test_extra_stock_hk_hot_rank, stock_hk_hot_rank);
macro_test!(test_extra_stock_hk_gxl_lg, stock_hk_gxl_lg);
macro_test!(test_extra_stock_js_weibo_nlp_time, stock_js_weibo_nlp_time);
macro_test!(
    test_extra_stock_margin_underlying_info_szse_dc,
    stock_margin_underlying_info_szse_dc
);
macro_test!(test_extra_stock_dzjy_hygt, stock_dzjy_hygt);
macro_test!(test_extra_stock_us_spot, stock_us_spot);
macro_test!(test_extra_stock_us_pink_spot, stock_us_pink_spot);
macro_test!(test_extra_stock_zh_a_spot, stock_zh_a_spot);
macro_test!(test_extra_stock_zh_a_new, stock_zh_a_new);
macro_test!(test_extra_stock_zh_a_stop, stock_zh_a_stop);
macro_test!(test_extra_stock_zh_ah_spot, stock_zh_ah_spot);
macro_test!(test_extra_stock_zh_ah_name, stock_zh_ah_name);
macro_test!(test_extra_stock_zh_b_spot, stock_zh_b_spot);
macro_test!(test_extra_stock_zh_index_spot_em, stock_zh_index_spot_em);
macro_test!(
    test_extra_stock_zh_index_spot_sina,
    stock_zh_index_spot_sina
);
macro_test!(test_extra_stock_zh_kcb_spot, stock_zh_kcb_spot);
macro_test!(test_extra_stock_ipo_declare, stock_ipo_declare);
macro_test!(test_extra_stock_ipo_review, stock_ipo_review);
macro_test!(test_extra_stock_ipo_tutor, stock_ipo_tutor);
macro_test!(test_extra_stock_history_dividend, stock_history_dividend);
macro_test!(test_extra_stock_ipo_hk, stock_ipo_hk);
macro_test!(test_extra_stock_comment, stock_comment);
macro_test!(test_extra_stock_dxsyl, stock_dxsyl);
macro_test!(test_extra_stock_esg_msci, stock_esg_msci);
macro_test!(test_extra_stock_esg_rft, stock_esg_rft);
macro_test!(test_extra_stock_esg_zd, stock_esg_zd);
macro_test!(test_extra_stock_esg_hz, stock_esg_hz);
macro_test!(
    test_extra_stock_sector_fund_flow_summary,
    stock_sector_fund_flow_summary
);
macro_test!(test_extra_stock_market_fund_flow, stock_market_fund_flow);
macro_test!(test_extra_stock_gpzy_profile, stock_gpzy_profile);
macro_test!(test_extra_stock_gpzy_pledge_ratio, stock_gpzy_pledge_ratio);
macro_test!(
    test_extra_stock_gpzy_pledge_detail,
    stock_gpzy_pledge_detail
);
macro_test!(
    test_extra_stock_gpzy_distribute_statistics_bank,
    stock_gpzy_distribute_statistics_bank
);
macro_test!(
    test_extra_stock_gpzy_distribute_statistics_company,
    stock_gpzy_distribute_statistics_company
);
macro_test!(
    test_extra_stock_gpzy_industry_data,
    stock_gpzy_industry_data
);
macro_test!(
    test_extra_stock_gpzy_pledge_ratio_detail,
    stock_gpzy_pledge_ratio_detail
);
macro_test!(
    test_extra_stock_hsgt_fund_flow_summary,
    stock_hsgt_fund_flow_summary
);
macro_test!(test_extra_stock_hk_ggt_components, stock_hk_ggt_components);
macro_test!(test_extra_stock_industry_category, stock_industry_category);
macro_test!(test_extra_stock_info_cjzc, stock_info_cjzc);
macro_test!(test_extra_stock_info_global_em, stock_info_global_em);
macro_test!(test_extra_stock_info_change_name, stock_info_change_name);
macro_test!(test_extra_stock_info_sh_delist, stock_info_sh_delist);
macro_test!(
    test_extra_stock_info_sz_change_name,
    stock_info_sz_change_name
);
macro_test!(test_extra_stock_info_sz_delist, stock_info_sz_delist);
macro_test!(test_extra_stock_info_global_cls, stock_info_global_cls);
macro_test!(test_extra_stock_info_global_futu, stock_info_global_futu);
macro_test!(test_extra_stock_info_global_sina, stock_info_global_sina);
macro_test!(test_extra_stock_info_global_ths, stock_info_global_ths);
macro_test!(test_extra_stock_inner_trade_xq, stock_inner_trade_xq);
macro_test!(test_extra_stock_lh_yyb_most, stock_lh_yyb_most);
macro_test!(test_extra_stock_lh_yyb_capital, stock_lh_yyb_capital);
macro_test!(test_extra_stock_lh_yyb_control, stock_lh_yyb_control);
macro_test!(test_extra_stock_lhb_ggtj, stock_lhb_ggtj);
macro_test!(test_extra_stock_lhb_jgmx, stock_lhb_jgmx);
macro_test!(
    test_extra_stock_margin_account_info,
    stock_margin_account_info
);
macro_test!(test_extra_stock_rank_cxd, stock_rank_cxd);
macro_test!(test_extra_stock_rank_cxfl, stock_rank_cxfl);
macro_test!(test_extra_stock_rank_cxg, stock_rank_cxg);
macro_test!(test_extra_stock_rank_cxsl, stock_rank_cxsl);
macro_test!(test_extra_stock_rank_lxsz, stock_rank_lxsz);
macro_test!(test_extra_stock_rank_lxxd, stock_rank_lxxd);
macro_test!(test_extra_stock_rank_ljqd, stock_rank_ljqd);
macro_test!(test_extra_stock_rank_ljqs, stock_rank_ljqs);
macro_test!(test_extra_stock_rank_xstp, stock_rank_xstp);
macro_test!(test_extra_stock_rank_xxtp, stock_rank_xxtp);
macro_test!(test_extra_stock_rank_xzjp, stock_rank_xzjp);
macro_test!(test_extra_stock_register_all, stock_register_all);
macro_test!(test_extra_stock_register_bj, stock_register_bj);
macro_test!(test_extra_stock_register_cyb, stock_register_cyb);
macro_test!(test_extra_stock_register_db, stock_register_db);
macro_test!(test_extra_stock_register_kcb, stock_register_kcb);
macro_test!(test_extra_stock_register_sh, stock_register_sh);
macro_test!(test_extra_stock_register_sz, stock_register_sz);
macro_test!(test_extra_stock_zh_a_spot_em, stock_zh_a_spot_em);
macro_test!(test_extra_stock_sh_a_spot, stock_sh_a_spot);
macro_test!(test_extra_stock_sz_a_spot, stock_sz_a_spot);
macro_test!(test_extra_stock_bj_a_spot, stock_bj_a_spot);
macro_test!(test_extra_stock_new_a_spot, stock_new_a_spot);
macro_test!(test_extra_stock_cy_a_spot, stock_cy_a_spot);
macro_test!(test_extra_stock_kc_a_spot, stock_kc_a_spot);
macro_test!(test_extra_stock_zh_ab_comparison, stock_zh_ab_comparison);
macro_test!(test_extra_stock_zh_b_spot_em, stock_zh_b_spot_em);
macro_test!(test_extra_stock_hk_spot_em, stock_hk_spot_em);
macro_test!(
    test_extra_stock_hk_main_board_spot,
    stock_hk_main_board_spot
);
macro_test!(test_extra_stock_us_spot_em, stock_us_spot_em);
macro_test!(test_extra_stock_info_a_code_name, stock_info_a_code_name);
macro_test!(
    test_extra_stock_a_below_net_asset_statistics,
    stock_a_below_net_asset_statistics
);
macro_test!(
    test_extra_stock_a_high_low_statistics,
    stock_a_high_low_statistics
);
macro_test!(
    test_extra_stock_market_activity_legu,
    stock_market_activity_legu
);
macro_test!(
    test_extra_stock_sgt_reference_exchange_rate_sse,
    stock_sgt_reference_exchange_rate_sse
);
macro_test!(
    test_extra_stock_sgt_reference_exchange_rate_szse,
    stock_sgt_reference_exchange_rate_szse
);
macro_test!(
    test_extra_stock_sgt_settlement_exchange_rate_sse,
    stock_sgt_settlement_exchange_rate_sse
);
macro_test!(
    test_extra_stock_sgt_settlement_exchange_rate_szse,
    stock_sgt_settlement_exchange_rate_szse
);
macro_test!(test_extra_stock_sy_profile, stock_sy_profile);
macro_test!(test_extra_stock_qbzf, stock_qbzf);
macro_test!(test_extra_stock_pg, stock_pg);

// === 1-arg functions ===
macro_test_arg1!(test_extra_a_share_quote, a_share_quote, "600000");
macro_test_arg1!(
    test_extra_a_share_announcement_detail,
    a_share_announcement_detail,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_board_concept_cons_em,
    stock_board_concept_cons_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_board_concept_index_em,
    stock_board_concept_index_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_board_industry_cons_em,
    stock_board_industry_cons_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_board_industry_index_em,
    stock_board_industry_index_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_board_concept_spot,
    stock_board_concept_spot,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_board_industry_spot,
    stock_board_industry_spot,
    "600000"
);
macro_test_arg1!(test_extra_stock_board_change, stock_board_change, "600000");
macro_test_arg1!(
    test_extra_stock_board_concept_info,
    stock_board_concept_info,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_board_concept_summary,
    stock_board_concept_summary,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_board_industry_info,
    stock_board_industry_info,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_board_industry_summary,
    stock_board_industry_summary,
    "600000"
);
macro_test_arg1!(test_extra_stock_bid_ask, stock_bid_ask, "600000");
macro_test_arg1!(test_extra_stock_intraday_em, stock_intraday_em, "600000");
macro_test_arg1!(
    test_extra_stock_individual_info,
    stock_individual_info,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_security_profile,
    stock_hk_security_profile,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_company_profile,
    stock_hk_company_profile,
    "600000"
);
macro_test_arg1!(test_extra_stock_hot_rank, stock_hot_rank, 10);
macro_test_arg1!(
    test_extra_stock_hot_rank_detail,
    stock_hot_rank_detail,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hot_rank_detail_realtime,
    stock_hot_rank_detail_realtime,
    "600000"
);
macro_test_arg1!(test_extra_stock_hot_keyword, stock_hot_keyword, "600000");
macro_test_arg1!(test_extra_stock_hot_up, stock_hot_up, 10);
macro_test_arg1!(
    test_extra_stock_hot_rank_latest,
    stock_hot_rank_latest,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hot_rank_relate,
    stock_hot_rank_relate,
    "600000"
);
macro_test_arg1!(test_extra_stock_hsgt_kamt_flow, stock_hsgt_kamt_flow, 10);
macro_test_arg1!(
    test_extra_stock_hsgt_north_net_flow_kamt,
    stock_hsgt_north_net_flow_kamt,
    10
);
macro_test_arg1!(
    test_extra_stock_hsgt_south_net_flow_kamt,
    stock_hsgt_south_net_flow_kamt,
    10
);
macro_test_arg1!(test_extra_stock_dzjy_sctj, stock_dzjy_sctj, 10);
macro_test_arg1!(test_extra_stock_repurchase, stock_repurchase, 10);
macro_test_arg1!(test_extra_stock_gsrl_gsdt, stock_gsrl_gsdt, "20240101");
macro_test_arg1!(test_extra_stock_sse_summary, stock_sse_summary, "20240101");
macro_test_arg1!(
    test_extra_stock_zh_growth_comparison,
    stock_zh_growth_comparison,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_zh_valuation_comparison,
    stock_zh_valuation_comparison,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_growth_comparison,
    stock_hk_growth_comparison,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_valuation_comparison,
    stock_hk_valuation_comparison,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_zh_a_spot_em_flex,
    stock_zh_a_spot_em_flex,
    10
);
macro_test_arg1!(test_extra_stock_zh_a_st, stock_zh_a_st, 10);
macro_test_arg1!(test_extra_stock_zh_a_new_em, stock_zh_a_new_em, 10);
macro_test_arg1!(test_extra_stock_staq_net_stop, stock_staq_net_stop, 10);
macro_test_arg1!(test_extra_stock_hk_spot_em_flex, stock_hk_spot_em_flex, 10);
macro_test_arg1!(test_extra_stock_us_spot_em_flex, stock_us_spot_em_flex, 10);
macro_test_arg1!(
    test_extra_stock_board_concept_name_em,
    stock_board_concept_name_em,
    10
);
macro_test_arg1!(
    test_extra_stock_board_industry_name_em,
    stock_board_industry_name_em,
    10
);
macro_test_arg1!(test_extra_stock_zh_ah_spot_em, stock_zh_ah_spot_em, 10);
macro_test_arg1!(test_extra_stock_hsgt_sh_hk_spot, stock_hsgt_sh_hk_spot, 10);
macro_test_arg1!(test_extra_stock_hsgt_sz_hk_spot, stock_hsgt_sz_hk_spot, 10);
macro_test_arg1!(test_extra_hk_quote, hk_quote, "600000");
macro_test_arg1!(
    test_extra_stock_hk_index_daily_em,
    stock_hk_index_daily_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_index_daily_sina,
    stock_hk_index_daily_sina,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_hot_rank_latest,
    stock_hk_hot_rank_latest,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_hot_rank_detail,
    stock_hk_hot_rank_detail,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_hot_rank_detail_realtime,
    stock_hk_hot_rank_detail_realtime,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_scale_comparison,
    stock_hk_scale_comparison,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_dividend_payout,
    stock_hk_dividend_payout,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_fhpx_detail,
    stock_hk_fhpx_detail,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_financial_indicator,
    stock_hk_financial_indicator,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hk_indicator_eniu,
    stock_hk_indicator_eniu,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_js_weibo_report,
    stock_js_weibo_report,
    "600000"
);
macro_test_arg1!(test_extra_stock_sector_spot, stock_sector_spot, "600000");
macro_test_arg1!(
    test_extra_stock_billboard_details_em,
    stock_billboard_details_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_billboard_statistic_em,
    stock_billboard_statistic_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_billboard_org_statistic_em,
    stock_billboard_org_statistic_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_margin_detail_sse_dc,
    stock_margin_detail_sse_dc,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_margin_detail_szse_dc,
    stock_margin_detail_szse_dc,
    "600000"
);
macro_test_arg1!(test_extra_stock_dzjy_mdetail, stock_dzjy_mdetail, "600000");
macro_test_arg1!(
    test_extra_stock_dzjy_detail_cls,
    stock_dzjy_detail_cls,
    "600000"
);
macro_test_arg1!(test_extra_us_quote, us_quote, "600000");
macro_test_arg1!(
    test_extra_stock_us_famous_spot,
    stock_us_famous_spot,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_individual_spot_xq,
    stock_individual_spot_xq,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_zh_dupont_comparison,
    stock_zh_dupont_comparison,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_zh_scale_comparison,
    stock_zh_scale_comparison,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_zh_index_value_csindex,
    stock_zh_index_value_csindex,
    "600000"
);
macro_test_arg1!(test_extra_stock_register, stock_register, "600000");
macro_test_arg1!(
    test_extra_stock_restricted_release_queue_em,
    stock_restricted_release_queue_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_profit_forecast_em,
    stock_profit_forecast_em,
    "600000"
);
macro_test_arg1!(test_extra_stock_zh_a_gbjg, stock_zh_a_gbjg, "600000");
macro_test_arg1!(test_extra_stock_zygc, stock_zygc, "600000");
macro_test_arg1!(
    test_extra_stock_financial_abstract,
    stock_financial_abstract,
    "600000"
);
macro_test_arg1!(test_extra_stock_ipo_info, stock_ipo_info, "600000");
macro_test_arg1!(test_extra_stock_add_stock, stock_add_stock, "600000");
macro_test_arg1!(
    test_extra_stock_restricted_release_queue_sina,
    stock_restricted_release_queue_sina,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_circulate_stock_holder,
    stock_circulate_stock_holder,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_fund_stock_holder,
    stock_fund_stock_holder,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_main_stock_holder,
    stock_main_stock_holder,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_institute_hold,
    stock_institute_hold,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_institute_recommend,
    stock_institute_recommend,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_institute_recommend_detail,
    stock_institute_recommend_detail,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_management_change,
    stock_management_change,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_shareholder_change,
    stock_shareholder_change,
    "600000"
);
macro_test_arg1!(test_extra_stock_ipo, stock_ipo, "600000");
macro_test_arg1!(test_extra_stock_zyjs, stock_zyjs, "600000");
macro_test_arg1!(test_extra_stock_analyst_rank, stock_analyst_rank, "600000");
macro_test_arg1!(
    test_extra_stock_comment_detail_zlkp_jgcyd,
    stock_comment_detail_zlkp_jgcyd,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_comment_detail_zhpj_lspf,
    stock_comment_detail_zhpj_lspf,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_comment_detail_scrd_focus,
    stock_comment_detail_scrd_focus,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_comment_detail_scrd_desire,
    stock_comment_detail_scrd_desire,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_zh_a_disclosure_relation,
    stock_zh_a_disclosure_relation,
    "600000"
);
macro_test_arg1!(test_extra_stock_xgsglb, stock_xgsglb, "600000");
macro_test_arg1!(test_extra_stock_fhps, stock_fhps, "600000");
macro_test_arg1!(
    test_extra_stock_fhps_detail_em,
    stock_fhps_detail_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_balance_sheet_by_report_em_typed,
    stock_balance_sheet_by_report_em_typed,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_profit_sheet_by_report_em_typed,
    stock_profit_sheet_by_report_em_typed,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_cash_flow_sheet_by_report_em_typed,
    stock_cash_flow_sheet_by_report_em_typed,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_fund_flow_individual,
    stock_fund_flow_individual,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_fund_flow_concept,
    stock_fund_flow_concept,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_fund_flow_industry,
    stock_fund_flow_industry,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_fund_flow_big_deal,
    stock_fund_flow_big_deal,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_sector_fund_flow_hist,
    stock_sector_fund_flow_hist,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_sector_fund_flow_rank,
    stock_sector_fund_flow_rank,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_main_fund_flow,
    stock_main_fund_flow,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_concept_fund_flow_hist,
    stock_concept_fund_flow_hist,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_gdfx_free_holding_statistics,
    stock_gdfx_free_holding_statistics,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_gdfx_holding_statistics,
    stock_gdfx_holding_statistics,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_gdfx_free_holding_change,
    stock_gdfx_free_holding_change,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_gdfx_holding_change,
    stock_gdfx_holding_change,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_gdfx_free_holding_detail,
    stock_gdfx_free_holding_detail,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_gdfx_free_holding_analyse,
    stock_gdfx_free_holding_analyse,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_gdfx_holding_analyse,
    stock_gdfx_holding_analyse,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_gdfx_free_holding_teamwork,
    stock_gdfx_free_holding_teamwork,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_gdfx_holding_teamwork,
    stock_gdfx_holding_teamwork,
    "600000"
);
macro_test_arg1!(test_extra_stock_hold_change, stock_hold_change, "600000");
macro_test_arg1!(test_extra_stock_hold_control, stock_hold_control, "600000");
macro_test_arg1!(
    test_extra_stock_hold_management_detail_cninfo,
    stock_hold_management_detail_cninfo,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hold_management_detail_em,
    stock_hold_management_detail_em,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hold_management_person,
    stock_hold_management_person,
    "600000"
);
macro_test_arg1!(test_extra_stock_gdhs, stock_gdhs, "600000");
macro_test_arg1!(test_extra_stock_gdhs_detail, stock_gdhs_detail, "600000");
macro_test_arg1!(test_extra_stock_zh_a_gdhs, stock_zh_a_gdhs, "600000");
macro_test_arg1!(
    test_extra_stock_zh_a_gdhs_detail,
    stock_zh_a_gdhs_detail,
    "600000"
);
macro_test_arg1!(test_extra_stock_ggcg, stock_ggcg, "600000");
macro_test_arg1!(
    test_extra_stock_gpzy_individual_pledge_ratio_detail,
    stock_gpzy_individual_pledge_ratio_detail,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hot_follow_xq,
    stock_hot_follow_xq,
    "600000"
);
macro_test_arg1!(test_extra_stock_hot_tweet_xq, stock_hot_tweet_xq, "600000");
macro_test_arg1!(test_extra_stock_hot_deal_xq, stock_hot_deal_xq, "600000");
macro_test_arg1!(test_extra_stock_hsgt_hist, stock_hsgt_hist, "600000");
macro_test_arg1!(
    test_extra_stock_hsgt_fund_min,
    stock_hsgt_fund_min,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_hsgt_individual,
    stock_hsgt_individual,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_industry_change,
    stock_industry_change,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_industry_clf_hist_sw,
    stock_industry_clf_hist_sw,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_industry_pe_ratio,
    stock_industry_pe_ratio,
    "600000"
);
macro_test_arg1!(test_extra_stock_irm, stock_irm, "600000");
macro_test_arg1!(test_extra_stock_irm_ans, stock_irm_ans, "600000");
macro_test_arg1!(test_extra_stock_jgdy_tj, stock_jgdy_tj, "600000");
macro_test_arg1!(test_extra_stock_jgdy_detail, stock_jgdy_detail, "600000");
macro_test_arg1!(
    test_extra_stock_lhb_stock_statistic,
    stock_lhb_stock_statistic,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_lhb_jgstatistic,
    stock_lhb_jgstatistic,
    "600000"
);
macro_test_arg1!(test_extra_stock_lhb_yybph, stock_lhb_yybph, "600000");
macro_test_arg1!(
    test_extra_stock_lhb_traderstatistic,
    stock_lhb_traderstatistic,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_lhb_stock_detail_date,
    stock_lhb_stock_detail_date,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_lhb_yyb_detail,
    stock_lhb_yyb_detail,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_lhb_detail_daily,
    stock_lhb_detail_daily,
    "600000"
);
macro_test_arg1!(test_extra_stock_lhb_jgzz, stock_lhb_jgzz, "600000");
macro_test_arg1!(test_extra_stock_lhb_yytj, stock_lhb_yytj, "600000");
macro_test_arg1!(
    test_extra_stock_margin_detail_sse,
    stock_margin_detail_sse,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_margin_detail_szse,
    stock_margin_detail_szse,
    "600000"
);
macro_test_arg1!(test_extra_stock_margin_szse, stock_margin_szse, "600000");
macro_test_arg1!(
    test_extra_stock_margin_underlying_info,
    stock_margin_underlying_info,
    "600000"
);
macro_test_arg1!(test_extra_stock_changes, stock_changes, "600000");
macro_test_arg1!(
    test_extra_stock_rank_forecast,
    stock_rank_forecast,
    "600000"
);
macro_test_arg1!(test_extra_stock_zcfz, stock_zcfz, "600000");
macro_test_arg1!(test_extra_stock_lrb, stock_lrb, "600000");
macro_test_arg1!(test_extra_stock_xjll, stock_xjll, "600000");
macro_test_arg1!(test_extra_stock_news, stock_news, "600000");
macro_test_arg1!(test_extra_stock_sy_yq, stock_sy_yq, "600000");
macro_test_arg1!(test_extra_stock_sy_jz, stock_sy_jz, "600000");
macro_test_arg1!(test_extra_stock_sy, stock_sy, "600000");
macro_test_arg1!(
    test_extra_stock_balance_sheet_by_report,
    stock_balance_sheet_by_report,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_balance_sheet_by_yearly,
    stock_balance_sheet_by_yearly,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_profit_sheet_by_report,
    stock_profit_sheet_by_report,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_profit_sheet_by_yearly,
    stock_profit_sheet_by_yearly,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_profit_sheet_by_quarterly,
    stock_profit_sheet_by_quarterly,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_cash_flow_sheet_by_report,
    stock_cash_flow_sheet_by_report,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_cash_flow_sheet_by_yearly,
    stock_cash_flow_sheet_by_yearly,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_cash_flow_sheet_by_quarterly,
    stock_cash_flow_sheet_by_quarterly,
    "600000"
);
macro_test_arg1!(test_extra_stock_yjbb, stock_yjbb, "600000");
macro_test_arg1!(test_extra_stock_yjkb, stock_yjkb, "600000");
macro_test_arg1!(test_extra_stock_yjyg, stock_yjyg, "600000");
macro_test_arg1!(test_extra_stock_zt_pool, stock_zt_pool, "600000");
macro_test_arg1!(test_extra_stock_zt_pool_dtgc, stock_zt_pool_dtgc, "600000");
macro_test_arg1!(
    test_extra_stock_zt_pool_previous,
    stock_zt_pool_previous,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_zt_pool_strong,
    stock_zt_pool_strong,
    "600000"
);
macro_test_arg1!(
    test_extra_stock_zt_pool_sub_new,
    stock_zt_pool_sub_new,
    "600000"
);
macro_test_arg1!(test_extra_stock_zt_pool_zbgc, stock_zt_pool_zbgc, "600000");

// === 2-arg functions ===
macro_test_arg2!(
    test_extra_a_share_capital_flow,
    a_share_capital_flow,
    "600000",
    10
);
macro_test_arg2!(
    test_extra_a_share_sector_rankings,
    a_share_sector_rankings,
    "600000",
    10
);
macro_test_arg2!(
    test_extra_a_share_sector_constituents,
    a_share_sector_constituents,
    "600000",
    10
);
macro_test_arg2!(
    test_extra_a_share_sector_capital_flow,
    a_share_sector_capital_flow,
    "600000",
    10
);
macro_test_arg2!(
    test_extra_a_share_billboard,
    a_share_billboard,
    "600000",
    10
);
macro_test_arg2!(
    test_extra_a_share_announcements,
    a_share_announcements,
    "600000",
    10
);
macro_test_arg2!(
    test_extra_stock_board_concept_hist_min,
    stock_board_concept_hist_min,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_board_industry_hist_min,
    stock_board_industry_hist_min,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_individual_fund_flow_rank,
    stock_individual_fund_flow_rank,
    "600000",
    10
);
macro_test_arg2!(
    test_extra_stock_hot_search,
    stock_hot_search,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_board_concept_cons,
    stock_board_concept_cons,
    "600000",
    10
);
macro_test_arg2!(
    test_extra_stock_board_industry_cons,
    stock_board_industry_cons,
    "600000",
    10
);
macro_test_arg2!(test_extra_hk_candles, hk_candles, "600000", 10);
macro_test_arg2!(
    test_extra_stock_billboard_org_detail_em,
    stock_billboard_org_detail_em,
    "600000",
    "600000"
);
macro_test_arg2!(test_extra_us_candles, us_candles, "600000", 10);
macro_test_arg2!(
    test_extra_stock_individual_basic_info_xq,
    stock_individual_basic_info_xq,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_individual_basic_info_us_xq,
    stock_individual_basic_info_us_xq,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_individual_basic_info_hk_xq,
    stock_individual_basic_info_hk_xq,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_zh_a_minute,
    stock_zh_a_minute,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_zh_a_tick_tx_js,
    stock_zh_a_tick_tx_js,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_zh_b_minute,
    stock_zh_b_minute,
    "600000",
    "600000"
);
macro_test_arg2!(test_extra_stock_zh_vote, stock_zh_vote, "600000", "600000");
macro_test_arg2!(test_extra_stock_zh_kcb_report, stock_zh_kcb_report, 1, 1);
macro_test_arg2!(
    test_extra_stock_financial_analysis_indicator_em,
    stock_financial_analysis_indicator_em,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_hk_analysis_indicator,
    stock_financial_hk_analysis_indicator,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_us_analysis_indicator,
    stock_financial_us_analysis_indicator,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_restricted_release_detail,
    stock_restricted_release_detail,
    "20240101",
    "20240131"
);
macro_test_arg2!(
    test_extra_stock_restricted_release_stockholder,
    stock_restricted_release_stockholder,
    "600000",
    "20240101"
);
macro_test_arg2!(
    test_extra_stock_notice_report,
    stock_notice_report,
    "公告",
    "20240101"
);
macro_test_arg2!(
    test_extra_stock_financial_report,
    stock_financial_report,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_analysis_indicator,
    stock_financial_analysis_indicator,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_institute_hold_detail,
    stock_institute_hold_detail,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_abstract_ths,
    stock_financial_abstract_ths,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_debt,
    stock_financial_debt,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_benefit,
    stock_financial_benefit,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_cash,
    stock_financial_cash,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_abstract_new,
    stock_financial_abstract_new,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_debt_new,
    stock_financial_debt_new,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_benefit_new,
    stock_financial_benefit_new,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_financial_cash_new,
    stock_financial_cash_new,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_profit_forecast_ths,
    stock_profit_forecast_ths,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_hk_profit_forecast_et,
    stock_hk_profit_forecast_et,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_analyst_detail,
    stock_analyst_detail,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_dzjy_mrtj,
    stock_dzjy_mrtj,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_dzjy_hygtj,
    stock_dzjy_hygtj,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_dzjy_hyyybtj,
    stock_dzjy_hyyybtj,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_dzjy_yybph,
    stock_dzjy_yybph,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_gdfx_free_top_10,
    stock_gdfx_free_top_10,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_gdfx_top_10,
    stock_gdfx_top_10,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_hold_num,
    stock_hold_num,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_hsgt_hold_stock,
    stock_hsgt_hold_stock,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_hsgt_board_rank,
    stock_hsgt_board_rank,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_hsgt_individual_detail,
    stock_hsgt_individual_detail,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_lhb_detail,
    stock_lhb_detail,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_lhb_jgmmtj,
    stock_lhb_jgmmtj,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_lhb_hyyyb,
    stock_lhb_hyyyb,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_margin_account_info_em,
    stock_margin_account_info_em,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_margin_ratio_pa,
    stock_margin_ratio_pa,
    "600000",
    "600000"
);
macro_test_arg2!(
    test_extra_stock_margin_sse,
    stock_margin_sse,
    "600000",
    "600000"
);
macro_test_arg2!(test_extra_stock_yysj, stock_yysj, "600000", "600000");

// === 3-arg functions ===
macro_test_arg3!(
    test_extra_a_share_candles,
    a_share_candles,
    "600000",
    "600000",
    10
);
macro_test_arg3!(
    test_extra_a_share_search,
    a_share_search,
    "600000",
    None,
    10
);
macro_test_arg3!(
    test_extra_a_share_billboard_seats,
    a_share_billboard_seats,
    "600000",
    "600000",
    10
);
macro_test_arg3!(
    test_extra_a_share_trade_calendar,
    a_share_trade_calendar,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_board_concept_index,
    stock_board_concept_index,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_board_industry_index,
    stock_board_industry_index,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_individual_fund_flow,
    stock_individual_fund_flow,
    "600000",
    "600000",
    10
);
macro_test_arg3!(
    test_extra_stock_report_fund_hold,
    stock_report_fund_hold,
    "600000",
    "600000",
    10
);
macro_test_arg3!(
    test_extra_stock_hk_valuation,
    stock_hk_valuation,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_intraday_sina,
    stock_intraday_sina,
    "600000",
    "20240101",
    10
);
macro_test_arg3!(
    test_extra_stock_us_daily,
    stock_us_daily,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_us_valuation,
    stock_us_valuation,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_zh_a_cdr_daily,
    stock_zh_a_cdr_daily,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_zh_a_hist_pre_min,
    stock_zh_a_hist_pre_min,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_zh_valuation,
    stock_zh_valuation,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_zh_index_daily_em,
    stock_zh_index_daily_em,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_zh_index_daily_tx,
    stock_zh_index_daily_tx,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_financial_hk_report,
    stock_financial_hk_report,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_financial_us_report,
    stock_financial_us_report,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_restricted_release_summary,
    stock_restricted_release_summary,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_history_dividend_detail,
    stock_history_dividend_detail,
    "600000",
    "600000",
    None
);
macro_test_arg3!(
    test_extra_stock_gdfx_holding_detail,
    stock_gdfx_holding_detail,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_hsgt_stock_statistics,
    stock_hsgt_stock_statistics,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_hsgt_institution_statistics,
    stock_hsgt_institution_statistics,
    "600000",
    "600000",
    "600000"
);
macro_test_arg3!(
    test_extra_stock_lhb_stock_detail,
    stock_lhb_stock_detail,
    "600000",
    "600000",
    "600000"
);

// === 4-arg functions ===
macro_test_arg4!(
    test_extra_stock_dzjy_mrmx,
    stock_dzjy_mrmx,
    "600000",
    "600000",
    "600000",
    10
);
macro_test_arg4!(
    test_extra_stock_hk_daily,
    stock_hk_daily,
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg4!(
    test_extra_stock_zh_a_daily,
    stock_zh_a_daily,
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg4!(
    test_extra_stock_zh_a_hist_tx,
    stock_zh_a_hist_tx,
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg4!(
    test_extra_stock_zh_ah_daily,
    stock_zh_ah_daily,
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg4!(
    test_extra_stock_zh_b_daily,
    stock_zh_b_daily,
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg4!(
    test_extra_stock_zh_kcb_daily,
    stock_zh_kcb_daily,
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg4!(
    test_extra_stock_individual_notice_report,
    stock_individual_notice_report,
    "600000",
    "600000",
    None,
    None
);
macro_test_arg4!(
    test_extra_stock_zh_a_disclosure_report,
    stock_zh_a_disclosure_report,
    "600000",
    "600000",
    "600000",
    "600000"
);

// === 5-arg functions ===
macro_test_arg5!(
    test_extra_stock_board_concept_hist_em,
    stock_board_concept_hist_em,
    "600000",
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg5!(
    test_extra_stock_board_industry_hist_em,
    stock_board_industry_hist_em,
    "600000",
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg5!(
    test_extra_stock_board_concept_hist,
    stock_board_concept_hist,
    "600000",
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg5!(
    test_extra_stock_board_industry_hist,
    stock_board_industry_hist,
    "600000",
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg5!(
    test_extra_stock_zh_a_hist,
    stock_zh_a_hist,
    "600000",
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg5!(
    test_extra_stock_zh_a_hist_min,
    stock_zh_a_hist_min,
    "600000",
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg5!(
    test_extra_stock_hk_hist,
    stock_hk_hist,
    "600000",
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg5!(
    test_extra_stock_hk_hist_min,
    stock_hk_hist_min,
    "600000",
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg5!(
    test_extra_stock_us_hist,
    stock_us_hist,
    "600000",
    "600000",
    "600000",
    "600000",
    "600000"
);
macro_test_arg5!(
    test_extra_stock_us_hist_min,
    stock_us_hist_min,
    "600000",
    "600000",
    "600000",
    "600000",
    "600000"
);
