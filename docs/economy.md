# Economy Module

Economic data: air quality, AMAC fund industry, articles/research, automotive, box office, Fortune/Forbes/Hurun rankings, FRED data, migration, NLP, and more.

## Functions (62 total)

### Air Quality (air_*)
- **air_city_table** — 城市空气质量列表
- **air_quality_hebei** — 河北空气质量
- **air_quality_hist** — 历史空气质量
- **air_quality_rank** — 空气质量排名
- **air_quality_watch_point** — 空气质量监测点

### AMAC Fund Industry (amac_*)
- **amac_manager_info** — 私募基金管理人信息
- **amac_fund_info** — 私募基金信息
- **amac_member_info** — 会员信息
- **amac_securities_info** — 证券信息
- ... and 10 more AMAC functions

### Articles & Research (article_*)
- **article_epu_index** — EPU经济政策不确定性指数
- **article_ff_crr** — Fama-French CRR因子
- **article_oman_rv** — Oman已实现波动率
- **article_rlab_rv** — Rlab已实现波动率

### Automotive (car_*)
- **car_market_total_cpca** — 乘用车市场总量
- **car_market_country_cpca** — 分国别市场
- **car_market_segment_cpca** — 分车型市场
- **car_sale_rank_gasgoo** — 盖世汽车销量排名

### Box Office (movie_*)
- **movie_boxoffice_daily** — 每日票房
- **movie_boxoffice_weekly** — 每周票房
- **movie_boxoffice_monthly** — 每月票房
- **movie_boxoffice_yearly** — 年度票房
- ... and 5 more box office functions

### Rankings
- **forbes_rank** — 福布斯排行榜
- **hurun_rank** — 胡润排行榜
- **xincaifu_rank** — 新财富排行榜
- **business_value_artist** — 商业价值艺人榜

### FRED Data
- **fred_md** — FRED Monthly Data
- **fred_qd** — FRED Quarterly Data

### Migration
- **migration_area_baidu** — 百度迁徙-城市间迁徙
- **migration_scale_baidu** — 百度迁徙-迁徙规模

### NLP
- **nlp_answer** — NLP问答
- **nlp_ownthink** — Ownthink知识图谱

### Other
- **game_hot_rank_taptap** — TapTap游戏热度排行
- **sunrise_daily** — 每日日出日落
- **sunrise_monthly** — 每月日出日落
- **video_tv** — 电视剧排行
- **video_variety_show** — 综艺节目排行
- **online_value_artist** — 明星商业价值榜

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();
let rank = client.forbes_rank("real_time").await?;
let box = client.movie_boxoffice_daily().await?;
```
