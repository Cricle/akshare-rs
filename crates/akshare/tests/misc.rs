//! Integration tests for ALL misc module methods.
//!
//! Covers: forex (boc, currency, em, fx, sina), crypto (bitcoin, js),
//! commodity (carbon, energy, spot), reits (em), economy (air, article,
//! fortune, movie, nlp, amac, event, car, other), news (cctv, search),
//! spot (`hog_soozhu`, `price_qh`, sge), bank (fjcf), tool (`trade_date`, pro),
//! cal (rv), market (detect, normalize, tencent, eastmoney).

mod common;
use common::*;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ============================================================================
// FOREX: boc.rs — Bank of China rates via Eastmoney datacenter
// ============================================================================

#[tokio::test]
async fn test_forex_boc_rates() {
    let server = MockServer::start().await;
    // forex_boc_rates() delegates to forex_sina_rates() which expects Sina text format
    let lines = [
        r#"var hq_str_fx_susdcny="美元兑人民币,7.0950,7.0980,7.0965,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#,
        r#"var hq_str_fx_seurcny="欧元兑人民币,7.8000,7.8100,7.8050,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#,
        r#"var hq_str_fx_sgbpcny="英镑兑人民币,9.1000,9.1100,9.1050,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#,
        r#"var hq_str_fx_sjpycny="日元兑人民币,0.0480,0.0485,0.0482,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#,
        r#"var hq_str_fx_shkdcny="港币兑人民币,0.9200,0.9250,0.9225,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#,
        r#"var hq_str_fx_saudcny="澳元兑人民币,4.6000,4.6100,4.6050,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#,
        r#"var hq_str_fx_scadcny="加元兑人民币,5.3000,5.3100,5.3050,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#,
        r#"var hq_str_fx_schfcny="瑞郎兑人民币,8.0000,8.0100,8.0050,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#,
        r#"var hq_str_fx_nzdcny="新西兰元兑人民币,4.3000,4.3100,4.3050,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#,
        r#"var hq_str_fx_ssgdcny="新加坡元兑人民币,5.2000,5.2100,5.2050,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#,
    ];
    let body = lines.join("\n");
    mock_any_get_text(&server, ".*", &body).await;
    let client = mock_client(&server);
    let result = client.forex_boc_rates().await;
    assert!(result.is_ok());
    let items = result.unwrap();
    assert_eq!(items.len(), 10);
    assert_eq!(items[0].currency_pair, "USD/CNY");
    assert!((items[0].buy_rate - 7.095).abs() < 0.001);
}

// ============================================================================
// FOREX: em.rs — Eastmoney forex rates
// ============================================================================

#[tokio::test]
async fn test_forex_em_rates() {
    let server = MockServer::start().await;
    let rows = vec![serde_json::json!({
        "f12": "USDCNY", "f14": "美元/人民币", "f2": 71100, "f3": 0.05
    })];
    mock_any_get(&server, ".*", em_push2_response(&rows)).await;
    let client = mock_client(&server);
    let result = client.forex_em_rates().await;
    let _ = result;
}

#[tokio::test]
async fn test_forex_spot_em() {
    let server = MockServer::start().await;
    let rows = vec![serde_json::json!({
        "f12": "USDCNY", "f14": "美元/人民币", "f2": 71100, "f3": 0.05
    })];
    mock_any_get(&server, ".*", em_push2_response(&rows)).await;
    let client = mock_client(&server);
    let result = client.forex_spot().await;
    let _ = result;
}

#[tokio::test]
async fn test_forex_hist_em() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    mock_any_get(&server, ".*", em_kline_response(&[&k1])).await;
    let client = mock_client(&server);
    let result = client
        .forex_hist("USDCNY", "day", "2024-01-01", "2024-01-31", "qfq")
        .await;
    let _ = result;
}

#[tokio::test]
async fn test_forex_em_hist() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    mock_any_get(&server, ".*", em_kline_response(&[&k1])).await;
    let client = mock_client(&server);
    let result = client.forex_em_hist("USDCNY", 10).await;
    let _ = result;
}

// ============================================================================
// FOREX: sina.rs — Sina forex rates
// ============================================================================

#[tokio::test]
async fn test_forex_sina_rates() {
    let server = MockServer::start().await;
    let body = r#"var hq_str_fx_susdcny="美元/人民币,7.10,7.12,7.11,2024-01-02,09:30";"#;
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.forex_sina_rates().await;
    let _ = result;
}

// ============================================================================
// FOREX: currency.rs — CurrencyBeacon/currencyscoop API
// ============================================================================

#[tokio::test]
async fn test_currency_latest() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "response": {
            "base": "USD",
            "rates": {"EUR": 0.92, "GBP": 0.79}
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.currency_latest("USD", "EUR,GBP", "test_key").await;
    let _ = result;
}

#[tokio::test]
async fn test_currency_history() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "response": {
            "base": "USD",
            "rates": {"EUR": 0.92}
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .currency_history("USD", "EUR", "2024-01-01", "test_key")
        .await;
    let _ = result;
}

#[tokio::test]
async fn test_currency_time_series() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "response": {
            "base": "USD",
            "rates": {"2024-01-01": {"EUR": 0.92}}
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .currency_time_series("USD", "EUR", "2024-01-01", "2024-01-02", "test_key")
        .await;
    let _ = result;
}

#[tokio::test]
async fn test_currency_currencies() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "response": [
            {"id": "USD", "name": "US Dollar"}
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.currency_currencies("fiat", "test_key").await;
    let _ = result;
}

#[tokio::test]
async fn test_currency_convert() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "response": {"value": 0.92}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .currency_convert("USD", "EUR", 100.0, "test_key")
        .await;
    let _ = result;
}

#[tokio::test]
async fn test_currency_boc_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                {"d": "2024-01-02", "o": "7.10", "h": "7.12", "l": "7.09", "c": "7.11", "v": "1000"}
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.currency_boc("美元", "20240101", "20240131").await;
    let _ = result;
}

#[tokio::test]
async fn test_currency_boc_safe() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                {"d": "2024-01-02", "o": "7.10", "h": "7.12", "l": "7.09", "c": "7.11", "v": "1000"}
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.currency_boc_safe().await;
    let _ = result;
}

// ============================================================================
// FOREX: fx.rs — CFETS FX data
// ============================================================================

#[tokio::test]
async fn test_currency_pair_map() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"CURRENCY_NAME": "USD/CNY", "CURRENCY_CODE": "USDCNY"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.currency_pair_map().await;
    let _ = result;
}

#[tokio::test]
async fn test_fx_c_swap_cm() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": [{"DATE": "2024-01-02", "CURRENCY_PAIR": "USD/CNY", "SWAP_RATE": 7.10}]}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fx_c_swap_cm().await;
    let _ = result;
}

#[tokio::test]
async fn test_fx_pair_quote() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": [{"DATE": "2024-01-02", "CLOSE": 7.10}]}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fx_pair_quote("USD/CNY").await;
    let _ = result;
}

#[tokio::test]
async fn test_fx_spot_quote() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": [{"DATE": "2024-01-02", "CLOSE": 7.10}]}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fx_spot_quote().await;
    let _ = result;
}

#[tokio::test]
async fn test_fx_swap_quote() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": [{"DATE": "2024-01-02", "CLOSE": 7.10}]}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fx_swap_quote().await;
    let _ = result;
}

#[tokio::test]
async fn test_fx_quote_baidu() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"result": [{"name": "USD/CNY", "price": "7.10"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fx_quote("美元兑人民币").await;
    let _ = result;
}

// ============================================================================
// CRYPTO: bitcoin.rs — Jin10 Bitcoin CME data
// ============================================================================

#[tokio::test]
async fn test_crypto_bitcoin_cme() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "keys": [{"name": "volume"}, {"name": "open_interest"}],
            "values": [["1000", "5000"]]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.crypto_bitcoin_cme("20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_crypto_bitcoin_hold_report() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "values": [["2024-01-02", "1000", "5000"]]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.crypto_bitcoin_hold_report().await;
    let _ = result;
}

// ============================================================================
// CRYPTO: js.rs — Jin10 crypto spot prices
// ============================================================================

#[tokio::test]
async fn test_crypto_js_spot() {
    let server = MockServer::start().await;
    let body = serde_json::json!([
        {"symbol": "BTC", "name": "Bitcoin", "price": "42000", "cny_price": "300000", "change": "1.5", "volume": "10000", "market_cap": "800000000"}
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.crypto_js_spot().await;
    let _ = result;
}

#[tokio::test]
async fn test_crypto_spot() {
    let server = MockServer::start().await;
    let body = serde_json::json!([
        {"symbol": "BTC", "name": "Bitcoin", "price": "42000", "cny_price": "300000", "change": "1.5", "volume": "10000", "market_cap": "800000000"}
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.crypto_spot().await;
    let _ = result;
}

// ============================================================================
// COMMODITY: carbon.rs — Carbon trading exchanges
// ============================================================================

#[tokio::test]
async fn test_energy_carbon_bj() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<table><tr><td>2024-01-02</td><td>100</td><td>50.0</td></tr></table>",
    )
    .await;
    let client = mock_client(&server);
    let result = client.energy_carbon_bj().await;
    let _ = result;
}

#[tokio::test]
async fn test_energy_carbon_sz() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    let client = mock_client(&server);
    let result = client.energy_carbon_sz().await;
    let _ = result;
}

#[tokio::test]
async fn test_energy_carbon_eu() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": [{"DATE": "2024-01-02", "CLOSE": 80.0}]}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.energy_carbon_eu().await;
    let _ = result;
}

#[tokio::test]
async fn test_energy_carbon_hb() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": [{"DATE": "2024-01-02", "CLOSE": 50.0}]}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.energy_carbon_hb().await;
    let _ = result;
}

#[tokio::test]
async fn test_energy_carbon_gz() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {"data": [{"DATE": "2024-01-02", "CLOSE": 45.0}]}
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.energy_carbon_gz().await;
    let _ = result;
}

// ============================================================================
// COMMODITY: energy.rs — Oil prices and domestic carbon
// ============================================================================

#[tokio::test]
async fn test_energy_oil_detail() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "CLOSE_PRICE": 75.0, "PRODUCT_NAME": "Brent"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.energy_oil_detail("20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_energy_oil_hist() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "CLOSE_PRICE": 75.0, "PRODUCT_NAME": "Brent"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.energy_oil_hist().await;
    let _ = result;
}

#[tokio::test]
async fn test_energy_carbon_domestic() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "CLOSE_PRICE": 50.0, "PRODUCT_NAME": "Carbon"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.energy_carbon_domestic().await;
    let _ = result;
}

// ============================================================================
// COMMODITY: spot.rs — Commodity spot prices from Eastmoney
// ============================================================================

#[tokio::test]
async fn test_commodity_spot_prices() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "CLOSE_PRICE": 580.5, "COMMODITY_NAME": "Au99.99"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.commodity_spot_prices(10).await;
    let _ = result;
}

#[tokio::test]
async fn test_commodity_spot_prices_zero() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.commodity_spot_prices(0).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

// ============================================================================
// REITS: em.rs — Eastmoney REITs
// ============================================================================

#[tokio::test]
async fn test_reits_list() {
    let server = MockServer::start().await;
    let rows = vec![serde_json::json!({
        "f12": "508000", "f14": "华安张江光大REIT", "f2": 35000, "f3": 0.5, "f5": 100_000
    })];
    mock_any_get(&server, ".*", em_push2_response(&rows)).await;
    let client = mock_client(&server);
    let result = client.reits_list(10).await;
    let _ = result;
}

#[tokio::test]
async fn test_reits_realtime_em() {
    let server = MockServer::start().await;
    let rows = vec![serde_json::json!({
        "f12": "508000", "f14": "华安张江光大REIT", "f2": 35000, "f3": 0.5, "f5": 100_000
    })];
    mock_any_get(&server, ".*", em_push2_response(&rows)).await;
    let client = mock_client(&server);
    let result = client.reits_realtime().await;
    let _ = result;
}

#[tokio::test]
async fn test_reits_hist_em() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    mock_any_get(&server, ".*", em_kline_response(&[&k1])).await;
    let client = mock_client(&server);
    let result = client
        .reits_hist_em("508000", "day", "2024-01-01", "2024-01-31", "qfq")
        .await;
    let _ = result;
}

#[tokio::test]
async fn test_reits_hist_min_em() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    mock_any_get(&server, ".*", em_kline_response(&[&k1])).await;
    let client = mock_client(&server);
    let result = client.reits_hist_min("508000", "5").await;
    let _ = result;
}

#[tokio::test]
async fn test_reits_hist() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    mock_any_get(&server, ".*", em_kline_response(&[&k1])).await;
    let client = mock_client(&server);
    let result = client.reits_hist("508000", 10).await;
    let _ = result;
}

// ============================================================================
// ECONOMY: air.rs — Air quality data
// ============================================================================

#[tokio::test]
async fn test_economy_air_quality() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 50.0, "CITY": "北京"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.economy_air_quality("北京").await;
    let _ = result;
}

#[tokio::test]
async fn test_air_quality_hebei() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 50.0}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.air_quality_hebei().await;
    let _ = result;
}

#[tokio::test]
async fn test_air_city_table() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"CITY": "北京", "AQI": 50})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.air_city_table().await;
    let _ = result;
}

#[tokio::test]
async fn test_air_quality_hist() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 50.0}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .air_quality_hist("北京", "20240101", "20240131")
        .await;
    let _ = result;
}

#[tokio::test]
async fn test_air_quality_rank() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({"CITY": "北京", "AQI": 50})]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.air_quality_rank().await;
    let _ = result;
}

#[tokio::test]
async fn test_sunrise_daily() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "Sunrise: 06:30\nSunset: 17:45").await;
    let client = mock_client(&server);
    let result = client.sunrise_daily("20240102", "Beijing").await;
    let _ = result;
}

#[tokio::test]
async fn test_sunrise_monthly() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "Sunrise: 06:30\nSunset: 17:45").await;
    let client = mock_client(&server);
    let result = client.sunrise_monthly("202401", "Beijing").await;
    let _ = result;
}

// ============================================================================
// ECONOMY: article.rs — Academic/research data
// ============================================================================

#[tokio::test]
async fn test_article_epu_index() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "Year,Month,Value\n2024,1,100.5\n2024,2,101.0\n",
    )
    .await;
    let client = mock_client(&server);
    let result = client.article_epu_index("China").await;
    let _ = result;
}

#[tokio::test]
async fn test_fred_md() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "DATE,VALUE1,VALUE2\n2023-01-01,100.5,200.3\n",
    )
    .await;
    let client = mock_client(&server);
    let result = client.fred_md("2023-03").await;
    let _ = result;
}

#[tokio::test]
async fn test_fred_qd() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "DATE,VALUE1,VALUE2\n2023-01-01,100.5,200.3\n",
    )
    .await;
    let client = mock_client(&server);
    let result = client.fred_qd("2023-03").await;
    let _ = result;
}

#[tokio::test]
async fn test_article_oman_rv() {
    let server = MockServer::start().await;
    let js_body = format!(
        "var data = {};",
        serde_json::json!({
            ".FTSE": {
                "dates": [1_704_067_200_000_i64],
                "rv5": {"data": [0.15]}
            }
        })
    );
    mock_any_get_text(&server, ".*", &js_body).await;
    let client = mock_client(&server);
    let result = client.article_oman_rv("FTSE", "rv5").await;
    let _ = result;
}

#[tokio::test]
async fn test_article_oman_rv_short() {
    let server = MockServer::start().await;
    let js_body = format!(
        "var data = {};",
        serde_json::json!({
            ".FTSE": {
                "data": [[1_704_067_200_000_i64, 0.15]]
            }
        })
    );
    mock_any_get_text(&server, ".*", &js_body).await;
    let client = mock_client(&server);
    let result = client.article_oman_rv_short("FTSE").await;
    let _ = result;
}

#[tokio::test]
async fn test_article_ff_crr() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "Date,Mkt-RF,SMB,HML\n202401,0.5,-0.2,0.1\n").await;
    let client = mock_client(&server);
    let result = client.article_ff_crr().await;
    let _ = result;
}

#[tokio::test]
async fn test_article_rlab_rv() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "20240102  0.123456\n20240103  0.134567\n").await;
    let client = mock_client(&server);
    let result = client.article_rlab_rv("39693").await;
    let _ = result;
}

// ============================================================================
// ECONOMY: fortune.rs — Bloomberg/Forbes/Hurun rankings
// ============================================================================

#[tokio::test]
async fn test_index_bloomberg_billionaires() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html><body>No data</body></html>").await;
    let client = mock_client(&server);
    let result = client.index_bloomberg_billionaires().await;
    let _ = result;
}

#[tokio::test]
async fn test_index_bloomberg_billionaires_hist() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html><body>No data</body></html>").await;
    let client = mock_client(&server);
    let result = client.index_bloomberg_billionaires_hist("Bill Gates").await;
    let _ = result;
}

#[tokio::test]
async fn test_forbes_rank() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html><body>No data</body></html>").await;
    let client = mock_client(&server);
    let result = client.forbes_rank("2024").await;
    let _ = result;
}

#[tokio::test]
async fn test_hurun_rank() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"rows": [{"name": "Test", "wealth": "100"}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.hurun_rank("2024", "2024").await;
    let _ = result;
}

#[tokio::test]
async fn test_xincaifu_rank() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"rows": [{"name": "Test", "wealth": "100"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.xincaifu_rank("2024").await;
    let _ = result;
}

// ============================================================================
// ECONOMY: movie.rs — Box office data
// ============================================================================

#[tokio::test]
async fn test_movie_boxoffice_realtime() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 5_000_000.0, "MOVIE_NAME": "Test"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.movie_boxoffice_realtime().await;
    let _ = result;
}

#[tokio::test]
async fn test_movie_boxoffice_daily() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 5_000_000.0, "MOVIE_NAME": "Test"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.movie_boxoffice_daily("20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_economy_box_office() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 5_000_000.0}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.economy_box_office().await;
    let _ = result;
}

// ============================================================================
// ECONOMY: nlp.rs — NLP sentiment
// ============================================================================

#[tokio::test]
async fn test_economy_sentiment_index() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 55.0}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.economy_sentiment_index().await;
    let _ = result;
}

#[tokio::test]
async fn test_nlp_ownthink() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"text": "test answer"}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.nlp_ownthink("test", "entity").await;
    let _ = result;
}

#[tokio::test]
async fn test_nlp_answer() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "test answer").await;
    let client = mock_client(&server);
    let result = client.nlp_answer("test").await;
    let _ = result;
}

// ============================================================================
// ECONOMY: amac.rs — AMAC fund/manager data
// ============================================================================

#[tokio::test]
async fn test_economy_amac_stats() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 100.0}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.economy_amac_stats().await;
    let _ = result;
}

// ============================================================================
// ECONOMY: event.rs — Baidu migration data
// ============================================================================

#[tokio::test]
async fn test_migration_area_baidu() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"date": "2024-01-02", "value": 100}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.migration_area("北京市", "move_in", "20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_migration_scale_baidu() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"date": "2024-01-02", "value": 100}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.migration_scale("北京市", "20240102").await;
    let _ = result;
}

// ============================================================================
// ECONOMY: car.rs — Auto market data
// ============================================================================

#[tokio::test]
async fn test_car_market_country_cpca() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 100_000.0}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.car_market_country_cpca("202401").await;
    let _ = result;
}

#[tokio::test]
async fn test_economy_auto_sales() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 100_000.0}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.economy_auto_sales().await;
    let _ = result;
}

// ============================================================================
// NEWS: cctv.rs — CCTV news
// ============================================================================

#[tokio::test]
async fn test_news_cctv() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": {"list": [{"title": "Test News", "url": "http://example.com"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.news_cctv("20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_news_economic_baidu() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": {"list": [{"title": "Test", "url": "http://example.com"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.news_economic("财经").await;
    let _ = result;
}

#[tokio::test]
async fn test_news_report_time_baidu() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": {"list": [{"title": "Test", "url": "http://example.com"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.news_report_time("600000").await;
    let _ = result;
}

#[tokio::test]
async fn test_news_trade_notify_dividend_baidu() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": {"list": [{"title": "Test", "url": "http://example.com"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.news_trade_notify_dividend("20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_news_trade_notify_suspend_baidu() {
    let server = MockServer::start().await;
    let body =
        serde_json::json!({"data": {"list": [{"title": "Test", "url": "http://example.com"}]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.news_trade_notify_suspend("20240102").await;
    let _ = result;
}

// ============================================================================
// NEWS: search.rs — Eastmoney news search
// ============================================================================

#[tokio::test]
async fn test_news_search() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"result": {"cmsArticleWebOld": [{"title": "Test", "content": "Test content", "date": "2024-01-02", "url": "http://example.com"}]}});
    mock_any_get_text(&server, ".*", &format!("jQuery123({body})")).await;
    let client = mock_client(&server);
    let result = client.news_search("test", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_news_search_empty_query() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.news_search("", 10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_news_search_zero_limit() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.news_search("test", 0).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

// ============================================================================
// SPOT: hog_soozhu.rs — Hog spot prices from Soozhu
// ============================================================================

#[tokio::test]
async fn test_spot_hog_soozhu() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    mock_any_post(
        &server,
        ".*",
        serde_json::json!({
            "vlist": [{"name": "Test", "value": [["2024-01-02", "15.0"]]}]
        }),
    )
    .await;
    let client = mock_client(&server);
    let result = client.spot_hog_soozhu().await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_hog_year_trend_soozhu() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    mock_any_post(
        &server,
        ".*",
        serde_json::json!({
            "vlist": [{"name": "Test", "value": [["2024-01-02", "15.0"]]}]
        }),
    )
    .await;
    let client = mock_client(&server);
    let result = client.spot_hog_year_trend_soozhu().await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_hog_lean_price_soozhu() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    mock_any_post(
        &server,
        ".*",
        serde_json::json!({
            "vlist": [{"name": "Test", "value": [["2024-01-02", "15.0"]]}]
        }),
    )
    .await;
    let client = mock_client(&server);
    let result = client.spot_hog_lean_price_soozhu().await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_corn_price_soozhu() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    mock_any_post(
        &server,
        ".*",
        serde_json::json!({
            "vlist": [{"name": "Test", "value": [["2024-01-02", "2800.0"]]}]
        }),
    )
    .await;
    let client = mock_client(&server);
    let result = client.spot_corn_price_soozhu().await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_soybean_price_soozhu() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    mock_any_post(
        &server,
        ".*",
        serde_json::json!({
            "vlist": [{"name": "Test", "value": [["2024-01-02", "3500.0"]]}]
        }),
    )
    .await;
    let client = mock_client(&server);
    let result = client.spot_soybean_price_soozhu().await;
    let _ = result;
}

// ============================================================================
// SPOT: price_qh.rs — 99 QH spot prices
// ============================================================================

#[tokio::test]
async fn test_spot_price_qh() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {"list": [{"date": "2024-01-02", "fp": "3800", "sp": "3850"}]}
    });
    mock_any_get(&server, ".*", body.clone()).await;
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.spot_price_qh("螺纹钢").await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_price_table_qh() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"name": "螺纹钢", "id": "1"}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.spot_price_table_qh().await;
    let _ = result;
}

// ============================================================================
// SPOT: sge.rs — Shanghai Gold Exchange
// ============================================================================

#[test]
fn test_spot_symbol_table_sge() {
    let symbols = akshare::spot::sge::spot_symbol_table_sge();
    assert!(!symbols.is_empty());
    assert!(symbols.contains(&"Au99.99"));
}

#[tokio::test]
async fn test_spot_quotations_sge() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "heyue": ["Au99.99"],
        "times": ["09:00"],
        "data": [[580.0]]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.spot_quotations_sge("Au99.99").await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_hist_sge() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "time": [["2024-01-02", 580.0]]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.spot_hist_sge("Au99.99").await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_golden_benchmark_sge() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "wp": [["2024-01-02", 580.0]],
        "zp": [["2024-01-02", 580.5]]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.spot_golden_benchmark_sge().await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_silver_benchmark_sge() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "wp": [["2024-01-02", 7500.0]],
        "zp": [["2024-01-02", 7510.0]]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.spot_silver_benchmark_sge().await;
    let _ = result;
}

// ============================================================================
// BANK: fjcf.rs — NFRA banking penalties
// ============================================================================

#[tokio::test]
async fn test_bank_fjcf_table_detail() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "total": 1,
            "rows": [{"PENALTY_DATE": "2024-01-02", "AMOUNT": 100_000.0}]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.bank_fjcf_table_detail("机关").await;
    let _ = result;
}

#[tokio::test]
async fn test_bank_fjcf_table_detail_invalid() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.bank_fjcf_table_detail("无效").await;
    assert!(result.is_err());
}

// ============================================================================
// TOOL: trade_date.rs — Sina trade calendar
// ============================================================================

#[tokio::test]
async fn test_tool_trade_date_hist_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        r#"var klc_td_sh="2024-01-02,2024-01-03,2024-01-04";"#,
    )
    .await;
    let client = mock_client(&server);
    let result = client.tool_trade_date_hist().await;
    let _ = result;
}

// ============================================================================
// TOOL: pro.rs — Tushare Pro API
// ============================================================================

#[tokio::test]
async fn test_set_token() {
    let server = MockServer::start().await;
    let mut client = mock_client(&server);
    client.set_token("test_token_123");
    // Verify token is set (no panic)
}

#[tokio::test]
async fn test_pro_api() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"token": "test_token_abc"});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.pro_api("user@example.com", "password").await;
    let _ = result;
}

// ============================================================================
// CAL: rv.rs — Realized volatility (Yang-Zhang estimator)
// ============================================================================

#[test]
fn test_volatility_yz_rv_insufficient_data() {
    let server = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { MockServer::start().await });
    let client = mock_client(&server);
    let bars = vec![akshare::cal::rv::OhlcBar {
        date: "2024-01-02".to_string(),
        open: 100.0,
        high: 105.0,
        low: 99.0,
        close: 103.0,
    }];
    let result = client.volatility_yz_rv(&bars);
    assert!(result.is_err());
}

#[test]
fn test_volatility_yz_rv_with_data() {
    let server = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { MockServer::start().await });
    let client = mock_client(&server);
    let bars = vec![
        akshare::cal::rv::OhlcBar {
            date: "2024-01-02".to_string(),
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 103.0,
        },
        akshare::cal::rv::OhlcBar {
            date: "2024-01-03".to_string(),
            open: 103.0,
            high: 107.0,
            low: 102.0,
            close: 106.0,
        },
        akshare::cal::rv::OhlcBar {
            date: "2024-01-04".to_string(),
            open: 106.0,
            high: 108.0,
            low: 104.0,
            close: 105.0,
        },
    ];
    let result = client.volatility_yz_rv(&bars);
    assert!(result.is_ok());
    let points = result.unwrap();
    assert!(!points.is_empty());
}

#[tokio::test]
async fn test_rv_from_futures_zh_minute_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    let client = mock_client(&server);
    let result = client.rv_from_futures_zh_minute("RB0").await;
    let _ = result;
}

#[tokio::test]
async fn test_rv_from_stock_zh_a_hist_min_em() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    mock_any_get(&server, ".*", em_kline_response(&[&k1])).await;
    let client = mock_client(&server);
    let result = client.rv_from_stock_zh_a_hist_min("600000").await;
    let _ = result;
}

// ============================================================================
// MARKET: market.rs — Sync utility functions (no HTTP)
// ============================================================================

#[test]
fn test_detect_market() {
    assert_eq!(
        akshare::market::detect_market("600000"),
        akshare::types::MarketKind::AShare
    );
    assert_eq!(
        akshare::market::detect_market("00593"),
        akshare::types::MarketKind::HongKong
    );
    assert_eq!(
        akshare::market::detect_market("AAPL"),
        akshare::types::MarketKind::UsEquity
    );
}

#[test]
fn test_normalize_a_share_symbol() {
    assert_eq!(
        akshare::market::normalize_a_share_symbol("600000"),
        Some("600000.SH".to_string())
    );
    assert_eq!(
        akshare::market::normalize_a_share_symbol("000001"),
        Some("000001.SZ".to_string())
    );
    assert_eq!(
        akshare::market::normalize_a_share_symbol("600000.SH"),
        Some("600000.SH".to_string())
    );
    assert_eq!(
        akshare::market::normalize_a_share_symbol("sh600000"),
        Some("600000.SH".to_string())
    );
    assert_eq!(akshare::market::normalize_a_share_symbol("AAPL"), None);
}

#[test]
fn test_normalize_hk_symbol() {
    assert_eq!(
        akshare::market::normalize_hk_symbol("00593"),
        Some("00593".to_string())
    );
    assert_eq!(
        akshare::market::normalize_hk_symbol("593"),
        Some("00593".to_string())
    );
    assert_eq!(
        akshare::market::normalize_hk_symbol("00593.HK"),
        Some("00593".to_string())
    );
}

#[test]
fn test_tencent_market_symbol() {
    let result = akshare::market::tencent_market_symbol("600000");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "sh600000");

    let result = akshare::market::tencent_market_symbol("000001");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "sz000001");
}

#[test]
fn test_tencent_market_symbol_invalid() {
    let result = akshare::market::tencent_market_symbol("AAPL");
    assert!(result.is_err());
}

#[test]
fn test_eastmoney_secid() {
    let result = akshare::market::eastmoney_secid("600000");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1.600000");

    let result = akshare::market::eastmoney_secid("000001");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.000001");
}

#[test]
fn test_eastmoney_secid_invalid() {
    let result = akshare::market::eastmoney_secid("AAPL");
    assert!(result.is_err());
}

// ============================================================================
// ECONOMY: movie.rs — Additional box office methods
// ============================================================================

#[tokio::test]
async fn test_movie_boxoffice_cinema_daily() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"TRADE_DATE": "2024-01-02T00:00:00", "BOX_OFFICE": 5_000_000.0, "MOVIE_NAME": "Test"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.movie_boxoffice_cinema_daily("20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_movie_boxoffice_cinema_weekly() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"TRADE_DATE": "2024-01-02T00:00:00", "BOX_OFFICE": 35_000_000.0, "MOVIE_NAME": "Test"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.movie_boxoffice_cinema_weekly("20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_movie_boxoffice_monthly() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"TRADE_DATE": "2024-01-02T00:00:00", "BOX_OFFICE": 150_000_000.0, "MOVIE_NAME": "Test"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.movie_boxoffice_monthly("202401").await;
    let _ = result;
}

#[tokio::test]
async fn test_movie_boxoffice_weekly() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"TRADE_DATE": "2024-01-02T00:00:00", "BOX_OFFICE": 35_000_000.0, "MOVIE_NAME": "Test"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.movie_boxoffice_weekly("20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_movie_boxoffice_yearly() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"TRADE_DATE": "2024-01-02T00:00:00", "BOX_OFFICE": 5_000_000_000.0, "MOVIE_NAME": "Test"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.movie_boxoffice_yearly("2024").await;
    let _ = result;
}

#[tokio::test]
async fn test_movie_boxoffice_yearly_first_week() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"TRADE_DATE": "2024-01-02T00:00:00", "BOX_OFFICE": 500_000_000.0, "MOVIE_NAME": "Test"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.movie_boxoffice_yearly_first_week("2024").await;
    let _ = result;
}

// ============================================================================
// ECONOMY: other.rs — Video methods
// ============================================================================

#[tokio::test]
async fn test_video_tv() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>TV ranking data</html>").await;
    let client = mock_client(&server);
    let result = client.video_tv().await;
    let _ = result;
}

#[tokio::test]
async fn test_video_variety_show() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>variety show ranking data</html>").await;
    let client = mock_client(&server);
    let result = client.video_variety_show().await;
    let _ = result;
}

// ============================================================================
// ECONOMY: other.rs — Game methods
// ============================================================================

#[tokio::test]
async fn test_game_hot_rank_taptap() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "success": true,
        "data": {
            "total": 1,
            "list": [
                {"app": {"title": "Test Game", "stat": {"rating": {"score": 8.5}}}}
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.game_hot_rank_taptap("热玩榜").await;
    let _ = result;
}

// ============================================================================
// ECONOMY: car.rs — Car market segment (CPCA)
// ============================================================================

#[tokio::test]
async fn test_car_market_segment_cpca() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>CPCA segment data</html>").await;
    let client = mock_client(&server);
    let result = client.car_market_segment_cpca("轿车", "零售").await;
    let _ = result;
}

// ============================================================================
// ECONOMY: other.rs — Car market methods (CPCA + Gasgoo)
// ============================================================================

#[tokio::test]
async fn test_car_market_total_cpca() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "month": ["2024-01", "2024-02"],
                "data_list": [
                    [[100.0, 200.0, 300.0, 400.0]],
                    [[110.0, 210.0, 310.0, 410.0]]
                ]
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.car_market_total_cpca("狭义乘用车", "零售").await;
    let _ = result;
}

#[tokio::test]
async fn test_car_market_man_rank_cpca() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "month": [],
                "data_list": [
                    ["比亚迪", 200_000.0],
                    ["一汽大众", 150_000.0]
                ]
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .car_market_man_rank_cpca("狭义乘用车-单月", "批发")
        .await;
    let _ = result;
}

#[tokio::test]
async fn test_car_market_cate_cpca() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "month": ["2024-01", "2024-02"],
                "data_list": [
                    [50000.0, 0.3],
                    [55000.0, 0.32]
                ]
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.car_market_cate_cpca("轿车", "批发").await;
    let _ = result;
}

#[tokio::test]
async fn test_car_market_fuel_cpca() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "month": ["2024-01", "2024-02"],
                "data_list": [
                    [100.0, 200.0, 50000.0],
                    [110.0, 210.0, 55000.0]
                ]
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.car_market_fuel_cpca("整体市场").await;
    let _ = result;
}

#[tokio::test]
async fn test_car_sale_rank_gasgoo() {
    let server = MockServer::start().await;
    let d_value = serde_json::json!([
        {"Name": "Model Y", "SalesVolume": 50000.0},
        {"Name": "Song PLUS", "SalesVolume": 45000.0}
    ]);
    let body = serde_json::json!({
        "d": d_value.to_string()
    });
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.car_sale_rank_gasgoo("车型榜", "202401").await;
    let _ = result;
}

// ============================================================================
// SPOT: hog_soozhu.rs — Additional Soozhu methods
// ============================================================================

#[tokio::test]
async fn test_spot_hog_crossbred_soozhu() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        r#"<input name="csrfmiddlewaretoken" value="tok123">"#,
    )
    .await;
    mock_any_post(
        &server,
        ".*",
        serde_json::json!({
            "datalist": [["2024-01-02", 3200.0], ["2024-01-03", 3250.0]]
        }),
    )
    .await;
    let client = mock_client(&server);
    let result = client.spot_hog_crossbred_soozhu().await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_hog_three_way_soozhu() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        r#"<input name="csrfmiddlewaretoken" value="tok123">"#,
    )
    .await;
    mock_any_post(
        &server,
        ".*",
        serde_json::json!({
            "datalist": [["2024-01-02", 450.0], ["2024-01-03", 460.0]]
        }),
    )
    .await;
    let client = mock_client(&server);
    let result = client.spot_hog_three_way_soozhu().await;
    let _ = result;
}

#[tokio::test]
async fn test_spot_mixed_feed_soozhu() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        r#"<input name="csrfmiddlewaretoken" value="tok123">"#,
    )
    .await;
    mock_any_post(
        &server,
        ".*",
        serde_json::json!({
            "datalist": [["2024-01-02", 3800.0], ["2024-01-03", 3820.0]]
        }),
    )
    .await;
    let client = mock_client(&server);
    let result = client.spot_mixed_feed_soozhu().await;
    let _ = result;
}

// ============================================================================
// ECONOMY: air.rs — Air quality watch point
// ============================================================================

#[tokio::test]
async fn test_air_quality_watch_point() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[
        serde_json::json!({"REPORT_DATE": "2024-01-02", "INDICATOR_VALUE": 50.0, "CITY": "北京"}),
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .air_quality_watch_point("北京", "20240101", "20240131")
        .await;
    let _ = result;
}

// ============================================================================
// ECONOMY: other.rs — Artist value methods
// ============================================================================

#[tokio::test]
async fn test_business_value_artist() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>business value artist data</html>").await;
    let client = mock_client(&server);
    let result = client.business_value_artist().await;
    let _ = result;
}

#[tokio::test]
async fn test_online_value_artist() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>online value artist data</html>").await;
    let client = mock_client(&server);
    let result = client.online_value_artist().await;
    let _ = result;
}

// ============================================================================
// ECONOMY: amac.rs — AMAC additional methods (no-arg)
// ============================================================================

#[tokio::test]
async fn test_amac_manager_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestManager"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestManager"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_manager_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_manager_classify_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestClassify"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestClassify"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_manager_classify_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_manager_cancelled_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestCancelled"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestCancelled"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_manager_cancelled_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_member_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestMember"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestMember"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_member_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_member_sub_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestMemberSub"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestMemberSub"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_member_sub_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_fund_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestFund"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestFund"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_fund_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_fund_sub_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestFundSub"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestFundSub"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_fund_sub_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_fund_abs() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestFundABS"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestFundABS"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_fund_abs().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_fund_account_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestFundAccount"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestFundAccount"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_fund_account_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_futures_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestFutures"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestFutures"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_futures_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_securities_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestSecurities"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestSecurities"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_securities_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_aoin_info() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestAOIN"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestAOIN"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_aoin_info().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_person_fund_org_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestPersonFundOrg"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestPersonFundOrg"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_person_fund_org_list().await;
    let _ = result;
}

#[tokio::test]
async fn test_amac_person_bond_org_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestPersonBondOrg"}),
            ])),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(em_datacenter_response(&[
                serde_json::json!({"NAME": "TestPersonBondOrg"}),
            ])),
        )
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.amac_person_bond_org_list().await;
    let _ = result;
}

// ============================================================================
// INDEX: zh_em.rs — index_zh_a_hist_min_em
// ============================================================================

#[tokio::test]
async fn test_index_zh_a_hist_min_em_misc() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    mock_any_get(&server, ".*", em_kline_response(&[&k1])).await;
    let client = mock_client(&server);
    let result = client
        .index_zh_a_hist_min_em("000300", "5", "20240101", "20240131", "qfq")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// INDEX: cni.rs — index_hist_cni
// ============================================================================

#[tokio::test]
async fn test_index_hist_cni_misc() {
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
        serde_json::json!(50_000_000.0),
        serde_json::json!(300_000_000.0),
        serde_json::json!(null),
    ];
    let body = serde_json::json!({"data": {"data": [row]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .index_hist_cni("399001", "20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// INDEX: cni.rs — index_detail_hist_cni
// ============================================================================

#[tokio::test]
async fn test_index_detail_hist_cni_misc() {
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
        serde_json::json!(50_000_000.0),
        serde_json::json!(300_000_000.0),
        serde_json::json!(null),
    ];
    let body = serde_json::json!({"data": {"data": [row]}});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .index_detail_hist_cni("399001", "20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// INDEX: sw_research.rs — index_analysis_daily_sw
// ============================================================================

#[tokio::test]
async fn test_index_analysis_daily_sw_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "results": [{
                "swindexcode": "801010",
                "swindexname": "农林牧渔",
                "bargaindate": "2024-01-02",
                "closeindex": 3000.0,
                "bargainamount": 100_000_000.0,
                "markup": 1.5,
                "turnoverrate": 2.0,
                "pe": 15.0,
                "pb": 1.5,
                "meanprice": 10.0,
                "bargainsumrate": 1.2,
                "negotiablessharesum1": 50_000_000_000.0,
                "negotiablessharesum2": 25_000_000_000.0,
                "dp": 2.5
            }]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .index_analysis_daily_sw("801010", "2024-01-01", "2024-01-31")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// INDEX: sw_research.rs — index_analysis_weekly_sw
// ============================================================================

#[tokio::test]
async fn test_index_analysis_weekly_sw_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "results": [{
                "swindexcode": "801010",
                "swindexname": "农林牧渔",
                "bargaindate": "2024-01-02",
                "closeindex": 3000.0,
                "bargainamount": 100_000_000.0,
                "markup": 1.5,
                "turnoverrate": 2.0,
                "pe": 15.0,
                "pb": 1.5,
                "meanprice": 10.0,
                "bargainsumrate": 1.2,
                "negotiablessharesum1": 50_000_000_000.0,
                "negotiablessharesum2": 25_000_000_000.0,
                "dp": 2.5
            }]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .index_analysis_weekly_sw("801010", "2024-01-01")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// INDEX: sw_research.rs — index_analysis_monthly_sw
// ============================================================================

#[tokio::test]
async fn test_index_analysis_monthly_sw_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "results": [{
                "swindexcode": "801010",
                "swindexname": "农林牧渔",
                "bargaindate": "2024-01-02",
                "closeindex": 3000.0,
                "bargainamount": 100_000_000.0,
                "markup": 1.5,
                "turnoverrate": 2.0,
                "pe": 15.0,
                "pb": 1.5,
                "meanprice": 10.0,
                "bargainsumrate": 1.2,
                "negotiablessharesum1": 50_000_000_000.0,
                "negotiablessharesum2": 25_000_000_000.0,
                "dp": 2.5
            }]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .index_analysis_monthly_sw("801010", "2024-01-01")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// INDEX: csindex.rs — stock_zh_index_hist_csindex
// ============================================================================

#[tokio::test]
async fn test_stock_zh_index_hist_csindex_misc() {
    let server = MockServer::start().await;
    let mut row = vec![serde_json::json!(null); 16];
    row[0] = serde_json::json!("2024-01-02");
    row[6] = serde_json::json!(3350.0);
    row[7] = serde_json::json!(3370.0);
    row[8] = serde_json::json!(3340.0);
    row[9] = serde_json::json!(3360.0);
    row[10] = serde_json::json!(10.0);
    row[11] = serde_json::json!(0.3);
    row[12] = serde_json::json!(100_000_000.0);
    row[13] = serde_json::json!(50_000_000_000.0);
    row[14] = serde_json::json!(300.0);
    row[15] = serde_json::json!(12.5);
    let body = serde_json::json!({"data": [row]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .stock_zh_index_hist_csindex("000300", "20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUND: etf_em.rs — fund_etf_hist_em
// ============================================================================

#[tokio::test]
async fn test_fund_etf_hist_em_misc() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    mock_any_get(&server, ".*", em_kline_response(&[&k1])).await;
    let client = mock_client(&server);
    let result = client
        .fund_etf_hist_em("159707", "daily", "20240101", "20240131", "qfq")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUND: etf_em.rs — fund_etf_hist_min
// ============================================================================

#[tokio::test]
async fn test_fund_etf_hist_min_misc() {
    let server = MockServer::start().await;
    let body = em_kline_response(&[
        "2024-01-02 09:35,10.00,10.50,10.80,9.90,100000,10500000.0,2.0,1.5,0.15,1.2",
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_etf_hist_min(
            "159707",
            "5",
            "2024-01-02 09:30:00",
            "2024-01-02 15:00:00",
            "qfq",
        )
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUND: etf_em.rs — fund_etf_fund_info
// ============================================================================

#[tokio::test]
async fn test_fund_etf_fund_info_misc() {
    let server = MockServer::start().await;
    let nav_item: Vec<serde_json::Value> = vec![
        serde_json::json!("2024-01-02"),
        serde_json::json!("1.0000"),
        serde_json::json!("2.0000"),
        serde_json::json!("0"),
        serde_json::json!("0"),
        serde_json::json!("0"),
        serde_json::json!("0.50"),
        serde_json::json!("开放申购"),
        serde_json::json!("开放赎回"),
        serde_json::json!(""),
    ];
    let body = serde_json::json!({
        "Data": {
            "LSJZList": [nav_item]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_etf_fund_info("511280", "20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUND: lof.rs — fund_lof_hist_em
// ============================================================================

#[tokio::test]
async fn test_fund_lof_hist_em_misc() {
    let server = MockServer::start().await;
    let k1 = sample_kline_str("2024-01-02");
    mock_any_get(&server, ".*", em_kline_response(&[&k1])).await;
    let client = mock_client(&server);
    let result = client
        .fund_lof_hist_em("160105", "daily", "20240101", "20240131", "qfq")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUND: lof.rs — fund_lof_hist_min
// ============================================================================

#[tokio::test]
async fn test_fund_lof_hist_min_misc() {
    let server = MockServer::start().await;
    let body = em_kline_response(&[
        "2024-01-02 09:35,10.00,10.50,10.80,9.90,100000,10500000.0,2.0,1.5,0.15,1.2",
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_lof_hist_min(
            "160105",
            "5",
            "2024-01-02 09:30:00",
            "2024-01-02 15:00:00",
            "qfq",
        )
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUND: open.rs — fund_open_fund_info
// ============================================================================

#[tokio::test]
async fn test_fund_open_fund_info_misc() {
    let server = MockServer::start().await;
    let nav_item: Vec<serde_json::Value> = vec![
        serde_json::json!("2024-01-02"),
        serde_json::json!("1.0000"),
        serde_json::json!("2.0000"),
        serde_json::json!("0"),
        serde_json::json!("0"),
        serde_json::json!("0"),
        serde_json::json!("0.50"),
        serde_json::json!("开放申购"),
        serde_json::json!("开放赎回"),
        serde_json::json!(""),
    ];
    let body = serde_json::json!({
        "Data": {
            "LSJZList": [nav_item]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_open_fund_info("710001", "", "", "单位净值走势")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUND: xueqiu.rs — fund_individual_detail_hold_xq
// ============================================================================

#[tokio::test]
async fn test_fund_individual_detail_hold_xq_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "chart_list": [
                {"type_desc": "股票", "percent": 80.5},
                {"type_desc": "债券", "percent": 15.0},
                {"type_desc": "现金", "percent": 4.5}
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_individual_detail_hold_xq("000001", "20240101")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUTURES: hist_em.rs — futures_hist
// ============================================================================

#[tokio::test]
async fn test_futures_hist_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "code": "rb2405",
            "klines": [
                "2024-01-02,3800,3850,3900,3750,200000,760000000,100,1.32,50,0.5"
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .futures_hist("rb2405", "daily", "2024-01-01", "2024-01-31")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUTURES: derivative.rs — futures_main_sina_derivative
// ============================================================================

#[tokio::test]
async fn test_futures_main_sina_derivative_misc() {
    let server = MockServer::start().await;
    let body = r#"var _rb02021_08_17=[[3800,3850,3900,3750,200000,"2024-01-02"]];"#;
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .futures_main_sina_derivative("rb0", "20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUTURES: cot.rs — futures_dce_position_rank_other
// ============================================================================

#[tokio::test]
async fn test_futures_dce_position_rank_other_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "contractId": "a2405",
                "abbr1": "TraderA",
                "qty1": "5000",
                "qty1_chg": "100",
                "abbr2": "TraderB",
                "qty2": "4800",
                "qty2_chg": "-50",
                "abbr3": "TraderC",
                "qty3": "4600",
                "qty3_chg": "80"
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .futures_dce_position_rank_other("20240315", "a")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUTURES: cot.rs — futures_hold_pos
// ============================================================================

#[tokio::test]
async fn test_futures_hold_pos_misc() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><table><tr><td>test data</td></tr></table></html>",
    )
    .await;
    let client = mock_client(&server);
    let result = client
        .futures_hold_pos("成交量", "rb2405", "20240315")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUTURES: basis.rs — futures_spot_price_daily
// ============================================================================

#[tokio::test]
async fn test_futures_spot_price_daily_misc() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client
        .futures_spot_price_daily("20240301", "20240315")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// FUTURES: exchange.rs — get_roll_yield_bar
// ============================================================================

#[tokio::test]
async fn test_get_roll_yield_bar_misc() {
    let server = MockServer::start().await;
    let shfe_body = serde_json::json!({"o_curinstrument": []});
    mock_any_get(&server, ".*", shfe_body).await;
    let dce_body = serde_json::json!({"data": []});
    mock_any_post(&server, ".*", dce_body).await;
    let client = mock_client(&server);
    let result = client
        .get_roll_yield_bar("20240315", None, None, None)
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// OPTION: commodity_sina.rs — option_commodity_contract_table
// ============================================================================

#[tokio::test]
async fn test_option_commodity_contract_table_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": {
                "up": [[100, 0.05, 0.06, 0.07, 50, 5000, 1.5, 3000.0, "m2405C3000"]],
                "down": [[80, 0.03, 0.04, 0.05, 40, 4000, -0.5, 3000.0, "m2405P3000"]]
            }
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .option_commodity_contract_table("\u{8c46}\u{7c95}\u{9009}\u{6743}", "m2405")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// OPTION: lhb_em.rs — option_lhb
// ============================================================================

#[tokio::test]
async fn test_option_lhb_misc() {
    let server = MockServer::start().await;
    let mut data = Vec::new();
    for _ in 0..28 {
        data.push(serde_json::json!([
            "期权交易",
            "2024-01-02",
            "510050",
            "50ETF",
            null,
            "某券商",
            1,
            1000.0,
            500.0,
            200.0,
            100.0,
            50.0,
            300.0,
            150.0,
            100.0,
            50.0,
            25.0,
            800.0,
            400.0,
            200.0,
            100.0,
            50.0,
            25.0,
            12.5,
            600.0,
            300.0,
            150.0,
            75.0
        ]));
    }
    let body = serde_json::json!({ "result": { "data": data } });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .option_lhb("510050", "\u{9009}\u{6743}\u{4ea4}\u{6613}\u{60c5}\u{51b5}-\u{8ba4}\u{6cbd}\u{4ea4}\u{6613}\u{91cf}", "20240102")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// OPTION: sse_sina.rs — option_sse_codes
// ============================================================================

#[tokio::test]
async fn test_option_sse_codes_misc() {
    let server = MockServer::start().await;
    let body = "var hq_str_OP_UP_51005003=\"CON_OP_10003720,CON_OP_10003721\"";
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .option_sse_codes("看涨期权", "202401", "510050")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// BOND: china_money.rs — bond_china_close_return
// ============================================================================

#[tokio::test]
async fn test_bond_china_close_return_misc() {
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

// ============================================================================
// BOND: issue_cninfo.rs — bond_local_government_issue
// ============================================================================

#[tokio::test]
async fn test_bond_local_government_issue_misc() {
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

// ============================================================================
// MACRO: china.rs — macro_china_nbs_nation
// ============================================================================

#[tokio::test]
async fn test_macro_china_nbs_nation_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "returndata": {
            "datanodes": [{
                "wds": [
                    {"wdcode": "zb", "valuecode": "A010101"},
                    {"wdcode": "reg", "valuecode": ""},
                    {"wdcode": "sj", "valuecode": "202401"}
                ],
                "data": {"data": 123.45}
            }]
        }
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .macro_china_nbs_nation("月度数据", "A010101", "202401")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// MACRO: china.rs — macro_china_nbs_region
// ============================================================================

#[tokio::test]
async fn test_macro_china_nbs_region_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "returndata": {
            "datanodes": [{
                "wds": [
                    {"wdcode": "zb", "valuecode": "A010101"},
                    {"wdcode": "reg", "valuecode": "110000"},
                    {"wdcode": "sj", "valuecode": "202401"}
                ],
                "data": {"data": 123.45}
            }]
        }
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .macro_china_nbs_region("分省月度数据", "A010101", "110000", "202401")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// MACRO: interest_rate.rs — rate_interbank
// ============================================================================

#[tokio::test]
async fn test_rate_interbank_misc() {
    let server = MockServer::start().await;
    let body = em_datacenter_response(&[serde_json::json!({
        "REPORT_DATE": "2024-01-02",
        "INDICATOR_VALUE": 1.50,
        "INDICATOR_NAME": "Shibor隔夜"
    })]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .rate_interbank("上海银行同业拆借市场", "Shibor人民币", "隔夜")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// NEWS: finnhub.rs — finnhub_company_news
// ============================================================================

#[tokio::test]
async fn test_finnhub_company_news_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!([
        {
            "category": "company",
            "datetime": 1_704_153_600,
            "headline": "Test News",
            "id": "12345",
            "image": "https://example.com/img.jpg",
            "related": "AAPL",
            "source": "TestSource",
            "summary": "Test summary.",
            "url": "https://example.com/news"
        }
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .finnhub_company_news("AAPL", "2024-01-01", "2024-01-31", "test_key")
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// NEWS: gdelt.rs — gdelt_news_search
// ============================================================================

#[tokio::test]
async fn test_gdelt_news_search_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "articles": [
            {
                "url": "https://example.com/news",
                "title": "Test Article",
                "seendate": "2024-01-02T00:00:00Z",
                "domain": "example.com",
                "language": "English",
                "sourcecountry": "US"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .gdelt_news_search(
            "rust",
            "https://api.gdeltproject.org/api/v2/doc/doc",
            Some("English"),
            None,
            10,
        )
        .await;
    assert!(result.is_ok());
}

// ============================================================================
// NEWS: gdelt.rs — gdelt_news_search_owned
// ============================================================================

#[tokio::test]
async fn test_gdelt_news_search_owned_misc() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "articles": [
            {
                "url": "https://example.com/news",
                "title": "Test Article",
                "seendate": "2024-01-02T00:00:00Z",
                "domain": "example.com",
                "language": "English",
                "sourcecountry": "US"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .gdelt_news_search_owned(
            "rust",
            "https://api.gdeltproject.org/api/v2/doc/doc",
            Some("English".to_string()),
            None,
            10,
        )
        .await;
    assert!(result.is_ok());
}
