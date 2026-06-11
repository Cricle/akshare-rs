# Provider Module

Data provider abstractions for Eastmoney, Sina, Tencent, Yahoo, Stooq, Tushare, and SEC EDGAR.

## Functions (12 total)

### Eastmoney
- **eastmoney_search** — 东方财富股票搜索
- **eastmoney_klines** — 东方财富K线数据
- **eastmoney_announcements** — 东方财富公告列表
- **eastmoney_announcement_detail** — 东方财富公告详情
- **eastmoney_billboard** — 东方财富龙虎榜
- **eastmoney_billboard_seats** — 东方财富龙虎榜营业部
- **eastmoney_capital_flow** — 东方财富资金流向
- **eastmoney_sector_rankings** — 东方财富板块排名
- **eastmoney_sector_constituents** — 东方财富板块成分股
- **eastmoney_sector_capital_flow** — 东方财富板块资金流

### Sina
- **sina_a_share_realtime** — 新浪A股实时行情
- **sina_us_daily** — 新浪美股日线

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();
let results = client.eastmoney_search("平安银行").await?;
let klines = client.eastmoney_klines("600000", "daily", 60).await?;
```
