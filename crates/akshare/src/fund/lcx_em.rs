//! Financial fund ranking from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch financial fund ranking (Python: fund_lcx_rank_em).
    pub async fn fund_lcx_rank_em(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .get("https://api.fund.eastmoney.com/FundRank/GetLcRankList")
            .header("Referer", "https://fund.eastmoney.com/fundguzhi.html")
            .query(&[
                ("intCompany", "0"),
                ("MinsgType", "undefined"),
                ("IsSale", "1"),
                ("strSortCol", "SYL_Z"),
                ("orderType", "desc"),
                ("pageIndex", "1"),
                ("pageSize", "50"),
                ("FBQ", ""),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("Data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| Error::not_found("no financial fund rank data"))?;

        if data.is_empty() {
            return Err(Error::not_found("no financial fund rank data"));
        }
        Ok(data.clone())
    }
}
