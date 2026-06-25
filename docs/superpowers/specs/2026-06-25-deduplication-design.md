# Codebase Deduplication & Dead Code Cleanup

**Date:** 2026-06-25
**Status:** Approved
**Scope:** Full codebase cleanup (big-bang approach)

## Problem

The `duplicate_code` tool found 30,593 lines of duplicate output across the codebase. Key offenders:

| File | Duplicate mentions |
|------|-------------------|
| `us_extra.rs` | 567 |
| `hk_extra.rs` | 469 |
| `eastmoney_misc.rs` | 456 |
| `board_em.rs` | 392 |
| `feature/stock_other.rs` | 388 |
| `zh_a.rs` | 360 |
| `eastmoney_detail.rs` | 329 |

## Duplicate Patterns Identified

### 1. OHLCV Struct Definitions

The same struct with `pub date: String, pub open: f64, pub close: f64, pub high: f64, pub low: f64, pub volume: f64` is defined in:
- `board_em.rs`
- `zh_a.rs`
- `zh_ah.rs`
- `feature/types.rs`

Extended version adds: `amount, amplitude_pct, change_pct, change_amount, turnover_rate`.

### 2. `fqt` Adjust Match Block

Identical 6-line match expression in 5 files:
```rust
let fqt = match adjust {
    "" => "0",
    "qfq" => "1",
    "hfq" => "2",
    _ => return Err(Error::invalid_input(format!("invalid adjust: {adjust}"))),
};
```

Files: `board_em.rs`, `hk_extra.rs`, `zh_a.rs`, `zh_b.rs`, `zh_kcb.rs`

### 3. HTTP Request Boilerplate

The same send-check pattern repeated 50+ times:
```rust
.send()
.await
.map_err(Error::from)?
.error_for_status()
.map_err(Error::from)?;
```

### 4. Eastmoney API Query Parameters

Same query parameter blocks in 10+ files:
```rust
("pn", "1"),
("pz", "5000"),
("po", "1"),
("np", "1"),
("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
("fltt", "2"),
("invt", "2"),
```

Also: `("source", "F10/HSF10"), ("client", "PC")` block.

## Dead Code

- `wire.rs`: 14 `#[allow(dead_code)]` annotations
- `lib.rs`: suppresses `unused_async` for "45 stubs that must stay async for API compatibility"
- Potential unreferenced modules/functions

## Design

### Phase 1: Shared Types (`types/mod.rs`)

Add to `crates/akshare/src/types/mod.rs`:

```rust
/// Standard OHLCV row used across stock/index modules
#[derive(Debug, Clone, Deserialize)]
pub struct OhlcvRow {
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

/// Extended OHLCV with additional fields
#[derive(Debug, Clone, Deserialize)]
pub struct OhlcvExtended {
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub amount: f64,
    pub amplitude_pct: f64,
    pub change_pct: f64,
    pub change_amount: f64,
    pub turnover_rate: f64,
}

/// Eastmoney adjust type (前复权/后复权/不复权)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjustType {
    /// No adjustment
    None,
    /// Forward adjustment (前复权)
    Forward,
    /// Backward adjustment (后复权)
    Backward,
}

impl AdjustType {
    /// Convert to Eastmoney API parameter value
    pub fn to_eastmoney_str(self) -> &'static str {
        match self {
            Self::None => "0",
            Self::Forward => "1",
            Self::Backward => "2",
        }
    }

    /// Parse from user-facing string ("", "qfq", "hfq")
    pub fn from_adjust_str(s: &str) -> crate::Result<Self> {
        match s {
            "" => Ok(Self::None),
            "qfq" => Ok(Self::Forward),
            "hfq" => Ok(Self::Backward),
            _ => Err(crate::Error::invalid_input(format!(
                "invalid adjust: {s}"
            ))),
        }
    }
}
```

### Phase 2: HTTP Helpers (`util.rs`)

Add to `crates/akshare/src/util.rs`:

```rust
use reqwest::Response;

/// Send request and check for HTTP errors
pub(crate) async fn send_and_check(
    builder: reqwest::RequestBuilder,
) -> crate::Result<Response> {
    builder
        .send()
        .await
        .map_err(crate::Error::from)?
        .error_for_status()
        .map_err(crate::Error::from)
}

/// Build Eastmoney clist query parameters
pub(crate) fn eastmoney_clist_params<'a>(
    pz: &'a str,
    extra: &[(&'a str, &'a str)],
) -> Vec<(&'a str, &'a str)> {
    let mut params = vec![
        ("pn", "1"),
        ("pz", pz),
        ("po", "1"),
        ("np", "1"),
        ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
        ("fltt", "2"),
        ("invt", "2"),
    ];
    params.extend_from_slice(extra);
    params
}

/// Build Eastmoney F10 query parameters
pub(crate) fn eastmoney_f10_params(source: &str) -> Vec<(&str, &str)> {
    vec![("source", source), ("client", "PC")]
}
```

### Phase 3: Dead Code Cleanup

- Remove unused fields/functions in `wire.rs` (remove `#[allow(dead_code)]` and the dead items)
- Audit `unused_async` stubs — keep only those that are part of the public API
- Remove any unreferenced modules

### Phase 4: Update All Consumers

Replace inline duplicates across all modules:

**stock/**: `board_em.rs`, `zh_a.rs`, `zh_b.rs`, `zh_kcb.rs`, `zh_ah.rs`, `hk_extra.rs`, `us_extra.rs`
- Replace local OHLCV structs with `types::OhlcvRow` / `OhlcvExtended`
- Replace `fqt` match with `AdjustType::from_adjust_str(adjust)?.to_eastmoney_str()`
- Replace HTTP send boilerplate with `util::send_and_check()`
- Replace eastmoney query params with `util::eastmoney_clist_params()`

**provider/**: `eastmoney.rs`, `eastmoney_detail.rs`
- Same replacements as above

**feature/**: `fund_flow.rs`, `stock_other.rs`, `rank_ths.rs`, etc.
- Same replacements

## Testing

1. `cargo test --all-features` after each phase
2. `cargo clippy --all-features` — no new warnings
3. `duplicate_code` rescan at end — measure improvement
4. `cargo build --all-features` — verify compilation

## Success Criteria

- `duplicate_code` output reduced by >50%
- No new clippy warnings
- All existing tests pass
- No public API changes (types added, not removed)
