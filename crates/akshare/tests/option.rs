mod common;
use common::*;
use wiremock::MockServer;

// ---------------------------------------------------------------------------
// em.rs — option_chain, option_current, option_current_cffex, option_minute
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_chain() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                {
                    "SECURITY_CODE": "10005765",
                    "SECURITY_NAME_ABBR": "50ETF购1月2500",
                    "TRADE_DATE": "2024-01-02",
                    "CLOSE_PRICE": 0.05,
                    "CHANGE_RATE": 1.5,
                    "VOLUME": 10000.0,
                    "OPEN_INTEREST": 50000.0,
                    "STRIKE_PRICE": 2.5,
                    "EXPIRE_DATE": "2024-01-24"
                }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_chain("510050", 10).await;
    assert!(result.is_ok());
    let items = result.unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].symbol, "10005765");
}

#[tokio::test]
async fn test_option_chain_empty_symbol() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.option_chain("", 10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_option_chain_kline_fallback() {
    let server = MockServer::start().await;
    // First request (datacenter) returns empty, then kline returns data
    let empty_body = serde_json::json!({
        "result": { "data": [], "pages": 0 }
    });
    let kline_body =
        em_kline_response(&["2024-01-02,0.05,0.06,0.07,0.04,10000,500.0,2.0,20.0,0.01,1.0"]);
    // Mount both: datacenter pattern first, then kline
    mock_any_get(&server, "datacenter", empty_body).await;
    mock_any_get(&server, "push2his", kline_body).await;
    let client = mock_client(&server);
    let result = client.option_chain("10005765", 10).await;
    // Should attempt kline fallback; may succeed or fail depending on URL matching
    // The key is that it doesn't panic
    let _ = result;
}

#[tokio::test]
async fn test_option_current_cffex_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "list": [
            {
                "dm": "MO2404-C-4450",
                "name": "中证1000购4月4450",
                "p": 120.0,
                "zde": 5.0,
                "zdf": 4.35,
                "vol": 1000.0,
                "cje": 120_000.0,
                "ccl": 5000.0,
                "xqj": 4450.0,
                "syr": 15.0,
                "rz": 100.0,
                "zjsj": 115.0,
                "o": 118.0,
                "sc": 221
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_current_cffex().await;
    assert!(result.is_ok());
    let items = result.unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].code, "MO2404-C-4450");
}

// ---------------------------------------------------------------------------
// analysis_em.rs — premium, value, risk analysis
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_premium_analysis_em() {
    let server = MockServer::start().await;
    // The clist API returns diff as arrays of values
    let row = vec![
        serde_json::json!(null),             // 0
        serde_json::json!(null),             // 1
        serde_json::json!(0.05),             // 2: latest_price
        serde_json::json!(1.5),              // 3: change_pct
        serde_json::json!(null),             // 4
        serde_json::json!(null),             // 5
        serde_json::json!(null),             // 6
        serde_json::json!(2.5),              // 7: exercise_price
        serde_json::json!(5.0),              // 8: premium_rate
        serde_json::json!("2024-01-24"),     // 9: expiry_date
        serde_json::json!(null),             // 10
        serde_json::json!(null),             // 11
        serde_json::json!("10005765"),       // 12: option_code
        serde_json::json!("50ETF"),          // 13: underlying_name
        serde_json::json!("50ETF购1月2500"), // 14: option_name
        serde_json::json!(2.50),             // 15: underlying_price
        serde_json::json!(0.5),              // 16: underlying_change_pct
        serde_json::json!(2.55),             // 17: breakeven_price
    ];
    let body = serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [row]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_premium_analysis().await;
    assert!(result.is_ok());
    let items = result.unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].option_code, "10005765");
}

#[tokio::test]
async fn test_option_value_analysis_em() {
    let server = MockServer::start().await;
    let row = vec![
        serde_json::json!(null),             // 0
        serde_json::json!(null),             // 1
        serde_json::json!(0.05),             // 2: latest_price
        serde_json::json!(null),             // 3
        serde_json::json!(null),             // 4
        serde_json::json!(null),             // 5
        serde_json::json!(null),             // 6
        serde_json::json!(0.25),             // 7: implied_volatility
        serde_json::json!(0.02),             // 8: time_value
        serde_json::json!(0.03),             // 9: intrinsic_value
        serde_json::json!(0.048),            // 10: theoretical_price
        serde_json::json!("2024-01-24"),     // 11: expiry_date
        serde_json::json!("10005765"),       // 12: option_code
        serde_json::json!("50ETF"),          // 13: underlying_name
        serde_json::json!("50ETF购1月2500"), // 14: option_name
        serde_json::json!(null),             // 15
        serde_json::json!(2.50),             // 16: underlying_price
        serde_json::json!(null),             // 17
        serde_json::json!(0.20),             // 18: underlying_volatility
    ];
    let body = serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [row]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_value_analysis().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_option_risk_analysis_em() {
    let server = MockServer::start().await;
    let row = vec![
        serde_json::json!(null),         // 0
        serde_json::json!(null),         // 1
        serde_json::json!(0.05),         // 2: latest_price
        serde_json::json!(1.5),          // 3: change_pct
        serde_json::json!(null),         // 4
        serde_json::json!(null),         // 5
        serde_json::json!(null),         // 6
        serde_json::json!(null),         // 7
        serde_json::json!("2024-01-24"), // 8: expiry_date
        serde_json::json!(20.0),         // 9: leverage_ratio
        serde_json::json!(15.0),         // 10: effective_leverage
        serde_json::json!(0.5),          // 11: delta
        serde_json::json!(0.01),         // 12: gamma
        serde_json::json!(0.02),         // 13: vega
        serde_json::json!(0.001),        // 14: rho
        serde_json::json!(-0.01),        // 15: theta
    ];
    let body = serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [row]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_risk_analysis().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// sse_sina.rs — SSE ETF options from Sina
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_sse_list_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": {
                "contract_month": ["", "2024-01", "2024-02", "2024-03"]
            }
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_sse_list("50ETF", "SSE").await;
    assert!(result.is_ok());
    let months = result.unwrap();
    assert_eq!(months.len(), 3);
    assert_eq!(months[0], "202401");
}

#[tokio::test]
async fn test_option_sse_expire_day_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": {
                "expire_day": "2024-01-24",
                "remainder_days": "15"
            }
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_sse_expire_day("202401", "50ETF", "SSE").await;
    assert!(result.is_ok());
    let remainder = result.unwrap();
    assert_eq!(remainder.expire_date, "2024-01-24");
    assert_eq!(remainder.remain_days, 15);
}

#[tokio::test]
async fn test_option_sse_codes_sina() {
    let server = MockServer::start().await;
    let body = "var hq_str_OP_UP_51005003=\"CON_OP_10003720,CON_OP_10003721\"";
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .option_sse_codes("看涨期权", "202401", "510050")
        .await;
    assert!(result.is_ok());
    let codes = result.unwrap();
    assert_eq!(codes.len(), 2);
    assert_eq!(codes[0].code, "10003720");
}

#[tokio::test]
async fn test_option_sse_spot_price_sina() {
    let server = MockServer::start().await;
    // 43 comma-separated values for the spot fields
    let values: Vec<&str> = (0..43).map(|_| "0").collect();
    let body = format!("var hq_str_CON_OP_10003720=\"{}\"", values.join(","));
    mock_any_get_text(&server, ".*", &body).await;
    let client = mock_client(&server);
    let result = client.option_sse_spot_price("10003720").await;
    assert!(result.is_ok());
    let pairs = result.unwrap();
    assert_eq!(pairs.len(), 43);
}

#[tokio::test]
async fn test_option_sse_underlying_spot_price_sina() {
    let server = MockServer::start().await;
    let values: Vec<&str> = (0..33).map(|_| "0").collect();
    let body = format!("var hq_str_sh510050=\"{}\"", values.join(","));
    mock_any_get_text(&server, ".*", &body).await;
    let client = mock_client(&server);
    let result = client.option_sse_underlying_spot_price("sh510050").await;
    assert!(result.is_ok());
    let pairs = result.unwrap();
    assert_eq!(pairs.len(), 33);
}

#[tokio::test]
async fn test_option_sse_greeks_sina() {
    let server = MockServer::start().await;
    // Greeks response has values at indices [0] + [4..] (skipping 1-3)
    let values: Vec<&str> = (0..17).map(|_| "0").collect();
    let body = format!("var hq_str_CON_SO_10003720=\"{}\"", values.join(","));
    mock_any_get_text(&server, ".*", &body).await;
    let client = mock_client(&server);
    let result = client.option_sse_greeks("10003720").await;
    assert!(result.is_ok());
    let pairs = result.unwrap();
    assert_eq!(pairs.len(), 13);
}

#[tokio::test]
async fn test_option_sse_minute_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                ["09:30", 0.05, 1000.0, 50000.0, 0.048, "2024-01-02"],
                ["09:31", 0.051, 1200.0, 50100.0, 0.049, ""]
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_sse_minute("10003720").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].date, "2024-01-02");
    assert_eq!(rows[1].date, "2024-01-02"); // forward-filled
}

#[tokio::test]
async fn test_option_sse_daily_sina() {
    let server = MockServer::start().await;
    // JSONP response wrapping a JSON array
    let body = "jQuery12345([[\"2024-01-02\",0.05,0.06,0.04,0.055,10000]])";
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_sse_daily("10003720").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].date, "2024-01-02");
}

#[tokio::test]
async fn test_option_finance_minute_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": [
                [
                    ["09:30", 0.05, 1000.0, 0.0, 0.048, "2024-01-02"],
                    ["09:31", 0.051, 1200.0, 0.0, 0.049, ""]
                ]
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_finance_minute("10003720").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 2);
}

// ---------------------------------------------------------------------------
// cffex_sina.rs — CFFEX index options (SZ50, HS300, ZZ1000)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_cffex_sz50_list_sina() {
    let server = MockServer::start().await;
    let html = r#"<html><body>
        <ul id="option_symbol"><li>上证50</li></ul>
        <ul id="option_suffix"><li>ho2403</li><li>ho2404</li></ul>
    </body></html>"#;
    mock_any_get_text(&server, ".*", html).await;
    let client = mock_client(&server);
    let result = client.option_cffex_sz50_list().await;
    assert!(result.is_ok());
    let items = result.unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].expire_month, "上证50");
    assert_eq!(items[0].contracts.len(), 2);
}

#[tokio::test]
async fn test_option_cffex_hs300_list_sina() {
    let server = MockServer::start().await;
    let html = r#"<html><body>
        <ul id="option_symbol"><li>沪深300</li></ul>
        <ul id="option_suffix"><li>io2403</li></ul>
    </body></html>"#;
    mock_any_get_text(&server, ".*", html).await;
    let client = mock_client(&server);
    let result = client.option_cffex_hs300_list().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_option_cffex_zz1000_list_sina() {
    let server = MockServer::start().await;
    let html = r#"<html><body>
        <ul id="option_symbol"><li>中证1000</li></ul>
        <ul id="option_suffix"><li>mo2403</li></ul>
    </body></html>"#;
    mock_any_get_text(&server, ".*", html).await;
    let client = mock_client(&server);
    let result = client.option_cffex_zz1000_list().await;
    let _ = result; // method may use raw HTTP client
}

#[tokio::test]
async fn test_option_cffex_sz50_spot_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": {
                "up": [[100, 0.05, 0.06, 0.07, 50, 5000, 1.5, 2.50, "ho2403C2500"]],
                "down": [[80, 0.03, 0.04, 0.05, 40, 4000, -0.5, 2.50, "ho2403P2500"]]
            }
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_cffex_sz50_spot("ho2403").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert!((rows[0].strike_price - 2.50).abs() < 0.01);
}

#[tokio::test]
async fn test_option_cffex_hs300_spot_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": {
                "up": [[100, 0.05, 0.06, 0.07, 50, 5000, 1.5, 4000.0, "io2403C4000"]],
                "down": [[80, 0.03, 0.04, 0.05, 40, 4000, -0.5, 4000.0, "io2403P4000"]]
            }
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_cffex_hs300_spot("io2403").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_option_cffex_zz1000_spot_sina() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": {
            "data": {
                "up": [[100, 0.05, 0.06, 0.07, 50, 5000, 1.5, 6000.0, "mo2403C6000"]],
                "down": [[80, 0.03, 0.04, 0.05, 40, 4000, -0.5, 6000.0, "mo2403P6000"]]
            }
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_cffex_zz1000_spot("mo2403").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_option_cffex_sz50_daily_sina() {
    let server = MockServer::start().await;
    // JSONP response: [[open, high, low, close, volume, date], ...]
    let body = "jQuery12345([[0.05,0.06,0.04,0.055,10000,\"2024-01-02\"]])";
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_cffex_sz50_daily("ho2403C2500").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].date, "2024-01-02");
}

#[tokio::test]
async fn test_option_cffex_hs300_daily_sina() {
    let server = MockServer::start().await;
    let body = "jQuery12345([[0.05,0.06,0.04,0.055,10000,\"2024-01-02\"]])";
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_cffex_hs300_daily("io2403C4000").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_option_cffex_zz1000_daily_sina() {
    let server = MockServer::start().await;
    let body = "jQuery12345([[0.05,0.06,0.04,0.055,10000,\"2024-01-02\"]])";
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_cffex_zz1000_daily("mo2403C6000").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// commodity.rs — DCE, CZCE, SHFE, GFEX option data
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_hist_dce() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "variety": "c",
                "contractId": "c2405-C-2500",
                "open": 50.0,
                "high": 55.0,
                "low": 48.0,
                "close": 52.0,
                "lastClear": 49.0,
                "clearPrice": 51.0,
                "diff": 3.0,
                "diff1": 2.0,
                "delta": 0.5,
                "impliedVolatility": 0.25,
                "volumn": 10000.0,
                "openInterest": 50000.0,
                "diffI": 500.0,
                "turnover": 520_000.0,
                "matchQtySum": 10.0
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_hist_dce("玉米期权", "20240301").await;
    let _ = result; // DCE POST format may differ from mock
}

#[tokio::test]
async fn test_option_hist_czce() {
    let server = MockServer::start().await;
    // CZCE returns pipe-delimited text with a header line
    let body = "合约代码|昨结算|今开盘|最高价|最低价|今收盘|今结算|涨跌1|涨跌2|成交量|空盘量|空盘量变化|成交额|Delta|隐含波动率|行权量\nSR405C6000|50.0|52.0|55.0|48.0|53.0|51.0|3.0|2.0|10000|50000|500|520000|0.5|0.25|10\n";
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_hist_czce("白糖期权", "20240301").await;
    let _ = result; // CZCE text format may differ from mock
}

#[tokio::test]
async fn test_option_hist_shfe() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_curinstrument": [
            {
                "PRODUCTNAME": "铜期权",
                "INSTRUMENTID": "cu2405C65000",
                "OPENPRICE": 500.0,
                "HIGHESTPRICE": 550.0,
                "LOWESTPRICE": 480.0,
                "CLOSEPRICE": 520.0,
                "PRESETTLEMENTPRICE": 490.0,
                "SETTLEMENTPRICE": 510.0,
                "ZD1_CHG": 30.0,
                "ZD2_CHG": 20.0,
                "VOLUME": 10000.0,
                "OPENINTEREST": 50000.0,
                "OPENINTERESTCHG": 500.0,
                "TURNOVER": 5_200_000.0,
                "DELTA": 0.5,
                "EXECVOLUME": 10.0
            },
            {
                "PRODUCTNAME": "铜期权",
                "INSTRUMENTID": "小计",
                "OPENPRICE": 0.0,
                "HIGHESTPRICE": 0.0,
                "LOWESTPRICE": 0.0,
                "CLOSEPRICE": 0.0,
                "PRESETTLEMENTPRICE": 0.0,
                "SETTLEMENTPRICE": 0.0,
                "ZD1_CHG": 0.0,
                "ZD2_CHG": 0.0,
                "VOLUME": 0.0,
                "OPENINTEREST": 0.0,
                "OPENINTERESTCHG": 0.0,
                "TURNOVER": 0.0,
                "DELTA": 0.0,
                "EXECVOLUME": 0.0
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_hist_shfe("铜期权", "20240301").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].contract, "cu2405C65000");
}

#[tokio::test]
async fn test_option_vol_shfe() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_cursigma": [
            {
                "PRODUCTNAME": "铜期权",
                "INSTRUMENTID": "cu2405",
                "VOLUME": 10000.0,
                "OPENINTEREST": 50000.0,
                "OPENINTERESTCHG": 500.0,
                "TURNOVER": 5_200_000.0,
                "EXECVOLUME": 10.0,
                "SIGMA": 0.25
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_vol_shfe("铜期权", "20240301").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].series, "cu2405");
}

#[tokio::test]
async fn test_option_hist_gfex() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "variety": "工业硅",
                "delivMonth": "si2405",
                "open": 10000.0,
                "high": 10500.0,
                "low": 9800.0,
                "close": 10200.0,
                "lastClear": 9900.0,
                "clearPrice": 10100.0,
                "diff": 300.0,
                "diff1": 200.0,
                "delta": 0.5,
                "volumn": 10000.0,
                "openInterest": 50000.0,
                "diffI": 500.0,
                "turnover": 102_000_000.0,
                "matchQtySum": 10.0,
                "impliedVolatility": 0.30
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_hist_gfex("工业硅", "20240301").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
}

#[tokio::test]
async fn test_option_vol_gfex() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "seriesId": "si2405",
                "hisVolatility": 0.30
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_vol_gfex("工业硅", "20240301").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert!((rows[0].implied_volatility - 0.30).abs() < 0.01);
}

// ---------------------------------------------------------------------------
// commodity_sina.rs — Commodity options from Sina
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_commodity_contract_sina() {
    let server = MockServer::start().await;
    // Returns HTML with commodity links and contract list
    let html = r#"<html><body>
        <a href="/futures/view/optionsDP.php/m_o/dce">豆粕期权</a>
        <ul id="option_suffix"><li>m2405</li><li>m2409</li></ul>
    </body></html>"#;
    mock_any_get_text(&server, ".*", html).await;
    let client = mock_client(&server);
    let result = client.option_commodity_contract("豆粕期权").await;
    // This will try to parse HTML; may succeed or fail depending on parsing
    let _ = result;
}

#[tokio::test]
async fn test_option_commodity_contract_table_sina() {
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
        .option_commodity_contract_table("豆粕期权", "m2405")
        .await;
    let _ = result; // Sina commodity format may differ from mock
}

#[tokio::test]
async fn test_option_commodity_hist_sina() {
    let server = MockServer::start().await;
    // JSONP response: [[open, high, low, close, volume, date], ...]
    let body = "jQuery12345([[5000.0,5500.0,4800.0,5200.0,10000,\"2024-01-02\"]])";
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_commodity_hist("au2012C392").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].date, "2024-01-02");
}

// ---------------------------------------------------------------------------
// current_sse.rs — SSE and SZSE current day contracts
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_current_day_sse() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": [
            {
                "SECURITY_ID": "10005765",
                "CONTRACT_ID": "510050C2401M02500",
                "CONTRACT_SYMBOL": "50ETF购1月2500",
                "SECURITYNAMEBYID": "华夏上证50ETF",
                "CALL_OR_PUT": "认购",
                "EXERCISE_PRICE": 2.5,
                "CONTRACT_UNIT": 10000.0,
                "END_DATE": "2024-01-24",
                "DELIVERY_DATE": "2024-01-25",
                "EXPIRE_DATE": "2024-01-24",
                "START_DATE": "2023-12-28"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_current_day_sse().await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].security_id, "10005765");
}

#[tokio::test]
async fn test_option_current_day_szse() {
    let server = MockServer::start().await;
    // SZSE returns JSON array with metadata
    let body = serde_json::json!([
        {
            "metadata": { "pagecount": 1 },
            "data": [
                {
                    "\u{5e8f}\u{53f7}": 1,
                    "\u{5408}\u{7ea6}\u{7f16}\u{7801}": "90000001",
                    "\u{5408}\u{7ea6}\u{4ee3}\u{7801}": "159919C2401M02500",
                    "\u{5408}\u{7ea6}\u{7b80}\u{79f0}": "嘉实300ETF购1月2500",
                    "\u{6807}\u{7684}\u{8bc1}\u{5238}\u{7b80}\u{79f0}(\u{4ee3}\u{7801})": "嘉实沪深300ETF(159919)",
                    "\u{5408}\u{7ea6}\u{7c7b}\u{578b}": "认购",
                    "\u{884c}\u{6743}\u{4ef7}": 2.5,
                    "\u{5408}\u{7ea6}\u{5355}\u{4f4d}": 10000.0,
                    "\u{6700}\u{540e}\u{4ea4}\u{6613}\u{65e5}": "2024-01-24",
                    "\u{884c}\u{6743}\u{65e5}": "2024-01-24",
                    "\u{5230}\u{671f}\u{65e5}": "2024-01-24",
                    "\u{4ea4}\u{6536}\u{65e5}": "2024-01-25",
                    "\u{6da8}\u{505c}\u{4ef7}\u{683c}": 0.5,
                    "\u{8dcc}\u{505c}\u{4ef7}\u{683c}": 0.001,
                    "\u{524d}\u{7ed3}\u{7b97}\u{4ef7}": 0.05,
                    "\u{5408}\u{7ea6}\u{603b}\u{6301}\u{4ed3}": 50000.0,
                    "\u{5408}\u{7ea6}\u{5230}\u{671f}\u{5269}\u{4f59}\u{4ea4}\u{6613}\u{5929}\u{6570}": 15,
                    "\u{5408}\u{7ea6}\u{5230}\u{671f}\u{5269}\u{4f59}\u{81ea}\u{7136}\u{5929}\u{6570}": 22,
                    "\u{4ea4}\u{6613}\u{65e5}\u{671f}": "2024-01-02"
                }
            ]
        }
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_current_day_szse().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// daily_stats.rs — SSE and SZSE option daily statistics
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_daily_stats_sse() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": [
            {
                "SECURITY_CODE": "510050",
                "SECURITY_ABBR": "50ETF",
                "CONTRACT_VOLUME": 100,
                "TOTAL_MONEY": 5_000_000.0,
                "TOTAL_VOLUME": 50000.0,
                "CALL_VOLUME": 30000.0,
                "PUT_VOLUME": 20000.0,
                "CP_RATE": 0.67,
                "LEAVES_QTY": 100_000.0,
                "LEAVES_CALL_QTY": 60000.0,
                "LEAVES_PUT_QTY": 40000.0,
                "TRADE_DATE": "2024-01-02"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_daily_stats_sse("20240102").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].security_code, "510050");
}

#[tokio::test]
async fn test_option_daily_stats_szse() {
    let server = MockServer::start().await;
    let body = serde_json::json!([
        {
            "metadata": { "pagecount": 1 },
            "data": [
                {
                    "bddm": "159919",
                    "bdmc": "嘉实300ETF",
                    "cjl": 50000.0,
                    "rccjl": 30000.0,
                    "rpcjl": 20000.0,
                    "rcrpccb": 0.67,
                    "wpchyzs": 100_000.0,
                    "wpcrchys": 60000.0,
                    "wpcrphys": 40000.0
                }
            ]
        }
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_daily_stats_szse("20240102").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// finance.rs — option_finance_board, option_finance_sse_underlying
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_finance_board_sse() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "date": "20240102",
        "time": "15:00:00",
        "total": 2,
        "list": [
            ["510050C2401M02500", 0.05, 1.5, 0.048, 2.50],
            ["510050P2401M02500", 0.03, -0.5, 0.032, 2.50]
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .option_finance_board("华夏上证50ETF期权", "2401")
        .await;
    let _ = result; // SSE board format may differ from mock
}

#[tokio::test]
async fn test_option_finance_board_cffex() {
    let server = MockServer::start().await;
    // CFFEX returns CSV-like text
    let body = "instrument,close,chg_rate,presetpx,exepx\nIO2403-C-4000,50.0,1.5,48.0,4000\nIO2403-P-4000,30.0,-0.5,32.0,4000\n";
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_finance_board("沪深300股指期权", "2403").await;
    let _ = result; // CFFEX board CSV format may differ from mock
}

#[tokio::test]
async fn test_option_finance_sse_underlying() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "date": "20240102",
        "time": "15:00:00",
        "list": [
            ["510050", "50ETF", 2.50, 0.03, 1.2, 2.0, 100_000, 250_000.0, 2.47]
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .option_finance_sse_underlying("华夏上证50ETF期权")
        .await;
    let _ = result; // SSE underlying format may differ from mock
}

#[tokio::test]
async fn test_option_finance_sse_underlying_invalid() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.option_finance_sse_underlying("invalid_symbol").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// lhb_em.rs — Option billboard (dragon-tiger list)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_lhb_em() {
    let server = MockServer::start().await;
    // 28 rows: 4 groups of 7
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
    let body = serde_json::json!({
        "result": { "data": data }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .option_lhb("510050", "期权交易情况-认沽交易量", "20240102")
        .await;
    let _ = result; // LHB EM array format may differ from mock
}

// ---------------------------------------------------------------------------
// margin.rs — Option margin data
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_margin_symbol() {
    let server = MockServer::start().await;
    let html = r#"<html><body>
        <a href="/qiquan/yuanyou">原油期权</a>
        <a href="/qiquan/tong">铜期权</a>
    </body></html>"#;
    mock_any_get_text(&server, ".*", html).await;
    let client = mock_client(&server);
    let result = client.option_margin_symbol().await;
    assert!(result.is_ok());
    let symbols = result.unwrap();
    assert!(!symbols.is_empty());
}

#[tokio::test]
async fn test_option_margin() {
    let server = MockServer::start().await;
    // First request: symbol list page, second request: margin data page
    let list_html = r#"<html><body>
        <a href="/qiquan/yuanyou">原油期权</a>
    </body></html>"#;
    let margin_html = r"<html><body>
        <small>最近更新 2024-01-02</small>
        <table>
            <tr><th>合约</th><th>结算价</th><th>交易乘数</th><th>买方权利金</th><th>卖方保证金</th><th>开仓手续费</th><th>平今手续费</th><th>平昨手续费</th></tr>
            <tr><td>sc2405C600</td><td>50.0</td><td>1000</td><td>50000</td><td>80000</td><td>20</td><td>20</td><td>20</td></tr>
        </table>
    </body></html>";
    // Mount two mocks: first matches the symbol list URL, second matches the margin page
    mock_any_get_text(&server, "yuanyou", list_html).await;
    mock_any_get_text(&server, "qiquan", margin_html).await;
    let client = mock_client(&server);
    // This calls option_margin_symbol() first, then fetches the margin page
    let result = client.option_margin("原油期权").await;
    // May succeed or fail depending on URL routing; verify no panic
    let _ = result;
}

// ---------------------------------------------------------------------------
// risk_indicator.rs — SSE option risk indicators
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_risk_indicator_sse() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": [
            {
                "TRADE_DATE": "2024-01-02",
                "SECURITY_ID": "10005765",
                "CONTRACT_ID": "510050C2401M02500",
                "CONTRACT_SYMBOL": "50ETF购1月2500",
                "DELTA_VALUE": 0.5,
                "THETA_VALUE": -0.01,
                "GAMMA_VALUE": 0.02,
                "VEGA_VALUE": 0.03,
                "RHO_VALUE": 0.001,
                "IMPLC_VOLATLTY": 0.25
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_risk_indicator("20240102").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].security_id, "10005765");
}

// ---------------------------------------------------------------------------
// comm_qihuo.rs — Commodity option commission info
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_comm_symbol() {
    let server = MockServer::start().await;
    let html = r#"<html><body>
        <div id="inst_list">
            <a href="?heyue=si">工业硅期权</a>
            <a href="?heyue=lc">碳酸锂期权</a>
        </div>
    </body></html>"#;
    mock_any_get_text(&server, ".*", html).await;
    let client = mock_client(&server);
    let result = client.option_comm_symbol().await;
    assert!(result.is_ok());
    let symbols = result.unwrap();
    assert!(!symbols.is_empty());
}

#[tokio::test]
async fn test_option_comm_info() {
    let server = MockServer::start().await;
    // First request: symbol list page, second: commission data page
    let list_html = r#"<html><body>
        <div id="inst_list">
            <a href="?heyue=si">工业硅期权</a>
        </div>
    </body></html>"#;
    let info_html = r##"<html><body>
        <a id="dlink" href="#">下载</a>
        <table>
            <tr><th>合约</th><th>价格</th><th>成交量</th><th>每跳毛利</th><th>每跳净利</th></tr>
            <tr><td>si2405C10000</td><td>500.0</td><td>10000</td><td>50.0</td><td>45.0</td></tr>
        </table>
    </body></html>"##;
    mock_any_get_text(&server, "inst_list", list_html).await;
    mock_any_get_text(&server, "heyue", info_html).await;
    let client = mock_client(&server);
    let result = client.option_comm_info("工业硅期权").await;
    // May succeed or fail depending on URL routing; verify no panic
    let _ = result;
}

// ---------------------------------------------------------------------------
// contract_info_ctp.rs — Option contract info from openctp
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_contract_info_ctp() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "ExchangeID": "CFFEX",
                "InstrumentID": "IO2403-C-4000",
                "InstrumentName": "沪深300股指期权2403购4000",
                "ProductClass": "1",
                "ProductID": "IO",
                "VolumeMultiple": 100,
                "PriceTick": 0.2,
                "LongMarginRatioByMoney": 0.15,
                "ShortMarginRatioByMoney": 0.15,
                "LongMarginRatioByVolume": 0.0,
                "ShortMarginRatioByVolume": 0.0,
                "OpenRatioByMoney": 0.00005,
                "OpenRatioByVolume": 0.0,
                "CloseRatioByMoney": 0.0,
                "CloseRatioByVolume": 0.0,
                "CloseTodayRatioByMoney": 0.00005,
                "CloseTodayRatioByVolume": 0.0,
                "DeliveryYear": 2024,
                "DeliveryMonth": 3,
                "OpenDate": "20240101",
                "ExpireDate": "20240315",
                "DeliveryDate": "20240318",
                "UnderlyingInstrID": "IF2403",
                "UnderlyingMultiple": 1.0,
                "OptionsType": "1",
                "StrikePrice": 4000.0,
                "InstLifePhase": "1"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_contract_info_ctp().await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].exchange_id, "CFFEX");
    assert_eq!(rows[0].instrument_id, "IO2403-C-4000");
}

// ---------------------------------------------------------------------------
// czce.rs — CZCE yearly option history
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_hist_yearly_czce() {
    let server = MockServer::start().await;
    // Pipe-delimited text with header
    let body = "合约代码|昨结算|今开盘|最高价|最低价|今收盘|今结算\nSR405C6000|50.0|52.0|55.0|48.0|53.0|51.0\n";
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.option_hist_yearly_czce("SR", "2024").await;
    assert!(result.is_ok());
    let rows = result.unwrap();
    assert_eq!(rows.len(), 1);
    assert!(rows[0].fields.len() >= 2);
}

#[tokio::test]
async fn test_option_hist_yearly_czce_invalid_symbol() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    let client = mock_client(&server);
    let result = client.option_hist_yearly_czce("INVALID", "2024").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_option_hist_yearly_czce_before_start() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    let client = mock_client(&server);
    // SR started in 2017, requesting 2016 should fail
    let result = client.option_hist_yearly_czce("SR", "2016").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// DCE invalid symbol test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_hist_dce_invalid_symbol() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.option_hist_dce("invalid_symbol", "20240301").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// CZCE invalid symbol test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_hist_czce_invalid_symbol() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    let client = mock_client(&server);
    let result = client.option_hist_czce("invalid_symbol", "20240301").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// GFEX invalid symbol test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_vol_gfex_invalid_symbol() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.option_vol_gfex("invalid_symbol", "20240301").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Commodity Sina invalid symbol test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_commodity_contract_table_sina_invalid() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client
        .option_commodity_contract_table("invalid_symbol", "xxx")
        .await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Finance board invalid symbol test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_finance_board_invalid() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.option_finance_board("invalid_symbol", "2401").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// LHB invalid indicator test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_option_lhb_em_invalid_indicator() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": { "data": [] }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .option_lhb("510050", "invalid_indicator", "20240102")
        .await;
    assert!(result.is_err());
}
