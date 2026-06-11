//! Fund rating data from various sources.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundRatingItem;

impl AkShareClient {
    /// Fetch fund ratings from Eastmoney.
    pub async fn fund_rating_em(&self, limit: usize) -> Result<Vec<serde_json::Value>> {
        let pn = limit.max(1).to_string();
        let resp = self
            .get("https://api.fund.eastmoney.com/FundRating/GetFundRatingList")
            .header("Referer", "https://fund.eastmoney.com/")
            .query(&[
                ("FundType", "0"), ("SortColumn", "SYL_1N"), ("Sort", "desc"),
                ("pageIndex", "1"), ("pageSize", pn.as_str()),
            ])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;

        let root: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let items = root.get("datas").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        if items.is_empty() {
            return Err(Error::not_found("no fund rating data"));
        }
        Ok(items.into_iter().take(limit).collect())
    }

    /// Fetch fund ratings from Zhaoshang.
    pub async fn fund_rating_zs(&self) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode("Zhaoshang fund rating requires HTML parsing"))
    }

    /// Fetch fund ratings from Tiantian.
    pub async fn fund_rating_tiantian(&self) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode("Tiantian fund rating requires HTML parsing"))
    }

    /// Fetch fund ratings from JiShi.
    pub async fn fund_rating_jiashi(&self) -> Result<Vec<serde_json::Value>> {
        Err(Error::decode("JiShi fund rating requires HTML parsing"))
    }

    /// Fetch all fund ratings summary (Python: fund_rating_all).
    pub async fn fund_rating_all(&self) -> Result<Vec<FundRatingItem>> {
        let response = self
            .get("https://fund.eastmoney.com/data/fundrating.html")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        // This returns HTML with embedded JS data; requires HTML parsing.
        if text.is_empty() {
            return Err(Error::not_found("no fund rating data"));
        }
        Err(Error::decode("fund_rating_all requires HTML/JS parsing"))
    }

    /// Fetch Shanghai Securities fund ratings (Python: fund_rating_sh).
    pub async fn fund_rating_sh(&self, _date: &str) -> Result<Vec<FundRatingItem>> {
        Err(Error::decode("fund_rating_sh requires HTML/JS parsing"))
    }

    /// Fetch Ji'an Jinxin fund ratings (Python: fund_rating_ja).
    pub async fn fund_rating_ja(&self, _date: &str) -> Result<Vec<FundRatingItem>> {
        Err(Error::decode("fund_rating_ja requires HTML/JS parsing"))
    }
}
