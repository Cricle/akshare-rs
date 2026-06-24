# Feature Flags Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Cargo feature flags so users can compile only the modules they need.

**Architecture:** Two-layer feature system — fine-grained `mod-*` features per module, aggregated `equity`/`derivatives`/`funds`/`macro`/`fx-commodity`/`misc` category features, and an `all` meta-feature (default). Core types/client/error are always compiled. Modules are gated via `#[cfg(feature = "mod-xxx")]` on `pub mod` in `lib.rs`.

**Tech Stack:** Cargo features, `#[cfg(feature = "...")]` conditional compilation

---

### Task 1: Define feature flags in Cargo.toml

**Files:**
- Modify: `crates/akshare/Cargo.toml`

- [ ] **Step 1: Replace the `[features]` section**

Replace the entire `[features]` block (lines 16-21) with:

```toml
[features]
# --- Fine-grained module features ---
mod-stock = []
mod-index = []
mod-futures = []
mod-option = []
mod-fund = []
mod-bond = []
mod-reits = []
mod-macro_data = []
mod-economy = []
mod-forex = []
mod-crypto = []
mod-commodity = []
mod-spot = []
mod-news = []
mod-bank = []
mod-cal = []
mod-ta = []
mod-tool = []

# --- Category features (aggregated) ---
equity = ["mod-stock", "mod-index"]
derivatives = ["mod-futures", "mod-option"]
funds = ["mod-fund", "mod-bond", "mod-reits"]
macro = ["mod-macro_data", "mod-economy"]
fx-commodity = ["mod-forex", "mod-crypto", "mod-commodity", "mod-spot"]
misc = ["mod-news", "mod-bank", "mod-cal", "mod-ta", "mod-tool"]

# --- Market client (existing, unchanged) ---
market-client = [
    "dep:reqwest-middleware", "dep:reqwest-tracing", "dep:opentelemetry",
    "dep:tracing", "dep:futures", "dep:anyhow",
]

# --- Aggregation ---
all = ["equity", "derivatives", "funds", "macro", "fx-commodity", "misc", "market-client"]
default = ["all"]
```

- [ ] **Step 2: Verify it parses**

Run: `cargo metadata --manifest-path crates/akshare/Cargo.toml --format-version 1 > /dev/null`
Expected: exit 0 (no error)

- [ ] **Step 3: Commit**

```bash
git add crates/akshare/Cargo.toml
git commit -m "feat: define module and category feature flags in Cargo.toml"
```

---

### Task 2: Gate modules in lib.rs

**Files:**
- Modify: `crates/akshare/src/lib.rs`

- [ ] **Step 1: Add cfg attributes to all module declarations**

Replace the module declaration section (lines 115-151) with:

```rust
// Equity Markets
#[cfg(feature = "mod-stock")]
pub mod stock;
#[cfg(feature = "mod-index")]
pub mod index;

// Derivatives
#[cfg(feature = "mod-futures")]
pub mod futures;
#[cfg(feature = "mod-option")]
pub mod option;

// Funds & Fixed Income
#[cfg(feature = "mod-bond")]
pub mod bond;
#[cfg(feature = "mod-fund")]
pub mod fund;
#[cfg(feature = "mod-reits")]
pub mod reits;

// Macro & Economy
#[cfg(feature = "mod-economy")]
pub mod economy;
#[cfg(feature = "mod-macro_data")]
pub mod macro_data;

// FX, Crypto & Commodities
#[cfg(feature = "mod-commodity")]
pub mod commodity;
#[cfg(feature = "mod-crypto")]
pub mod crypto;
#[cfg(feature = "mod-forex")]
pub mod forex;
#[cfg(feature = "mod-spot")]
pub mod spot;

// Other
#[cfg(feature = "mod-bank")]
pub mod bank;
#[cfg(feature = "mod-cal")]
pub mod cal;
#[cfg(feature = "mod-news")]
pub mod news;
pub mod provider;
#[cfg(feature = "mod-ta")]
pub mod ta;
#[cfg(feature = "mod-tool")]
pub mod tool;

// Internal
mod client;
mod error;
pub mod market;
pub mod types;
mod util;
```

Note: `provider`, `types`, `client`, `error`, `market`, `util` are always compiled (core).

- [ ] **Step 2: Verify full build still works**

Run: `cargo check -p akshare`
Expected: exit 0 (default features = all, so everything compiles)

- [ ] **Step 3: Commit**

```bash
git add crates/akshare/src/lib.rs
git commit -m "feat: gate module declarations with cfg feature attributes"
```

---

### Task 3: Handle cross-module dependency (stock → news)

**Files:**
- Modify: `crates/akshare/src/stock/feature/stock_other.rs:571-659`

`stock/feature/stock_other.rs` has 4 public functions (`stock_news`, `stock_news_em_by_name`, `stock_news_em_hk`, `stock_news_em_us`) and 1 private function (`stock_news_em_inner`) that import from `crate::news::search`. When `mod-news` is disabled but `mod-stock` is enabled, these will fail to compile.

- [ ] **Step 1: Gate all 5 news functions with cfg**

In `crates/akshare/src/stock/feature/stock_other.rs`, wrap lines 571-659 (the 4 public functions + `stock_news_em_inner`) with `#[cfg(feature = "mod-news")]`. The result should look like:

```rust
    /// 东方财富-个股新闻 (A股, by stock code)
    #[cfg(feature = "mod-news")]
    pub async fn stock_news(&self, symbol: &str) -> Result<Vec<StockNews>> {
        self.stock_news_em_inner(symbol, "default", "default", 20)
            .await
    }

    /// 东方财富-个股新闻 (by company name, much better coverage)
    #[cfg(feature = "mod-news")]
    pub async fn stock_news_em_by_name(&self, name: &str) -> Result<Vec<StockNews>> {
        self.stock_news_em_inner(name, "default", "relevance", 50)
            .await
    }

    /// 东方财富-港股个股新闻
    #[cfg(feature = "mod-news")]
    pub async fn stock_news_em_hk(&self, symbol: &str) -> Result<Vec<StockNews>> {
        self.stock_news_em_inner(symbol, "default", "default", 20)
            .await
    }

    /// 东方财富-美股个股新闻
    #[cfg(feature = "mod-news")]
    pub async fn stock_news_em_us(&self, symbol: &str) -> Result<Vec<StockNews>> {
        self.stock_news_em_inner(symbol, "default", "default", 20)
            .await
    }

    #[cfg(feature = "mod-news")]
    async fn stock_news_em_inner(
        &self,
        keyword: &str,
        scope: &str,
        sort: &str,
        page_size: u32,
    ) -> Result<Vec<StockNews>> {
        // ... existing body unchanged from line 602 onward ...
```

Each of the 5 functions gets its own `#[cfg(feature = "mod-news")]` attribute. The body of `stock_news_em_inner` stays exactly as-is (lines 602-659).

- [ ] **Step 2: Verify stock compiles without news**

Run: `cargo check -p akshare --no-default-features --features mod-stock`
Expected: exit 0

- [ ] **Step 3: Verify stock+news still works**

Run: `cargo check -p akshare --no-default-features --features mod-stock,mod-news`
Expected: exit 0

- [ ] **Step 4: Commit**

```bash
git add crates/akshare/src/stock/feature/stock_other.rs
git commit -m "feat: gate stock_news_em on mod-news feature for independent compilation"
```

---

### Task 4: Verify each module compiles independently

**Files:** None (verification only)

- [ ] **Step 1: Test each module feature in isolation**

Run each of these commands, expecting exit 0 for all:

```bash
cargo check -p akshare --no-default-features --features mod-stock
cargo check -p akshare --no-default-features --features mod-index
cargo check -p akshare --no-default-features --features mod-futures
cargo check -p akshare --no-default-features --features mod-option
cargo check -p akshare --no-default-features --features mod-fund
cargo check -p akshare --no-default-features --features mod-bond
cargo check -p akshare --no-default-features --features mod-reits
cargo check -p akshare --no-default-features --features mod-macro_data
cargo check -p akshare --no-default-features --features mod-economy
cargo check -p akshare --no-default-features --features mod-forex
cargo check -p akshare --no-default-features --features mod-crypto
cargo check -p akshare --no-default-features --features mod-commodity
cargo check -p akshare --no-default-features --features mod-spot
cargo check -p akshare --no-default-features --features mod-news
cargo check -p akshare --no-default-features --features mod-bank
cargo check -p akshare --no-default-features --features mod-cal
cargo check -p akshare --no-default-features --features mod-ta
cargo check -p akshare --no-default-features --features mod-tool
```

- [ ] **Step 2: Test category features**

```bash
cargo check -p akshare --no-default-features --features equity
cargo check -p akshare --no-default-features --features derivatives
cargo check -p akshare --no-default-features --features funds
cargo check -p akshare --no-default-features --features macro
cargo check -p akshare --no-default-features --features fx-commodity
cargo check -p akshare --no-default-features --features misc
```

- [ ] **Step 3: Test all feature**

```bash
cargo check -p akshare --no-default-features --features all
```

- [ ] **Step 4: Test no features (core only)**

```bash
cargo check -p akshare --no-default-features
```

Expected: exit 0 (core types/client/error compile standalone)

- [ ] **Step 5: Fix any compilation errors found**

If any check fails, fix the issue (likely a missing cfg gate or cross-module dependency) and re-run the failed check.

- [ ] **Step 6: Commit if fixes were needed**

```bash
git add -u
git commit -m "fix: resolve cross-module dependency issues for feature flags"
```

---

### Task 5: Run existing tests with default features

**Files:** None (verification only)

- [ ] **Step 1: Run the test suite**

Run: `cargo test -p akshare`
Expected: all tests pass (no regressions from feature gating)

- [ ] **Step 2: Run tests with a category feature**

Run: `cargo test -p akshare --no-default-features --features equity`
Expected: only equity-related tests run and pass

- [ ] **Step 3: Commit if any test fixes needed**

```bash
git add -u
git commit -m "fix: adjust tests for feature flag compatibility"
```
