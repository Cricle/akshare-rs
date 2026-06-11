//! AH comparison stock data — spot, daily, name.
//!
//! Covers Python functions:
//! - `stock_zh_ah_spot` — AH comparison spot data (Tencent)
//! - `stock_zh_ah_daily` — AH daily data (Tencent)
//! - `stock_zh_ah_name` — AH stock names (Tencent)

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// AH comparison spot quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhSpotQuote {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub buy_price: Option<f64>,
    #[serde(default)]
    pub sell_price: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub open: Option<f64>,
    #[serde(default)]
    pub prev_close: Option<f64>,
    #[serde(default)]
    pub high: Option<f64>,
    #[serde(default)]
    pub low: Option<f64>,
}

/// AH daily candle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhDailyCandle {
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

/// AH stock name entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhStockName {
    pub code: String,
    pub name: String,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get AH comparison spot data from Tencent.
    ///
    /// Python equivalent: `stock_zh_ah_spot()`
    pub async fn stock_zh_ah_spot(&self) -> Result<Vec<AhSpotQuote>> {
        let mut all_quotes = Vec::new();

        // Use the Tencent AH stock list API
        let url = "https://proxy.finance.qq.com/cgi/cgi-bin/mstats/hk_ah";
        for page in 0..10 {
            let page_str = page.to_string();
            let response = self
                                .get(url)
                .query(&[
                    ("reqPage", page_str.as_str()),
                    ("type", "AH"),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let text = response.text().await.map_err(Error::from)?;

            // Parse the response - try JSON first
            let json_start = text.find('{').unwrap_or(0);
            let json_end = text.rfind('}').map(|i| i + 1).unwrap_or(text.len());
            let json_text = &text[json_start..json_end];

            let data: serde_json::Value = match serde_json::from_str(json_text) {
                Ok(v) => v,
                Err(_) => break,
            };

            let page_data = data
                .get("data")
                .and_then(|d| d.get("page_data"))
                .and_then(|p| p.as_array());

            let page_data = match page_data {
                Some(arr) if !arr.is_empty() => arr,
                _ => break,
            };

            for entry in page_data {
                // Each entry is a tilde-separated string
                let entry_str = entry.as_str().unwrap_or("");
                let parts: Vec<&str> = entry_str.split('~').collect();
                if parts.len() < 13 {
                    continue;
                }
                all_quotes.push(AhSpotQuote {
                    code: parts[0].to_string(),
                    name: parts[1].to_string(),
                    latest_price: parts[2].parse().ok(),
                    change_pct: parts[3].parse().ok(),
                    change_amount: parts[4].parse().ok(),
                    buy_price: parts[5].parse().ok(),
                    sell_price: parts[6].parse().ok(),
                    volume: parts[7].parse().ok(),
                    amount: parts[8].parse().ok(),
                    open: parts[9].parse().ok(),
                    prev_close: parts[10].parse().ok(),
                    high: parts[11].parse().ok(),
                    low: parts.get(12).and_then(|s| s.parse().ok()),
                });
            }
        }

        if all_quotes.is_empty() {
            return Err(Error::not_found("tencent returned no AH spot data"));
        }
        Ok(all_quotes)
    }

    /// Get AH daily data from Tencent.
    ///
    /// Python equivalent: `stock_zh_ah_daily(symbol, start_year, end_year, adjust)`
    ///
    /// - `symbol`: stock code like "02318"
    /// - `start_year`: "2000"
    /// - `end_year`: "2019"
    /// - `adjust`: "", "qfq", "hfq"
    pub async fn stock_zh_ah_daily(
        &self,
        symbol: &str,
        start_year: &str,
        end_year: &str,
        adjust: &str,
    ) -> Result<Vec<AhDailyCandle>> {
        let mut all_candles = Vec::new();
        let start: i32 = start_year.parse().unwrap_or(2000);
        let end: i32 = end_year.parse().unwrap_or(2024);

        for year in start..=end {
            let (url, param_key) = if adjust.is_empty() {
                (
                    "http://web.ifzq.gtimg.cn/appstock/app/kline/kline".to_string(),
                    "param",
                )
            } else {
                (
                    "https://web.ifzq.gtimg.cn/appstock/app/hkfqkline/get".to_string(),
                    "param",
                )
            };

            let param_value = if adjust.is_empty() {
                format!("hk{},day,{}-01-01,{}-12-31,640,", symbol, year, year + 1)
            } else {
                format!(
                    "hk{},day,{}-01-01,{}-12-31,640,{}",
                    symbol,
                    year,
                    year + 1,
                    adjust
                )
            };

            let r: String = format!("{}", chrono::Utc::now().timestamp_millis() as f64 / 1000.0);

            let response = self
                                .get(&url)
                .query(&[
                    ("_var", format!("kline_day{}{}", adjust, year).as_str()),
                    (param_key, param_value.as_str()),
                    ("r", r.as_str()),
                ])
                .send()
                .await;

            let response = match response {
                Ok(r) => r,
                Err(_) => continue,
            };

            let text = match response.text().await {
                Ok(t) => t,
                Err(_) => continue,
            };

            // Parse response
            let json_start = text.find('{').unwrap_or(0);
            let json_end = text.rfind('}').map(|i| i + 1).unwrap_or(text.len());
            let json_text = &text[json_start..json_end];

            let data: serde_json::Value = match serde_json::from_str(json_text) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let kline_key = if adjust.is_empty() {
                "day"
            } else {
                &format!("{}day", adjust)
            };

            let klines = data
                .get("data")
                .and_then(|d| d.get(format!("hk{}", symbol)))
                .and_then(|v| v.get(kline_key))
                .and_then(|v| v.as_array());

            let klines = match klines {
                Some(arr) => arr,
                None => continue,
            };

            for entry in klines {
                let arr = match entry.as_array() {
                    Some(a) => a,
                    None => continue,
                };
                if arr.len() < 6 {
                    continue;
                }
                all_candles.push(AhDailyCandle {
                    date: arr[0].as_str().unwrap_or("").to_string(),
                    open: arr[1].as_f64().unwrap_or(0.0),
                    close: arr[2].as_f64().unwrap_or(0.0),
                    high: arr[3].as_f64().unwrap_or(0.0),
                    low: arr[4].as_f64().unwrap_or(0.0),
                    volume: arr[5].as_f64().unwrap_or(0.0),
                });
            }
        }

        if all_candles.is_empty() {
            return Err(Error::not_found("AH daily returned no data"));
        }
        Ok(all_candles)
    }

    /// Get AH stock names from Tencent.
    ///
    /// Python equivalent: `stock_zh_ah_name()`
    pub async fn stock_zh_ah_name(&self) -> Result<Vec<AhStockName>> {
        let mut all_names = Vec::new();
        let url = "https://proxy.finance.qq.com/cgi/cgi-bin/mstats/hk_ah";

        for page in 0..10 {
            let page_str = page.to_string();
            let response = self
                                .get(url)
                .query(&[
                    ("reqPage", page_str.as_str()),
                    ("type", "AH"),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let text = response.text().await.map_err(Error::from)?;

            let json_start = text.find('{').unwrap_or(0);
            let json_end = text.rfind('}').map(|i| i + 1).unwrap_or(text.len());
            let json_text = &text[json_start..json_end];

            let data: serde_json::Value = match serde_json::from_str(json_text) {
                Ok(v) => v,
                Err(_) => break,
            };

            let page_data = data
                .get("data")
                .and_then(|d| d.get("page_data"))
                .and_then(|p| p.as_array());

            let page_data = match page_data {
                Some(arr) if !arr.is_empty() => arr,
                _ => break,
            };

            for entry in page_data {
                let entry_str = entry.as_str().unwrap_or("");
                let parts: Vec<&str> = entry_str.split('~').collect();
                if parts.len() < 2 {
                    continue;
                }
                all_names.push(AhStockName {
                    code: parts[0].to_string(),
                    name: parts[1].to_string(),
                });
            }
        }

        if all_names.is_empty() {
            return Err(Error::not_found("tencent returned no AH stock names"));
        }
        Ok(all_names)
    }
}
