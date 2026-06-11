# Option Module

The `option` module provides options market data.

## Functions (48 total)

### Option Contracts
- **option_contract_info_ctp** — CTP contract info
- **option_current_day_sse** — SSE current day options
- **option_current_day_szse** — SZSE current day options

### Option Data
- **option_sse_sina** — SSE options from Sina
- **option_cffex_sina** — CFFEX options from Sina
- **option_commodity_sina** — Commodity options from Sina
- **option_comm_qihuo** — Commodity options from Qihuo
- **option_comm_info** — Commodity option info
- **option_comm_symbol** — Commodity option symbols

### Option Analysis
- **option_analysis_em** — Option analysis from Eastmoney
- **option_risk_indicator** — Risk indicators

### Option Billboard
- **option_lhb_em** — Option billboard data

### Margin
- **option_margin** — Option margin data
- **option_margin_symbol** — Option margin by symbol

### Daily Stats
- **option_daily_stats_sse** — SSE daily stats
- **option_daily_stats_szse** — SZSE daily stats

### Other
- **option_czce** — CZCE options
- **option_finance** — Option finance data

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// SSE options
let opts = client.option_current_day_sse("510050").await?;

// Option analysis
let analysis = client.option_analysis_em("510050", "认购").await?;
```
