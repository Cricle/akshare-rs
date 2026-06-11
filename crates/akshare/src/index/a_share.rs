//! A-share index data — candles, spot quotes from Sina and Eastmoney.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, IndexSpotItem};

/// Map common A-share index names to Eastmoney secid.
fn index_secid(symbol: &str) -> Result<String> {
    let s = symbol.trim().to_uppercase();
    match s.as_str() {
        "000300" | "399300" | "CSI300" | "CSI_300" | "沪深300" => Ok("1.000300".to_string()),
        "000016" | "SSE50" | "SSE_50" | "上证50" => Ok("1.000016".to_string()),
        "000905" | "399905" | "CSI500" | "CSI_500" | "中证500" => Ok("1.000905".to_string()),
        "000001" | "399001" | "上证指数" | "SHCOMP" => Ok("1.000001".to_string()),
        "399006" | "创业板指" | "CHINEXT" => Ok("0.399006".to_string()),
        "000688" | "科创50" | "STAR50" => Ok("1.000688".to_string()),
        _ => {
            // Try to use as-is secid format (e.g. "1.000300")
            if s.contains('.') && s.len() >= 3 {
                Ok(s)
            } else {
                Err(Error::invalid_input(format!(
                    "unknown A-share index: {symbol}"
                )))
            }
        }
    }
}

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
// Eastmoney series map
// ---------------------------------------------------------------------------

const EM_SERIES: &[(&str, &str)] = &[
    ("沪深重要指数", "b:MK0010"),
    ("上证系列指数", "m:1+t:1"),
    ("深证系列指数", "m:0 t:5"),
    ("指数成份", "m:1+s:3,m:0+t:5"),
    ("中证系列指数", "m:2"),
];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get A-share index candles.
    ///
    /// `symbol` can be a code like "000300", a name like "CSI300" / "沪深300",
    /// or a raw Eastmoney secid like "1.000300".
    pub async fn index_a_share_candles(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let secid = index_secid(symbol)?;
        self.eastmoney_klines(&secid, "qfq", limit).await
    }

    /// 东方财富 — A股指数实时行情 (分系列).
    ///
    /// `series` is one of: "沪深重要指数", "上证系列指数", "深证系列指数",
    /// "指数成份", "中证系列指数".
    pub async fn index_stock_zh_spot_em(&self, series: &str) -> Result<Vec<IndexSpotItem>> {
        let fs = EM_SERIES
            .iter()
            .find(|(name, _)| *name == series)
            .map(|(_, fs)| *fs)
            .ok_or_else(|| Error::invalid_input(format!("unknown series: {series}")))?;

        let response = self
            .get("https://48.push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", "200"),
                ("po", "1"),
                ("np", "1"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f12"),
                ("fs", fs),
                ("fields", "f2,f3,f4,f5,f6,f7,f12,f14,f15,f16,f17,f18"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: EmClistEnvelope = response.json().await.map_err(Error::from)?;
        let diff = payload.data.and_then(|d| d.diff).unwrap_or_default();

        let items: Vec<IndexSpotItem> = diff
            .into_iter()
            .filter_map(|v| {
                let obj = v.as_object()?;
                Some(IndexSpotItem {
                    code: obj.get("f12")?.as_str()?.to_string(),
                    name: obj.get("f14")?.as_str().unwrap_or("").to_string(),
                    close: obj.get("f2")?.as_f64().unwrap_or(0.0),
                    change_pct: obj.get("f3")?.as_f64().unwrap_or(0.0),
                    change_amount: obj.get("f4")?.as_f64().unwrap_or(0.0),
                    volume: obj.get("f5")?.as_f64().unwrap_or(0.0),
                    amount: obj.get("f6")?.as_f64().unwrap_or(0.0),
                    amplitude_pct: obj.get("f7")?.as_f64().unwrap_or(0.0),
                    high: obj.get("f15")?.as_f64().unwrap_or(0.0),
                    low: obj.get("f16")?.as_f64().unwrap_or(0.0),
                    open: obj.get("f17")?.as_f64().unwrap_or(0.0),
                    prev_close: obj.get("f18")?.as_f64().unwrap_or(0.0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no A-share index spot data",
            ));
        }
        Ok(items)
    }

    /// 新浪财经 — A股所有指数实时行情.
    ///
    /// Returns a paginated list of all A-share index spot data from Sina.
    /// Note: Heavy scraping may trigger IP bans.
    pub async fn index_stock_zh_spot_sina(&self) -> Result<Vec<IndexSpotSinaItem>> {
        // Get page count
        let count_resp = self
                        .get("http://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeStockCountSimple")
            .query(&[("node", "hs_s")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let count_text = count_resp.text().await.map_err(Error::from)?;
        let count: i64 = count_text
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse()
            .unwrap_or(0);
        let pages = ((count as f64) / 80.0).ceil() as i64;

        let mut all_items = Vec::new();
        for page in 1..=pages {
            let resp = self
                                .get("http://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeDataSimple")
                .query(&[
                    ("page", page.to_string().as_str()),
                    ("num", "80"),
                    ("sort", "symbol"),
                    ("asc", "1"),
                    ("node", "hs_s"),
                    ("_s_r_a", "page"),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let items: Vec<IndexSpotSinaItem> = resp.json().await.map_err(Error::from)?;
            all_items.extend(items);
        }

        if all_items.is_empty() {
            return Err(Error::not_found("sina returned no A-share index spot data"));
        }
        Ok(all_items)
    }

    /// 新浪财经 — A股指数历史行情 (日线).
    ///
    /// `symbol` uses Sina format, e.g. "sh000300", "sz399006".
    pub async fn stock_zh_index_daily(&self, symbol: &str) -> Result<Vec<CandlePoint>> {
        // Sina uses JS-encoded data; try to parse the response directly
        let url =
            format!("https://finance.sina.com.cn/realstock/company/{symbol}/hisdata/klc_kl.js");
        let response = self
            .get(&url)
            .query(&[("d", "2020_2_4")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let body = response.text().await.map_err(Error::from)?;
        // The response is JS-encoded; we can't decode it without a JS runtime.
        // Return an error suggesting alternatives.
        if body.contains("function d(") || body.contains("eval(") {
            return Err(Error::upstream(
                "sina index daily data requires JS decoding; \
                 use index_a_share_candles() via Eastmoney instead",
            ));
        }

        Err(Error::upstream(format!(
            "unexpected sina response format for {symbol}"
        )))
    }
}

/// Sina A-share index spot item (from JSON API).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSpotSinaItem {
    pub symbol: String,
    pub name: String,
    #[serde(default)]
    pub trade: Option<String>,
    #[serde(default)]
    pub pricechange: Option<String>,
    #[serde(default)]
    pub changepercent: Option<String>,
    #[serde(default)]
    pub buy: Option<String>,
    #[serde(default)]
    pub sell: Option<String>,
    #[serde(default)]
    pub settlement: Option<String>,
    #[serde(default)]
    pub open: Option<String>,
    #[serde(default)]
    pub high: Option<String>,
    #[serde(default)]
    pub low: Option<String>,
    #[serde(default)]
    pub volume: Option<String>,
    #[serde(default)]
    pub amount: Option<String>,
}
