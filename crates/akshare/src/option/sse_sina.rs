#![allow(dead_code)]
//! SSE stock options from Sina Finance.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::parse_f64_safe;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct SinaStockNameResponse {
    result: Option<SinaStockNameResult>,
}

#[derive(Debug, Deserialize)]
struct SinaStockNameResult {
    data: Option<SinaStockNameData>,
}

#[derive(Debug, Deserialize)]
struct SinaStockNameData {
    #[serde(default)]
    contract_month: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SinaRemainderResponse {
    result: Option<SinaRemainderResult>,
}

#[derive(Debug, Deserialize)]
struct SinaRemainderResult {
    data: Option<SinaRemainderData>,
}

#[derive(Debug, Deserialize)]
struct SinaRemainderData {
    #[serde(default)]
    expire_day: String,
    #[serde(default)]
    remainder_days: String,
}

#[derive(Debug, Deserialize)]
struct SinaMinuteResponse {
    result: Option<SinaMinuteResult>,
}

#[derive(Debug, Deserialize)]
struct SinaMinuteResult {
    data: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct SinaDailyJsonpResponse {
    // The response is a JSONP that wraps a JSON array
}

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// Entry in the SSE option codes list.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionSseCodeEntry {
    /// Sequential number (1-based).
    pub index: usize,
    /// Option contract code.
    pub code: String,
}

/// A field-value pair for SSE option spot data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionFieldValuePair {
    /// Field name.
    pub field: String,
    /// Value as string.
    pub value: String,
}

/// A single row of SSE option minute data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionSseMinuteRow {
    /// Date.
    pub date: String,
    /// Time.
    pub time: String,
    /// Price.
    pub price: f64,
    /// Volume.
    pub volume: f64,
    /// Open interest.
    pub open_interest: f64,
    /// Average price.
    pub average_price: f64,
}

/// A single row of SSE option daily kline data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionSseDailyRow {
    /// Date.
    pub date: String,
    /// Open.
    pub open: f64,
    /// High.
    pub high: f64,
    /// Low.
    pub low: f64,
    /// Close.
    pub close: f64,
    /// Volume.
    pub volume: f64,
}

/// A single row of option finance minute data (5-day).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionFinanceMinuteRow {
    /// Date.
    pub date: String,
    /// Time.
    pub time: String,
    /// Price.
    pub price: f64,
    /// Average price.
    pub average_price: f64,
    /// Volume.
    pub volume: f64,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

const SINA_REFERER: &str = "https://stock.finance.sina.com.cn/";

impl AkShareClient {
    /// SSE option expiry month list from Sina.
    ///
    /// `symbol` is "50ETF" or "300ETF".
    pub async fn option_sse_list_sina(&self, symbol: &str, exchange: &str) -> Result<Vec<String>> {
        let url = "https://stock.finance.sina.com.cn/futures/api/openapi.php/StockOptionService.getStockName";
        let resp: SinaStockNameResponse = self
            .get(url)
            .query(&[("exchange", exchange), ("cate", symbol)])
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let months = resp
            .result
            .and_then(|r| r.data)
            .map(|d| d.contract_month)
            .unwrap_or_default();

        // Remove dashes and skip the first entry
        Ok(months
            .into_iter()
            .skip(1)
            .map(|m| m.replace('-', ""))
            .collect())
    }

    /// SSE option expiry day and remaining days from Sina.
    ///
    /// Returns `(expiry_date_string, remaining_days)`.
    pub async fn option_sse_expire_day_sina(
        &self,
        trade_date: &str,
        symbol: &str,
        exchange: &str,
    ) -> Result<(String, i64)> {
        let url = "https://stock.finance.sina.com.cn/futures/api/openapi.php/StockOptionService.getRemainderDay";
        let date_param = if trade_date.len() >= 6 {
            format!("{}-{}", &trade_date[..4], &trade_date[4..])
        } else {
            trade_date.to_string()
        };

        let resp: SinaRemainderResponse = self
            .get(url)
            .query(&[
                ("exchange", exchange),
                ("cate", symbol),
                ("date", date_param.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let data = resp
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("sina expire day: missing data"))?;

        let remaining = data.remainder_days.parse::<i64>().unwrap_or(0);

        // If negative, try with "XD" prefix
        if remaining < 0 {
            let xd_symbol = format!("XD{symbol}");
            let resp2: SinaRemainderResponse = self
                .get(url)
                .query(&[
                    ("exchange", exchange),
                    ("cate", xd_symbol.as_str()),
                    ("date", date_param.as_str()),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .json()
                .await
                .map_err(Error::from)?;

            if let Some(data2) = resp2.result.and_then(|r| r.data) {
                let remaining2 = data2.remainder_days.parse::<i64>().unwrap_or(0);
                return Ok((data2.expire_day, remaining2));
            }
        }

        Ok((data.expire_day, remaining))
    }

    /// SSE option codes (call or put) from Sina.
    ///
    /// `symbol` is "看涨期权" (call) or "看跌期权" (put).
    /// `trade_date` is the expiry month, e.g. "202202".
    /// `underlying` is the underlying code, e.g. "510050" or "510300".
    pub async fn option_sse_codes_sina(
        &self,
        symbol: &str,
        trade_date: &str,
        underlying: &str,
    ) -> Result<Vec<OptionSseCodeEntry>> {
        let suffix = &trade_date[trade_date.len().saturating_sub(4)..];
        let list_prefix = if symbol.contains("涨") {
            "OP_UP_"
        } else {
            "OP_DOWN_"
        };
        let url = format!("https://hq.sinajs.cn/list={list_prefix}{underlying}{suffix}");

        let body = self
            .get(&url)
            .header("Referer", SINA_REFERER)
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Parse: var hq_str_OP_UP_51005003="CON_OP_10003720,CON_OP_...";
        let data = body
            .split_once('"')
            .and_then(|(_, r)| r.split_once('"'))
            .map_or("", |(s, _)| s);

        let codes: Vec<OptionSseCodeEntry> = data
            .split(',')
            .filter_map(|item| {
                let trimmed = item.trim();
                let code = trimmed.strip_prefix("CON_OP_").or(Some(trimmed))?;
                if code.is_empty() || code == trimmed && !trimmed.starts_with("CON_OP_") {
                    return None;
                }
                Some(code.to_string())
            })
            .enumerate()
            .map(|(i, code)| OptionSseCodeEntry { index: i + 1, code })
            .collect();

        if codes.is_empty() {
            return Err(Error::not_found(format!(
                "no SSE option codes for {symbol} {trade_date} {underlying}"
            )));
        }

        Ok(codes)
    }

    /// SSE option spot price from Sina.
    ///
    /// Returns field-value pairs with the option's real-time data.
    pub async fn option_sse_spot_price_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<OptionFieldValuePair>> {
        let url = format!("https://hq.sinajs.cn/list=CON_OP_{symbol}");
        let body = self
            .get(&url)
            .header("Referer", SINA_REFERER)
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        let data = body
            .split_once('"')
            .and_then(|(_, r)| r.split_once('"'))
            .map_or("", |(s, _)| s);

        let values: Vec<&str> = data.split(',').collect();
        let field_list = [
            "买量",
            "买价",
            "最新价",
            "卖价",
            "卖量",
            "持仓量",
            "涨幅",
            "行权价",
            "昨收价",
            "开盘价",
            "涨停价",
            "跌停价",
            "申卖价五",
            "申卖量五",
            "申卖价四",
            "申卖量四",
            "申卖价三",
            "申卖量三",
            "申卖价二",
            "申卖量二",
            "申卖价一",
            "申卖量一",
            "申买价一",
            "申买量一",
            "申买价二",
            "申买量二",
            "申买价三",
            "申买量三",
            "申买价四",
            "申买量四",
            "申买价五",
            "申买量五",
            "行情时间",
            "主力合约标识",
            "状态码",
            "标的证券类型",
            "标的股票",
            "期权合约简称",
            "振幅",
            "最高价",
            "最低价",
            "成交量",
            "成交额",
        ];

        let pairs: Vec<OptionFieldValuePair> = field_list
            .iter()
            .zip(values.iter())
            .map(|(field, value)| OptionFieldValuePair {
                field: field.to_string(),
                value: value.trim().to_string(),
            })
            .collect();

        if pairs.is_empty() {
            return Err(Error::not_found(format!(
                "no SSE option spot data for {symbol}"
            )));
        }

        Ok(pairs)
    }

    /// SSE option underlying spot price from Sina.
    ///
    /// `symbol` is e.g. "sh510050" or "sh510300".
    pub async fn option_sse_underlying_spot_price_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<OptionFieldValuePair>> {
        let url = format!("https://hq.sinajs.cn/list={symbol}");
        let body = self
            .get(&url)
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        let data = body
            .split_once('"')
            .and_then(|(_, r)| r.split_once('"'))
            .map_or("", |(s, _)| s);

        let values: Vec<&str> = data.split(',').collect();
        let field_list = [
            "证券简称",
            "今日开盘价",
            "昨日收盘价",
            "最近成交价",
            "最高成交价",
            "最低成交价",
            "买入价",
            "卖出价",
            "成交数量",
            "成交金额",
            "买数量一",
            "买价位一",
            "买数量二",
            "买价位二",
            "买数量三",
            "买价位三",
            "买数量四",
            "买价位四",
            "买数量五",
            "买价位五",
            "卖数量一",
            "卖价位一",
            "卖数量二",
            "卖价位二",
            "卖数量三",
            "卖价位三",
            "卖数量四",
            "卖价位四",
            "卖数量五",
            "卖价位五",
            "行情日期",
            "行情时间",
            "停牌状态",
        ];

        let pairs: Vec<OptionFieldValuePair> = field_list
            .iter()
            .zip(values.iter())
            .map(|(field, value)| OptionFieldValuePair {
                field: field.to_string(),
                value: value.trim().to_string(),
            })
            .collect();

        if pairs.is_empty() {
            return Err(Error::not_found(format!(
                "no underlying spot data for {symbol}"
            )));
        }

        Ok(pairs)
    }

    /// SSE option greeks from Sina.
    ///
    /// Returns field-value pairs with Delta, Gamma, Theta, Vega, implied volatility, etc.
    pub async fn option_sse_greeks_sina(&self, symbol: &str) -> Result<Vec<OptionFieldValuePair>> {
        let url = format!("https://hq.sinajs.cn/list=CON_SO_{symbol}");
        let body = self
            .get(&url)
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        let data = body
            .split_once('"')
            .and_then(|(_, r)| r.split_once('"'))
            .map_or("", |(s, _)| s);

        let values: Vec<&str> = data.split(',').collect();
        let field_list = [
            "期权合约简称",
            "成交量",
            "Delta",
            "Gamma",
            "Theta",
            "Vega",
            "隐含波动率",
            "最高价",
            "最低价",
            "交易代码",
            "行权价",
            "最新价",
            "理论价值",
        ];

        // Python code skips indices 1-3: [data_list[0]] + data_list[4:]
        let mut filtered_values = Vec::new();
        if let Some(first) = values.first() {
            filtered_values.push(*first);
        }
        for v in values.iter().skip(4) {
            filtered_values.push(*v);
        }

        let pairs: Vec<OptionFieldValuePair> = field_list
            .iter()
            .zip(filtered_values.iter())
            .map(|(field, value)| OptionFieldValuePair {
                field: field.to_string(),
                value: value.trim().to_string(),
            })
            .collect();

        if pairs.is_empty() {
            return Err(Error::not_found(format!("no greeks data for {symbol}")));
        }

        Ok(pairs)
    }

    /// SSE option minute data (current trading day) from Sina.
    pub async fn option_sse_minute_sina(&self, symbol: &str) -> Result<Vec<OptionSseMinuteRow>> {
        let url = "https://stock.finance.sina.com.cn/futures/api/openapi.php/StockOptionDaylineService.getOptionMinline";
        let con_symbol = format!("CON_OP_{symbol}");

        let resp: SinaMinuteResponse = self
            .get(url)
            .query(&[("symbol", con_symbol.as_str())])
            .header(
                "Referer",
                "https://stock.finance.sina.com.cn/option/quotes.html",
            )
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let data = resp
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("sina minute: missing data"))?;

        let mut rows = Vec::with_capacity(data.len());
        for item in &data {
            if let Some(arr) = item.as_array() {
                if arr.len() < 6 {
                    continue;
                }
                let date = arr[5].as_str().unwrap_or("").to_string();
                let time = arr[0].as_str().unwrap_or("").to_string();
                let price = json_to_f64(&arr[1]);
                let volume = json_to_f64(&arr[2]);
                let open_interest = json_to_f64(&arr[3]);
                let average_price = json_to_f64(&arr[4]);

                rows.push(OptionSseMinuteRow {
                    date,
                    time,
                    price,
                    volume,
                    open_interest,
                    average_price,
                });
            }
        }

        if rows.is_empty() {
            return Err(Error::not_found(format!("no minute data for {symbol}")));
        }

        // Forward-fill dates
        for i in 1..rows.len() {
            if rows[i].date.is_empty() {
                rows[i].date = rows[i - 1].date.clone();
            }
        }

        Ok(rows)
    }

    /// SSE option daily kline data from Sina.
    pub async fn option_sse_daily_sina(&self, symbol: &str) -> Result<Vec<OptionSseDailyRow>> {
        let url = "https://stock.finance.sina.com.cn/futures/api/jsonp_v2.php//StockOptionDaylineService.getSymbolInfo";
        let con_symbol = format!("CON_OP_{symbol}");

        let body = self
            .get(url)
            .query(&[("symbol", con_symbol.as_str())])
            .header(
                "Referer",
                "https://stock.finance.sina.com.cn/option/quotes.html",
            )
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Extract JSON array from JSONP: ...([...])
        let json_start = body.find('[').unwrap_or(0);
        let json_end = body.rfind(']').map_or(body.len(), |i| i + 1);
        if json_start >= json_end {
            return Err(Error::decode("sina daily: no JSON array found"));
        }
        let json_str = &body[json_start..json_end];

        // Format: [[date, open, high, low, close, volume], ...]
        let raw: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("sina daily json: {e}")))?;

        let rows: Vec<OptionSseDailyRow> = raw
            .iter()
            .filter(|item| item.len() >= 6)
            .map(|item| OptionSseDailyRow {
                date: item[0].as_str().unwrap_or("").to_string(),
                open: json_to_f64(&item[1]),
                high: json_to_f64(&item[2]),
                low: json_to_f64(&item[3]),
                close: json_to_f64(&item[4]),
                volume: json_to_f64(&item[5]),
            })
            .collect();

        if rows.is_empty() {
            return Err(Error::not_found(format!("no daily data for {symbol}")));
        }

        Ok(rows)
    }

    /// Option finance 5-day minute data from Sina.
    pub async fn option_finance_minute_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<OptionFinanceMinuteRow>> {
        let url = "https://stock.finance.sina.com.cn/futures/api/openapi.php/StockOptionDaylineService.getFiveDayLine";
        let con_symbol = format!("CON_OP_{symbol}");

        let body = self
            .get(url)
            .query(&[("symbol", con_symbol.as_str())])
            .header(
                "Referer",
                "https://stock.finance.sina.com.cn/option/quotes.html",
            )
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        let resp: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| Error::decode(format!("sina finance minute json: {e}")))?;

        let data = resp
            .get("result")
            .and_then(|r| r.get("data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut all_rows = Vec::new();
        for day_data in &data {
            if let Some(arr) = day_data.as_array() {
                let mut last_date = String::new();
                for item in arr {
                    if let Some(row) = item.as_array() {
                        if row.len() < 6 {
                            continue;
                        }
                        let time = row[0].as_str().unwrap_or("").to_string();
                        let price = json_to_f64(&row[1]);
                        let volume = json_to_f64(&row[2]);
                        let average_price = json_to_f64(&row[4]);
                        let date_raw = row[5].as_str().unwrap_or("").to_string();
                        if !date_raw.is_empty() {
                            last_date = date_raw;
                        }

                        all_rows.push(OptionFinanceMinuteRow {
                            date: last_date.clone(),
                            time,
                            price,
                            average_price,
                            volume,
                        });
                    }
                }
            }
        }

        if all_rows.is_empty() {
            return Err(Error::not_found(format!(
                "no finance minute data for {symbol}"
            )));
        }

        Ok(all_rows)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn json_to_f64(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => parse_f64_safe(s),
        _ => 0.0,
    }
}
