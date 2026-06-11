#![allow(dead_code, unused_variables)]

mod common;
use common::*;
use wiremock::MockServer;

// ---------------------------------------------------------------------------
// ETF functions (src/fund/etf_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_etf_category_ths() {
    let server = MockServer::start().await;
    // THS returns JSONP: g({...})
    let body = r#"g({"data":{"data":{"0":{"FCODE":"510050","SHORTNAME":"华夏上证50ETF"}}}})"#;
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_etf_category_ths("ETF", "").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_dividend_sina() {
    let server = MockServer::start().await;
    // Sina returns JS variable: var xxx={"data":[["2024-01-01",0,0,0.05]]}
    let body = r#"var hq_str_sh510050_hfq={"data":[["2024-01-01","0","0","0.05"],["1900-01-01","0","0","0"]]}"#;
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_etf_dividend_sina("sh510050").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_fund_info_em() {
    let server = MockServer::start().await;
    // Eastmoney lsjz API: {"Data":{"LSJZList":[["2024-01-02","1.0","2.0","0","0","0","0.5","开放申购","开放赎回"]]}}
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
        .fund_etf_fund_info_em("511280", "20240101", "20240131")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_hist_em() {
    let server = MockServer::start().await;
    let body = em_kline_response(vec![
        "2024-01-02,10.00,10.50,10.80,9.90,100000,10500000.0,2.0,1.5,0.15,1.2",
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_etf_hist_em("159707", "daily", "20240101", "20240131", "qfq")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_hist_min_em_5min() {
    let server = MockServer::start().await;
    let body = em_kline_response(vec![
        "2024-01-02 09:35,10.00,10.50,10.80,9.90,100000,10500000.0,2.0,1.5,0.15,1.2",
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_etf_hist_min_em(
            "159707",
            "5",
            "2024-01-02 09:30:00",
            "2024-01-02 15:00:00",
            "qfq",
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_hist_min_em_1min() {
    let server = MockServer::start().await;
    // 1-minute uses trends2 endpoint; response has data.trends
    let body = serde_json::json!({
        "rc": 0,
        "data": {
            "code": "159707",
            "trends": [
                "2024-01-02 09:31,10.00,10.50,10.80,9.90,100000,10500000.0,0"
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_etf_hist_min_em(
            "159707",
            "1",
            "2024-01-02 09:30:00",
            "2024-01-02 15:00:00",
            "qfq",
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_scale_sse() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "result": [
            {
                "SEC_CODE": "510050",
                "SEC_NAME": "华夏上证50ETF",
                "ETF_TYPE": "股票型",
                "STAT_DATE": "2024-01-02",
                "TOT_VOL": 100.0
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_etf_scale_sse("20240102").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_scale_szse() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // This always returns an error (xlsx parsing not supported)
    let result = client.fund_etf_scale_szse().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_etf_spot_em() {
    let server = MockServer::start().await;
    let row = serde_json::json!({
        "f2": 1.50, "f3": 0.5, "f4": 0.01, "f5": 50000.0, "f6": 75000000.0,
        "f7": 1.2, "f12": "159707", "f14": "测试ETF",
        "f15": 1.52, "f16": 1.48, "f17": 1.49, "f18": 1.49,
        "f20": 1000000000.0, "f21": 500000000.0, "f38": 0.5,
        "f62": 1000000.0, "f184": 0.1, "f402": 0.01, "f441": 1.50,
        "f297": "2024-01-02"
    });
    let body = em_push2_response(vec![row]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_etf_spot_em().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_spot_ths() {
    let server = MockServer::start().await;
    // fund_etf_spot_ths delegates to fund_etf_category_ths("ETF", date)
    // THS returns JSONP
    let body = r#"g({"data":{"data":{"0":{"FCODE":"510050","SHORTNAME":"华夏上证50ETF"}}}})"#;
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_etf_spot_ths("").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// LOF functions (src/fund/lof.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_lof_hist_em() {
    let server = MockServer::start().await;
    let body = em_kline_response(vec![
        "2024-01-02,10.00,10.50,10.80,9.90,100000,10500000.0,2.0,1.5,0.15,1.2",
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_lof_hist_em("160105", "daily", "20240101", "20240131", "qfq")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_lof_hist_min_em_5min() {
    let server = MockServer::start().await;
    let body = em_kline_response(vec![
        "2024-01-02 09:35,10.00,10.50,10.80,9.90,100000,10500000.0,2.0,1.5,0.15,1.2",
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_lof_hist_min_em(
            "160105",
            "5",
            "2024-01-02 09:30:00",
            "2024-01-02 15:00:00",
            "",
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_lof_hist_min_em_1min() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "rc": 0,
        "data": {
            "code": "160105",
            "trends": [
                "2024-01-02 09:31,10.00,10.50,10.80,9.90,100000,10500000.0,0"
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client
        .fund_lof_hist_min_em(
            "160105",
            "1",
            "2024-01-02 09:30:00",
            "2024-01-02 15:00:00",
            "",
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_lof_spot_em() {
    let server = MockServer::start().await;
    let row = serde_json::json!({
        "f2": 1.50, "f3": 0.5, "f4": 0.01, "f5": 50000.0, "f6": 75000000.0,
        "f7": 1.2, "f12": "160105", "f14": "南方LOF",
        "f15": 1.52, "f16": 1.48, "f17": 1.49, "f18": 1.49,
        "f20": 1000000000.0, "f21": 500000000.0
    });
    let body = em_push2_response(vec![row]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_lof_spot_em().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Open fund functions (src/fund/open.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_open_fund_daily_em() {
    let server = MockServer::start().await;
    // Endpoint returns text: var db={"showday":["2024-01-02","2024-01-01"],"datas":["000001,华夏成长,1,1.0000,2.0000,0.9900,1.9900,0.01,0.50"]}
    let text_body = r#"var db={"showday":["2024-01-02","2024-01-01"],"datas":["000001,华夏成长,1,1.0000,2.0000,0.9900,1.9900,0.01,0.50"]}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_open_fund_daily_em().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_open_fund_info_em() {
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
        .fund_open_fund_info_em("710001", "", "", "单位净值走势")
        .await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Money fund functions (src/fund/money.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_money_fund_daily_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"ok": true});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    // This always returns Err (HTML table parsing not supported)
    let result = client.fund_money_fund_daily_em().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_money_fund_info_em() {
    let server = MockServer::start().await;
    let nav_item: Vec<serde_json::Value> = vec![
        serde_json::json!("2024-01-02"),
        serde_json::json!("1.0000"),
        serde_json::json!("2.0000"),
        serde_json::json!("0"),
        serde_json::json!("0"),
        serde_json::json!("0"),
        serde_json::json!("0.50"),
        serde_json::json!("限大额"),
        serde_json::json!("限大额"),
        serde_json::json!(""),
    ];
    let body = serde_json::json!({
        "Data": {
            "LSJZList": [nav_item]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_money_fund_info_em("000009").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_money_rank_em() {
    let server = MockServer::start().await;
    // FundMoneyRankItem expects: [0]=year_1, [1]=year_2, [2]=year_3, [3]=year_5,
    //   [7]=fund_code, [8]=fund_name, [9]=date, [10]=yield_per_10k, [11]=annualized_7d,
    //   [13]=annualized_14d, [14]=annualized_28d, [15]=month_1, [16]=month_3,
    //   [17]=month_6, [18]=ytd, [19]=since_found
    let mut row: Vec<serde_json::Value> = (0..20).map(|i| serde_json::json!("0")).collect();
    row[0] = serde_json::json!("5.00"); // year_1
    row[1] = serde_json::json!("8.00"); // year_2
    row[2] = serde_json::json!("12.00"); // year_3
    row[3] = serde_json::json!("20.00"); // year_5
    row[7] = serde_json::json!("000009"); // fund_code
    row[8] = serde_json::json!("华夏货币"); // fund_name
    row[9] = serde_json::json!("2024-01-02"); // date
    row[10] = serde_json::json!("0.50"); // yield_per_10k
    row[11] = serde_json::json!("1.80"); // annualized_7d
    row[13] = serde_json::json!("1.90"); // annualized_14d
    row[14] = serde_json::json!("2.00"); // annualized_28d
    row[15] = serde_json::json!("0.15"); // month_1
    row[16] = serde_json::json!("0.45"); // month_3
    row[17] = serde_json::json!("0.90"); // month_6
    row[18] = serde_json::json!("3.50"); // ytd
    row[19] = serde_json::json!("15.00"); // since_found
    let body = serde_json::json!({
        "Data": [row]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_money_rank_em().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Graded fund functions (src/fund/graded.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_graded_fund_daily_em() {
    let server = MockServer::start().await;
    // Returns text: var db={"showday":["2024-01-02","2024-01-01"],"datas":["150001,国泰进取,1,1.0000,2.0000,0.9900,1.9900,0.01,0.50,1.05,5.00"]}
    let text_body = r#"var db={"showday":["2024-01-02","2024-01-01"],"datas":["150001,国泰进取,1,1.0000,2.0000,0.9900,1.9900,0.01,0.50,1.05,5.00"]}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_graded_fund_daily_em().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_graded_fund_info_em() {
    let server = MockServer::start().await;
    let nav_item: Vec<serde_json::Value> = vec![
        serde_json::json!("2024-01-02"),
        serde_json::json!("1.0000"),
        serde_json::json!("2.0000"),
        serde_json::json!("0"),
        serde_json::json!("0"),
        serde_json::json!("0"),
        serde_json::json!("0.50"),
        serde_json::json!("场内交易"),
        serde_json::json!("场内交易"),
        serde_json::json!(""),
    ];
    let body = serde_json::json!({
        "Data": {
            "LSJZList": [nav_item]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_graded_fund_info_em("150232").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Announcement functions (src/fund/announcement_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_announcement_dividend_em() {
    let server = MockServer::start().await;
    // Inner array: [0]=fund_code, [1]=title, [2]=fund_name, [5]=date, [7]=report_id
    let ann_item: Vec<serde_json::Value> = vec![
        serde_json::json!("000001"),
        serde_json::json!("分红公告"),
        serde_json::json!("华夏成长"),
        serde_json::json!(""),
        serde_json::json!(""),
        serde_json::json!("2024-01-02"),
        serde_json::json!(""),
        serde_json::json!("RPT001"),
    ];
    let body = serde_json::json!({
        "Data": [ann_item]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_announcement_dividend_em("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_announcement_report_em() {
    let server = MockServer::start().await;
    let ann_item: Vec<serde_json::Value> = vec![
        serde_json::json!("000001"),
        serde_json::json!("年度报告"),
        serde_json::json!("华夏成长"),
        serde_json::json!(""),
        serde_json::json!(""),
        serde_json::json!("2024-01-02"),
        serde_json::json!(""),
        serde_json::json!("RPT002"),
    ];
    let body = serde_json::json!({
        "Data": [ann_item]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_announcement_report_em("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_announcement_personnel_em() {
    let server = MockServer::start().await;
    let ann_item: Vec<serde_json::Value> = vec![
        serde_json::json!("000001"),
        serde_json::json!("基金经理变更公告"),
        serde_json::json!("华夏成长"),
        serde_json::json!(""),
        serde_json::json!(""),
        serde_json::json!("2024-01-02"),
        serde_json::json!(""),
        serde_json::json!("RPT003"),
    ];
    let body = serde_json::json!({
        "Data": [ann_item]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_announcement_personnel_em("000001").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// AUM functions (src/fund/aum_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_aum_hist_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"ok": true});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    // This always returns Err (HTML table parsing not supported)
    let result = client.fund_aum_hist_em("2024").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_aum_trend_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "x": ["2024-01-01", "2024-02-01"],
        "y": [250000.0, 260000.0]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_aum_trend_em().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// CF / split functions (src/fund/cf_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_cf_em() {
    let server = MockServer::start().await;
    // Returns text with JS array: [["000001","基金名称","2024-01-02","拆分","2.0","其他"]];var jjcf_jjgs={}
    let text_body = r#"[["000001","华夏成长","2024-01-02","拆分","2.0","其他"]];var jjcf_jjgs={}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_cf_em("2024").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Rank functions (src/fund/rank_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_exchange_rank_em() {
    let server = MockServer::start().await;
    // Returns text: {"datas":["code,name,...23+ comma-separated fields..."]}
    // The method finds the first '{' and last '}' to extract JSON.
    // Fields in each comma-separated string: [0]=code, [1]=name, [4]=date, [5]=nav, [6]=acc_nav,
    //   [7]=week_1, [8]=month_1, [9]=month_3, [10]=month_6, [11]=year_1,
    //   [12]=year_2, [13]=year_3, [14]=ytd, [15]=since_found, [16]=found_date, [22]=fund_type
    let fields = vec![
        "000001",
        "华夏成长",
        "type1",
        "type2",
        "2024-01-02",
        "1.0000",
        "2.0000",
        "0.10",
        "0.50",
        "1.20",
        "2.50",
        "5.00",
        "8.00",
        "12.00",
        "3.50",
        "15.00",
        "2020-01-01",
        "sub1",
        "sub2",
        "sub3",
        "sub4",
        "sub5",
        "股票型",
    ];
    let row = fields.join(",");
    let text_body = format!("var rankHandler={{\"datas\":[\"{}\"]}}", row);
    mock_any_get_text(&server, ".*", text_body.as_str()).await;
    let client = mock_client(&server);
    let result = client.fund_exchange_rank_em().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// FH / dividend functions (src/fund/fhsp_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_fh_em() {
    let server = MockServer::start().await;
    // Returns text: [["000001","基金名称","2024-01-01","2024-01-02","0.05","2024-01-03","备注"]];var jjfh_jjgs={}
    let text_body = r#"[["000001","华夏成长","2024-01-01","2024-01-02","0.05","2024-01-03","备注"]];var jjfh_jjgs={}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_fh_em("2024").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_fh_rank_em() {
    let server = MockServer::start().await;
    // Returns text: [["000001","基金名称","1.50","10","2020-01-01","备注"]];var fhph_jjgs={}
    let text_body = r#"[["000001","华夏成长","1.50","10","2020-01-01","备注"]];var fhph_jjgs={}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_fh_rank_em().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Financial fund functions (src/fund/financial_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_financial_fund_daily_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "Data": {
            "List": [
                {"FCODE": "000134", "SHORTNAME": "华夏理财"}
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_financial_fund_daily_em().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_financial_fund_info_em() {
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
    let result = client.fund_financial_fund_info_em("000134").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// HK fund functions (src/fund/hk_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_hk_rank_em() {
    let server = MockServer::start().await;
    // Inner array (20+ elements):
    // [2]=hk_fund_code, [3]=fund_code, [5]=fund_name, [6]=can_buy,
    // [7]=date, [8]=nav, [9]=change_pct, [11]=week_1, [12]=month_1,
    // [13]=month_3, [14]=month_6, [15]=year_1, [16]=year_2, [17]=year_3,
    // [18]=ytd, [19]=since_found/currency
    let mut row: Vec<serde_json::Value> = (0..20).map(|i| serde_json::json!("0")).collect();
    row[2] = serde_json::json!("HK001"); // hk_fund_code
    row[3] = serde_json::json!("968001"); // fund_code
    row[5] = serde_json::json!("华夏精选"); // fund_name
    row[6] = serde_json::json!("1"); // can_buy
    row[7] = serde_json::json!("2024-01-02"); // date
    row[8] = serde_json::json!("1.5000"); // nav
    row[9] = serde_json::json!("0.50"); // change_pct
    row[11] = serde_json::json!("0.10"); // week_1
    row[12] = serde_json::json!("0.50"); // month_1
    row[13] = serde_json::json!("1.20"); // month_3
    row[14] = serde_json::json!("2.50"); // month_6
    row[15] = serde_json::json!("5.00"); // year_1
    row[16] = serde_json::json!("8.00"); // year_2
    row[17] = serde_json::json!("12.00"); // year_3
    row[18] = serde_json::json!("3.50"); // ytd
    row[19] = serde_json::json!("HKD"); // since_found / currency
    let body = serde_json::json!({
        "Data": [row]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_hk_rank_em().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_hk_fund_hist_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "Data": [
            ["2024-01-02", "1.5000", "0.50"]
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_hk_fund_hist_em("968001", "历史净值明细").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Holder structure (src/fund/hold_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_hold_structure_em() {
    let server = MockServer::start().await;
    // Returns text: {data:[["2024-01-02","100","30.5","60.5","9.0","10000000","extra"]]}
    // Fields: [0]=report_date, [1]=fund_count, [2]=inst_ratio, [3]=indiv_ratio, [4]=internal_ratio, [5]=total_shares (needs >=7 elements)
    let text_body = r#"{"data":[["2024-01-02","100","30.5","60.5","9.0","10000000","extra"]]}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_hold_structure_em().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Xueqiu / Danjuan functions (src/fund/xueqiu.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_individual_achievement_xq() {
    let server = MockServer::start().await;
    // fund_individual_achievement_xq calls fund_xueqiu_achievement which hits /djapi/fundx/base/fund/achievement/{symbol}
    let body = serde_json::json!({
        "data": {
            "annual_performance_list": [
                {
                    "period_time": "2023",
                    "self_nav": "10.50%",
                    "self_max_draw_down": "-5.20%",
                    "self_nav_rank": "100/1000"
                }
            ],
            "stage_performance_list": [
                {
                    "period_time": "近1年",
                    "self_nav": "8.30%",
                    "self_max_draw_down": "-3.10%",
                    "self_nav_rank": "50/1000"
                }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_individual_achievement_xq("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_individual_analysis_xq() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "index_data_list": [
                {
                    "index_time_period": "近1年",
                    "investment_cost_performance": 1.5,
                    "risk_control": 0.8,
                    "self_index": {
                        "volatility_rank": 0.25,
                        "sharpe_rank": 1.2,
                        "max_draw_down": -5.5
                    }
                }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_individual_analysis_xq("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_individual_basic_info_xq() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "fd_code": "000001",
            "fd_name": "华夏成长",
            "fd_full_name": "华夏成长混合型证券投资基金",
            "found_date": "2001-12-18",
            "totshare": "100.00亿",
            "keeper_name": "华夏基金",
            "manager_name": "张三",
            "trup_name": "工商银行",
            "type_desc": "混合型",
            "rating_source": "晨星",
            "rating_desc": "五星",
            "invest_orientation": "成长型",
            "invest_target": "追求长期资本增值",
            "performance_bench_mark": "沪深300指数收益率*80%+中债综合指数收益率*20%"
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_individual_basic_info_xq("000001").await;
    assert!(result.is_ok());
    let items = result.unwrap();
    assert_eq!(items.len(), 14);
}

#[tokio::test]
async fn test_fund_individual_detail_hold_xq() {
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

#[tokio::test]
async fn test_fund_individual_detail_info_xq() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "fd_code": "000001",
            "fd_name": "华夏成长",
            "buy_fee": "1.50%",
            "sell_fee": "0.50%",
            "min_buy": "100.00"
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_individual_detail_info_xq("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_individual_profit_probability_xq() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "data_list": [
                {
                    "holding_time": "持有1年",
                    "profit_ratio": "85.5%",
                    "average_income": "12.30%"
                },
                {
                    "holding_time": "持有3年",
                    "profit_ratio": "95.0%",
                    "average_income": "35.00%"
                }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_individual_profit_probability_xq("000001").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// LCX financial fund rank (src/fund/lcx_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_lcx_rank_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "Data": [
            ["000134", "华夏理财", "1.5000", "2.50"]
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_lcx_rank_em().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Position Legu functions (src/fund/position_lg.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_stock_position_lg() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // Always returns Err (requires API authentication)
    let result = client.fund_stock_position_lg().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_balance_position_lg() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // Always returns Err (requires API authentication)
    let result = client.fund_balance_position_lg().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_linghuo_position_lg() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // Always returns Err (requires API authentication)
    let result = client.fund_linghuo_position_lg().await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Manager functions (src/fund/manager.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_manager_em() {
    let server = MockServer::start().await;
    // Returns text: var returnjson= {"data":[["","","张三","","华夏基金","000001,000002","华夏成长,华夏回报","1000","15.50","","","50.5亿元"]]}
    // Fields: [2]=name, [4]=company, [5]=fund_codes, [6]=fund_names, [7]=career_days, [8]=best_return, [11]=total_scale
    let text_body = r#"var returnjson= {"data":[["","","张三","","华夏基金","000001,000002","华夏成长,华夏回报","1000","15.50","","","50.5亿元"]]}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_manager_em().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// New fund functions (src/fund/init_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_new_found_em() {
    let server = MockServer::start().await;
    // Returns text: var newfunddata={"datas":["000001,华夏成长,华夏基金,,混合型,100.00,2024-01-02,5.00,张三,认购期"]}
    let text_body = r#"var newfunddata={"datas":["000001,华夏成长,华夏基金,,混合型,100.00,2024-01-02,5.00,张三,认购期"]}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_new_found_em().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_new_found_ths() {
    let server = MockServer::start().await;
    // Returns HTML page with jsonData={"0":{"FCODE":"000001","SHORTNAME":"新基金","zzfx":1}}
    let text_body = r#"<html><script>var jsonData={"0":{"FCODE":"000001","SHORTNAME":"新基金","zzfx":1}};</script></html>"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_new_found_ths("发行中").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Portfolio functions (src/fund/portfolio_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_portfolio_change_em() {
    let server = MockServer::start().await;
    // Returns text containing JSON: {"content":"<table>html content</table>"}
    let text_body = r#"var apidata={"content":"<table><tr><td>累计买入</td></tr></table>"}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_portfolio_change_em("000001", "累计买入").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_portfolio_industry_allocation_em() {
    let server = MockServer::start().await;
    // Returns JSONP: jQuery183006997159478989867_1648016188499({"Data":{"QuarterInfos":[{"HYPZInfo":[{"INDUSTRY":"制造业","RATIO":30.5}]}]}})
    let text_body = r#"jQuery183006997159478989867_1648016188499({"Data":{"QuarterInfos":[{"HYPZInfo":[{"INDUSTRY":"制造业","RATIO":30.5}]}]}})"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_portfolio_industry_allocation_em("000001").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Rating functions (src/fund/rating.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_rating_all() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"ok": true});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    // Always returns Err (HTML/JS parsing not supported)
    let result = client.fund_rating_all().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_rating_ja() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // Always returns Err (HTML/JS parsing not supported)
    let result = client.fund_rating_ja("2024-01-01").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_rating_sh() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // Always returns Err (HTML/JS parsing not supported)
    let result = client.fund_rating_sh("2024-01-01").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Report CNINFO functions (src/fund/report_cninfo.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_report_stock_cninfo() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // Always returns Err (JS authentication required)
    let result = client.fund_report_stock_cninfo("2024-01-01").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_report_industry_allocation_cninfo() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // Always returns Err (JS authentication required)
    let result = client
        .fund_report_industry_allocation_cninfo("2024-01-01")
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_report_asset_allocation_cninfo() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // Always returns Err (JS authentication required)
    let result = client.fund_report_asset_allocation_cninfo().await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Scale functions (src/fund/scale_em.rs, src/fund/scale_szse.rs, src/fund/scale_sina.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_scale_change_em() {
    let server = MockServer::start().await;
    // Returns text: {data:[["2024-01-02","100","5000.00","3000.00","8000.00","120000.00","more"]]}
    // Fields: [0]=report_date, [1]=fund_count, [2]=subscribe, [3]=redeem, [4]=end_shares, [5]=end_net_assets
    let text_body =
        r#"{"data":[["2024-01-02","100","5000.00","3000.00","8000.00","120000.00","extra"]]}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_scale_change_em().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_scale_daily_szse() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    // Always returns Err (xlsx parsing not supported)
    let result = client
        .fund_scale_daily_szse("20240101", "20240131", "ETF")
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_scale_structured_sina() {
    let server = MockServer::start().await;
    // fund_scale_structured_sina delegates to fund_scale_close_sina
    // Sina returns JSONP: IO.XSRV2.CallbackList({...({"data":[{"symbol":"sh500001","sname":"基金金泰","dwjz":1.5,"zmjgm":100000000,"jzrq":"2024-01-02"}]})})
    let text_body = r#"IO.XSRV2.CallbackList['J2cW8KXheoWKdSHc']({"data":[{"symbol":"sh500001","sname":"基金金泰","dwjz":1.5,"zmjgm":100000000,"jzrq":"2024-01-02"}]})"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_scale_structured_sina().await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Value estimation (src/fund/value_em.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_value_estimation_em() {
    let server = MockServer::start().await;
    // Inner array (28+ elements):
    // [0]=fund_code, [19]=deviation, [20]=estimated_value, [21]=estimated_change_pct,
    // [22]=published_change_pct, [24]=published_nav, [26]=fund_name
    let mut row: Vec<serde_json::Value> = (0..28).map(|_| serde_json::json!("0")).collect();
    row[0] = serde_json::json!("000001"); // fund_code
    row[19] = serde_json::json!("0.05"); // deviation
    row[20] = serde_json::json!("1.5000"); // estimated_value
    row[21] = serde_json::json!("0.50"); // estimated_change_pct
    row[22] = serde_json::json!("0.30"); // published_change_pct
    row[24] = serde_json::json!("1.4900"); // published_nav
    row[26] = serde_json::json!("华夏成长"); // fund_name
    let body = serde_json::json!({
        "Data": {
            "gzrq": "2024-01-02",
            "list": [row]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_value_estimation_em("全部").await;
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Additional fund methods
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fund_graded() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "diff": [
                { "f12": "150001", "f14": "国泰进取", "f2": 1.05, "f3": 1.5 }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_graded(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_fund_daily_em() {
    let server = MockServer::start().await;
    // fund_etf_fund_daily_em uses push2 clist endpoint, expects data.diff with f12,f14,f2,f3
    let row = serde_json::json!({
        "f12": "510050", "f14": "华夏上证50ETF", "f2": 3.0000, "f3": 0.50
    });
    let body = em_push2_response(vec![row]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_etf_fund_daily_em(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_hist() {
    let server = MockServer::start().await;
    let body = em_kline_response(vec![
        "2024-01-02,1.50,1.52,1.55,1.48,50000,75000.0,2.0,1.3,0.02,1.0",
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_etf_hist("510050", 10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_lof_list() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "diff": [
                { "f12": "160105", "f14": "南方LOF", "f2": 1.50, "f3": 0.5 }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_lof_list(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_lof_hist() {
    let server = MockServer::start().await;
    let body = em_kline_response(vec![
        "2024-01-02,1.50,1.52,1.55,1.48,50000,75000.0,2.0,1.3,0.02,1.0",
    ]);
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_lof_hist("160105", 10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_open_end_daily() {
    let server = MockServer::start().await;
    let text_body = r#"var db={"showday":["2024-01-02","2024-01-01"],"datas":["000001,华夏成长,1,1.0000,2.0000,0.9900,1.9900,0.01,0.50"]}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_open_end_daily(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_open_end_nav() {
    let server = MockServer::start().await;
    let text_body = r#"var db={"showday":["2024-01-02","2024-01-01"],"datas":["000001,华夏成长,1,1.0000,2.0000,0.9900,1.9900,0.01,0.50"]}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_open_end_nav("000001", 10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_money_market() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "Datas": [
            { "FCODE": "000009", "SHORTNAME": "华夏货币", "PDATE": "2024-01-02",
              "DWJZ": "1.0000", "LJJZ": "2.0000", "RZDF": "0.50" }
        ],
        "ErrCode": 0
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_money_market(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_purchase_em() {
    let server = MockServer::start().await;
    let text_body = r#"var reData={"datas":["000001,华夏成长,开放申购,开放赎回,1.0000"]}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_purchase_em(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_name_em() {
    let server = MockServer::start().await;
    let text_body = r#"var r = [["000001","华夏成长","混合型"],["000002","华夏回报","混合型"]];"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_name_em().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_info_index_em() {
    let server = MockServer::start().await;
    let text_body =
        r#"jQuery({"datas":[["510300","华泰柏瑞沪深300ETF","etf","3.5000","4.0000"]]})"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client
        .fund_info_index_em("沪深指数", "被动指数型", 10)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_rating_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "datas": [
            { "FCODE": "000001", "SHORTNAME": "华夏成长", "SYL_1N": "10.50" }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_rating_em(10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_rating_zs() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_rating_zs().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_rating_tiantian() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_rating_tiantian().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_rating_jiashi() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_rating_jiashi().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_overview_em() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_overview_em(10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_info_ths() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_info_ths(10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_lof_ths() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_lof_ths(10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_fee_em() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_fee_em(10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_open_fund_rank_em() {
    let server = MockServer::start().await;
    let text_body = r#"{"datas":[["000001","华夏成长","type1","type2","2024-01-02","1.0000","2.0000","0.50"]]}"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_open_fund_rank_em("全部", 10).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_position_lg() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_position_lg("000001").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_position_hist_lg() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_position_hist_lg("000001").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_position_est_lg() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_position_est_lg().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_aum_em() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><table><tr><td>row1</td></tr></table></html>",
    )
    .await;
    let client = mock_client(&server);
    let result = client.fund_aum_em().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_xueqiu_info() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "fd_code": "000001",
            "fd_name": "华夏成长",
            "update_date": "2024-01-02",
            "nav": 1.50,
            "acc_nav": 3.20,
            "percent": 0.50
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_xueqiu_info("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_xueqiu_achievement() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "annual_performance_list": [
                { "period_time": "2023", "self_nav": "10.50%", "self_max_draw_down": "-5.20%", "self_nav_rank": "100/1000" }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_xueqiu_achievement("000001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_category_sina() {
    let server = MockServer::start().await;
    let body =
        r#"IO.XSRV2.CallbackList['da_yPT46_Ll7K6WD']([["510050","华夏上证50ETF",3.00,100,1.50]])"#;
    mock_any_get_text(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.fund_etf_category_sina("ETF基金").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_etf_hist_sina() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_etf_hist_sina("sh510050").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_scale_open_sina() {
    let server = MockServer::start().await;
    let text_body = r#"IO.XSRV2.CallbackList['J2cW8KXheoWKdSHc']({"data":[{"symbol":"sh000001","sname":"华夏成长","dwjz":1.5,"zmjgm":100000000,"jzrq":"2024-01-02"}]})"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_scale_open_sina("股票型基金").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_scale_close_sina() {
    let server = MockServer::start().await;
    let text_body = r#"IO.XSRV2.CallbackList['J2cW8KXheoWKdSHc']({"data":[{"symbol":"sh500001","sname":"基金金泰","dwjz":1.5,"zmjgm":100000000,"jzrq":"2024-01-02"}]})"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_scale_close_sina().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_scale_money_sina() {
    let server = MockServer::start().await;
    let text_body = r#"IO.XSRV2.CallbackList['J2cW8KXheoWKdSHc']({"data":[{"symbol":"sh000009","sname":"华夏货币","dwjz":1.0,"zmjgm":500000000,"jzrq":"2024-01-02"}]})"#;
    mock_any_get_text(&server, ".*", text_body).await;
    let client = mock_client(&server);
    let result = client.fund_scale_money_sina().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fund_report_cninfo() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_report_cninfo("000001").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_report_half_year_cninfo() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_report_half_year_cninfo("000001").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_report_quarter_cninfo() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.fund_report_quarter_cninfo("000001").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_portfolio_hold_em() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>mock portfolio</html>").await;
    let client = mock_client(&server);
    let result = client.fund_portfolio_hold_em("000001", "2024-01-02").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_portfolio_bond_hold_em() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>mock portfolio</html>").await;
    let client = mock_client(&server);
    let result = client.fund_portfolio_bond_hold_em("000001").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_portfolio_asset_allocation_em() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>mock portfolio</html>").await;
    let client = mock_client(&server);
    let result = client.fund_portfolio_asset_allocation_em("000001").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// QDII functions (src/fund/qdii.rs)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_qdii_a_index_jsl() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "rows": [
            { "cell": { "fund_id": "513100", "fund_nm": "纳指ETF", "price": 1.234, "fund_nav": 1.230, "discount_rt": "0.33%" } }
        ]
    });
    mock_any_get_text(&server, ".*", &body.to_string()).await;
    let client = mock_client(&server);
    let result = client.qdii_a_index_jsl("").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_qdii_e_index_jsl() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "rows": [
            { "cell": { "fund_id": "513100", "fund_nm": "纳指ETF", "price": 1.234, "fund_nav": 1.230, "discount_rt": "0.33%" } }
        ]
    });
    mock_any_get_text(&server, ".*", &body.to_string()).await;
    let client = mock_client(&server);
    let result = client.qdii_e_index_jsl("").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_qdii_e_comm_jsl() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "rows": [
            { "cell": { "fund_id": "159985", "fund_nm": "豆粕ETF", "price": 2.50, "fund_nav": 2.48, "discount_rt": "0.81%" } }
        ]
    });
    mock_any_get_text(&server, ".*", &body.to_string()).await;
    let client = mock_client(&server);
    let result = client.qdii_e_comm_jsl("").await;
    assert!(result.is_ok());
}
