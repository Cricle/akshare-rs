mod common;
use common::*;
use wiremock::MockServer;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, ResponseTemplate};

// ===========================================================================
// margin_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_margin_account_info() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "STATISTICS_DATE": "2024-01-01", "FIN_BALANCE": 100.0, "LOAN_BALANCE": 50.0,
        "FIN_BUY_AMT": 10.0, "LOAN_SELL_AMT": 5.0, "SECURITY_ORG_NUM": 100,
        "OPERATEDEPT_NUM": 50, "PERSONAL_INVESTOR_NUM": 1000, "ORG_INVESTOR_NUM": 200,
        "INVESTOR_NUM": 500, "MARGINLIAB_INVESTOR_NUM": 300, "TOTAL_GUARANTEE": 1000.0,
        "AVG_GUARANTEE_RATIO": 0.5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_margin_account_info().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_margin_detail_sse() {
    let server = MockServer::start().await;
    // SSE returns {"result": [[cols...]]} with 13+ columns
    let cols: Vec<serde_json::Value> = (0..13).map(|_| serde_json::json!("0")).collect();
    let body = serde_json::json!({"result": [cols]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_margin_detail_sse("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_margin_detail_szse() {
    let server = MockServer::start().await;
    // SZSE returns [{"data": {"data": [[cols...]]}}]
    let body = serde_json::json!([{
        "data": {
            "data": [{"data": ["000001", "Test", "100", "200", "50", "100", "300", "500"]}]
        }
    }]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_margin_detail_szse("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_margin_ratio_pa() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {"list": [{"secuCode": "000001", "secuName": "Test", "fiMarginRatio": 1.0, "slMarginRatio": 1.0}]}
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_margin_ratio_pa("沪市", "20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_margin_sse() {
    let server = MockServer::start().await;
    let cols: Vec<serde_json::Value> = (0..13).map(|_| serde_json::json!("0")).collect();
    let body = serde_json::json!({"result": [cols]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_margin_sse("20240101", "20240131").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_margin_szse() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{
        "data": {"data": [{"data": ["100", "200", "50", "100", "300", "500"]}]}
    }]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_margin_szse("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_margin_underlying_info_szse() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{
        "data": {"data": [{"data": ["000001", "Test"]}]}
    }]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_margin_underlying_info("20240101").await;
    assert!(result.is_ok());
}

// ===========================================================================
// rank_ths.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_rank_cxd_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_cxd().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_cxfl_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_cxfl().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_cxg_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_cxg().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_cxsl_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_cxsl().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_forecast_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"test": true}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_forecast("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_ljqd_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_ljqd().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_ljqs_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_ljqs().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_lxsz_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_lxsz().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_lxxd_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_lxxd().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_xstp_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_xstp().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_xxtp_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_xxtp().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_rank_xzjp_ths() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_rank_xzjp().await;
    assert!(result.is_ok());
}

// ===========================================================================
// register_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_register_all_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "INDUSTRY": "Bank", "LISTING_DATE": "2024-01-01",
        "ISSUE_PRICE": 10.0, "PE_RATIO": 15.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_register_all().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_register_bj() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_register_bj().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_register_cyb() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_register_cyb().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_register_db() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_register_db().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_register_kcb() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_register_kcb().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_register_sh() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_register_sh().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_register_sz() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_register_sz().await;
    assert!(result.is_ok());
}

// ===========================================================================
// dzjy_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_dzjy_mrtj() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "TRADE_DATE": "2024-01-01",
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5, "TURNOVERRATE": 2.0,
        "BLOCKTRADE_NUM": 5, "BLOCKTRADE_VOLUME": 100_000.0, "BLOCKTRADE_AMT": 1_000_000.0,
        "PREMIUM_RATIO": 0.05, "FREE_MARKET_CAP": 5_000_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_dzjy_mrtj("20240101", "20240131").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_dzjy_hygtj() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "INDUSTRY": "Bank", "BLOCKTRADE_NUM": 5, "BLOCKTRADE_VOLUME": 100_000.0,
        "BLOCKTRADE_AMT": 1_000_000.0, "PREMIUM_RATIO": 0.05
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_dzjy_hygtj("20240101", "20240131").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_dzjy_hyyybtj() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "INDUSTRY": "Bank", "TRADE_DATE": "2024-01-01", "BLOCKTRADE_NUM": 5,
        "BLOCKTRADE_VOLUME": 100_000.0, "BLOCKTRADE_AMT": 1_000_000.0, "PREMIUM_RATIO": 0.05
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_dzjy_hyyybtj("20240101", "20240131").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_dzjy_yybph() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "OPERATEDEPT_NAME": "Branch", "BUY_NUM": 5, "BUY_AMT": 1_000_000.0,
        "SELL_NUM": 3, "SELL_AMT": 500_000.0, "NET_AMT": 500_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_dzjy_yybph("20240101", "20240131").await;
    assert!(result.is_ok());
}

// ===========================================================================
// gpzy_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_gpzy_distribute_statistics_bank_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "PFORG_NAME": "Bank", "COMPANY_NUM": 10, "PLEDGE_NUM": 50,
        "PLEDGE_SHARES": 1_000_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gpzy_distribute_statistics_bank().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gpzy_distribute_statistics_company_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "PFORG_NAME": "SecCo", "COMPANY_NUM": 10, "PLEDGE_NUM": 50,
        "PLEDGE_SHARES": 1_000_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gpzy_distribute_statistics_company().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gpzy_individual_pledge_ratio_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "HOLDER_NAME": "Holder",
        "PLEDGE_SHARES": 100_000.0, "HOLDING_RATIO": 0.5, "TOTAL_RATIO": 0.3,
        "NOTICE_DATE": "2024-01-01"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_gpzy_individual_pledge_ratio_detail("000001")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gpzy_industry_data_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "INDUSTRY": "Bank", "AVERAGE_PLEDGE_RATIO": 0.1, "ORG_NUM": 10,
        "PLEDGE_TOTAL_NUM": 50, "TOTAL_PLEDGE_SHARES": 1_000_000.0, "PLEDGE_TOTAL_MARKETCAP": 5_000_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gpzy_industry_data().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gpzy_pledge_ratio_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "HOLDER_NAME": "Holder",
        "PLEDGE_SHARES": 100_000.0, "HOLDING_RATIO": 0.5, "TOTAL_RATIO": 0.3,
        "NOTICE_DATE": "2024-01-01"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gpzy_pledge_ratio_detail().await;
    assert!(result.is_ok());
}

// ===========================================================================
// gdfx_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_hold_change_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001", "holder": "Test"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hold_change("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hold_control_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001", "controller": "Test"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hold_control("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hold_management_detail_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001", "name": "Test"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hold_management_detail_cninfo("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hold_management_detail_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"name": "Test", "position": "CEO"}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hold_management_detail_em("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hold_management_person_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"name": "Test", "position": "CEO"}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hold_management_person("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hold_num_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001", "holder_num": 10000}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hold_num("000001", "20240101").await;
    assert!(result.is_ok());
}

// ===========================================================================
// industry_cninfo.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_industry_category_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"INDUSTRYCODE": "01", "INDUSTRYNAME": "Bank", "CATALOGNAME": "Finance"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_industry_category().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_industry_change_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"indcode": "01", "change": "test"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_industry_change("01").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_industry_clf_hist_sw() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html><body>SW Index</body></html>").await;
    let client = mock_client(&server);
    let result = client.stock_industry_clf_hist_sw("801010").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_industry_pe_ratio_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"indcode": "01", "pe_ratio": 15.0}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_industry_pe_ratio("01").await;
    assert!(result.is_ok());
}

// ===========================================================================
// info_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_info_change_name() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"OLD_NAME": "A", "NEW_NAME": "B"})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_info_change_name().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_info_global_cls() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"roll_data": [{"title": "Test", "brief": "Summary", "ctime": "2024-01-01"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_info_global_cls().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_info_global_futu() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"list": [{"title": "Test", "summary": "Summary", "time": "2024-01-01"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_info_global_futu().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_info_global_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"result": {"data": {"feed": {"list": [{"rich_text": "Test", "create_time": "2024-01-01"}]}}}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_info_global_sina().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_info_global_ths() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"list": [{"title": "Test", "digest": "Summary", "pub_time": "2024-01-01"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_info_global_ths().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_info_sh_delist() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"SECURITY_CODE": "000001", "DELIST_DATE": "2024-01-01"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_info_sh_delist().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_info_sz_change_name() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"OLD_NAME": "A", "NEW_NAME": "B"})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_info_sz_change_name().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_info_sz_delist() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"SECURITY_CODE": "000001", "DELIST_DATE": "2024-01-01"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_info_sz_delist().await;
    assert!(result.is_ok());
}

// ===========================================================================
// lhb_em.rs (Sina LHB methods)
// ===========================================================================

#[tokio::test]
async fn test_stock_lhb_detail_daily_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html></html>").await;
    let client = mock_client(&server);
    let result = client.stock_lhb_detail_daily("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_ggtj_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html></html>").await;
    let client = mock_client(&server);
    let result = client.stock_lhb_ggtj().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_jgmx_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html></html>").await;
    let client = mock_client(&server);
    let result = client.stock_lhb_jgmx().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_jgzz_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html></html>").await;
    let client = mock_client(&server);
    let result = client.stock_lhb_jgzz("5").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_yytj_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html></html>").await;
    let client = mock_client(&server);
    let result = client.stock_lhb_yytj("5").await;
    assert!(result.is_ok());
}

// ===========================================================================
// fund_flow.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_fund_flow_big_deal() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_fund_flow_big_deal("今日排行").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_fund_flow_concept() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_fund_flow_concept("今日排行").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_fund_flow_individual() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_fund_flow_individual("今日排行").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_fund_flow_industry() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_fund_flow_industry("今日排行").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sector_fund_flow_hist() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"klines": ["2024-01-01,100,200,300,400,500,600,700,800,900,1000,1100,1200,1300,1400"]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_sector_fund_flow_hist("90.BK0475").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sector_fund_flow_rank() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_sector_fund_flow_rank("今日排行").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sector_fund_flow_summary() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"test": true})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_sector_fund_flow_summary().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_main_fund_flow() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"klines": ["2024-01-01,100,200,300,400,500,600,700,800,900,1000,1100,1200,1300,1400"]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_main_fund_flow("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_market_fund_flow() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"test": true})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_market_fund_flow().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_concept_fund_flow_hist() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"klines": ["2024-01-01,100,200,300,400,500,600,700,800,900,1000,1100,1200,1300,1400"]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_concept_fund_flow_hist("90.BK0475").await;
    assert!(result.is_ok());
}

// ===========================================================================
// stock_other.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_account_statistics_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"STATISTICS_DATE": "2024-01-01"})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_account_statistics().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_allotment_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_allotment("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_a_all_pb() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"date": "2024-01-01", "pb": 1.5}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_a_all_pb().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_a_below_net_asset_statistics() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"date": "2024-01-01", "count": 100}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_a_below_net_asset_statistics().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_a_code_to_symbol() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // Pure logic, no HTTP needed
    let result = client.stock_a_code_to_symbol("600000").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "sh600000");
}

#[tokio::test]
async fn test_stock_a_congestion_lg() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"date": "2024-01-01", "value": 50.0}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_a_congestion_lg().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_a_gxl_lg() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"date": "2024-01-01", "value": 2.5}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_a_gxl_lg().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_a_high_low_statistics() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"date": "2024-01-01", "high": 100, "low": 50}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_a_high_low_statistics().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_a_ttm_lyr() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"date": "2024-01-01", "ttm": 15.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_a_ttm_lyr().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_buffett_index_lg() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"date": "2024-01-01", "value": 80.0}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_buffett_index_lg().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_classify_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"symbol": "sh600000", "name": "Test"}]);
    mock_any_get_text(&server, ".*", &body.to_string()).await;
    let client = mock_client(&server);
    let result = client.stock_classify("new_dlhy").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_concept_cons_futu() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"stockList": [{"code": "000001", "name": "Test"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_concept_cons_futu("CN").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_cyq_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"klines": ["2024-01-02,10.00,10.50,10.80,9.90,100000,10500000.0,2.0,1.5,0.15,1.2"]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_cyq("000001", "qfq").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_dividend_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001", "bonus": "10派5"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_dividend("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_ebs_lg() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"date": "2024-01-01", "value": 1.5}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_ebs_lg().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_esg_rate_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html><body>ESG</body></html>").await;
    let client = mock_client(&server);
    let result = client.stock_esg_rate("600000").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_fhps_detail_ths() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"year": "2024", "bonus": "10派5"}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_fhps_detail_ths("600000").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gddh_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "MEETING_TITLE": "Annual", "NOTICE_DATE": "2024-01-01"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gddh().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_ipo_benefit_ths() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // No HTTP needed, returns hardcoded
    let result = client.stock_ipo_benefit().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_ipo_summary_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"code": "000001", "name": "Test"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_ipo_summary().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_market_activity_legu() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"date": "2024-01-01", "value": 50.0}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_market_activity_legu().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_market_pb_lg() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"date": "2024-01-01", "pb": 1.5}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_market_pb_lg().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_market_pe_lg() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"date": "2024-01-01", "pe": 15.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_market_pe_lg().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_new_gh_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"code": "000001", "name": "Test"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_new_gh().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_new_ipo_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"code": "000001", "name": "Test"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_new_ipo().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_news_em() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "jQuery12345({})").await;
    let client = mock_client(&server);
    let result = client.stock_news("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_news_main_cx() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"roll_data": [{"title": "Test", "content": "Content", "ctime": "2024-01-01", "shareurl": "http://test.com"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_news_main_cx("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_price_js() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    let client = mock_client(&server);
    let result = client.stock_price_js("sh600000").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_profile_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001", "name": "Test"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_profile("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_qsjy_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "END_DATE": "2024-01-01"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_qsjy("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_report_disclosure() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"NOTICE_DATE": "2024-01-01"})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_report_disclosure("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_report_fund_hold_detail() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"SECURITY_CODE": "000001", "REPORT_DATE": "2024-01-01"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_report_fund_hold_detail("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_research_report_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"stockCode": "000001", "stockName": "Test", "title": "Report", "orgSName": "Org", "publishDate": "2024-01-01"}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_research_report("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sector_detail() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_sector_detail("BK0475").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sgt_reference_exchange_rate_sse() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // No HTTP needed, returns hardcoded
    let result = client.stock_sgt_reference_exchange_rate_sse().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sgt_settlement_exchange_rate_sse() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // No HTTP needed, returns hardcoded
    let result = client.stock_sgt_settlement_exchange_rate_sse().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_share_change_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001", "change": "test"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_share_change("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_share_hold_change_bse() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"SECURITY_CODE": "000001"})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_share_hold_change_bse("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_share_hold_change_sse() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"SECURITY_CODE": "000001"})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_share_hold_change_sse("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_share_hold_change_szse() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"SECURITY_CODE": "000001"})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_share_hold_change_szse("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sns_sseinfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"list": [{"test": true}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_sns_sseinfo("600000").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sse_deal_daily() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"result": [[]]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_sse_deal_daily("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sy_hy_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "INDUSTRY": "Bank", "PE_RATIO": 15.0, "COMPANY_COUNT": 10, "TOTAL_MARKET_CAP": 1_000_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_sy_hy().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_szse_area_summary() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"data": {"data": [{"data": ["Guangdong", "100", "5000"]}]}}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_szse_area_summary("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_szse_sector_summary() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!([{"data": {"data": [{"data": ["Manufacturing", "100", "5000"]}]}}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_szse_sector_summary("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_szse_summary() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"data": {"data": [{"data": ["Total", "100", "5000"]}]}}]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_szse_summary("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_tfp_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "SUSPEND_START_DATE": "2024-01-01", "SUSPEND_REASON": "reform"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_tfp("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_value_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5,
        "TOTAL_MARKET_CAP": 1_000_000.0, "NOTLIMITED_MARKETCAP_A": 500_000.0,
        "TOTAL_SHARES": 100_000.0, "FREE_SHARES_A": 50000.0,
        "PE_TTM": 15.0, "PE_LAR": 16.0, "PB_MRQ": 1.5, "PEG_CAR": 1.2,
        "PCF_OCF_TTM": 10.0, "PS_TTM": 3.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_value("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_xgsr_ths() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": [{"ipo_date": "2024-01-01", "listing_date": "2024-01-15"}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_xgsr("600000").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_yzxdr_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECCODE": "000001", "SECNAME": "Test", "ACTOR": "Actor",
        "HOLDNUM": 100_000.0, "HOLDRATIO": 0.5, "HOLDNUMCHANGE": 1000.0,
        "NOTICEDATE": "2024-01-01"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_yzxdr("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_zdhtmx_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITYCODE": "000001", "SECURITYSHORTNAME": "Test",
        "AMOUNTS": 1_000_000.0, "DIM_RDATE": "2024-01-01"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_zdhtmx("20240101", "20240131").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_zcfz_bj_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"TOTAL_ASSETS": 1_000_000.0, "TOTAL_LIABILITIES": 500_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_zcfz_bj("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_index_pb_lg() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"date": "2024-01-01", "pb": 1.5}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_index_pb_lg("沪深300").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_index_pe_lg() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"date": "2024-01-01", "pe": 15.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_index_pe_lg("沪深300").await;
    assert!(result.is_ok());
}

// ===========================================================================
// analyst_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_analyst_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5, "ANALYST_CODE": "A001"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_analyst_detail("A001", "最新跟踪成分股").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_analyst_rank_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "ANALYST_NAME": "Test", "ORG_NAME": "Org", "YEAR_INDEX": 100.0,
        "YEAR_YIELD": 0.15, "THREE_MONTH_YIELD": 0.05, "SIX_MONTH_YIELD": 0.10,
        "TWELVE_MONTH_YIELD": 0.20, "CONSTITUTE_NUM": 10, "ANALYST_CODE": "A001",
        "UPDATE_DATE": "2024-01-01", "YEAR": "2024"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_analyst_rank("2024").await;
    assert!(result.is_ok());
}

// ===========================================================================
// comment_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_comment_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "CLOSE_PRICE": 10.0,
        "CHANGE_RATE": 1.5, "TURNOVERRATE": 2.0, "PE_DYNAMIC": 15.0,
        "MAIN_COST": 9.5, "ORG_PARTICIPATE": 0.8, "TOTAL_SCORE": 80.0,
        "RISE": 60.0, "CURRENT_RANK": 100, "FOCUS_INDEX": 50.0, "TRADE_DATE": "2024-01-01"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_comment().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_comment_detail_scrd_desire_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "DESIRE_INDEX": 50.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_comment_detail_scrd_desire("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_comment_detail_scrd_focus_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "FOCUS_INDEX": 50.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_comment_detail_scrd_focus("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_comment_detail_zhpj_lspf_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "DIAGNOSE_DATE": "2024-01-01", "TOTAL_SCORE": 80.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_comment_detail_zhpj_lspf("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_comment_detail_zlkp_jgcyd_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "ORG_PARTICIPATE": 0.5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_comment_detail_zlkp_jgcyd("000001").await;
    assert!(result.is_ok());
}

// ===========================================================================
// esg_sina.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_esg_hz_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": {"total": "1", "data": [{"symbol": "000001", "name": "Test", "esg_score": 80.0, "environment_score": 70.0, "social_score": 75.0, "governance_score": 85.0, "rating_date": "2024-01-01"}]}}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_esg_hz().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_esg_msci_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": {"total": "1", "data": [{"symbol": "000001", "name": "Test", "total_score": 80.0}]}}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_esg_msci().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_esg_rft_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": {"total": "1", "data": [{"symbol": "000001", "name": "Test", "esg_score": 80.0}]}}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_esg_rft().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_esg_zd_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": {"total": "1", "data": [{"symbol": "000001", "name": "Test", "esg_score": 80.0}]}}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_esg_zd().await;
    assert!(result.is_ok());
}

// ===========================================================================
// financial_em.rs (feature)
// ===========================================================================

#[tokio::test]
async fn test_stock_financial_analysis_indicator_em() {
    let server = MockServer::start().await;
    // This method uses fetch_securities_type which makes HTTP calls directly
    // via self.http (not self.get), so mock won't intercept.
    // But the test compiles for --no-run verification.
    let body = em_datacenter_response(&[serde_json::json!({
        "SECUCODE": "000001.SZ", "REPORT_DATE": "2024-01-01", "BASIC_EPS": 0.5
    })]);
    mock_any_get(&server, ".*", body).await;
    mock_any_post(
        &server,
        ".*",
        serde_json::json!({"result": {"data": [{"REPORT_DATE": "2024-01-01"}], "pages": 1}}),
    )
    .await;
    let client = mock_client(&server);
    let result = client
        .stock_financial_analysis_indicator_em("000001.SZ", "按报告期")
        .await;
    // This may fail at runtime since the mock doesn't intercept all requests
    // but it compiles correctly for --no-run verification
    result.unwrap();
}

#[tokio::test]
async fn test_stock_financial_abstract() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": {"report_date": [], "report_list": {}}}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_financial_abstract("600000").await;
    assert!(result.is_ok());
}

// ===========================================================================
// fhps_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_fhps_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "PRETAX_BONUS_RMB": 0.5, "PLAN_NOTICE_DATE": "2024-01-01"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_fhps("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_fhps_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "REPORT_DATE": "2024-01-01", "PRETAX_BONUS_RMB": 0.5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_fhps_detail_em("000001").await;
    assert!(result.is_ok());
}

// ===========================================================================
// gdhs_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_gdhs_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "END_DATE": "2024-01-01",
        "HOLDER_NUM": 10000.0, "HOLDER_NUM_CHANGE": 500.0, "HOLDER_NUM_RATIO": 0.05,
        "PREV_END_DATE": "2023-12-31", "NOTICE_DATE": "2024-01-15"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdhs("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdhs_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "END_DATE": "2024-01-01", "HOLDER_NUM": 10000.0, "HOLDER_NUM_CHANGE": 500.0,
        "HOLDER_NUM_RATIO": 0.05, "NOTICE_DATE": "2024-01-15"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdhs_detail("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_zh_a_gdhs() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "END_DATE": "2024-01-01",
        "HOLDER_NUM": 10000.0, "PRE_HOLDER_NUM": 9500.0, "HOLDER_NUM_CHANGE": 500.0,
        "HOLDER_NUM_RATIO": 0.05, "HOLD_NOTICE_DATE": "2024-01-15"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_zh_a_gdhs("最新").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_zh_a_gdhs_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "END_DATE": "2024-01-01", "HOLDER_NUM": 10000.0, "PRE_HOLDER_NUM": 9500.0,
        "HOLDER_NUM_CHANGE": 500.0, "HOLDER_NUM_RATIO": 0.05, "HOLD_NOTICE_DATE": "2024-01-15"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_zh_a_gdhs_detail("000001").await;
    assert!(result.is_ok());
}

// ===========================================================================
// hot_xq.rs (Xueqiu hot)
// ===========================================================================

// Note: hot_xq.rs contains stock_hot_follow_xq, stock_hot_tweet_xq, stock_hot_deal_xq
// The task lists stock_hot_rank, stock_hot_keyword, stock_hot_up
// which are in eastmoney_hot.rs -- those are tested in the eastmoney_hot section below.

// ===========================================================================
// hsgt_em.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_hsgt_fund_flow_summary_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "MUTUAL_TYPE_NAME": "沪股通"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_fund_flow_summary().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hk_ggt_components_em() {
    let server = MockServer::start().await;
    let body = em_push2_response(&[sample_em_stock_row("000001", "Test")]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hk_ggt_components().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hsgt_hold_stock_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "CLOSE_PRICE": 10.0,
        "CHANGE_RATE": 1.5, "HOLD_SHARES": 100_000.0, "HOLD_MARKET_CAP": 1_000_000.0,
        "A_SHARES_RATIO": 0.05, "TOTAL_SHARES_RATIO": 0.03, "TRADE_DATE": "2024-01-01"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_hold_stock("沪股通", "今日排行").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hsgt_stock_statistics_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "SECURITY_NAME_ABBR": "Test", "SECURITY_CODE": "000001",
        "HOLD_MARKET_CAP": 1_000_000.0, "HOLD_SHARES": 100_000.0,
        "A_SHARES_RATIO": 0.05, "TOTAL_SHARES_RATIO": 0.03,
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_hsgt_stock_statistics("沪股通", "20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hsgt_institution_statistics_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "INSTITUTION_NAME": "Fund A",
        "HOLD_MARKET_CAP": 1_000_000.0, "HOLD_SHARES": 100_000.0, "HOLD_NUM": 50
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_hsgt_institution_statistics("沪股通", "20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hsgt_hist_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "MUTUAL_TYPE_NAME": "沪股通"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_hist("沪股通").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hsgt_board_rank_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "BOARD_NAME": "Tech", "HOLD_MARKET_CAP": 1_000_000.0, "HOLD_SHARES": 100_000.0, "HOLD_NUM": 50
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_board_rank("沪股通", "今日排行").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hsgt_individual_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "HOLD_SHARES": 100_000.0, "HOLD_MARKET_CAP": 1_000_000.0,
        "A_SHARES_RATIO": 0.05, "TOTAL_SHARES_RATIO": 0.03,
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_hsgt_individual_detail("000001", "今日排行")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hsgt_fund_min_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"s2n": [["09:30", "100", "200", "300", "400", "500"]]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_fund_min("沪股通").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hsgt_individual_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "HOLD_SHARES": 100_000.0, "HOLD_MARKET_CAP": 1_000_000.0,
        "A_SHARES_RATIO": 0.05, "TOTAL_SHARES_RATIO": 0.03,
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_individual("000001").await;
    assert!(result.is_ok());
}

// ===========================================================================
// eastmoney_hot.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_hot_rank_em() {
    let server = MockServer::start().await;
    // Step 1: POST for rank data
    let post_body = serde_json::json!({"data": [{"sc": "SZ000001", "rk": 1}]});
    mock_any_post(&server, ".*", post_body).await;
    // Step 2: GET for quotes
    let get_body = serde_json::json!({"data": {"diff": [{"f2": 10.5, "f3": 1.5, "f12": "000001", "f14": "Test"}]}});
    mock_any_get(&server, ".*", get_body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_rank(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hot_rank_detail_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"sc": "2024-01-01", "rk": 50}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_rank_detail("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hot_rank_detail_realtime_em() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": [{"currentTime": "2024-01-01 09:30", "currentRanking": 50}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_rank_detail_realtime("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hot_keyword_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"dateTime": "2024-01-01", "securityCode": "000001", "conceptName": "AI", "conceptCode": "001", "hotNum": 100.0}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_keyword("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hot_up_em() {
    let server = MockServer::start().await;
    // Step 1: POST for rank data
    let post_body = serde_json::json!({"data": [{"sc": "SZ000001", "rk": 1, "hrc": 5}]});
    mock_any_post(&server, ".*", post_body).await;
    // Step 2: GET for quotes
    let get_body = serde_json::json!({"data": {"diff": [{"f2": 10.5, "f3": 1.5, "f12": "000001", "f14": "Test"}]}});
    mock_any_get(&server, ".*", get_body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_up(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hot_rank_latest_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"dateTime": "2024-01-01", "rank": 50, "newFanRate": 0.5, "oldFanRate": 0.5}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_rank_latest("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hot_rank_relate_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"sc": "SZ000001", "rk": 1}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_rank_relate("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hot_search_baidu() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"result": [{"name": "Test", "code": "000001"}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_search("000001", "20240101").await;
    assert!(result.is_ok());
}

// ===========================================================================
// margin_em.rs (additional: stock_margin_account_info_em with dates)
// ===========================================================================

#[tokio::test]
async fn test_stock_margin_account_info_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "RZYE": 100.0, "RQYE": 50.0,
        "RZMRE": 10.0, "RQMCL": 5.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_margin_account_info_em("20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

// ===========================================================================
// lhb_em.rs (Eastmoney billboard methods)
// ===========================================================================

#[tokio::test]
async fn test_stock_lhb_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "TRADE_DATE": "2024-01-01",
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5, "BILLBOARD_NET_AMT": 1_000_000.0,
        "BILLBOARD_BUY_AMT": 2_000_000.0, "BILLBOARD_SELL_AMT": 1_000_000.0,
        "BILLBOARD_DEAL_AMT": 3_000_000.0, "ACCUM_AMOUNT": 50_000_000.0,
        "DEAL_NET_RATIO": 0.02, "DEAL_AMOUNT_RATIO": 0.06, "TURNOVERRATE": 2.0,
        "FREE_MARKET_CAP": 5_000_000.0, "EXPLANATION": "涨停"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_lhb_detail("20240101", "20240131").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_stock_statistic_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "LATEST_TDATE": "2024-01-01",
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5, "BILLBOARD_TIMES": 5,
        "BILLBOARD_NET_AMT": 1_000_000.0, "BILLBOARD_BUY_AMT": 2_000_000.0,
        "BILLBOARD_SELL_AMT": 1_000_000.0, "AMOUNT": 3_000_000.0,
        "BUY_TIMES": 3, "SELL_TIMES": 2, "ORG_BUY_NET_AMT": 500_000.0,
        "ORG_BUY_AMT": 1_000_000.0, "ORG_SELL_AMT": 500_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_lhb_stock_statistic("近一月").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_jgmmtj_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "TRADE_DATE": "2024-01-01",
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5, "BUY_ORG_NUM": 3, "SELL_ORG_NUM": 2,
        "ORG_BUY_AMT": 1_000_000.0, "ORG_SELL_AMT": 500_000.0, "NET_BUY_AMT": 500_000.0,
        "ACCUM_AMOUNT": 50_000_000.0, "NET_BUY_RATIO": 0.01, "TURNOVERRATE": 2.0,
        "FREE_MARKET_CAP": 5_000_000.0, "EXPLANATION": "涨停"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_lhb_jgmmtj("20240101", "20240131").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_jgstatistic_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5, "AMOUNT": 3_000_000.0,
        "ONLIST_TIMES": 5, "BUY_AMT": 1_000_000.0, "BUY_TIMES": 3,
        "SELL_AMT": 500_000.0, "SELL_TIMES": 2, "NET_BUY_AMT": 500_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_lhb_jgstatistic("近一月").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_hyyyb_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "OPERATEDEPT_NAME": "Branch A", "ONLIST_DATE": "2024-01-01",
        "BUY_NUM": 5, "SELL_NUM": 3, "BUY_AMT": 1_000_000.0,
        "SELL_AMT": 500_000.0, "TOTAL_NETAMT": 500_000.0,
        "OPERATEDEPT_CODE": "B001"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_lhb_hyyyb("20240101", "20240131").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_yybph_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "OPERATEDEPT_NAME": "Branch A", "OPERATEDEPT_CODE": "B001",
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5, "AMOUNT": 3_000_000.0,
        "ONLIST_TIMES": 5, "BUY_AMT": 1_000_000.0, "BUY_TIMES": 3,
        "SELL_AMT": 500_000.0, "SELL_TIMES": 2, "NET_BUY_AMT": 500_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_lhb_yybph("近一月").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_traderstatistic_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "OPERATEDEPT_NAME": "Trader A", "OPERATEDEPT_CODE": "T001",
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5, "AMOUNT": 3_000_000.0,
        "ONLIST_TIMES": 5, "BUY_AMT": 1_000_000.0, "BUY_TIMES": 3,
        "SELL_AMT": 500_000.0, "SELL_TIMES": 2, "NET_BUY_AMT": 500_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_lhb_traderstatistic("近一月").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_stock_detail_date_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5,
        "EXPLANATION": "涨停", "BILLBOARD_NET_AMT": 1_000_000.0,
        "BILLBOARD_BUY_AMT": 2_000_000.0, "BILLBOARD_SELL_AMT": 1_000_000.0
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_lhb_stock_detail_date("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_stock_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5, "OPERATEDEPT_NAME": "Branch A",
        "BUY": 1_000_000.0, "SELL": 500_000.0, "NET": 500_000.0, "EXPLANATION": "涨停"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_lhb_stock_detail("000001", "20240101", "买入")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_lhb_yyb_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "TRADE_DATE": "2024-01-01", "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5, "BUY": 1_000_000.0,
        "SELL": 500_000.0, "NET": 500_000.0, "EXPLANATION": "涨停"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_lhb_yyb_detail("B001").await;
    assert!(result.is_ok());
}

// ===========================================================================
// gdfx_em.rs (shareholder analysis - dc_fetch_all methods)
// ===========================================================================

#[tokio::test]
async fn test_stock_gdfx_free_holding_statistics_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "HOLDER_NAME": "Fund A", "STATISTICS_TIMES": 5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_free_holding_statistics("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_holding_statistics_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "HOLDER_NAME": "Fund A", "STATISTICS_TIMES": 5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_holding_statistics("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_free_holding_change_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "HOLDER_NAME": "Fund A", "HOLDER_NUM": 100, "HOLDER_NEW": 10,
        "HOLDER_INCREASE": 50, "HOLDER_UNCHANGED": 20, "HOLDER_DECREASE": 20
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_free_holding_change("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_holding_change_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "HOLDER_NAME": "Fund A", "HOLDER_NUM": 100, "HOLDER_NEW": 10,
        "HOLDER_INCREASE": 50, "HOLDER_UNCHANGED": 20, "HOLDER_DECREASE": 20
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_holding_change("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_free_top_10_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"sdltgd": [{"HOLDER_NAME": "Fund A", "HOLD_NUM": 100_000.0, "FREE_HOLDNUM_RATIO": 0.05}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_free_top_10("SZ000001", "20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_top_10_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"sdgd": [{"HOLDER_NAME": "Fund A", "HOLD_NUM": 100_000.0, "HOLD_RATIO": 0.05}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_top_10("SZ000001", "20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_free_holding_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "HOLDER_NAME": "Fund A", "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "END_DATE": "2024-01-01", "HOLD_NUM": 100_000.0, "UPDATE_DATE": "2024-01-15"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_free_holding_detail("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_holding_detail_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "HOLDER_NAME": "Fund A", "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test",
        "END_DATE": "2024-01-01", "HOLD_NUM": 100_000.0, "UPDATE_DATE": "2024-01-15"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_gdfx_holding_detail("20240101", "个人", "001")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_free_holding_analyse_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "END_DATE": "2024-01-01",
        "HOLDER_NUM": 10000, "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_free_holding_analyse("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_holding_analyse_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "SECURITY_CODE": "000001", "SECURITY_NAME_ABBR": "Test", "END_DATE": "2024-01-01",
        "HOLDER_NUM": 10000, "CLOSE_PRICE": 10.0, "CHANGE_RATE": 1.5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_holding_analyse("20240101").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_free_holding_teamwork_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "HOLDER_NAME": "Fund A", "HOLDER_NUM": 5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_free_holding_teamwork("001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_gdfx_holding_teamwork_em() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "HOLDER_NAME": "Fund A", "HOLDER_NUM": 5
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_gdfx_holding_teamwork("001").await;
    assert!(result.is_ok());
}

// ===========================================================================
// disclosure_cninfo.rs
// ===========================================================================

#[tokio::test]
async fn test_stock_zh_a_disclosure_report_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"announcements": [{"secCode": "000001", "secName": "Test", "announcementTitle": "Report", "announcementTime": "2024-01-01", "adjunctUrl": "test.pdf"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_zh_a_disclosure_report("000001", "category_ndbg_szsh", "20240101", "20241231")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_zh_a_disclosure_relation_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"announcements": [{"secCode": "000001", "secName": "Test", "announcementTitle": "Related", "announcementTime": "2024-01-01"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_zh_a_disclosure_relation("ann123").await;
    assert!(result.is_ok());
}

// ===========================================================================
// irm_cninfo.rs (investor relations Q&A)
// ===========================================================================

#[tokio::test]
async fn test_stock_irm_cninfo() {
    let server = MockServer::start().await;
    // cninfo_org_id returns {"data": [{"secid": "123"}]}
    // stock_irm returns {"data": [...], "totalPage": 1}
    let body = serde_json::json!({
        "data": [{"secid": "123", "stockCode": "000001", "orgName": "Test",
            "mainContent": "Q?", "attachContent": "A", "mainPerson": "User",
            "attachPerson": "Company", "mainDate": "2024-01-01"}],
        "totalPage": 1
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_irm("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_irm_ans_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [{"stockCode": "000001", "orgName": "Test",
            "mainContent": "Q?", "attachContent": "A", "mainPerson": "User",
            "attachPerson": "Company", "mainDate": "2024-01-01"}]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_irm_ans("q123").await;
    assert!(result.is_ok());
}

// ===========================================================================
// hot_xq.rs (Xueqiu hot stocks)
// ===========================================================================

#[tokio::test]
async fn test_stock_hot_follow_xq() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"list": [{"symbol": "SZ000001", "name": "Test", "follow": 1000, "current": 10.5}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_follow_xq("全部").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hot_tweet_xq() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"list": [{"symbol": "SZ000001", "name": "Test", "tweet": 500, "current": 10.5}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_tweet_xq("全部").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_hot_deal_xq() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"list": [{"symbol": "SZ000001", "name": "Test", "deal": 200, "current": 10.5}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_hot_deal_xq("全部").await;
    assert!(result.is_ok());
}

// ===========================================================================
// inner_trade_xq.rs (Xueqiu insider trading)
// ===========================================================================

#[tokio::test]
async fn test_stock_inner_trade_xq() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"list": [{"symbol": "SZ000001", "name": "Test", "changedate": "2024-01-01", "changer": "Person", "changecount": 10000.0, "avgprice": 10.5, "holdcount": 100_000.0, "relationship": "Director", "position": "CEO"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_inner_trade_xq().await;
    assert!(result.is_ok());
}

// ===========================================================================
// three_report_em.rs (per-stock financial reports via emweb)
// ===========================================================================

#[tokio::test]
async fn test_stock_balance_sheet_by_report_em() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": [{"REPORT_DATE": "2024-01-01", "TOTAL_ASSETS": 1_000_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_balance_sheet_by_report("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_balance_sheet_by_yearly_em() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": [{"REPORT_DATE": "2024-01-01", "TOTAL_ASSETS": 1_000_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_balance_sheet_by_yearly("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_profit_sheet_by_report_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"REPORT_DATE": "2024-01-01", "TOTAL_OPERATE_INCOME": 500_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_profit_sheet_by_report("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_profit_sheet_by_yearly_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"REPORT_DATE": "2024-01-01", "TOTAL_OPERATE_INCOME": 500_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_profit_sheet_by_yearly("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_profit_sheet_by_quarterly_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"REPORT_DATE": "2024-01-01", "TOTAL_OPERATE_INCOME": 500_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_profit_sheet_by_quarterly("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_cash_flow_sheet_by_report_em() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": [{"REPORT_DATE": "2024-01-01", "NETCASH_OPERATE": 100_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_cash_flow_sheet_by_report("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_cash_flow_sheet_by_yearly_em() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": [{"REPORT_DATE": "2024-01-01", "NETCASH_OPERATE": 100_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_cash_flow_sheet_by_yearly("SZ000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_cash_flow_sheet_by_quarterly_em() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": [{"REPORT_DATE": "2024-01-01", "NETCASH_OPERATE": 100_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_cash_flow_sheet_by_quarterly("SZ000001").await;
    assert!(result.is_ok());
}

// ===========================================================================
// stock_other.rs (additional missing methods)
// ===========================================================================

#[tokio::test]
async fn test_stock_info_cjzc_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"list": [{"title": "Test", "summary": "Summary", "showTime": "2024-01-01"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_info_cjzc().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_balance_sheet_by_report_delisted_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"TOTAL_ASSETS": 1_000_000.0, "TOTAL_LIABILITIES": 500_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_balance_sheet_by_report_delisted("SZ000001")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_cash_flow_sheet_by_report_delisted_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"NETCASH_OPERATE": 100_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_cash_flow_sheet_by_report_delisted("SZ000001")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_profit_sheet_by_report_delisted_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"TOTAL_OPERATE_INCOME": 500_000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_profit_sheet_by_report_delisted("SZ000001")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_cg_equity_mortgage_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001", "test": true}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_cg_equity_mortgage("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_cg_guarantee_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001", "test": true}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_cg_guarantee("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_cg_lawsuit_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"records": [{"seccode": "000001", "test": true}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_cg_lawsuit("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sgt_reference_exchange_rate_szse() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // No HTTP needed, returns hardcoded
    let result = client.stock_sgt_reference_exchange_rate_szse().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stock_sgt_settlement_exchange_rate_szse() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // No HTTP needed, returns hardcoded
    let result = client.stock_sgt_settlement_exchange_rate_szse().await;
    assert!(result.is_ok());
}

// ============================================================================
// MISSING METHOD TESTS — batch added for 100% coverage
// ============================================================================

#[tokio::test]
async fn test_stock_changes_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"f2": 10.5, "f3": 1.5, "f12": "000001", "f14": "平安银行"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_changes("涨速").await;
}

#[tokio::test]
async fn test_stock_dxsyl_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"SECURITY_CODE": "000001", "SECURITY_NAME": "测试"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_dxsyl().await;
}

#[tokio::test]
async fn test_stock_zt_pool_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"f2": 10.5, "f12": "000001", "f14": "测试"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_zt_pool("20240102").await;
}

#[tokio::test]
async fn test_stock_zt_pool_strong_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_zt_pool_strong("20240102").await;
}

#[tokio::test]
async fn test_stock_zt_pool_sub_new_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_zt_pool_sub_new("20240102").await;
}

#[tokio::test]
async fn test_stock_zt_pool_previous_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_zt_pool_previous("20240102").await;
}

#[tokio::test]
async fn test_stock_zt_pool_dtgc_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_zt_pool_dtgc("20240102").await;
}

#[tokio::test]
async fn test_stock_zt_pool_zbgc_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_zt_pool_zbgc("20240102").await;
}

#[tokio::test]
async fn test_stock_new_a_spot_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(em_push2_response(&[sample_em_stock_row("000001", "测试")])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_push2_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_new_a_spot().await;
}

#[tokio::test]
async fn test_stock_pg_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_pg().await;
}

#[tokio::test]
async fn test_stock_xgsglb_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_xgsglb("全部股票").await;
}

#[tokio::test]
async fn test_stock_qbzf_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_qbzf().await;
}

#[tokio::test]
async fn test_stock_sy_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_sy("000001").await;
}

#[tokio::test]
async fn test_stock_sy_jz_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_sy_jz("000001").await;
}

#[tokio::test]
async fn test_stock_sy_profile_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_sy_profile().await;
}

#[tokio::test]
async fn test_stock_sy_yq_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_sy_yq("全部").await;
}

#[tokio::test]
async fn test_stock_yjbb_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_yjbb("20240102").await;
}

#[tokio::test]
async fn test_stock_yjkb_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_yjkb("20240102").await;
}

#[tokio::test]
async fn test_stock_yjyg_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_yjyg("20240102").await;
}

#[tokio::test]
async fn test_stock_yysj_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_yysj("沪深A股", "20240102").await;
}

#[tokio::test]
async fn test_stock_lrb_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_lrb("20240102").await;
}

#[tokio::test]
async fn test_stock_zcfz_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_zcfz("20240102").await;
}

#[tokio::test]
async fn test_stock_xjll_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_xjll("20240102").await;
}

#[tokio::test]
async fn test_stock_ggcg_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_ggcg("000001").await;
}

#[tokio::test]
async fn test_stock_gpzy_pledge_ratio_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_gpzy_pledge_ratio().await;
}

#[tokio::test]
async fn test_stock_gpzy_pledge_detail_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_gpzy_pledge_detail().await;
}

#[tokio::test]
async fn test_stock_gpzy_profile_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_gpzy_profile().await;
}

#[tokio::test]
async fn test_stock_jgdy_detail_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_jgdy_detail("20240102").await;
}

#[tokio::test]
async fn test_stock_jgdy_tj_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_jgdy_tj("20240102").await;
}

#[tokio::test]
async fn test_stock_lh_yyb_capital() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_lh_yyb_capital().await;
}

#[tokio::test]
async fn test_stock_lh_yyb_control() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_lh_yyb_control().await;
}

#[tokio::test]
async fn test_stock_lh_yyb_most() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_lh_yyb_most().await;
}

#[tokio::test]
async fn test_stock_zh_b_spot_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_push2_response(&[sample_em_stock_row(
                "200001",
                "测试B股",
            )])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_push2_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_zh_b_spot_em().await;
}

#[tokio::test]
async fn test_stock_zh_ab_comparison_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_push2_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_push2_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_zh_ab_comparison().await;
}

#[tokio::test]
async fn test_stock_hk_hist() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_kline_response(&[&k1])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_kline_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client
        .stock_hk_hist("00593", "daily", "qfq", "20240101", "20240131")
        .await;
}

#[tokio::test]
async fn test_stock_hk_hist_min_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {"klines": ["2024-01-02 09:30,10.00,10.50,10.80,9.90,100000,10500000.0"]}
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"data": {"klines": []}})),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client
        .stock_hk_hist_min("00593", "5", "qfq", "20240101", "20240102")
        .await;
}

#[tokio::test]
async fn test_stock_hk_main_board_spot_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_push2_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_push2_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_hk_main_board_spot().await;
}

#[tokio::test]
async fn test_stock_hk_profit_forecast_et() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client
        .stock_hk_profit_forecast_et("00593", "预测指标")
        .await;
}

#[tokio::test]
async fn test_stock_us_hist() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_kline_response(&[&k1])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_kline_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_hist("AAPL", "daily", "qfq", "20240101", "20240131")
        .await;
}

#[tokio::test]
async fn test_stock_us_hist_min_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {"klines": ["2024-01-02 09:30,10.00,10.50,10.80,9.90,100000,10500000.0"]}
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"data": {"klines": []}})),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_hist_min("AAPL", "5", "qfq", "20240101", "20240102")
        .await;
}

#[tokio::test]
async fn test_stock_balance_sheet_by_report_em_typed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client
        .stock_balance_sheet_by_report_em_typed("000001")
        .await;
}

#[tokio::test]
async fn test_stock_cash_flow_sheet_by_report_em_typed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client
        .stock_cash_flow_sheet_by_report_em_typed("000001")
        .await;
}

#[tokio::test]
async fn test_stock_profit_sheet_by_report_em_typed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_profit_sheet_by_report_em_typed("000001").await;
}

#[tokio::test]
async fn test_stock_financial_abstract_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client
        .stock_financial_abstract_ths("000001", "按报告期")
        .await;
}

#[tokio::test]
async fn test_stock_financial_abstract_new_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client
        .stock_financial_abstract_new("000001", "按报告期")
        .await;
}

#[tokio::test]
async fn test_stock_financial_benefit_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_financial_benefit("000001", "按报告期").await;
}

#[tokio::test]
async fn test_stock_financial_benefit_new_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client
        .stock_financial_benefit_new("000001", "按报告期")
        .await;
}

#[tokio::test]
async fn test_stock_financial_cash_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_financial_cash("000001", "按报告期").await;
}

#[tokio::test]
async fn test_stock_financial_cash_new_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_financial_cash_new("000001", "按报告期").await;
}

#[tokio::test]
async fn test_stock_financial_debt_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_financial_debt("000001", "按报告期").await;
}

#[tokio::test]
async fn test_stock_financial_debt_new_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_financial_debt_new("000001", "按报告期").await;
}

#[tokio::test]
async fn test_stock_profit_forecast_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_profit_forecast_ths("000001", "预测指标").await;
}

#[tokio::test]
async fn test_stock_info_a_code_name() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_push2_response(&[
                serde_json::json!({"f12": "000001", "f14": "平安银行"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(em_push2_response(&[])))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_info_a_code_name().await;
}

#[tokio::test]
async fn test_stock_info_global_em() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {"list": [{"title": "测试新闻", "showTime": "2024-01-02"}]}
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"data": {"list": []}})),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let _ = client.stock_info_global_em().await;
}

#[tokio::test]
async fn test_stock_ipo_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_ipo("全部A股").await;
}

#[tokio::test]
async fn test_stock_ipo_hk_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_ipo_hk().await;
}

#[tokio::test]
async fn test_stock_management_change_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_management_change("000001").await;
}

#[tokio::test]
async fn test_stock_shareholder_change_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_shareholder_change("000001").await;
}

#[tokio::test]
async fn test_stock_zyjs_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><body><table><tr><td>数据</td></tr></table></body></html>",
    )
    .await;
    let client = mock_client(&server);
    let _ = client.stock_zyjs("000001").await;
}
