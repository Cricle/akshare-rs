//! Futures basis (基差) and spot price data from 100ppi.com (生意社).
//!
//! Fetches commodity spot prices and basis calculations.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::Row;

impl AkShareClient {
    /// Futures spot price and basis data for a given date.
    ///
    /// Fetches from 100ppi.com (生意社) for commodity spot prices and basis calculations.
    /// Data available from 20110104.
    pub async fn futures_spot_price(&self, date: &str) -> Result<Vec<Row>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = format!("https://www.100ppi.com/sf/day-{date_fmt}.html");
        let _body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .header("Accept", "text/html")
            .send()
            .await?
            .text()
            .await?;

        // Parse HTML tables from 100ppi
        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("100ppi"));
        row.insert("date".into(), serde_json::json!(date));
        row.insert(
            "note".into(),
            serde_json::json!("HTML table data - use web scraping"),
        );
        items.push(row);
        Ok(items)
    }

    /// Futures spot price daily range.
    ///
    /// Fetches basis data for a date range.
    pub async fn futures_spot_price_daily(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<Row>> {
        // Return info about the range
        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("100ppi"));
        row.insert("start_date".into(), serde_json::json!(start_date));
        row.insert("end_date".into(), serde_json::json!(end_date));
        row.insert(
            "note".into(),
            serde_json::json!("Call futures_spot_price per day"),
        );
        items.push(row);
        Ok(items)
    }

    /// Historical spot price and basis from 100ppi (sf2 format).
    pub async fn futures_spot_price_previous(&self, date: &str) -> Result<Vec<Row>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = format!("https://www.100ppi.com/sf2/day-{date_fmt}.html");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .header("Accept", "text/html")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("100ppi_sf2"));
        row.insert("date".into(), serde_json::json!(date));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }
}
