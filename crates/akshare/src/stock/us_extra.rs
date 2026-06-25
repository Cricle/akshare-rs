//! US stock extra data — daily, spot, famous, pink, valuation, hot rank, index, financial, dividend.
//!
//! Covers Python functions:
//! - `stock_us_daily` — US daily candles (Sina)
//! - `stock_us_spot` — US spot quotes (Sina)
//! - `stock_us_famous_spot` — Famous US stocks (Eastmoney)
//! - `stock_us_pink_spot` — Pink sheet stocks (Eastmoney)
//! - `stock_us_valuation` — US valuation (Baidu)
//! - `stock_us_hot_rank` — US hot rank (Eastmoney)
//! - `stock_us_hot_rank_latest` — US latest hot rank (Eastmoney)
//! - `stock_us_hot_rank_detail` — US hot rank detail (Eastmoney)
//! - `stock_us_hot_rank_detail_realtime` — US realtime hot rank (Eastmoney)
//! - `stock_us_index_spot_em` — US index spot (Eastmoney)
//! - `stock_us_index_daily_em` — US index daily (Eastmoney)
//! - `stock_us_index_spot_sina` — US index spot (Sina)
//! - `stock_us_index_daily_sina` — US index daily (Sina)
//! - `stock_us_financial_indicator` — US financial indicators (Eastmoney)
//! - `stock_us_dividend_payout` — US dividend payout (Eastmoney)
//! - `stock_us_gxl_lg` — US dividend yield (Legulegu)
//! - `stock_us_scale_comparison` — US scale comparison (Eastmoney)
//! - `stock_us_hot_keyword` — US hot keywords (Eastmoney)

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::value_ext::ValueExt;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// US stock daily candle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsDailyCandle {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// US stock spot quote from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsSpotSina {
    pub symbol: String,
    pub name: String,
    #[serde(default)]
    pub chinese_name: Option<String>,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
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
}

/// Famous US stock from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsFamousStock {
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

/// Pink sheet stock from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsPinkStock {
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

/// US valuation data point from Baidu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsValuationBaidu {
    pub date: String,
    pub value: f64,
}

/// US hot rank entry from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsHotRank {
    #[serde(default)]
    pub rank: Option<i64>,
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
}

/// US hot rank detail entry from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsHotRankDetail {
    pub time: String,
    pub rank: i64,
    #[serde(default)]
    pub code: Option<String>,
}

/// US index spot from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsIndexSpotEm {
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

/// US index daily candle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsIndexDailyCandle {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

/// US index spot from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsIndexSpotSina {
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

/// US dividend yield from Legulegu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsGxlLg {
    pub date: String,
    pub dividend_yield: f64,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get US stock daily candles.
    ///
    /// Python equivalent: `stock_us_daily(symbol, start_date, end_date)`
    ///
    /// Uses Eastmoney kline API for US stocks.
    /// - `symbol`: US stock code like "AAPL" or Eastmoney format "105.AAPL"
    pub async fn stock_us_daily(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<UsDailyCandle>> {
        // Normalize to Eastmoney secid format for US stocks
        let secid = if symbol.contains('.') {
            symbol.to_string()
        } else {
            format!("105.{}", symbol.to_uppercase())
        };

        let klines = self
            .kline_fetch(
                &secid,
                "101",
                "1",
                1_000_000,
                &[("beg", start_date), ("end", end_date)],
            )
            .await?;

        let items: Vec<UsDailyCandle> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 6 {
                    return None;
                }
                Some(UsDailyCandle {
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
            return Err(Error::not_found("US daily returned no data"));
        }
        Ok(items)
    }

    /// Get all US stock spot quotes from Sina.
    ///
    /// Python equivalent: `stock_us_spot()`
    ///
    /// Note: Sina US stock API requires JavaScript execution for full data.
    /// This returns a basic set from the Sina API.
    pub async fn stock_us_spot(&self) -> Result<Vec<UsSpotSina>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            diff: Option<Vec<serde_json::Value>>,
        }
        // Use Eastmoney US spot API as a more reliable alternative
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params("5000", &[
                    ("fid", "f3"),
                    ("fs", "m:105,m:106,m:107"),
                    ("fields", "f2,f3,f4,f5,f6,f12,f14"),
                ])),
        )
        .await?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("US spot missing data"))?;

        let items: Vec<UsSpotSina> = diff
            .iter()
            .filter_map(|item| {
                let code = item.get("f12")?.as_str()?.to_string();
                let name = item.get("f14")?.as_str()?.to_string();
                Some(UsSpotSina {
                    symbol: code,
                    name,
                    chinese_name: None,
                    latest_price: item.f64_field(&["f2"]),
                    change_pct: item.f64_field(&["f3"]),
                    change_amount: item.f64_field(&["f4"]),
                    volume: item.f64_field(&["f5"]),
                    open: None,
                    high: None,
                    low: None,
                    prev_close: None,
                    market_cap: item.f64_field(&["f6"]),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("US spot returned no data"));
        }
        Ok(items)
    }

    /// Get famous US stocks from Eastmoney.
    ///
    /// Python equivalent: `stock_us_famous_spot(symbol)`
    ///
    /// - `symbol`: category like "科技类", "金融类", "医药食品类", "媒体类", "汽车能源类", "制造零售类"
    pub async fn stock_us_famous_spot(&self, symbol: &str) -> Result<Vec<UsFamousStock>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            diff: Option<serde_json::Value>,
        }
        let market_map: std::collections::HashMap<&str, &str> = [
            ("科技类", "0216"),
            ("金融类", "0217"),
            ("医药食品类", "0218"),
            ("媒体类", "0220"),
            ("汽车能源类", "0219"),
            ("制造零售类", "0221"),
        ]
        .iter()
        .copied()
        .collect();

        let code = market_map
            .get(symbol)
            .ok_or_else(|| Error::invalid_input(format!("invalid category: {symbol}")))?;

        let fs = format!("b:MK{code}");

        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&[
                    ("pn", "1"),
                    ("pz", "5000"),
                    ("po", "1"),
                    ("np", "2"),
                    ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                    ("fltt", "2"),
                    ("invt", "2"),
                    ("fid", "f3"),
                    ("fs", fs.as_str()),
                    ("fields", "f2,f3,f4,f5,f6,f9,f12,f14,f15,f16,f17,f18,f20"),
                ]),
        )
        .await?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("US famous stocks missing data"))?;

        let mut items = Vec::new();

        if let Some(arr) = diff.as_array() {
            for item in arr {
                if let (Some(code), Some(name)) =
                    (item.str_field(&["f12"]), item.str_field(&["f14"]))
                {
                    items.push(UsFamousStock {
                        code: code.to_string(),
                        name: name.to_string(),
                        latest_price: item.f64_field(&["f2"]),
                        change_pct: item.f64_field(&["f3"]),
                        change_amount: item.f64_field(&["f4"]),
                        volume: item.f64_field(&["f5"]),
                        amount: item.f64_field(&["f6"]),
                        open: item.f64_field(&["f17"]),
                        high: item.f64_field(&["f15"]),
                        low: item.f64_field(&["f16"]),
                        prev_close: item.f64_field(&["f18"]),
                        market_cap: item.f64_field(&["f20"]),
                        pe_ratio: item.f64_field(&["f9"]),
                    });
                }
            }
        } else if let Some(obj) = diff.as_object() {
            for (_, val) in obj {
                if let (Some(code), Some(name)) = (val.str_field(&["f12"]), val.str_field(&["f14"]))
                {
                    items.push(UsFamousStock {
                        code: code.to_string(),
                        name: name.to_string(),
                        latest_price: val.f64_field(&["f2"]),
                        change_pct: val.f64_field(&["f3"]),
                        change_amount: val.f64_field(&["f4"]),
                        volume: val.f64_field(&["f5"]),
                        amount: val.f64_field(&["f6"]),
                        open: val.f64_field(&["f17"]),
                        high: val.f64_field(&["f15"]),
                        low: val.f64_field(&["f16"]),
                        prev_close: val.f64_field(&["f18"]),
                        market_cap: val.f64_field(&["f20"]),
                        pe_ratio: val.f64_field(&["f9"]),
                    });
                }
            }
        }

        if items.is_empty() {
            return Err(Error::not_found("US famous stocks returned no data"));
        }
        Ok(items)
    }

    /// Get pink sheet stocks from Eastmoney.
    ///
    /// Python equivalent: `stock_us_pink_spot()`
    pub async fn stock_us_pink_spot(&self) -> Result<Vec<UsPinkStock>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            diff: Option<Vec<serde_json::Value>>,
        }
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&[
                    ("pn", "1"),
                    ("pz", "5000"),
                    ("po", "1"),
                    ("np", "1"),
                    ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                    ("fltt", "1"),
                    ("invt", "1"),
                    ("fid", "f3"),
                    ("fs", "m:153"),
                    ("fields", "f2,f3,f4,f5,f6,f9,f12,f14,f15,f16,f17,f18,f20"),
                ]),
        )
        .await?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("US pink stocks missing data"))?;

        let items: Vec<UsPinkStock> = diff
            .iter()
            .filter_map(|item| {
                let code = item.get("f12")?.as_str()?.to_string();
                let name = item.get("f14")?.as_str()?.to_string();
                Some(UsPinkStock {
                    code,
                    name,
                    latest_price: item.f64_field(&["f2"]),
                    change_pct: item.f64_field(&["f3"]),
                    change_amount: item.f64_field(&["f4"]),
                    volume: item.f64_field(&["f5"]),
                    amount: item.f64_field(&["f6"]),
                    open: item.f64_field(&["f17"]),
                    high: item.f64_field(&["f15"]),
                    low: item.f64_field(&["f16"]),
                    prev_close: item.f64_field(&["f18"]),
                    market_cap: item.f64_field(&["f20"]),
                    pe_ratio: item.f64_field(&["f9"]),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("US pink stocks returned no data"));
        }
        Ok(items)
    }

    /// Get US valuation data from Baidu.
    ///
    /// Python equivalent: `stock_us_valuation(symbol, indicator, period)`
    ///
    /// - `symbol`: US stock code like "NVDA"
    /// - `indicator`: "总市值", "市盈率(TTM)", "市盈率(静)", "市净率", "市现率"
    /// - `period`: "近一年", "近三年", "全部"
    pub async fn stock_us_valuation(
        &self,
        symbol: &str,
        indicator: &str,
        period: &str,
    ) -> Result<Vec<UsValuationBaidu>> {
        let url = "https://gushitong.baidu.com/opendata";
        let response = crate::util::send_and_check(
            self.get(url)
                .query(&[
                    ("openapi", "1"),
                    ("dspName", "iphone"),
                    ("tn", "tangram"),
                    ("client", "app"),
                    ("query", indicator),
                    ("code", symbol),
                    ("word", ""),
                    ("resource_id", "51171"),
                    ("market", "us"),
                    ("tag", indicator),
                    ("chart_select", period),
                    ("industry_select", ""),
                    ("skip_industry", "1"),
                    ("finClientType", "pc"),
                ]),
        )
        .await?;

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
            .ok_or_else(|| Error::upstream("baidu US valuation missing chart data"))?;

        let items: Vec<UsValuationBaidu> = body
            .iter()
            .filter_map(|item| {
                let arr = item.as_array()?;
                if arr.len() < 2 {
                    return None;
                }
                let date = arr[0].as_str().unwrap_or("").to_string();
                let value = arr[1].as_f64().unwrap_or(0.0);
                Some(UsValuationBaidu { date, value })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("baidu US valuation returned no data"));
        }
        Ok(items)
    }

    // -----------------------------------------------------------------------
    // US hot rank
    // -----------------------------------------------------------------------

    /// Get US stock hot rank from Eastmoney.
    ///
    /// Python equivalent: `stock_us_hot_rank()`
    pub async fn stock_us_hot_rank(&self) -> Result<Vec<UsHotRank>> {
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
            "marketType": "000001",
            "pageNo": 1,
            "pageSize": 100,
        });

        let response = crate::util::send_and_check(
            self.post(url).json(&payload),
        )
        .await?;

        let env: Env = response.json().await.map_err(Error::from)?;
        let rank_data = env
            .data
            .ok_or_else(|| Error::upstream("US hot rank missing data"))?;

        let mut items = Vec::new();
        for item in &rank_data {
            let sc = item.sc.as_deref().unwrap_or("");
            let parts: Vec<&str> = sc.split('|').collect();
            let code = parts.get(1).unwrap_or(&"").to_string();

            items.push(UsHotRank {
                rank: item.rk,
                code,
                name: String::new(),
                latest_price: None,
                change_pct: None,
            });
        }

        if items.is_empty() {
            return Err(Error::not_found("US hot rank returned no data"));
        }
        Ok(items)
    }

    /// Get US latest hot rank from Eastmoney.
    ///
    /// Python equivalent: `stock_us_hot_rank_latest(symbol)`
    pub async fn stock_us_hot_rank_latest(&self, symbol: &str) -> Result<Vec<UsHotRankDetail>> {
        let url = "https://emappdata.eastmoney.com/stockrank/getCurrentHkUsLatest";
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "marketType": "000001",
            "srcSecurityCode": format!("US|{}", symbol),
        });

        let response = crate::util::send_and_check(
            self.post(url).json(&payload),
        )
        .await?;

        let data: serde_json::Value = response.json().await.map_err(Error::from)?;
        let obj = data
            .get("data")
            .and_then(|d| d.as_object())
            .ok_or_else(|| Error::upstream("US hot rank latest missing data"))?;

        let mut items = Vec::new();
        for (key, val) in obj {
            let rank = val.as_i64().unwrap_or(0);
            items.push(UsHotRankDetail {
                time: key.clone(),
                rank,
                code: Some(symbol.to_string()),
            });
        }

        if items.is_empty() {
            return Err(Error::not_found("US hot rank latest returned no data"));
        }
        Ok(items)
    }

    /// Get US hot rank detail from Eastmoney.
    ///
    /// Python equivalent: `stock_us_hot_rank_detail(symbol)`
    pub async fn stock_us_hot_rank_detail(&self, symbol: &str) -> Result<Vec<UsHotRankDetail>> {
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
            "marketType": "000001",
            "srcSecurityCode": format!("US|{}", symbol),
        });

        let response = crate::util::send_and_check(
            self.post(url).json(&payload),
        )
        .await?;

        let env: Env = response.json().await.map_err(Error::from)?;
        let data = env
            .data
            .ok_or_else(|| Error::upstream("US hot rank detail missing data"))?;

        let items: Vec<UsHotRankDetail> = data
            .iter()
            .map(|item| UsHotRankDetail {
                time: item.dt.clone().unwrap_or_default(),
                rank: item.rk.unwrap_or(0),
                code: Some(symbol.to_string()),
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("US hot rank detail returned no data"));
        }
        Ok(items)
    }

    /// Get US realtime hot rank from Eastmoney.
    ///
    /// Python equivalent: `stock_us_hot_rank_detail_realtime(symbol)`
    pub async fn stock_us_hot_rank_detail_realtime(
        &self,
        symbol: &str,
    ) -> Result<Vec<UsHotRankDetail>> {
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
            "marketType": "000001",
            "srcSecurityCode": format!("US|{}", symbol),
        });

        let response = crate::util::send_and_check(
            self.post(url).json(&payload),
        )
        .await?;

        let env: Env = response.json().await.map_err(Error::from)?;
        let data = env
            .data
            .ok_or_else(|| Error::upstream("US realtime hot rank missing data"))?;

        let items: Vec<UsHotRankDetail> = data
            .iter()
            .map(|item| UsHotRankDetail {
                time: item.dt.clone().unwrap_or_default(),
                rank: item.rk.unwrap_or(0),
                code: Some(symbol.to_string()),
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("US realtime hot rank returned no data"));
        }
        Ok(items)
    }

    // -----------------------------------------------------------------------
    // US index
    // -----------------------------------------------------------------------

    /// Get US index spot data from Eastmoney.
    ///
    /// Python equivalent: `stock_us_index_spot_em()`
    pub async fn stock_us_index_spot_em(&self) -> Result<Vec<UsIndexSpotEm>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            diff: Option<Vec<serde_json::Value>>,
        }
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params("5000", &[
                    ("fid", "f3"),
                    ("fs", "i:100.NDX,i:100.DJIA,i:100.SPX"),
                    ("fields", "f1,f2,f3,f4,f5,f6,f7,f8,f12,f13,f14"),
                ])),
        )
        .await?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("US index spot missing data"))?;

        let mut items = Vec::new();
        for item in &diff {
            let code = item.str_or(&["f12"], "");
            let internal_id = item.str_or(&["f13"], "");
            let name = item.str_or(&["f14"], "");
            if code.is_empty() {
                continue;
            }
            items.push(UsIndexSpotEm {
                code,
                internal_id,
                name,
                latest_price: item.f64_field(&["f2"]),
                change_pct: item.f64_field(&["f3"]),
                change_amount: item.f64_field(&["f4"]),
                open: item.f64_field(&["f17"]),
                high: item.f64_field(&["f15"]),
                low: item.f64_field(&["f16"]),
                prev_close: item.f64_field(&["f18"]),
                volume: item.f64_field(&["f5"]),
                amount: item.f64_field(&["f6"]),
            });
        }

        if items.is_empty() {
            return Err(Error::not_found("US index spot returned no data"));
        }
        Ok(items)
    }

    /// Get US index daily K-line from Eastmoney.
    ///
    /// Python equivalent: `stock_us_index_daily_em(symbol)`
    ///
    /// - `symbol`: US index code like "NDX", "DJIA", "SPX"
    pub async fn stock_us_index_daily_em(&self, symbol: &str) -> Result<Vec<UsIndexDailyCandle>> {
        let secid = format!("100.{symbol}");

        let klines = self
            .kline_fetch(&secid, "101", "1", 10_000, &[("iscca", "1")])
            .await?;

        let items: Vec<UsIndexDailyCandle> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 5 {
                    return None;
                }
                Some(UsIndexDailyCandle {
                    date: parts[0].to_string(),
                    open: parts[1].parse().unwrap_or(0.0),
                    high: parts[3].parse().unwrap_or(0.0),
                    low: parts[4].parse().unwrap_or(0.0),
                    close: parts[2].parse().unwrap_or(0.0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("US index daily returned no data"));
        }
        Ok(items)
    }

    /// Get US index spot data from Sina.
    ///
    /// Python equivalent: `stock_us_index_spot_sina()`
    pub async fn stock_us_index_spot_sina(&self) -> Result<Vec<UsIndexSpotSina>> {
        let url = "https://hq.sinajs.cn/list=int_dji,int_nasdaq,int_sp500";
        let response = self
            .get(url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        let mut items = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || !line.contains('=') {
                continue;
            }
            let eq_pos = line.find('=').unwrap_or(0);
            let var_part = &line[..eq_pos];
            let val_part = &line[eq_pos + 2..]
                .trim_end_matches(';')
                .trim_end_matches('"');

            let code = var_part.split('_').next_back().unwrap_or("").to_string();
            let parts: Vec<&str> = val_part.split(',').collect();
            if parts.len() < 5 {
                continue;
            }
            let name = parts.first().unwrap_or(&"").to_string();
            let latest_price = parts.get(1).and_then(|s| s.parse::<f64>().ok());
            let change_amount = parts.get(2).and_then(|s| s.parse::<f64>().ok());
            let change_pct = parts.get(3).and_then(|s| s.parse::<f64>().ok());

            items.push(UsIndexSpotSina {
                code,
                name,
                latest_price,
                change_amount,
                change_pct,
                prev_close: None,
                open: None,
                high: None,
                low: None,
            });
        }

        if items.is_empty() {
            return Err(Error::not_found("US index spot sina returned no data"));
        }
        Ok(items)
    }

    /// Get US index daily data from Sina.
    ///
    /// Python equivalent: `stock_us_index_daily_sina(symbol)`
    ///
    /// - `symbol`: Sina index code like "int_dji", "int_nasdaq", "int_sp500"
    pub async fn stock_us_index_daily_sina(&self, symbol: &str) -> Result<Vec<UsIndexDailyCandle>> {
        let url = format!("https://finance.sina.com.cn/stock/usstock/{symbol}/klc_kl.js");

        let response = self
            .get(&url)
            .query(&[("d", "2023_5_01")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;

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

        let items: Vec<UsIndexDailyCandle> = data
            .iter()
            .filter_map(|v| {
                let arr = v.as_array()?;
                if arr.len() < 5 {
                    return None;
                }
                Some(UsIndexDailyCandle {
                    date: arr[0].as_str().unwrap_or("").to_string(),
                    open: arr[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    high: arr[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    low: arr[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    close: arr[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("US index daily sina returned no data"));
        }
        Ok(items)
    }

    // -----------------------------------------------------------------------
    // US financial indicator & dividend
    // -----------------------------------------------------------------------

    /// Get US financial indicators from Eastmoney.
    ///
    /// Python equivalent: `stock_us_financial_indicator(symbol)`
    pub async fn stock_us_financial_indicator(
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
            .query({
                let mut p = vec![
                    ("reportName", "RPT_USF10_FN_GMAININDICATOR"),
                    ("columns", "ALL"),
                    ("quoteColumns", ""),
                    ("filter", filter.as_str()),
                    ("pageNumber", "1"),
                    ("pageSize", ""),
                    ("sortTypes", "-1"),
                    ("sortColumns", "REPORT_DATE"),
                ];
                p.extend_from_slice(&crate::util::eastmoney_f10_params("F10"));
                p
            }.as_slice())
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("US financial indicators missing data"))?;

        if data.is_empty() {
            return Err(Error::not_found("US financial indicators returned no data"));
        }
        Ok(data)
    }

    /// Get US dividend payout from Eastmoney.
    ///
    /// Python equivalent: `stock_us_dividend_payout(symbol)`
    pub async fn stock_us_dividend_payout(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
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
            .query({
                let mut p = vec![
                    ("reportName", "RPT_USF10_FN_DIVIDEND"),
                    ("columns", "ALL"),
                    ("quoteColumns", ""),
                    ("filter", filter.as_str()),
                    ("pageNumber", "1"),
                    ("pageSize", ""),
                    ("sortTypes", "-1"),
                    ("sortColumns", "EX_DIVIDEND_DATE"),
                ];
                p.extend_from_slice(&crate::util::eastmoney_f10_params("F10"));
                p
            }.as_slice())
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("US dividend payout missing data"))?;

        if data.is_empty() {
            return Err(Error::not_found("US dividend payout returned no data"));
        }
        Ok(data)
    }

    /// Get US dividend yield from Legulegu.
    ///
    /// Python equivalent: `stock_us_gxl_lg()`
    ///
    /// Returns S&P 500 dividend yield data.
    pub async fn stock_us_gxl_lg(&self) -> Result<Vec<UsGxlLg>> {
        let url = "https://legulegu.com/api/stockdata/s-and-p-500";
        let response = self
            .get(url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let data: serde_json::Value = response.json().await.map_err(Error::from)?;
        let arr = data
            .as_array()
            .ok_or_else(|| Error::decode("legulegu response is not an array"))?;

        let items: Vec<UsGxlLg> = arr
            .iter()
            .filter_map(|item| {
                let date = item.get("date")?.as_str()?.to_string();
                let dv_ratio = item.get("dvRatio")?.as_f64()?;
                Some(UsGxlLg {
                    date,
                    dividend_yield: dv_ratio,
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("US dividend yield returned no data"));
        }
        Ok(items)
    }

    // -----------------------------------------------------------------------
    // US scale comparison
    // -----------------------------------------------------------------------

    /// Get US scale comparison from Eastmoney.
    ///
    /// Python equivalent: `stock_us_scale_comparison(symbol)`
    pub async fn stock_us_scale_comparison(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
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
            .query({
                let mut p = vec![
                    ("reportName", "RPT_PCF10_INDUSTRY_USSCALE"),
                    ("columns", "ALL"),
                    ("quoteColumns", ""),
                    ("filter", filter.as_str()),
                    ("pageNumber", ""),
                    ("pageSize", ""),
                    ("sortTypes", "1"),
                    ("sortColumns", "PAIMING"),
                ];
                p.extend_from_slice(&crate::util::eastmoney_f10_params("F10"));
                p
            }.as_slice())
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("US scale comparison missing data"))?;

        if data.is_empty() {
            return Err(Error::not_found("US scale comparison returned no data"));
        }
        Ok(data)
    }

    // -----------------------------------------------------------------------
    // US hot keyword
    // -----------------------------------------------------------------------

    /// Get US hot keyword from Eastmoney.
    ///
    /// Python equivalent: `stock_us_hot_keyword(symbol)`
    pub async fn stock_us_hot_keyword(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
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
            .query({
                let mut p = vec![
                    ("reportName", "RPT_USF10_HOT_KEYWORD"),
                    ("columns", "ALL"),
                    ("quoteColumns", ""),
                    ("filter", filter.as_str()),
                    ("pageNumber", "1"),
                    ("pageSize", ""),
                    ("sortTypes", "-1"),
                    ("sortColumns", "TRADE_DATE"),
                ];
                p.extend_from_slice(&crate::util::eastmoney_f10_params("F10"));
                p
            }.as_slice())
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("US hot keyword missing data"))?;

        if data.is_empty() {
            return Err(Error::not_found("US hot keyword returned no data"));
        }
        Ok(data)
    }
}
