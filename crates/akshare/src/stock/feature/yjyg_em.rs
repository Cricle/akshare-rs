//! Earnings forecast (业绩预告) and quick report (业绩快报) from Eastmoney.

use super::helpers::{fmt_date, json_f64_opt, json_str, json_str_opt};
use super::types::{EarningsForecast, EarningsQuickReport};
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 业绩快报
    pub async fn stock_yjkb_em(&self, date: &str) -> Result<Vec<EarningsQuickReport>> {
        let date_fmt = fmt_date(date);
        let filter = format!(
            "(SECURITY_TYPE_CODE in (\"058001001\",\"058001008\"))(TRADE_MARKET_CODE!=\"069001017\")(REPORT_DATE='{date_fmt}')"
        );
        let data = self
            .dc_fetch_all(
                "RPT_FCI_PERFORMANCEE",
                "ALL",
                &filter,
                "UPDATE_DATE,SECURITY_CODE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| EarningsQuickReport {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                notice_date: json_str(v, "NOTICE_DATE"),
                eps: json_f64_opt(v, "BASIC_EPS"),
                bvps: json_f64_opt(v, "BPS"),
                total_revenue: json_f64_opt(v, "TOTAL_OPERATE_INCOME"),
                net_profit: json_f64_opt(v, "PARENT_NETPROFIT"),
                revenue_yoy: json_f64_opt(v, "TOTAL_OPERATE_INCOME_YOY"),
                net_profit_yoy: json_f64_opt(v, "PARENT_NETPROFIT_YOY"),
                roe: json_f64_opt(v, "WEIGHTAVG_ROE"),
                net_margin: json_f64_opt(v, "XSJLL"),
            })
            .collect())
    }

    /// 业绩预告
    pub async fn stock_yjyg_em(&self, date: &str) -> Result<Vec<EarningsForecast>> {
        let date_fmt = fmt_date(date);
        let filter = format!("(REPORT_DATE='{date_fmt}')");
        let data = self
            .dc_fetch_all(
                "RPT_PUBLIC_OP_NEWPREDICT",
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
            .map(|v| EarningsForecast {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                notice_date: json_str(v, "NOTICE_DATE"),
                forecast_type: json_str(v, "PREDICT_FINANCE"),
                forecast_content: json_str_opt(v, "PREDICT_CONTENT"),
                change_range: json_str_opt(v, "CHANGE_REASON_EXPLAIN"),
                change_reason: json_str_opt(v, "PREDICT_REASON"),
                previous_year_net_profit: json_f64_opt(v, "PREYEAR_SAME_PERIOD"),
            })
            .collect())
    }

    /// 预约披露时间
    pub async fn stock_yysj_em(&self, market: &str, date: &str) -> Result<Vec<serde_json::Value>> {
        let date_fmt = fmt_date(date);
        let filter = match market {
            "沪市A股" => format!(
                "(SECURITY_TYPE_CODE in (\"058001001\",\"058001008\"))(TRADE_MARKET_CODE in (\"069001001001\",\"069001001003\",\"069001001006\"))(REPORT_DATE='{date_fmt}')"
            ),
            "科创板" => format!(
                "(SECURITY_TYPE_CODE in (\"058001001\",\"058001008\"))(TRADE_MARKET_CODE=\"069001001006\")(REPORT_DATE='{date_fmt}')"
            ),
            "深市A股" => format!(
                "(SECURITY_TYPE_CODE=\"058001001\")(TRADE_MARKET_CODE in (\"069001002001\",\"069001002002\",\"069001002003\",\"069001002005\"))(REPORT_DATE='{date_fmt}')"
            ),
            "创业板" => format!(
                "(SECURITY_TYPE_CODE=\"058001001\")(TRADE_MARKET_CODE=\"069001002002\")(REPORT_DATE='{date_fmt}')"
            ),
            "京市A股" => format!("(TRADE_MARKET_CODE=\"069001017\")(REPORT_DATE='{date_fmt}')"),
            _ => format!(
                "(SECURITY_TYPE_CODE in (\"058001001\",\"058001008\"))(TRADE_MARKET_CODE!=\"069001017\")(REPORT_DATE='{date_fmt}')"
            ),
        };
        self.dc_fetch_all(
            "RPT_PUBLIC_BS_APPOIN",
            "ALL",
            &filter,
            "FIRST_APPOINT_DATE,SECURITY_CODE",
            "1",
            500,
            10,
            &[],
        )
        .await
    }
}
