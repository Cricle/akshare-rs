//! US stock extra data — daily, spot, famous, pink, valuation.
//!
//! Covers Python functions:
//! - `stock_us_daily` — US daily candles (Sina)
//! - `stock_us_spot` — US spot quotes (Sina)
//! - `stock_us_famous_spot_em` — Famous US stocks (Eastmoney)
//! - `stock_us_pink_spot_em` — Pink sheet stocks (Eastmoney)
//! - `stock_us_valuation_baidu` — US valuation (Baidu)

use crate::client::AkShareClient;
use crate::error::{Error, Result};

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
        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            klines: Option<Vec<String>>,
        }
        // Normalize to Eastmoney secid format for US stocks
        let secid = if symbol.contains('.') {
            symbol.to_string()
        } else {
            format!("105.{}", symbol.to_uppercase())
        };

        let response = self
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                ("klt", "101"),
                ("fqt", "1"),
                ("beg", start_date),
                ("end", end_date),
                ("lmt", "1000000"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let klines = payload
            .data
            .and_then(|d| d.klines)
            .ok_or_else(|| Error::upstream("US daily kline missing data"))?;

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
                ("fs", "m:105,m:106,m:107"),
                ("fields", "f2,f3,f4,f5,f6,f12,f14"),
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
                    latest_price: item.get("f2").and_then(serde_json::Value::as_f64),
                    change_pct: item.get("f3").and_then(serde_json::Value::as_f64),
                    change_amount: item.get("f4").and_then(serde_json::Value::as_f64),
                    volume: item.get("f5").and_then(serde_json::Value::as_f64),
                    open: None,
                    high: None,
                    low: None,
                    prev_close: None,
                    market_cap: item.get("f6").and_then(serde_json::Value::as_f64),
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
    /// Python equivalent: `stock_us_famous_spot_em(symbol)`
    ///
    /// - `symbol`: category like "科技类", "金融类", "医药食品类", "媒体类", "汽车能源类", "制造零售类"
    pub async fn stock_us_famous_spot_em(&self, symbol: &str) -> Result<Vec<UsFamousStock>> {
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
                ("fs", fs.as_str()),
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
            .ok_or_else(|| Error::upstream("US famous stocks missing data"))?;

        let mut items = Vec::new();

        if let Some(arr) = diff.as_array() {
            for item in arr {
                if let (Some(code), Some(name)) = (
                    item.get("f12").and_then(|v| v.as_str()),
                    item.get("f14").and_then(|v| v.as_str()),
                ) {
                    items.push(UsFamousStock {
                        code: code.to_string(),
                        name: name.to_string(),
                        latest_price: item.get("f2").and_then(serde_json::Value::as_f64),
                        change_pct: item.get("f3").and_then(serde_json::Value::as_f64),
                        change_amount: item.get("f4").and_then(serde_json::Value::as_f64),
                        volume: item.get("f5").and_then(serde_json::Value::as_f64),
                        amount: item.get("f6").and_then(serde_json::Value::as_f64),
                        open: item.get("f17").and_then(serde_json::Value::as_f64),
                        high: item.get("f15").and_then(serde_json::Value::as_f64),
                        low: item.get("f16").and_then(serde_json::Value::as_f64),
                        prev_close: item.get("f18").and_then(serde_json::Value::as_f64),
                        market_cap: item.get("f20").and_then(serde_json::Value::as_f64),
                        pe_ratio: item.get("f9").and_then(serde_json::Value::as_f64),
                    });
                }
            }
        } else if let Some(obj) = diff.as_object() {
            for (_, val) in obj {
                if let (Some(code), Some(name)) = (
                    val.get("f12").and_then(|v| v.as_str()),
                    val.get("f14").and_then(|v| v.as_str()),
                ) {
                    items.push(UsFamousStock {
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
        }

        if items.is_empty() {
            return Err(Error::not_found("US famous stocks returned no data"));
        }
        Ok(items)
    }

    /// Get pink sheet stocks from Eastmoney.
    ///
    /// Python equivalent: `stock_us_pink_spot_em()`
    pub async fn stock_us_pink_spot_em(&self) -> Result<Vec<UsPinkStock>> {
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
                ("fltt", "1"),
                ("invt", "1"),
                ("fid", "f3"),
                ("fs", "m:153"),
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
            .ok_or_else(|| Error::upstream("US pink stocks missing data"))?;

        let items: Vec<UsPinkStock> = diff
            .iter()
            .filter_map(|item| {
                let code = item.get("f12")?.as_str()?.to_string();
                let name = item.get("f14")?.as_str()?.to_string();
                Some(UsPinkStock {
                    code,
                    name,
                    latest_price: item.get("f2").and_then(serde_json::Value::as_f64),
                    change_pct: item.get("f3").and_then(serde_json::Value::as_f64),
                    change_amount: item.get("f4").and_then(serde_json::Value::as_f64),
                    volume: item.get("f5").and_then(serde_json::Value::as_f64),
                    amount: item.get("f6").and_then(serde_json::Value::as_f64),
                    open: item.get("f17").and_then(serde_json::Value::as_f64),
                    high: item.get("f15").and_then(serde_json::Value::as_f64),
                    low: item.get("f16").and_then(serde_json::Value::as_f64),
                    prev_close: item.get("f18").and_then(serde_json::Value::as_f64),
                    market_cap: item.get("f20").and_then(serde_json::Value::as_f64),
                    pe_ratio: item.get("f9").and_then(serde_json::Value::as_f64),
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
    /// Python equivalent: `stock_us_valuation_baidu(symbol, indicator, period)`
    ///
    /// - `symbol`: US stock code like "NVDA"
    /// - `indicator`: "总市值", "市盈率(TTM)", "市盈率(静)", "市净率", "市现率"
    /// - `period`: "近一年", "近三年", "全部"
    pub async fn stock_us_valuation_baidu(
        &self,
        symbol: &str,
        indicator: &str,
        period: &str,
    ) -> Result<Vec<UsValuationBaidu>> {
        let url = "https://gushitong.baidu.com/opendata";
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
                ("market", "us"),
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
}
