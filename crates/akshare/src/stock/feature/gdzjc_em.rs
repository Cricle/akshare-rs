//! Shareholder changes (高管持股/股东增减持) from Eastmoney.

use super::helpers::{json_f64, json_f64_opt, json_str};
use super::types::Ggcg;
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 高管持股变动
    pub async fn stock_ggcg_em(&self, symbol: &str) -> Result<Vec<Ggcg>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_SHARECHANGE_EXECUTIVE",
                "ALL",
                &filter,
                "CHANGE_DATE",
                "-1",
                500,
                5,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| Ggcg {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                latest_price: json_f64_opt(v, "CLOSE_PRICE"),
                change_pct: json_f64_opt(v, "CHANGE_RATE"),
                holder_name: json_str(v, "PERSON_NAME"),
                direction: json_str(v, "CHANGE_DIRECTION"),
                change_amount: json_f64(v, "CHANGE_SHARES"),
                change_total_ratio: json_f64(v, "CHANGE_RATIO"),
                change_circulating_ratio: json_f64(v, "CHANGE_RATIO_CIRCULATING"),
                holding_count: json_f64(v, "HOLD_SHARES_AFTER"),
                holding_total_ratio: json_f64(v, "HOLD_RATIO_AFTER"),
                holding_circulating_count: json_f64(v, "HOLD_CIRCULATING_AFTER"),
                holding_circulating_ratio: json_f64(v, "HOLD_CIRCULATING_RATIO_AFTER"),
                start_date: json_str(v, "CHANGE_START_DATE"),
                end_date: json_str(v, "CHANGE_END_DATE"),
                notice_date: json_str(v, "NOTICE_DATE"),
            })
            .collect())
    }
}
