//! Board/concept data functions — concept and industry board listings,
//! historical data, constituents, and index data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::value_ext::ValueExt;
use crate::types::wire::ClistResp;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct BoardIndexEnvelope {
    data: Option<BoardIndexData>,
}

#[derive(Debug, Deserialize)]
struct BoardIndexData {
    diff: Option<Vec<serde_json::Value>>,
}

/// Board constituent stock item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardConsItem {
    pub code: String,
    pub name: String,
    pub latest_price: Option<f64>,
    pub change_pct: Option<f64>,
    pub change_amount: Option<f64>,
    pub volume: Option<f64>,
    pub amount: Option<f64>,
    pub turnover_rate: Option<f64>,
}

/// Board index point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardIndexItem {
    pub code: String,
    pub name: String,
    pub latest_price: Option<f64>,
    pub change_pct: Option<f64>,
    pub change_amount: Option<f64>,
    pub volume: Option<f64>,
    pub amount: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub prev_close: Option<f64>,
}

impl AkShareClient {
    // -----------------------------------------------------------------------
    // Concept board
    // -----------------------------------------------------------------------

    /// Get concept board historical data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_concept_hist_em(symbol, period, start_date, end_date, adjust)`
    ///
    /// - `symbol`: board name (e.g. "绿色电力") or board code (e.g. "BK0715")
    /// - `period`: "daily", "weekly", "monthly"
    /// - `start_date`: "20220101"
    /// - `end_date`: "20221128"
    /// - `adjust`: "", "qfq", "hfq"
    pub async fn stock_board_concept_hist_em(
        &self,
        symbol: &str,
        period: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let secid = self.resolve_board_secid(symbol, "concept").await?;
        let klt = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => return Err(Error::invalid_input(format!("invalid period: {period}"))),
        };
        let fqt = match adjust {
            "" => "0",
            "qfq" => "1",
            "hfq" => "2",
            _ => return Err(Error::invalid_input(format!("invalid adjust: {adjust}"))),
        };

        let response = crate::util::send_and_check(
            self.get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
                .query(&[
                    ("secid", secid.as_str()),
                    ("fields1", "f1,f2,f3,f4,f5,f6"),
                    ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                    ("klt", klt),
                    ("fqt", fqt),
                    ("beg", start_date),
                    ("end", end_date),
                    ("smplmt", "10000"),
                    ("lmt", "1000000"),
                ]),
        )
        .await?;

        let payload: crate::types::wire::KlineResp = response.json().await.map_err(Error::from)?;
        let klines = payload
            .data
            .and_then(|d| d.klines)
            .ok_or_else(|| Error::upstream("concept board hist missing data"))?;

        let items: Vec<serde_json::Value> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 11 {
                    return None;
                }
                let obj = serde_json::json!({
                    "date": parts[0],
                    "open": parts[1].parse::<f64>().unwrap_or(0.0),
                    "close": parts[2].parse::<f64>().unwrap_or(0.0),
                    "high": parts[3].parse::<f64>().unwrap_or(0.0),
                    "low": parts[4].parse::<f64>().unwrap_or(0.0),
                    "volume": parts[5].parse::<f64>().unwrap_or(0.0),
                    "amount": parts[6].parse::<f64>().unwrap_or(0.0),
                    "amplitude_pct": parts[7].parse::<f64>().unwrap_or(0.0),
                    "change_pct": parts[8].parse::<f64>().unwrap_or(0.0),
                    "change_amount": parts[9].parse::<f64>().unwrap_or(0.0),
                    "turnover_rate": parts[10].parse::<f64>().unwrap_or(0.0),
                });
                Some(obj)
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("concept board hist returned no data"));
        }
        Ok(items)
    }

    /// Get concept board constituent stocks from Eastmoney.
    ///
    /// Python equivalent: `stock_board_concept_cons_em(symbol)`
    ///
    /// - `symbol`: board name or code
    pub async fn stock_board_concept_cons_em(&self, symbol: &str) -> Result<Vec<BoardConsItem>> {
        let secid = self.resolve_board_secid(symbol, "concept").await?;
        let code = secid.split('.').nth(1).unwrap_or("");

        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(
                    "5000",
                    &[
                        ("fid", "f3"),
                        ("fs", &format!("b:{code}+f:!50")),
                        ("fields", "f2,f3,f4,f5,f6,f7,f8,f12,f14,f15,f16,f17,f18"),
                    ],
                )),
        )
        .await?;

        let payload: ClistResp = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("concept board cons missing data"))?;

        let items: Vec<BoardConsItem> = diff
            .iter()
            .filter_map(|item| {
                let code = item.str_field(&["f12"])?.to_string();
                let name = item.str_field(&["f14"])?.to_string();
                Some(BoardConsItem {
                    code,
                    name,
                    latest_price: item.f64_field(&["f2"]),
                    change_pct: item.f64_field(&["f3"]),
                    change_amount: item.f64_field(&["f4"]),
                    volume: item.f64_field(&["f5"]),
                    amount: item.f64_field(&["f6"]),
                    turnover_rate: item.f64_field(&["f8"]),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("concept board cons returned no data"));
        }
        Ok(items)
    }

    /// Get concept board index data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_concept_index_em(symbol)`
    ///
    /// - `symbol`: board name or code
    pub async fn stock_board_concept_index_em(&self, symbol: &str) -> Result<Vec<BoardIndexItem>> {
        let secid = self.resolve_board_secid(symbol, "concept").await?;

        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/stock/get")
                .query(&[
                    ("secid", secid.as_str()),
                    ("fields", "f43,f44,f45,f46,f47,f48,f50,f51,f52,f55,f57,f58,f60,f71,f116,f117,f162,f163,f164,f167,f168,f169,f170,f171"),
                    ("mpi", "1000"),
                    ("invt", "2"),
                    ("fltt", "1"),
                ]),
        )
        .await?;

        let payload: BoardIndexEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload
            .data
            .ok_or_else(|| Error::upstream("concept board index missing data"))?;

        let items: Vec<BoardIndexItem> = diff_to_index_items(&data.diff.unwrap_or_default());

        if items.is_empty() {
            return Err(Error::not_found("concept board index returned no data"));
        }
        Ok(items)
    }

    // -----------------------------------------------------------------------
    // Industry board
    // -----------------------------------------------------------------------

    /// Get industry board historical data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_industry_hist_em(symbol, period, start_date, end_date, adjust)`
    ///
    /// - `symbol`: board name (e.g. "小金属") or board code (e.g. "BK1027")
    /// - `period`: "daily", "weekly", "monthly"
    /// - `start_date`: "20211201"
    /// - `end_date`: "20220401"
    /// - `adjust`: "", "qfq", "hfq"
    pub async fn stock_board_industry_hist_em(
        &self,
        symbol: &str,
        period: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let secid = self.resolve_board_secid(symbol, "industry").await?;
        let klt = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => return Err(Error::invalid_input(format!("invalid period: {period}"))),
        };
        let fqt = match adjust {
            "" => "0",
            "qfq" => "1",
            "hfq" => "2",
            _ => return Err(Error::invalid_input(format!("invalid adjust: {adjust}"))),
        };

        let response = crate::util::send_and_check(
            self.get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
                .query(&[
                    ("secid", secid.as_str()),
                    ("fields1", "f1,f2,f3,f4,f5,f6"),
                    ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                    ("klt", klt),
                    ("fqt", fqt),
                    ("beg", start_date),
                    ("end", end_date),
                    ("smplmt", "10000"),
                    ("lmt", "1000000"),
                ]),
        )
        .await?;

        let payload: crate::types::wire::KlineResp = response.json().await.map_err(Error::from)?;
        let klines = payload
            .data
            .and_then(|d| d.klines)
            .ok_or_else(|| Error::upstream("industry board hist missing data"))?;

        let items: Vec<serde_json::Value> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 11 {
                    return None;
                }
                let obj = serde_json::json!({
                    "date": parts[0],
                    "open": parts[1].parse::<f64>().unwrap_or(0.0),
                    "close": parts[2].parse::<f64>().unwrap_or(0.0),
                    "high": parts[3].parse::<f64>().unwrap_or(0.0),
                    "low": parts[4].parse::<f64>().unwrap_or(0.0),
                    "volume": parts[5].parse::<f64>().unwrap_or(0.0),
                    "amount": parts[6].parse::<f64>().unwrap_or(0.0),
                    "amplitude_pct": parts[7].parse::<f64>().unwrap_or(0.0),
                    "change_pct": parts[8].parse::<f64>().unwrap_or(0.0),
                    "change_amount": parts[9].parse::<f64>().unwrap_or(0.0),
                    "turnover_rate": parts[10].parse::<f64>().unwrap_or(0.0),
                });
                Some(obj)
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("industry board hist returned no data"));
        }
        Ok(items)
    }

    /// Get industry board constituent stocks from Eastmoney.
    ///
    /// Python equivalent: `stock_board_industry_cons_em(symbol)`
    ///
    /// - `symbol`: board name or code
    pub async fn stock_board_industry_cons_em(&self, symbol: &str) -> Result<Vec<BoardConsItem>> {
        let secid = self.resolve_board_secid(symbol, "industry").await?;
        let code = secid.split('.').nth(1).unwrap_or("");

        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(
                    "5000",
                    &[
                        ("fid", "f3"),
                        ("fs", &format!("b:{code}+f:!50")),
                        ("fields", "f2,f3,f4,f5,f6,f7,f8,f12,f14,f15,f16,f17,f18"),
                    ],
                )),
        )
        .await?;

        let payload: ClistResp = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("industry board cons missing data"))?;

        let items: Vec<BoardConsItem> = diff
            .iter()
            .filter_map(|item| {
                let code = item.str_field(&["f12"])?.to_string();
                let name = item.str_field(&["f14"])?.to_string();
                Some(BoardConsItem {
                    code,
                    name,
                    latest_price: item.f64_field(&["f2"]),
                    change_pct: item.f64_field(&["f3"]),
                    change_amount: item.f64_field(&["f4"]),
                    volume: item.f64_field(&["f5"]),
                    amount: item.f64_field(&["f6"]),
                    turnover_rate: item.f64_field(&["f8"]),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("industry board cons returned no data"));
        }
        Ok(items)
    }

    /// Get industry board index data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_industry_index_em(symbol)`
    ///
    /// - `symbol`: board name or code
    pub async fn stock_board_industry_index_em(&self, symbol: &str) -> Result<Vec<BoardIndexItem>> {
        let secid = self.resolve_board_secid(symbol, "industry").await?;

        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/stock/get")
                .query(&[
                    ("secid", secid.as_str()),
                    ("fields", "f43,f44,f45,f46,f47,f48,f50,f51,f52,f55,f57,f58,f60,f71,f116,f117,f162,f163,f164,f167,f168,f169,f170,f171"),
                    ("mpi", "1000"),
                    ("invt", "2"),
                    ("fltt", "1"),
                ]),
        )
        .await?;

        let payload: BoardIndexEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload
            .data
            .ok_or_else(|| Error::upstream("industry board index missing data"))?;

        let items: Vec<BoardIndexItem> = diff_to_index_items(&data.diff.unwrap_or_default());

        if items.is_empty() {
            return Err(Error::not_found("industry board index returned no data"));
        }
        Ok(items)
    }
}

/// Convert a single-item diff array from `stock/get` into a `BoardIndexItem`.
fn diff_to_index_items(diff: &[serde_json::Value]) -> Vec<BoardIndexItem> {
    diff.iter()
        .filter_map(|item| {
            let code = item.str_or(&["f57"], "");
            let name = item.str_or(&["f58"], "");
            Some(BoardIndexItem {
                code,
                name,
                latest_price: item.f64_field(&["f43"]),
                change_pct: item.f64_field(&["f170"]),
                change_amount: item.f64_field(&["f169"]),
                volume: item.f64_field(&["f47"]),
                amount: item.f64_field(&["f48"]),
                high: item.f64_field(&["f44"]),
                low: item.f64_field(&["f45"]),
                open: item.f64_field(&["f46"]),
                prev_close: item.f64_field(&["f60"]),
            })
        })
        .collect()
}
