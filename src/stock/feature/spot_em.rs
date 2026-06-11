//! Spot market data from Eastmoney (stock_zh_a_spot_em, stock_sh_a_spot_em, etc.)

use crate::client::AkShareClient;
use crate::error::Result;
use super::helpers::*;
use super::types::SpotQuote;

const SPOT_FIELDS: &str = "f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f24,f25,f22,f11,f62";

fn parse_spot_quote(v: &serde_json::Value) -> Option<SpotQuote> {
    let code = v.get("f12")?.as_str()?.to_string();
    let name = v.get("f14")?.as_str()?.to_string();
    Some(SpotQuote {
        code,
        name,
        latest_price: json_f64(v, "f2"),
        change_pct: json_f64(v, "f3"),
        change_amount: json_f64(v, "f4"),
        volume: json_f64(v, "f5"),
        amount: json_f64(v, "f6"),
        amplitude_pct: json_f64(v, "f7"),
        turnover_rate: json_f64(v, "f8"),
        pe_dynamic: json_f64(v, "f9"),
        volume_ratio: json_f64(v, "f10"),
        high: json_f64(v, "f15"),
        low: json_f64(v, "f16"),
        open: json_f64(v, "f17"),
        prev_close: json_f64(v, "f18"),
        total_market_cap: json_f64(v, "f20"),
        circulating_market_cap: json_f64(v, "f21"),
        speed: json_f64(v, "f22"),
        pb: json_f64(v, "f23"),
        change_60d: json_f64(v, "f24"),
        change_ytd: json_f64(v, "f25"),
        change_5min: json_f64(v, "f62"),
    })
}

impl AkShareClient {
    /// 东方财富网-沪深京 A 股-实时行情
    pub async fn stock_zh_a_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch(
                "m:0 t:6,m:0 t:80,m:1 t:2,m:1 t:23,m:0 t:81 s:2048",
                SPOT_FIELDS,
                "5000",
                "f12",
            )
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-沪 A 股-实时行情
    pub async fn stock_sh_a_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:1 t:2,m:1 t:23", SPOT_FIELDS, "5000", "f12")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-深 A 股-实时行情
    pub async fn stock_sz_a_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:0 t:6,m:0 t:80", SPOT_FIELDS, "5000", "f12")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-京 A 股-实时行情
    pub async fn stock_bj_a_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:0 t:81 s:2048", SPOT_FIELDS, "5000", "f12")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-新股-实时行情
    pub async fn stock_new_a_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:0 f:8,m:1 f:8", SPOT_FIELDS, "5000", "f26")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-创业板-实时行情
    pub async fn stock_cy_a_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:0 t:80", SPOT_FIELDS, "5000", "f12")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-科创板-实时行情
    pub async fn stock_kc_a_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:1 t:23", SPOT_FIELDS, "5000", "f12")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-AB 股比价
    pub async fn stock_zh_ab_comparison_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:0 t:6,m:0 t:80,m:1 t:2", SPOT_FIELDS, "5000", "f12")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-B 股-实时行情
    pub async fn stock_zh_b_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:0 t:7,m:1 t:3", SPOT_FIELDS, "5000", "f12")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-港股-实时行情
    pub async fn stock_hk_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:128 t:3,m:128 t:4", SPOT_FIELDS, "5000", "f12")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-港股主板-实时行情
    pub async fn stock_hk_main_board_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:128 t:3", SPOT_FIELDS, "5000", "f12")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }

    /// 东方财富网-美股-实时行情
    pub async fn stock_us_spot_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch("m:105,m:106", SPOT_FIELDS, "5000", "f12")
            .await?;
        Ok(items.iter().filter_map(parse_spot_quote).collect())
    }
}
