//! Eastmoney fund flow data — individual stocks and rankings.
//!
//! Covers Python functions:
//! - `stock_individual_fund_flow` — Individual stock fund flow
//! - `stock_individual_fund_flow_rank` — Fund flow ranking
//! - `stock_market_fund_flow` — Market-level fund flow

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::CapitalFlowPoint;
use crate::types::value_ext::ValueExt;
use crate::types::wire::{ClistResp, KlineResp};
use crate::util::{parse_csv_line, parse_f64_safe};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Fund flow ranking entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundFlowRank {
    pub symbol: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub main_net_inflow: Option<f64>,
    #[serde(default)]
    pub main_net_inflow_pct: Option<f64>,
    #[serde(default)]
    pub super_large_net_inflow: Option<f64>,
    #[serde(default)]
    pub super_large_net_inflow_pct: Option<f64>,
    #[serde(default)]
    pub large_net_inflow: Option<f64>,
    #[serde(default)]
    pub large_net_inflow_pct: Option<f64>,
    #[serde(default)]
    pub medium_net_inflow: Option<f64>,
    #[serde(default)]
    pub medium_net_inflow_pct: Option<f64>,
    #[serde(default)]
    pub small_net_inflow: Option<f64>,
    #[serde(default)]
    pub small_net_inflow_pct: Option<f64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get individual stock fund flow history.
    ///
    /// Python equivalent: `stock_individual_fund_flow(stock, market)`
    ///
    /// `market` is "sh", "sz", "bj", "hk", or "us".
    pub async fn stock_individual_fund_flow(
        &self,
        symbol: &str,
        market: &str,
        limit: usize,
    ) -> Result<Vec<CapitalFlowPoint>> {
        let secid = match market {
            "sh" => format!("1.{symbol}"),
            "sz" | "bj" => format!("0.{symbol}"),
            "hk" => format!("116.{symbol}"),
            "us" => {
                // NASDAQ: 105, NYSE: 106 — default to NASDAQ
                // Common NYSE tickers: single-letter or specific known ones
                let market_code = if is_nyse_symbol(symbol) { "106" } else { "105" };
                format!("{market_code}.{symbol}")
            }
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported market: {market}"
                )));
            }
        };
        let lmt = limit.to_string();
        let response = crate::util::send_and_check(
            self.get("https://push2his.eastmoney.com/api/qt/stock/fflow/daykline/get")
                .query(&[
                    ("lmt", lmt.as_str()),
                    ("klt", "101"),
                    ("secid", secid.as_str()),
                    ("fields1", "f1,f2,f3,f7"),
                    (
                        "fields2",
                        "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64,f65",
                    ),
                    ("ut", "b2884a393a59ad64002292a3e90d46a5"),
                ]),
        )
        .await?;

        let payload: KlineResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .data
            .ok_or_else(|| Error::upstream("eastmoney fund flow response missing data"))?;
        let klines = data
            .klines
            .ok_or_else(|| Error::upstream("eastmoney fund flow response missing klines"))?;

        let mut items: Vec<CapitalFlowPoint> = klines
            .iter()
            .map(|line| parse_capital_flow_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no fund flow items"));
        }
        items.truncate(limit);
        Ok(items)
    }

    /// Get fund flow ranking for all stocks.
    ///
    /// Python equivalent: `stock_individual_fund_flow_rank(indicator)`
    ///
    /// `indicator` is one of: "today", "3day", "5day", "10day".
    pub async fn stock_individual_fund_flow_rank(
        &self,
        indicator: &str,
        limit: usize,
    ) -> Result<Vec<FundFlowRank>> {
        let (sort_field, fields) = match indicator {
            "today" => (
                "f62",
                "f12,f14,f2,f3,f62,f184,f66,f69,f72,f75,f78,f81,f84,f87,f204,f205,f124",
            ),
            "3day" => (
                "f267",
                "f12,f14,f2,f127,f267,f268,f269,f270,f271,f272,f273,f274,f275,f276,f257,f258,f124",
            ),
            "5day" => (
                "f164",
                "f12,f14,f2,f109,f164,f165,f166,f167,f168,f169,f170,f171,f172,f173,f257,f258,f124",
            ),
            "10day" => (
                "f174",
                "f12,f14,f2,f160,f174,f175,f176,f177,f178,f179,f180,f181,f182,f183,f260,f261,f124",
            ),
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported indicator: {indicator}"
                )));
            }
        };

        let pz = limit.to_string();
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(pz.as_str(), &[
                    ("fid", sort_field),
                    ("fs", "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23"),
                    ("fields", fields),
                ])),
        )
        .await?;

        let payload: ClistResp = response.json().await.map_err(Error::from)?;
        let diff = payload
            .data
            .and_then(|d| d.diff)
            .ok_or_else(|| Error::upstream("eastmoney fund flow rank missing data"))?;

        let items: Vec<FundFlowRank> = diff
            .iter()
            .take(limit)
            .filter_map(|item| {
                let symbol = item.get("f12")?.as_str()?.to_string();
                let name = item.get("f14")?.as_str()?.to_string();
                Some(FundFlowRank {
                    symbol,
                    name,
                    latest_price: item.f64_field(&["f2"]),
                    change_pct: item.f64_field(&["f3"]),
                    main_net_inflow: item.f64_field(&["f62"]),
                    main_net_inflow_pct: item.f64_field(&["f184"]),
                    super_large_net_inflow: item.f64_field(&["f66"]),
                    super_large_net_inflow_pct: item.f64_field(&["f69"]),
                    large_net_inflow: item.f64_field(&["f72"]),
                    large_net_inflow_pct: item.f64_field(&["f75"]),
                    medium_net_inflow: item.f64_field(&["f78"]),
                    medium_net_inflow_pct: item.f64_field(&["f81"]),
                    small_net_inflow: item.f64_field(&["f84"]),
                    small_net_inflow_pct: item.f64_field(&["f87"]),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no fund flow rank items",
            ));
        }
        Ok(items)
    }
}

// ---------------------------------------------------------------------------
// Market detection helpers
// ---------------------------------------------------------------------------

/// Heuristic to detect NYSE vs NASDAQ for US stocks.
/// NYSE tends to have shorter tickers (1-3 chars) and known patterns.
pub(crate) fn is_nyse_symbol(symbol: &str) -> bool {
    // Single-letter tickers are almost always NYSE
    if symbol.len() == 1 {
        return true;
    }
    // Known NYSE tickers (common ones)
    matches!(
        symbol,
        "BABA"
            | "TSM"
            | "NIO"
            | "JD"
            | "PDD"
            | "LI"
            | "XPEV"
            | "V"
            | "JPM"
            | "WMT"
            | "PG"
            | "JNJ"
            | "UNH"
            | "HD"
            | "MA"
            | "DIS"
            | "NV"
            | "KO"
            | "PEP"
            | "MRK"
            | "ABBV"
            | "CVX"
            | "XOM"
            | "LLY"
            | "TMO"
            | "COST"
            | "WFC"
            | "BAC"
            | "CRM"
            | "AMD"
            | "ORCL"
            | "T"
            | "VZ"
            | "NFLX"
            | "INTC"
            | "GS"
            | "CAT"
            | "BA"
            | "MMM"
            | "IBM"
            | "RTX"
            | "LMT"
            | "GE"
            | "F"
            | "GM"
            | "AAL"
            | "DAL"
            | "UAL"
            | "CCL"
    )
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

fn parse_capital_flow_line(line: &str) -> Result<CapitalFlowPoint> {
    let f = parse_csv_line(line);
    if f.len() < 13 {
        return Err(Error::decode(format!(
            "unexpected eastmoney capital flow format: {line}"
        )));
    }
    Ok(CapitalFlowPoint {
        trade_date: f[0].to_string(),
        main_net_inflow: parse_f64_safe(f[1]),
        small_net_inflow: parse_f64_safe(f[2]),
        medium_net_inflow: parse_f64_safe(f[3]),
        large_net_inflow: parse_f64_safe(f[4]),
        super_large_net_inflow: parse_f64_safe(f[5]),
        main_net_inflow_ratio_pct: parse_f64_safe(f[6]),
        small_net_inflow_ratio_pct: parse_f64_safe(f[7]),
        medium_net_inflow_ratio_pct: parse_f64_safe(f[8]),
        large_net_inflow_ratio_pct: parse_f64_safe(f[9]),
        super_large_net_inflow_ratio_pct: parse_f64_safe(f[10]),
        close: parse_f64_safe(f[11]),
        change_pct: parse_f64_safe(f[12]),
    })
}
