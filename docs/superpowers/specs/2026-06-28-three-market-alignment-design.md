# Three-Market Interface Alignment Design

**Date**: 2026-06-28
**Scope**: Add search, capital flow, and sector data for HK and US markets using Eastmoney as the single data source.

## Background

The akshare-rs library covers three equity markets (A-share, HK, US) but the interfaces are not symmetric:

| Feature | A-share | HK | US |
|---------|---------|----|----|
| Search | `a_share_search` | — | — |
| Capital flow | `a_share_capital_flow` | — | — |
| Sector rankings | `a_share_sector_rankings` | — | — |
| Sector constituents | `a_share_sector_constituents` | — | — |
| Sector capital flow | `a_share_sector_capital_flow` | — | — |

The goal is to add the missing HK and US equivalents using Eastmoney's existing APIs, which already support multi-market data.

## Key Finding

The underlying Eastmoney APIs already handle HK and US data:

- **Search**: `eastmoney_search` returns HK/US results (classify `"HK"`, `"UsStock"`). Only convenience wrappers are missing.
- **Capital flow**: `eastmoney_capital_flow` accepts any Eastmoney `secid`. HK uses `116.XXXXX`, US uses `105.XXXX`. Only convenience wrappers are missing.
- **Sector data**: A-share uses `m:90+t:2` / `m:90+t:3` for Eastmoney's own sector classification. HK/US use different classification systems (Hang Seng / GICS). The Eastmoney clist API parameters for HK/US sectors need investigation.

## Approach: Thin Wrapper Layer

Add `hk_*` and `us_*` convenience methods to `hk.rs`, `hk_extra.rs`, `us.rs`, and `us_extra.rs` that delegate to the existing Eastmoney provider functions.

### 1. Search

**Files**: `crates/akshare/src/stock/hk.rs`, `crates/akshare/src/stock/us.rs`

Add methods:

```rust
// hk.rs
impl AkShareClient {
    pub async fn hk_search(&self, query: &str, limit: usize) -> Result<Vec<StockSearchResult>> {
        self.eastmoney_search(query, Some("港股"), limit).await
    }
}

// us.rs
impl AkShareClient {
    pub async fn us_search(&self, query: &str, limit: usize) -> Result<Vec<StockSearchResult>> {
        self.eastmoney_search(query, Some("美股"), limit).await
    }
}
```

**Notes**:
- Reuses the existing `eastmoney_search` with market filter.
- The market filter values (`"港股"`, `"美股"`) match `classify_search_market` output.
- No new types needed.

### 2. Capital Flow

**Files**: `crates/akshare/src/stock/hk.rs`, `crates/akshare/src/stock/us.rs`

Add methods:

```rust
// hk.rs
impl AkShareClient {
    pub async fn hk_capital_flow(&self, symbol: &str, limit: usize) -> Result<Vec<CapitalFlowPoint>> {
        let code = symbol.trim_start_matches('0');
        let code = if code.is_empty() { "0" } else { code };
        let secid = format!("116.{code}");
        self.eastmoney_capital_flow(&secid, limit).await
    }
}

// us.rs
impl AkShareClient {
    pub async fn us_capital_flow(&self, symbol: &str, limit: usize) -> Result<Vec<CapitalFlowPoint>> {
        let secid = format!("105.{}", symbol.to_uppercase());
        self.eastmoney_capital_flow(&secid, limit).await
    }
}
```

**Notes**:
- HK secid format: `116.{code}` (matches `hk_extra.rs` line 303).
- US secid format: `105.{symbol}` (matches `us_extra.rs` line 243).
- Reuses `CapitalFlowPoint` type from `types.rs`.

### 3. Sector Data

**Status**: Needs investigation.

A-share sector data uses Eastmoney's internal sector classification (`m:90+t:2` for industry, `m:90+t:3` for concept). HK and US markets use different classification systems:

- **HK**: Hang Seng Industry Classification (HSIC). Eastmoney may use different market codes for HK sectors.
- **US**: GICS (Global Industry Classification Standard). Eastmoney may use different market codes for US sectors.

**Approach**:
1. Probe Eastmoney's `push2.eastmoney.com/api/qt/clist/get` endpoint with HK/US sector `fs` parameters.
2. If Eastmoney supports HK/US sectors, add `hk_sector_rankings` / `us_sector_rankings` and related methods.
3. If not, document as a limitation and defer to a future iteration.

**Tentative methods** (if supported):

```rust
// hk_extra.rs
impl AkShareClient {
    pub async fn hk_sector_rankings(&self, sector_type: &str, limit: usize) -> Result<Vec<SectorSnapshot>> { ... }
    pub async fn hk_sector_constituents(&self, sector_code: &str, limit: usize) -> Result<Vec<SectorConstituent>> { ... }
    pub async fn hk_sector_capital_flow(&self, sector_code: &str, limit: usize) -> Result<Vec<CapitalFlowPoint>> { ... }
}

// us_extra.rs
impl AkShareClient {
    pub async fn us_sector_rankings(&self, sector_type: &str, limit: usize) -> Result<Vec<SectorSnapshot>> { ... }
    pub async fn us_sector_constituents(&self, sector_code: &str, limit: usize) -> Result<Vec<SectorConstituent>> { ... }
    pub async fn us_sector_capital_flow(&self, sector_code: &str, limit: usize) -> Result<Vec<CapitalFlowPoint>> { ... }
}
```

## Implementation Plan

### Phase 1: Search + Capital Flow (confirmed, no unknowns)

1. Add `hk_search` to `hk.rs`
2. Add `us_search` to `us.rs`
3. Add `hk_capital_flow` to `hk.rs`
4. Add `us_capital_flow` to `us.rs`
5. Add tests for all four methods
6. Update `docs/stock.md` with new API entries

### Phase 2: Sector Data (needs investigation)

1. Probe Eastmoney API for HK/US sector parameters
2. If supported, implement `hk_sector_*` / `us_sector_*` methods
3. If not supported, document limitation and stop

## Files Modified

| File | Changes |
|------|---------|
| `crates/akshare/src/stock/hk.rs` | Add `hk_search`, `hk_capital_flow` |
| `crates/akshare/src/stock/us.rs` | Add `us_search`, `us_capital_flow` |
| `crates/akshare/src/stock/hk_extra.rs` | Add `hk_sector_*` (Phase 2, if supported) |
| `crates/akshare/src/stock/us_extra.rs` | Add `us_sector_*` (Phase 2, if supported) |
| `docs/stock.md` | Document new APIs |

## Testing

- Unit tests: verify secid format construction for HK (`116.XXXXX`) and US (`105.XXXX`)
- Integration tests: call search/capital flow with real API (behind `#[cfg(feature = "network-tests")]` or similar)
- Existing 486 tests must continue to pass

## Out of Scope

- Unifying the fallback chains across markets (A-share 3-level, HK 2-level, US 3-level)
- Fixing stub functions (Eniu, Drewry, CNINFO, etc.)
- Adding features that only exist for one market (billboard, announcements, pink sheet) to other markets
