//! Private fund (私募基金) data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::wire::EmDatacenterResp;

impl AkShareClient {
    /// Fetch private fund NAV data from Eastmoney.
    ///
    /// Returns current NAV (单位净值) and accumulated NAV (累计净值) for private funds.
    pub async fn fund_private_fund_nav_em(&self) -> Result<Vec<serde_json::Value>> {
        let mut all_items = Vec::new();

        for page in 1..=5 {
            let resp: EmDatacenterResp = self
                .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("reportName", "RPT_PRIVATE_FUND_NAV"),
                    ("columns", "ALL"),
                    ("sortColumns", "NAV_DATE"),
                    ("sortTypes", "-1"),
                    ("pageSize", "500"),
                    ("pageNumber", &page.to_string()),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ])
                .send()
                .await?
                .json()
                .await?;

            if let Some(msg) = resp.check_error("RPT_PRIVATE_FUND_NAV") {
                return Err(Error::upstream(msg));
            }
            let data = resp.result.map(|r| r.data).unwrap_or_default();
            if data.is_empty() {
                break;
            }
            all_items.extend(data);
        }

        if all_items.is_empty() {
            return Err(Error::not_found("no private fund NAV data available"));
        }
        Ok(all_items)
    }

    /// Fetch private fund ranking data from Eastmoney.
    ///
    /// Returns ranked list of private funds sorted by performance.
    pub async fn fund_private_fund_rank_em(&self) -> Result<Vec<serde_json::Value>> {
        let resp: EmDatacenterResp = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_PRIVATE_FUND_RANK"),
                ("columns", "ALL"),
                ("sortColumns", "TOTAL_RETURN"),
                ("sortTypes", "-1"),
                ("pageSize", "500"),
                ("pageNumber", "1"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        if let Some(msg) = resp.check_error("RPT_PRIVATE_FUND_RANK") {
            return Err(Error::upstream(msg));
        }
        let data = resp.result.map(|r| r.data).unwrap_or_default();

        if data.is_empty() {
            return Err(Error::not_found("no private fund ranking data available"));
        }
        Ok(data)
    }

    /// Fetch private fund manager information from Eastmoney.
    ///
    /// Returns manager details including AUM, fund count, and performance metrics.
    pub async fn fund_private_fund_manager_em(&self) -> Result<Vec<serde_json::Value>> {
        let resp: EmDatacenterResp = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_PRIVATE_FUND_MANAGER"),
                ("columns", "ALL"),
                ("sortColumns", "MANAGER_SCALE"),
                ("sortTypes", "-1"),
                ("pageSize", "500"),
                ("pageNumber", "1"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        if let Some(msg) = resp.check_error("RPT_PRIVATE_FUND_MANAGER") {
            return Err(Error::upstream(msg));
        }
        let data = resp.result.map(|r| r.data).unwrap_or_default();

        if data.is_empty() {
            return Err(Error::not_found("no private fund manager data available"));
        }
        Ok(data)
    }
}
