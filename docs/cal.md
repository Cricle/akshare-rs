# Cal Module

Realized volatility calculations for Chinese financial markets.

## Functions

- **rv_from_futures_zh_minute_sina** — 从新浪期货分钟数据计算已实现波动率
- **rv_from_stock_zh_a_hist_min_em** — 从东方财富A股分钟数据计算已实现波动率

Both functions compute various realized volatility estimators (standard RV, Yang-Zhang, Parkinson, Garman-Klass) from intraday data.

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();
let rv = client.rv_from_stock_zh_a_hist_min_em("000001", "2024-01-01", "2024-01-31").await?;
```
