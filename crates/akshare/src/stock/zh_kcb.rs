//! KCB (STAR Market) stock data — spot, daily, reports.
//!
//! Covers Python functions:
//! - `stock_zh_kcb_spot` — KCB spot from Sina
//! - `stock_zh_kcb_daily` — KCB daily from Sina (via Eastmoney kline)
//! - `stock_zh_kcb_report_em` — KCB reports from Eastmoney

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::market::eastmoney_secid;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// KCB spot quote from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KcbSpotQuote {
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
    #[serde(default)]
    pub pe_ratio: Option<f64>,
    #[serde(default)]
    pub pb_ratio: Option<f64>,
    #[serde(default)]
    pub circulating_market_cap: Option<f64>,
    #[serde(default)]
    pub total_market_cap: Option<f64>,
    #[serde(default)]
    pub turnover_rate: Option<f64>,
}

/// KCB daily candle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KcbDailyCandle {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    #[serde(default)]
    pub amount: Option<f64>,
}

/// KCB report entry from Eastmoney.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KcbReport {
    pub code: String,
    pub name: String,
    pub title: String,
    #[serde(default)]
    pub report_type: Option<String>,
    pub date: String,
    #[serde(default)]
    pub art_code: Option<String>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get KCB spot data from Sina.
    ///
    /// Python equivalent: `stock_zh_kcb_spot()`
    pub async fn stock_zh_kcb_spot(&self) -> Result<Vec<KcbSpotQuote>> {
        let count_url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeStockCount";
        let count_resp = self
            .get(count_url)
            .query(&[("node", "kcb")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let count_text = count_resp.text().await.map_err(Error::from)?;
        let total: i64 = count_text
            .chars()
            .filter(|c| c.is_ascii_digit())
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
                    ("node", "kcb"),
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

                all_quotes.push(KcbSpotQuote {
                    symbol,
                    name,
                    latest_price: parse_kcb_f64(item, "trade"),
                    change_amount: parse_kcb_f64(item, "pricechange"),
                    change_pct: parse_kcb_f64(item, "changepercent"),
                    buy_price: parse_kcb_f64(item, "buy"),
                    sell_price: parse_kcb_f64(item, "sell"),
                    prev_close: parse_kcb_f64(item, "settlement"),
                    open: parse_kcb_f64(item, "open"),
                    high: parse_kcb_f64(item, "high"),
                    low: parse_kcb_f64(item, "low"),
                    volume: parse_kcb_f64(item, "volume"),
                    amount: parse_kcb_f64(item, "amount"),
                    pe_ratio: parse_kcb_f64(item, "per"),
                    pb_ratio: parse_kcb_f64(item, "pb"),
                    circulating_market_cap: parse_kcb_f64(item, "nmc"),
                    total_market_cap: parse_kcb_f64(item, "mktcap"),
                    turnover_rate: parse_kcb_f64(item, "turnoverratio"),
                });
            }
        }

        if all_quotes.is_empty() {
            return Err(Error::not_found("sina returned no KCB spot data"));
        }
        Ok(all_quotes)
    }

    /// Get KCB daily candles. Uses Eastmoney kline API.
    ///
    /// Python equivalent: `stock_zh_kcb_daily(symbol, start_date, end_date, adjust)`
    ///
    /// - `symbol`: stock code like "sh688399" or "688399"
    /// - `start_date`: "20200101"
    /// - `end_date`: "20201231"
    /// - `adjust`: "", "qfq", "hfq"
    pub async fn stock_zh_kcb_daily(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<KcbDailyCandle>> {
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
            .ok_or_else(|| Error::upstream("KCB daily kline missing data"))?;

        let items: Vec<KcbDailyCandle> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 7 {
                    return None;
                }
                Some(KcbDailyCandle {
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
            return Err(Error::not_found("KCB daily returned no data"));
        }
        Ok(items)
    }

    /// Get KCB reports from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_kcb_report_em(from_page, to_page)`
    ///
    /// Returns KCB announcement reports.
    pub async fn stock_zh_kcb_report_em(
        &self,
        from_page: i32,
        to_page: i32,
    ) -> Result<Vec<KcbReport>> {
        let url = "https://np-anotice-stock.eastmoney.com/api/security/ann";
        let mut all_reports = Vec::new();

        for page in from_page..=to_page.min(from_page + 10) {
            let page_str = page.to_string();
            let response = self
                .get(url)
                .query(&[
                    ("sr", "-1"),
                    ("page_size", "100"),
                    ("page_index", page_str.as_str()),
                    ("ann_type", "KCB"),
                    ("client_source", "web"),
                    ("f_node", "0"),
                    ("s_node", "0"),
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
                list: Option<Vec<AnnItem>>,
            }
            #[derive(Deserialize)]
            struct AnnItem {
                codes: Option<Vec<CodeItem>>,
                title: Option<String>,
                columns: Option<Vec<ColumnItem>>,
                notice_date: Option<String>,
                art_code: Option<String>,
            }
            #[derive(Deserialize)]
            struct CodeItem {
                stock_code: Option<String>,
                short_name: Option<String>,
            }
            #[derive(Deserialize)]
            struct ColumnItem {
                column_name: Option<String>,
            }

            let payload: Env = response.json().await.map_err(Error::from)?;
            let list = payload.data.and_then(|d| d.list).unwrap_or_default();

            for item in &list {
                let code = item
                    .codes
                    .as_ref()
                    .and_then(|c| c.first())
                    .and_then(|c| c.stock_code.as_deref())
                    .unwrap_or("")
                    .to_string();
                let name = item
                    .codes
                    .as_ref()
                    .and_then(|c| c.first())
                    .and_then(|c| c.short_name.as_deref())
                    .unwrap_or("")
                    .to_string();
                let title = item.title.as_deref().unwrap_or("").to_string();
                let report_type = item
                    .columns
                    .as_ref()
                    .and_then(|c| c.first())
                    .and_then(|c| c.column_name.as_deref())
                    .map(|s| s.to_string());
                let date = item.notice_date.as_deref().unwrap_or("").to_string();
                let art_code = item.art_code.as_deref().map(|s| s.to_string());

                all_reports.push(KcbReport {
                    code,
                    name,
                    title,
                    report_type,
                    date,
                    art_code,
                });
            }
        }

        if all_reports.is_empty() {
            return Err(Error::not_found("KCB reports returned no data"));
        }
        Ok(all_reports)
    }
}

fn parse_kcb_f64(item: &serde_json::Value, key: &str) -> Option<f64> {
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
