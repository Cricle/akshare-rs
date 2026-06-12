//! High-frequency S&P 500 data from GitHub.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// High-frequency S&P 500 minute data.
    ///
    /// Downloads minute-level S&P 500 data from a GitHub repository.
    /// `year` ranges from "2012" to "2018".
    pub async fn hf_sp_500(&self, year: &str) -> Result<Vec<MacroDataPoint>> {
        let url = format!(
            "https://github.com/FutureSharks/financial-data/raw/master/pyfinancialdata/data/stocks/histdata/SPXUSD/DAT_ASCII_SPXUSD_M1_{year}.csv"
        );

        let body = self
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let mut items = Vec::new();
        for line in body.lines() {
            let fields: Vec<&str> = line.split(';').collect();
            if fields.len() < 6 {
                continue;
            }
            let date = fields[0].trim().to_string();
            if date.is_empty() {
                continue;
            }
            let close: f64 = fields[4].trim().parse().unwrap_or(0.0);
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value: close,
                name: "SPX".to_string(),
            });
        }

        if items.is_empty() {
            return Err(Error::not_found(format!(
                "hf sp500: no data for year {year}"
            )));
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hf_sp500_csv_parsing() {
        // Simulate a semicolon-separated line
        let line = "2017-01-02 09:31:00;2251.50;2252.00;2251.00;2251.75;1000";
        let fields: Vec<&str> = line.split(';').collect();
        assert_eq!(fields.len(), 6);
        let close: f64 = fields[4].parse().unwrap();
        assert!((close - 2251.75).abs() < 0.01);
    }
}
