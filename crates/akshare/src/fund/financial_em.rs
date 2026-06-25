//! Financial fund data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundNavHistory;

impl AkShareClient {
    /// Fetch financial fund daily data (Python: fund_financial_fund_daily).
    pub async fn fund_financial_fund_daily(&self) -> Result<Vec<serde_json::Value>> {
        let response = crate::util::send_and_check(
            self.get("https://api.fund.eastmoney.com/FundNetValue/GetLCJJJZ")
                .header("Referer", "https://fund.eastmoney.com/lcjj.html")
                .query(&[
                ("letter", ""),
                ("jjgsid", "0"),
                ("searchtext", ""),
                ("sort", "ljjz,desc"),
                ("page", "1,100"),
                ("AttentionCodes", ""),
                ("cycle", ""),
                ("OnlySale", "1"),
                ])
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let list = payload
            .get("Data")
            .and_then(|d| d.get("List"))
            .and_then(|l| l.as_array())
            .ok_or_else(|| Error::not_found("no financial fund data"))?;

        if list.is_empty() {
            return Err(Error::not_found("no financial fund data"));
        }
        Ok(list.clone())
    }

    /// Fetch financial fund info (historical NAV) (Python: fund_financial_fund_info).
    ///
    /// `symbol`: financial fund code (e.g. "000134").
    pub async fn fund_financial_fund_info(&self, symbol: &str) -> Result<Vec<FundNavHistory>> {
        let response = crate::util::send_and_check(
            self.get("https://api.fund.eastmoney.com/f10/lsjz")
                .query(&[
                ("fundCode", symbol),
                ("pageIndex", "1"),
                ("pageSize", "10000"),
                ("startDate", ""),
                ("endDate", ""),
                ])
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let list = payload
            .get("Data")
            .and_then(|d| d.get("LSJZList"))
            .and_then(|l| l.as_array())
            .ok_or_else(|| Error::not_found(format!("no financial fund data for {symbol}")))?;

        let mut result = Vec::new();
        for item in list {
            let Some(arr) = item.as_array() else { continue };
            if arr.len() < 10 {
                continue;
            }
            result.push(FundNavHistory {
                date: arr[0].as_str().unwrap_or("").to_string(),
                nav: arr[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                acc_nav: arr[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                change_pct: arr[6].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                subscribe_status: arr[7].as_str().unwrap_or("").to_string(),
                redeem_status: arr[8].as_str().unwrap_or("").to_string(),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found(format!(
                "no financial fund data for {symbol}"
            )));
        }
        result.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(result)
    }
}
