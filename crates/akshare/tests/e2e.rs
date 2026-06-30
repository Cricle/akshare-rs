//! End-to-end tests that call real external APIs.
//!
//! These tests are `#[ignore]`d by default and require the `RUN_E2E=1`
//! environment variable to run. They also rate-limit (2s sleep between tests)
//! to avoid impacting external systems.
//!
//! ```bash
//! RUN_E2E=1 cargo test -p akshare --test e2e -- --ignored
//! ```

use akshare::AkShareClient;

/// Skip unless `RUN_E2E=1` is set. Returns a real client.
fn require_e2e() -> AkShareClient {
    if std::env::var("RUN_E2E").unwrap_or_default() != "1" {
        panic!("Set RUN_E2E=1 to run E2E tests");
    }
    AkShareClient::new()
}

/// Rate-limit: sleep 2s between tests to avoid hitting APIs too fast.
async fn rate_limit() {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
}

// ===========================================================================
// Stock
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_stock_zh_a_spot() {
    let client = require_e2e();
    let result = client.stock_zh_a_spot().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_stock_zh_a_hist() {
    let client = require_e2e();
    let result = client
        .stock_zh_a_hist("600000", "daily", "qfq", "20240102", "20240105")
        .await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Index
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_index_zh_a_hist() {
    let client = require_e2e();
    let result = client.index_zh_a_hist("000001", "daily").await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Fund
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_fund_etf_spot_em() {
    let client = require_e2e();
    let result = client.fund_etf_spot_em().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Macro
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_china_gdp() {
    let client = require_e2e();
    let result = client.china_gdp().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Bond
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_bond_zh_cov() {
    let client = require_e2e();
    let result = client.bond_zh_cov(10).await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_bond_convertible_list() {
    let client = require_e2e();
    let result = client.bond_convertible_list(10).await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_bond_corporate_yields() {
    let client = require_e2e();
    let result = client.bond_corporate_yields(10).await;
    // Eastmoney report RPT_BOND_ISSUE has been retired
    assert!(result.is_err(), "expected error for retired report");
    rate_limit().await;
}

// ===========================================================================
// Forex
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_forex_boc_rates() {
    let client = require_e2e();
    let result = client.forex_boc_rates().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_forex_em_rates() {
    let client = require_e2e();
    let result = client.forex_em_rates().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Futures
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_futures_zh_realtime() {
    let client = require_e2e();
    let result = client.futures_zh_realtime("RB").await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Option
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_option_current_day_sse() {
    let client = require_e2e();
    let result = client.option_current_day_sse().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    rate_limit().await;
}

// ===========================================================================
// News
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_news_cctv() {
    let client = require_e2e();
    let result = client.news_cctv("20240101").await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_bing_news() {
    let client = require_e2e();
    let result = client.bing_news("finance", 10).await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Macro Data
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_china_cpi() {
    let client = require_e2e();
    let result = client.china_cpi().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_china_pmi() {
    let client = require_e2e();
    let result = client.china_pmi().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_china_trade() {
    let client = require_e2e();
    let result = client.china_trade().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_rate_interbank() {
    let client = require_e2e();
    let result = client
        .rate_interbank("上海银行同业拆借市场", "Shibor人民币", "1月")
        .await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Crypto
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_crypto_js_spot() {
    let client = require_e2e();
    let result = client.crypto_js_spot().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Commodity
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_energy_oil_hist() {
    let client = require_e2e();
    let result = client.energy_oil_hist().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_commodity_spot_prices() {
    let client = require_e2e();
    let result = client.commodity_spot_prices(10).await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// REITs
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_reits_list() {
    let client = require_e2e();
    let result = client.reits_list(10).await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Spot
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_spot_quotations_sge() {
    let client = require_e2e();
    let result = client.spot_quotations_sge("Au99.99").await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Fund (additional)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_bond_china_yield() {
    let client = require_e2e();
    let result = client.bond_china_yield("2024-01-01", "2024-01-31").await;
    // Eastmoney report RPT_BOND_GOV_CN_YIELD has been retired
    assert!(result.is_err(), "expected error for retired report");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_fund_open_fund_rank() {
    let client = require_e2e();
    let result = client.fund_open_fund_rank("1", 10).await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Index (additional)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_index_global_spot() {
    let client = require_e2e();
    let result = client.index_global_spot().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Stock (additional)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_stock_zh_a_spot_em() {
    let client = require_e2e();
    let result = client.stock_zh_a_spot_em_flex(10).await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_stock_zh_b_spot() {
    let client = require_e2e();
    let result = client.stock_zh_b_spot().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_stock_us_daily() {
    let client = require_e2e();
    let result = client.stock_us_daily("AAPL", "20240101", "20240110").await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_sina_a_share_realtime() {
    let client = require_e2e();
    let result = client.sina_a_share_realtime("sh600000").await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    rate_limit().await;
}

#[tokio::test]
#[ignore]
async fn e2e_stock_zh_index_spot_em() {
    let client = require_e2e();
    let result = client.stock_zh_index_spot_em().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Tool
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_tool_trade_date_hist() {
    let client = require_e2e();
    let result = client.tool_trade_date_hist().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Bond (additional)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_bond_zh_us_rate() {
    let client = require_e2e();
    let result = client.bond_zh_us_rate("20240101").await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Futures (additional)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_futures_main() {
    let client = require_e2e();
    let result = client.futures_main("RB", 10).await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Option (additional)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_option_sse_greeks() {
    let client = require_e2e();
    let result = client.option_sse_greeks("510050").await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    rate_limit().await;
}

// ===========================================================================
// Forex (additional)
// ===========================================================================
// Note: e2e_forex_boc_rates already exists above.

// ===========================================================================
// Crypto (additional)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_crypto_bitcoin() {
    let client = require_e2e();
    let result = client.crypto_bitcoin_hold_report().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Macro (additional)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_us_gdp() {
    let client = require_e2e();
    let result = client.us_gdp_monthly().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// News (additional)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_sogou_news() {
    let client = require_e2e();
    let result = client.sogou_news_search("finance", 10).await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}

// ===========================================================================
// Economy (additional)
// ===========================================================================

#[tokio::test]
#[ignore]
async fn e2e_economy_event() {
    let client = require_e2e();
    let result = client.economy_box_office().await;
    assert!(result.is_ok(), "API failed: {:?}", result.err());
    let data = result.unwrap();
    assert!(!data.is_empty(), "expected non-empty data");
    rate_limit().await;
}
