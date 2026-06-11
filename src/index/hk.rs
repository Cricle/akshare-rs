//! HK (港股) index data — spot quotes and daily candles from Sina and Eastmoney.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, HkIndexSpotItem};
use crate::util::*;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct EmClistEnvelope {
    data: Option<EmClistData>,
}

#[derive(Debug, Deserialize)]
struct EmClistData {
    diff: Option<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 新浪财经 — 港股指数实时行情.
    pub async fn index_hk_spot_sina(&self) -> Result<Vec<IndexSpotSina>> {
        let symbols = "hkCES100,hkCES120,hkCES280,hkCES300,hkCESA80,hkCESG10,\
            hkCESHKM,hkCSCMC,hkCSHK100,hkCSHKDIV,hkCSHKLC,hkCSHKLRE,hkCSHKMCS,\
            hkCSHKME,hkCSHKPE,hkCSHKSE,hkCSI300,hkCSRHK50,hkGEM,hkHKL,hkHSCCI,\
            hkHSCEI,hkHSI,hkHSMBI,hkHSMOGI,hkHSMPI,hkHSTECH,hkSSE180,hkSSE180GV,\
            hkSSE380,hkSSE50,hkSSECEQT,hkSSECOMP,hkSSEDIV,hkSSEITOP,hkSSEMCAP,\
            hkSSEMEGA,hkVHSI";
        let url = format!("https://hq.sinajs.cn/rn=mtf2t&list={symbols}");

        let body = self
                        .get(&url)
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        let mut items = Vec::new();
        for line in body.lines() {
            let data = match line.split_once('"').and_then(|(_, r)| r.rsplit_once('"')) {
                Some((d, _)) => d,
                None => continue,
            };
            let p: Vec<&str> = data.split(',').collect();
            if p.len() < 9 {
                continue;
            }
            items.push(IndexSpotSina {
                code: p[0].to_string(),
                name: p[1].to_string(),
                open: parse_f64_safe(p[2]),
                prev_close: parse_f64_safe(p[3]),
                high: parse_f64_safe(p[4]),
                low: parse_f64_safe(p[5]),
                close: parse_f64_safe(p[6]),
                change_amount: parse_f64_safe(p[7]),
                change_pct: parse_f64_safe(p[8]),
            });
        }

        if items.is_empty() {
            return Err(Error::not_found("sina returned no HK index spot data"));
        }
        Ok(items)
    }

    /// 东方财富 — 港股指数实时行情.
    pub async fn index_hk_spot_em(&self) -> Result<Vec<HkIndexSpotItem>> {
        let response = self
                        .get("https://15.push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", "200"),
                ("po", "1"),
                ("np", "1"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f3"),
                ("fs", "m:124,m:125,m:305"),
                ("fields", "f2,f3,f4,f5,f6,f12,f13,f14,f15,f16,f17,f18"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: EmClistEnvelope = response.json().await.map_err(Error::from)?;
        let diff = payload.data.and_then(|d| d.diff).unwrap_or_default();

        let items: Vec<HkIndexSpotItem> = diff
            .into_iter()
            .filter_map(|v| {
                let obj = v.as_object()?;
                Some(HkIndexSpotItem {
                    code: obj.get("f12")?.as_str()?.to_string(),
                    internal_id: obj.get("f13")?.as_f64()?.to_string(),
                    name: obj.get("f14")?.as_str().unwrap_or("").to_string(),
                    close: obj.get("f2")?.as_f64().unwrap_or(0.0),
                    change_pct: obj.get("f3")?.as_f64().unwrap_or(0.0),
                    change_amount: obj.get("f4")?.as_f64().unwrap_or(0.0),
                    volume: obj.get("f5")?.as_f64().unwrap_or(0.0),
                    amount: obj.get("f6")?.as_f64().unwrap_or(0.0),
                    high: obj.get("f15")?.as_f64().unwrap_or(0.0),
                    low: obj.get("f16")?.as_f64().unwrap_or(0.0),
                    open: obj.get("f17")?.as_f64().unwrap_or(0.0),
                    prev_close: obj.get("f18")?.as_f64().unwrap_or(0.0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no HK index spot data"));
        }
        Ok(items)
    }

    /// 东方财富 — 港股指数历史行情.
    ///
    /// `symbol` is the HK index code, e.g. "HSTECH", "HSTECF2L", "HSI".
    /// `internal_id` is the Eastmoney internal market id (e.g. "100", "124").
    /// Use [`index_hk_spot_em`] to discover the mapping.
    pub async fn index_hk_daily_em(
        &self,
        symbol: &str,
        internal_id: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let secid = format!("{internal_id}.{symbol}");
        self.eastmoney_klines(&secid, "qfq", limit).await
    }
}

/// Sina HK index spot item (simplified).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSpotSina {
    pub code: String,
    pub name: String,
    pub open: f64,
    pub prev_close: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub change_amount: f64,
    pub change_pct: f64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // HK index functions require network access.
    }
}
