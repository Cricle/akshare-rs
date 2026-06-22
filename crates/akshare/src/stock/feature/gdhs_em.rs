//! Shareholder count (股东户数) from Eastmoney.

use super::helpers::{fmt_date, json_f64, json_f64_opt, json_str};
use super::types::{Gdhs, GdhsDetail};
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 股东户数
    pub async fn stock_gdhs(&self, date: &str) -> Result<Vec<Gdhs>> {
        let date_fmt = fmt_date(date);
        let filter = format!("(END_DATE='{date_fmt}')");
        let data = self
            .dc_fetch_all(
                "RPT_F10_EH_HOLDERSNUM",
                "ALL",
                &filter,
                "SECURITY_CODE",
                "1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| Gdhs {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                latest_price: json_f64_opt(v, "CLOSE_PRICE"),
                change_pct: json_f64_opt(v, "CHANGE_RATE"),
                holder_count: json_f64(v, "HOLDER_NUM"),
                prev_holder_count: json_f64(v, "HOLDER_NUM_CHANGE"),
                holder_change: json_f64(v, "HOLDER_NUM_RATIO"),
                holder_change_ratio: json_f64(v, "HOLDER_NUM_RATIO"),
                interval_change_pct: json_f64_opt(v, "INTERVAL_CHANGE_PCT"),
                current_end_date: json_str(v, "END_DATE"),
                prev_end_date: json_str(v, "PREV_END_DATE"),
                avg_market_cap: json_f64_opt(v, "AVG_MARKET_CAP"),
                avg_shares: json_f64_opt(v, "AVG_SHARES"),
                total_market_cap: json_f64_opt(v, "TOTAL_MARKET_CAP"),
                total_shares: json_f64_opt(v, "TOTAL_SHARES"),
                notice_date: json_str(v, "NOTICE_DATE"),
            })
            .collect())
    }

    /// 股东户数详情
    pub async fn stock_gdhs_detail(&self, symbol: &str) -> Result<Vec<GdhsDetail>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_F10_EH_HOLDERSNUM",
                "ALL",
                &filter,
                "END_DATE",
                "-1",
                500,
                2,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| GdhsDetail {
                end_date: json_str(v, "END_DATE"),
                holder_count: json_f64(v, "HOLDER_NUM"),
                prev_holder_count: json_f64(v, "HOLDER_NUM_CHANGE"),
                holder_change: json_f64(v, "HOLDER_NUM_RATIO"),
                holder_change_ratio: json_f64(v, "HOLDER_NUM_RATIO"),
                avg_market_cap: json_f64_opt(v, "AVG_MARKET_CAP"),
                avg_shares: json_f64_opt(v, "AVG_SHARES"),
                total_market_cap: json_f64_opt(v, "TOTAL_MARKET_CAP"),
                total_shares: json_f64_opt(v, "TOTAL_SHARES"),
                notice_date: json_str(v, "NOTICE_DATE"),
                interval_change_pct: json_f64_opt(v, "INTERVAL_CHANGE_PCT"),
            })
            .collect())
    }

    /// 东方财富-股东户数 (Python: stock_zh_a_gdhs)
    pub async fn stock_zh_a_gdhs(&self, symbol: &str) -> Result<Vec<Gdhs>> {
        let (report, filter) = if symbol == "最新" {
            ("RPT_HOLDERNUMLATEST", String::new())
        } else {
            ("RPT_HOLDERNUMLATEST", format!("(END_DATE='{symbol}')"))
        };
        let data = self
            .dc_fetch_all(
                report,
                "ALL",
                &filter,
                "HOLD_NOTICE_DATE,SECURITY_CODE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| Gdhs {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                latest_price: json_f64_opt(v, "CLOSE_PRICE"),
                change_pct: json_f64_opt(v, "CHANGE_RATE"),
                holder_count: json_f64(v, "HOLDER_NUM"),
                prev_holder_count: json_f64(v, "PRE_HOLDER_NUM"),
                holder_change: json_f64(v, "HOLDER_NUM_CHANGE"),
                holder_change_ratio: json_f64(v, "HOLDER_NUM_RATIO"),
                interval_change_pct: json_f64_opt(v, "INTERVAL_CHRATE"),
                current_end_date: json_str(v, "END_DATE"),
                prev_end_date: json_str(v, "PRE_END_DATE"),
                avg_market_cap: json_f64_opt(v, "AVG_MARKET_CAP"),
                avg_shares: json_f64_opt(v, "AVG_HOLD_NUM"),
                total_market_cap: json_f64_opt(v, "TOTAL_MARKET_CAP"),
                total_shares: json_f64_opt(v, "TOTAL_A_SHARES"),
                notice_date: json_str(v, "HOLD_NOTICE_DATE"),
            })
            .collect())
    }

    /// 东方财富-股东户数详情 (Python: stock_zh_a_gdhs_detail)
    pub async fn stock_zh_a_gdhs_detail(&self, symbol: &str) -> Result<Vec<GdhsDetail>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_HOLDERNUM_DET",
                "ALL",
                &filter,
                "END_DATE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| GdhsDetail {
                end_date: json_str(v, "END_DATE"),
                holder_count: json_f64(v, "HOLDER_NUM"),
                prev_holder_count: json_f64(v, "PRE_HOLDER_NUM"),
                holder_change: json_f64(v, "HOLDER_NUM_CHANGE"),
                holder_change_ratio: json_f64(v, "HOLDER_NUM_RATIO"),
                avg_market_cap: json_f64_opt(v, "AVG_MARKET_CAP"),
                avg_shares: json_f64_opt(v, "AVG_HOLD_NUM"),
                total_market_cap: json_f64_opt(v, "TOTAL_MARKET_CAP"),
                total_shares: json_f64_opt(v, "TOTAL_A_SHARES"),
                notice_date: json_str(v, "HOLD_NOTICE_DATE"),
                interval_change_pct: json_f64_opt(v, "INTERVAL_CHRATE"),
            })
            .collect())
    }
}
