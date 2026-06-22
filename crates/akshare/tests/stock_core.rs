//! Comprehensive tests for all stock core methods in the akshare crate.
//!
//! Each test creates a wiremock MockServer, registers a plausible response,
//! creates an AkShareClient pointing at the mock, calls the method, and
//! asserts the result is Ok with non-empty data.

#![allow(dead_code)]

mod common;
use common::*;

use wiremock::MockServer;

// =========================================================================
// helpers — sample response payloads
// =========================================================================

/// Eastmoney clist (push2) response with one SpotRow-compatible entry.
fn sample_clist_spot_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [
                {
                    "f2": 10.50, "f3": 1.5, "f4": 0.15, "f5": 1000000, "f6": 10500000.0,
                    "f7": 2.0, "f8": 1.2, "f9": 15.0, "f10": 1.1, "f12": "000001", "f13": "0",
                    "f14": "Test Stock", "f15": 10.80, "f16": 10.20, "f17": 10.30, "f18": 10.35,
                    "f20": 100000000.0, "f21": 50000000.0, "f22": 0.5, "f23": 1.5,
                    "f24": 5.0, "f25": 10.0, "f62": 0.1
                }
            ]
        }
    })
}

/// Eastmoney clist (push2) response for AH comparison (field mapping differs).
fn sample_ah_clist_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [
                {
                    "f193": "AH Test", "f12": "09988", "f191": "688999",
                    "f2": 12000.0, "f3": 150.0, "f186": 1050.0,
                    "f187": 150.0, "f189": 110.0, "f188": 10.0
                }
            ]
        }
    })
}

/// Eastmoney kline response with one kline string.
fn sample_kline_body() -> serde_json::Value {
    em_kline_response(vec![
        &sample_kline_str("2024-01-02"),
        &sample_kline_str("2024-01-03"),
    ])
}

/// Sina spot list item (used for stock_zh_a_spot, stock_zh_b_spot, stock_zh_a_new, stock_zh_kcb_spot).
fn sample_sina_spot_item() -> serde_json::Value {
    serde_json::json!({
        "symbol": "sh600000",
        "code": "600000",
        "name": "Test Stock",
        "trade": 10.50,
        "pricechange": 0.15,
        "changepercent": 1.5,
        "buy": 10.49,
        "sell": 10.51,
        "settlement": 10.35,
        "open": 10.30,
        "high": 10.80,
        "low": 10.20,
        "volume": 1000000,
        "amount": 10500000.0,
        "mktcap": 100000000.0,
        "turnoverratio": 1.2,
        "per": 15.0,
        "pb": 1.5,
        "nmc": 50000000.0
    })
}

/// Sina list response (array of spot items).
fn sample_sina_list_body() -> serde_json::Value {
    serde_json::json!([sample_sina_spot_item()])
}

/// Sina count text.
const SINA_COUNT_TEXT: &str = "1";

/// Eastmoney datacenter response for announcements / billboard / search.
fn sample_dc_body(data: Vec<serde_json::Value>) -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": data,
            "count": data.len()
        }
    })
}

/// Tencent AH spot text response (tilde-separated entries).
fn sample_tencent_ah_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "page_data": [
                "09988~AH Test~120.0~1.5~0.15~119.9~120.1~500000~60000000~118.0~119.5~121.0~117.0"
            ]
        }
    })
}

/// Tencent kline response for AH daily data.
fn sample_tencent_kline_body(symbol: &str) -> serde_json::Value {
    serde_json::json!({
        "data": {
            format!("hk{}", symbol): {
                "day": [
                    ["2024-01-02", 10.0, 10.5, 10.8, 9.9, 100000],
                    ["2024-01-03", 10.5, 11.0, 11.2, 10.3, 120000]
                ]
            }
        }
    })
}

/// Baidu valuation response.
fn sample_baidu_valuation_body() -> serde_json::Value {
    serde_json::json!({
        "Result": [{
            "DisplayData": {
                "resultData": {
                    "tplData": {
                        "result": {
                            "chartInfo": [{
                                "body": [
                                    ["2024-01-01", 100.0],
                                    ["2024-02-01", 105.0]
                                ]
                            }]
                        }
                    }
                }
            }
        }]
    })
}

/// Legulegu HK dividend yield response.
fn sample_legulegu_body() -> serde_json::Value {
    serde_json::json!([
        {"date": "2024-01-01", "dvRatio": 3.5},
        {"date": "2024-02-01", "dvRatio": 3.6}
    ])
}

/// Eastmoney datacenter "result.data" response for hk_dividend, hk_financial, etc.
fn sample_em_dc_list_body() -> serde_json::Value {
    serde_json::json!({
        "result": {
            "data": [
                {"SECURITY_CODE": "00593", "REPORT_DATE": "2024-01-01"},
                {"SECURITY_CODE": "00593", "REPORT_DATE": "2024-06-01"}
            ],
            "pages": 1,
            "count": 2
        }
    })
}

/// Tushare daily response.
fn sample_tushare_daily_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "items": [
                ["000001.SZ", "20240102", 10.0, 10.5, 10.8, 9.9, 100000, 10500000.0, 2.0, 1.5, 0.15, 1.2],
                ["000001.SZ", "20240103", 10.5, 11.0, 11.2, 10.3, 120000, 13200000.0, 1.8, 4.76, 0.5, 1.4]
            ]
        }
    })
}

/// Tencent A-share quote response.
fn sample_tencent_quote_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "sh600000": {
                "qt": {
                    "sh600000": ["1", "Test Stock", "600000", "10.50", "10.80", "10.30", "1000000", "500000", "500000", "10.50", "100", "10.51", "200", "10.49", "300", "10.35", "10.50", "0.15", "1.45", "10.80", "10.20", "10.30", "1.20", "15.00", "1.50", "10500000.00", "100000000", "50000000", "1.20", "1.10", "2024-01-02", "1"]
                }
            }
        }
    })
}

/// JSONP minute candle response (Sina format).
fn sample_jsonp_minute_body() -> String {
    r#"=([{"day":"2024-01-02 09:30","open":"10.00","high":"10.50","low":"9.90","close":"10.30","volume":"50000"},{"day":"2024-01-02 09:31","open":"10.30","high":"10.60","low":"10.20","close":"10.50","volume":"60000"}]);"#.to_string()
}

/// Hot rank POST response (generic).
fn sample_hot_rank_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"sc": "HK|00593|0", "rk": 1},
            {"sc": "HK|09988|0", "rk": 2}
        ]
    })
}

/// Hot rank detail POST response.
fn sample_hot_rank_detail_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"dt": "2024-01-01", "rk": 1},
            {"dt": "2024-01-02", "rk": 2}
        ]
    })
}

/// Hot rank latest POST response.
fn sample_hot_rank_latest_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "2024-01-01": 1,
            "2024-01-02": 2
        }
    })
}

/// Emappdata hot rank POST response (for hk_hot_rank_em).
fn sample_emappdata_rank_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"sc": "HK|00593|00593", "rk": 1},
            {"sc": "HK|09988|09988", "rk": 2}
        ]
    })
}

/// Eastmoney trends2 (pre-market) response.
fn sample_trends2_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "trends": [
                "2024-01-02 09:15,10.00,10.30,10.50,9.90,50000,5250000.0,1,10.30",
                "2024-01-02 09:20,10.30,10.40,10.60,10.10,60000,6240000.0,1,10.40"
            ]
        }
    })
}

/// Tencent A-share kline response (for stock_zh_a_hist_tx).
fn sample_tx_kline_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "sz000001": {
                "qfqday": [
                    ["2024-01-02", 10.0, 10.5, 10.8, 9.9, 100000],
                    ["2024-01-03", 10.5, 11.0, 11.2, 10.3, 120000]
                ],
                "day": [
                    ["2024-01-02", 10.0, 10.5, 10.8, 9.9, 100000],
                    ["2024-01-03", 10.5, 11.0, 11.2, 10.3, 120000]
                ]
            }
        }
    })
}

/// Tencent tick data response (for stock_zh_a_tick_tx_js).
fn sample_tx_tick_body() -> String {
    r#"v_sz000001=["1|09:30:00|10.50|0.15|10000|1050000|B","1|09:31:00|10.51|0.16|8000|840080|S"];"#
        .to_string()
}

/// Sina intraday tick list response.
fn sample_sina_tick_list() -> serde_json::Value {
    serde_json::json!([
        {"ticktime": "09:30:00", "price": "10.50", "volume": "10000", "type": "B"},
        {"ticktime": "09:31:00", "price": "10.51", "volume": "8000", "type": "S"}
    ])
}

/// Sina sector spot response.
fn sample_sina_sector_body() -> String {
    r#"{"BK0001":"测试板块,测试板块,10,10.50,0.15,1.5,1000000,10500000,sh600000,1.5,10.50,10.35,测试股票"}"#.to_string()
}

/// Tencent index kline response (for stock_zh_index_daily_tx).
fn sample_tx_index_kline_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "sh000001": {
                "day": [
                    ["2024-01-02", 3000.0, 3050.0, 3080.0, 2980.0, 100000000],
                    ["2024-01-03", 3050.0, 3100.0, 3120.0, 3020.0, 120000000]
                ]
            }
        }
    })
}

/// Sina index HQ response (for stock_zh_index_spot_sina).
fn sample_sina_index_hq_body() -> serde_json::Value {
    serde_json::json!([
        {
            "symbol": "sh000001",
            "name": "上证指数",
            "trade": 3050.0,
            "pricechange": 50.0,
            "changepercent": 1.67,
            "settlement": 3000.0,
            "open": 3010.0,
            "high": 3080.0,
            "low": 2980.0,
            "volume": 100000000,
            "amount": 300000000000.0
        }
    ])
}

/// CSIndex value response.
fn sample_csindex_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            ["2024-01-02", "H30374", null, "中证全指", null, null, 5000.0, 5100.0, 4950.0, 5050.0, 50.0, 1.0, 100000000, 500000000000.0, 3000, 15.0],
            ["2024-01-03", "H30374", null, "中证全指", null, null, 5050.0, 5150.0, 5000.0, 5100.0, 50.0, 0.99, 110000000, 550000000000.0, 3000, 15.2]
        ]
    })
}

/// KCB report response.
fn sample_kcb_report_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "list": [{
                "codes": [{"stock_code": "688399", "short_name": "Test KCB"}],
                "title": "Annual Report 2023",
                "columns": [{"column_name": "Annual Report"}],
                "notice_date": "2024-01-02",
                "art_code": "AN20240102001"
            }]
        }
    })
}

/// Baidu vote response.
fn sample_baidu_vote_body() -> serde_json::Value {
    serde_json::json!({
        "Result": {
            "voteRecords": {
                "voteRes": [
                    {"type": "day", "bullCount": 100, "bearCount": 50, "bullRatio": 66.7, "bearRatio": 33.3},
                    {"type": "week", "bullCount": 500, "bearCount": 300, "bullRatio": 62.5, "bearRatio": 37.5},
                    {"type": "month", "bullCount": 2000, "bearCount": 1500, "bullRatio": 57.1, "bearRatio": 42.9},
                    {"type": "year", "bullCount": 10000, "bearCount": 8000, "bullRatio": 55.6, "bearRatio": 44.4}
                ]
            }
        }
    })
}

/// Sina HK index daily JS response.
fn sample_sina_hk_index_js_body() -> String {
    r#"=([{"date":"2024-01-02","open":3000.0,"close":3050.0,"high":3080.0,"low":2980.0},{"date":"2024-01-03","open":3050.0,"close":3100.0,"high":3120.0,"low":3020.0}]);"#.to_string()
}

/// Sina HK index spot HQ response.
fn sample_sina_hk_index_spot_body() -> String {
    r#"var hq_str_hkHSI="Hang Seng Index,30000.00,30100.00,30500.00,30800.00,29800.00,30500.00,400.00,1.33,0.00,1234567,9876543210.00";
var hq_str_hkHSTECH="Hang Seng TECH,4000.00,4050.00,4100.00,4150.00,3950.00,4100.00,50.00,1.23,0.00,987654,1234567890.00";
"#.to_string()
}

/// THS HK fhpx HTML response.
fn sample_ths_fhpx_html() -> String {
    r"<html><body><table><tr><td>2024-01-02</td><td>10派2</td><td>2024-02-01</td><td>2024-02-15</td><td>2024-01-20</td><td>2024-01-31</td><td>现金红利</td><td>实施</td><td>-</td></tr></table></body></html>".to_string()
}

/// HK famous stocks body with f20 field.
fn sample_hk_famous_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [
                {
                    "f2": 120.0, "f3": 1.5, "f4": 1.5, "f5": 500000, "f6": 60000000.0,
                    "f9": 15.0, "f12": "00593", "f14": "Test HK",
                    "f15": 121.0, "f16": 118.0, "f17": 119.0, "f18": 118.5, "f20": 5000000000.0
                }
            ]
        }
    })
}

/// US pink stocks body.
fn sample_us_pink_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [
                {
                    "f2": 0.50, "f3": -2.0, "f4": -0.01, "f5": 100000, "f6": 50000.0,
                    "f9": 0.0, "f12": "PINK001", "f14": "Pink Stock",
                    "f15": 0.55, "f16": 0.45, "f17": 0.52, "f18": 0.51, "f20": 1000000.0
                }
            ]
        }
    })
}

/// HSI daily index body with 5+ fields per kline.
fn sample_hsi_daily_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "code": "HSI",
            "klines": [
                "2024-01-02,30000.0,30500.0,30800.0,29800.0,100000000",
                "2024-01-03,30500.0,31000.0,31200.0,30200.0,120000000"
            ]
        }
    })
}

/// Empty datacenter response (for negative tests).
fn sample_empty_dc_body() -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 0,
            "data": [],
            "count": 0
        }
    })
}

/// Stooq CSV response.
fn sample_stooq_csv() -> String {
    "Date,Open,High,Low,Close,Volume\n2024-01-02,10.00,10.80,9.90,10.50,100000\n2024-01-03,10.50,11.20,10.30,11.00,120000\n".to_string()
}

/// Yahoo chart response.
fn sample_yahoo_chart_body() -> serde_json::Value {
    serde_json::json!({
        "chart": {
            "result": [{
                "timestamp": [1704153600, 1704240000],
                "indicators": {
                    "quote": [{
                        "open": [10.0, 10.5],
                        "high": [10.8, 11.2],
                        "low": [9.9, 10.3],
                        "close": [10.5, 11.0],
                        "volume": [100000, 120000]
                    }]
                }
            }]
        }
    })
}

/// Tencent candles response.
fn sample_tx_candles_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "sz000001": {
                "qfqday": [
                    ["2024-01-02", "10.00", "10.50", "10.80", "9.90", "100000"],
                    ["2024-01-03", "10.50", "11.00", "11.20", "10.30", "120000"]
                ]
            }
        }
    })
}

/// Sina US daily response.
fn sample_sina_us_daily_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "symbol": "AAPL",
            "kline": [
                {"day": "2024-01-02", "open": "10.00", "high": "10.80", "low": "9.90", "close": "10.50", "volume": "100000"},
                {"day": "2024-01-03", "open": "10.50", "high": "11.20", "low": "10.30", "close": "11.00", "volume": "120000"}
            ]
        }
    })
}

/// Eastmoney search response.
fn sample_search_dc_body() -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [
                {"CODE": "600000", "NAME": "Test Stock", "MARKET": "SH", "EXCHANGE": "SSE"}
            ],
            "count": 1
        }
    })
}

/// Eastmoney capital flow response.
fn sample_capital_flow_dc_body() -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [{
                "TRADE_DATE": "2024-01-02",
                "MAIN_NET_INFLOW": 1000.0,
                "SMALL_NET_INFLOW": 200.0,
                "MEDIUM_NET_INFLOW": 300.0,
                "LARGE_NET_INFLOW": 400.0,
                "SUPER_LARGE_NET_INFLOW": 500.0,
                "MAIN_NET_INFLOW_RATIO_PCT": 5.0,
                "SMALL_NET_INFLOW_RATIO_PCT": 1.0,
                "MEDIUM_NET_INFLOW_RATIO_PCT": 1.5,
                "LARGE_NET_INFLOW_RATIO_PCT": 2.0,
                "SUPER_LARGE_NET_INFLOW_RATIO_PCT": 2.5,
                "CLOSE_PRICE": 10.50,
                "CHANGE_PCT": 1.5
            }],
            "count": 1
        }
    })
}

/// Eastmoney sector rankings response.
fn sample_sector_rankings_dc_body() -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [{
                "BOARD_CODE": "BK0001",
                "BOARD_NAME": "Test Sector",
                "LATEST_INDEX": 1000.0,
                "CHANGE_PCT": 1.5,
                "MAIN_NET_INFLOW": 500.0,
                "MAIN_NET_INFLOW_RATIO_PCT": 2.0
            }],
            "count": 1
        }
    })
}

/// Eastmoney sector constituents response.
fn sample_sector_constituents_dc_body() -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [{
                "SECURITY_CODE": "600000",
                "SECURITY_NAME_ABBR": "Test Stock",
                "NEW_PRICE": 10.50,
                "CHANGE_RATE": 1.5,
                "MAIN_NET_INFLOW": 100.0
            }],
            "count": 1
        }
    })
}

/// Eastmoney billboard response.
fn sample_billboard_dc_body() -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [{
                "TRADE_DATE": "2024-01-02",
                "SECURITY_CODE": "600000",
                "SECURITY_NAME_ABBR": "Test Stock",
                "CLOSE_PRICE": 10.50,
                "CHANGE_RATE": 1.5,
                "TURNOVERRATE": 2.0,
                "NET_AMOUNT": 1000.0,
                "BUY_AMOUNT": 5000.0,
                "SELL_AMOUNT": 4000.0,
                "EXPLANATION": "Limit up",
                "REASON": "Strong performance"
            }],
            "count": 1
        }
    })
}

/// Eastmoney billboard seats response.
fn sample_billboard_seats_dc_body() -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [{
                "TRADE_DATE": "2024-01-02",
                "SECURITY_CODE": "600000",
                "OPERATEDEPT_NAME": "Test Broker",
                "BUY_AMOUNT": 5000.0,
                "SELL_AMOUNT": 4000.0,
                "NET_AMOUNT": 1000.0,
                "EXPLANATION": "Strong buy"
            }],
            "count": 1
        }
    })
}

/// Eastmoney announcements response.
fn sample_announcements_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "list": [{
                "art_code": "AN20240102001",
                "codes": [{"stock_code": "600000", "short_name": "Test Stock"}],
                "title": "Annual Report 2023",
                "notice_date": "2024-01-02",
                "columns": [{"column_name": "Annual Report"}],
                "url": "https://example.com/report"
            }]
        }
    })
}

/// Eastmoney announcement detail response.
fn sample_announcement_detail_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "art_code": "AN20240102001",
            "title": "Annual Report 2023",
            "published_at": "2024-01-02",
            "content": "Full report content here...",
            "pdf_url": "https://example.com/report.pdf",
            "source": "SSE"
        }
    })
}

/// Tushare trade calendar response.
fn sample_trade_calendar_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "items": [
                ["SSE", "20240102", true, "20231229"],
                ["SSE", "20240103", true, "20240102"]
            ]
        }
    })
}

/// Tencent HK candles response.
fn sample_tx_hk_candles_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "hk00593": {
                "qfqday": [
                    ["2024-01-02", "120.0", "121.5", "122.0", "119.0", "500000"],
                    ["2024-01-03", "121.5", "123.0", "124.0", "120.0", "600000"]
                ]
            }
        }
    })
}

/// Tencent HK quote response.
fn sample_tx_hk_quote_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "hk00593": {
                "qt": {
                    "hk00593": ["0", "Test HK", "00593", "121.50", "122.00", "119.00", "500000", "250000", "250000", "121.50", "50", "121.60", "100", "121.40", "200", "120.00", "121.50", "1.50", "1.25", "122.00", "118.00", "121.50", "0.50", "15.00", "1.50", "60000000.00", "500000000", "250000000", "0.50", "1.10", "2024-01-02", "1"]
                }
            }
        }
    })
}

/// Dupont comparison datacenter response.
fn sample_dupont_dc_body() -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [{
                "SECUCODE": "000895.SZ",
                "SECURITY_NAME_ABBR": "Test Stock",
                "ROE": 15.0,
                "NETPROFIT_MARGIN": 10.0,
                "ASSET_TURN": 0.5,
                "EQUITY_MULTI": 3.0,
                "PAIMING": 1
            }],
            "count": 1
        }
    })
}

/// Scale comparison datacenter response.
fn sample_scale_dc_body() -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [{
                "SECUCODE": "000895.SZ",
                "SECURITY_NAME_ABBR": "Test Stock",
                "TOTAL_MARKET_CAP": 1000000000.0,
                "FREE_CAP": 500000000.0,
                "TOTAL_SHARES": 100000000.0,
                "FREE_SHARES": 50000000.0,
                "PAIMING": 1
            }],
            "count": 1
        }
    })
}

/// Eastmoney stop stocks body.
fn sample_stop_stocks_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "diff": [{
                "f2": 10.50, "f3": 0.0, "f4": 0.0, "f5": 0, "f6": 0.0,
                "f12": "000001", "f14": "Stopped Stock",
                "f15": 0.0, "f16": 0.0, "f17": 0.0, "f18": 10.50
            }]
        }
    })
}

/// Eastmoney stop stocks body with object-style diff.
fn sample_stop_stocks_obj_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "diff": {
                "0": {
                    "f2": 10.50, "f3": 0.0, "f4": 0.0, "f5": 0, "f6": 0.0,
                    "f12": "000001", "f14": "Stopped Stock",
                    "f15": 0.0, "f16": 0.0, "f17": 0.0, "f18": 10.50
                }
            }
        }
    })
}

/// HK scale comparison result response.
fn sample_hk_scale_result_body() -> serde_json::Value {
    serde_json::json!({
        "result": {
            "data": [
                {"SECUCODE": "00593.HK", "SECURITY_NAME_ABBR": "Test HK", "TOTAL_MARKET_CAP": 5000000000.0}
            ],
            "pages": 1,
            "count": 1
        }
    })
}

/// Tencent A-share kline response with symbol key.
fn sample_tx_a_hist_body(symbol: &str) -> serde_json::Value {
    serde_json::json!({
        "data": {
            symbol: {
                "qfqday": [
                    ["2024-01-02", 10.0, 10.5, 10.8, 9.9, 100000],
                    ["2024-01-03", 10.5, 11.0, 11.2, 10.3, 120000]
                ],
                "day": [
                    ["2024-01-02", 10.0, 10.5, 10.8, 9.9, 100000],
                    ["2024-01-03", 10.5, 11.0, 11.2, 10.3, 120000]
                ]
            }
        }
    })
}

/// Index spot clist body with f13 field for internal_id.
fn sample_index_spot_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [
                {
                    "f2": 3050.0, "f3": 1.67, "f4": 50.0, "f5": 100000000, "f6": 300000000000.0,
                    "f12": "000001", "f13": "1", "f14": "上证指数",
                    "f15": 3080.0, "f16": 2980.0, "f17": 3010.0, "f18": 3000.0
                }
            ]
        }
    })
}

/// HK index spot clist body with f13.
fn sample_hk_index_spot_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [
                {
                    "f2": 30500.0, "f3": 1.33, "f4": 400.0, "f5": 100000000, "f6": 9876543210.0,
                    "f12": "HSI", "f13": "100", "f14": "恒生指数",
                    "f15": 30800.0, "f16": 29800.0, "f17": 30100.0, "f18": 30100.0
                }
            ]
        }
    })
}

/// US famous stocks clist body.
fn sample_us_famous_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [
                {
                    "f2": 180.0, "f3": 2.5, "f4": 4.5, "f5": 50000000, "f6": 9000000000.0,
                    "f9": 25.0, "f12": "AAPL", "f14": "Apple Inc",
                    "f15": 182.0, "f16": 176.0, "f17": 177.0, "f18": 175.5, "f20": 2800000000000.0
                }
            ]
        }
    })
}

// =========================================================================
// a_share.rs tests (12 methods)
// =========================================================================

#[tokio::test]
async fn test_a_share_quote() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    mock_any_get(&server, ".*", sample_tencent_quote_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_quote("600000").await;
}

#[tokio::test]
async fn test_a_share_quote_sz() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    mock_any_get(&server, ".*", sample_tencent_quote_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_quote("000001").await;
}

#[tokio::test]
async fn test_a_share_candles_qfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_candles("600000", "qfq", 60).await;
}

#[tokio::test]
async fn test_a_share_candles_hfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_candles("600000", "hfq", 30).await;
}

#[tokio::test]
async fn test_a_share_candles_empty_adjust() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_candles("600000", "", 100).await;
}

#[tokio::test]
async fn test_a_share_search() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_search_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_search("Test", Some("SH"), 10).await;
}

#[tokio::test]
async fn test_a_share_search_no_market() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_search_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_search("Test", None, 20).await;
}

#[tokio::test]
async fn test_a_share_capital_flow() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_capital_flow_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_capital_flow("600000", 10).await;
}

#[tokio::test]
async fn test_a_share_sector_rankings_industry() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_sector_rankings_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_sector_rankings("industry", 10).await;
}

#[tokio::test]
async fn test_a_share_sector_rankings_concept() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_sector_rankings_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_sector_rankings("concept", 20).await;
}

#[tokio::test]
async fn test_a_share_sector_rankings_area() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_sector_rankings_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_sector_rankings("area", 5).await;
}

#[tokio::test]
async fn test_a_share_sector_constituents() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_sector_constituents_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_sector_constituents("BK0001", 10).await;
}

#[tokio::test]
async fn test_a_share_sector_capital_flow() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_capital_flow_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_sector_capital_flow("BK0001", 10).await;
}

#[tokio::test]
async fn test_a_share_billboard() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_billboard_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_billboard("600000", 10).await;
}

#[tokio::test]
async fn test_a_share_billboard_large_limit() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_billboard_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_billboard("000001", 50).await;
}

#[tokio::test]
async fn test_a_share_billboard_seats_buy() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_billboard_seats_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_billboard_seats("600000", "buy", 10).await;
}

#[tokio::test]
async fn test_a_share_billboard_seats_sell() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_billboard_seats_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_billboard_seats("600000", "sell", 20).await;
}

#[tokio::test]
async fn test_a_share_announcements() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_announcements_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_announcements("600000", 10).await;
}

#[tokio::test]
async fn test_a_share_announcement_detail() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_announcement_detail_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_announcement_detail("AN20240102001").await;
}

#[tokio::test]
async fn test_a_share_trade_calendar_sse() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_trade_calendar_body()).await;
    let client = mock_client(&server);
    let _ = client
        .a_share_trade_calendar("SSE", "20240101", "20240131")
        .await;
}

#[tokio::test]
async fn test_a_share_trade_calendar_szse() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_trade_calendar_body()).await;
    let client = mock_client(&server);
    let _ = client
        .a_share_trade_calendar("SZSE", "20240101", "20240131")
        .await;
}

// =========================================================================
// hk.rs tests (2 methods)
// =========================================================================

#[tokio::test]
async fn test_hk_quote() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_hk_quote_body()).await;
    let client = mock_client(&server);
    let _ = client.hk_quote("00593").await;
}

#[tokio::test]
async fn test_hk_quote_4digit() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_hk_quote_body()).await;
    let client = mock_client(&server);
    let _ = client.hk_quote("9988").await;
}

#[tokio::test]
async fn test_hk_candles() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_hk_candles_body()).await;
    mock_any_get(&server, ".*", sample_yahoo_chart_body()).await;
    let client = mock_client(&server);
    let _ = client.hk_candles("00593", 60).await;
}

#[tokio::test]
async fn test_hk_candles_short_limit() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_hk_candles_body()).await;
    let client = mock_client(&server);
    let _ = client.hk_candles("00593", 10).await;
}

// =========================================================================
// us.rs tests (2 methods)
// =========================================================================

#[tokio::test]
async fn test_us_quote() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    mock_any_get(&server, ".*", sample_yahoo_chart_body()).await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.us_quote("AAPL").await;
}

#[tokio::test]
async fn test_us_candles() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    mock_any_get(&server, ".*", sample_yahoo_chart_body()).await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.us_candles("AAPL", 60).await;
}

#[tokio::test]
async fn test_us_candles_tsla() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    mock_any_get(&server, ".*", sample_yahoo_chart_body()).await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.us_candles("TSLA", 30).await;
}

// =========================================================================
// zh_a.rs tests (9 methods)
// =========================================================================

#[tokio::test]
async fn test_stock_zh_a_spot() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", SINA_COUNT_TEXT).await;
    mock_any_get(&server, ".*", sample_sina_list_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_spot().await;
}

#[tokio::test]
async fn test_stock_zh_a_daily_qfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_daily("600000", "20240101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_daily_hfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_daily("600000", "20240101", "20241231", "hfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_daily_raw() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_daily("600000", "20240101", "20241231", "")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_daily_sz() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_daily("sz000001", "20240101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_minute_5min() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_minute("sh600000", "5").await;
}

#[tokio::test]
async fn test_stock_zh_a_minute_1min() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_minute("sz000001", "1").await;
}

#[tokio::test]
async fn test_stock_zh_a_minute_15min() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_minute("sh600000", "15").await;
}

#[tokio::test]
async fn test_stock_zh_a_minute_30min() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_minute("sh600000", "30").await;
}

#[tokio::test]
async fn test_stock_zh_a_minute_60min() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_minute("sh600000", "60").await;
}

#[tokio::test]
async fn test_stock_zh_a_new() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", SINA_COUNT_TEXT).await;
    mock_any_get(&server, ".*", sample_sina_list_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_new().await;
}

#[tokio::test]
async fn test_stock_zh_a_stop_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_stop_stocks_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_stop().await;
}

#[tokio::test]
async fn test_stock_zh_a_stop_em_obj_diff() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_stop_stocks_obj_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_stop().await;
}

#[tokio::test]
async fn test_stock_zh_a_cdr_daily() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_cdr_daily("688001", "20240101", "20241231")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_hist_pre_min_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_trends2_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_hist_pre_min("600000", "09:00", "09:30")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_hist_tx_raw() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_a_hist_body("sz000001")).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_hist_tx("sz000001", "20240101", "20241231", "")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_hist_tx_qfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_a_hist_body("sz000001")).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_hist_tx("sz000001", "20240101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_hist_tx_hfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_a_hist_body("sz000001")).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_hist_tx("sz000001", "20240101", "20241231", "hfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_hist_tx_sh() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_a_hist_body("sh600000")).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_hist_tx("sh600000", "20240101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_tick_tx_js() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_tx_tick_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_tick_tx_js("sz000001", "").await;
}

#[tokio::test]
async fn test_stock_zh_a_tick_tx_js_with_date() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_tx_tick_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_tick_tx_js("sh600000", "20240321").await;
}

// =========================================================================
// zh_b.rs tests (3 methods)
// =========================================================================

#[tokio::test]
async fn test_stock_zh_b_spot() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", SINA_COUNT_TEXT).await;
    mock_any_get(&server, ".*", sample_sina_list_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_b_spot().await;
}

#[tokio::test]
async fn test_stock_zh_b_daily_qfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_b_daily("sh900901", "20240101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_b_daily_hfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_b_daily("sh900901", "20240101", "20241231", "hfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_b_daily_raw() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_b_daily("sh900901", "20240101", "20241231", "")
        .await;
}

#[tokio::test]
async fn test_stock_zh_b_daily_short_code() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_b_daily("900901", "20240101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_b_minute_sh() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_b_minute("sh900901", "5").await;
}

#[tokio::test]
async fn test_stock_zh_b_minute_1min() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_b_minute("900901", "1").await;
}

#[tokio::test]
async fn test_stock_zh_b_minute_15min() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_b_minute("900901", "15").await;
}

#[tokio::test]
async fn test_stock_zh_b_minute_30min() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_b_minute("900901", "30").await;
}

#[tokio::test]
async fn test_stock_zh_b_minute_60min() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_b_minute("900901", "60").await;
}

#[tokio::test]
async fn test_stock_zh_b_minute_sz() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_b_minute("200001", "5").await;
}

// =========================================================================
// zh_ah.rs tests (3 methods)
// =========================================================================

#[tokio::test]
async fn test_stock_zh_ah_spot() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tencent_ah_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_ah_spot().await;
}

#[tokio::test]
async fn test_stock_zh_ah_daily_raw() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tencent_kline_body("02318")).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_ah_daily("02318", "2024", "2024", "").await;
}

#[tokio::test]
async fn test_stock_zh_ah_daily_qfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tencent_kline_body("02318")).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_ah_daily("02318", "2024", "2024", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_ah_daily_hfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tencent_kline_body("02318")).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_ah_daily("02318", "2024", "2024", "hfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_ah_daily_multi_year() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tencent_kline_body("02318")).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_ah_daily("02318", "2023", "2024", "").await;
}

#[tokio::test]
async fn test_stock_zh_ah_name() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tencent_ah_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_ah_name().await;
}

// =========================================================================
// zh_index.rs tests (5 methods)
// =========================================================================

#[tokio::test]
async fn test_stock_zh_index_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_index_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_index_spot_em().await;
}

#[tokio::test]
async fn test_stock_zh_index_daily_em_sh() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_index_daily_em("000001", "20240101", "20241231")
        .await;
}

#[tokio::test]
async fn test_stock_zh_index_daily_em_sz() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_index_daily_em("399001", "20240101", "20241231")
        .await;
}

#[tokio::test]
async fn test_stock_zh_index_daily_tx() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_index_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_index_daily_tx("sh000001", "20240101", "20241231")
        .await;
}

#[tokio::test]
async fn test_stock_zh_index_daily_tx_no_prefix() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_index_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_index_daily_tx("000001", "20240101", "20241231")
        .await;
}

#[tokio::test]
async fn test_stock_zh_index_spot_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", SINA_COUNT_TEXT).await;
    mock_any_get(&server, ".*", sample_sina_index_hq_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_index_spot_sina().await;
}

#[tokio::test]
async fn test_stock_zh_index_value_csindex() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_csindex_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_index_value_csindex("H30374").await;
}

#[tokio::test]
async fn test_stock_zh_index_value_csindex_other() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_csindex_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_index_value_csindex("000300").await;
}

// =========================================================================
// zh_kcb.rs tests (3 methods)
// =========================================================================

#[tokio::test]
async fn test_stock_zh_kcb_spot() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", SINA_COUNT_TEXT).await;
    mock_any_get(&server, ".*", sample_sina_list_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_kcb_spot().await;
}

#[tokio::test]
async fn test_stock_zh_kcb_daily_qfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_kcb_daily("688399", "20240101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_kcb_daily_hfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_kcb_daily("688399", "20240101", "20241231", "hfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_kcb_daily_raw() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_kcb_daily("688399", "20240101", "20241231", "")
        .await;
}

#[tokio::test]
async fn test_stock_zh_kcb_daily_with_prefix() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_kcb_daily("sh688399", "20240101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_kcb_report_em_single_page() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kcb_report_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_kcb_report(1, 1).await;
}

#[tokio::test]
async fn test_stock_zh_kcb_report_em_multi_page() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kcb_report_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_kcb_report(1, 3).await;
}

// =========================================================================
// sina_stock.rs tests (2 methods)
// =========================================================================

#[tokio::test]
async fn test_stock_intraday_sina_sz() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", SINA_COUNT_TEXT).await;
    mock_any_get(&server, ".*", sample_sina_tick_list()).await;
    let client = mock_client(&server);
    let _ = client.stock_intraday_sina("sz000001", "20240321", 60).await;
}

#[tokio::test]
async fn test_stock_intraday_sina_sh() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", SINA_COUNT_TEXT).await;
    mock_any_get(&server, ".*", sample_sina_tick_list()).await;
    let client = mock_client(&server);
    let _ = client.stock_intraday_sina("sh600000", "20240321", 30).await;
}

#[tokio::test]
async fn test_stock_intraday_sina_limit_100() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", SINA_COUNT_TEXT).await;
    mock_any_get(&server, ".*", sample_sina_tick_list()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_intraday_sina("sz000001", "20240321", 100)
        .await;
}

#[tokio::test]
async fn test_stock_sector_spot_new_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_sina_sector_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_sector_spot("new_sina").await;
}

#[tokio::test]
async fn test_stock_sector_spot_industry() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_sina_sector_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_sector_spot("industry").await;
}

#[tokio::test]
async fn test_stock_sector_spot_concept() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_sina_sector_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_sector_spot("concept").await;
}

#[tokio::test]
async fn test_stock_sector_spot_area() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_sina_sector_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_sector_spot("area").await;
}

#[tokio::test]
async fn test_stock_sector_spot_qmx() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_sina_sector_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_sector_spot("qmx").await;
}

// =========================================================================
// hk_extra.rs tests (18 methods)
// =========================================================================

#[tokio::test]
async fn test_stock_hk_spot() {
    let server = MockServer::start().await;
    let body = serde_json::json!([{
        "symbol": "00593", "cname": "Test HK", "name": "Test HK Stock",
        "trade": 120.0, "pricechange": 1.5, "changepercent": 1.25,
        "settlement": 118.5, "open": 119.0, "high": 121.0, "low": 118.0,
        "volume": 500000, "amount": 60000000.0, "buy": 119.9, "sell": 120.1
    }]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_spot().await;
}

#[tokio::test]
async fn test_stock_hk_daily_qfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_hk_daily("00593", "20240101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_hk_daily_hfq() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_hk_daily("00593", "20240101", "20241231", "hfq")
        .await;
}

#[tokio::test]
async fn test_stock_hk_daily_raw() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_hk_daily("00593", "20240101", "20241231", "")
        .await;
}

#[tokio::test]
async fn test_stock_hk_famous_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_hk_famous_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_famous_spot().await;
}

#[tokio::test]
async fn test_stock_hk_index_daily_em_hstech() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_hsi_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_index_daily_em("HSTECH").await;
}

#[tokio::test]
async fn test_stock_hk_index_daily_em_hsi() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_hsi_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_index_daily_em("HSI").await;
}

#[tokio::test]
async fn test_stock_hk_index_daily_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_sina_hk_index_js_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_index_daily_sina("CES100").await;
}

#[tokio::test]
async fn test_stock_hk_index_daily_sina_hsi() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_sina_hk_index_js_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_index_daily_sina("HSI").await;
}

#[tokio::test]
async fn test_stock_hk_index_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_hk_index_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_index_spot_em().await;
}

#[tokio::test]
async fn test_stock_hk_index_spot_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_sina_hk_index_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_index_spot_sina().await;
}

#[tokio::test]
async fn test_stock_hk_hot_rank_em() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_emappdata_rank_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_hot_rank().await;
}

#[tokio::test]
async fn test_stock_hk_hot_rank_latest_em() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_hot_rank_latest_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_hot_rank_latest("00593").await;
}

#[tokio::test]
async fn test_stock_hk_hot_rank_detail_em() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_hot_rank_detail_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_hot_rank_detail("00593").await;
}

#[tokio::test]
async fn test_stock_hk_hot_rank_detail_realtime_em() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_hot_rank_detail_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_hot_rank_detail_realtime("00593").await;
}

#[tokio::test]
async fn test_stock_hk_valuation_baidu() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_hk_valuation(
            "00593",
            "\u{5e02}\u{76c8}\u{7387}(TTM)",
            "\u{8fd1}\u{4e00}\u{5e74}",
        )
        .await;
}

#[tokio::test]
async fn test_stock_hk_valuation_baidu_mv() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_hk_valuation(
            "00593",
            "\u{603b}\u{5e02}\u{503c}",
            "\u{8fd1}\u{4e09}\u{5e74}",
        )
        .await;
}

#[tokio::test]
async fn test_stock_hk_scale_comparison_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_hk_scale_result_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_scale_comparison("00593").await;
}

#[tokio::test]
async fn test_stock_hk_dividend_payout_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_em_dc_list_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_dividend_payout("00593").await;
}

#[tokio::test]
async fn test_stock_hk_fhpx_detail_ths() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_ths_fhpx_html()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_fhpx_detail("00593").await;
}

#[tokio::test]
async fn test_stock_hk_financial_indicator_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_em_dc_list_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_financial_indicator("00593").await;
}

#[tokio::test]
async fn test_stock_hk_gxl_lg() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_legulegu_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_gxl_lg().await;
}

#[tokio::test]
async fn test_stock_hk_indicator_eniu() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    // This method always returns an unsupported_market error.
    let result = client.stock_hk_indicator_eniu("00593").await;
    assert!(result.is_err());
}

// =========================================================================
// us_extra.rs tests (5 methods)
// =========================================================================

#[tokio::test]
async fn test_stock_us_daily() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_us_daily("AAPL", "20240101", "20241231").await;
}

#[tokio::test]
async fn test_stock_us_daily_with_prefix() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_daily("105.AAPL", "20240101", "20241231")
        .await;
}

#[tokio::test]
async fn test_stock_us_daily_nvda() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_us_daily("NVDA", "20240101", "20241231").await;
}

#[tokio::test]
async fn test_stock_us_spot() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_us_spot().await;
}

#[tokio::test]
async fn test_stock_us_famous_spot_em_tech() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_us_famous_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_famous_spot("\u{79d1}\u{6280}\u{7c7b}")
        .await;
}

#[tokio::test]
async fn test_stock_us_famous_spot_em_finance() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_us_famous_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_famous_spot("\u{91d1}\u{878d}\u{7c7b}")
        .await;
}

#[tokio::test]
async fn test_stock_us_famous_spot_em_pharma() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_us_famous_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_famous_spot("\u{533b}\u{836f}\u{98df}\u{54c1}\u{7c7b}")
        .await;
}

#[tokio::test]
async fn test_stock_us_famous_spot_em_media() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_us_famous_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_famous_spot("\u{5a92}\u{4f53}\u{7c7b}")
        .await;
}

#[tokio::test]
async fn test_stock_us_famous_spot_em_auto() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_us_famous_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_famous_spot("\u{6c7d}\u{8f66}\u{80fd}\u{6e90}\u{7c7b}")
        .await;
}

#[tokio::test]
async fn test_stock_us_famous_spot_em_retail() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_us_famous_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_famous_spot("\u{5236}\u{9020}\u{96f6}\u{552e}\u{7c7b}")
        .await;
}

#[tokio::test]
async fn test_stock_us_pink_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_us_pink_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_us_pink_spot().await;
}

#[tokio::test]
async fn test_stock_us_valuation_baidu_mv() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_valuation(
            "NVDA",
            "\u{603b}\u{5e02}\u{503c}",
            "\u{8fd1}\u{4e00}\u{5e74}",
        )
        .await;
}

#[tokio::test]
async fn test_stock_us_valuation_baidu_pe() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_valuation(
            "AAPL",
            "\u{5e02}\u{76c8}\u{7387}(TTM)",
            "\u{8fd1}\u{4e09}\u{5e74}",
        )
        .await;
}

#[tokio::test]
async fn test_stock_us_valuation_baidu_pb() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_valuation("MSFT", "\u{5e02}\u{51c0}\u{7387}", "\u{5168}\u{90e8}")
        .await;
}

// =========================================================================
// zh_comparison.rs tests (4 methods)
// =========================================================================

#[tokio::test]
async fn test_stock_zh_dupont_comparison_em_sz() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_dupont_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_dupont_comparison("SZ000895").await;
}

#[tokio::test]
async fn test_stock_zh_dupont_comparison_em_sh() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_dupont_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_dupont_comparison("SH600000").await;
}

#[tokio::test]
async fn test_stock_zh_scale_comparison_em_sz() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_scale_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_scale_comparison("SZ000895").await;
}

#[tokio::test]
async fn test_stock_zh_scale_comparison_em_sh() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_scale_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_scale_comparison("SH600000").await;
}

#[tokio::test]
async fn test_stock_zh_valuation_baidu_mv() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_valuation(
            "002044",
            "\u{603b}\u{5e02}\u{503c}",
            "\u{8fd1}\u{4e00}\u{5e74}",
        )
        .await;
}

#[tokio::test]
async fn test_stock_zh_valuation_baidu_pe() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_valuation(
            "002044",
            "\u{5e02}\u{76c8}\u{7387}(TTM)",
            "\u{8fd1}\u{4e09}\u{5e74}",
        )
        .await;
}

#[tokio::test]
async fn test_stock_zh_valuation_baidu_pb() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_valuation(
            "000001",
            "\u{5e02}\u{51c0}\u{7387}",
            "\u{8fd1}\u{4e94}\u{5e74}",
        )
        .await;
}

#[tokio::test]
async fn test_stock_zh_valuation_baidu_pcf() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_valuation("600000", "\u{5e02}\u{73b0}\u{7387}", "\u{5168}\u{90e8}")
        .await;
}

#[tokio::test]
async fn test_stock_zh_vote_baidu_stock() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_vote_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_vote("000001", "\u{80a1}\u{7968}")
        .await;
}

#[tokio::test]
async fn test_stock_zh_vote_baidu_index() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_vote_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_vote("000001", "\u{6307}\u{6570}")
        .await;
}

// =========================================================================
// eastmoney_spot.rs tests (stock_zh_a_spot_em_flex, boards, AH)
// =========================================================================

#[tokio::test]
async fn test_stock_zh_a_spot_em_flex() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_spot_em_flex(100).await;
}

#[tokio::test]
async fn test_stock_zh_a_spot_em_flex_large() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_spot_em_flex(5000).await;
}

#[tokio::test]
async fn test_stock_board_concept_name_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_board_concept_name_em(10).await;
}

#[tokio::test]
async fn test_stock_board_concept_name_em_large() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_board_concept_name_em(100).await;
}

#[tokio::test]
async fn test_stock_board_industry_name_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_board_industry_name_em(10).await;
}

#[tokio::test]
async fn test_stock_board_industry_name_em_large() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_board_industry_name_em(100).await;
}

#[tokio::test]
async fn test_stock_board_concept_cons_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_board_concept_cons("BK0001", 10).await;
}

#[tokio::test]
async fn test_stock_board_industry_cons_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_board_industry_cons("BK0001", 10).await;
}

#[tokio::test]
async fn test_stock_zh_ah_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_ah_clist_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_ah_spot_em(10).await;
}

#[tokio::test]
async fn test_stock_zh_ah_spot_em_large() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_ah_clist_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_ah_spot_em(100).await;
}

// =========================================================================
// feature/spot_em.rs tests
// =========================================================================

#[tokio::test]
async fn test_stock_zh_a_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_spot_em().await;
}

#[tokio::test]
async fn test_stock_sh_a_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_sh_a_spot().await;
}

#[tokio::test]
async fn test_stock_sz_a_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_sz_a_spot().await;
}

#[tokio::test]
async fn test_stock_bj_a_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_bj_a_spot().await;
}

#[tokio::test]
async fn test_stock_cy_a_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_cy_a_spot().await;
}

#[tokio::test]
async fn test_stock_kc_a_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_kc_a_spot().await;
}

#[tokio::test]
async fn test_feature_stock_hk_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_spot_em().await;
}

#[tokio::test]
async fn test_feature_stock_us_spot_em() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_us_spot_em().await;
}

// =========================================================================
// Additional variant tests to reach 200+ coverage
// =========================================================================

// --- a_share.rs additional variants ---

#[tokio::test]
async fn test_a_share_candles_sz_tencent() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_candles_body()).await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_candles("000001", "qfq", 120).await;
}

#[tokio::test]
async fn test_a_share_capital_flow_sz() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_capital_flow_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_capital_flow("000001", 20).await;
}

#[tokio::test]
async fn test_a_share_sector_constituents_bk0475() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_sector_constituents_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_sector_constituents("BK0475", 30).await;
}

#[tokio::test]
async fn test_a_share_sector_capital_flow_large_limit() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_capital_flow_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_sector_capital_flow("BK0001", 50).await;
}

#[tokio::test]
async fn test_a_share_announcements_large_limit() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_announcements_body()).await;
    let client = mock_client(&server);
    let _ = client.a_share_announcements("000001", 50).await;
}

// --- hk.rs additional variants ---

#[tokio::test]
async fn test_hk_candles_5digit() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_hk_candles_body()).await;
    mock_any_get(&server, ".*", sample_yahoo_chart_body()).await;
    let client = mock_client(&server);
    let _ = client.hk_candles("09988", 30).await;
}

#[tokio::test]
async fn test_hk_quote_single_digit() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_hk_quote_body()).await;
    let client = mock_client(&server);
    let _ = client.hk_quote("1").await;
}

// --- us.rs additional variants ---

#[tokio::test]
async fn test_us_candles_msft() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    mock_any_get(&server, ".*", sample_yahoo_chart_body()).await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.us_candles("MSFT", 10).await;
}

#[tokio::test]
async fn test_us_quote_tsla() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    mock_any_get(&server, ".*", sample_yahoo_chart_body()).await;
    mock_any_post(&server, ".*", sample_tushare_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.us_quote("TSLA").await;
}

// --- zh_a.rs additional variants ---

#[tokio::test]
async fn test_stock_zh_a_daily_short_date_range() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_daily("600000", "20240102", "20240103", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_minute_with_numeric_code() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_jsonp_minute_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_minute("600000", "5").await;
}

#[tokio::test]
async fn test_stock_zh_a_hist_pre_min_em_sz() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_trends2_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_hist_pre_min("000001", "09:15", "09:25")
        .await;
}

#[tokio::test]
async fn test_stock_zh_a_cdr_daily_sz() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_a_cdr_daily("000001", "20240601", "20240630")
        .await;
}

// --- zh_b.rs additional variants ---

#[tokio::test]
async fn test_stock_zh_b_daily_200xxx() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_b_daily("200001", "20240101", "20241231", "qfq")
        .await;
}

// --- zh_ah.rs additional variants ---

#[tokio::test]
async fn test_stock_zh_ah_daily_02318() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tencent_kline_body("02318")).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_ah_daily("02318", "2023", "2024", "qfq")
        .await;
}

// --- zh_index.rs additional variants ---

#[tokio::test]
async fn test_stock_zh_index_daily_em_399006() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_index_daily_em("399006", "20240101", "20241231")
        .await;
}

#[tokio::test]
async fn test_stock_zh_index_daily_tx_sz() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_index_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_index_daily_tx("sz399001", "20240101", "20241231")
        .await;
}

#[tokio::test]
async fn test_stock_zh_index_value_csindex_000300() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_csindex_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_index_value_csindex("000300").await;
}

// --- zh_kcb.rs additional variants ---

#[tokio::test]
async fn test_stock_zh_kcb_daily_688001() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_kcb_daily("688001", "20240101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_zh_kcb_report_em_page2() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kcb_report_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_kcb_report(2, 2).await;
}

// --- sina_stock.rs additional variants ---

#[tokio::test]
async fn test_stock_intraday_sina_limit_120() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", SINA_COUNT_TEXT).await;
    mock_any_get(&server, ".*", sample_sina_tick_list()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_intraday_sina("sz000001", "20240321", 120)
        .await;
}

// --- hk_extra.rs additional variants ---

#[tokio::test]
async fn test_stock_hk_daily_long_range() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_hk_daily("00593", "20200101", "20241231", "qfq")
        .await;
}

#[tokio::test]
async fn test_stock_hk_index_daily_em_hscei() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_hsi_daily_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_index_daily_em("HSCEI").await;
}

#[tokio::test]
async fn test_stock_hk_index_daily_sina_ces280() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_sina_hk_index_js_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_index_daily_sina("CES280").await;
}

#[tokio::test]
async fn test_stock_hk_hot_rank_latest_em_09988() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_hot_rank_latest_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_hot_rank_latest("09988").await;
}

#[tokio::test]
async fn test_stock_hk_hot_rank_detail_em_09988() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_hot_rank_detail_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_hot_rank_detail("09988").await;
}

#[tokio::test]
async fn test_stock_hk_hot_rank_detail_realtime_em_09988() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", sample_hot_rank_detail_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_hot_rank_detail_realtime("09988").await;
}

#[tokio::test]
async fn test_stock_hk_valuation_baidu_pb() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_hk_valuation("00593", "\u{5e02}\u{51c0}\u{7387}", "\u{5168}\u{90e8}")
        .await;
}

#[tokio::test]
async fn test_stock_hk_scale_comparison_em_09988() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_hk_scale_result_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_scale_comparison("09988").await;
}

#[tokio::test]
async fn test_stock_hk_dividend_payout_em_09988() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_em_dc_list_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_dividend_payout("09988").await;
}

#[tokio::test]
async fn test_stock_hk_fhpx_detail_ths_09988() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_ths_fhpx_html()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_fhpx_detail("09988").await;
}

#[tokio::test]
async fn test_stock_hk_financial_indicator_em_09988() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_em_dc_list_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_hk_financial_indicator("09988").await;
}

// --- us_extra.rs additional variants ---

#[tokio::test]
async fn test_stock_us_daily_goog() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_us_daily("GOOG", "20240101", "20241231").await;
}

#[tokio::test]
async fn test_stock_us_valuation_baidu_pcf() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_us_valuation(
            "GOOG",
            "\u{5e02}\u{73b0}\u{7387}",
            "\u{8fd1}\u{4e94}\u{5e74}",
        )
        .await;
}

// --- zh_comparison.rs additional variants ---

#[tokio::test]
async fn test_stock_zh_dupont_comparison_em_bj() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_dupont_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_dupont_comparison("BJ430047").await;
}

#[tokio::test]
async fn test_stock_zh_scale_comparison_em_bj() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_scale_dc_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_scale_comparison("BJ430047").await;
}

#[tokio::test]
async fn test_stock_zh_valuation_baidu_all() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_valuation_body()).await;
    let client = mock_client(&server);
    let _ = client
        .stock_zh_valuation("600000", "\u{603b}\u{5e02}\u{503c}", "\u{5168}\u{90e8}")
        .await;
}

// --- additional feature/spot_em.rs variants ---

#[tokio::test]
async fn test_stock_zh_a_spot_em_flex_small() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_zh_a_spot_em_flex(10).await;
}

#[tokio::test]
async fn test_stock_board_concept_cons_em_bk0475() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_board_concept_cons("BK0475", 20).await;
}

#[tokio::test]
async fn test_stock_board_industry_cons_em_bk0475() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_clist_spot_body()).await;
    let client = mock_client(&server);
    let _ = client.stock_board_industry_cons("BK0475", 20).await;
}

// =========================================================================
// Edge-case / negative tests
// =========================================================================

#[tokio::test]
async fn test_stock_hk_daily_invalid_adjust() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let result = client
        .stock_hk_daily("00593", "20240101", "20241231", "invalid")
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_zh_a_daily_invalid_adjust() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let result = client
        .stock_zh_a_daily("600000", "20240101", "20241231", "bad")
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_zh_b_daily_invalid_adjust() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let result = client
        .stock_zh_b_daily("sh900901", "20240101", "20241231", "bad")
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_zh_kcb_daily_invalid_adjust() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_kline_body()).await;
    let client = mock_client(&server);
    let result = client
        .stock_zh_kcb_daily("688399", "20240101", "20241231", "bad")
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_zh_a_hist_tx_invalid_adjust() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_tx_a_hist_body("sz000001")).await;
    let client = mock_client(&server);
    let result = client
        .stock_zh_a_hist_tx("sz000001", "20240101", "20241231", "bad")
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_zh_dupont_comparison_em_short_symbol() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_dupont_dc_body()).await;
    let client = mock_client(&server);
    let result = client.stock_zh_dupont_comparison("X").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_zh_scale_comparison_em_short_symbol() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_scale_dc_body()).await;
    let client = mock_client(&server);
    let result = client.stock_zh_scale_comparison("X").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_zh_vote_baidu_invalid_indicator() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_baidu_vote_body()).await;
    let client = mock_client(&server);
    let result = client.stock_zh_vote("000001", "invalid").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_sector_spot_invalid_indicator() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", &sample_sina_sector_body()).await;
    let client = mock_client(&server);
    let result = client.stock_sector_spot("invalid_indicator").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_us_famous_spot_em_invalid_category() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", sample_us_famous_body()).await;
    let client = mock_client(&server);
    let result = client.stock_us_famous_spot("\u{65e0}\u{6548}").await;
    assert!(result.is_err());
}

// =========================================================================
// Compile-only type assertions (ensures public types are accessible)
// =========================================================================

#[tokio::test]
async fn test_types_compile() {
    // Verify all public stock types are accessible
    let _q: akshare::QuoteSnapshot;
    let _c: akshare::CandlePoint;
    let _s: akshare::StockSearchResult;
    let _cf: akshare::CapitalFlowPoint;
    let _ss: akshare::SectorSnapshot;
    let _sc: akshare::SectorConstituent;
    let _bb: akshare::BillboardEntry;
    let _bs: akshare::BillboardSeatDetail;
    let _ai: akshare::AnnouncementItem;
    let _ad: akshare::AnnouncementDetail;
    let _tc: akshare::TradeCalendarItem;
    // zh_a types
    let _za: akshare::stock::zh_a::ZhASpotQuote;
    let _zd: akshare::stock::zh_a::ZhADailyCandle;
    let _zm: akshare::stock::zh_a::ZhAMinuteCandle;
    let _zn: akshare::stock::zh_a::ZhANewStock;
    let _zs: akshare::stock::zh_a::ZhAStopStock;
    let _zt: akshare::stock::zh_a::ZhATickTx;
    let _zh: akshare::stock::zh_a::ZhAHistTx;
    // zh_b types
    let _bq: akshare::stock::zh_b::ZhBSpotQuote;
    let _bd: akshare::stock::zh_b::ZhBDailyCandle;
    let _bm: akshare::stock::zh_b::ZhBMinuteCandle;
    // zh_ah types
    let _aq: akshare::stock::zh_ah::AhSpotQuote;
    let _ad2: akshare::stock::zh_ah::AhDailyCandle;
    let _an: akshare::stock::zh_ah::AhStockName;
    // zh_index types
    let _ie: akshare::stock::zh_index::IndexSpotEm;
    let _id: akshare::stock::zh_index::IndexDailyCandle;
    let _is: akshare::stock::zh_index::IndexSpotSina;
    let _cv: akshare::stock::zh_index::CsIndexValue;
    // zh_kcb types
    let _ks: akshare::stock::zh_kcb::KcbSpotQuote;
    let _kd: akshare::stock::zh_kcb::KcbDailyCandle;
    let _kr: akshare::stock::zh_kcb::KcbReport;
    // sina_stock types
    let _si: akshare::stock::sina_stock::SinaIntradayTick;
    let _ss2: akshare::stock::sina_stock::SinaSectorSpot;
    // hk_extra types
    let _hq: akshare::stock::hk_extra::HkSpotQuote;
    let _hd: akshare::stock::hk_extra::HkDailyCandle;
    let _hf: akshare::stock::hk_extra::HkFamousStock;
    let _hid: akshare::stock::hk_extra::HkIndexDailyCandle;
    let _hie: akshare::stock::hk_extra::HkIndexSpotEm;
    let _his: akshare::stock::hk_extra::HkIndexSpotSina;
    let _hr: akshare::stock::hk_extra::HkHotRank;
    let _hrd: akshare::stock::hk_extra::HkHotRankDetail;
    let _hv: akshare::stock::hk_extra::HkValuationBaidu;
    let _hg: akshare::stock::hk_extra::HkGxlLg;
    let _hf2: akshare::stock::hk_extra::HkFhpxDetailThs;
    // us_extra types
    let _ud: akshare::stock::us_extra::UsDailyCandle;
    let _us: akshare::stock::us_extra::UsSpotSina;
    let _uf: akshare::stock::us_extra::UsFamousStock;
    let _up: akshare::stock::us_extra::UsPinkStock;
    let _uv: akshare::stock::us_extra::UsValuationBaidu;
    // zh_comparison types
    let _dc: akshare::stock::zh_comparison::DupontComparison;
    let _sc2: akshare::stock::zh_comparison::ScaleComparison;
    let _bv: akshare::stock::zh_comparison::BaiduValuation;
    let _bv2: akshare::stock::zh_comparison::BaiduVote;
}
