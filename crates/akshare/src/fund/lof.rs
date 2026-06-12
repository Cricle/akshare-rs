//! LOF (Listed Open-ended Fund) data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, EtfSpotItem, FundSnapshot};
use crate::util::{parse_f64_safe, today_iso};

/// Map LOF symbol to Eastmoney secid.
fn lof_secid(symbol: &str) -> Result<String> {
    let s = symbol.trim();
    if s.contains('.') && s.len() >= 3 {
        return Ok(s.to_string());
    }
    if s.len() == 6 && s.chars().all(|c| c.is_ascii_digit()) {
        let prefix = if s.starts_with('1') || s.starts_with('0') {
            "0"
        } else if s.starts_with('5') {
            "1"
        } else {
            "0"
        };
        return Ok(format!("{prefix}.{s}"));
    }
    Err(Error::invalid_input(format!(
        "invalid LOF symbol: {symbol}"
    )))
}

impl AkShareClient {
    /// Fetch LOF fund list from Eastmoney.
    pub async fn fund_lof_list(&self, limit: usize) -> Result<Vec<FundSnapshot>> {
        let pz = limit.max(1).to_string();
        let response = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", pz.as_str()),
                ("po", "1"),
                ("np", "1"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fs", "b:MK0025"),
                ("fields", "f12,f14,f2,f3"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let items = payload
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let date = today_iso();
        let snapshots: Vec<FundSnapshot> = items
            .into_iter()
            .take(limit)
            .filter_map(|v| {
                Some(FundSnapshot {
                    symbol: v.get("f12")?.as_str()?.to_string(),
                    name: v
                        .get("f14")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string(),
                    date: date.clone(),
                    nav: v
                        .get("f2")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    acc_nav: v
                        .get("f2")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    change_pct: v
                        .get("f3")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    fund_type: Some("lof".to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found("eastmoney returned no LOF fund data"));
        }
        Ok(snapshots)
    }

    /// Fetch historical daily candles for a LOF fund.
    pub async fn fund_lof_hist(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        let secid = lof_secid(symbol)?;
        self.eastmoney_klines(&secid, "qfq", limit).await
    }

    /// Fetch LOF historical candles from Eastmoney with full parameters.
    ///
    /// `symbol`: 6-digit LOF code.
    /// `period`: "daily", "weekly", or "monthly".
    /// `start_date` / `end_date`: format "YYYYMMDD".
    /// `adjust`: "qfq", "hfq", or "".
    pub async fn fund_lof_hist_em(
        &self,
        symbol: &str,
        period: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<CandlePoint>> {
        let period_map = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported period: {period}"
                )));
            }
        };
        let adjust_map = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "" | "none" => "0",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported adjust: {adjust}"
                )));
            }
        };
        let secid = lof_secid(symbol)?;

        let response = self
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                (
                    "fields2",
                    "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f116",
                ),
                ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
                ("klt", period_map),
                ("fqt", adjust_map),
                ("secid", secid.as_str()),
                ("beg", start_date),
                ("end", end_date),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let klines = payload
            .get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .ok_or_else(|| Error::not_found(format!("no kline data for LOF {symbol}")))?;

        let mut candles = Vec::new();
        for kline in klines {
            let s = kline.as_str().unwrap_or("");
            let fields: Vec<&str> = s.split(',').collect();
            if fields.len() < 11 {
                continue;
            }
            candles.push(CandlePoint {
                trade_date: fields[0].to_string(),
                open: parse_f64_safe(fields[1]),
                close: parse_f64_safe(fields[2]),
                high: parse_f64_safe(fields[3]),
                low: parse_f64_safe(fields[4]),
                volume: fields[5].parse().unwrap_or(0),
                amount: parse_f64_safe(fields[6]),
                amplitude_pct: parse_f64_safe(fields[7]),
                change_pct: parse_f64_safe(fields[8]),
                change_amount: parse_f64_safe(fields[9]),
                turnover_pct: parse_f64_safe(fields[10]),
            });
        }
        Ok(candles)
    }

    /// Fetch LOF minute-level historical data from Eastmoney.
    ///
    /// `symbol`: 6-digit LOF code.
    /// `period`: "1", "5", "15", "30", or "60".
    /// `start_date` / `end_date`: format "YYYY-MM-DD HH:MM:SS".
    /// `adjust`: "", "qfq", or "hfq".
    pub async fn fund_lof_hist_min_em(
        &self,
        symbol: &str,
        period: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<CandlePoint>> {
        let adjust_map = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "" | "none" => "0",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported adjust: {adjust}"
                )));
            }
        };
        let secid = lof_secid(symbol)?;

        if period == "1" {
            let response = self
                .get("https://push2his.eastmoney.com/api/qt/stock/trends2/get")
                .query(&[
                    ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13"),
                    ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58"),
                    ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
                    ("ndays", "5"),
                    ("iscr", "0"),
                    ("secid", secid.as_str()),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
            let trends = payload
                .get("data")
                .and_then(|d| d.get("trends"))
                .and_then(|t| t.as_array())
                .ok_or_else(|| Error::not_found(format!("no trend data for LOF {symbol}")))?;

            let mut candles = Vec::new();
            for trend in trends {
                let s = trend.as_str().unwrap_or("");
                let fields: Vec<&str> = s.split(',').collect();
                if fields.len() < 8 {
                    continue;
                }
                let dt = fields[0];
                if dt < start_date || dt > end_date {
                    continue;
                }
                candles.push(CandlePoint {
                    trade_date: dt.to_string(),
                    open: parse_f64_safe(fields[1]),
                    close: parse_f64_safe(fields[2]),
                    high: parse_f64_safe(fields[3]),
                    low: parse_f64_safe(fields[4]),
                    volume: fields[5].parse().unwrap_or(0),
                    amount: parse_f64_safe(fields[6]),
                    amplitude_pct: 0.0,
                    change_pct: 0.0,
                    change_amount: 0.0,
                    turnover_pct: 0.0,
                });
            }
            Ok(candles)
        } else {
            let response = self
                .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
                .query(&[
                    ("fields1", "f1,f2,f3,f4,f5,f6"),
                    ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                    ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
                    ("klt", period),
                    ("fqt", adjust_map),
                    ("secid", secid.as_str()),
                    ("beg", "0"),
                    ("end", "20500000"),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
            let klines = payload
                .get("data")
                .and_then(|d| d.get("klines"))
                .and_then(|k| k.as_array())
                .ok_or_else(|| {
                    Error::not_found(format!("no minute kline data for LOF {symbol}"))
                })?;

            let mut candles = Vec::new();
            for kline in klines {
                let s = kline.as_str().unwrap_or("");
                let fields: Vec<&str> = s.split(',').collect();
                if fields.len() < 11 {
                    continue;
                }
                let dt = fields[0];
                if dt < start_date || dt > end_date {
                    continue;
                }
                candles.push(CandlePoint {
                    trade_date: dt.to_string(),
                    open: parse_f64_safe(fields[1]),
                    close: parse_f64_safe(fields[2]),
                    high: parse_f64_safe(fields[3]),
                    low: parse_f64_safe(fields[4]),
                    volume: fields[5].parse().unwrap_or(0),
                    amount: parse_f64_safe(fields[6]),
                    amplitude_pct: parse_f64_safe(fields[7]),
                    change_pct: parse_f64_safe(fields[8]),
                    change_amount: parse_f64_safe(fields[9]),
                    turnover_pct: parse_f64_safe(fields[10]),
                });
            }
            Ok(candles)
        }
    }

    /// Fetch LOF spot (real-time) data from Eastmoney.
    pub async fn fund_lof_spot_em(&self) -> Result<Vec<EtfSpotItem>> {
        let response = self
            .get("https://88.push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", "10000"),
                ("po", "1"),
                ("np", "1"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f3"),
                ("fs", "b:MK0404,b:MK0405,b:MK0406,b:MK0407"),
                (
                    "fields",
                    "f2,f3,f4,f5,f6,f7,f12,f14,f15,f16,f17,f18,f20,f21",
                ),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let items = payload
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let result: Vec<EtfSpotItem> = items
            .into_iter()
            .filter_map(|v| {
                Some(EtfSpotItem {
                    code: v.get("f12")?.as_str()?.to_string(),
                    name: v
                        .get("f14")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string(),
                    latest_price: v
                        .get("f2")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    change_pct: v
                        .get("f3")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    change_amount: v
                        .get("f4")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    volume: v
                        .get("f5")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    amount: v
                        .get("f6")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    open: v
                        .get("f17")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    high: v
                        .get("f15")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    low: v
                        .get("f16")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    prev_close: v
                        .get("f18")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    amplitude: v
                        .get("f7")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    turnover_rate: 0.0,
                    iopv: 0.0,
                    discount_rate: 0.0,
                    shares: 0.0,
                    circ_mv: v
                        .get("f21")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    total_mv: v
                        .get("f20")
                        .and_then(serde_json::Value::as_f64)
                        .unwrap_or(0.0),
                    data_date: String::new(),
                })
            })
            .collect();

        if result.is_empty() {
            return Err(Error::not_found("no LOF spot data"));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lof_secid_shenzhen() {
        assert_eq!(lof_secid("160105").unwrap(), "0.160105");
    }

    #[test]
    fn test_lof_secid_shanghai() {
        assert_eq!(lof_secid("501000").unwrap(), "1.501000");
    }

    #[test]
    fn test_lof_secid_raw() {
        assert_eq!(lof_secid("0.160105").unwrap(), "0.160105");
    }

    #[test]
    fn test_lof_secid_invalid() {
        assert!(lof_secid("abc").is_err());
        assert!(lof_secid("12345").is_err());
    }
}
