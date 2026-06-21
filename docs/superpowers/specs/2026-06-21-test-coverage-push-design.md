# Test Coverage Push — Design Spec

**Date:** 2026-06-21
**Goal:** Achieve comprehensive test coverage for all under-tested modules in akshare-rs.

## Problem

The akshare-rs codebase has 1488 public functions but only 1238 tests (83% coverage). The biggest gaps:

| Module | Functions | Tests | Gap |
|--------|-----------|-------|-----|
| macro_data | 423 | 10 | 413 untested |
| economy | 62 | ~5 | ~57 untested |
| forex | 19 | ~6 | ~13 untested |
| news | 17 | ~10 | ~7 untested |
| option | 48 | ~4 | ~4 untested |
| futures | 113 | ~5 | ~5 untested |
| bond | 56 | ~3 | ~3 untested |
| fund | 98 | ~5 | ~5 untested |
| stock | 469 | ~50 | ~50 untested |
| bank | 1 | 0 | 1 untested |
| crypto | 4 | ~0 | ~4 untested |

## Approach: Hybrid (Mock + Integration)

### Mock Tests (~558 tests)

For each untested function, generate a mock test that:
1. Starts a wiremock server
2. Registers catch-all GET/POST mocks returning plausible response shapes
3. Calls the function with mock client
4. Asserts the call doesn't panic (result is Ok or a recognized error)

Uses the existing `macro_test!` macro pattern from `tests/macro_data.rs`.

### Integration Tests (~35 tests)

Pick 2-3 representative functions per module for real-API integration tests:
- Different data sources (Eastmoney, Sina, Tencent, Yahoo, etc.)
- Complex parsing logic
- Functions with recent bug fixes

Each integration test calls the real API, asserts non-empty result, and spot-checks field values.

### Error Tests (~12 tests)

One test per data source pattern for HTTP 404, 403, malformed JSON, and empty responses.

## Test Infrastructure Changes

### `common/mod.rs` additions

- `mock_em_datacenter_no_args()` — mounts catch-all mock for Eastmoney datacenter
- `mock_sina_text(response)` — mounts catch-all mock for Sina text endpoints
- `mock_tencent_text(response)` — same for Tencent
- `sample_row(fields)` — generic row builder for `Vec<Row>` functions

### New macro variants

```rust
// Single-arg functions
macro_rules! macro_test_arg1 {
    ($test_name:ident, $method:ident, $arg:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = MockServer::start().await;
            mount_em_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg).await;
            let _ = result;
        }
    };
}

// Two-arg functions
macro_rules! macro_test_arg2 {
    ($test_name:ident, $method:ident, $arg1:expr, $arg2:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let server = MockServer::start().await;
            mount_em_mocks(&server).await;
            let client = common::mock_client(&server);
            let result = client.$method($arg1, $arg2).await;
            let _ = result;
        }
    };
}
```

For functions with 3+ args, write inline tests following the same pattern:
```rust
#[tokio::test]
async fn test_mock_stock_zh_a_hist() {
    let server = MockServer::start().await;
    mount_em_mocks(&server).await;
    let client = common::mock_client(&server);
    let result = client.stock_zh_a_hist("600000", "qfq", "2024-01-01", "2024-01-31").await;
    let _ = result;
}
```
```

## File Organization

```
tests/
  common/mod.rs              (extend with new helpers)
  mock_macro_data.rs         (~413 tests)
  mock_economy.rs            (~57 tests)
  mock_forex.rs              (~13 tests)
  mock_news.rs               (~7 tests)
  mock_bank.rs               (~1 test)
  mock_commodity.rs          (~0, already covered)
  mock_crypto.rs             (~4 tests)
  mock_option.rs             (~4 tests)
  mock_futures.rs            (~5 tests)
  mock_bond.rs               (~3 tests)
  mock_fund.rs               (~5 tests)
  mock_stock.rs              (~50 tests)
  mock_errors.rs             (~12 tests)
  integration_macro_data.rs  (~3 tests)
  integration_economy.rs     (~3 tests)
  integration_forex.rs       (~3 tests)
  integration_news.rs        (~3 tests)
  integration_misc.rs        (~23 tests)
```

## Test Naming Convention

- Mock: `test_mock_<module>_<function>()`
- Integration: `test_integration_<module>_<function>()`
- Error: `test_mock_error_<source>_http_<status>()`

## CI Strategy

- Every PR: `cargo test` (mock tests, fast, no network)
- Nightly/manual: `cargo test -- --ignored` (integration tests, slow, requires network)

## Implementation Order

1. Extend `common/mod.rs` with new helpers
2. Create `macro_test_arg1!` macro variant
3. Write `mock_macro_data.rs` (~413 tests)
4. Write `mock_economy.rs` (~57 tests)
5. Write remaining `mock_*.rs` files (~88 tests)
6. Write `mock_errors.rs` (~12 tests)
7. Write `integration_macro_data.rs` (~3 tests)
8. Write remaining `integration_*.rs` files (~32 tests)
9. Verify: `cargo test` passes, `cargo clippy` clean

## Success Criteria

- All 558+ mock tests pass with `cargo test`
- All 35+ integration tests pass with `cargo test -- --ignored`
- No new clippy warnings
- Test coverage increases from 83% to ~99%
