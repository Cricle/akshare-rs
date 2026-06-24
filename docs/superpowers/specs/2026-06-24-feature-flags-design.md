# Feature Flags Design

Date: 2026-06-24

## Goal

Allow users to compile only the modules they need via Cargo feature flags, reducing compile time and binary size.

## Approach

Hybrid two-layer feature system: fine-grained module features + aggregated category features.

## Feature Hierarchy

### Core (always compiled, never gated)

- `types`, `client`, `error`, `market`, `util`, `provider` (base)

### Module Features (fine-grained)

| Feature | Module |
|---------|--------|
| `mod-stock` | `stock` |
| `mod-index` | `index` |
| `mod-futures` | `futures` |
| `mod-option` | `option` |
| `mod-fund` | `fund` |
| `mod-bond` | `bond` |
| `mod-reits` | `reits` |
| `mod-macro_data` | `macro_data` |
| `mod-economy` | `economy` |
| `mod-forex` | `forex` |
| `mod-crypto` | `crypto` |
| `mod-commodity` | `commodity` |
| `mod-spot` | `spot` |
| `mod-news` | `news` |
| `mod-bank` | `bank` |
| `mod-cal` | `cal` |
| `mod-ta` | `ta` |
| `mod-tool` | `tool` |

### Category Features (aggregated)

| Feature | Includes |
|---------|----------|
| `equity` | `mod-stock` + `mod-index` |
| `derivatives` | `mod-futures` + `mod-option` |
| `funds` | `mod-fund` + `mod-bond` + `mod-reits` |
| `macro` | `mod-macro_data` + `mod-economy` |
| `fx-commodity` | `mod-forex` + `mod-crypto` + `mod-commodity` + `mod-spot` |
| `misc` | `mod-news` + `mod-bank` + `mod-cal` + `mod-ta` + `mod-tool` |

### Top-level Aggregation

```toml
all = equity + derivatives + funds + macro + fx-commodity + misc + market-client
default = ["all"]  # backward compatible
```

## Code Changes

### lib.rs

Each module gated with `#[cfg(feature = "mod-xxx")]`:

```rust
#[cfg(feature = "mod-stock")]
pub mod stock;
#[cfg(feature = "mod-index")]
pub mod index;
// ...
```

### client.rs

Methods gated by their owning module's feature:

```rust
#[cfg(feature = "mod-stock")]
impl AkShareClient {
    pub async fn a_share_quote(...) { ... }
}
```

### types/mod.rs, error.rs

Always compiled — shared foundation for all modules.

## User Consumption

```toml
# Default: full (backward compatible)
akshare = "0.1"

# Category: equity only
akshare = { version = "0.1", default-features = false, features = ["equity"] }

# Fine-grained: A-share + TA
akshare = { version = "0.1", default-features = false, features = ["mod-stock", "mod-ta"] }

# Scenario: macro research
akshare = { version = "0.1", default-features = false, features = ["macro", "fx-commodity"] }
```

## Testing & CI

- Each module feature verified independently: `cargo check --no-default-features --features mod-stock`
- Full build verified: `cargo check --no-default-features --features all`
- No cross-module feature dependencies (each feature compiles independently)
- Existing tests gated by their module's feature automatically

## Non-goals

- Runtime feature switching (this is compile-time only)
- Feature-gating individual functions within a module
- Cross-module feature dependencies
