//! Eastmoney board data — concept and industry boards: hist, minute, spot, change.
//!
//! Covers Python functions:
//! - `stock_board_concept_hist` — Concept board historical kline
//! - `stock_board_concept_hist_min` — Concept board minute kline
//! - `stock_board_concept_spot` — Concept board spot data
//! - `stock_board_industry_hist` — Industry board historical kline
//! - `stock_board_industry_hist_min` — Industry board minute kline
//! - `stock_board_industry_spot` — Industry board spot data
//! - `stock_board_change` — Board change data

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::AdjustType;
use crate::types::value_ext::ValueExt;
use crate::types::wire::KlineResp;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct StockGetEnvelope {
    data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct TrendsEnvelope {
    data: Option<TrendsData>,
}

#[derive(Debug, Deserialize)]
struct TrendsData {
    trends: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Board historical kline point (concept or industry).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardHistPoint {
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub amount: f64,
    pub amplitude_pct: f64,
    pub change_pct: f64,
    pub change_amount: f64,
    pub turnover_rate: f64,
}

/// Board minute kline point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardMinutePoint {
    pub datetime: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub amount: f64,
}

/// Board spot data item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardSpotItem {
    pub item: String,
    pub value: f64,
}

/// Board change data item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardChangeRow {
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
    pub turnover_rate: Option<f64>,
    #[serde(default)]
    pub main_net_inflow: Option<f64>,
    #[serde(default)]
    pub main_net_inflow_ratio: Option<f64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    // -- Concept board -------------------------------------------------------

    /// Get concept board historical kline data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_concept_hist(symbol, period, start_date, end_date, adjust)`
    ///
    /// - `symbol`: board name (e.g. "绿色电力") or board code (e.g. "BK0715")
    /// - `period`: "daily", "weekly", "monthly"
    /// - `start_date`: "20220101"
    /// - `end_date`: "20221128"
    /// - `adjust`: "", "qfq", "hfq"
    pub async fn stock_board_concept_hist(
        &self,
        symbol: &str,
        period: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<BoardHistPoint>> {
        let secid = self.resolve_board_secid(symbol, "concept").await?;
        let klt = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => return Err(Error::invalid_input(format!("invalid period: {period}"))),
        };
        let fqt = AdjustType::from_adjust_str(adjust)?.to_eastmoney_str();
        self.fetch_board_kline(&secid, klt, fqt, start_date, end_date)
            .await
    }

    /// Get concept board minute kline data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_concept_hist_min(symbol, period)`
    ///
    /// - `symbol`: board name or code
    /// - `period`: "1", "5", "15", "30", "60"
    pub async fn stock_board_concept_hist_min(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<BoardMinutePoint>> {
        let secid = self.resolve_board_secid(symbol, "concept").await?;
        self.fetch_board_minute_kline(&secid, period).await
    }

    /// Get concept board spot data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_concept_spot(symbol)`
    ///
    /// Returns key-value pairs of board metrics.
    pub async fn stock_board_concept_spot(&self, symbol: &str) -> Result<Vec<BoardSpotItem>> {
        let secid = self.resolve_board_secid(symbol, "concept").await?;
        self.fetch_board_spot(&secid).await
    }

    // -- Industry board ------------------------------------------------------

    /// Get industry board historical kline data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_industry_hist(symbol, period, start_date, end_date, adjust)`
    ///
    /// - `symbol`: board name (e.g. "小金属") or board code (e.g. "BK1027")
    /// - `period`: "daily", "weekly", "monthly"
    /// - `start_date`: "20211201"
    /// - `end_date`: "20220401"
    /// - `adjust`: "", "qfq", "hfq"
    pub async fn stock_board_industry_hist(
        &self,
        symbol: &str,
        period: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<BoardHistPoint>> {
        let secid = self.resolve_board_secid(symbol, "industry").await?;
        let klt = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => return Err(Error::invalid_input(format!("invalid period: {period}"))),
        };
        let fqt = AdjustType::from_adjust_str(adjust)?.to_eastmoney_str();
        self.fetch_board_kline(&secid, klt, fqt, start_date, end_date)
            .await
    }

    /// Get industry board minute kline data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_industry_hist_min(symbol, period)`
    ///
    /// - `symbol`: board name or code
    /// - `period`: "1", "5", "15", "30", "60"
    pub async fn stock_board_industry_hist_min(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<BoardMinutePoint>> {
        let secid = self.resolve_board_secid(symbol, "industry").await?;
        self.fetch_board_minute_kline(&secid, period).await
    }

    /// Get industry board spot data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_industry_spot(symbol)`
    pub async fn stock_board_industry_spot(&self, symbol: &str) -> Result<Vec<BoardSpotItem>> {
        let secid = self.resolve_board_secid(symbol, "industry").await?;
        self.fetch_board_spot(&secid).await
    }

    // -- Board change --------------------------------------------------------

    /// Get board change data from Eastmoney.
    ///
    /// Python equivalent: `stock_board_change(symbol)`
    ///
    /// `symbol` is a board filter like "行业板块" or "概念板块".
    pub async fn stock_board_change(&self, symbol: &str) -> Result<Vec<BoardChangeRow>> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            diff: Option<Vec<serde_json::Value>>,
        }
        let fs = match symbol {
            "行业板块" => "m:90 t:2 f:!50",
            "概念板块" => "m:90 t:3 f:!50",
            _ => symbol,
        };

        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(
                    "100",
                    &[
                        ("fid", "f62"),
                        ("fs", fs),
                        ("fields", "f2,f3,f4,f5,f6,f8,f12,f14,f62,f184"),
                    ],
                )),
        )
        .await?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("eastmoney board change missing data"))?;

        let items: Vec<BoardChangeRow> = diff
            .iter()
            .filter_map(|item| {
                let code = item.get("f12")?.as_str()?.to_string();
                let name = item.get("f14")?.as_str()?.to_string();
                Some(BoardChangeRow {
                    code,
                    name,
                    latest_price: item.f64_field(&["f2"]),
                    change_pct: item.f64_field(&["f3"]),
                    change_amount: item.f64_field(&["f4"]),
                    volume: item.f64_field(&["f5"]),
                    amount: item.f64_field(&["f6"]),
                    turnover_rate: item.f64_field(&["f8"]),
                    main_net_inflow: item.f64_field(&["f62"]),
                    main_net_inflow_ratio: item.f64_field(&["f184"]),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no board change items"));
        }
        Ok(items)
    }

    // -- Private helpers -----------------------------------------------------

    /// Resolve board symbol (name or code) to an Eastmoney secid like "90.BK0715".
    pub(crate) async fn resolve_board_secid(
        &self,
        symbol: &str,
        board_type: &str,
    ) -> Result<String> {
        #[derive(Deserialize)]
        struct Env {
            data: Option<EnvData>,
        }
        #[derive(Deserialize)]
        struct EnvData {
            diff: Option<Vec<serde_json::Value>>,
        }
        // If it already looks like a BK code, use directly
        if symbol.starts_with("BK") {
            return Ok(format!("90.{symbol}"));
        }

        // Otherwise resolve name -> code by fetching the board list
        let fs = match board_type {
            "concept" => "m:90 t:3 f:!50",
            "industry" => "m:90 t:2 f:!50",
            _ => {
                return Err(Error::invalid_input(format!(
                    "invalid board type: {board_type}"
                )));
            }
        };

        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(
                    "5000",
                    &[("fid", "f12"), ("fs", fs), ("fields", "f12,f14")],
                )),
        )
        .await?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("eastmoney board list missing data"))?;

        for item in &diff {
            let name = item.str_or(&["f14"], "");
            if name == symbol {
                let code = item.str_field(&["f12"]).ok_or_else(|| {
                    Error::not_found(format!("board code not found for: {symbol}"))
                })?;
                return Ok(format!("90.{code}"));
            }
        }

        Err(Error::not_found(format!("board not found: {symbol}")))
    }

    /// Fetch board kline (daily/weekly/monthly).
    async fn fetch_board_kline(
        &self,
        secid: &str,
        klt: &str,
        fqt: &str,
        beg: &str,
        end: &str,
    ) -> Result<Vec<BoardHistPoint>> {
        let response = crate::util::send_and_check(
            self.get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
                .query(&[
                    ("secid", secid),
                    ("fields1", "f1,f2,f3,f4,f5,f6"),
                    ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                    ("klt", klt),
                    ("fqt", fqt),
                    ("beg", beg),
                    ("end", end),
                    ("smplmt", "10000"),
                    ("lmt", "1000000"),
                ]),
        )
        .await?;

        let payload: KlineResp = response.json().await.map_err(Error::from)?;
        let klines = payload
            .data
            .and_then(|d| d.klines)
            .ok_or_else(|| Error::upstream("board kline missing data"))?;

        let items: Vec<BoardHistPoint> = klines
            .iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() < 11 {
                    return None;
                }
                Some(BoardHistPoint {
                    date: parts[0].to_string(),
                    open: parts[1].parse().unwrap_or(0.0),
                    close: parts[2].parse().unwrap_or(0.0),
                    high: parts[3].parse().unwrap_or(0.0),
                    low: parts[4].parse().unwrap_or(0.0),
                    volume: parts[5].parse().unwrap_or(0.0),
                    amount: parts[6].parse().unwrap_or(0.0),
                    amplitude_pct: parts[7].parse().unwrap_or(0.0),
                    change_pct: parts[8].parse().unwrap_or(0.0),
                    change_amount: parts[9].parse().unwrap_or(0.0),
                    turnover_rate: parts[10].parse().unwrap_or(0.0),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("board kline returned no data"));
        }
        Ok(items)
    }

    /// Fetch board minute kline.
    async fn fetch_board_minute_kline(
        &self,
        secid: &str,
        period: &str,
    ) -> Result<Vec<BoardMinutePoint>> {
        if period == "1" {
            // 1-minute uses trends endpoint
            let response = crate::util::send_and_check(
                self.get("https://push2his.eastmoney.com/api/qt/stock/trends2/get")
                    .query(&[
                        ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13"),
                        ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58"),
                        ("iscr", "0"),
                        ("ndays", "1"),
                        ("secid", secid),
                    ]),
            )
            .await?;

            let payload: TrendsEnvelope = response.json().await.map_err(Error::from)?;
            let trends = payload
                .data
                .and_then(|d| d.trends)
                .ok_or_else(|| Error::upstream("board trends missing data"))?;

            let items: Vec<BoardMinutePoint> = trends
                .iter()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() < 8 {
                        return None;
                    }
                    Some(BoardMinutePoint {
                        datetime: parts[0].to_string(),
                        open: parts[1].parse().unwrap_or(0.0),
                        close: parts[2].parse().unwrap_or(0.0),
                        high: parts[3].parse().unwrap_or(0.0),
                        low: parts[4].parse().unwrap_or(0.0),
                        volume: parts[5].parse().unwrap_or(0.0),
                        amount: parts[6].parse().unwrap_or(0.0),
                    })
                })
                .collect();

            if items.is_empty() {
                return Err(Error::not_found("board 1-min trends returned no data"));
            }
            Ok(items)
        } else {
            // 5/15/30/60-minute uses kline endpoint
            let response = crate::util::send_and_check(
                self.get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
                    .query(&[
                        ("secid", secid),
                        ("fields1", "f1,f2,f3,f4,f5,f6"),
                        ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                        ("klt", period),
                        ("fqt", "1"),
                        ("end", "20500101"),
                        ("lmt", "1000000"),
                    ]),
            )
            .await?;

            let payload: KlineResp = response.json().await.map_err(Error::from)?;
            let klines = payload
                .data
                .and_then(|d| d.klines)
                .ok_or_else(|| Error::upstream("board minute kline missing data"))?;

            let items: Vec<BoardMinutePoint> = klines
                .iter()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() < 11 {
                        return None;
                    }
                    Some(BoardMinutePoint {
                        datetime: parts[0].to_string(),
                        open: parts[1].parse().unwrap_or(0.0),
                        close: parts[2].parse().unwrap_or(0.0),
                        high: parts[3].parse().unwrap_or(0.0),
                        low: parts[4].parse().unwrap_or(0.0),
                        volume: parts[5].parse().unwrap_or(0.0),
                        amount: parts[6].parse().unwrap_or(0.0),
                    })
                })
                .collect();

            if items.is_empty() {
                return Err(Error::not_found("board minute kline returned no data"));
            }
            Ok(items)
        }
    }

    /// Fetch board spot data.
    async fn fetch_board_spot(&self, secid: &str) -> Result<Vec<BoardSpotItem>> {
        let field_map = [
            ("f43", "最新"),
            ("f44", "最高"),
            ("f45", "最低"),
            ("f46", "开盘"),
            ("f47", "成交量"),
            ("f48", "成交额"),
            ("f170", "涨跌幅"),
            ("f171", "振幅"),
            ("f168", "换手率"),
            ("f169", "涨跌额"),
        ];
        let fields: String = field_map
            .iter()
            .map(|(f, _)| *f)
            .collect::<Vec<_>>()
            .join(",");

        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/stock/get")
                .query(&[
                    ("fields", fields.as_str()),
                    ("mpi", "1000"),
                    ("invt", "2"),
                    ("fltt", "1"),
                    ("secid", secid),
                ]),
        )
        .await?;

        let payload: StockGetEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload
            .data
            .ok_or_else(|| Error::upstream("board spot missing data"))?;

        if !data.is_object() {
            return Err(Error::decode("board spot data is not an object"));
        }

        let mut items = Vec::new();
        for (field_key, label) in &field_map {
            if let Some(val) = data.f64_field(&[*field_key]) {
                // Apply unit conversion: divide by 100 for most fields, keep volume/amount as-is
                let converted = if *field_key == "f47" || *field_key == "f48" {
                    val
                } else {
                    val * 0.01
                };
                items.push(BoardSpotItem {
                    item: label.to_string(),
                    value: converted,
                });
            }
        }

        if items.is_empty() {
            return Err(Error::not_found("board spot returned no data"));
        }
        Ok(items)
    }
}
