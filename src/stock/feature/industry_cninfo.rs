//! Industry data (行业数据) from CNINFO and Shenwan.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use super::helpers::*;
use super::types::*;

impl AkShareClient {
    /// 巨潮-行业分类
    pub async fn stock_industry_category_cninfo(&self) -> Result<Vec<IndustryCategory>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1125";
        let resp = self.post(url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| IndustryCategory {
            code: json_str(v, "INDUSTRYCODE"),
            name: json_str(v, "INDUSTRYNAME"),
            industry: json_str_opt(v, "CATALOGNAME"),
            extra: None,
        }).collect())
    }

    /// 巨潮-行业变动
    pub async fn stock_industry_change_cninfo(&self, symbol: &str) -> Result<Vec<IndustryChange>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1126";
        let resp = self.post(url)
            .form(&[("indcode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| IndustryChange {
            data: v.clone(),
        }).collect())
    }

    /// 申万-行业分类历史
    pub async fn stock_industry_clf_hist_sw(&self, symbol: &str) -> Result<Vec<IndustryClfHistSw>> {
        let url = "https://www.swsindex.com/swindex.aspx";
        let resp = self.get(url)
            .query(&[("swindexcode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?;
        let _text = resp.text().await.map_err(Error::from)?;
        // SW website returns HTML - return placeholder
        Ok(vec![IndustryClfHistSw {
            data: serde_json::json!({"symbol": symbol, "note": "SW index HTML parsing not implemented"}),
        }])
    }

    /// 巨潮-行业市盈率
    pub async fn stock_industry_pe_ratio_cninfo(&self, symbol: &str) -> Result<Vec<IndustryPeRatio>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1131";
        let resp = self.post(url)
            .form(&[("indcode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| IndustryPeRatio {
            data: v.clone(),
        }).collect())
    }
}
