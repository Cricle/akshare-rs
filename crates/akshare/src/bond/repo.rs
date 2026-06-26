//! Bond repo and treasury yield data from SSE, SZSE, and Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::wire::EmDatacenterResp;

impl AkShareClient {
    /// SSE repo (质押式回购) data.
    ///
    /// `date` is in YYYYMMDD format.
    pub async fn bond_repo_sse(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let formatted = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        let url = "http://query.sse.com.cn/commonQuery.do";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[
                    ("sqlId", "COMMON_SSEBOND_GP_ZQHGXX_L"),
                    ("isPagination", "true"),
                    ("pageHelp.pageSize", "500"),
                    ("pageHelp.pageNo", "1"),
                    ("pageHelp.beginPage", "1"),
                    ("pageHelp.endPage", "50"),
                    ("TRADE_DATE", formatted.as_str()),
                ])
                .header("Referer", "http://bond.sse.com.cn/"),
        )
        .await?;

        let body = resp.text().await.map_err(Error::from)?;
        let root: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| Error::decode(format!("SSE repo parse: {e}")))?;

        let data = root["result"].as_array().cloned().unwrap_or_default();

        if data.is_empty() {
            return Err(Error::not_found(format!("SSE repo: no data for {date}")));
        }
        Ok(data)
    }

    /// SZSE repo (质押式回购) data.
    ///
    /// `date` is in YYYYMMDD format.
    pub async fn bond_repo_szse(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let formatted = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        let url = "http://www.szse.cn/api/report/ShowReport/data";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[
                    ("SHOWTYPE", "JSON"),
                    ("CATALOGID", "1812"),
                    ("txtDate", formatted.as_str()),
                ])
                .header("Referer", "http://www.szse.cn/"),
        )
        .await?;

        let body = resp.text().await.map_err(Error::from)?;
        let root: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| Error::decode(format!("SZSE repo parse: {e}")))?;

        let data = root.as_array().cloned().unwrap_or_default();
        let mut items = Vec::new();
        for entry in &data {
            let rows = entry["data"].as_array().cloned().unwrap_or_default();
            items.extend(rows);
        }

        if items.is_empty() {
            return Err(Error::not_found(format!("SZSE repo: no data for {date}")));
        }
        Ok(items)
    }

    /// China-US treasury yield comparison from Eastmoney.
    ///
    /// Returns time series of CN and US treasury yields for various tenors.
    pub async fn bond_zh_us_rate_latest(&self) -> Result<Vec<serde_json::Value>> {
        let mut all_items = Vec::new();

        for page in 1..=5 {
            let resp: EmDatacenterResp = self
                .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("reportName", "RPTA_WEB_TREASURYYIELD"),
                    ("columns", "ALL"),
                    ("sortColumns", "SOLAR_DATE"),
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

            if let Some(msg) = resp.check_error("RPTA_WEB_TREASURYYIELD") {
                return Err(Error::upstream(msg));
            }
            let data = resp.result.map(|r| r.data).unwrap_or_default();
            if data.is_empty() {
                break;
            }
            all_items.extend(data);
        }

        if all_items.is_empty() {
            return Err(Error::not_found(
                "no China-US treasury yield data available",
            ));
        }
        Ok(all_items)
    }

    /// China bond yield curve data from Eastmoney.
    ///
    /// Returns yield curve points for government bonds across various maturities.
    pub async fn bond_yield_curve(&self) -> Result<Vec<serde_json::Value>> {
        let resp: EmDatacenterResp = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_BOND_GOV_CN_YIELD"),
                ("columns", "ALL"),
                ("sortColumns", "SOLAR_DATE"),
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

        if let Some(msg) = resp.check_error("RPT_BOND_GOV_CN_YIELD") {
            return Err(Error::upstream(msg));
        }
        let data = resp.result.map(|r| r.data).unwrap_or_default();

        if data.is_empty() {
            return Err(Error::not_found("no yield curve data available"));
        }
        Ok(data)
    }

    /// Treasury bond issuance data from Eastmoney.
    ///
    /// `date` is in YYYYMMDD format; used as start date for filtering.
    pub async fn bond_treasury_issue_em(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let formatted = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        let resp: EmDatacenterResp = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_BOND_GOV_ISSUE"),
                ("columns", "ALL"),
                ("sortColumns", "ISSUE_DATE"),
                ("sortTypes", "-1"),
                ("filter", &format!("(ISSUE_DATE>='{formatted}')")),
                ("pageSize", "500"),
                ("pageNumber", "1"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        if let Some(msg) = resp.check_error("RPT_BOND_GOV_ISSUE") {
            return Err(Error::upstream(msg));
        }
        let data = resp.result.map(|r| r.data).unwrap_or_default();

        if data.is_empty() {
            return Err(Error::not_found(format!(
                "no treasury issuance data for {date}"
            )));
        }
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_date_formatting() {
        let date = "20250615";
        let formatted = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        assert_eq!(formatted, "2025-06-15");
    }
}
