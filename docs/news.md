# News Module

Financial news from CCTV, Baidu, and other sources.

## Functions

- **news_cctv** — CCTV新闻联播文字稿
- **news_economic_baidu** — 百度经济新闻
- **news_report_time_baidu** — 百度新闻发稿时间
- **news_search** — 新闻搜索
- **news_trade_notify_dividend_baidu** — 百度交易提醒-分红
- **news_trade_notify_suspend_baidu** — 百度交易提醒-停牌

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();
let cctv = client.news_cctv("20240101").await?;
let search = client.news_search("A股").await?;
```
