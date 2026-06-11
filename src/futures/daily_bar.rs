#![allow(dead_code)]
//! Futures daily bar data from Chinese exchanges (CFFEX, CZCE, SHFE, DCE, INE, GFEX).

use std::sync::LazyLock;

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FuturesDailyBar;

static RE_ALPHA: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-zA-Z_]+").unwrap());
static RE_ALPHA_ONLY: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-zA-Z]+").unwrap());

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ShfeResponse {
    #[serde(rename = "o_curinstrument")]
    instruments: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct DceResponse {
    data: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct GfexResponse {
    data: Option<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_variety(symbol: &str) -> String {
    RE_ALPHA.find(symbol).map(|m| m.as_str().to_uppercase()).unwrap_or_default()
}

fn parse_f64_val(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.replace(',', "").parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

fn parse_f64_str(s: &str) -> f64 {
    s.replace(',', "").trim().parse::<f64>().unwrap_or(0.0)
}

fn parse_pipe_table(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.split('|').map(|c| c.trim().to_string()).collect())
        .collect()
}

// ---------------------------------------------------------------------------
// CFFEX
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// CFFEX daily bar data.
    ///
    /// Fetches daily OHLCV + settlement data from CFFEX CSV files.
    /// Data starts from 20100416.
    pub async fn futures_daily_cffex(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        let url = format!(
            "http://www.cffex.com.cn/sj/hqsj/rtj/{}/{}/{}_1.csv",
            &date[..6],
            &date[6..],
            date
        );
        let body = self
                        .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        if body.trim().is_empty() || body.contains("html") {
            return Ok(vec![]);
        }

        let rows = parse_pipe_table(&body);
        if rows.len() < 3 {
            return Ok(vec![]);
        }

        let mut items = Vec::new();
        for row in &rows[2..] {
            if row.len() < 11 {
                continue;
            }
            let sym = row[0].trim().to_string();
            if sym.contains("小计") || sym.contains("合计") || sym.is_empty() {
                continue;
            }
            if sym.starts_with("IO") || sym.starts_with("MO") || sym.starts_with("HO") {
                continue;
            }
            let variety = extract_variety(&sym);
            items.push(FuturesDailyBar {
                symbol: sym,
                date: date.to_string(),
                open: parse_f64_str(&row[1]),
                high: parse_f64_str(&row[2]),
                low: parse_f64_str(&row[3]),
                close: parse_f64_str(row.get(8).unwrap_or(&String::new())),
                volume: parse_f64_str(&row[4]),
                open_interest: parse_f64_str(&row[6]),
                turnover: parse_f64_str(&row[5]),
                settle: parse_f64_str(row.get(9).unwrap_or(&String::new())),
                pre_settle: parse_f64_str(row.get(10).unwrap_or(&String::new())),
                variety,
            });
        }
        Ok(items)
    }

    /// SHFE daily bar data.
    ///
    /// Fetches daily OHLCV + settlement data from SHFE JSON API.
    pub async fn futures_daily_shfe(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        let url = format!(
            "https://www.shfe.com.cn/data/tradedata/future/dailydata/kx{}.dat",
            date
        );
        let body = self
                        .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let instruments = data["o_curinstrument"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for row in &instruments {
            let delivery_month = row["DELIVERYMONTH"].as_str().unwrap_or("").to_string();
            if delivery_month.contains("小计") || delivery_month.contains("合计") || delivery_month.is_empty() {
                continue;
            }
            let product_name = row["PRODUCTNAME"].as_str().unwrap_or("");
            if product_name.contains("总计") {
                continue;
            }
            let variety = row
                .get("PRODUCTGROUPID")
                .and_then(|v| v.as_str())
                .map(|s| s.trim().to_uppercase())
                .unwrap_or_else(|| {
                    row.get("PRODUCTID")
                        .and_then(|v| v.as_str())
                        .map(|s| s.split('_').next().unwrap_or("").to_uppercase())
                        .unwrap_or_default()
                });
            let sym = format!("{}{}", variety, delivery_month);
            let turnover = row.get("TURNOVER").map(parse_f64_val).unwrap_or(0.0);
            items.push(FuturesDailyBar {
                symbol: sym,
                date: date.to_string(),
                open: parse_f64_val(&row["OPENPRICE"]),
                high: parse_f64_val(&row["HIGHESTPRICE"]),
                low: parse_f64_val(&row["LOWESTPRICE"]),
                close: parse_f64_val(&row["CLOSEPRICE"]),
                volume: parse_f64_val(&row["VOLUME"]),
                open_interest: parse_f64_val(&row["OPENINTEREST"]),
                turnover,
                settle: parse_f64_val(&row["SETTLEMENTPRICE"]),
                pre_settle: parse_f64_val(&row["PRESETTLEMENTPRICE"]),
                variety,
            });
        }
        Ok(items)
    }

    /// INE daily bar data.
    ///
    /// Fetches daily OHLCV + settlement data from INE (Shanghai International Energy Exchange).
    pub async fn futures_daily_ine(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        let url = format!(
            "https://www.ine.cn/data/tradedata/future/dailydata/kx{}.dat",
            date
        );
        let body = self
                        .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let instruments = data["o_curinstrument"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for row in &instruments {
            let delivery_month = row["DELIVERYMONTH"].as_str().unwrap_or("").to_string();
            if delivery_month.contains("小计") || delivery_month.is_empty() {
                continue;
            }
            let product_name = row["PRODUCTNAME"].as_str().unwrap_or("");
            if product_name.contains("总计") {
                continue;
            }
            let variety = row
                .get("PRODUCTGROUPID")
                .and_then(|v| v.as_str())
                .map(|s| s.trim().to_uppercase())
                .unwrap_or_else(|| {
                    row.get("PRODUCTID")
                        .and_then(|v| v.as_str())
                        .map(|s| s.split('_').next().unwrap_or("").to_uppercase())
                        .unwrap_or_default()
                });
            let sym = format!("{}{}", variety, delivery_month);
            let turnover = row.get("TURNOVER").map(parse_f64_val).unwrap_or(0.0);
            items.push(FuturesDailyBar {
                symbol: sym,
                date: date.to_string(),
                open: parse_f64_val(&row["OPENPRICE"]),
                high: parse_f64_val(&row["HIGHESTPRICE"]),
                low: parse_f64_val(&row["LOWESTPRICE"]),
                close: parse_f64_val(&row["CLOSEPRICE"]),
                volume: parse_f64_val(&row["VOLUME"]),
                open_interest: parse_f64_val(&row["OPENINTEREST"]),
                turnover,
                settle: parse_f64_val(&row["SETTLEMENTPRICE"]),
                pre_settle: parse_f64_val(&row["PRESETTLEMENTPRICE"]),
                variety,
            });
        }
        Ok(items)
    }

    /// DCE daily bar data.
    ///
    /// Fetches daily OHLCV + settlement data from DCE (Dalian Commodity Exchange).
    pub async fn futures_daily_dce(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        let url = "http://www.dce.com.cn/dcereport/publicweb/dailystat/dayQuotes";
        let payload = serde_json::json!({
            "contractId": "",
            "lang": "zh",
            "optionSeries": "",
            "statisticsType": "0",
            "tradeDate": date,
            "tradeType": "1",
            "varietyId": "all",
        });

        let body = self
                        .post(url)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let variety_cn = row["variety"].as_str().unwrap_or("");
            if variety_cn.contains("小计") || variety_cn.contains("总计") {
                continue;
            }
            let sym = row["contractId"].as_str().unwrap_or("").to_string();
            if sym.is_empty() {
                continue;
            }
            let variety = extract_variety(&sym);
            items.push(FuturesDailyBar {
                symbol: sym,
                date: date.to_string(),
                open: parse_f64_val(&row["open"]),
                high: parse_f64_val(&row["high"]),
                low: parse_f64_val(&row["low"]),
                close: parse_f64_val(&row["close"]),
                volume: parse_f64_val(&row["volumn"]),
                open_interest: parse_f64_val(&row["openInterest"]),
                turnover: parse_f64_val(&row["turnover"]),
                settle: parse_f64_val(&row["clearPrice"]),
                pre_settle: parse_f64_val(&row["lastClear"]),
                variety,
            });
        }
        Ok(items)
    }

    /// CZCE daily bar data.
    ///
    /// Fetches daily OHLCV + settlement data from CZCE (Zhengzhou Commodity Exchange).
    /// Uses pipe-delimited text files from CZCE.
    pub async fn futures_daily_czce(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        let year = &date[..4];
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataDaily.txt",
            year, date
        );
        let body = self
                        .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        if body.contains("您的访问出错了") || body.is_empty() {
            return Ok(vec![]);
        }

        let lines: Vec<&str> = body.lines().collect();
        if lines.len() < 3 {
            return Ok(vec![]);
        }

        let mut items = Vec::new();
        for line in &lines[2..] {
            if line.trim().is_empty() || line.starts_with("小") {
                continue;
            }
            let fields: Vec<&str> = line.split('|').map(str::trim).collect();
            if fields.len() < 11 {
                continue;
            }
            let sym = fields[0].to_string();
            if sym.is_empty() || sym.contains("小计") || sym.contains("合计") {
                continue;
            }
            let variety = RE_ALPHA_ONLY
                .find(&sym)
                .map(|m| m.as_str().to_uppercase())
                .unwrap_or_default();
            items.push(FuturesDailyBar {
                symbol: sym,
                date: date.to_string(),
                open: parse_f64_str(fields.get(1).unwrap_or(&"")),
                high: parse_f64_str(fields.get(2).unwrap_or(&"")),
                low: parse_f64_str(fields.get(3).unwrap_or(&"")),
                close: parse_f64_str(fields.get(4).unwrap_or(&"")),
                volume: parse_f64_str(fields.get(5).unwrap_or(&"")),
                open_interest: parse_f64_str(fields.get(6).unwrap_or(&"")),
                turnover: parse_f64_str(fields.get(8).unwrap_or(&"")),
                settle: parse_f64_str(fields.get(7).unwrap_or(&"")),
                pre_settle: 0.0,
                variety,
            });
        }
        Ok(items)
    }

    /// GFEX daily bar data.
    ///
    /// Fetches daily OHLCV + settlement data from GFEX (Guangzhou Futures Exchange).
    pub async fn futures_daily_gfex(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        let url = "http://www.gfex.com.cn/u/interfacesWebTiDayQuotes/loadList";
        let body = self
                        .post(url)
            .form(&[("trade_date", date), ("trade_type", "0")])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let variety_str = row["variety"].as_str().unwrap_or("");
            if variety_str.contains("小计") || variety_str.contains("总计") {
                continue;
            }
            let variety_order = row["varietyOrder"]
                .as_str()
                .unwrap_or("")
                .to_uppercase();
            let deliv_month = row["delivMonth"].as_str().unwrap_or("");
            let sym = format!("{}{}", variety_order, deliv_month);
            items.push(FuturesDailyBar {
                symbol: sym,
                date: date.to_string(),
                open: parse_f64_val(&row["open"]),
                high: parse_f64_val(&row["high"]),
                low: parse_f64_val(&row["low"]),
                close: parse_f64_val(&row["close"]),
                volume: parse_f64_val(&row["volumn"]),
                open_interest: parse_f64_val(&row["openInterest"]),
                turnover: parse_f64_val(&row["turnover"]),
                settle: parse_f64_val(&row["clearPrice"]),
                pre_settle: parse_f64_val(&row["lastClear"]),
                variety: variety_order,
            });
        }
        Ok(items)
    }

    /// Get futures daily data from a specific exchange.
    ///
    /// `market` is one of: "CFFEX", "CZCE", "SHFE", "DCE", "INE", "GFEX".
    pub async fn get_futures_daily(
        &self,
        date: &str,
        market: &str,
    ) -> Result<Vec<FuturesDailyBar>> {
        match market.to_uppercase().as_str() {
            "CFFEX" => self.futures_daily_cffex(date).await,
            "CZCE" => self.futures_daily_czce(date).await,
            "SHFE" => self.futures_daily_shfe(date).await,
            "DCE" => self.futures_daily_dce(date).await,
            "INE" => self.futures_daily_ine(date).await,
            "GFEX" => self.futures_daily_gfex(date).await,
            _ => Err(Error::invalid_input(format!(
                "unsupported market: {}",
                market
            ))),
        }
    }

    /// CFFEX historical daily data using the newer CSV endpoint.
    pub async fn futures_hist_daily_cffex(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        self.futures_daily_cffex(date).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_variety() {
        assert_eq!(extract_variety("IF2403"), "IF");
        assert_eq!(extract_variety("rb2405"), "RB");
        assert_eq!(extract_variety("au2406"), "AU");
    }
}
