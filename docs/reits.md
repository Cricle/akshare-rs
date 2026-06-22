# REITs Module

Real Estate Investment Trusts (REITs) data.

## Functions

- **reits_list** — REITs列表
- **reits_realtime** — REITs实时行情
- **reits_hist** — REITs历史行情
- **reits_hist_em** — REITs历史行情(东方财富)
- **reits_hist_min** — REITs分钟行情(东方财富)

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();
let list = client.reits_list().await?;
let realtime = client.reits_realtime().await?;
```
