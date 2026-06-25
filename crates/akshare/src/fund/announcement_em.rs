//! Fund announcements from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundAnnouncementItem;

impl AkShareClient {
    /// Fetch fund announcements from Eastmoney.
    ///
    /// `symbol`: fund code.
    /// `ann_type`: "2" (dividend), "3" (report), "4" (personnel).
    async fn fund_announcement_em_inner(
        &self,
        symbol: &str,
        ann_type: &str,
    ) -> Result<Vec<FundAnnouncementItem>> {
        let ts = chrono::Utc::now().timestamp_millis().to_string();
        let referer = format!("http://fundf10.eastmoney.com/jjgg_{symbol}_{ann_type}.html");
        let response = crate::util::send_and_check(
            self.get("http://api.fund.eastmoney.com/f10/JJGG")
                .header("Referer", &referer)
                .query(&[
                ("fundcode", symbol),
                ("pageIndex", "1"),
                ("pageSize", "1000"),
                ("type", ann_type),
                ("_", ts.as_str()),
                ])
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("Data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| Error::not_found(format!("no announcement data for {symbol}")))?;

        let mut result = Vec::new();
        for item in data {
            let Some(arr) = item.as_array() else { continue };
            if arr.len() < 8 {
                continue;
            }
            result.push(FundAnnouncementItem {
                fund_code: arr[0].as_str().unwrap_or("").to_string(),
                title: arr[1].as_str().unwrap_or("").to_string(),
                fund_name: arr[2].as_str().unwrap_or("").to_string(),
                date: arr[5].as_str().unwrap_or("").to_string(),
                report_id: arr[7].as_str().unwrap_or("").to_string(),
            });
        }
        result.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(result)
    }

    /// Fetch fund dividend announcements (Python: fund_announcement_dividend).
    pub async fn fund_announcement_dividend(
        &self,
        symbol: &str,
    ) -> Result<Vec<FundAnnouncementItem>> {
        self.fund_announcement_em_inner(symbol, "2").await
    }

    /// Fetch fund report announcements (Python: fund_announcement_report).
    pub async fn fund_announcement_report(
        &self,
        symbol: &str,
    ) -> Result<Vec<FundAnnouncementItem>> {
        self.fund_announcement_em_inner(symbol, "3").await
    }

    /// Fetch fund personnel announcements (Python: fund_announcement_personnel).
    pub async fn fund_announcement_personnel(
        &self,
        symbol: &str,
    ) -> Result<Vec<FundAnnouncementItem>> {
        self.fund_announcement_em_inner(symbol, "4").await
    }
}
