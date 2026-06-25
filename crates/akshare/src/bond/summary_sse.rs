//! Bond summary data from SSE (上登债券信息网).

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch bond cash market summary from SSE.
    ///
    /// `date` is in YYYYMMDD format.
    /// Returns custody statistics for bond types.
    pub async fn bond_cash_summary(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let formatted = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        let url = "http://query.sse.com.cn/commonExcelDd.do";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[
                ("sqlId", "COMMON_SSEBOND_SCSJ_SCTJ_SCGL_ZQXQSCGL_CX_L"),
                ("TRADE_DATE", formatted.as_str()),
                ])
                .header("Referer", "http://bond.sse.com.cn/")
        )
        .await?;

        let _bytes = resp.bytes().await.map_err(Error::from)?;
        // SSE returns Excel files; parsing requires xlsx/xls support.
        Err(Error::decode(
            "SSE bond summary returns Excel files; add xlsx crate support to parse",
        ))
    }

    /// Fetch bond deal summary from SSE.
    ///
    /// `date` is in YYYYMMDD format.
    /// Returns trading volume and deal statistics by bond type.
    pub async fn bond_deal_summary(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let formatted = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        let url = "http://query.sse.com.cn/commonExcelDd.do";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[
                ("sqlId", "COMMON_SSEBOND_SCSJ_SCTJ_SCGL_ZQCJGL_CX_L"),
                ("TRADE_DATE", formatted.as_str()),
                ])
                .header("Referer", "http://bond.sse.com.cn/")
        )
        .await?;

        let _bytes = resp.bytes().await.map_err(Error::from)?;
        Err(Error::decode(
            "SSE bond deal summary returns Excel files; add xlsx crate support to parse",
        ))
    }
}
