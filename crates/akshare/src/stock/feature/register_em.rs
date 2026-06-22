//! Registration data (注册制/新股注册) from Eastmoney.

use super::helpers::{json_f64_opt, json_str, json_str_opt};
use super::types::RegisterEntry;
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// Fetch registration data from Eastmoney datacenter.
    async fn register_fetch(&self, board: &str) -> Result<Vec<RegisterEntry>> {
        let filter = match board {
            "科创板" => "(MARKET=\"科创板\")",
            "创业板" => "(MARKET=\"创业板\")",
            "北交所" => "(MARKET=\"北交所\")",
            "沪市主板" => "(MARKET=\"沪市主板\")",
            "深市主板" => "(MARKET=\"深市主板\")",
            _ => "",
        };
        let data = self
            .dc_fetch_all(
                "RPT_REGISTERED_VIEW",
                "SECURITY_CODE,SECURITY_NAME_ABBR,INDUSTRY,LISTING_DATE,ISSUE_PRICE,PE_RATIO",
                filter,
                "LISTING_DATE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| RegisterEntry {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                industry: json_str_opt(v, "INDUSTRY"),
                list_date: json_str_opt(v, "LISTING_DATE"),
                issue_price: json_f64_opt(v, "ISSUE_PRICE"),
                pe_ratio: json_f64_opt(v, "PE_RATIO"),
                extra: None,
            })
            .collect())
    }

    /// 全部注册制
    pub async fn stock_register_all(&self) -> Result<Vec<RegisterEntry>> {
        self.register_fetch("全部").await
    }

    /// 北交所注册制
    pub async fn stock_register_bj(&self) -> Result<Vec<RegisterEntry>> {
        self.register_fetch("北交所").await
    }

    /// 创业板注册制
    pub async fn stock_register_cyb(&self) -> Result<Vec<RegisterEntry>> {
        self.register_fetch("创业板").await
    }

    /// 沪深主板注册制
    pub async fn stock_register_db(&self) -> Result<Vec<RegisterEntry>> {
        self.register_fetch("全部").await
    }

    /// 科创板注册制
    pub async fn stock_register_kcb(&self) -> Result<Vec<RegisterEntry>> {
        self.register_fetch("科创板").await
    }

    /// 沪市主板注册制
    pub async fn stock_register_sh(&self) -> Result<Vec<RegisterEntry>> {
        self.register_fetch("沪市主板").await
    }

    /// 深市主板注册制
    pub async fn stock_register_sz(&self) -> Result<Vec<RegisterEntry>> {
        self.register_fetch("深市主板").await
    }
}
