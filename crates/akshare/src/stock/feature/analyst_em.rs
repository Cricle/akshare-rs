//! Analyst rankings from Eastmoney.

use super::helpers::{json_f64, json_f64_opt, json_i64, json_str, json_str_opt};
use super::types::{AnalystDetail, AnalystRank};
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 东方财富分析师指数
    pub async fn stock_analyst_rank(&self, year: &str) -> Result<Vec<AnalystRank>> {
        let filter = format!("(YEAR=\"{year}\")");
        let data = self
            .dc_fetch_all(
                "RPT_ANALYST_INDEX_RANK",
                "ALL",
                &filter,
                "YEAR_YIELD",
                "-1",
                500,
                1,
                &[("distinct", "ANALYST_CODE"), ("limit", "top100")],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| AnalystRank {
                analyst_name: json_str(v, "ANALYST_NAME"),
                analyst_org: json_str(v, "ORG_NAME"),
                annual_index: json_f64(v, "YEAR_INDEX"),
                annual_return: json_f64(v, "YEAR_YIELD"),
                return_3m: json_f64(v, "THREE_MONTH_YIELD"),
                return_6m: json_f64(v, "SIX_MONTH_YIELD"),
                return_12m: json_f64(v, "TWELVE_MONTH_YIELD"),
                constituent_count: json_i64(v, "CONSTITUTE_NUM"),
                latest_stock_name: json_str_opt(v, "RATING_NAME"),
                latest_stock_code: json_str_opt(v, "RATING_CODE"),
                analyst_id: json_str(v, "ANALYST_CODE"),
                industry_code: json_str_opt(v, "INDUSTRY_CODE"),
                industry: json_str_opt(v, "INDUSTRY_NAME"),
                update_date: json_str(v, "UPDATE_DATE"),
                year: json_str(v, "YEAR"),
            })
            .collect())
    }

    /// 东方财富分析师详情
    pub async fn stock_analyst_detail(
        &self,
        analyst_id: &str,
        indicator: &str,
    ) -> Result<Vec<AnalystDetail>> {
        let (report, cols) = match indicator {
            "历史跟踪成分股" => ("RPT_ANALYST_INDEX_HISTORY", "ALL"),
            "历史指数" => ("RPT_ANALYST_INDEX_HIST", "ALL"),
            _ => ("RPT_ANALYST_INDEX_LATEST", "ALL"),
        };
        let filter = format!("(ANALYST_CODE=\"{analyst_id}\")");
        let data = self
            .dc_fetch_all(report, cols, &filter, "TRADE_DATE", "-1", 500, 1, &[])
            .await?;
        Ok(data
            .iter()
            .map(|v| AnalystDetail {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                latest_price: json_f64_opt(v, "CLOSE_PRICE"),
                change_pct: json_f64_opt(v, "CHANGE_RATE"),
                target_price: json_f64_opt(v, "TARGET_PRICE"),
                rating: json_str_opt(v, "RATING_NAME"),
                report_title: json_str_opt(v, "REPORT_TITLE"),
                publish_date: json_str_opt(v, "PUBLISH_DATE"),
            })
            .collect())
    }
}
