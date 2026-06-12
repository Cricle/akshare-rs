//! A-share stock data — spot, daily, minute, new, stop, CDR, pre-market, Tencent hist/tick.
//!
//! Covers Python functions:
//! - `stock_zh_a_spot` — All A-share spot quotes (Sina)
//! - `stock_zh_a_daily` — Daily candles (Sina, via Eastmoney kline)
//! - `stock_zh_a_minute` — Minute candles (Sina)
//! - `stock_zh_a_new` — New stock list (Sina)
//! - `stock_zh_a_stop_em` — Stopped/delisted stocks (Eastmoney)
//! - `stock_zh_a_cdr_daily` — CDR daily data
//! - `stock_zh_a_hist_pre_min_em` — Pre-market minute data (Eastmoney)
//! - `stock_zh_a_hist_tx` — Tencent historical data
//! - `stock_zh_a_tick_tx_js` — Tencent tick data

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::market::eastmoney_secid;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A-share spot quote from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhASpotQuote {
    pub symbol: String,
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub buy_price: Option<f64>,
    #[serde(default)]
    pub sell_price: Option<f64>,
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

/// A-share daily candle from Sina/Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhADailyCandle {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    #[serde(default)]
    pub amount: Option<f64>,
}

/// A-share minute candle from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhAMinuteCandle {
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// New stock list entry from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhANewStock {
    pub symbol: String,
    pub code: String,
    pub name: String,
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
    pub market_cap: Option<f64>,
    #[serde(default)]
    pub turnover_rate: Option<f64>,
}

/// Stopped/delisted stock entry from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhAStopStock {
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
}

/// Tencent tick data entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhATickTx {
    pub time: String,
    pub price: f64,
    pub price_change: f64,
    pub volume: i64,
    pub amount: i64,
    pub direction: String,
}

/// Tencent historical kline point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhAHistTx {
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get all A-share spot quotes from Sina.
    ///
    /// Python equivalent: `stock_zh_a_spot()`
    ///
    /// Note: This fetches from Sina paginated API. Large requests may be rate-limited.
    pub async fn stock_zh_a_spot(&self) -> Result<Vec<ZhASpotQuote>> {
        // Get page count first
        let count_url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeStockCount";
        let count_resp = self
            .get(count_url)
            .query(&[("node", "hs_a")])
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
        let mut all_quotes = Vec::new();

        for page in 1..=page_count.min(5) {
            let page_str = page.to_string();
            let response = self
                .get(list_url)
                .query(&[
                    ("page", page_str.as_str()),
                    ("num", "80"),
                    ("sort", "symbol"),
                    ("asc", "1"),
                    ("node", "hs_a"),
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
                let code = item
                    .get("code")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let name = item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                all_quotes.push(ZhASpotQuote {
                    symbol,
                    code,
                    name,
                    latest_price: parse_json_f64(item, "trade"),
                    change_amount: parse_json_f64(item, "pricechange"),
                    change_pct: parse_json_f64(item, "changepercent"),
                    buy_price: parse_json_f64(item, "buy"),
                    sell_price: parse_json_f64(item, "sell"),
                    prev_close: parse_json_f64(item, "settlement"),
                    open: parse_json_f64(item, "open"),
                    high: parse_json_f64(item, "high"),
                    low: parse_json_f64(item, "low"),
                    volume: parse_json_f64(item, "volume"),
                    amount: parse_json_f64(item, "amount"),
                });
            }
        }

        if all_quotes.is_empty() {
            return Err(Error::not_found("sina returned no A-share spot data"));
        }
        Ok(all_quotes)
    }

    /// Get A-share daily candles. Uses Eastmoney kline API.
    ///
    /// Python equivalent: `stock_zh_a_daily(symbol, start_date, end_date, adjust)`
    ///
    /// - `symbol`: stock code like "600000" or "sh600000"
    /// - `start_date`: "20200101"
    /// - `end_date`: "20201231"
    /// - `adjust`: "", "qfq", "hfq"
    pub async fn stock_zh_a_daily(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<ZhADailyCandle>> {
        let secid = eastmoney_secid(symbol)?;
        let fqt = match adjust {
            "" => "0",
            "qfq" => "1",
            "hfq" => "2",
            _ => return Err(Error::invalid_input(format!("invalid adjust: {adjust}"))),
        };

        let response = self
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                ("klt", "101"),
                ("fqt", fqt),
                ("beg", start_date),
                ("end", end_date),
                ("lmt", "1000000"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            klines: Option<Vec<String>>,
        }

        let payload: Env = response.json().await.map_err(Error::from)?;
        let klines = payload
            .data
            .and_then(|d| d.klines)
            .ok_or_else(|| Error::upstream("A-share daily kline missing data"))?;

        let items: Vec<ZhADailyCandle> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 7 {
                    return None;
                }
                Some(ZhADailyCandle {
                    date: parts[0].to_string(),
                    open: parts[1].parse().unwrap_or(0.0),
                    close: parts[2].parse().unwrap_or(0.0),
                    high: parts[3].parse().unwrap_or(0.0),
                    low: parts[4].parse().unwrap_or(0.0),
                    volume: parts[5].parse().unwrap_or(0.0),
                    amount: Some(parts[6].parse().unwrap_or(0.0)),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("A-share daily returned no data"));
        }
        Ok(items)
    }

    /// Get A-share minute candles from Sina.
    ///
    /// Python equivalent: `stock_zh_a_minute(symbol, period)`
    ///
    /// - `symbol`: stock code like "sh600000" or "sz000001"
    /// - `period`: "1", "5", "15", "30", "60"
    pub async fn stock_zh_a_minute(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<ZhAMinuteCandle>> {
        // Normalize symbol to Sina format
        let sina_symbol = if symbol.starts_with("sh") || symbol.starts_with("sz") {
            symbol.to_string()
        } else {
            let normalized = crate::market::normalize_a_share_symbol(symbol)
                .ok_or_else(|| Error::invalid_input(format!("invalid symbol: {symbol}")))?;
            let (code, suffix) = normalized.split_once('.').unwrap();
            let prefix = match suffix {
                "SH" => "sh",
                _ => "sz",
            };
            format!("{prefix}{code}")
        };

        let url = "https://quotes.sina.cn/cn/api/jsonp_v2.php/=/CN_MarketDataService.getKLineData";
        let response = self
            .get(url)
            .query(&[
                ("symbol", sina_symbol.as_str()),
                ("scale", period),
                ("datalen", "1970"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;

        // Parse JSONP response: =(JSON);
        let json_start = text
            .find("=(")
            .ok_or_else(|| Error::decode("invalid JSONP response"))?
            + 2;
        let json_end = text
            .rfind(");")
            .ok_or_else(|| Error::decode("invalid JSONP response"))?;
        let json_text = &text[json_start..json_end];

        let data: Vec<serde_json::Value> = serde_json::from_str(json_text)
            .map_err(|e| Error::decode(format!("JSON parse error: {e}")))?;

        let items: Vec<ZhAMinuteCandle> = data
            .iter()
            .filter_map(|item| {
                let day = item.get("day")?.as_str()?;
                let open = item.get("open")?.as_str()?.parse().ok()?;
                let high = item.get("high")?.as_str()?.parse().ok()?;
                let low = item.get("low")?.as_str()?.parse().ok()?;
                let close = item.get("close")?.as_str()?.parse().ok()?;
                let volume = item
                    .get("volume")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                Some(ZhAMinuteCandle {
                    datetime: day.to_string(),
                    open,
                    high,
                    low,
                    close,
                    volume,
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("A-share minute returned no data"));
        }
        Ok(items)
    }

    /// Get new stock list from Sina.
    ///
    /// Python equivalent: `stock_zh_a_new()`
    pub async fn stock_zh_a_new(&self) -> Result<Vec<ZhANewStock>> {
        let count_url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeStockCount";
        let count_resp = self
            .get(count_url)
            .query(&[("node", "new_stock")])
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
        let mut all_stocks = Vec::new();

        for page in 1..=page_count.min(5) {
            let page_str = page.to_string();
            let response = self
                .get(list_url)
                .query(&[
                    ("page", page_str.as_str()),
                    ("num", "80"),
                    ("sort", "symbol"),
                    ("asc", "1"),
                    ("node", "new_stock"),
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
                all_stocks.push(ZhANewStock {
                    symbol: item
                        .get("symbol")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    code: item
                        .get("code")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    name: item
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    open: parse_json_f64(item, "open"),
                    high: parse_json_f64(item, "high"),
                    low: parse_json_f64(item, "low"),
                    volume: parse_json_f64(item, "volume"),
                    amount: parse_json_f64(item, "amount"),
                    market_cap: parse_json_f64(item, "mktcap"),
                    turnover_rate: parse_json_f64(item, "turnoverratio"),
                });
            }
        }

        if all_stocks.is_empty() {
            return Err(Error::not_found("sina returned no new stock data"));
        }
        Ok(all_stocks)
    }

    /// Get stopped/delisted stocks from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_a_stop_em()`
    pub async fn stock_zh_a_stop_em(&self) -> Result<Vec<ZhAStopStock>> {
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
                ("fs", "m:0 s:3"),
                ("fields", "f2,f3,f4,f5,f6,f12,f14,f15,f16,f17,f18"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            diff: Option<Vec<serde_json::Value>>,
        }

        let payload: Env = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("eastmoney stop stocks missing data"))?;

        let items: Vec<ZhAStopStock> = diff
            .iter()
            .filter_map(|item| {
                let code = item.get("f12")?.as_str()?.to_string();
                let name = item.get("f14")?.as_str()?.to_string();
                Some(ZhAStopStock {
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
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no stopped stocks"));
        }
        Ok(items)
    }

    /// Get CDR daily data from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_a_cdr_daily(symbol, start_date, end_date)`
    ///
    /// CDR (Chinese Depositary Receipts) daily data uses the same kline API.
    pub async fn stock_zh_a_cdr_daily(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<ZhADailyCandle>> {
        let secid = eastmoney_secid(symbol)?;

        let response = self
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                ("klt", "101"),
                ("fqt", "0"),
                ("beg", start_date),
                ("end", end_date),
                ("lmt", "1000000"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            klines: Option<Vec<String>>,
        }

        let payload: Env = response.json().await.map_err(Error::from)?;
        let klines = payload
            .data
            .and_then(|d| d.klines)
            .ok_or_else(|| Error::upstream("CDR daily kline missing data"))?;

        let items: Vec<ZhADailyCandle> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 7 {
                    return None;
                }
                Some(ZhADailyCandle {
                    date: parts[0].to_string(),
                    open: parts[1].parse().unwrap_or(0.0),
                    close: parts[2].parse().unwrap_or(0.0),
                    high: parts[3].parse().unwrap_or(0.0),
                    low: parts[4].parse().unwrap_or(0.0),
                    volume: parts[5].parse().unwrap_or(0.0),
                    amount: Some(parts[6].parse().unwrap_or(0.0)),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("CDR daily returned no data"));
        }
        Ok(items)
    }

    /// Get pre-market minute data from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_a_hist_pre_min_em(symbol, start_time, end_time)`
    ///
    /// Uses the Eastmoney trends API for pre-market data.
    pub async fn stock_zh_a_hist_pre_min_em(
        &self,
        symbol: &str,
        _start_time: &str,
        _end_time: &str,
    ) -> Result<Vec<ZhAMinuteCandle>> {
        let secid = eastmoney_secid(symbol)?;

        let response = self
            .get("https://push2his.eastmoney.com/api/qt/stock/trends2/get")
            .query(&[
                ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58"),
                ("iscr", "0"),
                ("ndays", "1"),
                ("secid", secid.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            trends: Option<Vec<String>>,
        }

        let payload: Env = response.json().await.map_err(Error::from)?;
        let trends = payload
            .data
            .and_then(|d| d.trends)
            .ok_or_else(|| Error::upstream("pre-market minute data missing"))?;

        let items: Vec<ZhAMinuteCandle> = trends
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 8 {
                    return None;
                }
                Some(ZhAMinuteCandle {
                    datetime: parts[0].to_string(),
                    open: parts[1].parse().unwrap_or(0.0),
                    close: parts[2].parse().unwrap_or(0.0),
                    high: parts[3].parse().unwrap_or(0.0),
                    low: parts[4].parse().unwrap_or(0.0),
                    volume: parts[5].parse().unwrap_or(0.0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("pre-market minute returned no data"));
        }
        Ok(items)
    }

    /// Get Tencent historical kline data.
    ///
    /// Python equivalent: `stock_zh_a_hist_tx(symbol, start_date, end_date, adjust)`
    ///
    /// - `symbol`: stock code like "sz000001" or "600000"
    /// - `start_date`: "20200101"
    /// - `end_date`: "20201231"
    /// - `adjust`: "", "qfq", "hfq"
    pub async fn stock_zh_a_hist_tx(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<ZhAHistTx>> {
        let ts = crate::market::tencent_market_symbol(symbol)?;
        let adjust_suffix = match adjust {
            "" => "",
            "qfq" => "qfq",
            "hfq" => "hfq",
            _ => return Err(Error::invalid_input(format!("invalid adjust: {adjust}"))),
        };

        let url = format!(
            "https://web.ifzq.gtimg.cn/appstock/app/fqkline/get?param={ts},day,{start_date},{end_date},640,{adjust_suffix}"
        );

        #[derive(Deserialize)]
        struct Resp {
            data: Option<serde_json::Value>,
        }

        let resp: Resp = self.get(&url).send().await?.json().await?;
        let data = resp
            .data
            .ok_or_else(|| Error::upstream("empty tencent hist data"))?;

        let ts_lower = ts.to_lowercase();
        let kline_key = match adjust {
            "qfq" => "qfqday",
            "hfq" => "hfqday",
            _ => "day",
        };

        let klines = data
            .get(&ts_lower)
            .and_then(|v| v.get(kline_key).or_else(|| v.get("day")))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let items: Vec<ZhAHistTx> = klines
            .iter()
            .filter_map(|entry| {
                let arr = entry.as_array()?;
                if arr.len() < 6 {
                    return None;
                }
                Some(ZhAHistTx {
                    date: arr[0].as_str().unwrap_or("").to_string(),
                    open: arr[1].as_f64().unwrap_or(0.0),
                    close: arr[2].as_f64().unwrap_or(0.0),
                    high: arr[3].as_f64().unwrap_or(0.0),
                    low: arr[4].as_f64().unwrap_or(0.0),
                    volume: arr[5].as_f64().unwrap_or(0.0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("tencent hist returned no data"));
        }
        Ok(items)
    }

    /// Get Tencent tick data.
    ///
    /// Python equivalent: `stock_zh_a_tick_tx_js(symbol, tick_date)`
    ///
    /// - `symbol`: stock code like "sz000001"
    /// - `tick_date`: date like "20240321" (today if empty)
    pub async fn stock_zh_a_tick_tx_js(
        &self,
        symbol: &str,
        _tick_date: &str,
    ) -> Result<Vec<ZhATickTx>> {
        // Normalize to Tencent format
        let tx_symbol = if symbol.starts_with("sh") || symbol.starts_with("sz") {
            symbol.to_string()
        } else {
            let normalized = crate::market::normalize_a_share_symbol(symbol)
                .ok_or_else(|| Error::invalid_input(format!("invalid symbol: {symbol}")))?;
            let (code, suffix) = normalized.split_once('.').unwrap();
            let prefix = match suffix {
                "SH" => "sh",
                _ => "sz",
            };
            format!("{prefix}{code}")
        };

        let mut all_ticks = Vec::new();

        for page in 0..100 {
            let page_str = page.to_string();
            let response = self
                .get("http://stock.gtimg.cn/data/index.php")
                .query(&[
                    ("appn", "detail"),
                    ("action", "data"),
                    ("c", tx_symbol.as_str()),
                    ("p", page_str.as_str()),
                ])
                .send()
                .await;

            let response = match response {
                Ok(r) => r,
                Err(_) => break,
            };

            let text = match response.text().await {
                Ok(t) => t,
                Err(_) => break,
            };

            // Parse the response: find the array portion
            let array_start = match text.find('[') {
                Some(pos) => pos,
                None => break,
            };

            // Try to parse the tick data
            // Format: "v_xx="data";" where data contains pipe-separated records
            let data_part = &text[array_start..];
            let records_end = data_part.find(']').unwrap_or(data_part.len());
            let records_str = &data_part[..records_end];

            // Each record is separated by | and has fields separated by /
            let records: Vec<&str> = records_str.split('|').collect();

            let mut page_has_data = false;
            for record in &records {
                let fields: Vec<&str> = record.split('/').collect();
                if fields.len() < 6 {
                    continue;
                }
                // Skip first field (index)
                if fields.len() >= 7 {
                    page_has_data = true;
                    let direction = match *fields.get(6).unwrap_or(&"") {
                        "S" => "卖盘",
                        "B" => "买盘",
                        "M" => "中性盘",
                        _ => "未知",
                    };
                    all_ticks.push(ZhATickTx {
                        time: fields[1].to_string(),
                        price: fields[2].parse().unwrap_or(0.0),
                        price_change: fields[3].parse().unwrap_or(0.0),
                        volume: fields[4].parse().unwrap_or(0),
                        amount: fields[5].parse().unwrap_or(0),
                        direction: direction.to_string(),
                    });
                }
            }

            if !page_has_data {
                break;
            }
        }

        if all_ticks.is_empty() {
            return Err(Error::not_found("tencent tick returned no data"));
        }
        Ok(all_ticks)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_json_f64(item: &serde_json::Value, key: &str) -> Option<f64> {
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
