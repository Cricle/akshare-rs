//! Sina global index data — name table and daily history.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct SinaGlobalEnvelope {
    result: Option<SinaGlobalResult>,
}

#[derive(Debug, Deserialize)]
struct SinaGlobalResult {
    data: Option<Vec<SinaGlobalRow>>,
}

#[derive(Debug, Deserialize)]
struct SinaGlobalRow {
    #[serde(default)]
    d: String,
    #[serde(default)]
    o: String,
    #[serde(default)]
    h: String,
    #[serde(default)]
    l: String,
    #[serde(default)]
    c: String,
    #[serde(default)]
    v: String,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 新浪财经 — 环球市场指数名称-代码映射表.
    pub fn index_global_name_table(&self) -> Vec<GlobalSinaNameEntry> {
        GLOBAL_SINA_SYMBOL_MAP
            .iter()
            .map(|(name, code)| GlobalSinaNameEntry {
                name: name.to_string(),
                code: code.to_string(),
            })
            .collect()
    }

    /// 新浪财经 — 环球市场历史行情.
    ///
    /// `name` is the Chinese name from `index_global_name_table()`.
    pub async fn index_global_hist_sina(&self, name: &str) -> Result<Vec<GlobalSinaHistPoint>> {
        let code = GLOBAL_SINA_SYMBOL_MAP
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, c)| *c)
            .ok_or_else(|| Error::invalid_input(format!("unknown global index name: {name}")))?;

        let response = self
            .get("https://gi.finance.sina.com.cn/hq/daily")
            .query(&[("symbol", code), ("num", "10000")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: SinaGlobalEnvelope = response.json().await.map_err(Error::from)?;
        let rows = payload.result.and_then(|r| r.data).unwrap_or_default();

        let points: Vec<GlobalSinaHistPoint> = rows
            .into_iter()
            .map(|r| GlobalSinaHistPoint {
                date: r.d,
                open: r.o.parse().unwrap_or(0.0),
                high: r.h.parse().unwrap_or(0.0),
                low: r.l.parse().unwrap_or(0.0),
                close: r.c.parse().unwrap_or(0.0),
                volume: r.v.parse().unwrap_or(0.0),
            })
            .collect();

        if points.is_empty() {
            return Err(Error::not_found("sina returned no global index history"));
        }
        Ok(points)
    }
}

/// Name-code entry for Sina global indices.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalSinaNameEntry {
    pub name: String,
    pub code: String,
}

/// Sina global index history point.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalSinaHistPoint {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// Static mapping: Chinese name -> Sina symbol code.
const GLOBAL_SINA_SYMBOL_MAP: &[(&str, &str)] = &[
    ("英国富时100指数", "UKX"),
    ("德国DAX 30种股价指数", "DAX"),
    ("俄罗斯MICEX指数", "INDEXCF"),
    ("法CAC40指数", "CAC"),
    ("瑞士股票指数", "SWI20"),
    ("富时意大利MIB指数", "FTSEMIB"),
    ("荷兰AEX综合指数", "AEX"),
    ("西班牙IBEX指数", "IBEX"),
    ("欧洲Stoxx50指数", "SX5E"),
    ("加拿大S&P/TSX综合指数", "GSPTSE"),
    ("墨西哥BOLSA指数", "MXX"),
    ("巴西BOVESPA股票指数", "IBOV"),
    ("中国台湾加权指数", "TWJQ"),
    ("日经225指数", "NKY"),
    ("首尔综合指数", "KOSPI"),
    ("印度尼西亚雅加达综合指数", "JCI"),
    ("印度孟买SENSEX指数", "SENSEX"),
    ("澳大利亚标准普尔200指数", "AS51"),
    ("新西兰NZSE 50指数", "NZ250"),
    ("埃及CASE 30指数", "CASE"),
];

#[cfg(test)]
mod tests {
    #[test]
    fn test_global_sina_map_not_empty() {
        assert!(!super::GLOBAL_SINA_SYMBOL_MAP.is_empty());
    }
}
