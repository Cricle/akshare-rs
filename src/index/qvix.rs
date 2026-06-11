//! Option QVIX volatility indices — 18 functions (9 daily + 9 intraday).
//!
//! Daily data is sourced from a single CSV at `http://1.optbbs.com/d/csv/d/k.csv`
//! with different column slices per instrument.  Intraday data comes from
//! individual CSV files per instrument.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{QvixDailyPoint, QvixMinPoint};
use crate::util::parse_f64_safe;

// ---------------------------------------------------------------------------
// Column index slices for the combined daily CSV (0-based)
// ---------------------------------------------------------------------------

// Columns: 0=date, 1-4=50ETF(OHLC), 5-8=?, 9-12=300ETF, ...
// We define the (open, high, low, close) column indices for each instrument.
const COLS_50ETF: [usize; 4] = [1, 2, 3, 4];
const COLS_300ETF: [usize; 4] = [9, 10, 11, 12];
const COLS_300INDEX: [usize; 4] = [17, 18, 19, 20];
const COLS_1000INDEX: [usize; 4] = [25, 26, 27, 28];
const COLS_500ETF: [usize; 4] = [67, 68, 69, 70];
const COLS_CYB: [usize; 4] = [71, 72, 73, 74];
const COLS_100ETF: [usize; 4] = [75, 76, 77, 78];
const COLS_50INDEX: [usize; 4] = [79, 80, 81, 82];
const COLS_KCB: [usize; 4] = [83, 84, 85, 86];

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Fetch the combined daily CSV and parse specific columns into `QvixDailyPoint`.
async fn fetch_qvix_daily(
    client: &AkShareClient,
    cols: &[usize; 4],
) -> Result<Vec<QvixDailyPoint>> {
    let body = client
                .get("http://1.optbbs.com/d/csv/d/k.csv")
        .send()
        .await
        .map_err(Error::from)?
        .error_for_status()
        .map_err(Error::from)?
        .text()
        .await
        .map_err(Error::from)?;

    let mut points = Vec::new();
    for (i, line) in body.lines().enumerate() {
        if i == 0 {
            continue; // skip header
        }
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() <= cols[3] {
            continue;
        }
        let date = fields[0].trim().to_string();
        if date.is_empty() {
            continue;
        }
        points.push(QvixDailyPoint {
            date,
            open: parse_f64_safe(fields[cols[0]]),
            high: parse_f64_safe(fields[cols[1]]),
            low: parse_f64_safe(fields[cols[2]]),
            close: parse_f64_safe(fields[cols[3]]),
        });
    }

    if points.is_empty() {
        return Err(Error::not_found("optbbs returned no daily QVIX data"));
    }
    Ok(points)
}

/// Fetch a single intraday QVIX CSV.
async fn fetch_qvix_min(
    client: &AkShareClient,
    url: &str,
) -> Result<Vec<QvixMinPoint>> {
    let body = client
                .get(url)
        .send()
        .await
        .map_err(Error::from)?
        .error_for_status()
        .map_err(Error::from)?
        .text()
        .await
        .map_err(Error::from)?;

    let mut points = Vec::new();
    for (i, line) in body.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 2 {
            continue;
        }
        let time = fields[0].trim().to_string();
        let qvix = parse_f64_safe(fields[1]);
        if time.is_empty() {
            continue;
        }
        points.push(QvixMinPoint { time, qvix });
    }

    if points.is_empty() {
        return Err(Error::not_found("optbbs returned no intraday QVIX data"));
    }
    Ok(points)
}

// ---------------------------------------------------------------------------
// Public API (18 functions)
// ---------------------------------------------------------------------------

impl AkShareClient {
    // -- Daily (9) --

    /// 50ETF 期权波动率指数 QVIX (daily).
    pub async fn index_option_50etf_qvix(&self) -> Result<Vec<QvixDailyPoint>> {
        fetch_qvix_daily(self, &COLS_50ETF).await
    }

    /// 300ETF 期权波动率指数 QVIX (daily).
    pub async fn index_option_300etf_qvix(&self) -> Result<Vec<QvixDailyPoint>> {
        fetch_qvix_daily(self, &COLS_300ETF).await
    }

    /// 500ETF 期权波动率指数 QVIX (daily).
    pub async fn index_option_500etf_qvix(&self) -> Result<Vec<QvixDailyPoint>> {
        fetch_qvix_daily(self, &COLS_500ETF).await
    }

    /// 创业板期权波动率指数 QVIX (daily).
    pub async fn index_option_cyb_qvix(&self) -> Result<Vec<QvixDailyPoint>> {
        fetch_qvix_daily(self, &COLS_CYB).await
    }

    /// 科创板期权波动率指数 QVIX (daily).
    pub async fn index_option_kcb_qvix(&self) -> Result<Vec<QvixDailyPoint>> {
        fetch_qvix_daily(self, &COLS_KCB).await
    }

    /// 深证100ETF 期权波动率指数 QVIX (daily).
    pub async fn index_option_100etf_qvix(&self) -> Result<Vec<QvixDailyPoint>> {
        fetch_qvix_daily(self, &COLS_100ETF).await
    }

    /// 中证300股指期权波动率指数 QVIX (daily).
    pub async fn index_option_300index_qvix(&self) -> Result<Vec<QvixDailyPoint>> {
        fetch_qvix_daily(self, &COLS_300INDEX).await
    }

    /// 中证1000股指期权波动率指数 QVIX (daily).
    pub async fn index_option_1000index_qvix(&self) -> Result<Vec<QvixDailyPoint>> {
        fetch_qvix_daily(self, &COLS_1000INDEX).await
    }

    /// 上证50股指期权波动率指数 QVIX (daily).
    pub async fn index_option_50index_qvix(&self) -> Result<Vec<QvixDailyPoint>> {
        fetch_qvix_daily(self, &COLS_50INDEX).await
    }

    // -- Intraday (9) --

    /// 50ETF 期权波动率指数 QVIX (intraday).
    pub async fn index_option_50etf_min_qvix(&self) -> Result<Vec<QvixMinPoint>> {
        fetch_qvix_min(self, "http://1.optbbs.com/d/csv/d/vix50.csv").await
    }

    /// 300ETF 期权波动率指数 QVIX (intraday).
    pub async fn index_option_300etf_min_qvix(&self) -> Result<Vec<QvixMinPoint>> {
        fetch_qvix_min(self, "http://1.optbbs.com/d/csv/d/vix300.csv").await
    }

    /// 500ETF 期权波动率指数 QVIX (intraday).
    pub async fn index_option_500etf_min_qvix(&self) -> Result<Vec<QvixMinPoint>> {
        fetch_qvix_min(self, "http://1.optbbs.com/d/csv/d/vix500.csv").await
    }

    /// 创业板期权波动率指数 QVIX (intraday).
    pub async fn index_option_cyb_min_qvix(&self) -> Result<Vec<QvixMinPoint>> {
        fetch_qvix_min(self, "http://1.optbbs.com/d/csv/d/vixcyb.csv").await
    }

    /// 科创板期权波动率指数 QVIX (intraday).
    pub async fn index_option_kcb_min_qvix(&self) -> Result<Vec<QvixMinPoint>> {
        fetch_qvix_min(self, "http://1.optbbs.com/d/csv/d/vixkcb.csv").await
    }

    /// 深证100ETF 期权波动率指数 QVIX (intraday).
    pub async fn index_option_100etf_min_qvix(&self) -> Result<Vec<QvixMinPoint>> {
        fetch_qvix_min(self, "http://1.optbbs.com/d/csv/d/vix100.csv").await
    }

    /// 中证300股指期权波动率指数 QVIX (intraday).
    pub async fn index_option_300index_min_qvix(&self) -> Result<Vec<QvixMinPoint>> {
        fetch_qvix_min(self, "http://1.optbbs.com/d/csv/d/vixindex.csv").await
    }

    /// 中证1000股指期权波动率指数 QVIX (intraday).
    pub async fn index_option_1000index_min_qvix(&self) -> Result<Vec<QvixMinPoint>> {
        fetch_qvix_min(self, "http://1.optbbs.com/d/csv/d/vixindex1000.csv").await
    }

    /// 上证50股指期权波动率指数 QVIX (intraday).
    pub async fn index_option_50index_min_qvix(&self) -> Result<Vec<QvixMinPoint>> {
        fetch_qvix_min(self, "http://1.optbbs.com/d/csv/d/vix50index.csv").await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_qvix_col_indices() {
        // Verify column slices are sensible.
        assert_eq!(super::COLS_50ETF, [1, 2, 3, 4]);
        assert_eq!(super::COLS_300ETF, [9, 10, 11, 12]);
        assert_eq!(super::COLS_KCB, [83, 84, 85, 86]);
    }
}
