# Fund Module

The `fund` module provides comprehensive fund data including ETFs, LOFs, money market funds, and more.

## Functions (98 total)

### ETF Data
- **fund_etf_spot_em** — Real-time ETF spot data from Eastmoney
- **fund_etf_spot_ths** — ETF spot data from THS
- **fund_etf_category_ths** — ETF categories from THS
- **fund_etf_hist_em** — ETF historical data

### Fund Rankings
- **fund_open_fund_rank** — Open-end fund rankings
- **fund_money_rank** — Money market fund rankings
- **fund_lcx_rank** — LCX fund rankings
- **fund_hk_rank** — HK fund rankings

### Fund NAV
- **fund_open_fund_info** — Fund NAV history
- **fund_etf_fund_info** — ETF NAV history

### Fund Holdings
- **fund_portfolio_hold** — Fund portfolio holdings
- **fund_portfolio_change** — Portfolio changes

### Fund Managers
- **fund_manager** — Fund manager rankings

### Fund Scale
- **fund_scale_change** — Fund scale changes
- **fund_aum_trend** — AUM trends

### Fund Ratings
- **fund_rating** — Fund ratings from multiple agencies

### Fund Dividends
- **fund_fh** — Fund dividend/split data
- **fund_fh_rank** — Fund dividend rankings

### Fund Announcements
- **fund_announcement_dividend** — Dividend announcements
- **fund_announcement_report** — Report announcements

### Fund Fees
- **fund_fee** — Fund fee data

### Other Fund Data
- **fund_info** — Fund info from THS
- **fund_xq** — Xueqiu fund data
- **fund_overview** — Fund overview
- **qdii** — QDII fund data

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// Get ETF spot data
let etf = client.fund_etf_spot().await?;

// Get fund NAV history
let nav = client.fund_open_fund_info("000001", "单位净值走势").await?;

// Get money market fund rankings
let money = client.fund_money_rank().await?;
```
