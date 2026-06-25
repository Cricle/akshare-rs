//! Fund dividend and split data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FundDividendItem, FundDividendRankItem};

impl AkShareClient {
    /// Fetch fund dividend data (Python: fund_fh).
    ///
    /// `year`: query year (e.g. "2025").
    pub async fn fund_fh(&self, year: &str) -> Result<Vec<FundDividendItem>> {
        let response = crate::util::send_and_check(
            self.get("https://fund.eastmoney.com/Data/funddataIndex_Interface.aspx")
                .query(&[
                ("dt", "8"),
                ("page", "1"),
                ("rank", "BZDM"),
                ("sort", "asc"),
                ("gs", ""),
                ("ftype", ""),
                ("year", year),
                ])
        )
        .await?;

        let text = response.text().await.map_err(Error::from)?;
        let start = text.find("[[").unwrap_or(0);
        let end_marker = text.find(";var jjfh_jjgs").unwrap_or(text.len());
        if start >= end_marker {
            return Ok(Vec::new());
        }
        let json_str = &text[start..end_marker];

        let data: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("fund dividend JSON parse: {e}")))?;

        let mut result = Vec::new();
        for (i, row) in data.iter().enumerate() {
            if row.len() < 7 {
                continue;
            }
            result.push(FundDividendItem {
                rank: (i + 1) as i32,
                fund_code: row[0].as_str().unwrap_or("").to_string(),
                fund_name: row[1].as_str().unwrap_or("").to_string(),
                record_date: row[2].as_str().unwrap_or("").to_string(),
                ex_date: row[3].as_str().unwrap_or("").to_string(),
                dividend: row[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                pay_date: row[5].as_str().unwrap_or("").to_string(),
            });
        }
        Ok(result)
    }

    /// Fetch fund dividend ranking (Python: fund_fh_rank).
    pub async fn fund_fh_rank(&self) -> Result<Vec<FundDividendRankItem>> {
        let response = crate::util::send_and_check(
            self.get("https://fund.eastmoney.com/Data/funddataIndex_Interface.aspx")
                .query(&[
                ("dt", "10"),
                ("page", "1"),
                ("rank", "FHFCZ"),
                ("sort", "desc"),
                ("gs", ""),
                ("ftype", ""),
                ])
        )
        .await?;

        let text = response.text().await.map_err(Error::from)?;
        let start = text.find("[[").unwrap_or(0);
        let end_marker = text.find(";var fhph_jjgs").unwrap_or(text.len());
        if start >= end_marker {
            return Ok(Vec::new());
        }
        let json_str = &text[start..end_marker];

        let data: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("fund dividend rank JSON parse: {e}")))?;

        let mut result = Vec::new();
        for (i, row) in data.iter().enumerate() {
            if row.len() < 6 {
                continue;
            }
            result.push(FundDividendRankItem {
                rank: (i + 1) as i32,
                fund_code: row[0].as_str().unwrap_or("").to_string(),
                fund_name: row[1].as_str().unwrap_or("").to_string(),
                cumulative_dividend: row[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                cumulative_count: row[3].as_str().unwrap_or("0").parse().unwrap_or(0),
                found_date: row[4].as_str().unwrap_or("").to_string(),
            });
        }
        Ok(result)
    }
}
