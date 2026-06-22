# Commodity Module

The `commodity` module provides commodity market data.

## Functions (9 total)

### Carbon Trading
- **energy_carbon_bj** — Beijing carbon trading
- **energy_carbon_domestic** — Domestic carbon trading
- **energy_carbon_eu** — EU carbon trading
- **energy_carbon_gz** — Guangzhou carbon trading
- **energy_carbon_hb** — Hubei carbon trading

### Energy
- **energy_oil_hist** — Oil price history
- **energy_oil_detail** — Oil detail data

### Spot
- **futures_spot_stock_em** — Spot-stock comparison

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// Beijing carbon trading
let carbon = client.energy_carbon_bj().await?;

// Oil prices
let oil = client.energy_oil_hist().await?;
```
