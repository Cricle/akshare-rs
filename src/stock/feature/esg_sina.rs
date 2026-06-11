//! ESG ratings (ESG评级) from Sina Finance.

use crate::client::AkShareClient;
use crate::error::Result;

use super::types::*;

impl AkShareClient {
    /// Sina ESG fetch helper
    async fn esg_sina_fetch(&self, endpoint: &str) -> Result<Vec<EsgRating>> {
        let mut all = Vec::new();
        let first_url = format!("https://global.finance.sina.com.cn/api/openapi.php/{}?p=1&num=100", endpoint);
        let first = self.get(&first_url)
            .header("Referer", "https://finance.sina.com.cn/")
            .send().await?
            .error_for_status()?
            .json::<serde_json::Value>().await?;

        let total: i64 = first.get("result").and_then(|r| r.get("data")).and_then(|d| d.get("total")).and_then(|t| t.as_str()).and_then(|s| s.parse().ok()).unwrap_or(0);
        let page_count = (total + 99) / 100;

        for page in 1..=page_count {
            let url = format!("https://global.finance.sina.com.cn/api/openapi.php/{}?p={}&num=100", endpoint, page);
            let resp = self.get(&url)
                .header("Referer", "https://finance.sina.com.cn/")
                .send().await?
                .error_for_status()?
                .json::<serde_json::Value>().await?;
            let data = resp.get("result").and_then(|r| r.get("data")).and_then(|d| d.get("data")).and_then(|d| d.as_array()).cloned().unwrap_or_default();
            for v in &data {
                all.push(EsgRating {
                    code: v.get("symbol").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                    name: v.get("name").and_then(|x| x.as_str()).map(|s| s.to_string()),
                    esg_score: v.get("esg_score").or_else(|| v.get("total_score")).and_then(|x| x.as_f64()),
                    env_score: v.get("environment_score").or_else(|| v.get("env_score")).and_then(|x| x.as_f64()),
                    social_score: v.get("social_score").and_then(|x| x.as_f64()),
                    governance_score: v.get("governance_score").and_then(|x| x.as_f64()),
                    rating_date: v.get("rating_date").or_else(|| v.get("update_date")).and_then(|x| x.as_str()).map(|s| s.to_string()),
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
