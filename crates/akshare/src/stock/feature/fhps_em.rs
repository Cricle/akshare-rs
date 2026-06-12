//! Dividends (分红送配) from Eastmoney.

use super::helpers::{fmt_date, json_f64_opt, json_str, json_str_opt};
use super::types::DividendInfo;
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 分红送配
    pub async fn stock_fhps_em(&self, date: &str) -> Result<Vec<DividendInfo>> {
        let date_fmt = fmt_date(date);
        let filter = format!("(REPORT_DATE='{date_fmt}')");
        let data = self
            .dc_fetch_all(
                "RPT_SHAREBONUS_DET",
                "ALL",
                &filter,
                "PLAN_NOTICE_DATE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| DividendInfo {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                bonus_shares_ratio: json_f64_opt(v, "BONUS_IT_RATIO"),
                transfer_ratio: json_f64_opt(v, "TRANSFER_IT_RATIO"),
                convert_ratio: json_f64_opt(v, "PRETAX_BONUS_RMB"),
                cash_dividend_ratio: json_f64_opt(v, "PRETAX_BONUS_RMB"),
                dividend_yield: json_f64_opt(v, "DIVIDEND_YIELD"),
                eps: json_f64_opt(v, "EPS"),
                bvps: json_f64_opt(v, "BPS"),
                capital_reserve_per_share: json_f64_opt(v, "CAPITAL_RESERVE_PS"),
                undistributed_profit_per_share: json_f64_opt(v, "UNDISTRIBUTED_PER_SHARE"),
                net_profit_yoy: json_f64_opt(v, "PARENT_NETPROFIT_YOY"),
                total_shares: json_f64_opt(v, "TOTAL_SHARES"),
                plan_notice_date: json_str_opt(v, "PLAN_NOTICE_DATE"),
                record_date: json_str_opt(v, "EQUITY_RECORD_DATE"),
                ex_date: json_str_opt(v, "EX_DIVIDEND_DATE"),
                plan_progress: json_str_opt(v, "IMPL_PLAN_PROFILE"),
                latest_notice_date: json_str_opt(v, "NOTICE_DATE"),
            })
            .collect())
    }

    /// 分红送配详情
    pub async fn stock_fhps_detail_em(&self, symbol: &str) -> Result<Vec<DividendInfo>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_SHAREBONUS_DET",
                "ALL",
                &filter,
                "REPORT_DATE",
                "-1",
                500,
                5,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| DividendInfo {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                bonus_shares_ratio: json_f64_opt(v, "BONUS_IT_RATIO"),
                transfer_ratio: json_f64_opt(v, "TRANSFER_IT_RATIO"),
                convert_ratio: json_f64_opt(v, "PRETAX_BONUS_RMB"),
                cash_dividend_ratio: json_f64_opt(v, "PRETAX_BONUS_RMB"),
                dividend_yield: json_f64_opt(v, "DIVIDEND_YIELD"),
                eps: json_f64_opt(v, "EPS"),
                bvps: json_f64_opt(v, "BPS"),
                capital_reserve_per_share: json_f64_opt(v, "CAPITAL_RESERVE_PS"),
                undistributed_profit_per_share: json_f64_opt(v, "UNDISTRIBUTED_PER_SHARE"),
                net_profit_yoy: json_f64_opt(v, "PARENT_NETPROFIT_YOY"),
                total_shares: json_f64_opt(v, "TOTAL_SHARES"),
                plan_notice_date: json_str_opt(v, "PLAN_NOTICE_DATE"),
                record_date: json_str_opt(v, "EQUITY_RECORD_DATE"),
                ex_date: json_str_opt(v, "EX_DIVIDEND_DATE"),
                plan_progress: json_str_opt(v, "IMPL_PLAN_PROFILE"),
                latest_notice_date: json_str_opt(v, "NOTICE_DATE"),
            })
            .collect())
    }
}
