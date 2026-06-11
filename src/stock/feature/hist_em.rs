//! Historical data from Eastmoney (stock_zh_a_hist, stock_hk_hist, stock_us_hist, etc.)

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::market::*;
use super::types::HistData;
use crate::util::parse_csv_line;
use crate::util::parse_f64_safe;

impl AkShareClient {
    /// 东方财富-沪深京A股-历史行情
    pub async fn stock_zh_a_hist(
        &self,
        symbol: &str,
        period: &str,
        adjust: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HistData>> {
        let klt = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => return Err(Error::invalid_input(format!("unsupported period: {period}"))),
        };
        let fqt = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "" => "0",
            _ => return Err(Error::invalid_input(format!("unsupported adjust: {adjust}"))),
        };
        let secid = eastmoney_secid(symbol)?;
        let resp = self
                        .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", klt),
                ("fqt", fqt),
                ("beg", if start_date.is_empty() { "0" } else { start_date }),
                ("end", if end_date.is_empty() { "20500000" } else { end_date }),
                ("lmt", "1000000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let klines = json
            .get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .ok_or_else(|| Error::upstream("missing klines data"))?;

        Ok(klines
            .iter()
            .filter_map(|k| {
                let s = k.as_str()?;
                let f = parse_csv_line(s);
                if f.len() < 11 {
                    return None;
                }
                Some(HistData {
                    trade_date: f[0].to_string(),
                    open: parse_f64_safe(f[1]),
                    close: parse_f64_safe(f[2]),
                    high: parse_f64_safe(f[3]),
                    low: parse_f64_safe(f[4]),
                    volume: parse_f64_safe(f[5]),
                    amount: parse_f64_safe(f[6]),
                    amplitude_pct: parse_f64_safe(f[7]),
                    change_pct: parse_f64_safe(f[8]),
                    change_amount: parse_f64_safe(f[9]),
                    turnover_rate: parse_f64_safe(f[10]),
                })
            })
            .collect())
    }

    /// 东方财富-沪深A股-分时行情
    pub async fn stock_zh_a_hist_min_em(
        &self,
        symbol: &str,
        period: &str,
        adjust: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HistData>> {
        let klt = match period {
            "1" => "1",
            "5" => "5",
            "15" => "15",
            "30" => "30",
            "60" => "60",
            _ => return Err(Error::invalid_input(format!("unsupported period: {period}"))),
        };
        let fqt = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "" => "0",
            _ => "0",
        };
        let secid = eastmoney_secid(symbol)?;
        let resp = self
                        .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", klt),
                ("fqt", fqt),
                ("beg", if start_date.is_empty() { "0" } else { start_date }),
                ("end", if end_date.is_empty() { "20500000" } else { end_date }),
                ("lmt", "1000000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let klines = json
            .get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .ok_or_else(|| Error::upstream("missing klines data"))?;

        Ok(klines
            .iter()
            .filter_map(|k| {
                let s = k.as_str()?;
                let f = parse_csv_line(s);
                if f.len() < 11 {
                    return None;
                }
                Some(HistData {
                    trade_date: f[0].to_string(),
                    open: parse_f64_safe(f[1]),
                    close: parse_f64_safe(f[2]),
                    high: parse_f64_safe(f[3]),
                    low: parse_f64_safe(f[4]),
                    volume: parse_f64_safe(f[5]),
                    amount: parse_f64_safe(f[6]),
                    amplitude_pct: parse_f64_safe(f[7]),
                    change_pct: parse_f64_safe(f[8]),
                    change_amount: parse_f64_safe(f[9]),
                    turnover_rate: parse_f64_safe(f[10]),
                })
            })
            .collect())
    }

    /// 东方财富-港股-历史行情
    pub async fn stock_hk_hist(
        &self,
        symbol: &str,
        period: &str,
        adjust: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HistData>> {
        let klt = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => return Err(Error::invalid_input(format!("unsupported period: {period}"))),
        };
        let fqt = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "" => "0",
            _ => "0",
        };
        let secid = format!("116.{}", symbol.trim());
        let resp = self
                        .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", klt),
                ("fqt", fqt),
                ("beg", if start_date.is_empty() { "0" } else { start_date }),
                ("end", if end_date.is_empty() { "20500000" } else { end_date }),
                ("lmt", "1000000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let klines = json
            .get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .ok_or_else(|| Error::upstream("missing klines data"))?;

        Ok(klines
            .iter()
            .filter_map(|k| {
                let s = k.as_str()?;
                let f = parse_csv_line(s);
                if f.len() < 11 {
                    return None;
                }
                Some(HistData {
                    trade_date: f[0].to_string(),
                    open: parse_f64_safe(f[1]),
                    close: parse_f64_safe(f[2]),
                    high: parse_f64_safe(f[3]),
                    low: parse_f64_safe(f[4]),
                    volume: parse_f64_safe(f[5]),
                    amount: parse_f64_safe(f[6]),
                    amplitude_pct: parse_f64_safe(f[7]),
                    change_pct: parse_f64_safe(f[8]),
                    change_amount: parse_f64_safe(f[9]),
                    turnover_rate: parse_f64_safe(f[10]),
                })
            })
            .collect())
    }

    /// 东方财富-港股-分时行情
    pub async fn stock_hk_hist_min_em(
        &self,
        symbol: &str,
        period: &str,
        adjust: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HistData>> {
        let klt = match period {
            "1" => "1",
            "5" => "5",
            "15" => "15",
            "30" => "30",
            "60" => "60",
            _ => return Err(Error::invalid_input(format!("unsupported period: {period}"))),
        };
        let fqt = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "" => "0",
            _ => "0",
        };
        let secid = format!("116.{}", symbol.trim());
        let resp = self
                        .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", klt),
                ("fqt", fqt),
                ("beg", if start_date.is_empty() { "0" } else { start_date }),
                ("end", if end_date.is_empty() { "20500000" } else { end_date }),
                ("lmt", "1000000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let klines = json
            .get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .ok_or_else(|| Error::upstream("missing klines data"))?;

        Ok(klines
            .iter()
            .filter_map(|k| {
                let s = k.as_str()?;
                let f = parse_csv_line(s);
                if f.len() < 11 {
                    return None;
                }
                Some(HistData {
                    trade_date: f[0].to_string(),
                    open: parse_f64_safe(f[1]),
                    close: parse_f64_safe(f[2]),
                    high: parse_f64_safe(f[3]),
                    low: parse_f64_safe(f[4]),
                    volume: parse_f64_safe(f[5]),
                    amount: parse_f64_safe(f[6]),
                    amplitude_pct: parse_f64_safe(f[7]),
                    change_pct: parse_f64_safe(f[8]),
                    change_amount: parse_f64_safe(f[9]),
                    turnover_rate: parse_f64_safe(f[10]),
                })
            })
            .collect())
    }

    /// 东方财富-美股-历史行情
    pub async fn stock_us_hist(
        &self,
        symbol: &str,
        period: &str,
        adjust: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HistData>> {
        let klt = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => return Err(Error::invalid_input(format!("unsupported period: {period}"))),
        };
        let fqt = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "" => "0",
            _ => "0",
        };
        // US stocks use 105/106 market codes
        let secid = if symbol.contains('.') {
            let parts: Vec<&str> = symbol.split('.').collect();
            if parts.len() == 2 {
                let code = parts[0];
                let suffix = parts[1].to_uppercase();
                let market = match suffix.as_str() {
                    "O" => "105",
                    "N" => "106",
                    _ => "105",
                };
                format!("{}.{}", market, code)
            } else {
                format!("105.{}", symbol)
            }
        } else {
            format!("105.{}", symbol)
        };

        let resp = self
                        .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", klt),
                ("fqt", fqt),
                ("beg", if start_date.is_empty() { "0" } else { start_date }),
                ("end", if end_date.is_empty() { "20500000" } else { end_date }),
                ("lmt", "1000000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let klines = json
            .get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .ok_or_else(|| Error::upstream("missing klines data"))?;

        Ok(klines
            .iter()
            .filter_map(|k| {
                let s = k.as_str()?;
                let f = parse_csv_line(s);
                if f.len() < 11 {
                    return None;
                }
                Some(HistData {
                    trade_date: f[0].to_string(),
                    open: parse_f64_safe(f[1]),
                    close: parse_f64_safe(f[2]),
                    high: parse_f64_safe(f[3]),
                    low: parse_f64_safe(f[4]),
                    volume: parse_f64_safe(f[5]),
                    amount: parse_f64_safe(f[6]),
                    amplitude_pct: parse_f64_safe(f[7]),
                    change_pct: parse_f64_safe(f[8]),
                    change_amount: parse_f64_safe(f[9]),
                    turnover_rate: parse_f64_safe(f[10]),
                })
            })
            .collect())
    }

    /// 东方财富-美股-分时行情
    pub async fn stock_us_hist_min_em(
        &self,
        symbol: &str,
        period: &str,
        adjust: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HistData>> {
        let klt = match period {
            "1" => "1",
            "5" => "5",
            "15" => "15",
            "30" => "30",
            "60" => "60",
            _ => return Err(Error::invalid_input(format!("unsupported period: {period}"))),
        };
        let fqt = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "" => "0",
            _ => "0",
        };
        let secid = format!("105.{}", symbol);
        let resp = self
                        .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", klt),
                ("fqt", fqt),
                ("beg", if start_date.is_empty() { "0" } else { start_date }),
                ("end", if end_date.is_empty() { "20500000" } else { end_date }),
                ("lmt", "1000000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let klines = json
            .get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .ok_or_else(|| Error::upstream("missing klines data"))?;

        Ok(klines
            .iter()
            .filter_map(|k| {
                let s = k.as_str()?;
                let f = parse_csv_line(s);
                if f.len() < 11 {
                    return None;
                }
                Some(HistData {
                    trade_date: f[0].to_string(),
                    open: parse_f64_safe(f[1]),
                    close: parse_f64_safe(f[2]),
                    high: parse_f64_safe(f[3]),
                    low: parse_f64_safe(f[4]),
                    volume: parse_f64_safe(f[5]),
                    amount: parse_f64_safe(f[6]),
                    amplitude_pct: parse_f64_safe(f[7]),
                    change_pct: parse_f64_safe(f[8]),
                    change_amount: parse_f64_safe(f[9]),
                    turnover_rate: parse_f64_safe(f[10]),
                })
            })
            .collect())
    }
}
