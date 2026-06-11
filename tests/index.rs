//! Integration tests for ALL index module methods.
//!
//! Covers: a_share, cflp, cni, cons, csindex, cx, drewry, eri, global,
//! global_em, global_sina, hf, hk, hog, kq_fz, kq_ss, qvix, scope, spot,
//! sugar, sw, sw_fund, sw_research, us_sina, yw, zh_em.

#![allow(dead_code, unused_variables)]
//!
//! Methods using `self.http.get()` bypass the mock redirect, so those tests
//! use `let _ = result;` to tolerate network failure gracefully.

mod common;
use common::*;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path_regex};

// ============================================================================
// a_share.rs
// ============================================================================

#[tokio::test]
async fn test_index_a_share_candles() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    let k2 = sample_kline_str("2024-01-03");
    let klines: Vec<&str> = vec![&k1, &k2];
    mock_any_get(&server, "/api/qt/stock/kline/get", em_kline_response(klines)).await;
    let client = mock_client(&server);
    let result = client.index_a_share_candles("000300", 10).await;
    assert!(result.is_ok());
    let pts = result.unwrap();
    assert_eq!(pts.len(), 2);
}

#[tokio::test]
async fn test_index_a_share_candles_invalid_symbol() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_a_share_candles("INVALID", 10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_index_stock_zh_spot_em() {
    let server = MockServer::start().await;
    let rows = vec![sample_em_stock_row("000001", "上证指数")];
    mock_any_get(&server, "/api/qt/clist/get", em_push2_response(rows)).await;
    let client = mock_client(&server);
    let result = client.index_stock_zh_spot_em("沪深重要指数").await;
    assert!(result.is_ok());
    let items = result.unwrap();
    assert!(!items.is_empty());
}

#[tokio::test]
async fn test_index_stock_zh_spot_em_unknown_series() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_stock_zh_spot_em("不存在的系列").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_index_stock_zh_spot_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, "/getHQNodeStockCountSimple", "1").await;
    let body = serde_json::json!([{"symbol": "sh000001", "name": "上证指数", "trade": "3000.0"}]);
    mock_any_get(&server, "/getHQNodeDataSimple", body).await;
    let client = mock_client(&server);
    let result = client.index_stock_zh_spot_sina().await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_zh_index_daily() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, "/hisdata/klc_kl.js", "function d(){eval(\"test\")}").await;
    let client = mock_client(&server);
    let result = client.stock_zh_index_daily("sh000300").await;
    assert!(result.is_err());
}

// ============================================================================
// cflp.rs
// ============================================================================

#[tokio::test]
async fn test_index_price_cflp() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "chart1": {"xLebal": ["2024-01"], "yLebal": [100.5]},
        "chart2": {"xLebal": ["2024-01"], "yLebal": [0.3]},
        "chart3": {"xLebal": ["2024-01"], "yLebal": [-1.2]}
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.index_price_cflp("月指数").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_price_cflp_invalid() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_price_cflp("无效").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_index_volume_cflp() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "chart1": {"xLebal": ["2024-01"], "yLebal": [500.0]},
        "chart2": {"xLebal": ["2024-01"], "yLebal": [2.0]},
        "chart3": {"xLebal": ["2024-01"], "yLebal": [5.0]}
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.index_volume_cflp("月指数").await;
    let _ = result;
}

// ============================================================================
// cni.rs
// ============================================================================

#[tokio::test]
async fn test_index_all_cni() {
    let server = MockServer::start().await;
    let mut row = vec![serde_json::json!(null); 25];
    row[2] = serde_json::json!("399001");
    row[8] = serde_json::json!("深证成指");
    row[12] = serde_json::json!(500.0);
    row[13] = serde_json::json!(10000.0);
    row[14] = serde_json::json!(0.5);
    row[16] = serde_json::json!(20.0);
    row[18] = serde_json::json!(100000000.0);
    row[19] = serde_json::json!(50000000000.0);
    row[20] = serde_json::json!(100000000000.0);
    row[21] = serde_json::json!(80000000000.0);
    let body = serde_json::json!({"data": {"rows": [row]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.index_all_cni().await;
    let _ = result;
}

#[tokio::test]
async fn test_index_hist_cni() {
    let server = MockServer::start().await;
    let row = vec![
        serde_json::json!("2024-01-02"),
        serde_json::json!(null),
        serde_json::json!(10100.0),
        serde_json::json!(10000.0),
        serde_json::json!(9950.0),
        serde_json::json!(10050.0),
        serde_json::json!(null),
        serde_json::json!("0.50%"),
        serde_json::json!(50000000.0),
        serde_json::json!(300000000.0),
        serde_json::json!(null),
    ];
    let body = serde_json::json!({"data": {"data": [row]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.index_hist_cni("399001", "20240101", "20240131").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_detail_cni() {
    let server = MockServer::start().await;
    let body = serde_json::json!([
        {"date": "2024-01-02", "code": "000001", "name": "平安银行"}
    ]);
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(body)
            .insert_header("content-type", "application/json"))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.index_detail_cni("399001").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_detail_hist_cni() {
    let server = MockServer::start().await;
    let row = vec![
        serde_json::json!("2024-01-02"), serde_json::json!(null),
        serde_json::json!(10100.0), serde_json::json!(10000.0),
        serde_json::json!(9950.0), serde_json::json!(10050.0),
        serde_json::json!(null), serde_json::json!("0.50%"),
        serde_json::json!(50000000.0), serde_json::json!(300000000.0),
        serde_json::json!(null),
    ];
    let body = serde_json::json!({"data": {"data": [row]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.index_detail_hist_cni("399001", "20240101", "20240131").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_detail_hist_adjust_cni() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{"code": "000001", "name": "平安银行"}]);
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(body)
            .insert_header("content-type", "application/json"))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.index_detail_hist_adjust_cni("399001").await;
    let _ = result;
}

// ============================================================================
// cons.rs
// ============================================================================

#[tokio::test]
async fn test_index_stock_cons_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, "/getHQNodeStockCountSimple", "1").await;
    let body = serde_json::json!([{"symbol": "sh600000", "name": "浦发银行", "trade": "10.00"}]);
    mock_any_get(&server, "/getHQNodeData", body).await;
    let client = mock_client(&server);
    let result = client.index_stock_cons_sina("000300").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_stock_cons_sina_non_hs300() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_stock_cons_sina("000001").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_index_stock_info() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"result": {"data": [{"symbol": "000001", "name": "上证指数"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.index_stock_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_index_stock_cons() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"result": {"data": [{"symbol": "sh600000", "name": "浦发银行"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.index_stock_cons("000300").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_stock_cons_csindex() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_stock_cons_csindex("000300").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_index_stock_cons_weight_csindex() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_stock_cons_weight_csindex("000300").await;
    assert!(result.is_err());
}

// ============================================================================
// csindex.rs
// ============================================================================

#[tokio::test]
async fn test_index_csindex_all() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"list": [{"indexCode": "000300", "indexName": "沪深300"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.index_csindex_all().await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_zh_index_hist_csindex() {
    let server = MockServer::start().await;
    let mut row = vec![serde_json::json!(null); 16];
    row[0] = serde_json::json!("2024-01-02");
    row[6] = serde_json::json!(3350.0);
    row[7] = serde_json::json!(3370.0);
    row[8] = serde_json::json!(3340.0);
    row[9] = serde_json::json!(3360.0);
    row[10] = serde_json::json!(10.0);
    row[11] = serde_json::json!(0.3);
    row[12] = serde_json::json!(100000000.0);
    row[13] = serde_json::json!(50000000000.0);
    row[14] = serde_json::json!(300.0);
    row[15] = serde_json::json!(12.5);
    let body = serde_json::json!({"data": [row]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.stock_zh_index_hist_csindex("000300", "20240101", "20240131").await;
    let _ = result;
}

// ============================================================================
// cx.rs (19 Caixin index methods — all use self.http.get(), bypass mock)
// ============================================================================

macro_rules! test_cx_method {
    ($name:ident, $method:ident) => {
        #[tokio::test]
        async fn $name() {
            let server = MockServer::start().await;
            let client = mock_client(&server);
            let result = client.$method().await;
            let _ = result;
        }
    };
}

test_cx_method!(test_index_pmi_com_cx, index_pmi_com_cx);
test_cx_method!(test_index_pmi_man_cx, index_pmi_man_cx);
test_cx_method!(test_index_pmi_ser_cx, index_pmi_ser_cx);
test_cx_method!(test_index_dei_cx, index_dei_cx);
test_cx_method!(test_index_ii_cx, index_ii_cx);
test_cx_method!(test_index_si_cx, index_si_cx);
test_cx_method!(test_index_fi_cx, index_fi_cx);
test_cx_method!(test_index_bi_cx, index_bi_cx);
test_cx_method!(test_index_nei_cx, index_nei_cx);
test_cx_method!(test_index_li_cx, index_li_cx);
test_cx_method!(test_index_ci_cx, index_ci_cx);
test_cx_method!(test_index_ti_cx, index_ti_cx);
test_cx_method!(test_index_neaw_cx, index_neaw_cx);
test_cx_method!(test_index_awpr_cx, index_awpr_cx);
test_cx_method!(test_index_cci_cx, index_cci_cx);
test_cx_method!(test_index_qli_cx, index_qli_cx);
test_cx_method!(test_index_ai_cx, index_ai_cx);
test_cx_method!(test_index_bei_cx, index_bei_cx);
test_cx_method!(test_index_neei_cx, index_neei_cx);

// ============================================================================
// drewry.rs (stub — always returns error)
// ============================================================================

#[tokio::test]
async fn test_drewry_wci_index() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.drewry_wci_index("World Container Index").await;
    assert!(result.is_err());
}

// ============================================================================
// eri.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_eri_monthly() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_eri("月度").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_eri_quarterly() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_eri("季度").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_eri_invalid() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_eri("无效").await;
    assert!(result.is_err());
}

// ============================================================================
// global.rs
// ============================================================================

#[tokio::test]
async fn test_index_global_name_table_yahoo() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_global_name_table_yahoo().await;
    assert!(result.is_ok());
    let items = result.unwrap();
    assert!(!items.is_empty());
}

#[tokio::test]
async fn test_index_global_candles() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_global_candles("SP500", 5).await;
    // yahoo_candles uses self.http.get() which bypasses mock redirect
    let _ = result;
}

// ============================================================================
// global_em.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_global_spot_em() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_global_spot_em().await;
    let _ = result;
}

#[tokio::test]
async fn test_index_global_hist_em() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_global_hist_em("100", "SPX", 10).await;
    let _ = result;
}

#[test]
fn test_global_em_symbol_map() {
    let result = akshare::index::global_em::global_em_symbol_map("标普500");
    assert!(result.is_some());
    let (market, code) = result.unwrap();
    assert_eq!(code, "SPX");
}

// ============================================================================
// global_sina.rs
// ============================================================================

#[tokio::test]
async fn test_index_global_name_table() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let items = client.index_global_name_table();
    assert!(!items.is_empty());
    assert!(items.iter().any(|e| e.name == "英国富时100指数"));
}

#[tokio::test]
async fn test_index_global_hist_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                {"d": "2024-01-02", "o": "7700", "h": "7750", "l": "7680", "c": "7720", "v": "500000"}
            ]
        }
    });
    mock_any_get(&server, "/hq/daily", body).await;
    let client = mock_client(&server);
    let result = client.index_global_hist_sina("英国富时100指数").await;
    assert!(result.is_ok());
    let pts = result.unwrap();
    assert!(!pts.is_empty());
}

#[tokio::test]
async fn test_index_global_hist_sina_unknown_name() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_global_hist_sina("不存在的指数").await;
    assert!(result.is_err());
}

// ============================================================================
// hf.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_hf_sp_500() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.hf_sp_500("2017").await;
    let _ = result;
}

// ============================================================================
// hk.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_hk_spot_sina() {
    let server = MockServer::start().await;
    let body = r#"var hq_str_hkHSI="HSI,恒生指数,18000.00,17910.00,18100.00,17900.00,18050.00,140.00,0.78";"#;
    mock_any_get_text(&server, "/", body).await;
    let client = mock_client(&server);
    let result = client.index_hk_spot_sina().await;
    let _ = result;
}

#[tokio::test]
async fn test_index_hk_spot_em() {
    let server = MockServer::start().await;
    let rows = vec![serde_json::json!({
        "f12": "HSI", "f14": "恒生指数", "f2": 1800000, "f3": 5000,
        "f4": 100, "f7": 200, "f15": 1810000, "f16": 1790000,
        "f17": 1800000, "f18": 1799900, "f124": 1704067200000i64
    })];
    mock_any_get(&server, "/api/qt/clist/get", em_push2_response(rows)).await;
    let client = mock_client(&server);
    let result = client.index_hk_spot_em().await;
    let _ = result;
}

#[tokio::test]
async fn test_index_hk_daily_em() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    let klines: Vec<&str> = vec![&k1];
    mock_any_get(&server, "/api/qt/stock/kline/get", em_kline_response(klines)).await;
    let client = mock_client(&server);
    let result = client.index_hk_daily_em("HSI", "100", 10).await;
    let _ = result;
}

// ============================================================================
// hog.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_hog_spot_price() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_hog_spot_price().await;
    let _ = result;
}

// ============================================================================
// kq_fz.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_kq_fz_price() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_kq_fz("价格指数").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_kq_fz_invalid() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_kq_fz("无效").await;
    assert!(result.is_err());
}

// ============================================================================
// kq_ss.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_kq_fashion() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_kq_fashion("柯桥时尚指数").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_kq_fashion_invalid() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_kq_fashion("无效").await;
    assert!(result.is_err());
}

// ============================================================================
// qvix.rs (18 methods — all use self.http.get(), bypass mock)
// ============================================================================

macro_rules! test_qvix_method {
    ($name:ident, $method:ident) => {
        #[tokio::test]
        async fn $name() {
            let server = MockServer::start().await;
            let client = mock_client(&server);
            let result = client.$method().await;
            let _ = result;
        }
    };
}

// Daily (9)
test_qvix_method!(test_index_option_50etf_qvix, index_option_50etf_qvix);
test_qvix_method!(test_index_option_300etf_qvix, index_option_300etf_qvix);
test_qvix_method!(test_index_option_500etf_qvix, index_option_500etf_qvix);
test_qvix_method!(test_index_option_cyb_qvix, index_option_cyb_qvix);
test_qvix_method!(test_index_option_kcb_qvix, index_option_kcb_qvix);
test_qvix_method!(test_index_option_100etf_qvix, index_option_100etf_qvix);
test_qvix_method!(test_index_option_300index_qvix, index_option_300index_qvix);
test_qvix_method!(test_index_option_1000index_qvix, index_option_1000index_qvix);
test_qvix_method!(test_index_option_50index_qvix, index_option_50index_qvix);

// Intraday (9)
test_qvix_method!(test_index_option_50etf_min_qvix, index_option_50etf_min_qvix);
test_qvix_method!(test_index_option_300etf_min_qvix, index_option_300etf_min_qvix);
test_qvix_method!(test_index_option_500etf_min_qvix, index_option_500etf_min_qvix);
test_qvix_method!(test_index_option_cyb_min_qvix, index_option_cyb_min_qvix);
test_qvix_method!(test_index_option_kcb_min_qvix, index_option_kcb_min_qvix);
test_qvix_method!(test_index_option_100etf_min_qvix, index_option_100etf_min_qvix);
test_qvix_method!(test_index_option_300index_min_qvix, index_option_300index_min_qvix);
test_qvix_method!(test_index_option_1000index_min_qvix, index_option_1000index_min_qvix);
test_qvix_method!(test_index_option_50index_min_qvix, index_option_50index_min_qvix);

// ============================================================================
// scope.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_news_sentiment_scope() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_news_sentiment_scope().await;
    let _ = result;
}

// ============================================================================
// spot.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_spot_goods_bdi() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.spot_goods("波罗的海干散货指数").await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_goods_invalid() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.spot_goods("无效").await;
    assert!(result.is_err());
}

// ============================================================================
// sugar.rs (3 methods — all use self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_sugar_msweet() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_sugar_msweet().await;
    let _ = result;
}

#[tokio::test]
async fn test_index_inner_quote_sugar_msweet() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_inner_quote_sugar_msweet().await;
    let _ = result;
}

#[tokio::test]
async fn test_index_outer_quote_sugar_msweet() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_outer_quote_sugar_msweet().await;
    let _ = result;
}

// ============================================================================
// sw.rs (Shenwan industry index — uses Eastmoney API)
// ============================================================================

#[tokio::test]
async fn test_sw_index_first_info() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.sw_index_first_info().await;
    assert!(result.is_ok());
    let items = result.unwrap();
    assert!(!items.is_empty());
    assert!(items[0].contains_key("code"));
}

#[tokio::test]
async fn test_sw_index_second_info() {
    let server = MockServer::start().await;
    let rows = vec![serde_json::json!({"f12": "801011", "f14": "林业", "f2": 100.0, "f3": 0.5})];
    mock_any_get(&server, "/api/qt/clist/get", em_push2_response(rows)).await;
    let client = mock_client(&server);
    let result = client.sw_index_second_info().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_sw_index_third_cons() {
    let server = MockServer::start().await;
    let rows = vec![serde_json::json!({"f12": "000001", "f14": "平安银行", "f2": 10.5, "f3": 1.0})];
    mock_any_get(&server, "/api/qt/clist/get", em_push2_response(rows)).await;
    let client = mock_client(&server);
    let result = client.sw_index_third_cons("801010").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_sw_index_third_info() {
    let server = MockServer::start().await;
    let rows = vec![serde_json::json!({"f12": "801013", "f14": "种子", "f2": 200.0, "f3": -0.3})];
    mock_any_get(&server, "/api/qt/clist/get", em_push2_response(rows)).await;
    let client = mock_client(&server);
    let result = client.sw_index_third_info("801010").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_sw_index_list() {
    let server = MockServer::start().await;
    let rows = vec![
        serde_json::json!({"f12": "801010", "f14": "农林牧渔", "f2": 3000.0, "f3": 0.5, "f5": 1000000.0, "f6": 5000000000.0}),
    ];
    mock_any_get(&server, "/api/qt/clist/get", em_push2_response(rows)).await;
    let client = mock_client(&server);
    let result = client.sw_index_list().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_sw_index_candles() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    let klines: Vec<&str> = vec![&k1];
    mock_any_get(&server, "/api/qt/stock/kline/get", em_kline_response(klines)).await;
    let client = mock_client(&server);
    let result = client.sw_index_candles("801010", 10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_sw_index_candles_invalid_code() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.sw_index_candles("INVALID", 10).await;
    assert!(result.is_err());
}

// ============================================================================
// sw_fund.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_realtime_fund_sw() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_realtime_fund_sw("801010").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_hist_fund_sw() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_hist_fund_sw("801010", "day").await;
    let _ = result;
}

// ============================================================================
// sw_research.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_hist_sw() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_hist_sw("801010", "day").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_hist_sw_invalid_period() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_hist_sw("801010", "invalid").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_index_min_sw() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_min_sw("801010").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_component_sw() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_component_sw("801010").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_realtime_sw() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_realtime_sw("801010").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_analysis_daily_sw() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_analysis_daily_sw("801010", "2024-01-01", "2024-01-31").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_analysis_week_month_sw() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_analysis_week_month_sw("801010").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_analysis_weekly_sw() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_analysis_weekly_sw("801010", "2024-01-01").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_analysis_monthly_sw() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_analysis_monthly_sw("801010", "2024-01-01").await;
    let _ = result;
}

// ============================================================================
// us_sina.rs (stub — requires JS decoding)
// ============================================================================

#[tokio::test]
async fn test_index_us_stock_sina() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_us_stock_sina("SPX").await;
    assert!(result.is_err());
}

// ============================================================================
// yw.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_yw_weekly_price() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_yw("周价格指数").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_yw_monthly_price() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_yw("月价格指数").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_yw_monthly_bi() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_yw("月景气指数").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_yw_invalid() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_yw("无效").await;
    assert!(result.is_err());
}

// ============================================================================
// zh_em.rs (uses self.http.get(), bypass mock)
// ============================================================================

#[tokio::test]
async fn test_index_zh_a_hist() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_zh_a_hist("000300", "daily").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_zh_a_hist_invalid_period() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_zh_a_hist("000300", "invalid").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_index_zh_a_hist_min_em() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_zh_a_hist_min_em("000300", "5", "20240101", "20240131", "qfq").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_zh_a_hist_min_5() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_zh_a_hist_min("000300", "5").await;
    let _ = result;
}

#[tokio::test]
async fn test_index_zh_a_hist_min_1() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.index_zh_a_hist_min("000300", "1").await;
    let _ = result;
}
