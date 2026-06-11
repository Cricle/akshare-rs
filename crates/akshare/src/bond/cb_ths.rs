//! Convertible bond info from THS (同花顺).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

#[derive(Debug, Deserialize)]
struct ThsBondResp {
    list: Option<Vec<serde_json::Value>>,
}

impl AkShareClient {
    /// Fetch convertible bond info from THS (同花顺).
    ///
    /// Returns CB issue details including subscription dates, listing dates,
    /// conversion prices, etc.
    pub async fn bond_zh_cov_info_ths(&self) -> Result<Vec<serde_json::Value>> {
        let resp: ThsBondResp = self
            .get("https://data.10jqka.com.cn/ipo/kzz/")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let items = resp.list.unwrap_or_default();
        if items.is_empty() {
            return Err(Error::not_found("ths returned no convertible bond data"));
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_ths_bond_resp() {
        let j = json!({"list": [{"bond_code": "123121", "bond_name": "测试转债"}]});
        let resp: ThsBondResp = serde_json::from_value(j).unwrap();
        assert_eq!(resp.list.unwrap().len(), 1);
    }
}
