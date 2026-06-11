# Futures Module

The `futures` module provides futures market data from multiple exchanges.

## Functions (109 total)

### Daily Bars
- **futures_main_sina** — Main contract daily data from Sina
- **futures_daily_bar** — Daily bar data from exchanges
- **futures_hist_em** — Historical futures data from Eastmoney

### Realtime Quotes
- **futures_zh_realtime_sina** — Realtime quotes from Sina
- **futures_foreign_commodity_realtime_sina** — Foreign commodity quotes

### Klines
- **futures_zh_minute_sina** — Minute klines from Sina
- **futures_global_spot_em** — Global spot klines
- **futures_global_hist_em** — Global historical klines

### Inventory & Delivery
- **futures_inventory_em** — Inventory data from Eastmoney
- **futures_inventory_99** — Inventory from 99qihuo
- **futures_to_spot** — Futures-to-spot comparison
- **futures_delivery** — Delivery data

### Settlement
- **futures_settle** — Settlement prices
- **futures_settlement_price_sgx** — SGX settlement prices

### Contract Info
- **futures_contract_detail** — Contract details
- **futures_comm_ctp** — CTP commodity info
- **futures_comm_js** — JS commodity info

### Position & COT
- **futures_cot** — Commitment of Traders
- **futures_position_rank** — Position rankings

### Indices
- **futures_index_ccidx** — CCIndex futures index
- **futures_index_sina** — Sina futures index

### Warehouse & Receipt
- **futures_warehouse_receipt** — Warehouse receipts
- **futures_rule** — Exchange rules

### News
- **futures_news_shmet** — SHMET news

### Roll Yield
- **futures_roll_yield** — Roll yield calculations

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// Main contract daily data
let bars = client.futures_main_sina("RB0").await?;

// Realtime quotes
let quotes = client.futures_zh_realtime_sina(&["RB2410"]).await?;

// Inventory data
let inventory = client.futures_inventory_em().await?;
```
