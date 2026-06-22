//! Academic article and research data: EPU index, FRED-MD/QD, FF factors, realized volatility.
//!
//! Sources:
//! - EPU: Economic Policy Uncertainty (policyuncertainty.com)
//! - FRED: Federal Reserve Economic Data (fred.stlouisfed.org)
//! - FF: Fama-French Data Library
//! - RV: Oxford-Man Realized Library; Risk Lab (dachxiu.chicagobooth.edu)

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Economic Policy Uncertainty (EPU) index for a given country/region.
    ///
    /// Downloads the CSV from `policyuncertainty.com` for the given symbol
    /// (e.g., "China", "USA", "Europe", "Hong Kong", "South Korea").
    pub async fn article_epu_index(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let file_name = match symbol {
            "China" | "China New" => "SCMP_China",
            "USA" => "US",
            "Hong Kong" => "HK",
            "Germany" | "France" | "Italy" => "Europe",
            "South Korea" => "Korea",
            "Spain New" => "Spain",
            _ => symbol,
        };

        let url = format!(
            "http://www.policyuncertainty.com/media/{file_name}_Policy_Uncertainty_Data.csv"
        );

        let body = self.get(&url).send().await?.text().await?;
        let mut items = Vec::new();
        for (idx, line) in body.lines().enumerate() {
            if idx == 0 {
                continue; // skip header
            }
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 2 {
                continue;
            }
            let date = fields[0].trim().to_string();
            let value: f64 = fields
                .last()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(0.0);
            if date.is_empty() {
                continue;
            }
            items.push(MacroDataPoint {
                date,
                value,
                name: format!("EPU - {symbol}"),
            });
        }
        Ok(items)
    }

    /// FRED-MD monthly macroeconomic dataset.
    ///
    /// Downloads from the FRED-MD S3 bucket for the given date (e.g., "2023-03").
    /// Returns raw CSV rows as `MacroDataPoint` values.
    pub async fn fred_md(&self, date: &str) -> Result<Vec<MacroDataPoint>> {
        let url = format!(
            "https://s3.amazonaws.com/files.fred.stlouisfed.org/fred-md/monthly/{date}.csv"
        );
        let body = self.get(&url).send().await?.text().await?;
        Ok(parse_fred_csv(&body, &format!("FRED-MD {date}")))
    }

    /// FRED-QD quarterly macroeconomic dataset.
    ///
    /// Downloads from the FRED-QD S3 bucket for the given date (e.g., "2023-03").
    pub async fn fred_qd(&self, date: &str) -> Result<Vec<MacroDataPoint>> {
        let url = format!(
            "https://s3.amazonaws.com/files.fred.stlouisfed.org/fred-md/quarterly/{date}.csv"
        );
        let body = self.get(&url).send().await?.text().await?;
        Ok(parse_fred_csv(&body, &format!("FRED-QD {date}")))
    }

    /// Oxford-Man Institute Realized Library - full realized volatility data.
    ///
    /// Fetches the complete realized library dataset and extracts the specified
    /// index for the given symbol (e.g., "FTSE", "SPX", "SSEC").
    ///
    /// Available indices: medrv, rk_twoscale, bv, rv10, rv5, rk_th2, rv10_ss,
    /// rsv, rv5_ss, bv_ss, rk_parzen, rsv_ss.
    pub async fn article_oman_rv(&self, symbol: &str, index: &str) -> Result<Vec<MacroDataPoint>> {
        let url =
            "https://realized.oxford-man.ox.ac.uk/theme/js/visualization-data.js?20191111113154";
        let body = self.get(url).send().await?.text().await?;

        // Extract JSON from the JS file content
        let json_start = body
            .find('{')
            .ok_or_else(|| Error::decode("oman rv: JSON start not found"))?;
        let json_end = body
            .rfind('}')
            .ok_or_else(|| Error::decode("oman rv: JSON end not found"))?;
        let json_str = &body[json_start..=json_end];

        let data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("oman rv JSON: {e}")))?;

        let key = format!(".{symbol}");
        let dates = data
            .get(&key)
            .and_then(|v| v.get("dates"))
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::not_found(format!("oman rv: symbol {symbol} not found")))?;

        let values = data
            .get(&key)
            .and_then(|v| v.get(index))
            .and_then(|v| v.get("data"))
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::not_found(format!("oman rv: index {index} not found")))?;

        let mut items = Vec::new();
        for (d, v) in dates.iter().zip(values.iter()) {
            let ts_ms = d.as_i64().unwrap_or(0);
            let date = chrono::DateTime::from_timestamp_millis(ts_ms)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();
            let value = v.as_f64().unwrap_or(0.0);
            items.push(MacroDataPoint {
                date,
                value,
                name: format!("{symbol}-{index}"),
            });
        }
        Ok(items)
    }

    /// Oxford-Man Institute Realized Library - short front-page chart data.
    pub async fn article_oman_rv_short(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let url = "https://realized.oxford-man.ox.ac.uk/theme/js/front-page-chart.js";
        let body = self
            .get(url)
            .header("Referer", "https://realized.oxford-man.ox.ac.uk/")
            .send()
            .await?
            .text()
            .await?;

        let json_start = body
            .find('{')
            .ok_or_else(|| Error::decode("oman rv short: JSON start not found"))?;
        let json_end = body
            .rfind('}')
            .ok_or_else(|| Error::decode("oman rv short: JSON end not found"))?;
        let json_str = &body[json_start..=json_end];

        let data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("oman rv short JSON: {e}")))?;

        let key = format!(".{symbol}");
        let data_arr = data
            .get(&key)
            .and_then(|v| v.get("data"))
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::not_found(format!("oman rv short: symbol {symbol} not found")))?;

        let mut items = Vec::new();
        for entry in data_arr {
            if let Some(arr) = entry.as_array()
                && arr.len() >= 2
            {
                let ts_ms = arr[0].as_i64().unwrap_or(0);
                let date = chrono::DateTime::from_timestamp_millis(ts_ms)
                    .map(|dt| dt.format("%Y-%m-%d").to_string())
                    .unwrap_or_default();
                let value = arr[1].as_f64().unwrap_or(0.0);
                items.push(MacroDataPoint {
                    date,
                    value,
                    name: symbol.to_string(),
                });
            }
        }
        Ok(items)
    }

    /// Fama-French factor data (CRR version).
    ///
    /// Returns the Fama-French 3-factor or 5-factor data from the
    /// Chicago Booth Risk Research Laboratory.
    pub async fn article_ff_crr(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://dachxiu.chicagobooth.edu/FF/F-F_Research_Data_Factors_CSV.zip";
        let body = self.get(url).send().await?.bytes().await?;

        // Attempt to read as text (the zip may fail, so try CSV directly)
        let text = String::from_utf8_lossy(&body);
        let mut items = Vec::new();
        for (idx, line) in text.lines().enumerate() {
            if idx == 0 || line.trim().is_empty() {
                continue;
            }
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 5 {
                continue;
            }
            let date = fields[0].trim().to_string();
            if date.len() != 6 {
                continue;
            }
            let formatted = format!("{}-{}-01", &date[..4], &date[4..]);
            if let Ok(mkt_rf) = fields[1].trim().parse::<f64>() {
                items.push(MacroDataPoint {
                    date: formatted.clone(),
                    value: mkt_rf,
                    name: "Mkt-RF".to_string(),
                });
            }
            if let Ok(smb) = fields[2].trim().parse::<f64>() {
                items.push(MacroDataPoint {
                    date: formatted.clone(),
                    value: smb,
                    name: "SMB".to_string(),
                });
            }
            if let Ok(hml) = fields[3].trim().parse::<f64>() {
                items.push(MacroDataPoint {
                    date: formatted,
                    value: hml,
                    name: "HML".to_string(),
                });
            }
        }
        Ok(items)
    }

    /// Risk Lab realized volatility from Dacheng Xiu's data (Chicago Booth).
    ///
    /// `symbol` is a stock ticker code (e.g., "39693" for AAPL).
    pub async fn article_rlab_rv(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let url = "https://dachxiu.chicagobooth.edu/data.php";
        let body = self
            .get(url)
            .query(&[("ticker", symbol)])
            .send()
            .await?
            .text()
            .await?;

        // Parse the plain-text response: lines of "date value"
        let mut items = Vec::new();
        let lines: Vec<&str> = body.lines().collect();
        // Find data lines (after the header lines)
        let mut in_data = false;
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            // Data lines look like: "  20200102  0.123456"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2
                && let (Ok(date_val), Ok(rv_val)) =
                    (parts[0].parse::<i64>(), parts[1].parse::<f64>())
                && date_val > 19_000_000
            {
                in_data = true;
                let date_str =
                    format!("{}-{}-{}", &parts[0][..4], &parts[0][4..6], &parts[0][6..8]);
                items.push(MacroDataPoint {
                    date: date_str,
                    value: rv_val,
                    name: format!("RLab RV - {symbol}"),
                });
            }
        }

        if items.is_empty() && !in_data {
            return Err(Error::decode("rlab rv: no data parsed from response"));
        }
        Ok(items)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse a FRED CSV file (first column = date, remaining = data columns).
/// Returns one `MacroDataPoint` per row per numeric column.
fn parse_fred_csv(body: &str, name_prefix: &str) -> Vec<MacroDataPoint> {
    let mut items = Vec::new();
    let lines: Vec<&str> = body.lines().collect();
    if lines.is_empty() {
        return items;
    }

    // Parse header to get column names
    let header: Vec<&str> = lines[0].split(',').map(str::trim).collect();
    if header.len() < 2 {
        return items;
    }

    for line in &lines[1..] {
        let fields: Vec<&str> = line.split(',').map(str::trim).collect();
        if fields.len() < 2 {
            continue;
        }
        let date = fields[0].to_string();
        if date.is_empty() {
            continue;
        }
        // For simplicity, sum the first numeric column as a representative value
        for (col_idx, field) in fields.iter().enumerate().skip(1) {
            if col_idx >= header.len() {
                break;
            }
            if let Ok(value) = field.parse::<f64>() {
                items.push(MacroDataPoint {
                    date: date.clone(),
                    value,
                    name: format!("{}:{}", name_prefix, header[col_idx]),
                });
            }
        }
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fred_csv() {
        let csv = "DATE,VALUE1,VALUE2\n2023-01-01,100.5,200.3\n2023-02-01,101.0,201.0\n";
        let items = parse_fred_csv(csv, "test");
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].date, "2023-01-01");
        assert!((items[0].value - 100.5).abs() < 0.01);
    }

    #[test]
    fn test_parse_fred_csv_empty() {
        let items = parse_fred_csv("", "test");
        assert!(items.is_empty());
    }

    #[test]
    fn test_parse_fred_csv_header_only() {
        let items = parse_fred_csv("DATE,VALUE\n", "test");
        assert!(items.is_empty());
    }
}
