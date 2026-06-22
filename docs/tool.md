# Tool Module

Utility functions: trade calendar, Tushare Pro API.

## Functions

- **tool_trade_date_hist** — 历史交易日历 (from Sina)
- **pro_api** — Tushare Pro API通用接口

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();
let dates = client.tool_trade_date_hist().await?;
```
