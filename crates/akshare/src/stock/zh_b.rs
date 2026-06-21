//! B-share stock data — spot, daily, minute.
//!
//! Covers Python functions:
//! - `stock_zh_b_spot` — B-share spot quotes (Sina)
//! - `stock_zh_b_daily` — B-share daily candles (Sina, via Eastmoney kline)
//! - `stock_zh_b_minute` — B-share minute candles (Sina)

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::market::eastmoney_secid;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// B-share spot quote from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhBSpotQuote {
    pub symbol: String,
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

/// B-share daily candle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhBDailyCandle {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    #[serde(default)]
    pub amount: Option<f64>,
}

/// B-share minute candle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhBMinuteCandle {
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get all B-share spot quotes from Sina.
    ///
    /// Python equivalent: `stock_zh_b_spot()`
    pub async fn stock_zh_b_spot(&self) -> Result<Vec<ZhBSpotQuote>> {
        let count_url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeStockCount";
        let count_resp = self
            .get(count_url)
            .query(&[("node", "hs_b")])
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
                    ("node", "hs_b"),
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

                all_quotes.push(ZhBSpotQuote {
                    symbol,
                    name,
                    latest_price: parse_b_f64(item, "trade"),
                    change_amount: parse_b_f64(item, "pricechange"),
                    change_pct: parse_b_f64(item, "changepercent"),
                    buy_price: parse_b_f64(item, "buy"),
                    sell_price: parse_b_f64(item, "sell"),
                    prev_close: parse_b_f64(item, "settlement"),
                    open: parse_b_f64(item, "open"),
                    high: parse_b_f64(item, "high"),
                    low: parse_b_f64(item, "low"),
                    volume: parse_b_f64(item, "volume"),
                    amount: parse_b_f64(item, "amount"),
                });
            }
        }

        if all_quotes.is_empty() {
            return Err(Error::not_found("sina returned no B-share spot data"));
        }
        Ok(all_quotes)
    }

    /// Get B-share daily candles. Uses Eastmoney kline API.
    ///
    /// Python equivalent: `stock_zh_b_daily(symbol, start_date, end_date, adjust)`
    ///
    /// - `symbol`: stock code like "sh900901" or "900901"
    /// - `start_date`: "19900101"
    /// - `end_date": "21000118"
    /// - `adjust`: "", "qfq", "hfq"
    pub async fn stock_zh_b_daily(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<ZhBDailyCandle>> {
        let secid = eastmoney_secid(symbol)?;
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

        let items: Vec<ZhBDailyCandle> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 7 {
                    return None;
                }
                Some(ZhBDailyCandle {
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
            return Err(Error::not_found("B-share daily returned no data"));
        }
        Ok(items)
    }

    /// Get B-share minute candles from Sina.
    ///
    /// Python equivalent: `stock_zh_b_minute(symbol, period)`
    ///
    /// - `symbol`: stock code like "sh900901"
    /// - `period`: "1", "5", "15", "30", "60"
    pub async fn stock_zh_b_minute(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<ZhBMinuteCandle>> {
        // Normalize symbol to Sina format
        let sina_symbol = if symbol.starts_with("sh") || symbol.starts_with("sz") {
            symbol.to_string()
        } else {
            // B-share: 900xxx -> sh, 200xxx -> sz
            let code = symbol.trim_start_matches('0');
            if code.starts_with('9') {
                format!("sh{symbol}")
            } else {
                format!("sz{symbol}")
            }
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

        let items: Vec<ZhBMinuteCandle> = data
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
                Some(ZhBMinuteCandle {
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
            return Err(Error::not_found("B-share minute returned no data"));
        }
        Ok(items)
    }
}

fn parse_b_f64(item: &serde_json::Value, key: &str) -> Option<f64> {
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
