mod common;
use common::*;
use wiremock::MockServer;

// ---------------------------------------------------------------------------
// convertible.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_cb_jsl() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "rows": [
            { "cell": { "bond_id": "123121", "bond_nm": "TestBond", "price": 105.5 } }
        ]
    });
    mock_any_get_text(&server, ".*", &body.to_string()).await;
    let client = mock_client(&server);
    let result = client.bond_cb_jsl().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_cov_comparison() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "diff": [
                { "f12": "123121", "f14": "TestBond", "f2": 105.5, "f3": 1.2 }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_cov_comparison(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_cov_issue_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "records": [
            { "SECURITY_CODE": "123121", "SECURITY_NAME_ABBR": "TestBond" }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_cov_issue("20240101", "20240131").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_zh_cov() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(vec![serde_json::json!({
        "SECURITY_CODE": "123121",
        "SECURITY_NAME_ABBR": "TestBond",
        "CURRENT_BOND_PRICE": 105.5,
        "TRANSFER_PREMIUM_RATIO": 15.0,
        "RATING": "AA"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_zh_cov(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_zh_cov_value_analysis() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                { "date": "2024-01-02", "value": 100.5 }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_zh_cov_value_analysis("123121").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_convertible_list() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "diff": [
                { "f12": "113050", "f14": "TestCB", "f2": 101.5, "f3": 0.5 }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_convertible_list(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_convertible_hist() {
    let server = MockServer::start().await;
    let body = em_kline_response(vec![
        "2024-01-02,100.50,101.20,102.00,99.80,50000,5050000.00,2.20,0.70,0.70,1.50",
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_convertible_hist("113050", 10).await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// government.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_china_yield() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                {
                    "SOLAR_DATE": "2024-01-02 00:00:00",
                    "EMG01446460": 2.10,
                    "EMG01446461": 2.30,
                    "EMG01446462": 2.45,
                    "EMG01446463": 2.55,
                    "EMG01446464": 2.70,
                    "EMG01446465": 2.80,
                    "EMG01446466": 3.00
                }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_china_yield("2024-01-01", "2024-01-31").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// corporate.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_corporate_yields() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                {
                    "SECURITY_CODE": "123456",
                    "SECURITY_NAME_ABBR": "TestCorpBond",
                    "ISSUE_DATE": "2024-01-02",
                    "ISSUE_PRICE": 100.0,
                    "COUPON_RATE": 3.5
                }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_corporate_yields(10).await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// spot.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_spot_deal() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "records": [
            { "bond_code": "010107", "bond_name": "21国债(7)", "price": "100.50" }
        ]
    });
    mock_any_get_text(&server, ".*", &body.to_string()).await;
    let client = mock_client(&server);
    let result = client.bond_spot_deal().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_spot_quote() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "records": [
            { "bond_code": "010107", "bond_name": "21国债(7)", "quote": "100.50" }
        ]
    });
    mock_any_get_text(&server, ".*", &body.to_string()).await;
    let client = mock_client(&server);
    let result = client.bond_spot_quote().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_spot_rates() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                {
                    "SOLAR_DATE": "2024-01-02 00:00:00",
                    "EMM00588704": 1.85,
                    "EMM00166462": 2.10,
                    "EMM00166466": 2.45,
                    "EMM00166469": 2.80
                }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_spot_rates(10).await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// cb_sina.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_cb_profile_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>mock</html>").await;
    let client = mock_client(&server);
    // This method returns an error indicating HTML parsing is required
    let result = client.bond_cb_profile("sz128039").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_bond_cb_summary_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>mock</html>").await;
    let client = mock_client(&server);
    // This method returns an error indicating HTML parsing is required
    let result = client.bond_cb_summary("sh155255").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// cb_ths.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_zh_cov_info_ths() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "list": [
            { "bond_code": "123121", "bond_name": "TestBond" }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_zh_cov_info_ths().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// china_money.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_china_close_return() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "records": [
            { "newDateValue": "2024-01-02", "yield": 2.80 }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .bond_china_close_return("国债", "1", "20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// gb_sina.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_gb_zh_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                ["2024-01-02", "2.80", "2.82", "2.78", "2.81", "1000"]
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_gb_zh("中国10年期国债").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_gb_us_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                ["2024-01-02", "4.50", "4.52", "4.48", "4.51", "1000"]
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_gb_us("美国10年期国债").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// issue_cninfo.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_corporate_issue_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "records": [
            { "SECURITY_CODE": "123456", "SECURITY_NAME_ABBR": "TestCorpBond" }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .bond_corporate_issue("20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_local_government_issue_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "records": [
            { "SECURITY_CODE": "123456", "SECURITY_NAME_ABBR": "TestLocalBond" }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .bond_local_government_issue("20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// jsl.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_cb_adj_logs_jsl() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "rows": [
            { "cell": { "bond_id": "123121", "adj_date": "2024-01-02" } }
        ]
    });
    mock_any_get_text(&server, ".*", &body.to_string()).await;
    let client = mock_client(&server);
    let result = client.bond_cb_adj_logs_jsl().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_cb_index_jsl() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "rows": [
            { "cell": { "index_date": "2024-01-02", "index_value": 100.5 } }
        ]
    });
    mock_any_get_text(&server, ".*", &body.to_string()).await;
    let client = mock_client(&server);
    let result = client.bond_cb_index_jsl().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_cb_redeem_jsl() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "rows": [
            { "cell": { "bond_id": "123121", "redeem_date": "2024-01-02" } }
        ]
    });
    mock_any_get_text(&server, ".*", &body.to_string()).await;
    let client = mock_client(&server);
    let result = client.bond_cb_redeem_jsl().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// cbond_index.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_composite_index_cbond() {
    let server = MockServer::start().await;
    // CBond returns a map keyed by "{ind_code}_{per_code}" with timestamp->value entries
    let ts: f64 = 1704153600000.0; // 2024-01-02 in millis
    let body = serde_json::json!({
        "CFZS_00": { ts.to_string(): 105.5 }
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_composite_index_cbond("财富", "总值").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_available_index_cbond() {
    let server = MockServer::start().await;
    let body = serde_json::json!([
        { "id": "8a8b2ca0332abed20134ea76d8885831", "name": "中债-总指数" }
    ]);
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_available_index_cbond().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// zh_cov.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_zh_hs_cov_daily() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "klines": [
                "2024-01-02,100.50,101.20,102.00,99.80,50000,5050000.00"
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_zh_hs_cov_daily("127018").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_zh_hs_cov_min() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "klines": [
                "2024-01-02 09:31,100.50,101.20,102.00,99.80,50000,5050000.00"
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_zh_hs_cov_min("127018", "5").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_zh_hs_cov_pre_min() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "trends": [
                "2024-01-02 09:25,100.50,101.20,102.00,99.80,50000"
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_zh_hs_cov_pre_min("127018").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_zh_hs_cov_spot() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "f43": 101.20,
            "f44": 102.00,
            "f45": 99.80,
            "f46": 100.50,
            "f47": 50000,
            "f48": 5050000.0,
            "f170": 0.70
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_zh_hs_cov_spot("127018").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// zh_sina.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_zh_hs_spot() {
    let server = MockServer::start().await;
    let body = serde_json::json!([
        {
            "symbol": "sh010107",
            "name": "21国债(7)",
            "trade": "100.50",
            "change": "0.30",
            "change_pct": "0.30",
            "buy": "100.40",
            "sell": "100.60",
            "settlement": "100.20",
            "open": "100.30",
            "high": "100.80",
            "low": "100.10",
            "volume": "5000",
            "amount": "502500"
        }
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_zh_hs_spot(1).await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// nafmii.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_debt_nafmii() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "rows": [
            { "regFileName": "TestBond", "entityName": "TestEntity" }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_debt_nafmii(1).await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// buyback.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_sh_buy_back_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "diff": [
                { "f12": "204001", "f14": "GC001", "f2": 2.50, "f3": 0.10 }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_sh_buy_back(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_buy_back_hist_em() {
    let server = MockServer::start().await;
    let body = em_kline_response(vec![
        "2024-01-02,2.50,2.55,2.60,2.45,100000,250000.0,2.0,0.50,0.01,1.0",
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_buy_back_hist("204001", 10).await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// summary_sse.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_cash_summary_sse() {
    let server = MockServer::start().await;
    // SSE returns Excel files; the method returns an error
    mock_any_get_text(&server, ".*", "binary data").await;
    let client = mock_client(&server);
    let result = client.bond_cash_summary("20240102").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_bond_deal_summary_sse() {
    let server = MockServer::start().await;
    // SSE returns Excel files; the method returns an error
    mock_any_get_text(&server, ".*", "binary data").await;
    let client = mock_client(&server);
    let result = client.bond_deal_summary("20240102").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// em_rate.rs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_zh_us_rate() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                {
                    "SOLAR_DATE": "2024-01-02 00:00:00",
                    "EMM00588704": 1.85,
                    "EMM00166462": 2.10,
                    "EMM00166466": 2.45,
                    "EMM00166469": 2.80,
                    "EMG00001306": 4.20,
                    "EMG00001308": 4.05,
                    "EMG00001310": 4.50,
                    "EMG00001312": 4.70
                }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_zh_us_rate("20240101").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// issue_cninfo.rs — additional methods
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_treasure_issue_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "records": [
            { "SECURITY_CODE": "123456", "SECURITY_NAME_ABBR": "TestTreasuryBond" }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .bond_treasure_issue("20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_local_gov_issue_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "records": [
            { "SECURITY_CODE": "123456", "SECURITY_NAME_ABBR": "TestLocalGovBond" }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .bond_local_gov_issue("20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_cov_stock_issue_cninfo() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "records": [
            { "SECURITY_CODE": "123456", "SECURITY_NAME_ABBR": "TestCovStock" }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_cov_stock_issue().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// cbond_index.rs — additional methods
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_index_general_cbond() {
    let server = MockServer::start().await;
    let ts: f64 = 1704153600000.0;
    let body = serde_json::json!({
        "CFZS_00": { ts.to_string(): 105.5 }
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_index_general_cbond("财富", "总值").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_treasury_index_cbond() {
    let server = MockServer::start().await;
    let ts: f64 = 1704153600000.0;
    let body = serde_json::json!({
        "CFZS_00": { ts.to_string(): 105.5 }
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_treasury_index_cbond("财富", "10Y").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_new_composite_index_cbond() {
    let server = MockServer::start().await;
    let ts: f64 = 1704153600000.0;
    let body = serde_json::json!({
        "CFZS_00": { ts.to_string(): 105.5 }
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_new_composite_index_cbond("财富", "总值").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// zh_cov.rs — bond_zh_cov_info
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_zh_cov_info() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                { "SECURITY_CODE": "123121", "SECURITY_NAME_ABBR": "TestBond" }
            ],
            "pages": 1
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_zh_cov_info("123121", "基本信息").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bond_zh_cov_info_invalid_indicator() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.bond_zh_cov_info("123121", "invalid_indicator").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// cbond_index.rs — bond_cbond_indicators / bond_cbond_periods (sync fns)
// ---------------------------------------------------------------------------

#[test]
fn test_bond_cbond_indicators() {
    let indicators = akshare::bond::cbond_index::bond_cbond_indicators();
    assert!(!indicators.is_empty());
    assert!(indicators.contains(&"财富"));
    assert!(indicators.contains(&"全价"));
    assert!(indicators.contains(&"净价"));
}

#[test]
fn test_bond_cbond_periods() {
    let periods = akshare::bond::cbond_index::bond_cbond_periods();
    assert!(!periods.is_empty());
    assert!(periods.contains(&"总值"));
    assert!(periods.contains(&"1-3年"));
}

// ---------------------------------------------------------------------------
// china_money.rs — bond_china_close_return_types (sync fn)
// ---------------------------------------------------------------------------

#[test]
fn test_bond_china_close_return_types() {
    let types = akshare::bond::china_money::bond_china_close_return_types();
    assert!(!types.is_empty());
    assert!(types.contains(&"国债"));
    assert!(types.contains(&"同业存单(AAA)"));
}

// ---------------------------------------------------------------------------
// gb_sina.rs — bond_gb_us_symbols / bond_gb_zh_symbols (sync fns)
// ---------------------------------------------------------------------------

#[test]
fn test_bond_gb_us_symbols() {
    let symbols = akshare::bond::gb_sina::bond_gb_us_symbols();
    assert!(!symbols.is_empty());
    assert!(symbols.contains(&"美国10年期国债"));
    assert!(symbols.contains(&"美国30年期国债"));
}

#[test]
fn test_bond_gb_zh_symbols() {
    let symbols = akshare::bond::gb_sina::bond_gb_zh_symbols();
    assert!(!symbols.is_empty());
    assert!(symbols.contains(&"中国10年期国债"));
    assert!(symbols.contains(&"中国30年期国债"));
}

// ---------------------------------------------------------------------------
// buyback.rs — bond_sz_buy_back
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_sz_buy_back_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "diff": [
                { "f12": "131810", "f14": "RC-001", "f2": 2.50, "f3": 0.10 }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bond_sz_buy_back(10).await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// zh_sina.rs — bond_zh_hs_daily
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_bond_zh_hs_daily() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    let client = mock_client(&server);
    // bond_zh_hs_daily returns Err because Sina bond daily requires JS decryption
    let result = client.bond_zh_hs_daily("sh010107").await;
    assert!(result.is_err());
}
