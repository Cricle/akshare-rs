//! Eastmoney global index data — spot quotes and daily history.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::CandlePoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct EmGlobalEnvelope {
    data: Option<EmGlobalData>,
}

#[derive(Debug, Deserialize)]
struct EmGlobalData {
    diff: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 东方财富 — 全球指数实时行情.
    pub async fn index_global_spot(&self) -> Result<Vec<GlobalEmSpotItem>> {
        let fs = "i:1.000001,i:0.399001,i:0.399005,i:0.399006,i:1.000300,\
            i:100.HSI,i:100.HSCEI,i:124.HSCCI,i:100.TWII,i:100.N225,\
            i:100.KOSPI200,i:100.KS11,i:100.STI,i:100.SENSEX,i:100.KLSE,\
            i:100.SET,i:100.PSI,i:100.KSE100,i:100.VNINDEX,i:100.JKSE,\
            i:100.CSEALL,i:100.SX5E,i:100.FTSE,i:100.MCX,i:100.AXX,\
            i:100.FCHI,i:100.GDAXI,i:100.RTS,i:100.IBEX,i:100.PSI20,\
            i:100.OMXC20,i:100.BFX,i:100.AEX,i:100.WIG,i:100.OMXSPI,\
            i:100.SSMI,i:100.HEX,i:100.OSEBX,i:100.ATX,i:100.MIB,\
            i:100.ASE,i:100.ICEXI,i:100.PX,i:100.ISEQ,i:100.DJIA,\
            i:100.SPX,i:100.NDX,i:100.TSX,i:100.BVSP,i:100.MXX,\
            i:100.AS51,i:100.AORD,i:100.NZ50,i:100.UDI,i:100.BDI,i:100.CRB";

        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&[
                ("np", "2"),
                ("fltt", "1"),
                ("invt", "2"),
                ("fs", fs),
                ("fields", "f12,f14,f2,f3,f4,f7,f15,f16,f17,f18,f124"),
                ("fid", "f3"),
                ("pn", "1"),
                ("pz", "200"),
                ("po", "1"),
                ("dect", "1"),
                ])
        )
        .await?;

        let payload: EmGlobalEnvelope = response.json().await.map_err(Error::from)?;
        let diff = payload.data.and_then(|d| d.diff).unwrap_or_default();

        // diff may be an object (keyed by index) or an array
        let entries: Vec<(String, serde_json::Value)> = match &diff {
            serde_json::Value::Object(map) => {
                map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
            }
            serde_json::Value::Array(arr) => arr
                .iter()
                .enumerate()
                .map(|(i, v)| (i.to_string(), v.clone()))
                .collect(),
            _ => vec![],
        };

        let items: Vec<GlobalEmSpotItem> = entries
            .into_iter()
            .filter_map(|(_, v)| {
                let obj = v.as_object()?;
                Some(GlobalEmSpotItem {
                    code: obj.get("f12")?.as_str()?.to_string(),
                    name: obj.get("f14")?.as_str().unwrap_or("").to_string(),
                    close: obj.get("f2")?.as_f64().unwrap_or(0.0) / 100.0,
                    change_pct: obj.get("f3")?.as_f64().unwrap_or(0.0) / 100.0,
                    change_amount: obj.get("f4")?.as_f64().unwrap_or(0.0) / 100.0,
                    amplitude_pct: obj.get("f7")?.as_f64().unwrap_or(0.0) / 100.0,
                    high: obj.get("f15")?.as_f64().unwrap_or(0.0) / 100.0,
                    low: obj.get("f16")?.as_f64().unwrap_or(0.0) / 100.0,
                    open: obj.get("f17")?.as_f64().unwrap_or(0.0) / 100.0,
                    prev_close: obj.get("f18")?.as_f64().unwrap_or(0.0) / 100.0,
                    timestamp: obj.get("f124")?.as_i64().unwrap_or(0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no global index spot data",
            ));
        }
        Ok(items)
    }

    /// 东方财富 — 全球指数历史行情.
    ///
    /// `market` and `code` come from the `index_global_em_symbol_map` mapping,
    /// e.g. market="100", code="SPX" for S&P 500.
    pub async fn index_global_hist_em(
        &self,
        market: &str,
        code: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let secid = format!("{market}.{code}");
        self.eastmoney_klines(&secid, "qfq", limit).await
    }
}

/// Eastmoney global index spot item.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalEmSpotItem {
    pub code: String,
    pub name: String,
    pub close: f64,
    pub change_pct: f64,
    pub change_amount: f64,
    pub amplitude_pct: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub prev_close: f64,
    pub timestamp: i64,
}

/// Well-known global index symbol map (Eastmoney).
///
/// Maps Chinese name -> (market_id, code).
#[must_use]
pub fn global_em_symbol_map(name: &str) -> Option<(&str, &str)> {
    match name {
        "上证指数" => Some(("1", "000001")),
        "深证成指" => Some(("0", "399001")),
        "中小100" => Some(("0", "399005")),
        "创业板指" => Some(("0", "399006")),
        "沪深300" => Some(("1", "000300")),
        "恒生指数" => Some(("100", "HSI")),
        "国企指数" => Some(("100", "HSCEI")),
        "红筹指数" => Some(("124", "HSCCI")),
        "台湾加权" => Some(("100", "TWII")),
        "日经225" => Some(("100", "N225")),
        "韩国KOSPI200" => Some(("100", "KOSPI200")),
        "韩国KOSPI" => Some(("100", "KS11")),
        "富时新加坡海峡时报" => Some(("100", "STI")),
        "印度孟买SENSEX" => Some(("100", "SENSEX")),
        "富时马来西亚KLCI" => Some(("100", "KLSE")),
        "泰国SET" => Some(("100", "SET")),
        "菲律宾马尼拉" => Some(("100", "PSI")),
        "巴基斯坦卡拉奇" => Some(("100", "KSE100")),
        "越南胡志明" => Some(("100", "VNINDEX")),
        "印尼雅加达综合" => Some(("100", "JKSE")),
        "斯里兰卡科伦坡" => Some(("100", "CSEALL")),
        "欧洲斯托克50" => Some(("100", "SX5E")),
        "英国富时100" => Some(("100", "FTSE")),
        "英国富时250" => Some(("100", "MCX")),
        "富时AIM全股" => Some(("100", "AXX")),
        "法国CAC40" => Some(("100", "FCHI")),
        "德国DAX30" => Some(("100", "GDAXI")),
        "俄罗斯RTS" => Some(("100", "RTS")),
        "西班牙IBEX35" => Some(("100", "IBEX")),
        "葡萄牙PSI20" => Some(("100", "PSI20")),
        "OMX哥本哈根20" => Some(("100", "OMXC20")),
        "比利时BFX" => Some(("100", "BFX")),
        "荷兰AEX" => Some(("100", "AEX")),
        "波兰WIG" => Some(("100", "WIG")),
        "瑞典OMXSPI" => Some(("100", "OMXSPI")),
        "瑞士SMI" => Some(("100", "SSMI")),
        "芬兰赫尔辛基" => Some(("100", "HEX")),
        "挪威OSEBX" => Some(("100", "OSEBX")),
        "奥地利ATX" => Some(("100", "ATX")),
        "富时意大利MIB" => Some(("100", "MIB")),
        "希腊雅典ASE" => Some(("100", "ASE")),
        "冰岛ICEX" => Some(("100", "ICEXI")),
        "布拉格指数" => Some(("100", "PX")),
        "爱尔兰综合" => Some(("100", "ISEQ")),
        "道琼斯" => Some(("100", "DJIA")),
        "标普500" => Some(("100", "SPX")),
        "纳斯达克" => Some(("100", "NDX")),
        "加拿大S&P/TSX" => Some(("100", "TSX")),
        "巴西BOVESPA" => Some(("100", "BVSP")),
        "墨西哥BOLSA" => Some(("100", "MXX")),
        "澳大利亚标普200" => Some(("100", "AS51")),
        "澳大利亚普通股" => Some(("100", "AORD")),
        "新西兰50" => Some(("100", "NZ50")),
        "美元指数" => Some(("100", "UDI")),
        "波罗的海BDI指数" => Some(("100", "BDI")),
        "路透CRB商品指数" => Some(("100", "CRB")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_em_symbol_map() {
        assert_eq!(global_em_symbol_map("恒生指数"), Some(("100", "HSI")));
        assert_eq!(global_em_symbol_map("标普500"), Some(("100", "SPX")));
        assert_eq!(global_em_symbol_map("不存在"), None);
    }
}
