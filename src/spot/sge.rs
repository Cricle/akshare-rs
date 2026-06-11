#![allow(dead_code)]
//! Shanghai Gold Exchange (上海黄金交易所) spot data.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

/// Available SGE symbol list.
const SGE_SYMBOLS: &[&str] = &[
    "Au99.99", "Au99.95", "Au100g", "Pt99.95", "Ag(T+D)", "Au(T+D)",
    "mAu(T+D)", "Au(T+N1)", "Au(T+N2)", "Ag99.99", "iAu99.99", "Au99.5",
    "iAu100g", "iAu99.5", "PGC30g", "NYAuTN06", "NYAuTN12",
];

#[derive(Debug, Deserialize)]
struct SgeQuotationResp {
    heyue: Option<Vec<String>>,
    times: Option<Vec<String>>,
    data: Option<Vec<serde_json::Value>>,
    delaystr: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SgeHistResp {
    time: Option<Vec<Vec<serde_json::Value>>>,
}

#[derive(Debug, Deserialize)]
struct SgeBenchmarkResp {
    wp: Option<Vec<Vec<serde_json::Value>>>,
    zp: Option<Vec<Vec<serde_json::Value>>>,
}

/// List available SGE symbols.
pub fn spot_symbol_table_sge() -> Vec<&'static str> {
    SGE_SYMBOLS.to_vec()
}

impl AkShareClient {
    /// Fetch SGE real-time quotation data for a given symbol.
    ///
    /// Returns intraday price series with timestamps.
    pub async fn spot_quotations_sge(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let resp: SgeQuotationResp = self
                        .get("https://www.sge.com.cn/graph/quotations")
            .query(&[("instid", symbol)])
            .send()
            .await?
            .json()
            .await?;

        let times = resp.times.unwrap_or_default();
        let data = resp.data.unwrap_or_default();

        if times.is_empty() || data.is_empty() {
            return Err(Error::not_found(format!("sge returned no data for {symbol}")));
        }

        let items: Vec<MacroDataPoint> = times
            .into_iter()
            .zip(data.into_iter())
            .filter_map(|(time, val)| {
                let price = val.as_f64()?;
                Some(MacroDataPoint {
                    date: time,
                    value: price,
                    name: symbol.to_string(),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(format!("no valid sge data for {symbol}")));
        }
        Ok(items)
    }

    /// Fetch SGE historical daily OHLC data for a given symbol.
    ///
    /// Returns daily open/close/high/low data.
    pub async fn spot_hist_sge(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let resp: SgeHistResp = self
                        .post("https://www.sge.com.cn/graph/Dailyhq")
            .form(&[("instid", symbol)])
            .send()
            .await?
            .json()
            .await?;

        let rows = resp.time.unwrap_or_default();
        if rows.is_empty() {
            return Err(Error::not_found(format!("sge returned no hist data for {symbol}")));
        }

        let items: Vec<MacroDataPoint> = rows
            .into_iter()
            .filter_map(|row| {
                if row.len() < 5 {
                    return None;
                }
                let date_val = row[0].as_f64().or_else(|| {
                    row[0].as_str().and_then(|s| {
                        chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                            .ok()
                            .and_then(|d| {
                                d.and_hms_milli_opt(0, 0, 0, 0)
                                    .map(|dt| dt.and_utc().timestamp_millis() as f64)
                            })
                    })
                })?;
                let date = chrono::DateTime::from_timestamp_millis(date_val as i64)?;
                let close = row[2].as_f64()?;
                Some(MacroDataPoint {
                    date: date.format("%Y-%m-%d").to_string(),
                    value: close,
                    name: symbol.to_string(),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(format!("no valid sge hist data for {symbol}")));
        }
        Ok(items)
    }

    /// Fetch Shanghai Gold benchmark price historical data.
    ///
    /// Returns early and late session benchmark prices.
    pub async fn spot_golden_benchmark_sge(&self) -> Result<Vec<MacroDataPoint>> {
        let resp: SgeBenchmarkResp = self
                        .post("https://www.sge.com.cn/graph/DayilyJzj")
            .send()
            .await?
            .json()
            .await?;

        parse_benchmark(resp, "Shanghai Gold")
    }

    /// Fetch Shanghai Silver benchmark price historical data.
    ///
    /// Returns early and late session benchmark prices.
    pub async fn spot_silver_benchmark_sge(&self) -> Result<Vec<MacroDataPoint>> {
        let resp: SgeBenchmarkResp = self
                        .post("https://www.sge.com.cn/graph/DayilyShsilverJzj")
            .send()
            .await?
            .json()
            .await?;

        parse_benchmark(resp, "Shanghai Silver")
    }
}

fn parse_benchmark(resp: SgeBenchmarkResp, label: &str) -> Result<Vec<MacroDataPoint>> {
    let wp = resp.wp.unwrap_or_default();
    let zp = resp.zp.unwrap_or_default();

    let mut items = Vec::new();
    for row in &wp {
        if row.len() >= 2
            && let (Some(ts), Some(val)) = (row[0].as_f64(), row[1].as_f64())
                && let Some(dt) = chrono::DateTime::from_timestamp_millis(ts as i64) {
                    items.push(MacroDataPoint {
                        date: dt.format("%Y-%m-%d").to_string(),
                        value: val,
                        name: format!("{label} late"),
                    });
                }
    }
    for row in &zp {
        if row.len() >= 2
            && let (Some(ts), Some(val)) = (row[0].as_f64(), row[1].as_f64())
                && let Some(dt) = chrono::DateTime::from_timestamp_millis(ts as i64) {
                    items.push(MacroDataPoint {
                        date: dt.format("%Y-%m-%d").to_string(),
                        value: val,
                        name: format!("{label} early"),
                    });
                }
    }

    if items.is_empty() {
        return Err(Error::not_found(format!("sge returned no benchmark data for {label}")));
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sge_symbols() {
        let syms = spot_symbol_table_sge();
        assert_eq!(syms.len(), 17);
        assert!(syms.contains(&"Au99.99"));
        assert!(syms.contains(&"Ag(T+D)"));
    }
}
