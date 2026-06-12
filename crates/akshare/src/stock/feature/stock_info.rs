//! Stock info utilities from Eastmoney.

use super::helpers::json_str;
use crate::client::AkShareClient;
use crate::error::Result;

/// 股票信息查询 (from Eastmoney search)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockInfo {
    pub code: String,
    pub name: String,
    pub market: String,
    pub industry: Option<String>,
}

impl AkShareClient {
    /// 东方财富-股票信息查询
    pub async fn stock_info_a_code_name(&self) -> Result<Vec<StockInfo>> {
        let data = self
            .clist_spot_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f12,f14",
                "5000",
                "f12",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| StockInfo {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                market: "A".to_string(),
                industry: None,
            })
            .collect())
    }
}
