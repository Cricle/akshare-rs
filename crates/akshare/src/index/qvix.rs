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
    let body = crate::util::send_and_check(
        client.get("http://1.optbbs.com/d/csv/d/k.csv"),
    )
    .await?
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
async fn fetch_qvix_min(client: &AkShareClient, url: &str) -> Result<Vec<QvixMinPoint>> {
    let body = crate::util::send_and_check(client.get(url))
        .await?
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
// Public API — macro-generated (18 functions: 9 daily + 9 intraday)
// ---------------------------------------------------------------------------

macro_rules! qvix_daily {
    ($(#[$meta:meta])* $name:ident, $cols:expr) => {
        $(#[$meta])*
        pub async fn $name(&self) -> Result<Vec<QvixDailyPoint>> {
            fetch_qvix_daily(self, $cols).await
        }
    };
}

macro_rules! qvix_min {
    ($(#[$meta:meta])* $name:ident, $url:expr) => {
        $(#[$meta])*
        pub async fn $name(&self) -> Result<Vec<QvixMinPoint>> {
            fetch_qvix_min(self, $url).await
        }
    };
}

impl AkShareClient {
    // -- Daily (9) --

    qvix_daily!(/// 50ETF 期权波动率指数 QVIX (daily).
        index_option_50etf_qvix, &COLS_50ETF);
    qvix_daily!(/// 300ETF 期权波动率指数 QVIX (daily).
        index_option_300etf_qvix, &COLS_300ETF);
    qvix_daily!(/// 500ETF 期权波动率指数 QVIX (daily).
        index_option_500etf_qvix, &COLS_500ETF);
    qvix_daily!(/// 创业板期权波动率指数 QVIX (daily).
        index_option_cyb_qvix, &COLS_CYB);
    qvix_daily!(/// 科创板期权波动率指数 QVIX (daily).
        index_option_kcb_qvix, &COLS_KCB);
    qvix_daily!(/// 深证100ETF 期权波动率指数 QVIX (daily).
        index_option_100etf_qvix, &COLS_100ETF);
    qvix_daily!(/// 中证300股指期权波动率指数 QVIX (daily).
        index_option_300index_qvix, &COLS_300INDEX);
    qvix_daily!(/// 中证1000股指期权波动率指数 QVIX (daily).
        index_option_1000index_qvix, &COLS_1000INDEX);
    qvix_daily!(/// 上证50股指期权波动率指数 QVIX (daily).
        index_option_50index_qvix, &COLS_50INDEX);

    // -- Intraday (9) --

    qvix_min!(/// 50ETF 期权波动率指数 QVIX (intraday).
        index_option_50etf_min_qvix, "http://1.optbbs.com/d/csv/d/vix50.csv");
    qvix_min!(/// 300ETF 期权波动率指数 QVIX (intraday).
        index_option_300etf_min_qvix, "http://1.optbbs.com/d/csv/d/vix300.csv");
    qvix_min!(/// 500ETF 期权波动率指数 QVIX (intraday).
        index_option_500etf_min_qvix, "http://1.optbbs.com/d/csv/d/vix500.csv");
    qvix_min!(/// 创业板期权波动率指数 QVIX (intraday).
        index_option_cyb_min_qvix, "http://1.optbbs.com/d/csv/d/vixcyb.csv");
    qvix_min!(/// 科创板期权波动率指数 QVIX (intraday).
        index_option_kcb_min_qvix, "http://1.optbbs.com/d/csv/d/vixkcb.csv");
    qvix_min!(/// 深证100ETF 期权波动率指数 QVIX (intraday).
        index_option_100etf_min_qvix, "http://1.optbbs.com/d/csv/d/vix100.csv");
    qvix_min!(/// 中证300股指期权波动率指数 QVIX (intraday).
        index_option_300index_min_qvix, "http://1.optbbs.com/d/csv/d/vixindex.csv");
    qvix_min!(/// 中证1000股指期权波动率指数 QVIX (intraday).
        index_option_1000index_min_qvix, "http://1.optbbs.com/d/csv/d/vixindex1000.csv");
    qvix_min!(/// 上证50股指期权波动率指数 QVIX (intraday).
        index_option_50index_min_qvix, "http://1.optbbs.com/d/csv/d/vix50index.csv");
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
