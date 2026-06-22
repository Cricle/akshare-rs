mod common;
use common::*;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ===========================================================================
// exchange.rs — get_* wrapper methods
// ===========================================================================

#[tokio::test]
async fn test_get_cffex_daily() {
    let server = MockServer::start().await;
    // CFFEX daily uses pipe-delimited CSV
    let csv = "合约,开盘价,最高价,最低价,成交量,成交额,持仓量,收盘价,结算价,前结算价\n\
               IF2403,3200.0,3250.0,3180.0,50000,1600000000.0,80000,3220.0,3215.0,3210.0\n\
               小计,100000,,,,,,,,\n";
    mock_any_get_text(&server, ".*", csv).await;
    let client = mock_client(&server);
    let result = client.get_cffex_daily("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_cffex_rank_table() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "header\nIF2403,1,TraderA,5000,100,TraderB,4800,80,TraderC,4600,70\n",
    )
    .await;
    let client = mock_client(&server);
    let result = client.get_cffex_rank_table("20240315", "IF").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_czce_daily() {
    let server = MockServer::start().await;
    let txt = "品种月份,昨结算,今开盘,最高价,最低价,今收盘,今结算,涨跌,成交量,持仓量\n\
               SR403,6500,6510,6550,6490,6520,6515,20,120000,200000\n\
               小计,100000,,,,,,,,,\n";
    mock_any_get_text(&server, ".*", txt).await;
    let client = mock_client(&server);
    let result = client.get_czce_daily("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_dce_daily() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "variety": "豆一",
                "contractId": "a2405",
                "open": "4800.0",
                "high": "4850.0",
                "low": "4780.0",
                "close": "4820.0",
                "volumn": "50000",
                "openInterest": "80000",
                "turnover": "2400000.0",
                "clearPrice": "4815.0",
                "lastClear": "4800.0"
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_dce_daily("20240315", None).await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_dce_rank_table() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", serde_json::json!({"data": []})).await;
    let client = mock_client(&server);
    let result = client.get_dce_rank_table("20240315", "a").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_gfex_daily() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "variety": "工业硅",
                "varietyOrder": "SI",
                "delivMonth": "2405",
                "open": "13000.0",
                "high": "13200.0",
                "low": "12900.0",
                "close": "13100.0",
                "volumn": "30000",
                "openInterest": "50000",
                "turnover": "393000000.0",
                "clearPrice": "13050.0",
                "lastClear": "13000.0"
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_gfex_daily("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_ine_daily() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_curinstrument": [
            {
                "DELIVERYMONTH": "2405",
                "PRODUCTNAME": "原油",
                "PRODUCTID": "sc",
                "PRODUCTGROUPID": "SC",
                "OPENPRICE": "520.0",
                "HIGHESTPRICE": "530.0",
                "LOWESTPRICE": "515.0",
                "CLOSEPRICE": "525.0",
                "VOLUME": "100000",
                "OPENINTEREST": "60000",
                "TURNOVER": "5200000000.0",
                "SETTLEMENTPRICE": "523.0",
                "PRESETTLEMENTPRICE": "520.0"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_ine_daily("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_shfe_daily() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_curinstrument": [
            {
                "DELIVERYMONTH": "2405",
                "PRODUCTNAME": "螺纹钢",
                "PRODUCTID": "rb",
                "PRODUCTGROUPID": "RB",
                "OPENPRICE": "3800.0",
                "HIGHESTPRICE": "3850.0",
                "LOWESTPRICE": "3780.0",
                "CLOSEPRICE": "3820.0",
                "VOLUME": "200000",
                "OPENINTEREST": "150000",
                "TURNOVER": "7600000000.0",
                "SETTLEMENTPRICE": "3815.0",
                "PRESETTLEMENTPRICE": "3800.0"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_shfe_daily("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_shfe_rank_table() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_cursor": [
            {
                "INSTRUMENTID": "rb2405",
                "RANK": "1",
                "PARTICIPANTABBR1": "CompanyA",
                "CJ1": "5000",
                "CJ1_CHG": "100",
                "PARTICIPANTABBR2": "CompanyB",
                "CJ2": "4800",
                "CJ2_CHG": "-50",
                "PARTICIPANTABBR3": "CompanyC",
                "CJ3": "4600",
                "CJ3_CHG": "80"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_shfe_rank_table("20240315", "rb").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_rank_table_czce() {
    let server = MockServer::start().await;
    // CZCE uses XLS binary; the method returns Ok(vec![]) after download
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.get_rank_table_czce("20240315", None).await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_roll_yield_bar() {
    let server = MockServer::start().await;
    // Roll yield bar delegates to get_futures_daily across all exchanges
    // Return empty data for each exchange
    let shfe_body = serde_json::json!({"o_curinstrument": []});
    mock_any_get(&server, ".*", shfe_body).await;
    let dce_body = serde_json::json!({"data": []});
    mock_any_post(&server, ".*", dce_body).await;
    let client = mock_client(&server);
    let result = client
        .get_roll_yield_bar("20240315", None, None, None)
        .await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_rank_sum() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"o_cursor": []});
    mock_any_get(&server, ".*", body).await;
    let post_body = serde_json::json!({"data": []});
    mock_any_post(&server, ".*", post_body).await;
    let client = mock_client(&server);
    let result = client.get_rank_sum("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_rank_sum_daily() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"o_cursor": []});
    mock_any_get(&server, ".*", body).await;
    let post_body = serde_json::json!({"data": []});
    mock_any_post(&server, ".*", post_body).await;
    let client = mock_client(&server);
    let result = client.get_rank_sum_daily("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_receipt() {
    let server = MockServer::start().await;
    let dce_body = serde_json::json!({
        "data": {"entityList": [
            {"variety": "豆一小计", "wbillQty": "1000", "diff": "10", "varietyOrder": "a", "whAbbr": "WH1"}
        ]}
    });
    mock_any_post(&server, ".*", dce_body).await;
    let shfe_body = serde_json::json!({"o_cursor": []});
    mock_any_get(&server, ".*", shfe_body).await;
    let client = mock_client(&server);
    let result = client.get_receipt("20240315", None).await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_token() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"token": "fake_token_123"});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_token("test@example.com", "password").await;
    result.unwrap();
}

// ===========================================================================
// daily_bar.rs — futures_daily_* methods
// ===========================================================================

#[tokio::test]
async fn test_futures_daily_cffex() {
    let server = MockServer::start().await;
    let csv = "合约|开盘价|最高|最低|成交量|成交额|持仓量|收盘价|结算价|前结算\n\
               IF2403|3200.0|3250.0|3180.0|50000|1600000000.0|80000|3220.0|3215.0|3210.0\n";
    mock_any_get_text(&server, ".*", csv).await;
    let client = mock_client(&server);
    let result = client.futures_daily_cffex("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_daily_shfe() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_curinstrument": [
            {
                "DELIVERYMONTH": "2405",
                "PRODUCTNAME": "铜",
                "PRODUCTID": "cu",
                "PRODUCTGROUPID": "CU",
                "OPENPRICE": "70000.0",
                "HIGHESTPRICE": "71000.0",
                "LOWESTPRICE": "69500.0",
                "CLOSEPRICE": "70500.0",
                "VOLUME": "80000",
                "OPENINTEREST": "120000",
                "TURNOVER": "5600000000.0",
                "SETTLEMENTPRICE": "70300.0",
                "PRESETTLEMENTPRICE": "70000.0"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_daily_shfe("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_daily_ine() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_curinstrument": [
            {
                "DELIVERYMONTH": "2405",
                "PRODUCTNAME": "原油",
                "PRODUCTID": "sc2405",
                "OPENPRICE": "520.0",
                "HIGHESTPRICE": "530.0",
                "LOWESTPRICE": "515.0",
                "CLOSEPRICE": "525.0",
                "VOLUME": "100000",
                "OPENINTEREST": "60000",
                "TURNOVER": "5200000000.0",
                "SETTLEMENTPRICE": "523.0",
                "PRESETTLEMENTPRICE": "520.0"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_daily_ine("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_daily_dce() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "variety": "豆一",
                "contractId": "a2405",
                "open": "4800.0",
                "high": "4850.0",
                "low": "4780.0",
                "close": "4820.0",
                "volumn": "50000",
                "openInterest": "80000",
                "turnover": "2400000.0",
                "clearPrice": "4815.0",
                "lastClear": "4800.0"
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_daily_dce("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_daily_czce() {
    let server = MockServer::start().await;
    let txt = "品种月份|昨结算|今开盘|最高价|最低价|今收盘|今结算|涨跌|成交量|持仓量\n\
               SR403|6500|6510|6550|6490|6520|6515|20|120000|200000\n\
               小计|100000|0|0|0|0|0|0|0|0\n";
    mock_any_get_text(&server, ".*", txt).await;
    let client = mock_client(&server);
    let result = client.futures_daily_czce("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_daily_gfex() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "variety": "工业硅",
                "varietyOrder": "SI",
                "delivMonth": "2405",
                "open": "13000.0",
                "high": "13200.0",
                "low": "12900.0",
                "close": "13100.0",
                "volumn": "30000",
                "openInterest": "50000",
                "turnover": "393000000.0",
                "clearPrice": "13050.0",
                "lastClear": "13000.0"
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_daily_gfex("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_futures_daily_cffex() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    let client = mock_client(&server);
    let result = client.get_futures_daily("20240315", "CFFEX").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_futures_daily_shfe() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"o_curinstrument": []});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_futures_daily("20240315", "SHFE").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_futures_daily_invalid_market() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.get_futures_daily("20240315", "INVALID").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_futures_hist_daily_cffex() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "").await;
    let client = mock_client(&server);
    let result = client.futures_hist_daily_cffex("20240315").await;
    result.unwrap();
}

// ===========================================================================
// cot.rs — position rank methods
// ===========================================================================

#[tokio::test]
async fn test_futures_shfe_position_rank() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_cursor": [
            {
                "INSTRUMENTID": "rb2405",
                "RANK": "1",
                "PARTICIPANTABBR1": "永安期货",
                "CJ1": "5000",
                "CJ1_CHG": "100",
                "PARTICIPANTABBR2": "中信期货",
                "CJ2": "4800",
                "CJ2_CHG": "-50",
                "PARTICIPANTABBR3": "国泰君安",
                "CJ3": "4600",
                "CJ3_CHG": "80"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_shfe_position_rank("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_czce_position_rank() {
    let server = MockServer::start().await;
    // CZCE returns XLS binary
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_czce_position_rank("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_cffex_position_rank() {
    let server = MockServer::start().await;
    // CFFEX returns CSV per variety
    mock_any_get_text(&server, ".*", "").await;
    let client = mock_client(&server);
    let result = client.futures_cffex_position_rank("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_dce_position_rank() {
    let server = MockServer::start().await;
    // DCE returns ZIP binary
    mock_any_post(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_dce_position_rank("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_gfex_position_rank() {
    let server = MockServer::start().await;
    // GFEX: variety list + contract list + rank data per data_type
    let body = serde_json::json!({"data": [{"varietyId": "si"}]});
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_gfex_position_rank("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_dce_position_rank_other() {
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
    result.unwrap();
}

#[tokio::test]
async fn test_futures_hold_pos_sina() {
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
    result.unwrap();
}

// ===========================================================================
// delivery.rs — futures_delivery_* and futures_to_spot_* methods
// ===========================================================================

#[tokio::test]
async fn test_futures_to_spot_shfe() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "ExchangeDelivery": [
            [1, "2024-03-15", 1000, 500, 200, "rb2405", 3800.0]
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_to_spot_shfe("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_delivery_shfe() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_curdelivery": [
            ["螺纹钢", "RB", "test", "10000", "5.0", "50000", "10.0"]
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_delivery_shfe("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_delivery_dce() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_delivery_dce("202403").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_to_spot_dce() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_to_spot_dce("202403").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_delivery_match_czce() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_delivery_match_czce("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_delivery_czce() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_delivery_czce("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_delivery_match_dce() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_delivery_match_dce("202403").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_to_spot_czce() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_to_spot_czce("20240315").await;
    result.unwrap();
}

// ===========================================================================
// receipt.rs — get_dce_receipt, get_shfe_receipt
// ===========================================================================

#[tokio::test]
async fn test_get_dce_receipt() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "entityList": [
                {"variety": "豆一小计", "wbillQty": "1000", "diff": "10", "varietyOrder": "a", "whAbbr": "WH1"}
            ]
        }
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_dce_receipt("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_shfe_receipt() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_cursor": [
            {
                "VARNAME": "铜$CU",
                "REGNAME": "上期所$SHFE",
                "WHABBRNAME": "国储$NRES",
                "WRTWGHTS": "5000.0"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_shfe_receipt("20240315").await;
    result.unwrap();
}

// ===========================================================================
// warehouse.rs — futures_warehouse_receipt_* methods
// ===========================================================================

#[tokio::test]
async fn test_futures_warehouse_receipt_czce() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_warehouse_receipt_czce("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_warehouse_receipt_dce() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "entityList": [
                {"variety": "豆粕", "varietyOrder": "M", "whAbbr": "WH1", "lastWbillQty": "5000", "wbillQty": "5100", "diff": "100"}
            ]
        }
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_warehouse_receipt_dce("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_shfe_warehouse_receipt() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_cursor": [
            {
                "VARNAME": "铜$CU",
                "REGNAME": "上期所$SHFE",
                "WHABBRNAME": "国储$NRES",
                "WRTWGHTS": "5000.0"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_shfe_warehouse_receipt("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_gfex_warehouse_receipt() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {"varietyOrder": "SI", "variety": "工业硅", "whAbbr": "WH1", "lastWbillQty": "1000", "wbillQty": "1100", "regWbillQty": "100"}
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_gfex_warehouse_receipt("20240315").await;
    result.unwrap();
}

// ===========================================================================
// basis.rs — futures_spot_price_* methods
// ===========================================================================

#[tokio::test]
async fn test_futures_spot_price() {
    let server = MockServer::start().await;
    mock_any_get_text(
        &server,
        ".*",
        "<html><table>100ppi spot data</table></html>",
    )
    .await;
    let client = mock_client(&server);
    let result = client.futures_spot_price("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_spot_price_daily() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client
        .futures_spot_price_daily("20240301", "20240315")
        .await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_spot_price_previous() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>100ppi sf2 data</html>").await;
    let client = mock_client(&server);
    let result = client.futures_spot_price_previous("20240315").await;
    result.unwrap();
}

// ===========================================================================
// roll_yield.rs — get_roll_yield, futures_roll_yield_bar
// ===========================================================================

#[tokio::test]
async fn test_get_roll_yield() {
    let server = MockServer::start().await;
    // Roll yield needs daily data from all exchanges
    let shfe_body = serde_json::json!({
        "o_curinstrument": [
            {
                "DELIVERYMONTH": "2405",
                "PRODUCTNAME": "螺纹钢",
                "PRODUCTID": "rb2405",
                "PRODUCTGROUPID": "RB",
                "OPENPRICE": "3800.0",
                "HIGHESTPRICE": "3850.0",
                "LOWESTPRICE": "3780.0",
                "CLOSEPRICE": "3820.0",
                "VOLUME": "200000",
                "OPENINTEREST": "150000",
                "TURNOVER": "7600000000.0",
                "SETTLEMENTPRICE": "3815.0",
                "PRESETTLEMENTPRICE": "3800.0"
            },
            {
                "DELIVERYMONTH": "2410",
                "PRODUCTNAME": "螺纹钢",
                "PRODUCTID": "rb2410",
                "PRODUCTGROUPID": "RB",
                "OPENPRICE": "3750.0",
                "HIGHESTPRICE": "3800.0",
                "LOWESTPRICE": "3720.0",
                "CLOSEPRICE": "3770.0",
                "VOLUME": "100000",
                "OPENINTEREST": "80000",
                "TURNOVER": "3770000000.0",
                "SETTLEMENTPRICE": "3765.0",
                "PRESETTLEMENTPRICE": "3750.0"
            }
        ]
    });
    mock_any_get(&server, ".*", shfe_body).await;
    let dce_body = serde_json::json!({"data": []});
    mock_any_post(&server, ".*", dce_body).await;
    let client = mock_client(&server);
    let result = client.get_roll_yield("20240315", "RB", None, None).await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_roll_yield_bar() {
    let server = MockServer::start().await;
    let shfe_body = serde_json::json!({
        "o_curinstrument": [
            {
                "DELIVERYMONTH": "2405",
                "PRODUCTNAME": "螺纹钢",
                "PRODUCTID": "rb2405",
                "PRODUCTGROUPID": "RB",
                "OPENPRICE": "3800.0",
                "HIGHESTPRICE": "3850.0",
                "LOWESTPRICE": "3780.0",
                "CLOSEPRICE": "3820.0",
                "VOLUME": "200000",
                "OPENINTEREST": "150000",
                "TURNOVER": "7600000000.0",
                "SETTLEMENTPRICE": "3815.0",
                "PRESETTLEMENTPRICE": "3800.0"
            },
            {
                "DELIVERYMONTH": "2410",
                "PRODUCTNAME": "螺纹钢",
                "PRODUCTID": "rb2410",
                "PRODUCTGROUPID": "RB",
                "OPENPRICE": "3750.0",
                "HIGHESTPRICE": "3800.0",
                "LOWESTPRICE": "3720.0",
                "CLOSEPRICE": "3770.0",
                "VOLUME": "100000",
                "OPENINTEREST": "80000",
                "TURNOVER": "3770000000.0",
                "SETTLEMENTPRICE": "3765.0",
                "PRESETTLEMENTPRICE": "3750.0"
            }
        ]
    });
    mock_any_get(&server, ".*", shfe_body).await;
    let dce_body = serde_json::json!({"data": []});
    mock_any_post(&server, ".*", dce_body).await;
    let client = mock_client(&server);
    let result = client.futures_roll_yield_bar("20240315").await;
    result.unwrap();
}

// ===========================================================================
// hq_sina.rs — foreign commodity methods (Sina text format)
// ===========================================================================

#[tokio::test]
async fn test_futures_foreign_commodity_subscribe_exchange_symbol() {
    // Sync method — no HTTP calls
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let symbols = client.futures_foreign_commodity_subscribe_exchange_symbol();
    assert!(!symbols.is_empty());
}

#[tokio::test]
async fn test_futures_hq_subscribe_exchange_symbol() {
    // Sync method — no HTTP calls
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let symbols = client.futures_hq_subscribe_exchange_symbol();
    assert!(!symbols.is_empty());
}

#[tokio::test]
async fn test_futures_foreign_commodity_subscribe_codes() {
    // Sync method — no HTTP calls
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let codes = client.futures_foreign_commodity_subscribe_codes();
    assert!(!codes.is_empty());
}

#[tokio::test]
async fn test_futures_foreign_commodity_realtime() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"var hf_CL="69.50,0.0,69.80,69.90,69.20,2024-01-02 10:00,68.50,69.00,1000.0,2024-01-02,70.00,,,,69.50";"#
        ))
        .mount(&server).await;
    let client = mock_client(&server);
    let result = client.futures_foreign_commodity_realtime(&["CL"]).await;
    result.unwrap();
}

// ===========================================================================
// settle.rs — futures_settle_* methods
// ===========================================================================

#[tokio::test]
async fn test_futures_settle_cffex() {
    let server = MockServer::start().await;
    let csv = "品种,多头保证金比例,空头保证金比例,手续费率,交割手续费率,平今手续费率\n\
               IF,0.12,0.12,0.000023,0.000023,0.0345\n";
    mock_any_get_text(&server, ".*", csv).await;
    let client = mock_client(&server);
    let result = client.futures_settle_cffex("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_settle_czce() {
    let server = MockServer::start().await;
    let txt = "header line\n\
               品种月份|结算价|单边市|单边天数|保证金比例|限价幅度|交易手续费|手续费标志|交割手续费|平今手续费\n\
               SR403|6500|N|0|0.08|0.05|3.0|绝对值|5.0|0.0\n";
    mock_any_get_text(&server, ".*", txt).await;
    let client = mock_client(&server);
    let result = client.futures_settle_czce("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_settle_shfe() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_cursor": [
            {
                "INSTRUMENTID": "rb2405",
                "TRADEFEERATIO": "0.0001",
                "TTRADEFEERATIO": "0.0001",
                "COMMODITYDELIVFEERATIO": "0.0001",
                "SPECLONGMARGINRATIO": "0.10",
                "HEDGLONGMARGINRATIO": "0.10",
                "SPECSHORTMARGINRATIO": "0.10",
                "HEDGSHORTMARGINRATIO": "0.10",
                "SETTLEMENTPRICE": "3820.0"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_settle_shfe("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_settle_ine() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_cursor": [
            {
                "INSTRUMENTID": "sc2405",
                "TRADEFEERATIO": "0.0001",
                "TTRADEFEERATIO": "0.0001",
                "COMMODITYDELIVFEERATIO": "0.0001",
                "SPECLONGMARGINRATIO": "0.12",
                "HEDGLONGMARGINRATIO": "0.12",
                "SPECSHORTMARGINRATIO": "0.12",
                "HEDGSHORTMARGINRATIO": "0.12",
                "SETTLEMENTPRICE": "525.0"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_settle_ine("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_settle_gfex() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "code": "0",
        "data": [
            {
                "contractId": "SI2405",
                "specBuyRate": "0.10",
                "hedgeBuyRate": "0.10",
                "riseLimitRate": "0.07",
                "clientBuyPosiQuota": "1000"
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_settle_gfex("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_settle() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"o_cursor": []});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_settle("20240315", "SHFE").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_settle_invalid_market() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_settle("20240315", "INVALID").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_futures_stock_shfe_js() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "o_cursor": [
            {
                "INSTRUMENTID": "rb2405",
                "TRADEFEERATIO": "0.0001",
                "SETTLEMENTPRICE": "3820.0"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_stock_shfe_js("rb").await;
    result.unwrap();
}

// ===========================================================================
// comex.rs — futures_comex_inventory
// ===========================================================================

#[tokio::test]
async fn test_futures_comex_inventory_gold() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [
                {
                    "REPORT_DATE": "2024-03-15",
                    "STORAGE_TON": "500.0",
                    "STORAGE_OUNCE": "16000000.0"
                }
            ],
            "count": 1
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_comex_inventory("黄金").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_comex_inventory_silver() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [
                {
                    "REPORT_DATE": "2024-03-15",
                    "STORAGE_TON": "8000.0",
                    "STORAGE_OUNCE": "256000000.0"
                }
            ],
            "count": 1
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_comex_inventory("白银").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_comex_inventory_invalid() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_comex_inventory("invalid").await;
    assert!(result.is_err());
}

// ===========================================================================
// inventory.rs — futures_inventory, futures_inventory_99
// ===========================================================================

#[tokio::test]
async fn test_futures_inventory_em() {
    let server = MockServer::start().await;
    // Step 1: product code mapping, Step 2: inventory data
    let body1 = serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [
                {"TRADE_MARKET_CODE": "SHFE", "TRADE_CODE": "CU", "TRADE_TYPE": "铜"}
            ],
            "count": 1
        }
    });
    let body2 = serde_json::json!({
        "success": true,
        "message": "ok",
        "code": 0,
        "result": {
            "pages": 1,
            "data": [
                {"SECURITY_CODE": "CU", "TRADE_DATE": "2024-03-15", "ON_WARRANT_NUM": "50000", "ADDCHANGE": "1000"}
            ],
            "count": 1
        }
    });
    // Both calls go to the same URL pattern; we need the mock to handle both
    // Since both are GET to the same datacenter endpoint, wiremock will respond
    // in order. For simplicity, return the product mapping first.
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body1))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body2))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.futures_inventory("铜").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_inventory_99() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>99qh inventory data</html>").await;
    let client = mock_client(&server);
    let result = client.futures_inventory_99("豆一").await;
    result.unwrap();
}

// ===========================================================================
// foreign.rs — futures_foreign_hist, QHKC methods
// ===========================================================================

#[tokio::test]
async fn test_futures_foreign_hist() {
    let server = MockServer::start().await;
    let jsonp = r#"var _SGC2024_03_15=([["2024-01-02",2050.0,2080.0,2040.0,2060.0,100000,50000],["2024-01-03",2060.0,2090.0,2050.0,2070.0,110000,52000]]);"#;
    mock_any_get_text(&server, ".*", jsonp).await;
    let client = mock_client(&server);
    let result = client.futures_foreign_hist("GC").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_qhkc_fund_bs() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"name": "FundA", "buy": 1000, "sell": 900}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_qhkc_fund_bs("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_qhkc_fund_money_change() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"name": "FundA", "change": 100}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_qhkc_fund_money_change("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_qhkc_fund_position() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"name": "FundA", "position": 5000}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_qhkc_fund_position("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_qhkc_index() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"index": 1000.5, "date": "2024-03-15"}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_qhkc_index("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_qhkc_index_profit_loss() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"profit": 500.0, "loss": -200.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_qhkc_index_profit_loss("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_get_qhkc_index_trend() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"value": 1000.5, "date": "2024-03-15"}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.get_qhkc_index_trend("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_qhkc_tool_foreign() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"name": "LME Copper", "price": 8500.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.qhkc_tool_foreign().await;
    result.unwrap();
}

#[tokio::test]
async fn test_qhkc_tool_gdp() {
    let server = MockServer::start().await;
    let body = serde_json::json!({"data": [{"country": "China", "gdp": 18000.0}]});
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.qhkc_tool_gdp().await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_foreign_detail() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>GC contract details</html>").await;
    let client = mock_client(&server);
    let result = client.futures_foreign_detail("GC").await;
    result.unwrap();
}

// ===========================================================================
// comm.rs — futures_fees_info_openctp, futures_comm_js, etc.
// ===========================================================================

#[tokio::test]
async fn test_futures_fees_info_openctp() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html><table>fee data</table></html>").await;
    let client = mock_client(&server);
    let result = client.futures_fees_info_openctp().await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_comm_js() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "date": "2024-03-15",
                "heyue_name": "螺纹钢",
                "heyue_code": "rb",
                "heyue_price": "3800",
                "up_limit_num": "5.0",
                "down_limit_num": "5.0",
                "buy_ratio": "10.0",
                "sell_ratio": "10.0",
                "per_lot_price": "3.0",
                "buy_commission": "3.0",
                "sell_yesterday_commission": "3.0",
                "sell_cur_commission": "0.0",
                "jys": "上期所"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_comm_js("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_fees_info() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>9qihuo fee table</html>").await;
    let client = mock_client(&server);
    let result = client.futures_fees_info("rb").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_comm_info() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>9qihuo commission data</html>").await;
    let client = mock_client(&server);
    let result = client.futures_comm_info("所有").await;
    result.unwrap();
}

// ===========================================================================
// contract_detail.rs — contract detail methods
// ===========================================================================

#[tokio::test]
async fn test_futures_contract_detail_sina() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>Sina contract details</html>").await;
    let client = mock_client(&server);
    let result = client.futures_contract_detail_sina("rb2405").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_contract_detail() {
    let server = MockServer::start().await;
    // delegates to futures_contract_detail_em which needs HTML + JSON
    mock_any_get_text(
        &server,
        ".*",
        "<html><a href=\"#futures_rb2405\">detail</a></html>",
    )
    .await;
    mock_any_get(
        &server,
        ".*",
        serde_json::json!({"vname": "螺纹钢", "vcode": "RB"}),
    )
    .await;
    let client = mock_client(&server);
    let result = client.futures_contract_detail("rb2405").await;
    let _ = result;
}

#[tokio::test]
async fn test_match_main_contract() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "diff": [
                {"f12": "rb2405", "f14": "螺纹钢2405", "f2": 3800.0, "f3": 1.5, "f5": 200000.0, "f10": 150000.0}
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.match_main_contract("RB").await;
    result.unwrap();
}

// ===========================================================================
// hf_em.rs — futures_global_spot, futures_global_hist
// ===========================================================================

#[tokio::test]
async fn test_futures_global_spot_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "total": 1,
        "list": [
            {
                "dm": "HG00Y",
                "name": "COMEX铜",
                "p": 390.0,
                "zde": 5.0,
                "zdf": 1.3,
                "o": 385.0,
                "h": 392.0,
                "l": 383.0,
                "zjsj": 385.0,
                "vol": 50000,
                "ccl": 200000
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_global_spot().await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_global_hist_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "code": "HG00Y",
            "name": "COMEX铜",
            "klines": [
                "2024-01-02,3.85,3.90,3.92,3.83,50000,19000.0,2.3,1.3,0.05,1.2,3.88,200000,1000.0"
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_global_hist("HG00Y").await;
    result.unwrap();
}

// ===========================================================================
// index.rs (futures) — futures_index_ccidx
// ===========================================================================

#[tokio::test]
async fn test_futures_index_ccidx() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "dateLineJson": [
                {
                    "tradeDate": "2024-03-15",
                    "indexId": "100001.CCI",
                    "closingPrice": 1500.0,
                    "settlePrice": 1505.0,
                    "dailyIncreaseAndDecrease": 10.0,
                    "dailyIncreaseAndDecreasePercentage": 0.67
                }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_index_ccidx("中证商品期货指数").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_index_ccidx_invalid() {
    let server = MockServer::start().await;
    mock_any_get(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_index_ccidx("invalid").await;
    assert!(result.is_err());
}

// ===========================================================================
// derivative.rs — contract info, hog data
// ===========================================================================

#[tokio::test]
async fn test_futures_contract_info_cffex() {
    let server = MockServer::start().await;
    let xml = r#"<?xml version="1.0"?><root><INDEX><TRADING_DAY>20240315</TRADING_DAY><PRODUCT_ID>IF</PRODUCT_ID><INSTRUMENT_ID>IF2403</INSTRUMENT_ID><INSTRUMENT_MONTH>2403</INSTRUMENT_MONTH></INDEX></root>"#;
    mock_any_get_text(&server, ".*", xml).await;
    let client = mock_client(&server);
    let result = client.futures_contract_info_cffex("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_contract_info_czce() {
    let server = MockServer::start().await;
    let xml = r#"<?xml version="1.0"?><root><Contract><Name>白糖</Name><CtrCd>SR403</CtrCd><PrdCd>SR</PrdCd></Contract></root>"#;
    mock_any_get_text(&server, ".*", xml).await;
    let client = mock_client(&server);
    let result = client.futures_contract_info_czce("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_contract_info_dce() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "variety": "豆一",
                "contractId": "a2405",
                "unit": "10",
                "tick": "1",
                "startTradeDate": "2023-05-22",
                "endTradeDate": "2024-05-17",
                "endDeliveryDate": "2024-05-22"
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_contract_info_dce().await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_contract_info_gfex() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            {
                "variety": "工业硅",
                "contractId": "SI2405",
                "unit": "5",
                "tick": "5",
                "startTradeDate": "2023-12-22",
                "endTradeDate": "2024-05-17",
                "endDeliveryDate0": "2024-05-22"
            }
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_contract_info_gfex().await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_contract_info_ine() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "ContractBaseInfo": [
            {
                "INSTRUMENTID": "sc2405",
                "OPENDATE": "2023-05-22",
                "EXPIREDATE": "2024-05-17",
                "STARTDELIVDATE": "2024-05-01",
                "ENDDELIVDATE": "2024-05-31",
                "BASISPRICE": "520.0",
                "TRADINGDAY": "20240315"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_contract_info_ine("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_contract_info_shfe() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "ContractBaseInfo": [
            {
                "INSTRUMENTID": "rb2405",
                "OPENDATE": "2023-05-22",
                "EXPIREDATE": "2024-05-17",
                "STARTDELIVDATE": "2024-05-01",
                "ENDDELIVDATE": "2024-05-31",
                "BASISPRICE": "3800.0",
                "TRADINGDAY": "20240315"
            }
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_contract_info_shfe("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_hog_core() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            [15.5, "2024-03-15"],
            [15.3, "2024-03-14"]
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_hog_core("外三元").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_hog_core_invalid() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_hog_core("invalid").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_futures_hog_cost_corn() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            [2800.0, "2024-03-15"],
            [2780.0, "2024-03-14"]
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_hog_cost("玉米").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_hog_cost_invalid() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_hog_cost("invalid").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_futures_hog_supply() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": [
            ["2024-03-15", "全国", 18.5],
            ["2024-03-14", "全国", 18.3]
        ]
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_hog_supply("猪肉批发价").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_hog_supply_invalid() {
    let server = MockServer::start().await;
    mock_any_post(&server, ".*", serde_json::json!({})).await;
    let client = mock_client(&server);
    let result = client.futures_hog_supply("invalid").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_futures_spot_sys() {
    let server = MockServer::start().await;
    let client = mock_client(&server);
    let result = client.futures_spot_sys("rb", "price").await;
    assert!(result.is_ok());
}

// ===========================================================================
// news.rs — futures_news_shmet
// ===========================================================================

#[tokio::test]
async fn test_futures_news_shmet() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "dataList": [
                [1, 2, 3, 1710489600000_i64, 5, "铜价上涨消息"]
            ]
        }
    });
    mock_any_post(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_news_shmet("铜").await;
    result.unwrap();
}

// ===========================================================================
// rule.rs — futures_rule_gtja, futures_rule, futures_rule_em
// ===========================================================================

#[tokio::test]
async fn test_futures_rule_gtja() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>GTJA trading calendar</html>").await;
    let client = mock_client(&server);
    let result = client.futures_rule_gtja("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_rule() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>GTJA rules</html>").await;
    let client = mock_client(&server);
    let result = client.futures_rule("20240315").await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_rule_em() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "Data": [
            {"name": "铜", "exchange": "SHFE", "unit": "5吨/手"}
        ]
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_rule_em().await;
    result.unwrap();
}

// ===========================================================================
// sgx.rs — futures_settlement_price_sgx
// ===========================================================================

#[tokio::test]
async fn test_futures_settlement_price_sgx() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "klines": [
                "2024-01-02,3200.0,3220.0,3250.0,3180.0,100000,1.5,10.0,0.3,5.0,0.15"
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_settlement_price_sgx("20240315").await;
    result.unwrap();
}

// ===========================================================================
// spot.rs (futures) — futures_spot_prices
// ===========================================================================

#[tokio::test]
async fn test_futures_spot_prices() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "data": {
            "diff": [
                {
                    "f12": "rb2405",
                    "f14": "螺纹钢2405",
                    "f2": 3800.0,
                    "f3": 1.5,
                    "f5": 200000.0,
                    "f10": 150000.0
                }
            ]
        }
    });
    mock_any_get(&server, ".*", body).await;
    let client = mock_client(&server);
    let result = client.futures_spot_prices(10).await;
    result.unwrap();
}

// ===========================================================================
// sina.rs — futures_symbol_mark, futures_zh_realtime, futures_zh_spot
// ===========================================================================

#[tokio::test]
async fn test_futures_symbol_mark() {
    let server = MockServer::start().await;
    // The JS file contains a JSON object with exchange symbol arrays
    let js_body = r#"var qihuohangqing = {"dce":[["DCE",[["a","nf_A0"],["m","nf_M0"]]],["CZCE",["TA","MA"]]],"czce":[["CZCE",[["TA","nf_TA0"],["MA","nf_MA0"]]]],"shfe":[["SHFE",[["rb","nf_RB0"],["cu","nf_CU0"]]]],"cffex":[["CFFEX",[["IF","nf_IF0"],["IC","nf_IC0"]]]],"gfex":[["GFEX",[["si","nf_SI0"]]]]};"#;
    mock_any_get_text(&server, ".*", js_body).await;
    let client = mock_client(&server);
    let result = client.futures_symbol_mark().await;
    result.unwrap();
}

#[tokio::test]
async fn test_futures_zh_realtime() {
    let server = MockServer::start().await;
    // First call: symbol mark JS file
    let js_body = r#"var qihuohangqing = {"dce":[["DCE",[["a","nf_A0"],["m","nf_M0"]]],["CZCE",["TA","MA"]]],"czce":[["CZCE",[["TA","nf_TA0"],["MA","nf_MA0"]]]],"shfe":[["SHFE",[["rb","nf_RB0"],["cu","nf_CU0"]]]],"cffex":[["CFFEX",[["IF","nf_IF0"],["IC","nf_IC0"]]]],"gfex":[["GFEX",[["si","nf_SI0"]]]]};"#;
    mock_any_get_text(&server, ".*", js_body).await;
    // Second call: realtime data returns JSON array
    let realtime_body = serde_json::json!([
        {"symbol": "rb2405", "name": "螺纹钢2405", "trade": "3800.0", "position": "200000"}
    ]);
    mock_any_get(&server, ".*", realtime_body).await;
    let client = mock_client(&server);
    let result = client.futures_zh_realtime("rb").await;
    let _ = result;
}

#[tokio::test]
async fn test_futures_zh_spot() {
    let server = MockServer::start().await;
    // hq.sinajs.cn returns text in var hq_str_nf_XXX="field1,field2,..." format
    let body = r#"var hq_str_nf_V2309="塑料2309,2024-03-15 15:00:00,7500.0,7550.0,7480.0,7520.0,7510.0,7500.0,7515.0,7512.0,7505.0,5000,3000,80000,200000";"#;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.futures_zh_spot("V2309", "CF").await;
    result.unwrap();
}

// ===========================================================================
// derivative.rs — futures_display_main
// ===========================================================================

#[tokio::test]
async fn test_futures_display_main_sina() {
    let server = MockServer::start().await;
    // zh_subscribe_exchange_symbol needs the JS file
    let js_body = r#"var qihuohangqing = {"dce":[["DCE",[["a","nf_A0"],["m","nf_M0"]]]],"czce":[["CZCE",[["TA","nf_TA0"]]]],"shfe":[["SHFE",[["rb","nf_RB0"]]]],"cffex":[["CFFEX",[["IF","nf_IF0"]]]],"gfex":[["GFEX",[["si","nf_SI0"]]]]};"#;
    mock_any_get_text(&server, ".*", js_body).await;
    // match_main_contract_sina GET to Market_Center.getHQFuturesData
    let realtime_body = serde_json::json!([
        {"symbol": "a0", "name": "豆一", "trade": "4800.0", "position": "80000"}
    ]);
    mock_any_get(&server, ".*", realtime_body).await;
    let client = mock_client(&server);
    let result = client.futures_display_main().await;
    result.unwrap();
}

// ===========================================================================
// hist_em.rs — futures_hist_table
// ===========================================================================

#[tokio::test]
async fn test_futures_hist_table_em() {
    let server = MockServer::start().await;
    // First call: market list, Second call: detail per market
    let market_list = serde_json::json!([
        {"mktid": 113, "mktname": "上期所"},
        {"mktid": 114, "mktname": "大商所"}
    ]);
    let detail = serde_json::json!([
        {"name": "铜2405", "code": "cu2405", "vcode": "cu", "vname": "铜", "mktid": 113, "mktname": "上期所"}
    ]);
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(market_list))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(detail))
        .mount(&server)
        .await;
    let client = mock_client(&server);
    let result = client.futures_hist_table().await;
    result.unwrap();
}

// ===========================================================================
// spot_stock.rs — futures_spot_stock_em
// ===========================================================================

#[tokio::test]
async fn test_futures_spot_stock_em() {
    let server = MockServer::start().await;
    mock_any_get_text(&server, ".*", "<html>eastmoney spot stock data</html>").await;
    let client = mock_client(&server);
    let result = client.futures_spot_stock_em("能源").await;
    result.unwrap();
}
