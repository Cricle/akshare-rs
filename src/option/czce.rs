//! CZCE yearly option history.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// CZCE option start year mapping
// ---------------------------------------------------------------------------

fn czce_symbol_start_year(symbol: &str) -> Option<&'static str> {
    match symbol {
        "SR" => Some("2017"),
        "CF" => Some("2019"),
        "TA" => Some("2019"),
        "MA" => Some("2019"),
        "RM" => Some("2020"),
        "ZC" => Some("2020"),
        "OI" => Some("2022"),
        "PK" => Some("2022"),
        "PX" => Some("2023"),
        "SH" => Some("2023"),
        "SA" => Some("2023"),
        "PF" => Some("2023"),
        "SM" => Some("2023"),
        "SF" => Some("2023"),
        "UR" => Some("2023"),
        "AP" => Some("2023"),
        "CJ" => Some("2024"),
        "FG" => Some("2024"),
        "PR" => Some("2024"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// CZCE yearly option history row (flexible - raw pipe-delimited data).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionHistYearlyCzceRow {
    /// Raw field values from the pipe-delimited text file.
    pub fields: Vec<String>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// CZCE yearly option history download.
    ///
    /// `symbol` is the product code, e.g. "SR", "CF", "TA", "MA", "RM", etc.
    /// `year` is the year string, e.g. "2021".
    pub async fn option_hist_yearly_czce(
        &self,
        symbol: &str,
        year: &str,
    ) -> Result<Vec<OptionHistYearlyCzceRow>> {
        let start_year = czce_symbol_start_year(symbol)
            .ok_or_else(|| Error::invalid_input(format!("unsupported CZCE symbol: {symbol}")))?;

        if year < start_year {
            return Err(Error::invalid_input(format!(
                "{symbol} started trading in {start_year}, cannot get data for {year}"
            )));
        }

        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Option/{year}/OptionDataAllHistory/{symbol}OPTIONS{year}.txt"
        );

        let body = self
                        .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        let lines: Vec<&str> = body.lines().collect();
        if lines.len() < 2 {
            return Ok(vec![]);
        }

        // Skip header line (first line)
        let mut rows = Vec::new();
        for line in lines.iter().skip(1) {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("----") {
                continue;
            }
            let fields: Vec<String> = trimmed
                .split('|')
                .map(|s| s.trim().to_string())
                .collect();
            if fields.len() < 2 {
                continue;
            }
            rows.push(OptionHistYearlyCzceRow { fields });
        }

        Ok(rows)
    }
}
