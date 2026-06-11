//! Eastmoney spot/list data — A-share, HK, US, boards, ST, new stocks, etc.
//!
//! Covers Python functions:
//! - `stock_zh_a_spot_em` — All A-share spot data
//! - `stock_zh_a_st_em` — ST / risk-warning stocks
//! - `stock_zh_a_new` — New stocks (次新股)
//! - `stock_staq_net_stop` — STAQ/NET delisted stocks
//! - `stock_hk_spot_em` — HK stock spot data
//! - `stock_us_spot_em` — US stock spot data
//! - `stock_board_concept_name_em` — Concept board names
//! - `stock_board_concept_cons_em` — Concept board constituents
//! - `stock_board_industry_name_em` — Industry board names
//! - `stock_board_industry_cons_em` — Industry board constituents
//! - `stock_zh_ah_spot_em` — AH stock comparison
//! - `stock_hsgt_sh_hk_spot_em` — HSGT stocks (Shanghai)
//! - `stock_hsgt_sz_hk_spot_em` — HSGT stocks (Shenzhen)

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ClistEnvelope {
    data: Option<ClistData>,
}

#[derive(Debug, Deserialize)]
struct ClistData {
    diff: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A single row from the Eastmoney spot data endpoint.
/// Fields are intentionally flexible (`serde_json::Value`) because different
/// endpoints return different subsets of fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotRow {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub amplitude_pct: Option<f64>,
    #[serde(default)]
    pub turnover_rate: Option<f64>,
    #[serde(default)]
    pub pe_ratio: Option<f64>,
    #[serde(default)]
    pub volume_ratio: Option<f64>,
    #[serde(default)]
    pub high: Option<f64>,
    #[serde(default)]
    pub low: Option<f64>,
    #[serde(default)]
    pub open: Option<f64>,
    #[serde(default)]
    pub prev_close: Option<f64>,
    #[serde(default)]
    pub total_market_cap: Option<f64>,
    #[serde(default)]
    pub float_market_cap: Option<f64>,
    #[serde(default)]
    pub pb_ratio: Option<f64>,
    #[serde(default)]
    pub market: Option<String>,
    /// Raw JSON for any fields not captured above.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// A board (concept or industry) summary row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardRow {
    pub board_code: String,
    pub board_name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub total_market_cap: Option<f64>,
    #[serde(default)]
    pub turnover_rate: Option<f64>,
    #[serde(default)]
    pub rising_count: Option<i64>,
    #[serde(default)]
    pub falling_count: Option<i64>,
    #[serde(default)]
    pub leading_stock: Option<String>,
    #[serde(default)]
    pub leading_stock_change_pct: Option<f64>,
}

/// AH stock comparison row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhComparisonRow {
    pub name: String,
    pub h_code: String,
    #[serde(default)]
    pub h_latest_price_hkd: Option<f64>,
    #[serde(default)]
    pub h_change_pct: Option<f64>,
    pub a_code: String,
    #[serde(default)]
    pub a_latest_price_rmb: Option<f64>,
    #[serde(default)]
    pub a_change_pct: Option<f64>,
    #[serde(default)]
    pub price_ratio: Option<f64>,
    #[serde(default)]
    pub premium: Option<f64>,
}

/// HSGT (沪深港通) stock row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsgtStockRow {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub open: Option<f64>,
    #[serde(default)]
    pub high: Option<f64>,
    #[serde(default)]
    pub low: Option<f64>,
    #[serde(default)]
    pub prev_close: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    // -- A-share spot data --------------------------------------------------

    /// Get all A-share spot data from Eastmoney (flexible row format).
    ///
    /// For the comprehensive typed version, see `feature::spot_em::stock_zh_a_spot_em`.
    pub async fn stock_zh_a_spot_em_flex(&self, limit: usize) -> Result<Vec<SpotRow>> {
        self.fetch_spot_list("m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048", limit)
            .await
    }

    /// Get ST / risk-warning stocks from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_a_st_em()`
    pub async fn stock_zh_a_st_em(&self, limit: usize) -> Result<Vec<SpotRow>> {
        self.fetch_spot_list("m:0 f:4,m:1 f:4", limit).await
    }

    /// Get new stocks (次新股) from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_a_new_em()`
    pub async fn stock_zh_a_new_em(&self, limit: usize) -> Result<Vec<SpotRow>> {
        self.fetch_spot_list("m:0+f:8,m:1+f:8", limit).await
    }

    /// Get STAQ/NET delisted stocks from Eastmoney.
    ///
    /// Python equivalent: `stock_staq_net_stop()`
    pub async fn stock_staq_net_stop(&self, limit: usize) -> Result<Vec<SpotRow>> {
        self.fetch_spot_list("m:0 s:3", limit).await
    }

    // -- HK stock spot data -------------------------------------------------

    /// Get HK stock spot data from Eastmoney (flexible row format).
    pub async fn stock_hk_spot_em_flex(&self, limit: usize) -> Result<Vec<SpotRow>> {
        self.fetch_spot_list("m:128+t:3,m:128+t:4,m:128+t:1,m:128+t:2", limit)
            .await
    }

    // -- US stock spot data -------------------------------------------------

    /// Get US stock spot data from Eastmoney (flexible row format).
    pub async fn stock_us_spot_em_flex(&self, limit: usize) -> Result<Vec<SpotRow>> {
        self.fetch_spot_list("m:105,m:106,m:107", limit).await
    }

    // -- Board data ---------------------------------------------------------

    /// Get concept board names from Eastmoney.
    ///
    /// Python equivalent: `stock_board_concept_name_em()`
    pub async fn stock_board_concept_name_em(&self, limit: usize) -> Result<Vec<BoardRow>> {
        self.fetch_board_list("m:90 t:3 f:!50", limit).await
    }

    /// Get concept board constituents from Eastmoney.
    ///
    /// Python equivalent: `stock_board_concept_cons_em(board_code)`
    pub async fn stock_board_concept_cons_em(
        &self,
        board_code: &str,
        limit: usize,
    ) -> Result<Vec<SpotRow>> {
        let fs = format!("b:{board_code}+f:!50");
        self.fetch_spot_list(&fs, limit).await
    }

    /// Get industry board names from Eastmoney.
    ///
    /// Python equivalent: `stock_board_industry_name_em()`
    pub async fn stock_board_industry_name_em(&self, limit: usize) -> Result<Vec<BoardRow>> {
        self.fetch_board_list("m:90 t:2 f:!50", limit).await
    }

    /// Get industry board constituents from Eastmoney.
    ///
    /// Python equivalent: `stock_board_industry_cons_em(board_code)`
    pub async fn stock_board_industry_cons_em(
        &self,
        board_code: &str,
        limit: usize,
    ) -> Result<Vec<SpotRow>> {
        let fs = format!("b:{board_code}+f:!50");
        self.fetch_spot_list(&fs, limit).await
    }

    // -- AH comparison ------------------------------------------------------

    /// Get AH stock comparison data from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_ah_spot_em()`
    pub async fn stock_zh_ah_spot_em(&self, limit: usize) -> Result<Vec<AhComparisonRow>> {
        let pz = limit.to_string();
        let response = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("np", "1"),
                ("fltt", "1"),
                ("invt", "2"),
                ("fs", "b:DLMK0101"),
                (
                    "fields",
                    "f193,f191,f192,f12,f13,f14,f1,f2,f4,f3,f152,f186,f190,f187,f189,f188",
                ),
                ("fid", "f3"),
                ("pn", "1"),
                ("pz", pz.as_str()),
                ("po", "1"),
                ("dect", "1"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: ClistEnvelope = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("eastmoney AH comparison missing data"))?;

        let items = parse_ah_rows(&diff, limit);
        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no AH comparison items",
            ));
        }
        Ok(items)
    }

    // -- HSGT stocks --------------------------------------------------------

    /// Get HSGT stocks (Shanghai -> HK) from Eastmoney.
    ///
    /// Python equivalent: `stock_hsgt_sh_hk_spot_em()`
    pub async fn stock_hsgt_sh_hk_spot_em(&self, limit: usize) -> Result<Vec<HsgtStockRow>> {
        self.fetch_hsgt_stocks("b:DLMK0144", limit).await
    }

    /// Get HSGT stocks (Shenzhen -> HK) from Eastmoney.
    ///
    /// Python equivalent: `stock_hsgt_sz_hk_spot_em()`
    pub async fn stock_hsgt_sz_hk_spot_em(&self, limit: usize) -> Result<Vec<HsgtStockRow>> {
        self.fetch_hsgt_stocks("b:DLMK0145", limit).await
    }

    // -- Private helpers ----------------------------------------------------

    /// Generic spot list fetcher using the Eastmoney clist API.
    async fn fetch_spot_list(&self, fs: &str, limit: usize) -> Result<Vec<SpotRow>> {
        let pz = limit.to_string();
        let response = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", pz.as_str()),
                ("po", "1"),
                ("np", "1"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f3"),
                ("fs", fs),
                (
                    "fields",
                    "f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f62",
                ),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: ClistEnvelope = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("eastmoney spot missing data"))?;

        let items = parse_spot_rows(&diff, limit);
        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no spot items"));
        }
        Ok(items)
    }

    /// Generic board list fetcher using the Eastmoney clist API.
    async fn fetch_board_list(&self, fs: &str, limit: usize) -> Result<Vec<BoardRow>> {
        let pz = limit.to_string();
        let response = self
                        .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", pz.as_str()),
                ("po", "1"),
                ("np", "1"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f3"),
                ("fs", fs),
                (
                    "fields",
                    "f2,f3,f4,f8,f12,f14,f15,f16,f17,f18,f20,f21,f24,f25,f22,f33,f11,f62,f128,f124,f107,f104,f105,f136",
                ),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: ClistEnvelope = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("eastmoney board list missing data"))?;

        let items = parse_board_rows(&diff, limit);
        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no board items"));
        }
        Ok(items)
    }

    /// Fetch HSGT stock list.
    async fn fetch_hsgt_stocks(&self, fs: &str, limit: usize) -> Result<Vec<HsgtStockRow>> {
        let pz = limit.to_string();
        let response = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("np", "1"),
                ("fltt", "1"),
                ("invt", "2"),
                ("fs", fs),
                (
                    "fields",
                    "f12,f13,f14,f19,f1,f2,f4,f3,f152,f17,f18,f15,f16,f5,f6",
                ),
                ("fid", "f12"),
                ("pn", "1"),
                ("pz", pz.as_str()),
                ("po", "1"),
                ("dect", "1"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: ClistEnvelope = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("eastmoney HSGT stocks missing data"))?;

        let items = parse_hsgt_rows(&diff, limit);
        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no HSGT stock items"));
        }
        Ok(items)
    }
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

fn get_f64(val: &serde_json::Value, key: &str) -> Option<f64> {
    val.get(key)?.as_f64()
}

fn get_i64(val: &serde_json::Value, key: &str) -> Option<i64> {
    val.get(key)?.as_i64()
}

fn get_string(val: &serde_json::Value, key: &str) -> Option<String> {
    val.get(key)?.as_str().map(|s| s.to_string())
}

fn parse_spot_rows(diff: &serde_json::Value, limit: usize) -> Vec<SpotRow> {
    let arr = match diff.as_array() {
        Some(a) => a,
        None => return vec![],
    };

    arr.iter()
        .take(limit)
        .filter_map(|item| {
            let code = get_string(item, "f12")?;
            let name = get_string(item, "f14")?;
            Some(SpotRow {
                code,
                name,
                latest_price: get_f64(item, "f2"),
                change_pct: get_f64(item, "f3"),
                change_amount: get_f64(item, "f4"),
                volume: get_f64(item, "f5"),
                amount: get_f64(item, "f6"),
                amplitude_pct: get_f64(item, "f7"),
                turnover_rate: get_f64(item, "f8"),
                pe_ratio: get_f64(item, "f9"),
                volume_ratio: get_f64(item, "f10"),
                high: get_f64(item, "f15"),
                low: get_f64(item, "f16"),
                open: get_f64(item, "f17"),
                prev_close: get_f64(item, "f18"),
                total_market_cap: get_f64(item, "f20"),
                float_market_cap: get_f64(item, "f21"),
                pb_ratio: get_f64(item, "f23"),
                market: get_string(item, "f13"),
                extra: std::collections::HashMap::new(),
            })
        })
        .collect()
}

fn parse_board_rows(diff: &serde_json::Value, limit: usize) -> Vec<BoardRow> {
    let arr = match diff.as_array() {
        Some(a) => a,
        None => return vec![],
    };

    arr.iter()
        .take(limit)
        .filter_map(|item| {
            let board_code = get_string(item, "f12")?;
            let board_name = get_string(item, "f14")?;
            Some(BoardRow {
                board_code,
                board_name,
                latest_price: get_f64(item, "f2"),
                change_pct: get_f64(item, "f3"),
                change_amount: get_f64(item, "f4"),
                total_market_cap: get_f64(item, "f20"),
                turnover_rate: get_f64(item, "f8"),
                rising_count: get_i64(item, "f104"),
                falling_count: get_i64(item, "f105"),
                leading_stock: get_string(item, "f128").or_else(|| get_string(item, "f136")),
                leading_stock_change_pct: get_f64(item, "f136").or_else(|| get_f64(item, "f33")),
            })
        })
        .collect()
}

fn parse_ah_rows(diff: &serde_json::Value, limit: usize) -> Vec<AhComparisonRow> {
    let arr = match diff.as_array() {
        Some(a) => a,
        None => return vec![],
    };

    arr.iter()
        .take(limit)
        .map(|item| {
            let name = get_string(item, "f193").unwrap_or_default();
            let h_code = get_string(item, "f12").unwrap_or_default();
            let a_code = get_string(item, "f191").unwrap_or_default();
            AhComparisonRow {
                name,
                h_code,
                h_latest_price_hkd: get_f64(item, "f2").map(|v| v / 1000.0),
                h_change_pct: get_f64(item, "f3").map(|v| v / 100.0),
                a_code,
                a_latest_price_rmb: get_f64(item, "f186").map(|v| v / 100.0),
                a_change_pct: get_f64(item, "f187").map(|v| v / 100.0),
                price_ratio: get_f64(item, "f189").map(|v| v / 100.0),
                premium: get_f64(item, "f188").map(|v| v / 100.0),
            }
        })
        .collect()
}

fn parse_hsgt_rows(diff: &serde_json::Value, limit: usize) -> Vec<HsgtStockRow> {
    let arr = match diff.as_array() {
        Some(a) => a,
        None => return vec![],
    };

    arr.iter()
        .take(limit)
        .filter_map(|item| {
            let code = get_string(item, "f12")?;
            let name = get_string(item, "f14")?;
            Some(HsgtStockRow {
                code,
                name,
                latest_price: get_f64(item, "f2").map(|v| v / 1000.0),
                change_amount: get_f64(item, "f4").map(|v| v / 1000.0),
                change_pct: get_f64(item, "f3").map(|v| v / 100.0),
                open: get_f64(item, "f17").map(|v| v / 1000.0),
                high: get_f64(item, "f15").map(|v| v / 1000.0),
                low: get_f64(item, "f16").map(|v| v / 1000.0),
                prev_close: get_f64(item, "f18").map(|v| v / 1000.0),
                volume: get_f64(item, "f5"),
                amount: get_f64(item, "f6"),
            })
        })
        .collect()
}
