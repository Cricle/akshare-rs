//! Earnings report (业绩报表) from Eastmoney.

use crate::client::AkShareClient;
use crate::error::Result;
use super::helpers::*;
use super::types::*;

impl AkShareClient {
    /// 业绩报表
    pub async fn stock_yjbb_em(&self, date: &str) -> Result<Vec<EarningsReport>> {
        let date_fmt = fmt_date(date);
        let filter = format!("(REPORTDATE='{}')", date_fmt);
        let data = self.dc_fetch_all("RPT_LICO_FN_CPD", "ALL", &filter, "UPDATE_DATE,SECURITY_CODE", "-1", 500, 10, &[]).await?;
        Ok(data.iter().map(|v| EarningsReport {
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            eps: json_f64(v, "BASIC_EPS"),
            total_revenue: json_f64(v, "TOTAL_OPERATE_INCOME"),
            total_revenue_yoy: json_f64(v, "TOTAL_OPERATE_INCOME_YOY"),
            total_revenue_qoq: json_f64(v, "TOTAL_OPERATE_INCOME_QOQ"),
            net_profit: json_f64(v, "PARENT_NETPROFIT"),
            net_profit_yoy: json_f64(v, "PARENT_NETPROFIT_YOY"),
            net_profit_qoq: json_f64(v, "PARENT_NETPROFIT_QOQ"),
            bvps: json_f64(v, "BPS"),
            roe: json_f64(v, "WEIGHTAVG_ROE"),
            operating_cash_flow_per_share: json_f64(v, "MGJYXJJE"),
            gross_margin: json_f64(v, "XSMLL"),
            industry: json_str_opt(v, "INDUSTRY"),
            notice_date: json_str(v, "NOTICE_DATE"),
        }).collect())
    }
}
