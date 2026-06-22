//! Trade calendar data from Sina Finance.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Historical trade calendar from Sina Finance.
    ///
    /// Downloads the A-share trading calendar from Sina Finance.
    /// Returns a list of trading dates.
    pub async fn tool_trade_date_hist(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://finance.sina.com.cn/realstock/company/klc_td_sh.txt";
        let body = self.get(url).send().await?.text().await?;

        // The response format is: var klc_td_sh="date1,date2,...";
        let dates_str = body
            .split_once('=')
            .and_then(|(_, r)| r.split_once(';'))
            .map_or("", |(s, _)| s.trim_matches('"'));

        if dates_str.is_empty() {
            return Err(Error::decode("sina trade date: empty response"));
        }

        let mut items = Vec::new();
        for date in dates_str.split(',') {
            let date = date.trim();
            if !date.is_empty() {
                items.push(MacroDataPoint {
                    date: date.to_string(),
                    value: 1.0, // 1 = trading day
                    name: "Trade Date".to_string(),
                });
            }
        }

        // Add known missing date
        items.push(MacroDataPoint {
            date: "1992-05-04".to_string(),
            value: 1.0,
            name: "Trade Date".to_string(),
        });

        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trade_date_parsing() {
        // Simulate Sina response format
        let body = r#"var klc_td_sh="2024-01-02,2024-01-03,2024-01-04";"#;
        let dates_str = body
            .split_once('=')
            .and_then(|(_, r)| r.split_once(';'))
            .map_or("", |(s, _)| s.trim_matches('"'));
        let dates: Vec<&str> = dates_str.split(',').collect();
        assert_eq!(dates.len(), 3);
        assert_eq!(dates[0], "2024-01-02");
    }
}
