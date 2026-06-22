//! Futures data from Sina Finance.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, Row};
use crate::util::{amplitude_pct, apply_change_metrics};

impl AkShareClient {
    /// Fetch main contract futures candles from Sina Finance.
    ///
    /// `symbol` should be a Sina futures symbol, e.g.:
    /// - `"nf_AG0"` — Silver main contract
    /// - `"nf_CU0"` — Copper main contract
    /// - `"nf_SC0"` — Crude oil (Shanghai INE)
    /// - `"nf_RB0"` — Rebar
    /// - `"nf_IF0"` — CSI 300 index futures
    pub async fn futures_main(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        let url = format!(
            "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_=/InnerFuturesNewService.getDailyKLine?symbol={symbol}&_={}",
            chrono::Utc::now().timestamp_millis()
        );
        let body = self
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        // Extract JSON array from JSONP callback: var _=([[...]])
        let json_str = body
            .find("[[")
            .and_then(|start| {
                body[start..]
                    .rfind("]]")
                    .map(|end| &body[start..start + end + 2])
            })
            .ok_or_else(|| Error::decode("sina futures: invalid JSONP response"))?;

        // Parse as Vec<Vec<...>> where each inner is [date, open, high, low, close, volume, ...]
        let rows: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str).unwrap_or_default();

        let mut items = Vec::with_capacity(rows.len());
        for row in &rows {
            if row.len() < 6 {
                continue;
            }
            let date = row[0].as_str().unwrap_or("").to_string();
            let open = row[1].as_f64().unwrap_or(0.0);
            let high = row[2].as_f64().unwrap_or(0.0);
            let low = row[3].as_f64().unwrap_or(0.0);
            let close = row[4].as_f64().unwrap_or(0.0);
            let volume = row[5].as_f64().unwrap_or(0.0) as i64;

            items.push(CandlePoint {
                trade_date: date,
                open,
                close,
                high,
                low,
                volume,
                amount: 0.0,
                amplitude_pct: amplitude_pct(high, low),
                change_pct: 0.0,
                change_amount: 0.0,
                turnover_pct: 0.0,
            });
        }
        apply_change_metrics(&mut items);

        if items.len() > limit {
            items = items[items.len() - limit..].to_vec();
        }
        if items.is_empty() {
            return Err(Error::upstream("sina futures: no data returned"));
        }
        Ok(items)
    }

    /// Futures symbol-to-mark mapping from Sina.
    ///
    /// Returns a list of (exchange, symbol, mark) triples for all Chinese futures
    /// symbols available on Sina Finance.
    pub async fn futures_symbol_mark(&self) -> Result<Vec<Row>> {
        let url = "https://vip.stock.finance.sina.com.cn/quotes_service/view/js/qihuohangqing.js";
        let body = self
            .get(url)
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .send()
            .await?
            .text()
            .await?;

        // The JS file has encoding issues with gb2312, but we can still parse the JSON
        // Extract the JSON object from the JS variable assignment
        let json_str = body
            .find('{')
            .and_then(|start| {
                body[start..]
                    .find('}')
                    .map(|end| &body[start..=start + end])
            })
            .ok_or_else(|| Error::decode("sina symbol mark: invalid JS response"))?;

        let data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|_| Error::decode("sina symbol mark: JSON parse error"))?;

        let mut items = Vec::new();
        for exchange in &["czce", "dce", "shfe", "cffex", "gfex"] {
            let Some(arr) = data[exchange].as_array() else {
                continue;
            };
            if arr.len() < 2 {
                continue;
            }
            // First element is the exchange name, rest are [symbol, mark] pairs
            for entry in arr.iter().skip(1) {
                let Some(arr_entry) = entry.as_array() else {
                    continue;
                };
                if arr_entry.len() < 2 {
                    continue;
                }
                let mut r = Row::new();
                r.insert("exchange".into(), serde_json::json!(exchange));
                r.insert("symbol".into(), arr_entry[0].clone());
                r.insert("mark".into(), arr_entry[1].clone());
                items.push(r);
            }
        }
        Ok(items)
    }

    /// Realtime quotes for all contracts of a given futures variety.
    ///
    /// `symbol` is the variety name like "PTA", "RB", "CU".
    /// Returns realtime data for all tradeable contracts of that variety.
    pub async fn futures_zh_realtime(&self, symbol: &str) -> Result<Vec<Row>> {
        // First get the symbol mark mapping
        let marks = self.futures_symbol_mark().await?;
        let mark = marks
            .iter()
            .find(|r| r.get("symbol").and_then(|v| v.as_str()) == Some(symbol))
            .and_then(|r| {
                r.get("mark")
                    .and_then(|v| v.as_str().map(std::string::ToString::to_string))
            })
            .ok_or_else(|| Error::not_found(format!("sina: unknown futures symbol: {symbol}")))?;

        let url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQFuturesData";
        let body = self
            .get(url)
            .query(&[
                ("page", "1"),
                ("sort", "position"),
                ("asc", "0"),
                ("node", mark.as_str()),
                ("base", "futures"),
            ])
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .send()
            .await?
            .text()
            .await?;

        let data: Vec<serde_json::Value> = serde_json::from_str(&body)
            .map_err(|_| Error::decode("sina realtime: JSON parse error"))?;

        let mut items = Vec::new();
        for entry in &data {
            let mut r = Row::new();
            let empty = serde_json::Map::new();
            for (key, val) in entry.as_object().unwrap_or(&empty) {
                r.insert(key.clone(), val.clone());
            }
            items.push(r);
        }
        Ok(items)
    }

    /// Realtime spot quotes for specific futures contracts from Sina hq.sinajs.cn.
    ///
    /// `symbols` is a comma-separated list of contract codes like "V2309,V2401".
    /// `market` is "CF" for commodity futures, "FF" for financial futures.
    pub async fn futures_zh_spot(&self, symbols: &str, market: &str) -> Result<Vec<Row>> {
        let subscribe_list: Vec<String> = symbols
            .split(',')
            .map(|s| format!("nf_{}", s.trim()))
            .collect();
        let list_str = subscribe_list.join(",");
        let rn = format!("{:x}", chrono::Utc::now().timestamp_millis());

        let url = format!("https://hq.sinajs.cn/rn={rn}&list={list_str}");
        let body = self
            .get(&url)
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        for line in body.split(';') {
            let line = line.trim();
            if line.is_empty() || !line.contains('=') {
                continue;
            }
            // Extract symbol from var hq_str_nf_XXX=
            let symbol_part = line.split('=').next().unwrap_or("");
            let symbol_name = symbol_part.split("hq_str_").last().unwrap_or("");
            let nf_symbol = symbol_name.strip_prefix("nf_").unwrap_or(symbol_name);

            // Extract the comma-separated values between quotes
            let value_part = line.split('=').nth(1).unwrap_or("");
            let values: Vec<&str> = value_part.trim_matches('"').split(',').collect();

            if values.len() < 15 {
                continue;
            }

            let mut r = Row::new();
            r.insert("symbol".into(), serde_json::json!(nf_symbol));

            if market == "CF" {
                // Commodity futures format
                r.insert("time".into(), serde_json::json!(values[0]));
                r.insert("open".into(), serde_json::json!(values[1]));
                r.insert("high".into(), serde_json::json!(values[2]));
                r.insert("low".into(), serde_json::json!(values[3]));
                r.insert("last_close".into(), serde_json::json!(values[4]));
                r.insert("bid_price".into(), serde_json::json!(values[5]));
                r.insert("ask_price".into(), serde_json::json!(values[6]));
                r.insert("current_price".into(), serde_json::json!(values[7]));
                r.insert("avg_price".into(), serde_json::json!(values[8]));
                r.insert("last_settle_price".into(), serde_json::json!(values[9]));
                r.insert("buy_vol".into(), serde_json::json!(values[10]));
                r.insert("sell_vol".into(), serde_json::json!(values[11]));
                r.insert("hold".into(), serde_json::json!(values[12]));
                r.insert("volume".into(), serde_json::json!(values[13]));
            } else {
                // Financial futures format (FF) - different field positions
                r.insert("open".into(), serde_json::json!(values[0]));
                r.insert("high".into(), serde_json::json!(values[1]));
                r.insert("low".into(), serde_json::json!(values[2]));
                r.insert("current_price".into(), serde_json::json!(values[3]));
                r.insert("volume".into(), serde_json::json!(values[4]));
                r.insert("amount".into(), serde_json::json!(values[5]));
                r.insert("hold".into(), serde_json::json!(values[6]));
                // time is at a different position for financial futures
                if values.len() > 32 {
                    r.insert("time".into(), serde_json::json!(values[32]));
                }
            }
            items.push(r);
        }
        Ok(items)
    }

    /// Minute-frequency kline data for a futures contract from Sina.
    ///
    /// `symbol`: contract code like "IF2008", "RB0"
    /// `period`: "1", "5", "15", "30", "60" for 1/5/15/30/60 minute bars
    pub async fn futures_zh_minute(&self, symbol: &str, period: &str) -> Result<Vec<Row>> {
        let url = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/=/InnerFuturesNewService.getFewMinLine";
        let body = self
            .get(url)
            .query(&[("symbol", symbol), ("type", period)])
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        // Extract JSON from JSONP callback: =([...]);
        let json_str = body
            .find("=(")
            .and_then(|start| {
                body[start..]
                    .rfind(");")
                    .map(|end| &body[start + 2..start + end])
            })
            .ok_or_else(|| Error::decode("sina minute kline: invalid JSONP response"))?;

        let data: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|_| Error::decode("sina minute kline: JSON parse error"))?;

        let mut items = Vec::new();
        for row in &data {
            if row.len() < 7 {
                continue;
            }
            let mut r = Row::new();
            r.insert("datetime".into(), row[0].clone());
            r.insert("open".into(), row[1].clone());
            r.insert("high".into(), row[2].clone());
            r.insert("low".into(), row[3].clone());
            r.insert("close".into(), row[4].clone());
            r.insert("volume".into(), row[5].clone());
            r.insert("hold".into(), row[6].clone());
            items.push(r);
        }
        Ok(items)
    }

    /// Daily kline data for a specific futures contract from Sina.
    ///
    /// `symbol`: contract code like "RB0", "V2105"
    pub async fn futures_zh_daily(&self, symbol: &str) -> Result<Vec<Row>> {
        let date = chrono::Utc::now().format("%Y%m%d").to_string();
        let date_formatted = format!("{}_{}_{}", &date[..4], &date[4..6], &date[6..8]);
        let url = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_dummy=/InnerFuturesNewService.getDailyKLine";
        let body = self
            .get(url)
            .query(&[("symbol", symbol), ("type", &date_formatted)])
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        // Extract JSON from JSONP callback
        let json_str = body
            .find("=(")
            .and_then(|start| {
                body[start..]
                    .rfind(");")
                    .map(|end| &body[start + 2..start + end])
            })
            .ok_or_else(|| Error::decode("sina daily kline: invalid JSONP response"))?;

        let data: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|_| Error::decode("sina daily kline: JSON parse error"))?;

        let mut items = Vec::new();
        for row in &data {
            if row.len() < 8 {
                continue;
            }
            let mut r = Row::new();
            r.insert("date".into(), row[0].clone());
            r.insert("open".into(), row[1].clone());
            r.insert("high".into(), row[2].clone());
            r.insert("low".into(), row[3].clone());
            r.insert("close".into(), row[4].clone());
            r.insert("volume".into(), row[5].clone());
            r.insert("hold".into(), row[6].clone());
            r.insert("settle".into(), row[7].clone());
            items.push(r);
        }
        Ok(items)
    }
}
