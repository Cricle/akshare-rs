# Stock Module

The `stock` module provides comprehensive access to A-share, Hong Kong, and US stock market data.

## Sub-modules

### stock::feature (200+ functions)
Stock feature data from Eastmoney and other sources:

- **spot_em** — Real-time A-share spot data
- **hist_em** — Historical OHLCV data
- **hsgt_em** — Hong Kong/Shanghai/Shenzhen Connect data
- **lhb_em** — Billboard (龙虎榜) data
- **gdfx_em** — Shareholder analysis
- **comment_em** — Stock comments/ratings
- **analyst_em** — Analyst rankings
- **financial_em** — Financial indicators
- **margin_em** — Margin trading data
- **gpzy_em** — Share pledge data
- **sy_em** — Restricted shares
- **gdhs_em** — Shareholder count
- **gdzjc_em** — Shareholder changes
- **ztb_em** — Limit-up/down pools
- **pankou_em** — Order book data
- **zf_pg_em** — Allotment/dividend data
- **dxsyl_em** — IPO subscription data
- **yjbb_em** — Earnings reports
- **yjyg_em** — Earnings forecasts
- **fhps_em** — Dividend/split data
- **jgdy_em** — Institutional research
- **hot_xq** — Xueqiu hot stocks
- **inner_trade_xq** — Insider trading
- **esg_sina** — ESG ratings
- **irm_cninfo** — Investor Q&A
- **disclosure_cninfo** — Company disclosures
- **info_em** — News and info feeds
- **three_report_em** — Quarterly reports
- **report_em** — Research reports
- **lh_yybpm** — Broker rankings
- **stock_info** — Stock basic info
- **rank_ths** — THS rankings
- **register_em** — Registration info
- **fund_flow** — Capital flow
- **industry_cninfo** — Industry classification
- **stock_other** — PE/PB historical data
- **dzjy_em** — Block trades

### stock::fundamental (57 functions)
Fundamental data:
- Financial statements (balance sheet, profit, cash flow)
- IPO data, restricted releases
- Profit forecasts, share capital structure

### stock::board
Sector/concept board data from Eastmoney and THS

### stock::eastmoney_*
Various Eastmoney data endpoints:
- **eastmoney_spot** — A-share spot data
- **eastmoney_detail** — Stock detail pages
- **eastmoney_fund_flow** — Capital flow
- **eastmoney_hot** — Hot stocks
- **eastmoney_hsgt** — Connect data
- **eastmoney_misc** — Miscellaneous

### stock::zh_a / zh_b / zh_ah / zh_index / zh_kcb
Specific A-share market segments

### stock::hk / hk_extra
Hong Kong stock data:
- `hk_quote` — HK stock quote (Tencent → Yahoo fallback)
- `hk_candles` — HK stock candles (Tencent → Yahoo fallback)
- `hk_financial` — HK financial snapshot (PE, PB, EPS, market cap)
- `hk_search` — Search HK stocks via Eastmoney
- `hk_capital_flow` — HK stock capital flow via Eastmoney
- `hk_sector_rankings` — HK sector rankings (industry/concept)
- `hk_sector_capital_flow` — HK sector capital flow
- `hk_sector_constituents` — HK sector constituent stocks (client-side filter)

### stock::us / us_extra
US stock data with multiple source fallbacks:
- `us_quote` — US stock quote (derived from candles)
- `us_candles` — US stock candles (Sina → Yahoo → Stooq fallback)
- `us_search` — Search US stocks via Eastmoney
- `us_capital_flow` — US stock capital flow via Eastmoney
- `us_stock_profile` — US stock profile from Yahoo Finance
- `us_stock_key_stats` — Key financial stats from Yahoo Finance
- `us_sector_rankings` — US sector rankings (industry/concept)
- `us_sector_capital_flow` — US sector capital flow
- `us_sector_constituents` — US sector constituent stocks (client-side filter)

### stock::sina_stock
Sina Finance stock data

### stock::xueqiu
Xueqiu (Snowball) stock data

### stock::jin10
Jin10 financial data

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// A-share real-time quote
let quote = client.a_share_quote("600000").await?;

// Historical candles
let candles = client.a_share_candles("600000", "qfq", 60).await?;

// HK stock quote
let hk_quote = client.hk_quote("00593").await?;

// US stock candles
let us_candles = client.us_candles("AAPL", 30).await?;
```
