//! Goodwill (商誉) from Eastmoney.

use super::helpers::{json_f64, json_str, json_str_opt};
use super::types::{SyDetail, SyJzDetail, SyProfile, SyYqDetail};
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// A股商誉市场概况
    pub async fn stock_sy_profile_em(&self) -> Result<Vec<SyProfile>> {
        let data = self
            .dc_fetch_all(
                "RPT_GOODWILL_MARKET",
                "ALL",
                "",
                "REPORT_DATE",
                "-1",
                500,
                2,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| SyProfile {
                report_date: json_str(v, "REPORT_DATE"),
                goodwill: json_f64(v, "GOODWILL"),
                goodwill_impairment: json_f64(v, "GOODWILL_IMPAIRMENT"),
                net_assets: json_f64(v, "NET_ASSETS"),
                goodwill_to_net_assets_ratio: json_f64(v, "GOODWILL_TO_NET_ASSETS_RATIO"),
                impairment_to_net_assets_ratio: json_f64(v, "IMPAIRMENT_TO_NET_ASSETS_RATIO"),
                net_profit: json_f64(v, "NET_PROFIT"),
                impairment_to_net_profit_ratio: json_f64(v, "IMPAIRMENT_TO_NET_PROFIT_RATIO"),
            })
            .collect())
    }

    /// 商誉减值预期明细
    pub async fn stock_sy_yq_em(&self, indicator: &str) -> Result<Vec<SyYqDetail>> {
        let report_name = match indicator {
            "沪市主板" => "RPT_GOODWILL_EXPECT_SH",
            "深市主板" => "RPT_GOODWILL_EXPECT_SZ",
            "创业板" => "RPT_GOODWILL_EXPECT_CYB",
            _ => "RPT_GOODWILL_EXPECT",
        };
        let data = self
            .dc_fetch_all(report_name, "ALL", "", "GOODWILL", "-1", 500, 5, &[])
            .await?;
        Ok(data
            .iter()
            .map(|v| SyYqDetail {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                report_date: json_str(v, "REPORT_DATE"),
                notice_date: json_str(v, "NOTICE_DATE"),
                goodwill: json_f64(v, "GOODWILL"),
                predicted_impairment: json_f64(v, "PREDICTED_IMPAIRMENT"),
                net_profit: json_f64(v, "NET_PROFIT"),
                impairment_reason: json_str_opt(v, "IMPAIRMENT_REASON"),
            })
            .collect())
    }

    /// 个股商誉减值明细
    pub async fn stock_sy_jz_em(&self, symbol: &str) -> Result<Vec<SyJzDetail>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_GOODWILL_IMPAIRMENT",
                "ALL",
                &filter,
                "REPORT_DATE",
                "-1",
                500,
                2,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| SyJzDetail {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                report_date: json_str(v, "REPORT_DATE"),
                notice_date: json_str(v, "NOTICE_DATE"),
                goodwill: json_f64(v, "GOODWILL"),
                impairment_amount: json_f64(v, "IMPAIRMENT_AMOUNT"),
                net_profit: json_f64(v, "NET_PROFIT"),
            })
            .collect())
    }

    /// 个股商誉明细
    pub async fn stock_sy_em(&self, symbol: &str) -> Result<Vec<SyDetail>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_GOODWILL_DETAIL",
                "ALL",
                &filter,
                "REPORT_DATE",
                "-1",
                500,
                2,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| SyDetail {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                report_date: json_str(v, "REPORT_DATE"),
                notice_date: json_str(v, "NOTICE_DATE"),
                goodwill: json_f64(v, "GOODWILL"),
                goodwill_change: json_f64(v, "GOODWILL_CHANGE"),
                net_profit: json_f64(v, "NET_PROFIT"),
                goodwill_to_net_assets_ratio: json_f64(v, "GOODWILL_TO_NET_ASSETS_RATIO"),
            })
            .collect())
    }
}
