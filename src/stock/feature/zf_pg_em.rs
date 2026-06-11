//! SEO (增发) and rights issue (配股) from Eastmoney.

use super::helpers::*;
use super::types::*;
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 全部增发
    pub async fn stock_qbzf_em(&self) -> Result<Vec<Qbzf>> {
        let data = self
            .dc_fetch_all(
                "RPT_SEO_DETAIL",
                "ALL",
                "",
                "ISSUE_DATE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| Qbzf {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                seo_code: json_str(v, "SEO_CODE"),
                issue_type: json_str(v, "SEO_TYPE"),
                issue_count: json_f64(v, "ISSUE_NUM"),
                online_issue: json_f64_opt(v, "ONLINE_ISSUE_NUM"),
                issue_price: json_f64(v, "ISSUE_PRICE"),
                latest_price: json_f64_opt(v, "NEW_PRICE"),
                issue_date: json_str(v, "ISSUE_DATE"),
                listing_date: json_str_opt(v, "LISTING_DATE"),
                lock_period: json_str_opt(v, "LOCK_PERIOD"),
            })
            .collect())
    }

    /// 配股
    pub async fn stock_pg_em(&self) -> Result<Vec<Pg>> {
        let data = self
            .dc_fetch_all(
                "RPT_RIGHTS_ISSUE",
                "ALL",
                "",
                "PLAN_NOTICE_DATE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| Pg {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                pg_code: json_str(v, "RIGHTS_ISSUE_CODE"),
                issue_price: json_f64(v, "ISSUE_PRICE"),
                latest_price: json_f64_opt(v, "NEW_PRICE"),
                issue_count: json_f64(v, "ISSUE_NUM"),
                plan_notice_date: json_str_opt(v, "PLAN_NOTICE_DATE"),
                record_date: json_str_opt(v, "RECORD_DATE"),
                pay_date: json_str_opt(v, "PAY_DATE"),
                listing_date: json_str_opt(v, "LISTING_DATE"),
                issue_type: json_str_opt(v, "ISSUE_TYPE"),
            })
            .collect())
    }
}
