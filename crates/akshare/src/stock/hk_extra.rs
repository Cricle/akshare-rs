#![allow(dead_code)]
//! HK stock extra data — spot, daily, famous, index, hot rank, financial, valuation, etc.
//!
//! Covers Python functions:
//! - `stock_hk_spot` — HK spot (Sina)
//! - `stock_hk_daily` — HK daily (Sina, via Eastmoney kline)
//! - `stock_hk_famous_spot_em` — Famous HK stocks (Eastmoney)
//! - `stock_hk_index_daily_em` — HK index daily (Eastmoney)
//! - `stock_hk_index_daily_sina` — HK index daily (Sina)
//! - `stock_hk_index_spot_em` — HK index spot (Eastmoney)
//! - `stock_hk_index_spot_sina` — HK index spot (Sina)
//! - `stock_hk_indicator_eniu` — Eniu indicators
//! - `stock_hk_scale_comparison_em` — HK scale comparison (Eastmoney)
//! - `stock_hk_valuation_baidu` — HK valuation (Baidu)
//! - `stock_hk_hot_rank_em` — HK hot rank (Eastmoney)
//! - `stock_hk_hot_rank_latest_em` — HK latest hot rank (Eastmoney)
//! - `stock_hk_hot_rank_detail_em` — HK hot rank detail (Eastmoney)
//! - `stock_hk_hot_rank_detail_realtime_em` — HK realtime hot rank (Eastmoney)
//! - `stock_hk_dividend_payout_em` — HK dividend payout (Eastmoney)
//! - `stock_hk_fhpx_detail_ths` — HK fhpx detail (THS)
//! - `stock_hk_financial_indicator_em` — HK financial indicators (Eastmoney)
//! - `stock_hk_gxl_lg` — HK dividend yield (Legulegu)

use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use crate::client::AkShareClient;
use crate::error::{Error, Result};

static RE_TR: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<tr[^>]*>([\s\S]*?)</tr>").unwrap());
static RE_TD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<td[^>]*>([\s\S]*?)</td>").unwrap());

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// HK stock spot quote from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkSpotQuote {
    pub code: String,
    pub chinese_name: String,
    #[serde(default)]
    pub english_name: Option<String>,
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
    #[serde(default)]
    pub buy_price: Option<f64>,
    #[serde(default)]
    pub sell_price: Option<f64>,
}

/// HK stock daily candle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkDailyCandle {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// Famous HK stock from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkFamousStock {
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
    pub open: Option<f64>,
    #[serde(default)]
    pub high: Option<f64>,
    #[serde(default)]
    pub low: Option<f64>,
    #[serde(default)]
    pub prev_close: Option<f64>,
    #[serde(default)]
    pub market_cap: Option<f64>,
    #[serde(default)]
    pub pe_ratio: Option<f64>,
}

/// HK index daily candle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkIndexDailyCandle {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

/// HK index spot from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkIndexSpotEm {
    pub code: String,
    pub internal_id: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
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

/// HK index spot from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkIndexSpotSina {
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
}

/// HK hot rank entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkHotRank {
    #[serde(default)]
    pub rank: Option<i64>,
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
}

/// HK hot rank detail entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkHotRankDetail {
    pub time: String,
    pub rank: i64,
    #[serde(default)]
    pub code: Option<String>,
}

/// HK valuation from Baidu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkValuationBaidu {
    pub date: String,
    pub value: f64,
}

/// HK dividend yield from Legulegu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkGxlLg {
    pub date: String,
    pub dividend_yield: f64,
}

/// HK fhpx detail from THS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkFhpxDetailThs {
    pub announce_date: String,
    #[serde(default)]
    pub scheme: Option<String>,
    #[serde(default)]
    pub ex_date: Option<String>,
    #[serde(default)]
    pub payout_date: Option<String>,
    #[serde(default)]
    pub transfer_start: Option<String>,
    #[serde(default)]
    pub transfer_end: Option<String>,
    #[serde(default)]
    pub dividend_type: Option<String>,
    #[serde(default)]
    pub progress: Option<String>,
    #[serde(default)]
    pub stock_dividend: Option<String>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get HK stock spot data from Sina.
    ///
    /// Python equivalent: `stock_hk_spot()`
    pub async fn stock_hk_spot(&self) -> Result<Vec<HkSpotQuote>> {
        let url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHKStockData";
        let mut all_quotes = Vec::new();

        for page in 1..100 {
            let page_str = page.to_string();
            let response = self
                .get(url)
                .query(&[
                    ("page", page_str.as_str()),
                    ("num", "60"),
                    ("sort", "symbol"),
                    ("asc", "1"),
                    ("node", "qbgg_hk"),
                    ("_s_r_a", "init"),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let data: Vec<serde_json::Value> = response.json().await.map_err(Error::from)?;
            if data.is_empty() {
                break;
            }

            for item in &data {
                let code = item
                    .get("symbol")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let cname = item
                    .get("cname")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let ename = item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string);

                all_quotes.push(HkSpotQuote {
                    code,
                    chinese_name: cname,
                    english_name: ename,
                    latest_price: parse_hk_f64(item, "trade"),
                    change_amount: parse_hk_f64(item, "pricechange"),
                    change_pct: parse_hk_f64(item, "changepercent"),
                    prev_close: parse_hk_f64(item, "settlement"),
                    open: parse_hk_f64(item, "open"),
                    high: parse_hk_f64(item, "high"),
                    low: parse_hk_f64(item, "low"),
                    volume: parse_hk_f64(item, "volume"),
                    amount: parse_hk_f64(item, "amount"),
                    buy_price: parse_hk_f64(item, "buy"),
                    sell_price: parse_hk_f64(item, "sell"),
                });
            }
        }

        if all_quotes.is_empty() {
            return Err(Error::not_found("sina returned no HK spot data"));
        }
        Ok(all_quotes)
    }

    /// Get HK stock daily candles. Uses Eastmoney kline API.
    ///
    /// Python equivalent: `stock_hk_daily(symbol, start_date, end_date, adjust)`
    ///
    /// - `symbol`: HK stock code like "00981"
    /// - `adjust`: "", "qfq", "hfq"
    pub async fn stock_hk_daily(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<HkDailyCandle>> {
        let code = symbol.trim_start_matches('0');
        let code = if code.is_empty() { "0" } else { code };
        let secid = format!("116.{code}");
        let fqt = match adjust {
            "" => "0",
            "qfq" => "1",
            "hfq" => "2",
            _ => return Err(Error::invalid_input(format!("invalid adjust: {adjust}"))),
        };

        let klines = self
            .kline_fetch(
                &secid,
                "101",
                fqt,
                1_000_000,
                &[("beg", start_date), ("end", end_date)],
            )
            .await?;

        let items: Vec<HkDailyCandle> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 6 {
                    return None;
                }
                Some(HkDailyCandle {
                    date: parts[0].to_string(),
                    open: parts[1].parse().unwrap_or(0.0),
                    close: parts[2].parse().unwrap_or(0.0),
                    high: parts[3].parse().unwrap_or(0.0),
                    low: parts[4].parse().unwrap_or(0.0),
                    volume: parts[5].parse().unwrap_or(0.0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("HK daily returned no data"));
        }
        Ok(items)
    }

    /// Get famous HK stocks from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_famous_spot_em()`
    pub async fn stock_hk_famous_spot_em(&self) -> Result<Vec<HkFamousStock>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            diff: Option<serde_json::Value>,
        }
        let response = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", "5000"),
                ("po", "1"),
                ("np", "2"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f3"),
                ("fs", "b:DLMK0106"),
                ("fields", "f2,f3,f4,f5,f6,f9,f12,f14,f15,f16,f17,f18,f20"),
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
            .ok_or_else(|| Error::upstream("HK famous stocks missing data"))?;

        let mut items = Vec::new();

        let entries: Vec<&serde_json::Value> = if let Some(arr) = diff.as_array() {
            arr.iter().collect()
        } else if let Some(obj) = diff.as_object() {
            obj.values().collect()
        } else {
            vec![]
        };

        for val in entries {
            if let (Some(code), Some(name)) = (
                val.get("f12").and_then(|v| v.as_str()),
                val.get("f14").and_then(|v| v.as_str()),
            ) {
                items.push(HkFamousStock {
                    code: code.to_string(),
                    name: name.to_string(),
                    latest_price: val.get("f2").and_then(serde_json::Value::as_f64),
                    change_pct: val.get("f3").and_then(serde_json::Value::as_f64),
                    change_amount: val.get("f4").and_then(serde_json::Value::as_f64),
                    volume: val.get("f5").and_then(serde_json::Value::as_f64),
                    amount: val.get("f6").and_then(serde_json::Value::as_f64),
                    open: val.get("f17").and_then(serde_json::Value::as_f64),
                    high: val.get("f15").and_then(serde_json::Value::as_f64),
                    low: val.get("f16").and_then(serde_json::Value::as_f64),
                    prev_close: val.get("f18").and_then(serde_json::Value::as_f64),
                    market_cap: val.get("f20").and_then(serde_json::Value::as_f64),
                    pe_ratio: val.get("f9").and_then(serde_json::Value::as_f64),
                });
            }
        }

        if items.is_empty() {
            return Err(Error::not_found("HK famous stocks returned no data"));
        }
        Ok(items)
    }

    /// Get HK index daily data from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_index_daily_em(symbol)`
    ///
    /// - `symbol`: HK index code like "HSTECH"
    pub async fn stock_hk_index_daily_em(&self, symbol: &str) -> Result<Vec<HkIndexDailyCandle>> {
        // Use secid format: internal_id.symbol
        let secid = format!("100.{symbol}");

        let klines = self
            .kline_fetch(&secid, "101", "1", 10_000, &[("iscca", "1")])
            .await?;

        let items: Vec<HkIndexDailyCandle> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 5 {
                    return None;
                }
                Some(HkIndexDailyCandle {
                    date: parts[0].to_string(),
                    open: parts[1].parse().unwrap_or(0.0),
                    high: parts[3].parse().unwrap_or(0.0),
                    low: parts[4].parse().unwrap_or(0.0),
                    close: parts[2].parse().unwrap_or(0.0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("HK index daily returned no data"));
        }
        Ok(items)
    }

    /// Get HK index daily data from Sina.
    ///
    /// Python equivalent: `stock_hk_index_daily_sina(symbol)`
    ///
    /// - `symbol`: HK index code like "CES100"
    pub async fn stock_hk_index_daily_sina(&self, symbol: &str) -> Result<Vec<HkIndexDailyCandle>> {
        let url = format!("https://finance.sina.com.cn/stock/hkstock/{symbol}/klc2_kl.js");

        let response = self
            .get(&url)
            .query(&[("d", "2023_5_01")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;

        // Parse JS response - extract JSON
        let json_start = text
            .find("=(")
            .ok_or_else(|| Error::decode("invalid JS response"))?
            + 2;
        let json_end = text
            .find(");")
            .ok_or_else(|| Error::decode("invalid JS response"))?;
        let json_text = &text[json_start..json_end];

        let data: Vec<serde_json::Value> = serde_json::from_str(json_text)
            .map_err(|e| Error::decode(format!("JSON parse error: {e}")))?;

        let items: Vec<HkIndexDailyCandle> = data
            .iter()
            .filter_map(|item| {
                let date = item.get("date")?.as_str()?.to_string();
                let open = item.get("open")?.as_f64()?;
                let close = item.get("close")?.as_f64()?;
                let high = item.get("high")?.as_f64()?;
                let low = item.get("low")?.as_f64()?;
                Some(HkIndexDailyCandle {
                    date,
                    open,
                    high,
                    low,
                    close,
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("sina HK index daily returned no data"));
        }
        Ok(items)
    }

    /// Get HK index spot from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_index_spot_em()`
    pub async fn stock_hk_index_spot_em(&self) -> Result<Vec<HkIndexSpotEm>> {
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
                ("fs", "m:124,m:125,m:305"),
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
            .ok_or_else(|| Error::upstream("HK index spot missing data"))?;

        let items: Vec<HkIndexSpotEm> = diff
            .iter()
            .filter_map(|item| {
                let code = item.get("f12")?.as_str()?.to_string();
                let internal_id = item
                    .get("f13")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let name = item.get("f14")?.as_str()?.to_string();
                Some(HkIndexSpotEm {
                    code,
                    internal_id,
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
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no HK index spot"));
        }
        Ok(items)
    }

    /// Get HK index spot from Sina.
    ///
    /// Python equivalent: `stock_hk_index_spot_sina()`
    pub async fn stock_hk_index_spot_sina(&self) -> Result<Vec<HkIndexSpotSina>> {
        let url = "https://hq.sinajs.cn/rn=mtf2t&list=hkCES100,hkCES120,hkCES280,hkCES300,hkCESA80,hkCESG10,hkCESHKM,hkCSCMC,hkCSHK100,hkCSHKDIV,hkCSHKLC,hkCSHKLRE,hkCSHKMCS,hkCSHKME,hkCSHKPE,hkCSHKSE,hkCSI300,hkCSRHK50,hkGEM,hkHKL,hkHSCCI,hkHSCEI,hkHSI,hkHSMBI,hkHSMOGI,hkHSMPI,hkHSTECH,hkSSE180,hkSSE180GV,hkSSE380,hkSSE50,hkSSECEQT,hkSSECOMP,hkSSEDIV,hkSSEITOP,hkSSEMCAP,hkSSEMEGA,hkVHSI";

        let response = self
            .get(url)
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;

        let mut items = Vec::new();
        for line in text.lines() {
            let parts: Vec<&str> = line.split('"').collect();
            if parts.len() < 2 {
                continue;
            }
            let data_str = parts[1];
            let fields: Vec<&str> = data_str.split(',').collect();
            if fields.len() < 9 {
                continue;
            }
            // Extract code from var name: var hq_str_hkHSI -> hkHSI
            let var_part = line.split('=').next().unwrap_or("");
            let code = var_part.split('_').next_back().unwrap_or("");

            items.push(HkIndexSpotSina {
                code: code.to_string(),
                name: fields[0].to_string(),
                latest_price: fields[6].parse().ok(),
                change_amount: fields[7].parse().ok(),
                change_pct: fields[8].parse().ok(),
                prev_close: fields[3].parse().ok(),
                open: fields[2].parse().ok(),
                high: fields[4].parse().ok(),
                low: fields[5].parse().ok(),
            });
        }

        if items.is_empty() {
            return Err(Error::not_found("sina returned no HK index spot"));
        }
        Ok(items)
    }

    /// Get HK stock hot rank from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_hot_rank_em()`
    pub async fn stock_hk_hot_rank_em(&self) -> Result<Vec<HkHotRank>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<Vec<RankItem>>,
        }
        #[derive(Deserialize)]
        struct RankItem {
            sc: Option<String>,
            rk: Option<i64>,
        }
        let url = "https://emappdata.eastmoney.com/stockrank/getAllCurrHkUsList";
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "marketType": "000003",
            "pageNo": 1,
            "pageSize": 100,
        });

        let response = self
            .post(url)
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let env: Env = response.json().await.map_err(Error::from)?;
        let rank_data = env
            .data
            .ok_or_else(|| Error::upstream("HK hot rank missing data"))?;

        let mut items = Vec::new();
        for item in &rank_data {
            let sc = item.sc.as_deref().unwrap_or("");
            let parts: Vec<&str> = sc.split('|').collect();
            let code = parts.get(1).unwrap_or(&"").to_string();
            let _mark = format!("116.{}", &sc[3..]);

            items.push(HkHotRank {
                rank: item.rk,
                code,
                name: String::new(), // Will be filled if price data available
                latest_price: None,
                change_pct: None,
            });
        }

        if items.is_empty() {
            return Err(Error::not_found("HK hot rank returned no data"));
        }
        Ok(items)
    }

    /// Get HK latest hot rank from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_hot_rank_latest_em(symbol)`
    pub async fn stock_hk_hot_rank_latest_em(&self, symbol: &str) -> Result<Vec<HkHotRankDetail>> {
        let url = "https://emappdata.eastmoney.com/stockrank/getCurrentHkUsLatest";
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "marketType": "000003",
            "srcSecurityCode": format!("HK|{}", symbol),
        });

        let response = self
            .post(url)
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let data: serde_json::Value = response.json().await.map_err(Error::from)?;
        let obj = data
            .get("data")
            .and_then(|d| d.as_object())
            .ok_or_else(|| Error::upstream("HK hot rank latest missing data"))?;

        let mut items = Vec::new();
        for (key, val) in obj {
            let rank = val.as_i64().unwrap_or(0);
            items.push(HkHotRankDetail {
                time: key.clone(),
                rank,
                code: Some(symbol.to_string()),
            });
        }

        if items.is_empty() {
            return Err(Error::not_found("HK hot rank latest returned no data"));
        }
        Ok(items)
    }

    /// Get HK hot rank detail from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_hot_rank_detail_em(symbol)`
    pub async fn stock_hk_hot_rank_detail_em(&self, symbol: &str) -> Result<Vec<HkHotRankDetail>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<Vec<RankDetailItem>>,
        }
        #[derive(Deserialize)]
        struct RankDetailItem {
            dt: Option<String>,
            rk: Option<i64>,
        }
        let url = "https://emappdata.eastmoney.com/stockrank/getHisHkUsList";
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "marketType": "000003",
            "srcSecurityCode": format!("HK|{}", symbol),
        });

        let response = self
            .post(url)
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let env: Env = response.json().await.map_err(Error::from)?;
        let data = env
            .data
            .ok_or_else(|| Error::upstream("HK hot rank detail missing data"))?;

        let items: Vec<HkHotRankDetail> = data
            .iter()
            .map(|item| HkHotRankDetail {
                time: item.dt.clone().unwrap_or_default(),
                rank: item.rk.unwrap_or(0),
                code: Some(symbol.to_string()),
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("HK hot rank detail returned no data"));
        }
        Ok(items)
    }

    /// Get HK realtime hot rank from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_hot_rank_detail_realtime_em(symbol)`
    pub async fn stock_hk_hot_rank_detail_realtime_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<HkHotRankDetail>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<Vec<RankDetailItem>>,
        }
        #[derive(Deserialize)]
        struct RankDetailItem {
            dt: Option<String>,
            rk: Option<i64>,
        }
        let url = "https://emappdata.eastmoney.com/stockrank/getCurrentHkUsList";
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "marketType": "000003",
            "srcSecurityCode": format!("HK|{}", symbol),
        });

        let response = self
            .post(url)
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let env: Env = response.json().await.map_err(Error::from)?;
        let data = env
            .data
            .ok_or_else(|| Error::upstream("HK realtime hot rank missing data"))?;

        let items: Vec<HkHotRankDetail> = data
            .iter()
            .map(|item| HkHotRankDetail {
                time: item.dt.clone().unwrap_or_default(),
                rank: item.rk.unwrap_or(0),
                code: Some(symbol.to_string()),
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("HK realtime hot rank returned no data"));
        }
        Ok(items)
    }

    /// Get HK valuation from Baidu.
    ///
    /// Python equivalent: `stock_hk_valuation_baidu(symbol, indicator, period)`
    pub async fn stock_hk_valuation_baidu(
        &self,
        symbol: &str,
        indicator: &str,
        period: &str,
    ) -> Result<Vec<HkValuationBaidu>> {
        let url = "https://finance.baidu.com/opendata";
        let response = self
            .get(url)
            .query(&[
                ("openapi", "1"),
                ("dspName", "iphone"),
                ("tn", "tangram"),
                ("client", "app"),
                ("query", indicator),
                ("code", symbol),
                ("word", ""),
                ("resource_id", "51171"),
                ("market", "hk"),
                ("tag", indicator),
                ("chart_select", period),
                ("industry_select", ""),
                ("skip_industry", "1"),
                ("finClientType", "pc"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let data: serde_json::Value = response.json().await.map_err(Error::from)?;

        let body = data
            .get("Result")
            .and_then(|r| r.get(0))
            .and_then(|r| r.get("DisplayData"))
            .and_then(|d| d.get("resultData"))
            .and_then(|r| r.get("tplData"))
            .and_then(|t| t.get("result"))
            .and_then(|r| r.get("chartInfo"))
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("body"))
            .and_then(|b| b.as_array())
            .ok_or_else(|| Error::upstream("baidu HK valuation missing chart data"))?;

        let items: Vec<HkValuationBaidu> = body
            .iter()
            .filter_map(|item| {
                let arr = item.as_array()?;
                if arr.len() < 2 {
                    return None;
                }
                Some(HkValuationBaidu {
                    date: arr[0].as_str().unwrap_or("").to_string(),
                    value: arr[1].as_f64().unwrap_or(0.0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("baidu HK valuation returned no data"));
        }
        Ok(items)
    }

    /// Get HK scale comparison from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_scale_comparison_em(symbol)`
    pub async fn stock_hk_scale_comparison_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        #[derive(Deserialize)]
        struct Env {
            result: Option<EnvResult>,
        }
        #[derive(Deserialize)]
        struct EnvResult {
            data: Option<Vec<serde_json::Value>>,
        }
        let filter = format!("(SECUCODE=\"{symbol}.HK\")");

        let url = "https://datacenter.eastmoney.com/securities/api/data/v1/get";
        let response = self
            .get(url)
            .query(&[
                ("reportName", "RPT_PCF10_INDUSTRY_HKSCALE"),
                ("columns", "ALL"),
                ("quoteColumns", ""),
                ("filter", filter.as_str()),
                ("pageNumber", "1"),
                ("pageSize", ""),
                ("sortTypes", ""),
                ("sortColumns", ""),
                ("source", "F10"),
                ("client", "PC"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("HK scale comparison missing data"))?;

        if data.is_empty() {
            return Err(Error::not_found("HK scale comparison returned no data"));
        }
        Ok(data)
    }

    /// Get HK dividend payout from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_dividend_payout_em(symbol)`
    pub async fn stock_hk_dividend_payout_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        #[derive(Deserialize)]
        struct Env {
            result: Option<EnvResult>,
        }
        #[derive(Deserialize)]
        struct EnvResult {
            data: Option<Vec<serde_json::Value>>,
        }
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");

        let url = "https://datacenter.eastmoney.com/securities/api/data/v1/get";
        let response = self
            .get(url)
            .query(&[
                ("reportName", "RPT_HKF10_FN_DIVIDEND"),
                ("columns", "ALL"),
                ("quoteColumns", ""),
                ("filter", filter.as_str()),
                ("pageNumber", "1"),
                ("pageSize", ""),
                ("sortTypes", "-1"),
                ("sortColumns", "EX_DIVIDEND_DATE"),
                ("source", "F10"),
                ("client", "PC"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("HK dividend payout missing data"))?;

        if data.is_empty() {
            return Err(Error::not_found("HK dividend payout returned no data"));
        }
        Ok(data)
    }

    /// Get HK fhpx detail from THS.
    ///
    /// Python equivalent: `stock_hk_fhpx_detail_ths(symbol)`
    pub async fn stock_hk_fhpx_detail_ths(&self, symbol: &str) -> Result<Vec<HkFhpxDetailThs>> {
        let url = format!("https://basic.10jqka.com.cn/176/HK{symbol}/bonus.html");

        let response = self
                        .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let html = response.text().await.map_err(Error::from)?;

        // Parse HTML table - find rows with dividend data
        let mut items = Vec::new();
        for tr_cap in RE_TR.captures_iter(&html) {
            let tr_html = &tr_cap[1];
            let tds: Vec<String> = RE_TD
                .captures_iter(tr_html)
                .map(|c| {
                    c[1].replace("<br>", "/")
                        .replace("<br/>", "/")
                        .trim()
                        .to_string()
                })
                .collect();

            if tds.len() >= 7 {
                items.push(HkFhpxDetailThs {
                    announce_date: tds[0].clone(),
                    scheme: Some(tds[1].clone()),
                    ex_date: Some(tds[2].clone()),
                    payout_date: Some(tds[3].clone()),
                    transfer_start: Some(tds[4].clone()),
                    transfer_end: Some(tds[5].clone()),
                    dividend_type: Some(tds[6].clone()),
                    progress: tds.get(7).cloned(),
                    stock_dividend: tds.get(8).cloned(),
                });
            }
        }

        if items.is_empty() {
            return Err(Error::not_found("THS HK fhpx returned no data"));
        }
        Ok(items)
    }

    /// Get HK financial indicators from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_financial_indicator_em(symbol)`
    pub async fn stock_hk_financial_indicator_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        #[derive(Deserialize)]
        struct Env {
            result: Option<EnvResult>,
        }
        #[derive(Deserialize)]
        struct EnvResult {
            data: Option<Vec<serde_json::Value>>,
        }
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");

        let url = "https://datacenter.eastmoney.com/securities/api/data/v1/get";
        let response = self
            .get(url)
            .query(&[
                ("reportName", "RPT_HKF10_FN_MAINFINADATA"),
                ("columns", "ALL"),
                ("quoteColumns", ""),
                ("filter", filter.as_str()),
                ("pageNumber", "1"),
                ("pageSize", ""),
                ("sortTypes", "-1"),
                ("sortColumns", "REPORT_DATE"),
                ("source", "F10"),
                ("client", "PC"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("HK financial indicators missing data"))?;

        if data.is_empty() {
            return Err(Error::not_found("HK financial indicators returned no data"));
        }
        Ok(data)
    }

    /// Get HK dividend yield from Legulegu.
    ///
    /// Python equivalent: `stock_hk_gxl_lg()`
    ///
    /// Returns HSI (Hang Seng Index) dividend yield data.
    pub async fn stock_hk_gxl_lg(&self) -> Result<Vec<HkGxlLg>> {
        let url = "https://legulegu.com/api/stockdata/hs";
        let response = self
            .get(url)
            .query(&[("indexCode", "HSI")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let data: serde_json::Value = response.json().await.map_err(Error::from)?;
        let arr = data
            .as_array()
            .ok_or_else(|| Error::decode("legulegu response is not an array"))?;

        let items: Vec<HkGxlLg> = arr
            .iter()
            .filter_map(|item| {
                let date = item.get("date")?.as_str()?.to_string();
                let dv_ratio = item.get("dvRatio")?.as_f64()?;
                Some(HkGxlLg {
                    date,
                    dividend_yield: dv_ratio,
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(
                "legulegu HK dividend yield returned no data",
            ));
        }
        Ok(items)
    }

    /// Get Eniu HK indicators (stub - requires authentication).
    ///
    /// Python equivalent: `stock_hk_indicator_eniu(symbol)`
    pub async fn stock_hk_indicator_eniu(&self, _symbol: &str) -> Result<Vec<serde_json::Value>> {
        Err(Error::unsupported_market(
            "stock_hk_indicator_eniu requires Eniu API authentication, not yet implemented",
        ))
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_hk_f64(item: &serde_json::Value, key: &str) -> Option<f64> {
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

async fn parse_em_stock_list<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
    context: &str,
) -> Result<Vec<T>> {
    #[derive(Deserialize)]
    struct Env {
        data: Option<EnvData>,
    }
    #[derive(Deserialize)]
    struct EnvData {
        diff: Option<serde_json::Value>,
    }

    let payload: Env = response.json().await.map_err(Error::from)?;
    let diff = payload
        .data
        .and_then(|d| d.diff)
        .ok_or_else(|| Error::upstream(format!("{context} missing data")))?;

    let mut items = Vec::new();

    if let Some(arr) = diff.as_array() {
        for val in arr {
            if let Ok(item) = serde_json::from_value::<T>(val.clone()) {
                items.push(item);
            }
        }
    } else if let Some(obj) = diff.as_object() {
        for (_, val) in obj {
            if let Ok(item) = serde_json::from_value::<T>(val.clone()) {
                items.push(item);
            }
        }
    }

    if items.is_empty() {
        return Err(Error::not_found(format!("{context} returned no data")));
    }
    Ok(items)
}
