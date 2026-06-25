# Codebase Deduplication & Dead Code Cleanup Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Eliminate ~30K lines of duplicate code and clean up dead code across the akshare-rs codebase.

**Architecture:** Extract shared types (OHLCV, AdjustType) into `types/mod.rs`, HTTP helpers into `util.rs`, then update all consumer modules to use them. Remove dead code annotations where fields are truly unused.

**Tech Stack:** Rust, reqwest, serde, serde_json

---

## File Map

| File | Action | Purpose |
|------|--------|---------|
| `crates/akshare/src/types/mod.rs` | Modify | Add `AdjustType` enum, reuse existing `CandlePoint` |
| `crates/akshare/src/util.rs` | Modify | Add `send_and_check()`, `eastmoney_clist_params()`, `eastmoney_f10_params()` |
| `crates/akshare/src/stock/board_em.rs` | Modify | Replace local structs and inline patterns |
| `crates/akshare/src/stock/zh_a.rs` | Modify | Replace `fqt` match, use shared types |
| `crates/akshare/src/stock/zh_b.rs` | Modify | Replace `fqt` match |
| `crates/akshare/src/stock/zh_kcb.rs` | Modify | Replace `fqt` match |
| `crates/akshare/src/stock/hk_extra.rs` | Modify | Replace `fqt` match, HTTP boilerplate, query params |
| `crates/akshare/src/stock/us_extra.rs` | Modify | Replace HTTP boilerplate, query params |
| `crates/akshare/src/stock/eastmoney_misc.rs` | Modify | Replace HTTP boilerplate |
| `crates/akshare/src/stock/eastmoney_detail.rs` | Modify | Replace HTTP boilerplate |
| `crates/akshare/src/stock/eastmoney_spot.rs` | Modify | Replace HTTP boilerplate, query params |
| `crates/akshare/src/stock/eastmoney_fund_flow.rs` | Modify | Replace HTTP boilerplate, query params |
| `crates/akshare/src/stock/eastmoney_hot.rs` | Modify | Replace HTTP boilerplate |
| `crates/akshare/src/stock/zh_comparison.rs` | Modify | Replace HTTP boilerplate |
| `crates/akshare/src/stock/zh_index.rs` | Modify | Replace query params |
| `crates/akshare/src/stock/feature/fund_flow.rs` | Modify | Replace HTTP boilerplate |
| `crates/akshare/src/stock/feature/stock_other.rs` | Modify | Replace HTTP boilerplate |
| `crates/akshare/src/stock/feature/rank_ths.rs` | Modify | Replace HTTP boilerplate |
| `crates/akshare/src/stock/feature/types.rs` | Modify | Remove duplicate `HistData`, alias to shared type |
| `crates/akshare/src/provider/market_client/wire.rs` | Modify | Audit dead_code annotations |

---

## Task 1: Add `AdjustType` to `types/mod.rs`

**Files:**
- Modify: `crates/akshare/src/types/mod.rs`

- [ ] **Step 1: Add `AdjustType` enum after `MarketKind`**

```rust
/// Eastmoney adjust type (前复权/后复权/不复权).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdjustType {
    /// No adjustment (不复权)
    None,
    /// Forward adjustment (前复权)
    Forward,
    /// Backward adjustment (后复权)
    Backward,
}

impl AdjustType {
    /// Convert to Eastmoney API parameter value.
    pub fn to_eastmoney_str(self) -> &'static str {
        match self {
            Self::None => "0",
            Self::Forward => "1",
            Self::Backward => "2",
        }
    }

    /// Parse from user-facing string ("", "qfq", "hfq").
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

- [ ] **Step 2: Verify it compiles**

Run: `cargo check -p akshare`
Expected: Compiles without errors

- [ ] **Step 3: Commit**

```bash
git add crates/akshare/src/types/mod.rs
git commit -m "feat: add AdjustType enum to shared types"
```

---

## Task 2: Add HTTP helpers to `util.rs`

**Files:**
- Modify: `crates/akshare/src/util.rs`

- [ ] **Step 1: Add `send_and_check` function**

Add at the end of `util.rs`:

```rust
/// Send a request builder and check for HTTP errors.
///
/// This replaces the common pattern:
/// ```ignore
/// .send().await.map_err(Error::from)?
///     .error_for_status().map_err(Error::from)?
/// ```
pub(crate) async fn send_and_check(
    builder: reqwest::RequestBuilder,
) -> crate::Result<reqwest::Response> {
    builder
        .send()
        .await
        .map_err(crate::Error::from)?
        .error_for_status()
        .map_err(crate::Error::from)
}
```

- [ ] **Step 2: Add `eastmoney_clist_params` function**

```rust
/// Build Eastmoney clist query parameters.
///
/// Returns the common parameter block used by `push2.eastmoney.com/api/qt/clist/get`.
/// Callers pass `pz` (page size) and any extra fields specific to their endpoint.
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
```

- [ ] **Step 3: Add `eastmoney_f10_params` function**

```rust
/// Build Eastmoney F10/HSF10 query parameters.
pub(crate) fn eastmoney_f10_params<'a>(source: &'a str) -> Vec<(&'a str, &'a str)> {
    vec![("source", source), ("client", "PC")]
}
```

- [ ] **Step 4: Verify it compiles**

Run: `cargo check -p akshare`
Expected: Compiles without errors

- [ ] **Step 5: Commit**

```bash
git add crates/akshare/src/util.rs
git commit -m "feat: add HTTP helper functions to util.rs"
```

---

## Task 3: Replace `fqt` match pattern in stock modules

**Files:**
- Modify: `crates/akshare/src/stock/board_em.rs:131-136`
- Modify: `crates/akshare/src/stock/zh_a.rs:249-254`
- Modify: `crates/akshare/src/stock/zh_b.rs:168-173`
- Modify: `crates/akshare/src/stock/zh_kcb.rs:192-197`
- Modify: `crates/akshare/src/stock/hk_extra.rs:309-314`

- [ ] **Step 1: Update `board_em.rs` — replace fqt match**

In `stock_board_concept_hist` method, replace:
```rust
        let fqt = match adjust {
            "" => "0",
            "qfq" => "1",
            "hfq" => "2",
            _ => return Err(Error::invalid_input(format!("invalid adjust: {adjust}"))),
        };
```

With:
```rust
        let fqt = crate::types::AdjustType::from_adjust_str(adjust)?.to_eastmoney_str();
```

Also add import at top of file:
```rust
use crate::types::AdjustType;
```

- [ ] **Step 2: Update `board_em.rs` — second fqt match**

In `stock_board_industry_hist` method (around line 192), apply the same replacement.

- [ ] **Step 3: Update `zh_a.rs`**

In `stock_zh_a_daily` method (around line 249), replace the fqt match with:
```rust
        let fqt = AdjustType::from_adjust_str(adjust)?.to_eastmoney_str();
```

Add import:
```rust
use crate::types::AdjustType;
```

- [ ] **Step 4: Update `zh_b.rs`**

In `stock_zh_b_daily` method (around line 168), same replacement.
Add import: `use crate::types::AdjustType;`

- [ ] **Step 5: Update `zh_kcb.rs`**

In `stock_zh_kcb_daily` method (around line 192), same replacement.
Add import: `use crate::types::AdjustType;`

- [ ] **Step 6: Update `hk_extra.rs`**

In `hk_daily` method (around line 309), same replacement.
Add import: `use crate::types::AdjustType;`

- [ ] **Step 7: Verify compilation**

Run: `cargo check -p akshare --features all`
Expected: Compiles without errors

- [ ] **Step 8: Run tests**

Run: `cargo test -p akshare --features all -- --test-threads=4`
Expected: All tests pass

- [ ] **Step 9: Commit**

```bash
git add crates/akshare/src/stock/board_em.rs crates/akshare/src/stock/zh_a.rs crates/akshare/src/stock/zh_b.rs crates/akshare/src/stock/zh_kcb.rs crates/akshare/src/stock/hk_extra.rs
git commit -m "refactor: replace fqt match pattern with AdjustType in stock modules"
```

---

## Task 4: Replace HTTP boilerplate with `send_and_check`

**Files:**
- Modify: `crates/akshare/src/stock/board_em.rs`
- Modify: `crates/akshare/src/stock/eastmoney_detail.rs`
- Modify: `crates/akshare/src/stock/eastmoney_misc.rs`
- Modify: `crates/akshare/src/stock/eastmoney_spot.rs`
- Modify: `crates/akshare/src/stock/eastmoney_fund_flow.rs`
- Modify: `crates/akshare/src/stock/eastmoney_hot.rs`
- Modify: `crates/akshare/src/stock/hk_extra.rs`
- Modify: `crates/akshare/src/stock/us_extra.rs`
- Modify: `crates/akshare/src/stock/zh_comparison.rs`
- Modify: `crates/akshare/src/stock/feature/fund_flow.rs`
- Modify: `crates/akshare/src/stock/feature/stock_other.rs`
- Modify: `crates/akshare/src/stock/feature/rank_ths.rs`

- [ ] **Step 1: Update `board_em.rs` — first occurrence**

In `fetch_board_change` method (around line 260-265), replace:
```rust
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
```

With:
```rust
            ;
        let resp = crate::util::send_and_check(builder).await?;
```

Note: The `.query(&[...])` call returns a `RequestBuilder`, so we need to capture it and pass to `send_and_check`. The pattern is:

Before:
```rust
        let response = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                // ...
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
```

After:
```rust
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&[
                    ("pn", "1"),
                    // ...
                ]),
        )
        .await?;
```

- [ ] **Step 2: Update remaining occurrences in `board_em.rs`**

Apply the same pattern to all `.send().await.map_err(Error::from)?.error_for_status().map_err(Error::from)?` sequences in `board_em.rs`.

- [ ] **Step 3: Update `eastmoney_detail.rs`**

Apply `send_and_check` to all HTTP send boilerplate in this file. There are approximately 6 occurrences.

- [ ] **Step 4: Update `eastmoney_misc.rs`**

Apply `send_and_check` to all HTTP send boilerplate. There are approximately 10 occurrences.

- [ ] **Step 5: Update `eastmoney_spot.rs`**

Apply `send_and_check` to all HTTP send boilerplate. There are approximately 4 occurrences.

- [ ] **Step 6: Update `eastmoney_fund_flow.rs`**

Apply `send_and_check` to all HTTP send boilerplate. There are approximately 2 occurrences.

- [ ] **Step 7: Update `eastmoney_hot.rs`**

Apply `send_and_check` to all HTTP send boilerplate. There are approximately 2 occurrences.

- [ ] **Step 8: Update `hk_extra.rs`**

Apply `send_and_check` to all HTTP send boilerplate. There are approximately 6 occurrences.

- [ ] **Step 9: Update `us_extra.rs`**

Apply `send_and_check` to all HTTP send boilerplate. There are approximately 6 occurrences.

- [ ] **Step 10: Update `zh_comparison.rs`**

Apply `send_and_check` to all HTTP send boilerplate. There are approximately 2 occurrences.

- [ ] **Step 11: Update `feature/fund_flow.rs`**

Apply `send_and_check` to all HTTP send boilerplate. There are approximately 4 occurrences.

- [ ] **Step 12: Update `feature/stock_other.rs`**

Apply `send_and_check` to all HTTP send boilerplate. There are approximately 2 occurrences.

- [ ] **Step 13: Update `feature/rank_ths.rs`**

Apply `send_and_check` to all HTTP send boilerplate. There are approximately 1 occurrence.

- [ ] **Step 14: Verify compilation**

Run: `cargo check -p akshare --features all`
Expected: Compiles without errors

- [ ] **Step 15: Run tests**

Run: `cargo test -p akshare --features all -- --test-threads=4`
Expected: All tests pass

- [ ] **Step 16: Commit**

```bash
git add crates/akshare/src/stock/
git commit -m "refactor: replace HTTP boilerplate with send_and_check helper"
```

---

## Task 5: Replace Eastmoney query params with helpers

**Files:**
- Modify: `crates/akshare/src/stock/board_em.rs`
- Modify: `crates/akshare/src/stock/us_extra.rs`
- Modify: `crates/akshare/src/stock/hk_extra.rs`
- Modify: `crates/akshare/src/stock/zh_a.rs`
- Modify: `crates/akshare/src/stock/zh_index.rs`
- Modify: `crates/akshare/src/stock/eastmoney_spot.rs`
- Modify: `crates/akshare/src/stock/eastmoney_fund_flow.rs`
- Modify: `crates/akshare/src/stock/eastmoney_misc.rs`
- Modify: `crates/akshare/src/stock/zh_comparison.rs`

- [ ] **Step 1: Update `board_em.rs` — resolve_board_secid method**

In `resolve_board_secid` (around line 327-339), replace:
```rust
            .query(&[
                ("pn", "1"),
                ("pz", "5000"),
                ("po", "1"),
                ("np", "1"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f12"),
                ("fs", fs),
                ("fields", "f12,f14"),
            ])
```

With:
```rust
            .query(&crate::util::eastmoney_clist_params("5000", &[
                ("fid", "f12"),
                ("fs", fs),
                ("fields", "f12,f14"),
            ]))
```

- [ ] **Step 2: Update `board_em.rs` — fetch_board_change method**

In `fetch_board_change` (around line 249-259), apply same pattern with extra fields `("fid", "f62"), ("fs", fs), ("fields", "f2,f3,f4,f5,f6,f8,f12,f14,f62,f184")`.

- [ ] **Step 3: Update `us_extra.rs`**

Replace all `("pn","1"),("pz","5000"),("po","1"),("np","1"),("ut","bd1d9ddb...")` blocks with `eastmoney_clist_params` calls. There are approximately 4 occurrences.

- [ ] **Step 4: Update `hk_extra.rs`**

Replace all eastmoney clist param blocks. There are approximately 2 occurrences.

- [ ] **Step 5: Update `zh_a.rs`**

Replace eastmoney clist param block if present.

- [ ] **Step 6: Update `zh_index.rs`**

Replace eastmoney clist param block if present.

- [ ] **Step 7: Update `eastmoney_spot.rs`**

Replace eastmoney clist param blocks. There are approximately 2 occurrences.

- [ ] **Step 8: Update `eastmoney_fund_flow.rs`**

Replace eastmoney clist param blocks. There are approximately 2 occurrences.

- [ ] **Step 8b: Update `eastmoney_misc.rs` — F10 params**

In `eastmoney_misc.rs`, replace all `("source", "F10"), ("client", "PC")` or `("source", "HSF10"), ("client", "PC")` blocks with `eastmoney_f10_params("F10")` or `eastmoney_f10_params("HSF10")`. There are approximately 6 occurrences.

- [ ] **Step 8c: Update `zh_comparison.rs` — F10 params**

In `zh_comparison.rs`, replace all `("source", "HSF10"), ("client", "PC")` blocks with `eastmoney_f10_params("HSF10")`. There are approximately 2 occurrences.

- [ ] **Step 9: Verify compilation**

Run: `cargo check -p akshare --features all`
Expected: Compiles without errors

- [ ] **Step 10: Run tests**

Run: `cargo test -p akshare --features all -- --test-threads=4`
Expected: All tests pass

- [ ] **Step 11: Commit**

```bash
git add crates/akshare/src/stock/
git commit -m "refactor: replace eastmoney query params with helper function"
```

---

## Task 6: Replace `HistData` alias in `feature/types.rs`

**Files:**
- Modify: `crates/akshare/src/stock/feature/types.rs`

- [ ] **Step 1: Replace `HistData` with type alias**

In `feature/types.rs`, replace the `HistData` struct definition (lines 41-54):
```rust
/// Historical candlestick data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistData {
    pub trade_date: String,
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
```

With a type alias:
```rust
/// Historical candlestick data point.
///
/// This is a type alias for the shared [`crate::types::CandlePoint`] type.
pub type HistData = crate::types::CandlePoint;
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check -p akshare --features all`
Expected: Compiles without errors

- [ ] **Step 3: Run tests**

Run: `cargo test -p akshare --features all -- --test-threads=4`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add crates/akshare/src/stock/feature/types.rs
git commit -m "refactor: alias HistData to shared CandlePoint type"
```

---

## Task 7: Audit dead code in `wire.rs`

**Files:**
- Modify: `crates/akshare/src/provider/market_client/wire.rs`

- [ ] **Step 1: Check which `#[allow(dead_code)]` fields are actually used**

Run: `grep -rn "security_type_name\|\.code\b\|\.name\b\|\.total\b\|secucode\|security_code\|security_name_abbr\|report_type\|basic_eps\|diluted_eps\|report_date" crates/akshare/src/provider/market_client/ --include="*.rs" | grep -v "dead_code\|struct\|pub(crate)"`

This will show which fields are actually accessed.

- [ ] **Step 2: Remove truly dead fields**

For any field that is:
1. Deserialized from JSON but never read in any code path
2. Not needed for struct completeness

Remove the field AND the `#[allow(dead_code)]` annotation.

For fields that ARE needed for serde deserialization but never read:
- Keep the field
- Keep `#[serde(default)]` if present
- The `#[allow(dead_code)]` is appropriate — leave it

- [ ] **Step 3: Verify compilation**

Run: `cargo check -p akshare --features all`
Expected: Compiles without errors

- [ ] **Step 4: Run tests**

Run: `cargo test -p akshare --features all -- --test-threads=4`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/akshare/src/provider/market_client/wire.rs
git commit -m "refactor: clean up dead code annotations in wire.rs"
```

---

## Task 8: Final verification and duplicate_code rescan

- [ ] **Step 1: Run full test suite**

Run: `cargo test -p akshare --features all -- --test-threads=4`
Expected: All tests pass

- [ ] **Step 2: Run clippy**

Run: `cargo clippy -p akshare --features all -- -D warnings`
Expected: No warnings

- [ ] **Step 3: Run duplicate_code rescan**

Run: `cd crates/akshare/src && duplicate_code --minimum-successive-lines 6 --ignore-line-regex "^use " "^// " "^/// " "^#" "^$" 2>&1 | wc -l`
Expected: Significantly less than 30,593 lines

- [ ] **Step 4: Run cargo fmt**

Run: `cargo fmt -p akshare`
Expected: No changes needed (code already formatted)

- [ ] **Step 5: Final commit if needed**

```bash
git add -A
git commit -m "style: apply cargo fmt after deduplication"
```
