//! Institutional research (机构调研) from Eastmoney.

use super::helpers::*;
use super::types::*;
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 机构调研统计
    pub async fn stock_jgdy_tj_em(&self, date: &str) -> Result<Vec<JgdyTj>> {
        let date_fmt = fmt_date(date);
        let filter = format!(
            "(NUMBERNEW=\"1\")(IS_SOURCE=\"1\")(NOTICE_DATE>'{}')",
            date_fmt
        );
        let data = self
            .dc_fetch_all(
                "RPT_ORG_SURVEYNEW",
                "ALL",
                &filter,
                "NOTICE_DATE,SUM,RECEIVE_START_DATE,SECURITY_CODE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| JgdyTj {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                latest_price: json_f64_opt(v, "CLOSE_PRICE"),
                change_pct: json_f64_opt(v, "CHANGE_RATE"),
                org_count: json_i64(v, "SUM"),
                receive_method: json_str_opt(v, "RECEIVE_WAY"),
                receive_personnel: json_str_opt(v, "RECEIVE_TARGET"),
                receive_location: json_str_opt(v, "RECEIVE_PLACE"),
                receive_date: json_str(v, "RECEIVE_START_DATE"),
                notice_date: json_str(v, "NOTICE_DATE"),
            })
            .collect())
    }

    /// 机构调研详细
    pub async fn stock_jgdy_detail_em(&self, date: &str) -> Result<Vec<JgdyDetail>> {
        let date_fmt = fmt_date(date);
        let filter = format!("(NOTICE_DATE>'{}')", date_fmt);
        let data = self
            .dc_fetch_all(
                "RPT_ORG_SURVEY",
                "ALL",
                &filter,
                "NOTICE_DATE,SECURITY_CODE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| JgdyDetail {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                notice_date: json_str(v, "NOTICE_DATE"),
                receive_date: json_str(v, "RECEIVE_START_DATE"),
                receive_location: json_str_opt(v, "RECEIVE_PLACE"),
                receive_method: json_str_opt(v, "RECEIVE_WAY"),
                receive_personnel: json_str_opt(v, "RECEIVE_TARGET"),
                org_count: json_i64(v, "ORG_NUM"),
                content: json_str_opt(v, "ORG_SURVEY_CONTENT"),
            })
            .collect())
    }
}
