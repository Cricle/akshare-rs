//! Index data — spot, daily, CSIndex value.
//!
//! Covers Python functions:
//! - `stock_zh_index_spot_em` — Index spot from Eastmoney
//! - `stock_zh_index_daily_em` — Index daily from Eastmoney
//! - `stock_zh_index_daily_tx` — Index daily from Tencent
//! - `stock_zh_index_spot_sina` — Index spot from Sina
//! - `stock_zh_index_value_csindex` — CSIndex value data

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Index spot quote from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSpotEm {
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
    pub high: Option<f64>,
    #[serde(default)]
    pub low: Option<f64>,
    #[serde(default)]
    pub open: Option<f64>,
    #[serde(default)]
    pub prev_close: Option<f64>,
    #[serde(default)]
    pub internal_id: Option<String>,
}

/// Index daily candle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDailyCandle {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    #[serde(default)]
    pub volume: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
}

/// Index spot quote from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSpotSina {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub prev_close: Option<f64>,
    #[serde(default)]
    pub open: Option<f64>,
    #[serde(default)]
    pub high: Option<f64>,
    #[serde(default)]
    pub low: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
}

/// CSIndex value data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsIndexValue {
    pub date: String,
    #[serde(default)]
    pub index_code: Option<String>,
    #[serde(default)]
    pub index_name: Option<String>,
    #[serde(default)]
    pub open: Option<f64>,
    #[serde(default)]
    pub high: Option<f64>,
    #[serde(default)]
    pub low: Option<f64>,
    #[serde(default)]
    pub close: Option<f64>,
    #[serde(default)]
    pub change: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub sample_count: Option<f64>,
    #[serde(default)]
    pub pe_ttm: Option<f64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get index spot data from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_index_spot_em()`
    pub async fn stock_zh_index_spot_em(&self) -> Result<Vec<IndexSpotEm>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            diff: Option<Vec<serde_json::Value>>,
        }
        let response = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", "5000"),
                ("po", "1"),
                ("np", "1"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f3"),
                ("fs", "m:1+s:2,m:0+t:5,m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23"),
                ("fields", "f2,f3,f4,f5,f6,f12,f13,f14,f15,f16,f17,f18"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("eastmoney index spot missing data"))?;

        let items: Vec<IndexSpotEm> = diff
            .iter()
            .filter_map(|item| {
                let code = item.get("f12")?.as_str()?.to_string();
                let name = item.get("f14")?.as_str()?.to_string();
                Some(IndexSpotEm {
                    code,
                    name,
                    latest_price: item.get("f2").and_then(serde_json::Value::as_f64),
                    change_pct: item.get("f3").and_then(serde_json::Value::as_f64),
                    change_amount: item.get("f4").and_then(serde_json::Value::as_f64),
                    volume: item.get("f5").and_then(serde_json::Value::as_f64),
                    amount: item.get("f6").and_then(serde_json::Value::as_f64),
                    high: item.get("f15").and_then(serde_json::Value::as_f64),
                    low: item.get("f16").and_then(serde_json::Value::as_f64),
                    open: item.get("f17").and_then(serde_json::Value::as_f64),
                    prev_close: item.get("f18").and_then(serde_json::Value::as_f64),
                    internal_id: item
                        .get("f13")
                        .and_then(|v| v.as_str())
                        .map(std::string::ToString::to_string),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no index spot data"));
        }
        Ok(items)
    }

    /// Get index daily data from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_index_daily_em(symbol, start_date, end_date)`
    ///
    /// - `symbol`: index code like "000001" (上证指数)
    pub async fn stock_zh_index_daily_em(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<IndexDailyCandle>> {
        // Determine market prefix: 1 for SH indices, 0 for SZ indices
        let market = if symbol.starts_with('0') || symbol.starts_with('3') {
            "1"
        } else {
            "0"
        };
        let secid = format!("{market}.{symbol}");

        let klines = self
            .kline_fetch(
                &secid,
                "101",
                "1",
                1_000_000,
                &[("beg", start_date), ("end", end_date)],
            )
            .await?;

        let items: Vec<IndexDailyCandle> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 7 {
                    return None;
                }
                Some(IndexDailyCandle {
                    date: parts[0].to_string(),
                    open: parts[1].parse().unwrap_or(0.0),
                    close: parts[2].parse().unwrap_or(0.0),
                    high: parts[3].parse().unwrap_or(0.0),
                    low: parts[4].parse().unwrap_or(0.0),
                    volume: parts[5].parse().ok(),
                    amount: parts[6].parse().ok(),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("index daily returned no data"));
        }
        Ok(items)
    }

    /// Get index daily data from Tencent.
    ///
    /// Python equivalent: `stock_zh_index_daily_tx(symbol, start_date, end_date)`
    ///
    /// - `symbol`: index code like "sh000001"
    pub async fn stock_zh_index_daily_tx(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<IndexDailyCandle>> {
        #[derive(Deserialize)]
        struct Resp {
            data: Option<serde_json::Value>,
        }
        // Normalize to Tencent format
        let tx_symbol = if symbol.starts_with("sh") || symbol.starts_with("sz") {
            symbol.to_string()
        } else if symbol.starts_with('0') {
            format!("sh{symbol}")
        } else {
            format!("sz{symbol}")
        };

        let url = format!(
            "https://web.ifzq.gtimg.cn/appstock/app/fqkline/get?param={tx_symbol},day,{start_date},{end_date},640,"
        );

        let resp: Resp = self.get(&url).send().await?.json().await?;
        let data = resp
            .data
            .ok_or_else(|| Error::upstream("empty tencent index data"))?;

        let ts_lower = tx_symbol.to_lowercase();
        let klines = data
            .get(&ts_lower)
            .and_then(|v| v.get("day"))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let items: Vec<IndexDailyCandle> = klines
            .iter()
            .filter_map(|entry| {
                let arr = entry.as_array()?;
                if arr.len() < 6 {
                    return None;
                }
                Some(IndexDailyCandle {
                    date: arr[0].as_str().unwrap_or("").to_string(),
                    open: arr[1].as_f64().unwrap_or(0.0),
                    close: arr[2].as_f64().unwrap_or(0.0),
                    high: arr[3].as_f64().unwrap_or(0.0),
                    low: arr[4].as_f64().unwrap_or(0.0),
                    volume: arr.get(5).and_then(serde_json::Value::as_f64),
                    amount: None,
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("tencent index daily returned no data"));
        }
        Ok(items)
    }

    /// Get index spot data from Sina.
    ///
    /// Python equivalent: `stock_zh_index_spot_sina()`
    pub async fn stock_zh_index_spot_sina(&self) -> Result<Vec<IndexSpotSina>> {
        let count_url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getNameCount";
        let count_resp = self
            .get(count_url)
            .query(&[("node", "hs_s")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let count_text = count_resp.text().await.map_err(Error::from)?;
        let total: i64 = count_text
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        let page_count = ((total as f64) / 80.0).ceil() as i64;

        let list_url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData";
        let mut all_indices = Vec::new();

        for page in 1..=page_count.min(5) {
            let page_str = page.to_string();
            let response = self
                .get(list_url)
                .query(&[
                    ("page", page_str.as_str()),
                    ("num", "80"),
                    ("sort", "symbol"),
                    ("asc", "1"),
                    ("node", "hs_s"),
                    ("symbol", ""),
                    ("_s_r_a", "page"),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let data: Vec<serde_json::Value> = response.json().await.map_err(Error::from)?;

            for item in &data {
                let symbol = item
                    .get("symbol")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let name = item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                all_indices.push(IndexSpotSina {
                    code: symbol,
                    name,
                    latest_price: parse_idx_f64(item, "trade"),
                    change_amount: parse_idx_f64(item, "pricechange"),
                    change_pct: parse_idx_f64(item, "changepercent"),
                    prev_close: parse_idx_f64(item, "settlement"),
                    open: parse_idx_f64(item, "open"),
                    high: parse_idx_f64(item, "high"),
                    low: parse_idx_f64(item, "low"),
                    volume: parse_idx_f64(item, "volume"),
                    amount: parse_idx_f64(item, "amount"),
                });
            }
        }

        if all_indices.is_empty() {
            return Err(Error::not_found("sina returned no index spot data"));
        }
        Ok(all_indices)
    }

    /// Get CSIndex value data.
    ///
    /// Python equivalent: `stock_zh_index_value_csindex(symbol)`
    ///
    /// - `symbol`: index code like "H30374"
    pub async fn stock_zh_index_value_csindex(&self, symbol: &str) -> Result<Vec<CsIndexValue>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<Vec<serde_json::Value>>,
        }
        let url = "https://www.csindex.com.cn/csindex-home/perf/index-perf";
        let start_date = "20000101";
        let end_date = chrono::Utc::now().format("%Y%m%d").to_string();

        let response = self
            .get(url)
            .query(&[
                ("indexCode", symbol),
                ("startDate", start_date),
                ("endDate", end_date.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let data = payload
            .data
            .ok_or_else(|| Error::upstream("CSIndex value missing data"))?;

        let items: Vec<CsIndexValue> = data
            .iter()
            .filter_map(|item| {
                let date = item.get(0)?.as_str()?.to_string();
                Some(CsIndexValue {
                    date,
                    index_code: item
                        .get(1)
                        .and_then(|v| v.as_str())
                        .map(std::string::ToString::to_string),
                    index_name: item
                        .get(3)
                        .and_then(|v| v.as_str())
                        .map(std::string::ToString::to_string),
                    open: item.get(6).and_then(serde_json::Value::as_f64),
                    high: item.get(7).and_then(serde_json::Value::as_f64),
                    low: item.get(8).and_then(serde_json::Value::as_f64),
                    close: item.get(9).and_then(serde_json::Value::as_f64),
                    change: item.get(10).and_then(serde_json::Value::as_f64),
                    change_pct: item.get(11).and_then(serde_json::Value::as_f64),
                    volume: item.get(12).and_then(serde_json::Value::as_f64),
                    amount: item.get(13).and_then(serde_json::Value::as_f64),
                    sample_count: item.get(14).and_then(serde_json::Value::as_f64),
                    pe_ttm: item.get(15).and_then(serde_json::Value::as_f64),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("CSIndex value returned no data"));
        }
        Ok(items)
    }
}

fn parse_idx_f64(item: &serde_json::Value, key: &str) -> Option<f64> {
    item.get(key).and_then(|v| {
        if let Some(n) = v.as_f64() {
            Some(n)
        } else if let Some(s) = v.as_str() {
            s.parse().ok()
        } else {
            None
        }
    })
}
