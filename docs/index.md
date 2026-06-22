# Index Module

The `index` module provides index data from multiple sources.

## Functions (64 total)

### A-share Indices
- **index_zh_a_hist** — A-share index historical data
- **index_zh_a_hist_min_em** — A-share index minute data
- **index_code_id_map** — Code-to-ID mapping

### HK Indices
- **index_hk_spot_em** — HK index real-time
- **index_hk_spot_sina** — HK index real-time from Sina
- **index_hk_daily** — HK index daily data

### Global Indices
- **index_global_spot** — Global index real-time
- **index_global_hist_em** — Global index historical
- **index_global_hist_sina** — Global index from Sina
- **index_global_name_table** — Global index name table

### Shenwan Indices
- **sw_index_first_info** — Shenwan Level-1 info
- **sw_index_second_info** — Shenwan Level-2 info
- **sw_index_third_info** — Shenwan Level-3 info
- **sw_index_third_cons** — Shenwan Level-3 constituents
- **index_realtime_sw** — Shenwan real-time
- **index_hist_sw** — Shenwan historical
- **sw_index_daily_indicator** — Daily indicators

### CNIndex
- **index_cni** — CNIndex overview
- **index_cni_hist** — CNIndex historical

### CSIndex
- **index_csindex_hist** — CSIndex historical

### CX/Caixin
- **index_cx** — Caixin index data

### QVIX
- **index_option_50etf_qvix** — 50ETF QVIX daily
- **index_option_300etf_qvix** — 300ETF QVIX daily
- **index_option_500etf_qvix** — 500ETF QVIX daily
- **index_option_50etf_qvix_min** — 50ETF QVIX minute

### Other Indices
- **index_stock_hk** — HK stock index
- **index_cflp** — CFLP logistics index
- **index_sugar** — Sugar price index
- **index_hog** — Hog price index
- **index_eri** — Emission rights index
- **index_spot** — Spot goods index
- **index_drewry** — Drewry WCI index
- **index_kq_fz** — KQ FZ index
- **index_kq_ss** — KQ SS index
- **index_yw** — YW index

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// A-share index historical
let hist = client.index_zh_a_hist("000001", "daily", "20240101", "20241231").await?;

// Global index spot
let spot = client.index_global_spot().await?;

// Shenwan index info
let sw = client.sw_index_first_info().await?;
```
