//! Bond issuance data from CNINFO (巨潮资讯).
//!
//! Note: The Python akshare implementation uses JavaScript evaluation (py_mini_racer)
//! to generate authentication codes for CNINFO APIs. Since we cannot evaluate JS in Rust,
//! these functions attempt direct API calls which may require additional auth headers.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

#[derive(Debug, Deserialize)]
struct CninfoResp {
    records: Option<Vec<serde_json::Value>>,
}

/// CNINFO API endpoint mapping for different bond issuance types.
const CNINFO_ENDPOINTS: &[(&str, &str)] = &[
    (
        "treasure",
        "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1120",
    ),
    (
        "local_government",
        "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1121",
    ),
    (
        "corporate",
        "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1122",
    ),
    (
        "convertible",
        "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1123",
    ),
    (
        "convertible_stock",
        "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1124",
    ),
];

impl AkShareClient {
    /// Fetch government bond issuance data from CNINFO.
    ///
    /// `start_date` and `end_date` are YYYYMMDD strings.
    pub async fn bond_treasure_issue(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.fetch_cninfo_issue("treasure", start_date, end_date)
            .await
    }

    /// Fetch local government bond issuance data from CNINFO.
    pub async fn bond_local_gov_issue(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.fetch_cninfo_issue("local_government", start_date, end_date)
            .await
    }

    /// Fetch corporate bond issuance data from CNINFO.
    pub async fn bond_corporate_issue(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.fetch_cninfo_issue("corporate", start_date, end_date)
            .await
    }

    /// Fetch convertible bond issuance data from CNINFO.
    pub async fn bond_cov_issue(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.fetch_cninfo_issue("convertible", start_date, end_date)
            .await
    }

    /// Fetch local government bond issuance data from CNINFO (exact Python name).
    pub async fn bond_local_government_issue(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.bond_local_gov_issue(start_date, end_date).await
    }

    /// Fetch convertible bond stock conversion data from CNINFO.
    pub async fn bond_cov_stock_issue(&self) -> Result<Vec<serde_json::Value>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1124";
        let resp: CninfoResp = self.post(url).send().await?.json().await?;

        let records = resp.records.unwrap_or_default();
        if records.is_empty() {
            return Err(Error::not_found(
                "cninfo returned no convertible bond stock conversion data",
            ));
        }
        Ok(records)
    }

    async fn fetch_cninfo_issue(
        &self,
        bond_type: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let url = CNINFO_ENDPOINTS
            .iter()
            .find(|(t, _)| *t == bond_type)
            .map(|(_, u)| *u)
            .ok_or_else(|| Error::invalid_input(format!("unknown bond type: {bond_type}")))?;

        let sd = format!(
            "{}-{}-{}",
            &start_date[..4],
            &start_date[4..6],
            &start_date[6..8]
        );
        let ed = format!("{}-{}-{}", &end_date[..4], &end_date[4..6], &end_date[6..8]);

        let resp: CninfoResp = self
            .post(url)
            .query(&[("sdate", sd.as_str()), ("edate", ed.as_str())])
            .send()
            .await?
            .json()
            .await?;

        let records = resp.records.unwrap_or_default();
        if records.is_empty() {
            return Err(Error::not_found(format!(
                "cninfo returned no {bond_type} bond issuance data"
            )));
        }
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cninfo_endpoints() {
        assert_eq!(CNINFO_ENDPOINTS.len(), 5);
        assert!(CNINFO_ENDPOINTS.iter().any(|(t, _)| *t == "treasure"));
        assert!(CNINFO_ENDPOINTS.iter().any(|(t, _)| *t == "convertible"));
    }
}
