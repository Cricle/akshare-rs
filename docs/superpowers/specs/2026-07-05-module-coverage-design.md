# Module Coverage Push — Design Spec

**Date:** 2026-07-05
**Goal:** Achieve 100% test coverage and 100% documentation coverage per module, starting with `stock/a_share.rs` and `stock/hk.rs`.

## Problem

The codebase has 1,569 pub async functions across 324 source files. Current coverage:
- 79 of 241 modules have tests (33%)
- 404 test blocks total — many modules have zero tests
- 248 of 324 files have at least one `///` doc comment, but depth is inconsistent
- `stock/a_share.rs` (12 fns, 0 tests, thin docs)
- `stock/hk.rs` (4 fns, 1 unit test for a helper, good docs for HK fns)

These two modules are the primary user entry points — they appear in README quick-start examples and are the first functions new users call.

## Approach: Module-by-Module, Batch 1

For each module in the batch, deliver three things simultaneously:
1. **Unit tests** — `#[cfg(test)]` block in source file, testing pure functions
2. **Mock integration tests** — wiremock-based tests in `tests/` that verify functions return non-panicking results
3. **Documentation** — expanded `///` doc comments and per-module docs sections

### Batch 1 target: `stock/a_share.rs` + `stock/hk.rs`

These share the same testing infrastructure (macro macros, wiremock helpers) and documentation format, so they form a natural first batch.

## Test Design

### Layer 1: Pure-function Unit Tests (in source file)

**`stock/a_share.rs`**: No pure functions to test — all 12 functions are thin wrappers over provider methods. The only logic worth testing is indirect (symbol normalization happens inside called functions). So unit tests will validate:

```rust
#[cfg(test)]
mod tests {
    // Validate that invalid symbols produce proper errors on Tencent quote path
    // We can test this via the mock client path
}
```

Actually, since `a_share.rs` has no pure helper functions, unit tests go into the mock integration layer. The `#[cfg(test)]` block will contain doc-tests for verification assertions: testing that docs examples compile.

**`stock/hk.rs`**: Already has `test_hk_yahoo_symbol` tests. Add:
- Edge cases for `hk_yahoo_symbol`: max-length (5-digit), empty string, non-ASCII, leading zeros variants
- Test for `hk_market_cap_from_tencent` with crafted Tencent response text (since it parses a specific format)

### Layer 2: Mock Integration Tests (in `tests/mock_stock.rs`)

Extend the existing file. Use the macro pattern already established:

```
macro_test_arg1!(test_mock_a_share_search, a_share_search, "平安");
macro_test_arg2!(test_mock_a_share_candles, a_share_candles, "600000", "qfq", 60);
```

Functions and their arg counts:

**`a_share.rs` (12 functions)**:
| Function | Args | Macro |
|---|---|---|
| `a_share_quote` | 1: symbol | `macro_test_arg1!` |
| `a_share_candles` | 3: symbol, adjust, limit | `macro_test_arg3!` (new) |
| `a_share_search` | 3: query, market, limit | `macro_test_arg3!` |
| `a_share_capital_flow` | 2: symbol, limit | `macro_test_arg2!` |
| `a_share_sector_rankings` | 2: sector_type, limit | `macro_test_arg2!` |
| `a_share_sector_constituents` | 2: sector_code, limit | `macro_test_arg2!` |
| `a_share_sector_capital_flow` | 2: sector_code, limit | `macro_test_arg2!` |
| `a_share_billboard` | 2: symbol, limit | `macro_test_arg2!` |
| `a_share_billboard_seats` | 3: symbol, side, limit | `macro_test_arg3!` |
| `a_share_announcements` | 2: symbol, limit | `macro_test_arg2!` |
| `a_share_announcement_detail` | 1: art_code | `macro_test_arg1!` |
| `a_share_trade_calendar` | 3: exchange, start, end | `macro_test_arg3!` |

Need a new `macro_test_arg3!` macro for 3-arg functions.

**`hk.rs` (4 functions)**:
| Function | Args | Macro |
|---|---|---|
| `hk_quote` | 1: symbol | `macro_test_arg1!` |
| `hk_candles` | 2: symbol, limit | `macro_test_arg2!` |
| `hk_financial` | 1: symbol | `macro_test_arg1!` |
| `hk_search` | 2: query, limit | `macro_test_arg2!` |
| `hk_capital_flow` | 2: symbol, limit | `macro_test_arg2!` |
| `hk_market_cap_from_tencent` | 1: symbol | `macro_test_arg1!` |

Note: `hk_market_cap_from_tencent` returns `Result<Option<f64>>`, not typical — validates it doesn't panic.

### Layer 3: Error-path tests (in `tests/mock_errors.rs`)

Add targeted error tests:
- `test_mock_error_tencent_hk_quote_empty` — empty Tencent response for HK quote, verify fallback to Yahoo path
- `test_mock_error_a_share_invalid_symbol` — invalid symbol on `a_share_quote`, verify error kind
- `test_mock_error_a_share_all_providers_fail` — all providers returning empty, verify error

### Layer 4: Unit test for `hk_market_cap_from_tencent` parsing

Since this function does its own HTTP + response parsing (not delegating to a provider method), it needs a unit test with crafted response text. Add to `hk.rs` `#[cfg(test)]`:

```rust
#[test]
fn test_parse_tencent_hk_market_cap_response() {
    // Craft a minimal Tencent qt response and verify market cap extraction
}
```

## Documentation Design

### Source-level doc comments

**`a_share.rs`**: Each function already has a brief `///` comment. Expand each to:
1. One-line summary
2. Parameter descriptions
3. Return type description
4. Fallback chain (when applicable — quote, candles)
5. Example usage code block

Current state for `a_share_quote`:
```rust
/// Get A-share quote with fallback: Tencent -> Sina realtime -> Tushare daily
```

Target state:
```rust
/// Get A-share real-time quote with multi-source fallback.
///
/// Tries providers in order: Tencent → Sina realtime → Tushare daily.
/// Falls through each layer only if the previous one fails.
///
/// # Arguments
/// * `symbol` - A-share stock code (e.g. "600000" for 浦发银行)
///
/// # Returns
/// * `QuoteSnapshot` with open, high, low, close, volume, and date
///
/// # Errors
/// * `InvalidInput` if symbol format is unrecognized
/// * `Upstream` if all three providers fail
///
/// # Example
/// ```rust,no_run
/// # use akshare::AkShareClient;
/// # async fn example() -> Result<(), akshare::Error> {
/// let client = AkShareClient::new();
/// let quote = client.a_share_quote("600000").await?;
/// println!("{}: close={}", quote.symbol, quote.close);
/// # Ok(())
/// # }
/// ```
```

Same treatment for all 12 `a_share_*` functions and all 6 `hk_*` functions.

### Module-level docs (`docs/stock.md`)

Add two new sections after the existing sub-module list:

**A-Share Core Functions Reference** — table with columns: Function, Description, Parameters, Returns, Example

**HK Stock Core Functions Reference** — same format

### Doc-tests

Add `#[cfg(doctest)]` blocks or integrate `///` example code blocks (rust,no_run) for key functions. Since these call external APIs, use `no_run` to avoid network dependency. The primary verification is that examples compile.

## Verification

1. `cargo test` — all mock tests pass, no regressions
2. `cargo clippy` — no new warnings
3. `cargo doc --no-deps` — docs generate without errors
4. Manual review of `docs/stock.md` for accuracy

## Success Criteria

- All 12 `a_share_*` functions have mock integration tests
- All 6 `hk_*` functions have mock integration tests (2 may already exist)
- `hk.rs` unit tests cover `hk_yahoo_symbol` edge cases + `hk_market_cap_from_tencent` parsing
- All 18 functions have expanded `///` doc comments with arguments, returns, errors, examples
- `docs/stock.md` has A-Share and HK reference tables
- `cargo test` passes, `cargo clippy` clean
