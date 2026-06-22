//! ESG ratings (ESG评级) from Sina Finance.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::value_ext::ValueExt;

use super::types::EsgRating;

impl AkShareClient {
    /// Sina ESG fetch helper
    async fn esg_sina_fetch(&self, endpoint: &str) -> Result<Vec<EsgRating>> {
        let mut all = Vec::new();
        let first_url =
            format!("https://global.finance.sina.com.cn/api/openapi.php/{endpoint}?p=1&num=100");
        let first = self
            .get(&first_url)
            .header("Referer", "https://finance.sina.com.cn/")
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        let total: i64 = first
            .nested(&["result", "data", "total"])
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let page_count = (total + 99) / 100;

        for page in 1..=page_count {
            let url = format!(
                "https://global.finance.sina.com.cn/api/openapi.php/{endpoint}?p={page}&num=100"
            );
            let resp = self
                .get(&url)
                .header("Referer", "https://finance.sina.com.cn/")
                .send()
                .await?
                .error_for_status()?
                .json::<serde_json::Value>()
                .await?;
            let data = resp
                .nested(&["result", "data", "data"])
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            for v in &data {
                all.push(EsgRating {
                    code: v.str_or(&["symbol"], ""),
                    name: v.str_field(&["name"]).map(std::string::ToString::to_string),
                    esg_score: v.f64_field(&["esg_score", "total_score"]),
                    env_score: v.f64_field(&["environment_score", "env_score"]),
                    social_score: v.f64_field(&["social_score"]),
                    governance_score: v.f64_field(&["governance_score"]),
                    rating_date: v
                        .str_field(&["rating_date", "update_date"])
                        .map(std::string::ToString::to_string),
                    market: Some("CN".to_string()),
                });
            }
        }
        Ok(all)
    }

    /// 新浪-ESG评级-MSCI
    pub async fn stock_esg_msci_sina(&self) -> Result<Vec<EsgRating>> {
        self.esg_sina_fetch("EsgService.getMsciEsgStocks").await
    }

    /// 新浪-ESG评级-路孚特
    pub async fn stock_esg_rft_sina(&self) -> Result<Vec<EsgRating>> {
        self.esg_sina_fetch("EsgService.getRftEsgStocks").await
    }

    /// 新浪-ESG评级-秩鼎
    pub async fn stock_esg_zd_sina(&self) -> Result<Vec<EsgRating>> {
        self.esg_sina_fetch("EsgService.getZdEsgStocks").await
    }

    /// 新浪-ESG评级-华证
    pub async fn stock_esg_hz_sina(&self) -> Result<Vec<EsgRating>> {
        self.esg_sina_fetch("EsgService.getHzEsgStocks").await
    }
}
