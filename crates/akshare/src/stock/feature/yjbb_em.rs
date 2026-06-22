//! Earnings report (业绩报表) from Eastmoney.

use super::helpers::{fmt_date, json_f64, json_str, json_str_opt};
use super::types::EarningsReport;
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 业绩报表
    pub async fn stock_yjbb(&self, date: &str) -> Result<Vec<EarningsReport>> {
        let date_fmt = fmt_date(date);
        let filter = format!("(REPORTDATE='{date_fmt}')");
        let data = self
            .dc_fetch_all(
                "RPT_LICO_FN_CPD",
                "ALL",
                &filter,
                "UPDATE_DATE,SECURITY_CODE",
                "-1,-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| EarningsReport {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                eps: json_f64(v, "BASIC_EPS"),
                total_revenue: json_f64(v, "TOTAL_OPERATE_INCOME"),
                total_revenue_yoy: json_f64(v, "YSTZ"),
                total_revenue_qoq: json_f64(v, "YSHZ"),
                net_profit: json_f64(v, "PARENT_NETPROFIT"),
                net_profit_yoy: json_f64(v, "SJLTZ"),
                net_profit_qoq: json_f64(v, "SJLHZ"),
                bvps: json_f64(v, "BPS"),
                roe: json_f64(v, "WEIGHTAVG_ROE"),
                operating_cash_flow_per_share: json_f64(v, "MGJYXJJE"),
                gross_margin: json_f64(v, "XSMLL"),
                industry: json_str_opt(v, "BOARD_NAME"),
                notice_date: json_str(v, "NOTICE_DATE"),
            })
            .collect())
    }
}
