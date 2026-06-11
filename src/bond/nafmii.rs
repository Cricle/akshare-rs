//! NAFMII bond registration data (中国银行间市场交易商协会).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

#[derive(Debug, Deserialize)]
struct NafmiiResp {
    rows: Option<Vec<serde_json::Value>>,
}

impl AkShareClient {
    /// Fetch NAFMII bond registration info.
    ///
    /// `page` is the page number (1-indexed, 50 items per page).
    /// Returns bond names, types, amounts, registration numbers, and dates.
    pub async fn bond_debt_nafmii(&self, page: u32) -> Result<Vec<serde_json::Value>> {
        let resp: NafmiiResp = self
                        .post("http://zhuce.nafmii.org.cn/fans/publicQuery/releFileProjDataGrid")
            .form(&[
                ("regFileName", ""),
                ("itemType", ""),
                ("startTime", ""),
                ("endTime", ""),
                ("entityName", ""),
                ("leadManager", ""),
                ("regPrdtType", ""),
                ("page", &page.to_string()),
                ("rows", "50"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let rows = resp.rows.unwrap_or_default();
        if rows.is_empty() {
            return Err(Error::not_found(format!(
                "nafmii returned no data for page {page}"
            )));
        }
        Ok(rows)
    }
}
