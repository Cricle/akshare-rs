# Fund Module

The `fund` module provides comprehensive fund data including ETFs, LOFs, money market funds, and more.

## Functions (98 total)

### ETF Data
- **fund_etf_spot_em** — Real-time ETF spot data from Eastmoney
- **fund_etf_spot_ths** — ETF spot data from THS
- **fund_etf_category_ths** — ETF categories from THS
- **fund_etf_hist_em** — ETF historical data

### Fund Rankings
- **fund_open_fund_rank_em** — Open-end fund rankings
- **fund_money_rank_em** — Money market fund rankings
- **fund_lcx_rank_em** — LCX fund rankings
- **fund_hk_rank_em** — HK fund rankings

### Fund NAV
- **fund_open_fund_info_em** — Fund NAV history
- **fund_etf_fund_info_em** — ETF NAV history

### Fund Holdings
- **fund_portfolio_hold_em** — Fund portfolio holdings
- **fund_portfolio_change_em** — Portfolio changes

### Fund Managers
- **fund_manager** — Fund manager rankings

### Fund Scale
- **fund_scale_change_em** — Fund scale changes
- **fund_aum_trend_em** — AUM trends

### Fund Ratings
- **fund_rating** — Fund ratings from multiple agencies

### Fund Dividends
- **fund_fhsp_em** — Dividend/split data

### Fund Announcements
- **fund_announcement_dividend_em** — Dividend announcements
- **fund_announcement_report_em** — Report announcements

### Fund Fees
- **fund_fee_em** — Fund fee data

### Other Fund Data
- **fund_info_ths** — Fund info from THS
- **fund_xq** — Xueqiu fund data
- **fund_overview_em** — Fund overview
- **qdii** — QDII fund data

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// Get ETF spot data
let etf = client.fund_etf_spot_em().await?;

// Get fund NAV history
let nav = client.fund_open_fund_info_em("000001", "单位净值走势").await?;

// Get money market fund rankings
let money = client.fund_money_rank_em().await?;
```
