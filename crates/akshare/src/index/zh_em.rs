//! Eastmoney A-share index historical data — daily and intraday.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::{parse_csv_line, parse_f64_safe};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 东方财富 — 中国股票指数行情数据 (daily / weekly / monthly).
    ///
    /// `symbol` is a bare index code, e.g. "000300", "399006".
    /// `period` is "daily", "weekly", or "monthly".
    pub async fn index_zh_a_hist(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<IndexZhAHistPoint>> {
        let klt = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported period: {period}"
                )));
            }
        };

        // Try market prefixes in order: 1 (SH), 0 (SZ), 2 (CSI), 47
        for market in &["1", "0", "2", "47"] {
            let secid = format!("{market}.{symbol}");
            let Ok(klines) = self.kline_fetch(&secid, klt, "0", usize::MAX, &[]).await else {
                continue;
            };
            let points: Vec<IndexZhAHistPoint> = klines
                .iter()
                .map(|line| parse_zh_a_hist_line(line))
                .collect::<Result<Vec<_>>>()?;
            return Ok(points);
        }

        Err(Error::not_found(format!(
            "eastmoney returned no index data for {symbol}"
        )))
    }

    /// Eastmoney index minute data with full parameter support (Python-compatible name).
    ///
    /// `symbol`: index code, e.g. "000300"
    /// `period`: "1", "5", "15", "30", "60"
    /// `start_date`: format YYYYMMDD (currently unused)
    /// `end_date`: format YYYYMMDD (currently unused)
    /// `adjust`: "qfq", "hfq", or ""
    pub async fn index_zh_a_hist_min_em(
        &self,
        symbol: &str,
        period: &str,
        _start_date: &str,
        _end_date: &str,
        _adjust: &str,
    ) -> Result<Vec<IndexZhAHistMinPoint>> {
        self.index_zh_a_hist_min(symbol, period).await
    }

    /// 东方财富 — 指数分时行情 (1/5/15/30/60 min).
    ///
    /// For `period` = "1", uses the trends endpoint.
    /// For other periods, uses the kline endpoint.
    pub async fn index_zh_a_hist_min(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<IndexZhAHistMinPoint>> {
        if period == "1" {
            return self.index_zh_a_hist_trends(symbol).await;
        }

        let secid = format!("1.{symbol}");
        let klines = self
            .kline_fetch(&secid, period, "1", usize::MAX, &[])
            .await?;

        let points: Vec<IndexZhAHistMinPoint> = klines
            .iter()
            .map(|line| {
                let f = parse_csv_line(line);
                if f.len() < 11 {
                    return Err(Error::decode(format!("unexpected min kline: {line}")));
                }
                Ok(IndexZhAHistMinPoint {
                    time: f[0].to_string(),
                    open: parse_f64_safe(f[1]),
                    close: parse_f64_safe(f[2]),
                    high: parse_f64_safe(f[3]),
                    low: parse_f64_safe(f[4]),
                    change_pct: parse_f64_safe(f[8]),
                    change_amount: parse_f64_safe(f[9]),
                    volume: parse_f64_safe(f[5]),
                    amount: parse_f64_safe(f[6]),
                    amplitude_pct: parse_f64_safe(f[7]),
                    turnover_pct: parse_f64_safe(f[10]),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        if points.is_empty() {
            return Err(Error::not_found("eastmoney returned no intraday data"));
        }
        Ok(points)
    }

    // Private: 1-min trends
    async fn index_zh_a_hist_trends(&self, symbol: &str) -> Result<Vec<IndexZhAHistMinPoint>> {
        #[derive(Deserialize)]
        struct TrendsEnvelope {
            data: Option<TrendsData>,
        }
        #[derive(Deserialize)]
        struct TrendsData {
            trends: Option<Vec<String>>,
        }

        let secid = format!("1.{symbol}");
        let response = self
            .get("https://push2his.eastmoney.com/api/qt/stock/trends2/get")
            .query(&[
                ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58"),
                ("iscr", "0"),
                ("ndays", "5"),
                ("secid", secid.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: TrendsEnvelope = response.json().await.map_err(Error::from)?;
        let trends = payload.data.and_then(|d| d.trends).unwrap_or_default();

        let points: Vec<IndexZhAHistMinPoint> = trends
            .iter()
            .map(|line| {
                let f = parse_csv_line(line);
                if f.len() < 8 {
                    return Err(Error::decode(format!("unexpected trend line: {line}")));
                }
                Ok(IndexZhAHistMinPoint {
                    time: f[0].to_string(),
                    open: parse_f64_safe(f[1]),
                    close: parse_f64_safe(f[2]),
                    high: parse_f64_safe(f[3]),
                    low: parse_f64_safe(f[4]),
                    change_pct: 0.0,
                    change_amount: 0.0,
                    volume: parse_f64_safe(f[5]),
                    amount: parse_f64_safe(f[6]),
                    amplitude_pct: 0.0,
                    turnover_pct: 0.0,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        if points.is_empty() {
            return Err(Error::not_found("eastmoney returned no trend data"));
        }
        Ok(points)
    }

    /// 东方财富-股票和市场代码 (Python: index_code_id_map_em)
    ///
    /// Returns a mapping of stock codes to market IDs.
    pub async fn index_code_id_map_em(&self) -> Result<std::collections::HashMap<String, i64>> {
        let items = self
            .clist_spot_fetch(
                "b:MK0010,m:1+t:1,m:0 t:5,m:1+s:3,m:0+t:5,m:2",
                "f3,f12,f13",
                "100",
                "f3",
            )
            .await?;
        let mut map = std::collections::HashMap::new();
        for item in &items {
            if let (Some(code), Some(id)) = (
                item.get("f12").and_then(|v| v.as_str()),
                item.get("f13").and_then(serde_json::Value::as_i64),
            ) {
                map.insert(code.to_string(), id);
            }
        }
        Ok(map)
    }
}

/// A-share index history point.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexZhAHistPoint {
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
    pub turnover_pct: f64,
}

/// A-share index intraday point.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexZhAHistMinPoint {
    pub time: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub change_pct: f64,
    pub change_amount: f64,
    pub volume: f64,
    pub amount: f64,
    pub amplitude_pct: f64,
    pub turnover_pct: f64,
}

fn parse_zh_a_hist_line(line: &str) -> Result<IndexZhAHistPoint> {
    let f = parse_csv_line(line);
    if f.len() < 11 {
        return Err(Error::decode(format!(
            "unexpected eastmoney kline format: {line}"
        )));
    }
    Ok(IndexZhAHistPoint {
        date: f[0].to_string(),
        open: parse_f64_safe(f[1]),
        close: parse_f64_safe(f[2]),
        high: parse_f64_safe(f[3]),
        low: parse_f64_safe(f[4]),
        volume: parse_f64_safe(f[5]),
        amount: parse_f64_safe(f[6]),
        amplitude_pct: parse_f64_safe(f[7]),
        change_pct: parse_f64_safe(f[8]),
        change_amount: parse_f64_safe(f[9]),
        turnover_pct: parse_f64_safe(f[10]),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_zh_a_hist_line() {
        let line = "2025-01-02,3350.00,3360.50,3370.00,3340.00,123456789,987654321.00,0.90,0.31,10.50,1.23";
        let p = super::parse_zh_a_hist_line(line).unwrap();
        assert_eq!(p.date, "2025-01-02");
        assert!((p.close - 3360.50).abs() < 0.01);
    }
}
