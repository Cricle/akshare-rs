# Bond Module

The `bond` module provides bond market data from multiple sources.

## Functions (47 total)

### Government Bonds
- **bond_zh_us_rate** — China/US government bond yields
- **bond_china_close_return** — ChinaMoney yield curves
- **bond_china_close_return_map** — Current yield curve snapshot

### Convertible Bonds
- **bond_zh_cov** — Convertible bond list
- **bond_zh_cov_info** — Convertible bond info
- **bond_zh_cov_info_ths** — Convertible bond info from THS
- **bond_zh_cov_value_analysis** — Convertible bond valuation

### Corporate Bonds
- **bond_corporate_issue** — Corporate bond issuance
- **bond_info_cm** — ChinaMoney bond info query
- **bond_info_cm_query** — ChinaMoney query parameters
- **bond_info_detail_cm** — ChinaMoney bond details

### Bond Spot
- **bond_spot_quote** — Bond spot quotes
- **bond_spot_deal** — Bond spot deals

### Bond Indices
- **bond_cb_index_jsl** — Convertible bond index
- **bond_cb_jsl** — Convertible bond data from JSL

### Bond Buyback
- **bond_sh_buy_back** — Shanghai bond buyback
- **bond_sz_buy_back** — Shenzhen bond buyback
- **bond_buy_back_hist** — Buyback history

### Bond Summary
- **bond_cash_summary** — SSE cash bond summary
- **bond_deal_summary** — SSE bond deal summary

### Bond Issuance
- **bond_treasury_index_cbond** — Treasury index from CBond
- **bond_local_government_issue** — Local government bonds

### Other Bond Data
- **bond_gb_zh** — Chinese government bond data
- **bond_cb_profile** — Convertible bond profile from Sina
- **bond_cb_summary** — Convertible bond summary from Sina
- **bond_nafmii** — NAFMII data
- **bond_em_rate** — Eastmoney rate data

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// China/US government bond yields
let rates = client.bond_zh_us_rate("中国国债").await?;

// Convertible bond list
let cov = client.bond_zh_cov().await?;

// ChinaMoney yield curves
let yields = client.bond_china_close_return("国债", "1", "20240101", "20240201").await?;
```
