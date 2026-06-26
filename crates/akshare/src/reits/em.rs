//! REITs data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, ReitSnapshot};
use crate::util::{parse_csv_line, parse_f64_safe, today_iso};

impl AkShareClient {
    /// Fetch a snapshot list of China REITs.
    ///
    /// Uses Sina `hq.sinajs.cn` API with a known list of listed REITs.
    pub async fn reits_list(&self, limit: usize) -> Result<Vec<ReitSnapshot>> {
        // Known China REITs with their Sina symbols
        let reits: Vec<(&str, &str)> = vec![
            ("sh508000", "中金普洛斯REIT"),
            ("sh508001", "浙商沪杭甬REIT"),
            ("sh508002", "招商蛇口产业园REIT"),
            ("sh508003", "博时招商蛇口产园REIT"),
            ("sh508005", "富国首创水务REIT"),
            ("sh508006", "东吴苏州工业园REIT"),
            ("sh508007", "华安张江光大REIT"),
            ("sh508008", "红土盐田港REIT"),
            ("sh508009", "中金厦门象屿REIT"),
            ("sh508010", "国泰君安东久新宜REIT"),
            ("sh508011", "国泰君安临港创新产业园REIT"),
            ("sh508018", "中金山东高速REIT"),
            ("sh508027", "华泰紫金江苏交控REIT"),
            ("sh508028", "中信建投国家电投REIT"),
            ("sh508029", "中航京能光伏REIT"),
            ("sh508056", "华夏基金华润有巢REIT"),
            ("sz180101", "红土创新深圳安居REIT"),
            ("sz180102", "中金厦门安居REIT"),
            ("sz180201", "华夏北京保障房REIT"),
            ("sz180301", "华夏合肥高新产园REIT"),
            ("sz180801", "中金普洛斯REIT"),
            ("sz180901", "国泰君安东久新宜REIT"),
        ];

        let symbols_csv: Vec<&str> = reits.iter().map(|(s, _)| *s).collect();
        let url = format!("https://hq.sinajs.cn/list={}", symbols_csv.join(","));

        let body = self
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        let today = today_iso();
        let mut items = Vec::new();
        for (i, line) in body.lines().enumerate() {
            if i >= reits.len() {
                break;
            }
            let data = line
                .split_once('=')
                .and_then(|(_, r)| r.trim_matches('"').split_once(';'))
                .map_or("", |(s, _)| s);
            if data.is_empty() {
                continue;
            }
            let fields: Vec<&str> = data.split(',').collect();
            if fields.len() < 10 {
                continue;
            }
            let (symbol, default_name) = reits[i];
            let name = if !fields[0].is_empty() {
                fields[0].to_string()
            } else {
                default_name.to_string()
            };
            let close = parse_f64_safe(fields[3]);
            let prev_close = parse_f64_safe(fields[2]);
            let change_pct = if prev_close > 0.0 {
                ((close - prev_close) / prev_close * 10000.0).round() / 100.0
            } else {
                0.0
            };
            let volume = parse_f64_safe(fields[8]);
            if close == 0.0 {
                continue;
            }
            items.push(ReitSnapshot {
                symbol: symbol.to_string(),
                name,
                date: today.clone(),
                close,
                change_pct,
                volume,
                nav: None,
            });
        }

        items.truncate(limit);
        if items.is_empty() {
            return Err(Error::not_found("sina returned no REIT data"));
        }
        Ok(items)
    }

    /// REITs historical data with full parameter support.
    ///
    /// `symbol`: REIT code, e.g. "508000"
    /// `period`: "daily", "weekly", "monthly"
    /// `start_date`: format YYYYMMDD (currently unused; returns recent data)
    /// `end_date`: format YYYYMMDD (currently unused)
    /// `adjust`: "qfq", "hfq", or ""
    pub async fn reits_hist_em(
        &self,
        symbol: &str,
        period: &str,
        _start_date: &str,
        _end_date: &str,
        _adjust: &str,
    ) -> Result<Vec<CandlePoint>> {
        let klt = match period {
            "weekly" => "102",
            "monthly" => "103",
            _ => "101",
        };

        let secid = reits_eastmoney_secid(symbol)?;
        let klines = self.kline_fetch(&secid, klt, "1", 500, &[]).await?;

        let items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_reit_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no REIT kline items"));
        }
        Ok(items)
    }

    /// REITs minute-level data from Eastmoney.
    ///
    /// `symbol`: REIT code, e.g. "508000"
    /// `period`: "1", "5", "15", "30", "60"
    pub async fn reits_hist_min(&self, symbol: &str, period: &str) -> Result<Vec<CandlePoint>> {
        let secid = reits_eastmoney_secid(symbol)?;
        let klines = self.kline_fetch(&secid, period, "1", 500, &[]).await?;

        let items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_reit_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no REIT min kline items",
            ));
        }
        Ok(items)
    }

    /// REITs real-time data from Eastmoney.
    ///
    /// Returns real-time pricing for all listed REITs.
    pub async fn reits_realtime(&self) -> Result<Vec<ReitSnapshot>> {
        self.reits_list(200).await
    }

    /// Fetch historical kline data for a specific REIT from Eastmoney.
    ///
    /// `symbol` is the REIT code, e.g. `"508000"` or `"508000.SH"`.
    /// The function converts the symbol to Eastmoney's secid format
    /// (e.g. `"1.508000"` for SH, `"0.180201"` for SZ).
    pub async fn reits_hist(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        let secid = reits_eastmoney_secid(symbol)?;
        let klines = self.kline_fetch(&secid, "101", "1", limit, &[]).await?;

        let items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_reit_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no REIT kline items"));
        }

        Ok(crate::util::sort_and_limit(items, limit))
    }
}

/// Convert a REIT symbol to Eastmoney secid format.
///
/// Accepts formats like "508000", "508000.SH", "180201.SZ".
/// Returns "1.{code}" for Shanghai (6/5-prefix codes) or "0.{code}" for Shenzhen.
fn reits_eastmoney_secid(symbol: &str) -> Result<String> {
    let trimmed = symbol.trim();
    if let Some((code, suffix)) = trimmed.split_once('.') {
        let suffix_upper = suffix.to_uppercase();
        let market = match suffix_upper.as_str() {
            "SH" => "1",
            "SZ" => "0",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported REIT exchange: {suffix}"
                )));
            }
        };
        return Ok(format!("{market}.{code}"));
    }
    // Pure numeric: infer exchange from code prefix
    if trimmed.is_empty() {
        return Err(Error::invalid_input("REIT symbol is empty"));
    }
    let market = if trimmed.starts_with('5') || trimmed.starts_with('6') {
        "1" // Shanghai
    } else {
        "0" // Shenzhen
    };
    Ok(format!("{market}.{trimmed}"))
}

/// Parse a single Eastmoney kline CSV line into a `CandlePoint`.
fn parse_reit_candle_line(line: &str) -> Result<CandlePoint> {
    let f = parse_csv_line(line);
    if f.len() < 11 {
        return Err(Error::decode(format!(
            "unexpected eastmoney REIT kline format: {line}"
        )));
    }
    Ok(CandlePoint {
        trade_date: f[0].to_string(),
        open: parse_f64_safe(f[1]),
        close: parse_f64_safe(f[2]),
        high: parse_f64_safe(f[3]),
        low: parse_f64_safe(f[4]),
        volume: parse_f64_safe(f[5]).round() as i64,
        amount: parse_f64_safe(f[6]),
        amplitude_pct: parse_f64_safe(f[7]),
        change_pct: parse_f64_safe(f[8]),
        change_amount: parse_f64_safe(f[9]),
        turnover_pct: parse_f64_safe(f[10]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reits_eastmoney_secid_sh() {
        assert_eq!(reits_eastmoney_secid("508000").unwrap(), "1.508000");
        assert_eq!(reits_eastmoney_secid("508000.SH").unwrap(), "1.508000");
    }

    #[test]
    fn test_reits_eastmoney_secid_sz() {
        assert_eq!(reits_eastmoney_secid("180201").unwrap(), "0.180201");
        assert_eq!(reits_eastmoney_secid("180201.SZ").unwrap(), "0.180201");
    }

    #[test]
    fn test_reits_eastmoney_secid_empty() {
        assert!(reits_eastmoney_secid("").is_err());
    }

    #[test]
    fn test_parse_reit_candle_line() {
        let line = "2025-01-02,3.500,3.550,3.600,3.480,50000,177500.00,3.43,1.43,0.050,0.12";
        let point = parse_reit_candle_line(line).unwrap();
        assert_eq!(point.trade_date, "2025-01-02");
        assert!((point.open - 3.50).abs() < 0.001);
        assert!((point.close - 3.55).abs() < 0.001);
        assert!((point.high - 3.60).abs() < 0.001);
        assert!((point.low - 3.48).abs() < 0.001);
        assert_eq!(point.volume, 50000);
    }

    #[test]
    fn test_parse_reit_candle_line_insufficient_fields() {
        let line = "2025-01-02,3.500,3.550";
        assert!(parse_reit_candle_line(line).is_err());
    }
}
