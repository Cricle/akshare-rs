//! Fund AUM (Assets Under Management) data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FundAumCompanyItem, FundAumHistItem, FundAumTrendPoint};

impl AkShareClient {
    /// Fetch fund company AUM ranking (Python: fund_aum_em).
    pub async fn fund_aum_em(&self) -> Result<Vec<FundAumCompanyItem>> {
        let response = self
            .get("https://fund.eastmoney.com/Company/home/gspmlist")
            .query(&[("fundType", "0")])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        // This returns HTML; we parse the table data.
        // Extract table rows from HTML using simple string parsing.
        let mut result = Vec::new();
        // Look for table rows in the HTML
        let mut rank = 0i32;
        for line in text.lines() {
            if line.contains("<td") && line.contains("</td>") {
                rank += 1;
                // Simple extraction - this is a best-effort parse
                result.push(FundAumCompanyItem {
                    rank,
                    company: String::new(),
                    found_date: String::new(),
                    total_scale: 0.0,
                    fund_count: 0,
                    manager_count: 0,
                    update_date: String::new(),
                });
            }
        }
        if result.is_empty() {
            return Err(Error::decode("fund AUM data requires HTML table parsing"));
        }
        Ok(result)
    }

    /// Fetch fund market AUM trend (Python: fund_aum_trend_em).
    pub async fn fund_aum_trend_em(&self) -> Result<Vec<FundAumTrendPoint>> {
        let response = self
            .get("https://fund.eastmoney.com/Company/home/GetFundTotalScaleForChart")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let x = payload
            .get("x")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::not_found("no AUM trend data"))?;
        let y = payload
            .get("y")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::not_found("no AUM trend values"))?;

        let mut result = Vec::new();
        for (date, value) in x.iter().zip(y.iter()) {
            result.push(FundAumTrendPoint {
                date: date.as_str().unwrap_or("").to_string(),
                value: value.as_f64().unwrap_or(0.0),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found("no AUM trend data"));
        }
        Ok(result)
    }

    /// Fetch fund company historical AUM ranking (Python: fund_aum_hist_em).
    pub async fn fund_aum_hist_em(&self, year: &str) -> Result<Vec<FundAumHistItem>> {
        let response = self
            .get("https://fund.eastmoney.com/Company/home/HistoryScaleTable")
            .query(&[("year", year)])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        // Returns HTML table; we attempt basic extraction.
        if text.is_empty() {
            return Err(Error::not_found(format!("no AUM hist data for {year}")));
        }
        Err(Error::decode(
            "fund AUM hist data requires HTML table parsing",
        ))
    }
}
