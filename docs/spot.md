# Spot Module

Spot market prices: Shanghai Gold Exchange (SGE), hog prices, agricultural commodities.

## Functions (14 total)

### Shanghai Gold Exchange (SGE)
- **spot_golden_benchmark_sge** — 上海金基准价
- **spot_silver_benchmark_sge** — 上海银基准价
- **spot_hist_sge** — SGE历史行情
- **spot_quotations_sge** — SGE实时报价

### Hog Prices (soozhu)
- **spot_hog_soozhu** — 生猪价格
- **spot_hog_lean_price_soozhu** — 瘦肉型生猪价格
- **spot_hog_crossbred_soozhu** — 三元杂交生猪价格
- **spot_hog_three_way_soozhu** — 三元猪价格
- **spot_hog_year_trend_soozhu** — 生猪年度走势

### Agricultural
- **spot_corn_price_soozhu** — 玉米价格
- **spot_soybean_price_soozhu** — 大豆价格
- **spot_mixed_feed_soozhu** — 混合饲料价格

### QH Spot
- **spot_price_qh** — 奇货可查现货价格
- **spot_price_table_qh** — 奇货可查现货价格表

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();
let gold = client.spot_golden_benchmark_sge().await?;
let hog = client.spot_hog_soozhu().await?;
```
