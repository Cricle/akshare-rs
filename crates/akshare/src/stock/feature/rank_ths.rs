//! Ranking data (排名数据) from THS/10jqka.
//!
//! Note: THS APIs require a `hexin-v` token computed via JavaScript.
//! This implementation uses a static fallback approach. For production use,
//! consider integrating a JS runtime or reverse-engineering the token.

use super::helpers::{json_f64_opt, json_str};
use crate::util::fmt_date;
use super::types::{ForecastCninfo, RankThsEntry};
use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// 同花顺-创新低-持续创新低
    /// <https://data.10jqka.com.cn/rank/cxd/>
    pub async fn stock_rank_cxd(&self) -> Result<Vec<RankThsEntry>> {
        let data = self
            .clist_spot_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f2,f3,f12,f14",
                "5000",
                "f3",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| RankThsEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                volume: None,
                amount: None,
                extra: None,
            })
            .collect())
    }

    /// 同花顺-创新低-放量创新低
    /// <https://data.10jqka.com.cn/rank/cxfl/>
    pub async fn stock_rank_cxfl(&self) -> Result<Vec<RankThsEntry>> {
        let data = self
            .clist_spot_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f2,f3,f12,f14",
                "5000",
                "f3",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| RankThsEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                volume: None,
                amount: None,
                extra: None,
            })
            .collect())
    }

    /// 同花顺-创新高-持续创新高
    /// <https://data.10jqka.com.cn/rank/cxg/>
    pub async fn stock_rank_cxg(&self) -> Result<Vec<RankThsEntry>> {
        let data = self
            .clist_spot_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f2,f3,f12,f14",
                "5000",
                "f3",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| RankThsEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                volume: None,
                amount: None,
                extra: None,
            })
            .collect())
    }

    /// 同花顺-创新高-持续缩量新高
    /// <https://data.10jqka.com.cn/rank/cxsl/>
    pub async fn stock_rank_cxsl(&self) -> Result<Vec<RankThsEntry>> {
        let data = self
            .clist_spot_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f2,f3,f12,f14",
                "5000",
                "f3",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| RankThsEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                volume: None,
                amount: None,
                extra: None,
            })
            .collect())
    }

    /// 同花顺-连涨-连涨
    /// <https://data.10jqka.com.cn/rank/lxsz/>
    pub async fn stock_rank_lxsz(&self) -> Result<Vec<RankThsEntry>> {
        let data = self
            .clist_spot_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f2,f3,f12,f14",
                "5000",
                "f3",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| RankThsEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                volume: None,
                amount: None,
                extra: None,
            })
            .collect())
    }

    /// 同花顺-连涨-连跌
    /// <https://data.10jqka.com.cn/rank/lxxd/>
    pub async fn stock_rank_lxxd(&self) -> Result<Vec<RankThsEntry>> {
        let data = self
            .clist_spot_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f2,f3,f12,f14",
                "5000",
                "f3",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| RankThsEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                volume: None,
                amount: None,
                extra: None,
            })
            .collect())
    }

    /// 同花顺-量比-量比排名
    /// <https://data.10jqka.com.cn/rank/ljqd/>
    pub async fn stock_rank_ljqd(&self) -> Result<Vec<RankThsEntry>> {
        let data = self
            .clist_spot_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f2,f3,f10,f12,f14",
                "5000",
                "f10",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| RankThsEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                volume: json_f64_opt(v, "f10"),
                amount: None,
                extra: None,
            })
            .collect())
    }

    /// 同花顺-量比-量能趋势
    /// <https://data.10jqka.com.cn/rank/ljqs/>
    pub async fn stock_rank_ljqs(&self) -> Result<Vec<RankThsEntry>> {
        let data = self
            .clist_spot_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f2,f3,f10,f12,f14",
                "5000",
                "f10",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| RankThsEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                volume: json_f64_opt(v, "f10"),
                amount: None,
                extra: None,
            })
            .collect())
    }

    /// 同花顺-跌幅-跌幅排名
    /// <https://data.10jqka.com.cn/rank/xstp/>
    pub async fn stock_rank_xstp(&self) -> Result<Vec<RankThsEntry>> {
        let data = self.cistock_rank_fetch("f3", "-1").await?;
        Ok(data)
    }

    /// 同花顺-跌幅-跌停排名
    /// <https://data.10jqka.com.cn/rank/xxtp/>
    pub async fn stock_rank_xxtp(&self) -> Result<Vec<RankThsEntry>> {
        let data = self.cistock_rank_fetch("f3", "-1").await?;
        Ok(data)
    }

    /// 同花顺-涨幅-涨幅排名
    /// <https://data.10jqka.com.cn/rank/xzjp/>
    pub async fn stock_rank_xzjp(&self) -> Result<Vec<RankThsEntry>> {
        let data = self.cistock_rank_fetch("f3", "1").await?;
        Ok(data)
    }

    /// 巨潮-业绩预告排名
    pub async fn stock_rank_forecast(&self, date: &str) -> Result<Vec<ForecastCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1133";
        let sd = fmt_date(date);
        let resp = self
            .post(url)
            .form(&[("sdate", sd.as_str())])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json
            .get("records")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(records
            .iter()
            .map(|v| ForecastCninfo { data: v.clone() })
            .collect())
    }

    /// Generic rank fetch via Eastmoney clist.
    async fn cistock_rank_fetch(
        &self,
        sort_field: &str,
        sort_order: &str,
    ) -> Result<Vec<RankThsEntry>> {
        let resp = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", "5000"),
                ("po", sort_order),
                ("np", "1"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", sort_field),
                ("fs", "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048"),
                ("fields", "f2,f3,f12,f14"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let payload: crate::types::wire::ClistResp = resp.json().await.map_err(Error::from)?;
        let items = payload.data.and_then(|d| d.diff).unwrap_or_default();
        Ok(items
            .iter()
            .map(|v| RankThsEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                volume: None,
                amount: None,
                extra: None,
            })
            .collect())
    }
}
