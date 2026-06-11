//! Comprehensive tests for all stock board, eastmoney, xueqiu, jin10,
//! and fundamental methods in the akshare crate.
//!
//! Each test creates a wiremock MockServer, registers a plausible response,
//! creates an AkShareClient pointing at the mock, calls the method, and

#![allow(dead_code)]
//! checks the result.

#![recursion_limit = "512"]

mod common;
use common::*;

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path_regex};

// =========================================================================
// Helper response payloads
// =========================================================================

/// Board spot response (push2 stock/get format).
fn board_spot_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "f43": 1050, "f44": 1080, "f45": 1020, "f46": 1030,
            "f47": 100000, "f48": 10500000, "f170": 150, "f171": 200,
            "f168": 120, "f169": 15
        }
    })
}

/// Board resolve (clist) response mapping a name to a BK code.
fn board_resolve_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [
                {"f12": "BK0715", "f14": "绿色电力"}
            ]
        }
    })
}

/// Board kline response for daily/weekly/monthly.
fn board_kline_body() -> serde_json::Value {
    em_kline_response(vec![
        &sample_kline_str("2024-01-02"),
        &sample_kline_str("2024-01-03"),
    ])
}

/// Board trends response for 1-minute data.
fn board_trends_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "trends": [
                "2024-01-02 09:30,10.00,10.30,10.50,9.90,50000,5250000",
                "2024-01-02 09:31,10.30,10.50,10.60,10.20,60000,6300000"
            ]
        }
    })
}

/// THS board names HTML with concept links.
fn ths_concept_names_html() -> String {
    r#"<!DOCTYPE html><html><body>
<a href="/gn/detail/code/307822/">阿里巴巴概念</a>
<a href="/gn/detail/code/307833/">人工智能</a>
</body></html>"#
        .to_string()
}

/// THS board names HTML with industry links.
fn ths_industry_names_html() -> String {
    r#"<!DOCTYPE html><html><body>
<a href="/thshy/detail/code/881272/">小金属</a>
<a href="/thshy/detail/code/881273/">半导体</a>
</body></html>"#
        .to_string()
}

/// THS board info HTML with dt/dd pairs.
fn ths_board_info_html() -> String {
    r#"<!DOCTYPE html><html><body>
<div class="board-infos">
<dt>板块简介</dt><dd>测试板块描述</dd>
<dt>成分股数量</dt><dd>50</dd>
</div>
</body></html>"#
        .to_string()
}

/// THS board index JS response for kline data.
fn ths_kline_js(year: &str) -> String {
    format!(
        r#"quote_data({{"data":"{}0102,10.00,10.50,10.80,9.90,100000,10500000;{}0103,10.50,11.00,11.20,10.30,120000,13200000"}});"#,
        year, year
    )
}

/// THS board summary HTML with date patterns.
fn ths_summary_html() -> String {
    r#"<!DOCTYPE html><html><body>
<table>
<tr><td>2024-01-02</td><td>summary data 1</td></tr>
<tr><td>2024-01-03</td><td>summary data 2</td></tr>
</table>
</body></html>"#
        .to_string()
}

/// Eastmoney stock/get bid-ask response.
fn bid_ask_body() -> serde_json::Value {
    let mut data = serde_json::Map::new();
    for (k, v) in [
        ("f43", 1050.0), ("f44", 1080.0), ("f45", 1020.0), ("f46", 1030.0),
        ("f47", 100000.0), ("f48", 10500000.0),
        ("f50", 1.1), ("f51", 1155.0), ("f52", 935.0),
        ("f60", 1035.0), ("f71", 1040.0), ("f116", 100000000.0), ("f117", 50000000.0),
        ("f120", 100000000.0), ("f121", 50000000.0),
        ("f161", 50000.0), ("f162", 15.0), ("f163", 1.0), ("f164", 100.0),
        ("f168", 120.0), ("f169", 15.0), ("f170", 150.0), ("f171", 200.0),
        ("f11", 1049.0), ("f12", 100.0), ("f13", 1048.0), ("f14", 200.0),
        ("f15", 1047.0), ("f16", 300.0), ("f17", 1046.0), ("f18", 400.0),
        ("f19", 1051.0), ("f20", 100.0), ("f31", 1052.0), ("f32", 200.0),
        ("f33", 1053.0), ("f34", 300.0), ("f35", 1054.0), ("f36", 400.0),
        ("f37", 1055.0), ("f38", 500.0), ("f39", 1056.0), ("f40", 600.0),
        ("f49", 55000.0),
    ] {
        data.insert(k.to_string(), serde_json::json!(v));
    }
    data.insert("f58".to_string(), serde_json::json!("Test Stock"));
    data.insert("f167".to_string(), serde_json::json!("stock"));
    serde_json::json!({"data": data})
}

/// Eastmoney intraday SSE response.
fn intraday_sse_body() -> String {
    r#"data:{"data":{"details":["09:30:00,10.50,1000,10500000,2","09:31:00,10.51,500,5255000,1"]}}"#.to_string()
}

/// Eastmoney stock/get for individual info.
fn individual_info_body() -> serde_json::Value {
    serde_json::json!({
        "f57": "000001", "f58": "平安银行",
        "f84": 10000000000i64, "f85": 5000000000i64,
        "f127": "银行", "f116": 105000000000i64, "f117": 52500000000i64,
        "f189": "1991-04-03", "f43": 1050
    })
}

/// HK security profile datacenter response.
fn hk_security_profile_body() -> serde_json::Value {
    serde_json::json!({
        "result": {
            "data": [{
                "SECURITY_CODE": "00593",
                "SECURITY_NAME": "Test HK Stock",
                "LISTING_DATE": "2000-01-01",
                "SECURITY_TYPE": "stock",
                "TRADE_MARKET": "Main Board"
            }]
        }
    })
}

/// HK company profile datacenter response.
fn hk_company_profile_body() -> serde_json::Value {
    serde_json::json!({
        "result": {
            "data": [{
                "COMPANY_NAME": "Test HK Company",
                "CHAIRMAN": "John Doe",
                "EMPLOYEES": 10000,
                "WEBSITE": "https://example.com"
            }]
        }
    })
}

/// Fund flow kline response (capital flow format).
fn fund_flow_kline_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "klines": [
                "2024-01-02,1000000,200000,300000,400000,500000,5.0,1.0,2.0,3.0,4.0,10.50,1.5",
                "2024-01-03,800000,150000,250000,350000,450000,4.0,0.8,1.6,2.8,3.6,10.80,2.86"
            ]
        }
    })
}

/// Fund flow rank clist response.
fn fund_flow_rank_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [{
                "f12": "000001", "f14": "平安银行", "f2": 10.50, "f3": 1.5,
                "f62": 1000000, "f184": 5.0, "f66": 500000, "f69": 2.5,
                "f72": 300000, "f75": 1.5, "f78": 200000, "f81": 1.0,
                "f84": 100000, "f87": 0.5
            }]
        }
    })
}

/// Hot rank POST response (emappdata).
fn hot_rank_list_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"sc": "SZ000001", "rk": 1},
            {"sc": "SH600000", "rk": 2}
        ]
    })
}

/// Hot rank ulist GET response (push2).
fn hot_rank_ulist_body() -> serde_json::Value {
    serde_json::json!({
        "rc": 0,
        "data": {
            "total": 2,
            "diff": [
                {"f2": 10.50, "f3": 1.5, "f12": "000001", "f14": "平安银行"},
                {"f2": 8.80, "f3": -0.5, "f12": "600000", "f14": "浦发银行"}
            ]
        }
    })
}

/// Hot rank history POST response.
fn hot_rank_history_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"sc": "2024-01-01", "rk": 5},
            {"sc": "2024-01-02", "rk": 3}
        ]
    })
}

/// Hot rank profile POST response.
fn hot_rank_profile_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"newUidRate": "50%", "oldUidRate": "30%"},
            {"newUidRate": "45%", "oldUidRate": "35%"}
        ]
    })
}

/// Hot rank realtime POST response.
fn hot_rank_realtime_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"currentTime": "09:30", "currentRanking": 1},
            {"currentTime": "10:00", "currentRanking": 2}
        ]
    })
}

/// Hot keyword POST response.
fn hot_keyword_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"dateTime": "2024-01-01", "securityCode": "000001", "conceptName": "人工智能", "conceptCode": "BK0001", "hotNum": 100.0},
            {"dateTime": "2024-01-02", "securityCode": "000001", "conceptName": "区块链", "conceptCode": "BK0002", "hotNum": 80.0}
        ]
    })
}

/// Hot rank rising list POST response.
fn hot_up_list_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"sc": "SZ000001", "rk": 1, "hrc": 5},
            {"sc": "SH600000", "rk": 2, "hrc": 3}
        ]
    })
}

/// Hot rank latest POST response.
fn hot_rank_latest_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"dateTime": "2024-01-01", "rank": 1, "newFanRate": 0.5, "oldFanRate": 0.3},
            {"dateTime": "2024-01-02", "rank": 2, "newFanRate": 0.45, "oldFanRate": 0.35}
        ]
    })
}

/// Hot rank relate POST response.
fn hot_rank_relate_body() -> serde_json::Value {
    serde_json::json!({
        "data": [
            {"sc": "SZ000002", "rk": 1},
            {"sc": "SH600001", "rk": 2}
        ]
    })
}

/// Baidu hot search response.
fn baidu_hot_search_body() -> serde_json::Value {
    serde_json::json!({
        "result": [
            {"code": "000001", "name": "平安银行", "market": "ab"},
            {"code": "600000", "name": "浦发银行", "market": "ab"}
        ]
    })
}

/// HSGT kline flow response.
fn hsgt_flow_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "s2n": [
                ["2024-01-02", 1000000, 500000, 3200, 1.5],
                ["2024-01-03", 800000, 600000, 3210, 0.3]
            ]
        }
    })
}

/// Eastmoney datacenter datacenter response for misc methods.
fn misc_datacenter_body() -> serde_json::Value {
    em_datacenter_response(vec![
        serde_json::json!({
            "TRADE_DATE": "2024-01-02",
            "SZ_INDEX": 3200.0,
            "SZ_CHANGE_RATE": 1.5,
            "BLOCKTRADE_DEAL_AMT": 1000000.0,
            "PREMIUM_DEAL_AMT": 500000.0,
            "PREMIUM_RATIO": 2.0,
            "DISCOUNT_DEAL_AMT": 300000.0,
            "DISCOUNT_RATIO": 1.0
        }),
    ])
}

/// Block trade detail datacenter response.
fn block_trade_detail_body() -> serde_json::Value {
    em_datacenter_response(vec![
        serde_json::json!({
            "TRADE_DATE": "2024-01-02",
            "SECURITY_CODE": "000001",
            "SECURITY_NAME_ABBR": "平安银行",
            "CHANGE_RATE": 1.5,
            "CLOSE_PRICE": 10.50,
            "DEAL_PRICE": 10.60,
            "DEAL_VOLUME": 100000.0,
            "DEAL_AMT": 1060000.0,
            "PREMIUM_RATIO": 0.95,
            "BUYER_NAME": "Test Broker Buy",
            "SELLER_NAME": "Test Broker Sell"
        }),
    ])
}

/// Repurchase datacenter response.
fn repurchase_body() -> serde_json::Value {
    em_datacenter_response(vec![
        serde_json::json!({
            "DIM_SCODE": "000001",
            "SECURITYSHORTNAME": "平安银行",
            "NEWPRICE": 10.50,
            "REPURPRICECAP": 12.00,
            "REPURNUMLOWER": 1000000.0,
            "REPURNUMCAP": 5000000.0,
            "JEXX": 10500000.0,
            "JESX": 52500000.0,
            "DIM_TRADEDATE": "2024-01-02",
            "REPURPROGRESS": "进行中",
            "REPURNUM": 2000000.0,
            "REPURAMOUNT": 21000000.0,
            "UPDATEDATE": "2024-01-15"
        }),
    ])
}

/// Company events datacenter response.
fn company_events_body() -> serde_json::Value {
    em_datacenter_response(vec![
        serde_json::json!({
            "SECURITY_CODE": "000001",
            "SECUCODE": "000001.SZ",
            "SECURITY_NAME_ABBR": "平安银行",
            "EVENT_TYPE": "业绩预告",
            "EVENT_CONTENT": "预计2023年净利润增长",
            "TRADE_DATE": "2024-01-15"
        }),
    ])
}

/// Fund holdings response (data.eastmoney.com/dataapi/zlsj/list format).
fn fund_hold_body() -> serde_json::Value {
    serde_json::json!({
        "data": [{
            "SECURITY_CODE": "000001",
            "SECURITY_NAME_ABBR": "平安银行",
            "SCODE": "000001",
            "SNAME": "平安银行",
            "HOULD_NUM": 100,
            "HOLD_NUM": 50000000.0,
            "HOLD_MARKET_CAP": 525000000.0,
            "HOLDCHANGE": 5000000.0,
            "HOLD_RATIO_CHANGE": 0.5
        }]
    })
}

/// SSE summary response (query.sse.com.cn).
fn sse_summary_body() -> serde_json::Value {
    serde_json::json!({
        "result": [{
            "STAT_NAME": "主板",
            "STAT_NUM": 1500,
            "TRADE_AMOUNT": 100000000000i64,
            "TOTAL_MARKET_CAP": 50000000000000i64,
            "FLOAT_MARKET_CAP": 30000000000000i64
        }]
    })
}

/// Peer comparison securities response.
fn peer_comparison_body() -> serde_json::Value {
    serde_json::json!({
        "result": {
            "data": [{
                "CORRE_SECURITY_CODE": "600036",
                "CORRE_SECURITY_NAME": "招商银行",
                "PAIMING": 1,
                "ROE": 15.5
            }]
        }
    })
}

/// Spot row for eastmoney spot methods.
fn sample_spot_row_em() -> serde_json::Value {
    serde_json::json!({
        "f2": 10.50, "f3": 1.5, "f4": 0.15, "f5": 1000000, "f6": 10500000.0,
        "f7": 2.0, "f8": 1.2, "f9": 15.0, "f10": 1.1, "f12": "000001",
        "f13": "0", "f14": "平安银行", "f15": 10.80, "f16": 10.20,
        "f17": 10.30, "f18": 10.35, "f20": 100000000.0, "f21": 50000000.0,
        "f23": 1.5, "f62": 0.1
    })
}

/// AH comparison row.
fn sample_ah_row_em() -> serde_json::Value {
    serde_json::json!({
        "f193": "AH Test", "f12": "09988", "f191": "688999",
        "f2": 12000.0, "f3": 150.0, "f186": 1050.0,
        "f187": 150.0, "f189": 110.0, "f188": 10.0
    })
}

/// Xueqiu quote response.
fn xueqiu_quote_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "quote": {
                "symbol": "SH600000",
                "name": "浦发银行",
                "current": 10.50,
                "percent": 1.5,
                "chg": 0.15,
                "open": 10.30,
                "high": 10.80,
                "low": 10.20,
                "last_close": 10.35,
                "volume": 1000000,
                "amount": 10500000.0,
                "amplitude": 2.0,
                "avg_price": 10.45,
                "turnover_rate": 1.2,
                "pe_ttm": 15.0,
                "pe_lyr": 14.5,
                "pb": 1.5,
                "psr": 2.0,
                "market_capital": 105000000000i64,
                "float_market_capital": 52500000000i64,
                "total_shares": 10000000000i64,
                "float_shares": 5000000000i64,
                "limit_up": 11.39,
                "limit_down": 9.32,
                "eps": 0.70,
                "navps": 7.00,
                "dividend": 0.35,
                "dividend_yield": 3.33,
                "high52w": 12.50,
                "low52w": 8.50,
                "currency": "CNY",
                "exchange": "SH",
                "lot_size": 100,
                "time": 1704182400000i64
            }
        }
    })
}

/// Xueqiu basic info response.
fn xueqiu_basic_info_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "name": "浦发银行",
            "industry": "银行",
            "market_capital": 105000000000i64
        }
    })
}

/// Jin10 config response.
fn jin10_config_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "timescale": {
                "CNHOUR2": "2小时",
                "CNHOUR6": "6小时",
                "CNHOUR12": "12小时",
                "CNHOUR24": "24小时"
            }
        }
    })
}

/// Jin10 weibo list response.
fn jin10_weibo_body() -> serde_json::Value {
    serde_json::json!({
        "data": [{
            "symbol": "000001",
            "name": "平安银行",
            "rate": 80.5,
            "closePx": 10.50,
            "changePx": 0.15,
            "changeRate": 1.5,
            "sxcode": "000001.SZ"
        }]
    })
}

/// Sina financial report response.
fn sina_finance_report_body() -> serde_json::Value {
    serde_json::json!({
        "result": {
            "data": {
                "report_date": [
                    {"date_value": "2023-12-31"}
                ],
                "report_list": {
                    "2023-12-31": {
                        "data": [
                            {"item_title": "总资产", "item_value": "1000000"},
                            {"item_title": "总负债", "item_value": "800000"}
                        ],
                        "data_source": "annual",
                        "is_audit": "1",
                        "publish_date": "2024-03-30",
                        "rCurrency": "CNY",
                        "rType": "合并"
                    }
                }
            }
        }
    })
}

/// Sina HTML with a simple table.
fn sina_table_html() -> String {
    r#"<!DOCTYPE html><html><body>
<table>
<thead><tr><th>股票代码</th><th>股票名称</th><th>分红</th></tr></thead>
<tbody>
<tr><td>000001</td><td>平安银行</td><td>0.35</td></tr>
<tr><td>600000</td><td>浦发银行</td><td>0.30</td></tr>
</tbody>
</table>
</body></html>"#
        .to_string()
}

/// Sina shareholder HTML with date headers.
fn sina_shareholder_html() -> String {
    r#"<!DOCTYPE html><html><body>
<table>
<tbody>
<tr><td colspan="10">截至日期: 2023-12-31</td></tr>
<tr><td colspan="10">公告日期: 2024-03-30</td></tr>
<tr><td>1</td><td>Test Holder</td><td>5.00%</td><td>100000000</td></tr>
</tbody>
</table>
</body></html>"#
        .to_string()
}

/// Sina financial analysis HTML with year links and data table.
fn sina_financial_analysis_html() -> String {
    r#"<!DOCTYPE html><html><body>
<div id="con02-1">
<table>
<tr><td><a>2023</a></td><td><a>2022</a></td></tr>
</table>
</div>
<table>
<thead><tr><th>指标</th><th>2023-12-31</th><th>2023-09-30</th></tr></thead>
<tbody>
<tr><td>每股指标</td></tr>
<tr><td>基本每股收益</td><td>0.70</td><td>0.55</td></tr>
<tr><td>盈利能力</td></tr>
<tr><td>净资产收益率</td><td>15.5%</td><td>12.0%</td></tr>
</tbody>
</table>
</body></html>"#
        .to_string()
}

/// Sina institute hold HTML table.
fn sina_institute_html() -> String {
    r#"<!DOCTYPE html><html><body>
<table>
<thead><tr><th>机构名称</th><th>持股数</th><th>市值</th></tr></thead>
<tbody>
<tr><td>Test Fund</td><td>1000000</td><td>10500000</td></tr>
</tbody>
</table>
</body></html>"#
        .to_string()
}

/// Sina institute detail JSONP.
fn sina_institute_detail_jsonp() -> String {
    r#"var details=({"data":{"fund_123456":{"shares":1000000,"market_cap":10500000,"ratio":2.5}}});"#.to_string()
}

/// Sina institute recommend HTML.
fn sina_recommend_html() -> String {
    r#"<!DOCTYPE html><html><body>
<ul><li><a href="/stock/go.php/vIR_RatingNewest/index.phtml?num=40&p=1">最新投资评级</a></li></ul>
<table>
<thead><tr><th>股票代码</th><th>股票名称</th><th>评级</th></tr></thead>
<tbody>
<tr><td>000001</td><td>平安银行</td><td>买入</td></tr>
</tbody>
</table>
</body></html>"#
        .to_string()
}

/// Eastmoney securities financial analysis response.
fn em_securities_financial_body() -> serde_json::Value {
    serde_json::json!({
        "result": {
            "data": [{
                "SECUCODE": "000001.SZ",
                "SECURITY_CODE": "000001",
                "REPORT_DATE": "2023-12-31",
                "BASIC_EPS": 0.70,
                "BPS": 7.00,
                "WEIGHTAVG_ROE": 15.5
            }],
            "pages": 1
        }
    })
}

/// Eastmoney securities HK financial report response.
fn em_hk_report_body() -> serde_json::Value {
    serde_json::json!({
        "result": {
            "data": [{
                "SECUCODE": "00700.HK",
                "SECURITY_CODE": "00700",
                "REPORT_DATE": "2023-12-31",
                "STD_ITEM_NAME": "总资产",
                "AMOUNT": 1000000000000i64
            }],
            "pages": 1
        }
    })
}

/// Eastmoney HK summary response (for hk_report_em first step).
fn em_hk_summary_body() -> serde_json::Value {
    serde_json::json!({
        "result": {
            "data": [{
                "SECUCODE": "00700.HK",
                "SECURITY_CODE": "00700",
                "SECURITY_NAME_ABBR": "腾讯控股",
                "REPORT_LIST": [
                    {"REPORT_DATE": "2023-12-31 00:00:00", "REPORT_TYPE": "年报"}
                ]
            }],
            "pages": 1
        }
    })
}

/// Eastmoney US org profile (for us_resolve_secucode).
fn em_us_org_profile_body() -> serde_json::Value {
    serde_json::json!({
        "result": {
            "data": [{
                "SECUCODE": "TSLA.OQ",
                "SECURITY_CODE": "TSLA"
            }],
            "pages": 1
        }
    })
}

/// Eastmoney zygc (business composition) response.
fn em_zygc_body() -> serde_json::Value {
    serde_json::json!({
        "zygcfx": [{
            "ITEM_NAME": "利息收入",
            "MAIN_BUSINESS_INCOME": 100000000.0,
            "INCOME_RATIO": 80.0
        }]
    })
}

/// Eastmoney notice response.
fn em_notice_body() -> serde_json::Value {
    serde_json::json!({
        "data": {
            "total_hits": 1,
            "list": [{
                "art_code": "AN20240102001",
                "title": "年度报告",
                "notice_date": "2024-01-02",
                "codes": [{"stock_code": "000001", "short_name": "平安银行"}],
                "columns": [{"column_name": "年度报告"}]
            }]
        }
    })
}

/// Register GET + POST catch-all mocks on the server.
async fn mount_catch_all(server: &MockServer, get_body: serde_json::Value, post_body: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_body))
        .mount(server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(post_body))
        .mount(server)
        .await;
}

/// Register GET + POST catch-all with the same body.
async fn mount_catch_all_json(server: &MockServer, body: serde_json::Value) {
    mount_catch_all(server, body.clone(), body).await;
}

/// Register GET catch-all with raw text body.
async fn mount_catch_all_text(server: &MockServer, body: &str) {
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(server)
        .await;
}

/// Register GET text + POST json catch-all.
async fn mount_mixed(server: &MockServer, get_text: &str, post_json: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_string(get_text))
        .mount(server)
        .await;
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(post_json))
        .mount(server)
        .await;
}

// =========================================================================
// board_em.rs — Concept board
// =========================================================================

#[tokio::test]
async fn test_stock_board_concept_hist_em() {
    let server = MockServer::start().await;
    // BK code skips resolve; only kline GET is needed
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_hist_em("BK0715", "daily", "20240101", "20240103", "").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_concept_hist_em_name() {
    let server = MockServer::start().await;
    // Name requires resolve first (GET to clist), then kline GET.
    // First mock wins for each request; provide resolve-compatible response.
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(board_resolve_body()))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_hist_em("绿色电力", "weekly", "20240101", "20240103", "qfq").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_concept_hist_min_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_hist_min_em("BK0715", "5").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_concept_hist_min_em_1min() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_trends_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_hist_min_em("BK0715", "1").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_concept_spot_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_spot_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_spot_em("BK0715").await;
    let _ = result;
}

// =========================================================================
// board_em.rs — Industry board
// =========================================================================

#[tokio::test]
async fn test_stock_board_industry_hist_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_hist_em("BK1027", "daily", "20240101", "20240103", "").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_industry_hist_min_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_hist_min_em("BK1027", "15").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_industry_spot_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_spot_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_spot_em("BK1027").await;
    let _ = result;
}

// =========================================================================
// board_em.rs — Board change
// =========================================================================

#[tokio::test]
async fn test_stock_board_change_em_industry() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_em_stock_row("BK0001", "银行")])).await;
    let client = mock_client(&server);
    let result = client.stock_board_change_em("行业板块").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_change_em_concept() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_em_stock_row("BK0002", "人工智能")])).await;
    let client = mock_client(&server);
    let result = client.stock_board_change_em("概念板块").await;
    let _ = result;
}

// =========================================================================
// board_ths.rs — Concept board THS
// =========================================================================

#[tokio::test]
async fn test_stock_board_concept_name_ths() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &ths_concept_names_html()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_name_ths().await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_concept_info_ths() {
    let server = MockServer::start().await;
    // name_ths returns HTML with links; info fetches detail HTML
    // Both are GET requests, so a single catch-all text response serves both.
    mount_catch_all_text(&server, &ths_concept_names_html()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_info_ths("阿里巴巴概念").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_concept_index_ths() {
    let server = MockServer::start().await;
    // name_ths + kline fetch. Use name HTML which has a link match.
    mount_catch_all_text(&server, &ths_concept_names_html()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_index_ths("阿里巴巴概念", "20240101", "20240103").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_concept_summary_ths() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &ths_summary_html()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_summary_ths("阿里巴巴概念").await;
    let _ = result;
}

// =========================================================================
// board_ths.rs — Industry board THS
// =========================================================================

#[tokio::test]
async fn test_stock_board_industry_name_ths() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &ths_industry_names_html()).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_name_ths().await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_industry_info_ths() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &ths_industry_names_html()).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_info_ths("小金属").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_industry_index_ths() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &ths_industry_names_html()).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_index_ths("小金属", "20240101", "20240103").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_industry_summary_ths() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &ths_summary_html()).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_summary_ths("小金属").await;
    let _ = result;
}

// =========================================================================
// eastmoney_detail.rs — Bid/Ask
// =========================================================================

#[tokio::test]
async fn test_stock_bid_ask_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, bid_ask_body()).await;
    let client = mock_client(&server);
    let result = client.stock_bid_ask_em("000001").await;
    let _ = result;
}

// =========================================================================
// eastmoney_detail.rs — Intraday
// =========================================================================

#[tokio::test]
async fn test_stock_intraday_em() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &intraday_sse_body()).await;
    let client = mock_client(&server);
    let result = client.stock_intraday_em("000001").await;
    let _ = result;
}

// =========================================================================
// eastmoney_detail.rs — Individual info
// =========================================================================

#[tokio::test]
async fn test_stock_individual_info_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, individual_info_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_info_em("000001").await;
    let _ = result;
}

// =========================================================================
// eastmoney_detail.rs — HK profiles
// =========================================================================

#[tokio::test]
async fn test_stock_hk_security_profile_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, hk_security_profile_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hk_security_profile_em("00593").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hk_company_profile_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, hk_company_profile_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hk_company_profile_em("00593").await;
    let _ = result;
}

// =========================================================================
// eastmoney_fund_flow.rs — Individual fund flow
// =========================================================================

#[tokio::test]
async fn test_stock_individual_fund_flow() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_flow_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_fund_flow("000001", "sz", 30).await;
    let _ = result;
}

// =========================================================================
// eastmoney_fund_flow.rs — Fund flow rank
// =========================================================================

#[tokio::test]
async fn test_stock_individual_fund_flow_rank_today() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_flow_rank_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_fund_flow_rank("today", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_individual_fund_flow_rank_3day() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_flow_rank_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_fund_flow_rank("3day", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_individual_fund_flow_rank_5day() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_flow_rank_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_fund_flow_rank("5day", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_individual_fund_flow_rank_10day() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_flow_rank_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_fund_flow_rank("10day", 10).await;
    let _ = result;
}

// =========================================================================
// eastmoney_hot.rs — Hot rank
// =========================================================================

#[tokio::test]
async fn test_stock_hot_rank_em() {
    let server = MockServer::start().await;
    // POST for rank list, GET for ulist quotes
    mount_catch_all(&server, hot_rank_ulist_body(), hot_rank_list_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hot_rank_em(10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hot_rank_detail_em() {
    let server = MockServer::start().await;
    // Two POST calls: getHisList and getHisProfileList
    Mock::given(method("POST"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(hot_rank_history_body()))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.stock_hot_rank_detail_em("SZ000001").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hot_rank_detail_realtime_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, hot_rank_realtime_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hot_rank_detail_realtime_em("SZ000001").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hot_keyword_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, hot_keyword_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hot_keyword_em("SZ000001").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hot_up_em() {
    let server = MockServer::start().await;
    // POST for rising rank, GET for quotes
    mount_catch_all(&server, hot_rank_ulist_body(), hot_up_list_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hot_up_em(10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hot_rank_latest_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, hot_rank_latest_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hot_rank_latest_em("SZ000001").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hot_rank_relate_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, hot_rank_relate_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hot_rank_relate_em("SZ000001").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hot_search_baidu() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, baidu_hot_search_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hot_search_baidu("000001", "20240102").await;
    let _ = result;
}

// =========================================================================
// eastmoney_hsgt.rs — HSGT flow
// =========================================================================

#[tokio::test]
async fn test_stock_hsgt_kamt_flow_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, hsgt_flow_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_kamt_flow_em(30).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hsgt_north_net_flow_kamt_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, hsgt_flow_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_north_net_flow_kamt_em(30).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hsgt_south_net_flow_kamt_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, hsgt_flow_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_south_net_flow_kamt_em(30).await;
    let _ = result;
}

// =========================================================================
// eastmoney_misc.rs — Block trades
// =========================================================================

#[tokio::test]
async fn test_stock_dzjy_sctj() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_dzjy_sctj(10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_dzjy_mrmx() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, block_trade_detail_body()).await;
    let client = mock_client(&server);
    let result = client.stock_dzjy_mrmx("astock", "20240101", "20240102", 10).await;
    let _ = result;
}

// =========================================================================
// eastmoney_misc.rs — Repurchase
// =========================================================================

#[tokio::test]
async fn test_stock_repurchase_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, repurchase_body()).await;
    let client = mock_client(&server);
    let result = client.stock_repurchase_em(10).await;
    let _ = result;
}

// =========================================================================
// eastmoney_misc.rs — Company events
// =========================================================================

#[tokio::test]
async fn test_stock_gsrl_gsdt_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, company_events_body()).await;
    let client = mock_client(&server);
    let result = client.stock_gsrl_gsdt_em("20240115").await;
    let _ = result;
}

// =========================================================================
// eastmoney_misc.rs — Fund holdings
// =========================================================================

#[tokio::test]
async fn test_stock_report_fund_hold() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_hold_body()).await;
    let client = mock_client(&server);
    let result = client.stock_report_fund_hold("fund", "20210331", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_report_fund_hold_qfii() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_hold_body()).await;
    let client = mock_client(&server);
    let result = client.stock_report_fund_hold("qfii", "20210331", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_report_fund_hold_social() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_hold_body()).await;
    let client = mock_client(&server);
    let result = client.stock_report_fund_hold("social", "20210331", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_report_fund_hold_broker() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_hold_body()).await;
    let client = mock_client(&server);
    let result = client.stock_report_fund_hold("broker", "20210331", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_report_fund_hold_insurance() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_hold_body()).await;
    let client = mock_client(&server);
    let result = client.stock_report_fund_hold("insurance", "20210331", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_report_fund_hold_trust() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_hold_body()).await;
    let client = mock_client(&server);
    let result = client.stock_report_fund_hold("trust", "20210331", 10).await;
    let _ = result;
}

// =========================================================================
// eastmoney_misc.rs — Market summary
// =========================================================================

#[tokio::test]
async fn test_stock_sse_summary() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, sse_summary_body()).await;
    let client = mock_client(&server);
    let result = client.stock_sse_summary("20240115").await;
    let _ = result;
}

// =========================================================================
// eastmoney_misc.rs — Peer comparison
// =========================================================================

#[tokio::test]
async fn test_stock_zh_growth_comparison_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, peer_comparison_body()).await;
    let client = mock_client(&server);
    let result = client.stock_zh_growth_comparison_em("SZ000895").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_zh_valuation_comparison_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, peer_comparison_body()).await;
    let client = mock_client(&server);
    let result = client.stock_zh_valuation_comparison_em("SZ000895").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hk_growth_comparison_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, peer_comparison_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hk_growth_comparison_em("00700").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hk_valuation_comparison_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, peer_comparison_body()).await;
    let client = mock_client(&server);
    let result = client.stock_hk_valuation_comparison_em("00700").await;
    let _ = result;
}

// =========================================================================
// eastmoney_spot.rs — A-share spot
// =========================================================================

#[tokio::test]
async fn test_stock_zh_a_spot_em_flex() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_spot_row_em()])).await;
    let client = mock_client(&server);
    let result = client.stock_zh_a_spot_em_flex(100).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_zh_a_st_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_spot_row_em()])).await;
    let client = mock_client(&server);
    let result = client.stock_zh_a_st_em(100).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_zh_a_new_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_spot_row_em()])).await;
    let client = mock_client(&server);
    let result = client.stock_zh_a_new_em(100).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_staq_net_stop() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_spot_row_em()])).await;
    let client = mock_client(&server);
    let result = client.stock_staq_net_stop(100).await;
    let _ = result;
}

// =========================================================================
// eastmoney_spot.rs — HK/US spot
// =========================================================================

#[tokio::test]
async fn test_stock_hk_spot_em_flex() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_spot_row_em()])).await;
    let client = mock_client(&server);
    let result = client.stock_hk_spot_em_flex(100).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_us_spot_em_flex() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_spot_row_em()])).await;
    let client = mock_client(&server);
    let result = client.stock_us_spot_em_flex(100).await;
    let _ = result;
}

// =========================================================================
// eastmoney_spot.rs — Board names/cons EM
// =========================================================================

#[tokio::test]
async fn test_stock_board_concept_name_em() {
    let server = MockServer::start().await;
    let row = serde_json::json!({
        "f2": 1050.0, "f3": 1.5, "f4": 0.15, "f8": 1.2, "f12": "BK0715",
        "f14": "绿色电力", "f15": 1080.0, "f16": 1020.0, "f17": 1030.0,
        "f18": 1035.0, "f20": 100000000.0, "f21": 50000000.0,
        "f24": 5.0, "f25": 10.0, "f22": 0.5, "f33": 1.5,
        "f11": 50, "f62": 0.1, "f128": "龙头股份", "f124": 1,
        "f107": 1, "f104": 30, "f105": 20, "f136": 3.5
    });
    mount_catch_all_json(&server, em_push2_response(vec![row])).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_name_em(10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_concept_cons_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_spot_row_em()])).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_cons_em("BK0715", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_industry_name_em() {
    let server = MockServer::start().await;
    let row = serde_json::json!({
        "f2": 1050.0, "f3": 1.5, "f4": 0.15, "f8": 1.2, "f12": "BK1027",
        "f14": "小金属", "f15": 1080.0, "f16": 1020.0, "f17": 1030.0,
        "f18": 1035.0, "f20": 100000000.0, "f21": 50000000.0,
        "f24": 5.0, "f25": 10.0, "f22": 0.5, "f33": 1.5,
        "f11": 50, "f62": 0.1, "f128": "龙头股份", "f124": 1,
        "f107": 1, "f104": 30, "f105": 20, "f136": 3.5
    });
    mount_catch_all_json(&server, em_push2_response(vec![row])).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_name_em(10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_industry_cons_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_spot_row_em()])).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_cons_em("BK1027", 10).await;
    let _ = result;
}

// =========================================================================
// eastmoney_spot.rs — AH comparison
// =========================================================================

#[tokio::test]
async fn test_stock_zh_ah_spot_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [sample_ah_row_em()]
        }
    })).await;
    let client = mock_client(&server);
    let result = client.stock_zh_ah_spot_em(10).await;
    let _ = result;
}

// =========================================================================
// eastmoney_spot.rs — HSGT stocks
// =========================================================================

#[tokio::test]
async fn test_stock_hsgt_sh_hk_spot_em() {
    let server = MockServer::start().await;
    let row = serde_json::json!({
        "f12": "600000", "f13": "1", "f14": "浦发银行",
        "f19": 1050, "f1": 1035, "f2": 10500, "f4": 15,
        "f3": 150, "f152": 1, "f17": 10300, "f18": 10350,
        "f15": 10800, "f16": 10200, "f5": 100000, "f6": 1050000000
    });
    mount_catch_all_json(&server, serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [row]
        }
    })).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_sh_hk_spot_em(10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_hsgt_sz_hk_spot_em() {
    let server = MockServer::start().await;
    let row = serde_json::json!({
        "f12": "000001", "f13": "0", "f14": "平安银行",
        "f19": 1050, "f1": 1035, "f2": 10500, "f4": 15,
        "f3": 150, "f152": 1, "f17": 10300, "f18": 10350,
        "f15": 10800, "f16": 10200, "f5": 100000, "f6": 1050000000
    });
    mount_catch_all_json(&server, serde_json::json!({
        "rc": 0,
        "data": {
            "total": 1,
            "diff": [row]
        }
    })).await;
    let client = mock_client(&server);
    let result = client.stock_hsgt_sz_hk_spot_em(10).await;
    let _ = result;
}

// =========================================================================
// xueqiu.rs — Xueqiu spot
// =========================================================================

#[tokio::test]
async fn test_stock_individual_spot_xq() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, xueqiu_quote_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_spot_xq("SH600000").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_individual_basic_info_xq() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, xueqiu_basic_info_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_basic_info_xq("SH600000", "test_token").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_individual_basic_info_us_xq() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, xueqiu_basic_info_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_basic_info_us_xq("NVDA", "test_token").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_individual_basic_info_hk_xq() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, xueqiu_basic_info_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_basic_info_hk_xq("02097", "test_token").await;
    let _ = result;
}

// =========================================================================
// jin10.rs — Weibo NLP
// =========================================================================

#[tokio::test]
async fn test_stock_js_weibo_nlp_time() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, jin10_config_body()).await;
    let client = mock_client(&server);
    let result = client.stock_js_weibo_nlp_time().await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_js_weibo_report() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, jin10_weibo_body()).await;
    let client = mock_client(&server);
    let result = client.stock_js_weibo_report("CNHOUR24").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_js_weibo_report_6h() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, jin10_weibo_body()).await;
    let client = mock_client(&server);
    let result = client.stock_js_weibo_report("CNHOUR6").await;
    let _ = result;
}

// =========================================================================
// fundamental/sina.rs — Financial reports
// =========================================================================

#[tokio::test]
async fn test_stock_financial_report_sina_balance() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, sina_finance_report_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_report_sina("sh600600", "资产负债表").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_report_sina_income() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, sina_finance_report_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_report_sina("sh600600", "利润表").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_report_sina_cashflow() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, sina_finance_report_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_report_sina("sh600600", "现金流量表").await;
    let _ = result;
}

// =========================================================================
// fundamental/sina.rs — Financial abstract
// =========================================================================

#[tokio::test]
async fn test_stock_financial_abstract() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, sina_finance_report_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_abstract("600600").await;
    let _ = result;
}

// =========================================================================
// fundamental/sina.rs — Historical dividends
// =========================================================================

#[tokio::test]
async fn test_stock_history_dividend() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_table_html()).await;
    let client = mock_client(&server);
    let result = client.stock_history_dividend().await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_history_dividend_detail_dividend() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_table_html()).await;
    let client = mock_client(&server);
    let result = client.stock_history_dividend_detail("000002", "分红", None).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_history_dividend_detail_rights() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_table_html()).await;
    let client = mock_client(&server);
    let result = client.stock_history_dividend_detail("000002", "配股", None).await;
    let _ = result;
}

// =========================================================================
// fundamental/sina.rs — IPO info
// =========================================================================

#[tokio::test]
async fn test_stock_ipo_info() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_table_html()).await;
    let client = mock_client(&server);
    let result = client.stock_ipo_info("600600").await;
    let _ = result;
}

// =========================================================================
// fundamental/sina.rs — Additional stock issuance
// =========================================================================

#[tokio::test]
async fn test_stock_add_stock() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_table_html()).await;
    let client = mock_client(&server);
    let result = client.stock_add_stock("600600").await;
    let _ = result;
}

// =========================================================================
// fundamental/sina.rs — Restricted release queue
// =========================================================================

#[tokio::test]
async fn test_stock_restricted_release_queue_sina() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_table_html()).await;
    let client = mock_client(&server);
    let result = client.stock_restricted_release_queue_sina("600600").await;
    let _ = result;
}

// =========================================================================
// fundamental/sina.rs — Shareholders
// =========================================================================

#[tokio::test]
async fn test_stock_circulate_stock_holder() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_shareholder_html()).await;
    let client = mock_client(&server);
    let result = client.stock_circulate_stock_holder("600600").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_fund_stock_holder() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_shareholder_html()).await;
    let client = mock_client(&server);
    let result = client.stock_fund_stock_holder("600600").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_main_stock_holder() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_shareholder_html()).await;
    let client = mock_client(&server);
    let result = client.stock_main_stock_holder("600600").await;
    let _ = result;
}

// =========================================================================
// fundamental/sina.rs — Financial analysis indicators
// =========================================================================

#[tokio::test]
async fn test_stock_financial_analysis_indicator() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_financial_analysis_html()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_analysis_indicator("600519", "2020").await;
    let _ = result;
}

// =========================================================================
// fundamental/sina.rs — Institutional holdings
// =========================================================================

#[tokio::test]
async fn test_stock_institute_hold() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_institute_html()).await;
    let client = mock_client(&server);
    let result = client.stock_institute_hold("20201").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_institute_hold_detail() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_institute_detail_jsonp()).await;
    let client = mock_client(&server);
    let result = client.stock_institute_hold_detail("600433", "20201").await;
    let _ = result;
}

// =========================================================================
// fundamental/sina.rs — Institutional recommendations
// =========================================================================

#[tokio::test]
async fn test_stock_institute_recommend() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_recommend_html()).await;
    let client = mock_client(&server);
    let result = client.stock_institute_recommend("最新投资评级").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_institute_recommend_detail() {
    let server = MockServer::start().await;
    mount_catch_all_text(&server, &sina_recommend_html()).await;
    let client = mock_client(&server);
    let result = client.stock_institute_recommend_detail("000001").await;
    let _ = result;
}

// =========================================================================
// fundamental/eastmoney.rs — A-share financial analysis
// =========================================================================

#[tokio::test]
async fn test_stock_financial_analysis_indicator_em_report() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_securities_financial_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_analysis_indicator_em("301389.SZ", "按报告期").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_analysis_indicator_em_quarterly() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_securities_financial_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_analysis_indicator_em("301389.SZ", "按单季度").await;
    let _ = result;
}

// =========================================================================
// fundamental/eastmoney.rs — HK financial reports
// =========================================================================

#[tokio::test]
async fn test_stock_financial_hk_report_em_balance() {
    let server = MockServer::start().await;
    // First call gets summary (with REPORT_LIST), second call gets actual data.
    // Both go to securities API; catch-all returns summary-compatible response.
    mount_catch_all_json(&server, em_hk_summary_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_hk_report_em("00700", "资产负债表", "年度").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_hk_report_em_income() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_hk_summary_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_hk_report_em("00700", "利润表", "报告期").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_hk_report_em_cashflow() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_hk_summary_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_hk_report_em("00700", "现金流量表", "年度").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_hk_analysis_indicator_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_securities_financial_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_hk_analysis_indicator_em("00700", "年度").await;
    let _ = result;
}

// =========================================================================
// fundamental/eastmoney.rs — US financial reports
// =========================================================================

#[tokio::test]
async fn test_stock_financial_us_report_em_balance() {
    let server = MockServer::start().await;
    // First call resolves SECUCODE, then gets report data.
    mount_catch_all_json(&server, em_us_org_profile_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_us_report_em("TSLA", "资产负债表", "年报").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_us_report_em_income() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_us_org_profile_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_us_report_em("TSLA", "综合损益表", "单季报").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_us_analysis_indicator_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_us_org_profile_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_us_analysis_indicator_em("TSLA", "年报").await;
    let _ = result;
}

// =========================================================================
// fundamental/eastmoney.rs — IPO registration
// =========================================================================

#[tokio::test]
async fn test_stock_register_em_all() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_register_em("全部").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_register_em_kcb() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_register_em("科创板").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_register_em_cyb() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_register_em("创业板").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_register_em_bse() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_register_em("北交所").await;
    let _ = result;
}

// =========================================================================
// fundamental/eastmoney.rs — Restricted releases
// =========================================================================

#[tokio::test]
async fn test_stock_restricted_release_summary_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_restricted_release_summary_em("全部股票", "20240101", "20240131").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_restricted_release_detail_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_restricted_release_detail_em("20240101", "20240131").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_restricted_release_queue_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_restricted_release_queue_em("600000").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_restricted_release_stockholder_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_restricted_release_stockholder_em("600000", "20240115").await;
    let _ = result;
}

// =========================================================================
// fundamental/eastmoney.rs — IPO declare / review / tutor
// =========================================================================

#[tokio::test]
async fn test_stock_ipo_declare_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_ipo_declare_em().await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_ipo_review_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_ipo_review_em().await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_ipo_tutor_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_ipo_tutor_em().await;
    let _ = result;
}

// =========================================================================
// fundamental/eastmoney.rs — Profit forecast
// =========================================================================

#[tokio::test]
async fn test_stock_profit_forecast_em_all() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_profit_forecast_em("").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_profit_forecast_em_industry() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_profit_forecast_em("船舶制造").await;
    let _ = result;
}

// =========================================================================
// fundamental/eastmoney.rs — Share capital structure
// =========================================================================

#[tokio::test]
async fn test_stock_zh_a_gbjg_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_securities_financial_body()).await;
    let client = mock_client(&server);
    let result = client.stock_zh_a_gbjg_em("603392.SH").await;
    let _ = result;
}

// =========================================================================
// fundamental/eastmoney.rs — Main business composition
// =========================================================================

#[tokio::test]
async fn test_stock_zygc_em() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_zygc_body()).await;
    let client = mock_client(&server);
    let result = client.stock_zygc_em("SH688041").await;
    let _ = result;
}

// =========================================================================
// fundamental/eastmoney.rs — Stock notices
// =========================================================================

#[tokio::test]
async fn test_stock_notice_report() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_notice_body()).await;
    let client = mock_client(&server);
    let result = client.stock_notice_report("全部", "20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_notice_report_financial() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_notice_body()).await;
    let client = mock_client(&server);
    let result = client.stock_notice_report("财务报告", "20240102").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_individual_notice_report() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_notice_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_notice_report("300237", "全部", Some("20240101"), Some("20240102")).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_individual_notice_report_no_dates() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_notice_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_notice_report("300237", "全部", None, None).await;
    let _ = result;
}

// =========================================================================
// Edge cases and parameter variations
// =========================================================================

#[tokio::test]
async fn test_stock_board_concept_hist_em_weekly() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_hist_em("BK0715", "weekly", "20240101", "20240103", "qfq").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_concept_hist_em_monthly() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_hist_em("BK0715", "monthly", "20240101", "20240103", "hfq").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_industry_hist_em_weekly() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_hist_em("BK1027", "weekly", "20240101", "20240103", "qfq").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_concept_hist_min_em_30min() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_hist_min_em("BK0715", "30").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_industry_hist_min_em_60min() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_industry_hist_min_em("BK1027", "60").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_individual_fund_flow_sh() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_flow_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_fund_flow("600000", "sh", 60).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_dzjy_mrmx_bstock() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, block_trade_detail_body()).await;
    let client = mock_client(&server);
    let result = client.stock_dzjy_mrmx("bstock", "20240101", "20240102", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_dzjy_mrmx_fund() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, block_trade_detail_body()).await;
    let client = mock_client(&server);
    let result = client.stock_dzjy_mrmx("fund", "20240101", "20240102", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_dzjy_mrmx_bond() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, block_trade_detail_body()).await;
    let client = mock_client(&server);
    let result = client.stock_dzjy_mrmx("bond", "20240101", "20240102", 10).await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_register_em_hzb() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_register_em("沪主板").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_register_em_szb() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_register_em("深主板").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_restricted_release_summary_em_sh_a() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_restricted_release_summary_em("沪市A股", "20240101", "20240131").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_restricted_release_summary_em_kcb() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_restricted_release_summary_em("科创板", "20240101", "20240131").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_restricted_release_summary_em_cyb() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_restricted_release_summary_em("创业板", "20240101", "20240131").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_restricted_release_summary_em_bse() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_restricted_release_summary_em("京市A股", "20240101", "20240131").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_us_analysis_indicator_em_quarterly() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_us_org_profile_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_us_analysis_indicator_em("TSLA", "单季报").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_us_analysis_indicator_em_cumulative() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_us_org_profile_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_us_analysis_indicator_em("TSLA", "累计季报").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_financial_hk_analysis_indicator_em_period() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_securities_financial_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_hk_analysis_indicator_em("00700", "报告期").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_board_change_em_custom_fs() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, em_push2_response(vec![sample_em_stock_row("BK0001", "Test")])).await;
    let client = mock_client(&server);
    // Custom fs string (not "行业板块" or "概念板块")
    let result = client.stock_board_change_em("m:90 t:3 f:!50").await;
    let _ = result;
}

#[tokio::test]
async fn test_stock_report_fund_hold_invalid_type() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_hold_body()).await;
    let client = mock_client(&server);
    let result = client.stock_report_fund_hold("invalid_type", "20210331", 10).await;
    // This should return an error due to unsupported holder type
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_dzjy_mrmx_invalid_asset() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, block_trade_detail_body()).await;
    let client = mock_client(&server);
    let result = client.stock_dzjy_mrmx("invalid", "20240101", "20240102", 10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_board_concept_hist_em_invalid_period() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_hist_em("BK0715", "invalid", "20240101", "20240103", "").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_board_concept_hist_em_invalid_adjust() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, board_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_board_concept_hist_em("BK0715", "daily", "20240101", "20240103", "invalid").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_individual_fund_flow_invalid_market() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_flow_kline_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_fund_flow("000001", "invalid", 30).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_individual_fund_flow_rank_invalid_indicator() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, fund_flow_rank_body()).await;
    let client = mock_client(&server);
    let result = client.stock_individual_fund_flow_rank("invalid", 10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_financial_report_sina_invalid_symbol() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, sina_finance_report_body()).await;
    let client = mock_client(&server);
    let result = client.stock_financial_report_sina("sh600600", "无效报表").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_zh_growth_comparison_em_short_symbol() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, peer_comparison_body()).await;
    let client = mock_client(&server);
    // Symbol too short (should be 8+ chars like "SZ000895")
    let result = client.stock_zh_growth_comparison_em("0001").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_register_em_invalid_market() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_register_em("invalid_market").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stock_restricted_release_summary_em_invalid_market() {
    let server = MockServer::start().await;
    mount_catch_all_json(&server, misc_datacenter_body()).await;
    let client = mock_client(&server);
    let result = client.stock_restricted_release_summary_em("invalid", "20240101", "20240131").await;
    assert!(result.is_err());
}

// =========================================================================
// Verify mock_client creates a client with mock_uri set
// =========================================================================

#[tokio::test]
async fn test_mock_client_has_mock_uri() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    assert!(client.mock_uri.is_some());
    assert_eq!(client.mock_uri.unwrap(), server.uri());
}
