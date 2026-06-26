# akshare-rs Public Function Inventory

Generated from: crates/akshare/src/
Total pub async fn: 1535

---

## stock/

### stock/a_share.rs  | fns=12  | tests=no  | source=TENCENT
    a_share_quote
    a_share_candles
    a_share_search
    a_share_capital_flow
    a_share_sector_rankings
    a_share_sector_constituents
    a_share_sector_capital_flow
    a_share_billboard
    a_share_billboard_seats
    a_share_announcements
    a_share_announcement_detail
    a_share_trade_calendar

### stock/board_em.rs  | fns=7  | tests=no  | source=EASTMONEY
    stock_board_concept_hist
    stock_board_concept_hist_min
    stock_board_concept_spot
    stock_board_industry_hist
    stock_board_industry_hist_min
    stock_board_industry_spot
    stock_board_change

### stock/board_ths.rs  | fns=8  | tests=no  | source=THS
    stock_board_concept_name_ths
    stock_board_concept_info
    stock_board_concept_index
    stock_board_concept_summary
    stock_board_industry_name_ths
    stock_board_industry_info
    stock_board_industry_index
    stock_board_industry_summary

### stock/eastmoney_detail.rs  | fns=7  | tests=no  | source=EASTMONEY
    stock_bid_ask
    stock_intraday_em
    stock_individual_info
    stock_individual_info_em_by_secid
    stock_info_by_secid
    stock_hk_security_profile
    stock_hk_company_profile

### stock/eastmoney_fund_flow.rs  | fns=2  | tests=no  | source=EASTMONEY
    stock_individual_fund_flow
    stock_individual_fund_flow_rank

### stock/eastmoney_hot.rs  | fns=8  | tests=no  | source=EASTMONEY
    stock_hot_rank
    stock_hot_rank_detail
    stock_hot_rank_detail_realtime
    stock_hot_keyword
    stock_hot_up
    stock_hot_rank_latest
    stock_hot_rank_relate
    stock_hot_search

### stock/eastmoney_hsgt.rs  | fns=3  | tests=no  | source=EASTMONEY
    stock_hsgt_kamt_flow
    stock_hsgt_north_net_flow_kamt
    stock_hsgt_south_net_flow_kamt

### stock/eastmoney_misc.rs  | fns=14  | tests=no  | source=EASTMONEY
    stock_dzjy_sctj
    stock_dzjy_mrmx
    stock_repurchase
    stock_gsrl_gsdt
    stock_report_fund_hold
    stock_sse_summary
    stock_zh_growth_comparison
    stock_zh_valuation_comparison
    stock_hk_growth_comparison
    stock_hk_valuation_comparison
    stock_us_growth_comparison
    stock_us_valuation_comparison
    stock_zh_a_financial_indicator
    stock_zh_a_dividend_payout

### stock/eastmoney_spot.rs  | fns=13  | tests=no  | source=EASTMONEY
    stock_zh_a_spot_em_flex
    stock_zh_a_st
    stock_zh_a_new_em
    stock_staq_net_stop
    stock_hk_spot_em_flex
    stock_us_spot_em_flex
    stock_board_concept_name_em
    stock_board_concept_cons
    stock_board_industry_name_em
    stock_board_industry_cons
    stock_zh_ah_spot_em
    stock_hsgt_sh_hk_spot
    stock_hsgt_sz_hk_spot

### stock/feature/analyst_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    stock_analyst_rank
    stock_analyst_detail

### stock/feature/comment_em.rs  | fns=5  | tests=no  | source=EASTMONEY
    stock_comment
    stock_comment_detail_zlkp_jgcyd
    stock_comment_detail_zhpj_lspf
    stock_comment_detail_scrd_focus
    stock_comment_detail_scrd_desire

### stock/feature/disclosure_cninfo.rs  | fns=2  | tests=no  | source=(see file)
    stock_zh_a_disclosure_report
    stock_zh_a_disclosure_relation

### stock/feature/dxsyl_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    stock_dxsyl
    stock_xgsglb

### stock/feature/dzjy_em.rs  | fns=4  | tests=no  | source=EASTMONEY
    stock_dzjy_mrtj
    stock_dzjy_hygtj
    stock_dzjy_hyyybtj
    stock_dzjy_yybph

### stock/feature/esg_sina.rs  | fns=4  | tests=no  | source=SINA
    stock_esg_msci
    stock_esg_rft
    stock_esg_zd
    stock_esg_hz

### stock/feature/fhps_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    stock_fhps
    stock_fhps_detail_em

### stock/feature/financial_em.rs  | fns=3  | tests=no  | source=EASTMONEY
    stock_balance_sheet_by_report_em_typed
    stock_profit_sheet_by_report_em_typed
    stock_cash_flow_sheet_by_report_em_typed

### stock/feature/fund_flow.rs  | fns=10  | tests=no  | source=EASTMONEY
    stock_fund_flow_individual
    stock_fund_flow_concept
    stock_fund_flow_industry
    stock_fund_flow_big_deal
    stock_sector_fund_flow_hist
    stock_sector_fund_flow_rank
    stock_sector_fund_flow_summary
    stock_main_fund_flow
    stock_market_fund_flow
    stock_concept_fund_flow_hist

### stock/feature/gdfx_em.rs  | fns=18  | tests=no  | source=EASTMONEY
    stock_gdfx_free_holding_statistics
    stock_gdfx_holding_statistics
    stock_gdfx_free_holding_change
    stock_gdfx_holding_change
    stock_gdfx_free_top_10
    stock_gdfx_top_10
    stock_gdfx_free_holding_detail
    stock_gdfx_holding_detail
    stock_gdfx_free_holding_analyse
    stock_gdfx_holding_analyse
    stock_gdfx_free_holding_teamwork
    stock_gdfx_holding_teamwork
    stock_hold_change
    stock_hold_control
    stock_hold_management_detail_cninfo
    stock_hold_management_detail_em
    stock_hold_management_person
    stock_hold_num

### stock/feature/gdhs_em.rs  | fns=4  | tests=no  | source=EASTMONEY
    stock_gdhs
    stock_gdhs_detail
    stock_zh_a_gdhs
    stock_zh_a_gdhs_detail

### stock/feature/gdzjc_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    stock_ggcg

### stock/feature/gpzy_em.rs  | fns=8  | tests=no  | source=EASTMONEY
    stock_gpzy_profile
    stock_gpzy_pledge_ratio
    stock_gpzy_pledge_detail
    stock_gpzy_distribute_statistics_bank
    stock_gpzy_distribute_statistics_company
    stock_gpzy_individual_pledge_ratio_detail
    stock_gpzy_industry_data
    stock_gpzy_pledge_ratio_detail

### stock/feature/hist_em.rs  | fns=6  | tests=no  | source=EASTMONEY
    stock_zh_a_hist
    stock_zh_a_hist_min
    stock_hk_hist
    stock_hk_hist_min
    stock_us_hist
    stock_us_hist_min

### stock/feature/hot_xq.rs  | fns=3  | tests=no  | source=XUEQIU
    stock_hot_follow_xq
    stock_hot_tweet_xq
    stock_hot_deal_xq

### stock/feature/hsgt_em.rs  | fns=10  | tests=no  | source=EASTMONEY
    stock_hsgt_fund_flow_summary
    stock_hk_ggt_components
    stock_hsgt_hold_stock
    stock_hsgt_stock_statistics
    stock_hsgt_institution_statistics
    stock_hsgt_hist
    stock_hsgt_board_rank
    stock_hsgt_individual_detail
    stock_hsgt_fund_min
    stock_hsgt_individual

### stock/feature/industry_cninfo.rs  | fns=4  | tests=no  | source=CNINFO
    stock_industry_category
    stock_industry_change
    stock_industry_clf_hist_sw
    stock_industry_pe_ratio

### stock/feature/info_em.rs  | fns=13  | tests=no  | source=EASTMONEY
    stock_info_cjzc
    stock_info_global_em
    stock_info_change_name
    stock_info_sh_delist
    stock_info_sz_change_name
    stock_info_sz_delist
    stock_info_global_cls
    stock_info_global_futu
    stock_info_global_sina
    stock_info_global_ths
    stock_info_sh_name_code
    stock_info_sz_name_code
    stock_info_bj_name_code

### stock/feature/inner_trade_xq.rs  | fns=1  | tests=no  | source=XUEQIU
    stock_inner_trade_xq

### stock/feature/irm_cninfo.rs  | fns=2  | tests=no  | source=(see file)
    stock_irm
    stock_irm_ans

### stock/feature/jgdy_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    stock_jgdy_tj
    stock_jgdy_detail

### stock/feature/lhb_em.rs  | fns=15  | tests=no  | source=EASTMONEY
    stock_lhb_detail
    stock_lhb_stock_statistic
    stock_lhb_jgmmtj
    stock_lhb_jgstatistic
    stock_lhb_hyyyb
    stock_lhb_yybph
    stock_lhb_traderstatistic
    stock_lhb_stock_detail_date
    stock_lhb_stock_detail
    stock_lhb_yyb_detail
    stock_lhb_detail_daily
    stock_lhb_ggtj
    stock_lhb_jgmx
    stock_lhb_jgzz
    stock_lhb_yytj

### stock/feature/lh_yybpm.rs  | fns=3  | tests=no  | source=THS
    stock_lh_yyb_most
    stock_lh_yyb_capital
    stock_lh_yyb_control

### stock/feature/margin_em.rs  | fns=8  | tests=no  | source=EASTMONEY
    stock_margin_account_info_em
    stock_margin_account_info
    stock_margin_detail_sse
    stock_margin_detail_szse
    stock_margin_ratio_pa
    stock_margin_sse
    stock_margin_szse
    stock_margin_underlying_info

### stock/feature/pankou_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    stock_changes

### stock/feature/rank_ths.rs  | fns=12  | tests=no  | source=THS
    stock_rank_cxd
    stock_rank_cxfl
    stock_rank_cxg
    stock_rank_cxsl
    stock_rank_lxsz
    stock_rank_lxxd
    stock_rank_ljqd
    stock_rank_ljqs
    stock_rank_xstp
    stock_rank_xxtp
    stock_rank_xzjp
    stock_rank_forecast

### stock/feature/register_em.rs  | fns=7  | tests=no  | source=EASTMONEY
    stock_register_all
    stock_register_bj
    stock_register_cyb
    stock_register_db
    stock_register_kcb
    stock_register_sh
    stock_register_sz

### stock/feature/report_em.rs  | fns=3  | tests=no  | source=EASTMONEY
    stock_zcfz
    stock_lrb
    stock_xjll

### stock/feature/spot_em.rs  | fns=12  | tests=no  | source=EASTMONEY
    stock_zh_a_spot_em
    stock_sh_a_spot
    stock_sz_a_spot
    stock_bj_a_spot
    stock_new_a_spot
    stock_cy_a_spot
    stock_kc_a_spot
    stock_zh_ab_comparison
    stock_zh_b_spot_em
    stock_hk_spot_em
    stock_hk_main_board_spot
    stock_us_spot_em

### stock/feature/stock_info.rs  | fns=1  | tests=no  | source=EASTMONEY
    stock_info_a_code_name

### stock/feature/stock_other.rs  | fns=65  | tests=no  | source=EASTMONEY
    stock_account_statistics
    stock_allotment
    stock_a_all_pb
    stock_a_below_net_asset_statistics
    stock_a_code_to_symbol
    stock_a_congestion_lg
    stock_a_gxl_lg
    stock_a_high_low_statistics
    stock_a_ttm_lyr
    stock_buffett_index_lg
    stock_classify
    stock_concept_cons_futu
    stock_cyq
    stock_dividend
    stock_ebs_lg
    stock_esg_rate
    stock_fhps_detail_ths
    stock_gddh
    stock_ipo_benefit
    stock_ipo_summary
    stock_market_activity_legu
    stock_market_pb_lg
    stock_market_pe_lg
    stock_new_gh
    stock_new_ipo
    stock_news
    stock_news_em_by_name
    stock_news_em_hk
    stock_news_em_us
    stock_news_main_cx
    stock_price_js
    stock_profile
    stock_qsjy
    stock_report_disclosure
    stock_report_fund_hold_detail
    stock_research_report
    stock_sector_detail
    stock_sgt_reference_exchange_rate_sse
    stock_sgt_reference_exchange_rate_szse
    stock_sgt_settlement_exchange_rate_sse
    stock_sgt_settlement_exchange_rate_szse
    stock_share_change
    stock_share_hold_change_bse
    stock_share_hold_change_sse
    stock_share_hold_change_szse
    stock_sns_sseinfo
    stock_sse_deal_daily
    stock_sy_hy
    stock_szse_area_summary
    stock_szse_sector_summary
    stock_szse_summary
    stock_tfp
    stock_value
    stock_xgsr
    stock_yzxdr
    stock_zdhtmx
    stock_balance_sheet_by_report_delisted
    stock_cash_flow_sheet_by_report_delisted
    stock_profit_sheet_by_report_delisted
    stock_cg_equity_mortgage
    stock_cg_guarantee
    stock_cg_lawsuit
    stock_zcfz_bj
    stock_index_pb_lg
    stock_index_pe_lg

### stock/feature/sy_em.rs  | fns=4  | tests=no  | source=EASTMONEY
    stock_sy_profile
    stock_sy_yq
    stock_sy_jz
    stock_sy

### stock/feature/three_report_em.rs  | fns=8  | tests=no  | source=EASTMONEY
    stock_balance_sheet_by_report
    stock_balance_sheet_by_yearly
    stock_profit_sheet_by_report
    stock_profit_sheet_by_yearly
    stock_profit_sheet_by_quarterly
    stock_cash_flow_sheet_by_report
    stock_cash_flow_sheet_by_yearly
    stock_cash_flow_sheet_by_quarterly

### stock/feature/yjbb_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    stock_yjbb

### stock/feature/yjyg_em.rs  | fns=3  | tests=no  | source=EASTMONEY
    stock_yjkb
    stock_yjyg
    stock_yysj

### stock/feature/zf_pg_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    stock_qbzf
    stock_pg

### stock/feature/ztb_em.rs  | fns=6  | tests=no  | source=EASTMONEY
    stock_zt_pool
    stock_zt_pool_dtgc
    stock_zt_pool_previous
    stock_zt_pool_strong
    stock_zt_pool_sub_new
    stock_zt_pool_zbgc

### stock/fundamental/eastmoney.rs  | fns=26  | tests=no  | source=EASTMONEY
    stock_financial_analysis_indicator_em
    stock_financial_hk_report
    stock_financial_hk_analysis_indicator
    stock_financial_us_report
    stock_financial_us_analysis_indicator
    stock_financial_hk_analysis_indicator_em_typed
    stock_financial_hk_balance_sheet_typed
    stock_financial_hk_income_sheet_typed
    stock_financial_hk_cashflow_sheet_typed
    stock_financial_us_analysis_indicator_em_typed
    stock_financial_us_balance_sheet_typed
    stock_financial_us_income_sheet_typed
    stock_financial_us_cashflow_sheet_typed
    stock_register
    stock_restricted_release_summary
    stock_restricted_release_detail
    stock_restricted_release_queue_em
    stock_restricted_release_stockholder
    stock_ipo_declare
    stock_ipo_review
    stock_ipo_tutor
    stock_profit_forecast_em
    stock_zh_a_gbjg
    stock_zygc
    stock_notice_report
    stock_individual_notice_report

### stock/fundamental/sina.rs  | fns=15  | tests=no  | source=SINA
    stock_financial_report
    stock_financial_abstract
    stock_history_dividend
    stock_history_dividend_detail
    stock_ipo_info
    stock_add_stock
    stock_restricted_release_queue_sina
    stock_circulate_stock_holder
    stock_fund_stock_holder
    stock_main_stock_holder
    stock_financial_analysis_indicator
    stock_institute_hold
    stock_institute_hold_detail
    stock_institute_recommend
    stock_institute_recommend_detail

### stock/fundamental/ths.rs  | fns=15  | tests=no  | source=TONGHUASHUN
    stock_financial_abstract_ths
    stock_financial_debt
    stock_financial_benefit
    stock_financial_cash
    stock_financial_abstract_new
    stock_financial_debt_new
    stock_financial_benefit_new
    stock_financial_cash_new
    stock_management_change
    stock_shareholder_change
    stock_ipo
    stock_ipo_hk
    stock_profit_forecast_ths
    stock_zyjs
    stock_hk_profit_forecast_et

### stock/hk_extra.rs  | fns=18  | tests=no  | source=SINA
    stock_hk_spot
    stock_hk_daily
    stock_hk_famous_spot
    stock_hk_index_daily_em
    stock_hk_index_daily_sina
    stock_hk_index_spot_em
    stock_hk_index_spot_sina
    stock_hk_hot_rank
    stock_hk_hot_rank_latest
    stock_hk_hot_rank_detail
    stock_hk_hot_rank_detail_realtime
    stock_hk_valuation
    stock_hk_scale_comparison
    stock_hk_dividend_payout
    stock_hk_fhpx_detail
    stock_hk_financial_indicator
    stock_hk_gxl_lg
    stock_hk_indicator_eniu

### stock/hk.rs  | fns=4  | tests=YES  | source=TENCENT
    hk_market_cap_from_tencent
    hk_quote
    hk_candles
    hk_financial

### stock/jin10.rs  | fns=2  | tests=no  | source=JIN10
    stock_js_weibo_nlp_time
    stock_js_weibo_report

### stock/sina_stock.rs  | fns=2  | tests=no  | source=SINA
    stock_intraday_sina
    stock_sector_spot

### stock/us_extra.rs  | fns=18  | tests=no  | source=SINA
    stock_us_daily
    stock_us_spot
    stock_us_famous_spot
    stock_us_pink_spot
    stock_us_valuation
    stock_us_hot_rank
    stock_us_hot_rank_latest
    stock_us_hot_rank_detail
    stock_us_hot_rank_detail_realtime
    stock_us_index_spot_em
    stock_us_index_daily_em
    stock_us_index_spot_sina
    stock_us_index_daily_sina
    stock_us_financial_indicator
    stock_us_dividend_payout
    stock_us_gxl_lg
    stock_us_scale_comparison
    stock_us_hot_keyword

### stock/us_profile.rs  | fns=3  | tests=YES  | source=YAHOO
    us_stock_profile
    us_stock_industry
    us_stock_key_stats

### stock/us.rs  | fns=4  | tests=no  | source=(see file)
    us_quote
    us_candles
    us_market_cap_from
    get_us_stock_name

### stock/xueqiu.rs  | fns=4  | tests=no  | source=XUEQIU
    stock_individual_spot_xq
    stock_individual_basic_info_xq
    stock_individual_basic_info_us_xq
    stock_individual_basic_info_hk_xq

### stock/zh_ah.rs  | fns=3  | tests=no  | source=TENCENT
    stock_zh_ah_spot
    stock_zh_ah_daily
    stock_zh_ah_name

### stock/zh_a.rs  | fns=9  | tests=no  | source=TENCENT
    stock_zh_a_spot
    stock_zh_a_daily
    stock_zh_a_minute
    stock_zh_a_new
    stock_zh_a_stop
    stock_zh_a_cdr_daily
    stock_zh_a_hist_pre_min
    stock_zh_a_hist_tx
    stock_zh_a_tick_tx_js

### stock/zh_b.rs  | fns=3  | tests=no  | source=SINA
    stock_zh_b_spot
    stock_zh_b_daily
    stock_zh_b_minute

### stock/zh_comparison.rs  | fns=4  | tests=no  | source=BAIDU
    stock_zh_dupont_comparison
    stock_zh_scale_comparison
    stock_zh_valuation
    stock_zh_vote

### stock/zh_index.rs  | fns=5  | tests=no  | source=EASTMONEY
    stock_zh_index_spot_em
    stock_zh_index_daily_em
    stock_zh_index_daily_tx
    stock_zh_index_spot_sina
    stock_zh_index_value_csindex

### stock/zh_kcb.rs  | fns=3  | tests=no  | source=SINA
    stock_zh_kcb_spot
    stock_zh_kcb_daily
    stock_zh_kcb_report


## bond/

### bond/buyback.rs  | fns=3  | tests=YES  | source=EASTMONEY
    bond_sh_buy_back
    bond_sz_buy_back
    bond_buy_back_hist

### bond/cbond_index.rs  | fns=5  | tests=YES  | source=(see file)
    bond_index_general_cbond
    bond_treasury_index_cbond
    bond_new_composite_index_cbond
    bond_available_index_cbond
    bond_composite_index_cbond

### bond/cb_sina.rs  | fns=2  | tests=no  | source=SINA
    bond_cb_profile
    bond_cb_summary

### bond/cb_ths.rs  | fns=1  | tests=YES  | source=THS
    bond_zh_cov_info_ths

### bond/china_money.rs  | fns=3  | tests=YES  | source=(see file)
    bond_china_close_return
    macro_china_swap_rate
    macro_china_bond_public

### bond/convertible.rs  | fns=2  | tests=YES  | source=EASTMONEY
    bond_convertible_list
    bond_convertible_hist

### bond/corporate.rs  | fns=1  | tests=no  | source=EASTMONEY
    bond_corporate_yields

### bond/em_rate.rs  | fns=1  | tests=YES  | source=EASTMONEY
    bond_zh_us_rate

### bond/gb_sina.rs  | fns=2  | tests=YES  | source=SINA
    bond_gb_zh
    bond_gb_us

### bond/government.rs  | fns=1  | tests=no  | source=EASTMONEY
    bond_china_yield

### bond/issue_cninfo.rs  | fns=6  | tests=YES  | source=CNINFO
    bond_treasure_issue
    bond_local_gov_issue
    bond_corporate_issue
    bond_cov_issue
    bond_local_government_issue
    bond_cov_stock_issue

### bond/jsl.rs  | fns=4  | tests=no  | source=(see file)
    bond_cb_jsl
    bond_cb_index_jsl
    bond_cb_adj_logs_jsl
    bond_cb_redeem_jsl

### bond/nafmii.rs  | fns=1  | tests=no  | source=(see file)
    bond_debt_nafmii

### bond/spot.rs  | fns=3  | tests=YES  | source=EASTMONEY
    bond_spot_deal
    bond_spot_quote
    bond_spot_rates

### bond/summary_sse.rs  | fns=2  | tests=no  | source=SSE
    bond_cash_summary
    bond_deal_summary

### bond/zh_cov.rs  | fns=8  | tests=YES  | source=EASTMONEY
    bond_zh_cov
    bond_cov_comparison
    bond_zh_cov_info
    bond_zh_hs_cov_daily
    bond_zh_hs_cov_min
    bond_zh_hs_cov_pre_min
    bond_zh_hs_cov_spot
    bond_zh_cov_value_analysis

### bond/zh_sina.rs  | fns=2  | tests=YES  | source=SINA
    bond_zh_hs_spot
    bond_zh_hs_daily


## fund/

### fund/announcement_em.rs  | fns=3  | tests=no  | source=EASTMONEY
    fund_announcement_dividend
    fund_announcement_report
    fund_announcement_personnel

### fund/aum_em.rs  | fns=3  | tests=no  | source=EASTMONEY
    fund_aum
    fund_aum_trend
    fund_aum_hist

### fund/cf_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    fund_cf

### fund/em.rs  | fns=3  | tests=no  | source=EASTMONEY
    fund_purchase
    fund_name
    fund_info_index

### fund/etf_em.rs  | fns=10  | tests=no  | source=EASTMONEY
    fund_etf_fund_daily
    fund_etf_hist_em
    fund_etf_hist_min
    fund_etf_spot_em
    fund_etf_fund_info
    fund_etf_scale_sse
    fund_etf_scale_szse
    fund_etf_category_ths
    fund_etf_spot_ths
    fund_etf_dividend

### fund/etf.rs  | fns=1  | tests=no  | source=EASTMONEY
    fund_etf_hist

### fund/etf_sina.rs  | fns=2  | tests=no  | source=SINA
    fund_etf_category_sina
    fund_etf_hist_sina

### fund/fee_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    fund_fee

### fund/fhsp_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    fund_fh
    fund_fh_rank

### fund/financial_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    fund_financial_fund_daily
    fund_financial_fund_info

### fund/graded.rs  | fns=3  | tests=YES  | source=EASTMONEY
    fund_graded
    fund_graded_fund_daily
    fund_graded_fund_info

### fund/hk_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    fund_hk_rank
    fund_hk_fund_hist

### fund/hold_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    fund_hold_structure

### fund/info_ths.rs  | fns=1  | tests=no  | source=THS
    fund_info

### fund/init_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    fund_new_found_em
    fund_new_found_ths

### fund/lcx_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    fund_lcx_rank

### fund/lof.rs  | fns=5  | tests=YES  | source=EASTMONEY
    fund_lof_list
    fund_lof_hist
    fund_lof_hist_em
    fund_lof_hist_min
    fund_lof_spot

### fund/lof_ths.rs  | fns=1  | tests=no  | source=THS
    fund_lof

### fund/manager.rs  | fns=1  | tests=no  | source=EASTMONEY
    fund_manager

### fund/money.rs  | fns=4  | tests=YES  | source=EASTMONEY
    fund_money_market
    fund_money_fund_daily
    fund_money_fund_info
    fund_money_rank

### fund/open.rs  | fns=4  | tests=YES  | source=EASTMONEY
    fund_open_end_daily
    fund_open_end_nav
    fund_open_fund_daily
    fund_open_fund_info

### fund/overview_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    fund_overview

### fund/portfolio_em.rs  | fns=5  | tests=no  | source=EASTMONEY
    fund_portfolio_hold
    fund_portfolio_bond_hold
    fund_portfolio_asset_allocation
    fund_portfolio_industry_allocation
    fund_portfolio_change

### fund/position_lg.rs  | fns=6  | tests=no  | source=LEGU
    fund_position_lg
    fund_position_hist_lg
    fund_position_est_lg
    fund_stock_position_lg
    fund_balance_position_lg
    fund_linghuo_position_lg

### fund/qdii.rs  | fns=3  | tests=YES  | source=(see file)
    qdii_a_index_jsl
    qdii_e_index_jsl
    qdii_e_comm_jsl

### fund/rank_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    fund_open_fund_rank
    fund_exchange_rank

### fund/rating.rs  | fns=7  | tests=no  | source=EASTMONEY
    fund_rating
    fund_rating_zs
    fund_rating_tiantian
    fund_rating_jiashi
    fund_rating_all
    fund_rating_sh
    fund_rating_ja

### fund/report_cninfo.rs  | fns=6  | tests=no  | source=CNINFO
    fund_report
    fund_report_half_year
    fund_report_quarter
    fund_report_stock
    fund_report_industry_allocation
    fund_report_asset_allocation

### fund/scale_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    fund_scale_change

### fund/scale_sina.rs  | fns=4  | tests=no  | source=SINA
    fund_scale_open
    fund_scale_close
    fund_scale_money
    fund_scale_structured

### fund/scale_szse.rs  | fns=1  | tests=no  | source=SZSE
    fund_scale_daily

### fund/value_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    fund_value_estimation

### fund/xueqiu.rs  | fns=8  | tests=YES  | source=XUEQIU
    fund_xueqiu_info
    fund_xueqiu_achievement
    fund_individual_basic_info_xq
    fund_individual_achievement_xq
    fund_individual_analysis_xq
    fund_individual_profit_probability_xq
    fund_individual_detail_info_xq
    fund_individual_detail_hold_xq


## index/

### index/a_share.rs  | fns=4  | tests=no  | source=SINA
    index_a_share_candles
    index_stock_zh_spot_em
    index_stock_zh_spot_sina
    stock_zh_index_daily

### index/cflp.rs  | fns=2  | tests=YES  | source=CFLP
    index_price_cflp
    index_volume_cflp

### index/cni.rs  | fns=5  | tests=YES  | source=CNINDEX
    index_all_cni
    index_hist_cni
    index_detail_cni
    index_detail_hist_cni
    index_detail_hist_adjust_cni

### index/cons.rs  | fns=5  | tests=YES  | source=SINA
    index_stock_cons_sina
    index_stock_info
    index_stock_cons
    index_stock_cons_csindex
    index_stock_cons_weight_csindex

### index/csindex.rs  | fns=2  | tests=YES  | source=(see file)
    index_csindex_all
    stock_zh_index_hist_csindex

### index/cx.rs  | fns=3  | tests=YES  | source=(see file)
//! parameter. We implement a shared helper and expose one `pub async fn` per
    $name
    $name

### index/drewry.rs  | fns=1  | tests=YES  | source=DREWRY
    drewry_wci_index

### index/eri.rs  | fns=1  | tests=YES  | source=ERI
    index_eri

### index/global_em.rs  | fns=2  | tests=YES  | source=EASTMONEY
    index_global_spot
    index_global_hist_em

### index/global.rs  | fns=2  | tests=no  | source=YAHOO
    index_global_name_table_yahoo
    index_global_candles

### index/global_sina.rs  | fns=1  | tests=YES  | source=SINA
    index_global_hist_sina

### index/hf.rs  | fns=1  | tests=YES  | source=(see file)
    hf_sp_500

### index/hk.rs  | fns=3  | tests=YES  | source=SINA
    index_hk_spot_sina
    index_hk_spot_em
    index_hk_daily

### index/hog.rs  | fns=1  | tests=YES  | source=(see file)
    index_hog_spot_price

### index/kq_fz.rs  | fns=1  | tests=YES  | source=(see file)
    index_kq_fz

### index/kq_ss.rs  | fns=1  | tests=YES  | source=(see file)
    index_kq_fashion

### index/qvix.rs  | fns=2  | tests=YES  | source=(see file)
    $name
    $name

### index/scope.rs  | fns=1  | tests=YES  | source=(see file)
    index_news_sentiment_scope

### index/spot.rs  | fns=1  | tests=YES  | source=(see file)
    spot_goods

### index/sugar.rs  | fns=3  | tests=YES  | source=(see file)
    index_sugar_msweet
    index_inner_quote_sugar_msweet
    index_outer_quote_sugar_msweet

### index/sw_fund.rs  | fns=2  | tests=YES  | source=SHENWAN
    index_realtime_fund_sw
    index_hist_fund_sw

### index/sw_research.rs  | fns=8  | tests=YES  | source=SHENWAN
    index_hist_sw
    index_min_sw
    index_component_sw
    index_realtime_sw
    index_analysis_daily_sw
    index_analysis_week_month_sw
    index_analysis_weekly_sw
    index_analysis_monthly_sw

### index/sw.rs  | fns=6  | tests=YES  | source=SHENWAN
    sw_index_candles
    sw_index_first_info
    sw_index_second_info
    sw_index_third_cons
    sw_index_third_info
    sw_index_list

### index/us_sina.rs  | fns=1  | tests=YES  | source=SINA
    index_us_stock

### index/yw.rs  | fns=1  | tests=YES  | source=(see file)
    index_yw

### index/zh_em.rs  | fns=4  | tests=YES  | source=EASTMONEY
    index_zh_a_hist
    index_zh_a_hist_min_em
    index_zh_a_hist_min
    index_code_id_map


## futures/

### futures/basis.rs  | fns=3  | tests=no  | source=(see file)
    futures_spot_price
    futures_spot_price_daily
    futures_spot_price_previous

### futures/comex.rs  | fns=1  | tests=no  | source=EASTMONEY
    futures_comex_inventory

### futures/comm.rs  | fns=4  | tests=no  | source=JIN10
    futures_fees_info_openctp
    futures_comm_js
    futures_fees_info
    futures_comm_info

### futures/contract_detail.rs  | fns=4  | tests=no  | source=SINA
    futures_contract_detail_sina
    futures_contract_detail
    match_main_contract
    futures_contract_detail_em

### futures/cot.rs  | fns=7  | tests=no  | source=SHFE
    futures_shfe_position_rank
    futures_czce_position_rank
    futures_cffex_position_rank
    futures_dce_position_rank
    futures_gfex_position_rank
    futures_dce_position_rank_other
    futures_hold_pos

### futures/daily_bar.rs  | fns=8  | tests=YES  | source=CFFEX
    futures_daily_cffex
    futures_daily_shfe
    futures_daily_ine
    futures_daily_dce
    futures_daily_czce
    futures_daily_gfex
    get_futures_daily
    futures_hist_daily_cffex

### futures/delivery.rs  | fns=8  | tests=no  | source=SHFE
    futures_to_spot_shfe
    futures_delivery_shfe
    futures_delivery_dce
    futures_to_spot_dce
    futures_delivery_match_czce
    futures_delivery_czce
    futures_delivery_match_dce
    futures_to_spot_czce

### futures/derivative.rs  | fns=12  | tests=no  | source=CFFEX
    futures_contract_info_cffex
    futures_contract_info_czce
    futures_contract_info_dce
    futures_contract_info_gfex
    futures_contract_info_ine
    futures_contract_info_shfe
    futures_hog_core
    futures_hog_cost
    futures_hog_supply
    futures_display_main
    futures_main_sina_derivative
    futures_spot_sys

### futures/exchange.rs  | fns=15  | tests=no  | source=CFFEX
    get_cffex_daily
    get_cffex_rank_table
    get_czce_daily
    get_dce_daily
    get_dce_rank_table
    get_gfex_daily
    get_ine_daily
    get_shfe_daily
    get_shfe_rank_table
    get_rank_table_czce
    get_roll_yield_bar
    get_rank_sum
    get_rank_sum_daily
    get_receipt
    get_token

### futures/foreign.rs  | fns=10  | tests=no  | source=SINA
    futures_foreign_hist
    get_qhkc_fund_bs
    get_qhkc_fund_money_change
    get_qhkc_fund_position
    get_qhkc_index
    get_qhkc_index_profit_loss
    get_qhkc_index_trend
    qhkc_tool_foreign
    qhkc_tool_gdp
    futures_foreign_detail

### futures/hf_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    futures_global_spot
    futures_global_hist

### futures/hist_em.rs  | fns=2  | tests=no  | source=EASTMONEY
    futures_hist_table
    futures_hist

### futures/hq_sina.rs  | fns=2  | tests=no  | source=SINA
    futures_foreign_commodity_realtime
    futures_foreign_commodity_realtime_str

### futures/index.rs  | fns=1  | tests=no  | source=(see file)
    futures_index_ccidx

### futures/inventory.rs  | fns=2  | tests=no  | source=EASTMONEY
    futures_inventory
    futures_inventory_99

### futures/news.rs  | fns=1  | tests=no  | source=(see file)
    futures_news_shmet

### futures/receipt.rs  | fns=2  | tests=no  | source=DCE
    get_dce_receipt
    get_shfe_receipt

### futures/roll_yield.rs  | fns=2  | tests=no  | source=(see file)
    get_roll_yield
    futures_roll_yield_bar

### futures/rule.rs  | fns=3  | tests=no  | source=EASTMONEY
    futures_rule_gtja
    futures_rule
    futures_rule_em

### futures/settle.rs  | fns=7  | tests=no  | source=CFFEX
    futures_settle_cffex
    futures_settle_czce
    futures_settle_shfe
    futures_settle_ine
    futures_settle_gfex
    futures_stock_shfe_js
    futures_settle

### futures/sgx.rs  | fns=1  | tests=no  | source=EASTMONEY
    futures_settlement_price_sgx

### futures/sina.rs  | fns=6  | tests=no  | source=SINA
    futures_main
    futures_symbol_mark
    futures_zh_realtime
    futures_zh_spot
    futures_zh_minute
    futures_zh_daily

### futures/spot.rs  | fns=1  | tests=YES  | source=EASTMONEY
    futures_spot_prices

### futures/spot_stock.rs  | fns=2  | tests=no  | source=EASTMONEY
    futures_spot_stock
    futures_spot_stock_em

### futures/warehouse.rs  | fns=4  | tests=no  | source=CZCE
    futures_warehouse_receipt_czce
    futures_warehouse_receipt_dce
    futures_shfe_warehouse_receipt
    futures_gfex_warehouse_receipt


## option/

### option/analysis_em.rs  | fns=3  | tests=no  | source=EASTMONEY
    option_premium_analysis
    option_value_analysis
    option_risk_analysis

### option/cffex_sina.rs  | fns=9  | tests=YES  | source=CFFEX
    option_cffex_sz50_list
    option_cffex_hs300_list
    option_cffex_zz1000_list
    option_cffex_sz50_spot
    option_cffex_hs300_spot
    option_cffex_zz1000_spot
    option_cffex_sz50_daily
    option_cffex_hs300_daily
    option_cffex_zz1000_daily

### option/commodity.rs  | fns=6  | tests=no  | source=DCE
    option_hist_dce
    option_hist_czce
    option_hist_shfe
    option_vol_shfe
    option_hist_gfex
    option_vol_gfex

### option/commodity_sina.rs  | fns=3  | tests=no  | source=SINA
    option_commodity_contract
    option_commodity_contract_table
    option_commodity_hist

### option/comm_qihuo.rs  | fns=2  | tests=no  | source=(see file)
    option_comm_symbol
    option_comm_info

### option/contract_info_ctp.rs  | fns=1  | tests=no  | source=(see file)
    option_contract_info_ctp

### option/current_sse.rs  | fns=2  | tests=no  | source=SSE
    option_current_day_sse
    option_current_day_szse

### option/czce.rs  | fns=1  | tests=no  | source=CZCE
    option_hist_yearly_czce

### option/daily_stats.rs  | fns=2  | tests=no  | source=SSE
    option_daily_stats_sse
    option_daily_stats_szse

### option/em.rs  | fns=4  | tests=YES  | source=EASTMONEY
    option_chain
    option_current
    option_current_cffex
    option_minute

### option/finance.rs  | fns=2  | tests=no  | source=SSE
    option_finance_sse_underlying
    option_finance_board

### option/lhb_em.rs  | fns=1  | tests=no  | source=EASTMONEY
    option_lhb

### option/margin.rs  | fns=2  | tests=no  | source=(see file)
    option_margin_symbol
    option_margin

### option/risk_indicator.rs  | fns=1  | tests=no  | source=SSE
    option_risk_indicator

### option/sse_sina.rs  | fns=9  | tests=no  | source=SSE
    option_sse_list
    option_sse_expire_day
    option_sse_codes
    option_sse_spot_price
    option_sse_underlying_spot_price
    option_sse_greeks
    option_sse_minute
    option_sse_daily
    option_finance_minute


## economy/

### economy/air.rs  | fns=8  | tests=YES  | source=EASTMONEY
    economy_air_quality
    air_quality_hebei
    air_city_table
    air_quality_hist
    air_quality_rank
    air_quality_watch_point
    sunrise_daily
    sunrise_monthly

### economy/amac.rs  | fns=15  | tests=YES  | source=AMAC
    economy_amac_stats
    amac_manager_info
    amac_manager_classify_info
    amac_manager_cancelled_info
    amac_fund_info
    amac_fund_abs
    amac_fund_sub_info
    amac_fund_account_info
    amac_member_info
    amac_member_sub_info
    amac_securities_info
    amac_futures_info
    amac_aoin_info
    amac_person_bond_org_list
    amac_person_fund_org_list

### economy/article.rs  | fns=7  | tests=YES  | source=(see file)
    article_epu_index
    fred_md
    fred_qd
    article_oman_rv
    article_oman_rv_short
    article_ff_crr
    article_rlab_rv

### economy/car.rs  | fns=3  | tests=YES  | source=EASTMONEY
    car_market_country_cpca
    car_market_segment_cpca
    economy_auto_sales

### economy/event.rs  | fns=2  | tests=YES  | source=BAIDU
    migration_area
    migration_scale

### economy/fortune.rs  | fns=5  | tests=YES  | source=(see file)
    index_bloomberg_billionaires
    index_bloomberg_billionaires_hist
    forbes_rank
    hurun_rank
    xincaifu_rank

### economy/movie.rs  | fns=9  | tests=YES  | source=EASTMONEY
    movie_boxoffice_cinema_daily
    movie_boxoffice_cinema_weekly
    movie_boxoffice_daily
    movie_boxoffice_monthly
    movie_boxoffice_realtime
    movie_boxoffice_weekly
    movie_boxoffice_yearly
    movie_boxoffice_yearly_first_week
    economy_box_office

### economy/nlp.rs  | fns=3  | tests=YES  | source=EASTMONEY
    economy_sentiment_index
    nlp_ownthink
    nlp_answer

### economy/other.rs  | fns=10  | tests=YES  | source=(see file)
    car_market_total_cpca
    car_market_man_rank_cpca
    car_market_cate_cpca
    car_market_fuel_cpca
    car_sale_rank_gasgoo
    business_value_artist
    online_value_artist
    video_tv
    video_variety_show
    game_hot_rank_taptap


## forex/

### forex/boc.rs  | fns=1  | tests=no  | source=EASTMONEY
    forex_boc_rates

### forex/currency.rs  | fns=7  | tests=YES  | source=(see file)
    currency_latest
    currency_history
    currency_time_series
    currency_currencies
    currency_convert
    currency_boc
    currency_boc_safe

### forex/em.rs  | fns=4  | tests=YES  | source=EASTMONEY
    forex_em_rates
    forex_spot
    forex_hist
    forex_em_hist

### forex/fx.rs  | fns=6  | tests=YES  | source=BAIDU
    currency_pair_map
    fx_c_swap_cm
    fx_pair_quote
    fx_spot_quote
    fx_swap_quote
    fx_quote

### forex/sina.rs  | fns=1  | tests=YES  | source=SINA
    forex_sina_rates


## commodity/

### commodity/carbon.rs  | fns=6  | tests=YES  | source=(see file)
    energy_carbon_bj
    energy_carbon_sz
    energy_carbon_eu
    energy_carbon_hb
    energy_carbon_gz
    energy_oil_detail

### commodity/energy.rs  | fns=2  | tests=no  | source=EASTMONEY
    energy_oil_hist
    energy_carbon_domestic

### commodity/spot.rs  | fns=1  | tests=YES  | source=EASTMONEY
    commodity_spot_prices


## bank/

### bank/fjcf.rs  | fns=1  | tests=YES  | source=(see file)
    bank_fjcf_table_detail


## reits/

### reits/em.rs  | fns=5  | tests=YES  | source=EASTMONEY
    reits_list
    reits_hist_em
    reits_hist_min
    reits_realtime
    reits_hist


## spot/

### spot/hog_soozhu.rs  | fns=8  | tests=YES  | source=(see file)
    spot_hog_soozhu
    spot_hog_year_trend_soozhu
    spot_hog_lean_price_soozhu
    spot_hog_three_way_soozhu
    spot_hog_crossbred_soozhu
    spot_corn_price_soozhu
    spot_soybean_price_soozhu
    spot_mixed_feed_soozhu

### spot/price_qh.rs  | fns=2  | tests=YES  | source=(see file)
    spot_price_qh
    spot_price_table_qh

### spot/sge.rs  | fns=4  | tests=YES  | source=SGE
    spot_quotations_sge
    spot_hist_sge
    spot_golden_benchmark_sge
    spot_silver_benchmark_sge


## news/

### news/baidu.rs  | fns=1  | tests=no  | source=BAIDU
    baidu_news_search

### news/cctv.rs  | fns=5  | tests=no  | source=CCTV
    news_cctv
    news_economic
    news_report_time
    news_trade_notify_dividend
    news_trade_notify_suspend

### news/finnhub.rs  | fns=1  | tests=YES  | source=FINNHUB
    finnhub_company_news

### news/gdelt.rs  | fns=2  | tests=no  | source=GDELT
    gdelt_news_search
    gdelt_news_search_owned

### news/marketaux.rs  | fns=1  | tests=YES  | source=(see file)
    marketaux_news

### news/rss.rs  | fns=4  | tests=YES  | source=(see file)
    bing_news
    bing_news_rss_with_lang
    google_news
    seeking_alpha_news

### news/search.rs  | fns=2  | tests=YES  | source=EASTMONEY
    news_search
    news_search_with_scope

### news/sogou.rs  | fns=1  | tests=no  | source=SOGOU
    sogou_news_search


## macro_data/

### macro_data/australia.rs  | fns=14  | tests=no  | source=EASTMONEY
    australia_retail_rate_monthly
    australia_trade
    australia_unemployment_rate
    australia_ppi_quarterly
    australia_cpi_quarterly
    australia_cpi_yearly
    australia_bank_rate
    macro_australia_bank_rate
    macro_australia_cpi_quarterly
    macro_australia_cpi_yearly
    macro_australia_ppi_quarterly
    macro_australia_retail_rate_monthly
    macro_australia_trade
    macro_australia_unemployment_rate

### macro_data/bank.rs  | fns=22  | tests=no  | source=JIN10
    bank_usa_interest_rate
    bank_euro_interest_rate
    bank_newzealand_interest_rate
    bank_china_interest_rate
    bank_switzerland_interest_rate
    bank_england_interest_rate
    bank_australia_interest_rate
    bank_japan_interest_rate
    bank_russia_interest_rate
    bank_india_interest_rate
    bank_brazil_interest_rate
    macro_bank_australia_interest_rate
    macro_bank_brazil_interest_rate
    macro_bank_china_interest_rate
    macro_bank_euro_interest_rate
    macro_bank_india_interest_rate
    macro_bank_japan_interest_rate
    macro_bank_newzealand_interest_rate
    macro_bank_russia_interest_rate
    macro_bank_switzerland_interest_rate
    macro_bank_usa_interest_rate
    macro_bank_english_interest_rate

### macro_data/canada.rs  | fns=20  | tests=no  | source=EASTMONEY
    canada_new_house_rate
    canada_unemployment_rate
    canada_trade
    canada_retail_rate_monthly
    canada_bank_rate
    canada_core_cpi_yearly
    canada_core_cpi_monthly
    canada_cpi_yearly
    canada_cpi_monthly
    canada_gdp_monthly
    macro_canada_bank_rate
    macro_canada_core_cpi_monthly
    macro_canada_core_cpi_yearly
    macro_canada_cpi_monthly
    macro_canada_cpi_yearly
    macro_canada_gdp_monthly
    macro_canada_new_house_rate
    macro_canada_retail_rate_monthly
    macro_canada_trade
    macro_canada_unemployment_rate

### macro_data/china.rs  | fns=135  | tests=no  | source=EASTMONEY
    china_gdp
    china_cpi
    china_ppi
    china_pmi
    china_money_supply
    china_trade
    china_goods_index
    china_fdi
    china_lpr
    china_new_house_price
    china_enterprise_boom_index
    china_national_tax_receipts
    china_new_financial_credit
    china_fx_gold
    china_stock_market_cap
    china_fixed_asset_investment
    china_fiscal_revenue
    china_fx_loans
    china_fx_deposits
    china_consumer_confidence
    china_industrial_growth
    china_reserve_requirement_ratio
    china_consumer_goods_retail
    china_bank_financing
    china_insurance_income
    china_mobile_number
    china_vegetable_basket
    china_agricultural_product
    china_agricultural_index
    china_energy_index
    china_commodity_price_index
    china_yw_electronic_index
    china_construction_index
    china_construction_price_index
    china_lpi_index
    china_bdti_index
    china_bsi_index
    china_real_estate
    china_gdp_yearly
    china_cpi_yearly
    china_cpi_monthly
    china_ppi_yearly
    china_exports_yoy
    china_imports_yoy
    china_trade_balance_jin10
    china_industrial_production_yoy
    china_pmi_jin10
    china_caixin_pmi
    china_caixin_services_pmi
    china_non_man_pmi
    china_fx_reserves_yearly
    china_m2_yearly
    china_shibor
    china_hibor
    china_rmb_central_parity
    china_margin_sz
    china_margin_sh
    china_sge_report
    macro_china_agricultural_index
    macro_china_agricultural_product
    macro_china_bank_financing
    macro_china_bdti_index
    macro_china_bsi_index
    macro_china_commodity_price_index
    macro_china_construction_index
    macro_china_construction_price_index
    macro_china_consumer_goods_retail
    macro_china_cpi
    macro_china_cpi_monthly
    macro_china_cpi_yearly
    macro_china_energy_index
    macro_china_enterprise_boom_index
    macro_china_exports_yoy
    macro_china_fdi
    macro_china_fx_gold
    macro_china_fx_reserves_yearly
    macro_china_gdp
    macro_china_gdp_yearly
    macro_china_imports_yoy
    macro_china_industrial_production_yoy
    macro_china_insurance_income
    macro_china_lpi_index
    macro_china_lpr
    macro_china_m2_yearly
    macro_china_mobile_number
    macro_china_money_supply
    macro_china_national_tax_receipts
    macro_china_new_financial_credit
    macro_china_new_house_price
    macro_china_non_man_pmi
    macro_china_pmi
    macro_china_ppi
    macro_china_ppi_yearly
    macro_china_real_estate
    macro_china_reserve_requirement_ratio
    macro_china_stock_market_cap
    macro_china_vegetable_basket
    macro_china_yw_electronic_index
    macro_china_au_report
    macro_china_shibor_all
    macro_china_hk_market_info
    macro_china_rmb
    macro_china_market_margin_sz
    macro_china_market_margin_sh
    macro_china_trade_balance
    macro_china_pmi_yearly
    macro_china_cx_pmi_yearly
    macro_china_cx_services_pmi_yearly
    macro_china_czsr
    macro_china_gdzctz
    macro_china_gyzjz
    macro_china_hgjck
    macro_china_whxd
    macro_china_wbck
    macro_china_xfzxx
    macro_china_qyspjg
    macro_cnbs
    macro_china_central_bank_balance
    macro_china_insurance
    macro_china_supply_of_money
    macro_china_foreign_exchange_gold
    macro_china_retail_price_index
    macro_china_society_electricity
    macro_china_society_traffic_volume
    macro_china_postal_telecommunicational
    macro_china_international_tourism_fx
    macro_china_passenger_load_factor
    macro_china_freight_index
    macro_china_shrzgm
    macro_rmb_loan
    macro_rmb_deposit
    macro_china_daily_energy
    macro_china_nbs_nation
    macro_china_nbs_region
    macro_china_urban_unemployment

### macro_data/constitute.rs  | fns=5  | tests=no  | source=JIN10
    gold_etf_holding
    silver_etf_holding
    macro_cons_gold
    macro_cons_silver
    macro_cons_opec_month

### macro_data/euro.rs  | fns=32  | tests=no  | source=JIN10
    euro_gdp_yoy
    euro_cpi_mom
    euro_cpi_yoy
    euro_ppi_mom
    euro_retail_sales_mom
    euro_employment_change_qoq
    euro_unemployment_rate
    euro_trade_balance
    euro_current_account_mom
    euro_industrial_production_mom
    euro_manufacturing_pmi
    euro_services_pmi
    euro_zew_economic_sentiment
    euro_sentix_investor_confidence
    euro_lme_holding
    euro_lme_stock
    macro_euro_cpi_mom
    macro_euro_cpi_yoy
    macro_euro_current_account_mom
    macro_euro_employment_change_qoq
    macro_euro_gdp_yoy
    macro_euro_industrial_production_mom
    macro_euro_lme_holding
    macro_euro_lme_stock
    macro_euro_manufacturing_pmi
    macro_euro_ppi_mom
    macro_euro_retail_sales_mom
    macro_euro_sentix_investor_confidence
    macro_euro_services_pmi
    macro_euro_trade_balance
    macro_euro_zew_economic_sentiment
    macro_euro_unemployment_rate_mom

### macro_data/germany.rs  | fns=16  | tests=no  | source=EASTMONEY
    germany_ifo
    germany_cpi_monthly
    germany_cpi_yearly
    germany_trade_adjusted
    germany_gdp
    germany_retail_sale_monthly
    germany_retail_sale_yearly
    germany_zew
    macro_germany_cpi_monthly
    macro_germany_cpi_yearly
    macro_germany_gdp
    macro_germany_ifo
    macro_germany_retail_sale_monthly
    macro_germany_retail_sale_yearly
    macro_germany_trade_adjusted
    macro_germany_zew

### macro_data/global.rs  | fns=5  | tests=no  | source=(see file)
    macro_global_sox_index
    macro_shipping_bci
    macro_shipping_bdi
    macro_shipping_bpi
    macro_shipping_bcti

### macro_data/hk.rs  | fns=18  | tests=no  | source=EASTMONEY
    hk_cpi
    hk_cpi_ratio
    hk_unemployment_rate
    hk_gdp
    hk_gdp_ratio
    hk_building_volume
    hk_building_amount
    hk_trade_diff_ratio
    hk_ppi
    macro_china_hk_building_amount
    macro_china_hk_building_volume
    macro_china_hk_cpi
    macro_china_hk_cpi_ratio
    macro_china_hk_ppi
    macro_china_hk_trade_diff_ratio
    macro_china_hk_gbp
    macro_china_hk_gbp_ratio
    macro_china_hk_rate_of_unemployment

### macro_data/interest_rate.rs  | fns=1  | tests=YES  | source=EASTMONEY
    rate_interbank

### macro_data/japan.rs  | fns=10  | tests=no  | source=EASTMONEY
    japan_bank_rate
    japan_cpi_yearly
    japan_core_cpi_yearly
    japan_unemployment_rate
    japan_leading_indicator
    macro_japan_bank_rate
    macro_japan_core_cpi_yearly
    macro_japan_cpi_yearly
    macro_japan_unemployment_rate
    macro_japan_head_indicator

### macro_data/other.rs  | fns=4  | tests=no  | source=JIN10
    macro_crypto_spot
    macro_fx_sentiment
    macro_info_ws
    macro_stock_finance

### macro_data/rate.rs  | fns=2  | tests=YES  | source=(see file)
    repo_rate_query
    repo_rate_hist

### macro_data/swiss.rs  | fns=12  | tests=no  | source=EASTMONEY
    swiss_svme
    swiss_trade
    swiss_cpi_yearly
    swiss_gdp_quarterly
    swiss_gdp_yearly
    swiss_bank_rate
    macro_swiss_cpi_yearly
    macro_swiss_gdp_quarterly
    macro_swiss_svme
    macro_swiss_trade
    macro_swiss_gbd_yearly
    macro_swiss_gbd_bank_rate

### macro_data/uk.rs  | fns=30  | tests=no  | source=EASTMONEY
    uk_halifax_monthly
    uk_halifax_yearly
    uk_trade
    uk_bank_rate
    uk_core_cpi_yearly
    uk_core_cpi_monthly
    uk_cpi_yearly
    uk_cpi_monthly
    uk_retail_monthly
    uk_retail_yearly
    uk_rightmove_yearly
    uk_rightmove_monthly
    uk_gdp_quarterly
    uk_gdp_yearly
    uk_unemployment_rate
    macro_uk_bank_rate
    macro_uk_core_cpi_monthly
    macro_uk_core_cpi_yearly
    macro_uk_cpi_monthly
    macro_uk_cpi_yearly
    macro_uk_gdp_quarterly
    macro_uk_gdp_yearly
    macro_uk_halifax_monthly
    macro_uk_halifax_yearly
    macro_uk_retail_monthly
    macro_uk_retail_yearly
    macro_uk_rightmove_monthly
    macro_uk_rightmove_yearly
    macro_uk_trade
    macro_uk_unemployment_rate

### macro_data/us.rs  | fns=97  | tests=no  | source=EASTMONEY
    us_pending_home_sales
    us_cpi_yoy
    us_gdp_monthly
    us_cpi_monthly
    us_core_cpi_monthly
    us_personal_spending
    us_retail_sales
    us_import_price
    us_export_price
    us_lmci
    us_unemployment_rate
    us_job_cuts
    us_non_farm
    us_adp_employment
    us_core_pce_price
    us_real_consumer_spending
    us_trade_balance
    us_current_account
    us_ppi
    us_core_ppi
    us_api_crude_stock
    us_pmi
    us_ism_pmi
    us_industrial_production
    us_durable_goods_orders
    us_factory_orders
    us_services_pmi
    us_business_inventories
    us_ism_non_pmi
    us_nahb_house_market_index
    us_house_starts
    us_new_home_sales
    us_building_permits
    us_exist_home_sales
    us_house_price_index
    us_spcs20
    us_pending_home_sales_jin10
    us_cb_consumer_confidence
    us_nfib_small_business
    us_michigan_consumer_sentiment
    us_eia_crude_rate
    us_initial_jobless
    us_rig_count
    us_crude_production
    macro_usa_crude_inner
    macro_usa_phs
    macro_usa_cpi_yoy
    macro_usa_gdp_monthly
    macro_usa_cpi_monthly
    macro_usa_core_cpi_monthly
    macro_usa_personal_spending
    macro_usa_retail_sales
    macro_usa_import_price
    macro_usa_export_price
    macro_usa_lmci
    macro_usa_unemployment_rate
    macro_usa_job_cuts
    macro_usa_non_farm
    macro_usa_adp_employment
    macro_usa_core_pce_price
    macro_usa_real_consumer_spending
    macro_usa_trade_balance
    macro_usa_current_account
    macro_usa_ppi
    macro_usa_core_ppi
    macro_usa_api_crude_stock
    macro_usa_pmi
    macro_usa_ism_pmi
    macro_usa_industrial_production
    macro_usa_durable_goods_orders
    macro_usa_factory_orders
    macro_usa_services_pmi
    macro_usa_business_inventories
    macro_usa_ism_non_pmi
    macro_usa_nahb_house_market_index
    macro_usa_house_starts
    macro_usa_new_home_sales
    macro_usa_building_permits
    macro_usa_exist_home_sales
    macro_usa_house_price_index
    macro_usa_spcs20
    macro_usa_pending_home_sales
    macro_usa_cb_consumer_confidence
    macro_usa_nfib_small_business
    macro_usa_michigan_consumer_sentiment
    macro_usa_eia_crude_rate
    macro_usa_initial_jobless
    macro_usa_rig_count
    macro_usa_cftc_nc_holding
    macro_usa_cftc_c_holding
    macro_usa_cftc_merchant_currency_holding
    macro_usa_cftc_merchant_goods_holding
    macro_usa_cme_merchant_goods_holding
    us_cftc_nc_holding
    us_cftc_c_holding
    us_cftc_merchant_currency_holding
    us_cftc_merchant_goods_holding


## tool/

### tool/pro.rs  | fns=1  | tests=no  | source=(see file)
    pro_api

### tool/trade_date.rs  | fns=1  | tests=YES  | source=SINA
    tool_trade_date_hist


## cal/

### cal/rv.rs  | fns=2  | tests=YES  | source=(see file)
    rv_from_futures_zh_minute
    rv_from_stock_zh_a_hist_min


## ta/


## provider/

### provider/eastmoney.rs  | fns=10  | tests=no  | source=EASTMONEY
    eastmoney_search
    eastmoney_klines
    eastmoney_sector_rankings
    eastmoney_sector_constituents
    eastmoney_sector_capital_flow
    eastmoney_capital_flow
    eastmoney_billboard
    eastmoney_billboard_seats
    eastmoney_announcements
    eastmoney_announcement_detail

### provider/market_client/cache.rs  | fns=2  | tests=no  | source=MARKETDATACLIENT
    enter
    do_once<F, Fut, T>

### provider/market_client/client/batch.rs  | fns=2  | tests=no  | source=MARKETDATACLIENT
    fetch_quotes_batch
    fetch_fundamentals_batch

### provider/market_client/client/billboard.rs  | fns=12  | tests=no  | source=MARKETDATACLIENT
    fetch_billboard_entries
    fetch_billboard_seats
    fetch_lhb_detail
    fetch_lhb_stock_statistic
    fetch_lhb_jgmmtj
    fetch_lhb_jgstatistic
    fetch_lhb_hyyyb
    fetch_lhb_yybph
    fetch_lhb_trader_statistic
    fetch_lhb_stock_detail_date
    fetch_lhb_stock_detail
    fetch_lhb_yyb_detail

### provider/market_client/client/fund_flow.rs  | fns=4  | tests=no  | source=MARKETDATACLIENT
    fetch_fund_flow_individual
    fetch_fund_flow_concept
    fetch_fund_flow_industry
    fetch_main_fund_flow

### provider/market_client/client/margin.rs  | fns=6  | tests=no  | source=MARKETDATACLIENT
    fetch_margin_account_info
    fetch_margin_sse_detail
    fetch_margin_szse_detail
    fetch_margin_ratio_pa
    fetch_margin_sse_summary
    fetch_margin_szse_summary

### provider/market_client/client/mod.rs  | fns=94  | tests=no  | source=MARKETDATACLIENT
    fetch_quote
    fetch_quote_with_provider
    fetch_fundamentals
    fetch_news
    fetch_news_with_diagnostics
    fetch_news_with_diagnostics_query
    fetch_global_news
    fetch_global_news_with_diagnostics
    fetch_insider_transactions
    fetch_candles
    fetch_candles_with_provider
    fetch_capital_flow
    fetch_a_share_sector_rankings
    fetch_a_share_sector_constituents
    fetch_a_share_sector_capital_flow
    fetch_announcement_detail
    fetch_announcements
    search_stocks
    fetch_trade_calendar
    fetch_return_since
    fetch_zt_pool
    fetch_zt_pool_dtgc
    fetch_zt_pool_previous
    fetch_zt_pool_strong
    fetch_zt_pool_sub_new
    fetch_zt_pool_zbgc
    fetch_earnings_forecast
    fetch_earnings_quick_report
    fetch_earnings_report
    fetch_analyst_rank
    fetch_analyst_detail
    fetch_gdfx_free_holding_statistics
    fetch_gdfx_holding_statistics
    fetch_gdfx_free_holding_change
    fetch_gdfx_holding_change
    fetch_gdfx_free_top10
    fetch_gdfx_top10
    fetch_gdfx_free_holding_detail
    fetch_gdfx_holding_detail
    fetch_gdfx_free_holding_analyse
    fetch_gdfx_holding_analyse
    fetch_gdfx_free_teamwork
    fetch_gdfx_teamwork
    fetch_block_trade_daily
    fetch_block_trade_industry
    fetch_block_trade_industry_daily
    fetch_block_trade_seat_ranking
    fetch_hot_follow_xq
    fetch_hot_tweet_xq
    fetch_hot_deal_xq
    fetch_pankou_changes
    fetch_dividends
    fetch_dividend_detail
    fetch_pledge_profile
    fetch_pledge_ratio
    fetch_pledge_detail
    fetch_pledge_ratio_detail
    fetch_pledge_distribute_bank
    fetch_pledge_distribute_company
    fetch_pledge_industry
    fetch_institutional_research
    fetch_institutional_research_detail
    fetch_esg_msci
    fetch_esg_rft
    fetch_esg_zd
    fetch_esg_hz
    fetch_balance_sheet
    fetch_profit_sheet
    fetch_cash_flow_sheet
    fetch_stock_comments
    fetch_comment_org_participation
    fetch_comment_hist_score
    fetch_comment_focus_index
    fetch_comment_desire_index
    fetch_executive_shareholding
    fetch_shareholder_count
    fetch_shareholder_count_detail
    fetch_industry_category
    fetch_hk_spot
    fetch_hk_famous_spot
    fetch_hk_hot_rank
    fetch_hk_hot_rank_latest
    fetch_hk_hot_rank_detail
    fetch_hk_hot_rank_realtime
    fetch_hk_dividend_payout
    fetch_hk_fhpx_detail
    fetch_hk_dividend_yield
    fetch_hk_financial_indicators
    fetch_hk_valuation
    fetch_us_spot
    fetch_us_famous_spot
    fetch_us_pink_spot
    fetch_us_valuation
    fetch_xq_spot

### provider/market_client/diagnosis.rs  | fns=3  | tests=YES  | source=MARKETDATACLIENT
    fetch_with_rotation<T>
    fetch_quote_with_rotation
    fetch_candles_with_rotation

### provider/market_client/mod.rs  | fns=2  | tests=YES  | source=MARKETDATACLIENT
    new
    from_config

### provider/market_client/news_search/mod.rs  | fns=2  | tests=YES  | source=MARKETDATACLIENT
    fetch_news_search_evidence
    fetch_news_search_queries_with_attempts

### provider/market_client/tools/mod.rs  | fns=1  | tests=YES  | source=MARKETDATACLIENT
    execute

### provider/sina.rs  | fns=2  | tests=no  | source=SINA
    sina_a_share_realtime
    sina_us_daily

### provider/stooq.rs  | fns=1  | tests=no  | source=STOOQ
    stooq_candles

