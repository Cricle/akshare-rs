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
