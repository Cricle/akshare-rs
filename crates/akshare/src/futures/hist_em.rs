#![allow(dead_code)]
//! Futures historical data from Eastmoney (东方财富网).
//!
//! Provides kline data for Chinese and international futures.

use std::sync::LazyLock;

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::{FuturesHistKline, Row};

static RE_ALPHA: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-zA-Z]+").unwrap());

#[derive(Debug, Deserialize)]
struct EmRedisEntry {
    name: Option<String>,
    code: Option<String>,
    vcode: Option<String>,
    vname: Option<String>,
    mktid: Option<i64>,
    mktname: Option<String>,
}

fn parse_f64(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap_or(0.0)
}

impl AkShareClient {
    /// Eastmoney futures market table (品种对照表).
    ///
    /// Returns contract names and codes for all exchanges.
    pub async fn futures_hist_table(&self) -> Result<Vec<Row>> {
        let url = "https://futsse-static.eastmoney.com/redis";
        let body = self
            .get(url)
            .query(&[("msgid", "gnweb")])
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let items_arr = data.as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for item in &items_arr {
            let mkt_id = item["mktid"].as_i64().unwrap_or(0);
            // Fetch details per market
            let detail_url = "https://futsse-static.eastmoney.com/redis";
            let detail_body = match self
                .get(detail_url)
                .query(&[("msgid", &mkt_id.to_string())])
                .send()
                .await
            {
                Ok(resp) => resp.text().await.unwrap_or_default(),
                Err(_) => String::new(),
            };
            if detail_body.is_empty() {
                continue;
            }
            if let Ok(detail_data) = serde_json::from_str::<serde_json::Value>(&detail_body)
                && let Some(arr) = detail_data.as_array()
            {
                for entry in arr {
                    let mut row = Row::new();
                    row.insert("market_name".into(), entry["mktname"].clone());
                    row.insert("contract_name".into(), entry["name"].clone());
                    row.insert("contract_code".into(), entry["code"].clone());
                    row.insert("variety_code".into(), entry["vcode"].clone());
                    row.insert("variety_name".into(), entry["vname"].clone());
                    items.push(row);
                }
            }
        }
        Ok(items)
    }

    /// Eastmoney futures historical kline data.
    ///
    /// `symbol`: Chinese name like "热卷主连" or code like "rb2505"
    /// `period`: "daily", "weekly", or "monthly"
    pub async fn futures_hist(
        &self,
        symbol: &str,
        period: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<FuturesHistKline>> {
        let period_code = match period {
            "weekly" => "102",
            "monthly" => "103",
            _ => "101",
        };

        // Try to determine sec_id from symbol
        // For Chinese names, we need the market code; for codes, try direct
        let sec_id = if symbol.chars().any(|c| c > '\u{4e00}') {
            // Chinese symbol - need to look up market code
            format!("113.{symbol}") // default to SHFE
        } else {
            // English code
            let variety = RE_ALPHA.find(symbol).map_or("", |m| m.as_str());
            let market = match variety.to_uppercase().as_str() {
                "IF" | "IC" | "IH" | "IM" | "T" | "TF" | "TS" => "8",
                "m" | "y" | "a" | "b" | "c" | "cs" | "jd" | "pp" | "v" | "l" | "eg" | "eb"
                | "pg" | "rr" | "lh" | "fb" | "bb" => "114",
                "CF" | "SR" | "TA" | "OI" | "MA" | "FG" | "RM" | "RS" | "ZC" | "SF" | "SM"
                | "AP" | "CJ" | "UR" | "SA" | "PF" | "PK" | "SH" | "PX" => "115",
                "si" | "lc" => "225",
                _ => "113",
            };
            format!("{market}.{symbol}")
        };

        let klines = self
            .kline_fetch(&sec_id, period_code, "1", 10000, &[("iscca", "1")])
            .await
            .unwrap_or_default();

        let mut items = Vec::new();
        for line in &klines {
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 14 {
                continue;
            }
            let date = fields[0];
            if !start_date.is_empty() && date < start_date {
                continue;
            }
            if !end_date.is_empty() && date > end_date {
                continue;
            }
            items.push(FuturesHistKline {
                date: date.to_string(),
                open: parse_f64(fields[1]),
                close: parse_f64(fields[2]),
                high: parse_f64(fields[3]),
                low: parse_f64(fields[4]),
                volume: parse_f64(fields[5]),
                amount: parse_f64(fields[6]),
                change_amount: parse_f64(fields[9]),
                change_pct: parse_f64(fields[8]),
                open_interest: parse_f64(fields[12]),
            });
        }
        Ok(items)
    }
}
