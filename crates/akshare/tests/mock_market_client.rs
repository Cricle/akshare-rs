#![cfg(feature = "market-client")]

mod common;

use wiremock::MockServer;

async fn mount_mocks(server: &MockServer) {
    common::mount_em_mocks(server).await;
}

#[tokio::test]
async fn test_market_fetch_quote() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_quote("test").await;
}

#[tokio::test]
async fn test_market_fetch_fundamentals() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_fundamentals("test").await;
}

#[tokio::test]
async fn test_market_fetch_insider_transactions() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_insider_transactions("test").await;
}

#[tokio::test]
async fn test_market_fetch_candles() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_candles("test", "test", 10).await;
}

#[tokio::test]
async fn test_market_fetch_capital_flow() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_capital_flow("test", 10).await;
}

#[tokio::test]
async fn test_market_fetch_a_share_sector_rankings() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_a_share_sector_rankings("test", 10).await;
}

#[tokio::test]
async fn test_market_fetch_a_share_sector_constituents() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_a_share_sector_constituents("test", 10).await;
}

#[tokio::test]
async fn test_market_fetch_a_share_sector_capital_flow() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_a_share_sector_capital_flow("test", 10).await;
}

#[tokio::test]
async fn test_market_fetch_announcement_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_announcement_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_announcements() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_announcements("test", 10).await;
}

#[tokio::test]
async fn test_market_fetch_trade_calendar() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_trade_calendar("test", "test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_zt_pool() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_zt_pool("test").await;
}

#[tokio::test]
async fn test_market_fetch_zt_pool_dtgc() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_zt_pool_dtgc("test").await;
}

#[tokio::test]
async fn test_market_fetch_zt_pool_previous() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_zt_pool_previous("test").await;
}

#[tokio::test]
async fn test_market_fetch_zt_pool_strong() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_zt_pool_strong("test").await;
}

#[tokio::test]
async fn test_market_fetch_zt_pool_sub_new() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_zt_pool_sub_new("test").await;
}

#[tokio::test]
async fn test_market_fetch_zt_pool_zbgc() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_zt_pool_zbgc("test").await;
}

#[tokio::test]
async fn test_market_fetch_earnings_report() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_earnings_report("test").await;
}

#[tokio::test]
async fn test_market_fetch_analyst_rank() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_analyst_rank("test").await;
}

#[tokio::test]
async fn test_market_fetch_analyst_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_analyst_detail("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_free_holding_statistics() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_free_holding_statistics("test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_holding_statistics() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_holding_statistics("test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_free_holding_change() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_free_holding_change("test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_holding_change() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_holding_change("test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_free_top10() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_free_top10("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_top10() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_top10("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_free_holding_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_free_holding_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_holding_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client
        .fetch_gdfx_holding_detail("test", "test", "test")
        .await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_free_holding_analyse() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_free_holding_analyse("test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_holding_analyse() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_holding_analyse("test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_free_teamwork() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_free_teamwork("test").await;
}

#[tokio::test]
async fn test_market_fetch_gdfx_teamwork() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_gdfx_teamwork("test").await;
}

#[tokio::test]
async fn test_market_fetch_block_trade_daily() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_block_trade_daily("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_block_trade_industry() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_block_trade_industry("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_block_trade_industry_daily() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client
        .fetch_block_trade_industry_daily("test", "test")
        .await;
}

#[tokio::test]
async fn test_market_fetch_block_trade_seat_ranking() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_block_trade_seat_ranking("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_hot_follow_xq() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hot_follow_xq("test").await;
}

#[tokio::test]
async fn test_market_fetch_hot_tweet_xq() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hot_tweet_xq("test").await;
}

#[tokio::test]
async fn test_market_fetch_hot_deal_xq() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hot_deal_xq("test").await;
}

#[tokio::test]
async fn test_market_fetch_pankou_changes() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_pankou_changes("test").await;
}

#[tokio::test]
async fn test_market_fetch_dividends() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_dividends("test").await;
}

#[tokio::test]
async fn test_market_fetch_dividend_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_dividend_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_pledge_profile() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_pledge_profile().await;
}

#[tokio::test]
async fn test_market_fetch_pledge_ratio() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_pledge_ratio().await;
}

#[tokio::test]
async fn test_market_fetch_pledge_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_pledge_detail().await;
}

#[tokio::test]
async fn test_market_fetch_pledge_ratio_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_pledge_ratio_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_pledge_distribute_bank() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_pledge_distribute_bank().await;
}

#[tokio::test]
async fn test_market_fetch_pledge_distribute_company() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_pledge_distribute_company().await;
}

#[tokio::test]
async fn test_market_fetch_pledge_industry() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_pledge_industry().await;
}

#[tokio::test]
async fn test_market_fetch_institutional_research() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_institutional_research("test").await;
}

#[tokio::test]
async fn test_market_fetch_institutional_research_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_institutional_research_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_esg_msci() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_esg_msci().await;
}

#[tokio::test]
async fn test_market_fetch_esg_rft() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_esg_rft().await;
}

#[tokio::test]
async fn test_market_fetch_esg_zd() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_esg_zd().await;
}

#[tokio::test]
async fn test_market_fetch_esg_hz() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_esg_hz().await;
}

#[tokio::test]
async fn test_market_fetch_balance_sheet() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_balance_sheet("test").await;
}

#[tokio::test]
async fn test_market_fetch_profit_sheet() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_profit_sheet("test").await;
}

#[tokio::test]
async fn test_market_fetch_cash_flow_sheet() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_cash_flow_sheet("test").await;
}

#[tokio::test]
async fn test_market_fetch_stock_comments() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_stock_comments().await;
}

#[tokio::test]
async fn test_market_fetch_comment_org_participation() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_comment_org_participation("test").await;
}

#[tokio::test]
async fn test_market_fetch_comment_hist_score() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_comment_hist_score("test").await;
}

#[tokio::test]
async fn test_market_fetch_comment_focus_index() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_comment_focus_index("test").await;
}

#[tokio::test]
async fn test_market_fetch_comment_desire_index() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_comment_desire_index("test").await;
}

#[tokio::test]
async fn test_market_fetch_executive_shareholding() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_executive_shareholding("test").await;
}

#[tokio::test]
async fn test_market_fetch_shareholder_count() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_shareholder_count("test").await;
}

#[tokio::test]
async fn test_market_fetch_shareholder_count_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_shareholder_count_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_industry_category() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_industry_category().await;
}

#[tokio::test]
async fn test_market_fetch_hk_spot() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_spot().await;
}

#[tokio::test]
async fn test_market_fetch_hk_famous_spot() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_famous_spot().await;
}

#[tokio::test]
async fn test_market_fetch_hk_hot_rank() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_hot_rank().await;
}

#[tokio::test]
async fn test_market_fetch_hk_hot_rank_latest() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_hot_rank_latest("test").await;
}

#[tokio::test]
async fn test_market_fetch_hk_hot_rank_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_hot_rank_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_hk_hot_rank_realtime() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_hot_rank_realtime("test").await;
}

#[tokio::test]
async fn test_market_fetch_hk_dividend_payout() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_dividend_payout("test").await;
}

#[tokio::test]
async fn test_market_fetch_hk_fhpx_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_fhpx_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_hk_dividend_yield() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_dividend_yield().await;
}

#[tokio::test]
async fn test_market_fetch_hk_financial_indicators() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_financial_indicators("test").await;
}

#[tokio::test]
async fn test_market_fetch_hk_valuation() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_hk_valuation("test", "test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_us_spot() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_us_spot().await;
}

#[tokio::test]
async fn test_market_fetch_us_famous_spot() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_us_famous_spot("test").await;
}

#[tokio::test]
async fn test_market_fetch_us_pink_spot() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_us_pink_spot().await;
}

#[tokio::test]
async fn test_market_fetch_us_valuation() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_us_valuation("test", "test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_xq_spot() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_xq_spot("test").await;
}

#[tokio::test]
async fn test_market_fetch_billboard_entries() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_billboard_entries("test", 10).await;
}

#[tokio::test]
async fn test_market_fetch_billboard_seats() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_billboard_seats("test", "test", 10).await;
}

#[tokio::test]
async fn test_market_fetch_lhb_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_lhb_detail("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_lhb_stock_statistic() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_lhb_stock_statistic("test").await;
}

#[tokio::test]
async fn test_market_fetch_lhb_jgmmtj() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_lhb_jgmmtj("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_lhb_jgstatistic() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_lhb_jgstatistic("test").await;
}

#[tokio::test]
async fn test_market_fetch_lhb_hyyyb() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_lhb_hyyyb("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_lhb_yybph() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_lhb_yybph("test").await;
}

#[tokio::test]
async fn test_market_fetch_lhb_trader_statistic() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_lhb_trader_statistic("test").await;
}

#[tokio::test]
async fn test_market_fetch_lhb_stock_detail_date() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_lhb_stock_detail_date("test").await;
}

#[tokio::test]
async fn test_market_fetch_lhb_stock_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_lhb_stock_detail("test", "test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_lhb_yyb_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_lhb_yyb_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_fund_flow_individual() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_fund_flow_individual("test").await;
}

#[tokio::test]
async fn test_market_fetch_fund_flow_concept() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_fund_flow_concept("test").await;
}

#[tokio::test]
async fn test_market_fetch_fund_flow_industry() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_fund_flow_industry("test").await;
}

#[tokio::test]
async fn test_market_fetch_main_fund_flow() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_main_fund_flow("test").await;
}

#[tokio::test]
async fn test_market_fetch_margin_account_info() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_margin_account_info("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_margin_sse_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_margin_sse_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_margin_szse_detail() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_margin_szse_detail("test").await;
}

#[tokio::test]
async fn test_market_fetch_margin_ratio_pa() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_margin_ratio_pa("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_margin_sse_summary() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_margin_sse_summary("test", "test").await;
}

#[tokio::test]
async fn test_market_fetch_margin_szse_summary() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_margin_szse_summary("test").await;
}
// === Complex signatures (manually added) ===

#[tokio::test]
async fn test_market_fetch_news() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_news("test", 10, None, None).await;
}

#[tokio::test]
async fn test_market_fetch_global_news() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_global_news("test", "20250101", 7, 10).await;
}

#[tokio::test]
async fn test_market_search_stocks() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.search_stocks("test", None, 10).await;
}

#[tokio::test]
async fn test_market_fetch_quotes_batch() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_quotes_batch(&["test"]).await;
}

#[tokio::test]
async fn test_market_fetch_fundamentals_batch() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_fundamentals_batch(&["test"]).await;
}

#[tokio::test]
async fn test_market_fetch_return_since() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_return_since("test", "20240101", 30).await;
}

#[tokio::test]
async fn test_market_fetch_earnings_forecast() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_earnings_forecast("20250101").await;
}

#[tokio::test]
async fn test_market_fetch_earnings_quick_report() {
    let server = wiremock::MockServer::start().await;
    mount_mocks(&server).await;
    let client = common::mock_market_client(&server).await;
    let _ = client.fetch_earnings_quick_report("20250101").await;
}
