//! Stock comments (千股千评) from Eastmoney.

use super::helpers::*;
use super::types::*;
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 千股千评
    pub async fn stock_comment_em(&self) -> Result<Vec<StockComment>> {
        let data = self
            .dc_fetch_all(
                "RPT_DMSK_TS_STOCKNEW",
                "ALL",
                "",
                "SECURITY_CODE",
                "1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| StockComment {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                latest_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                turnover_rate: json_f64(v, "TURNOVERRATE"),
                pe: json_f64(v, "PE_DYNAMIC"),
                main_cost: json_f64(v, "MAIN_COST"),
                org_participation: json_f64(v, "ORG_PARTICIPATE"),
                total_score: json_f64(v, "TOTAL_SCORE"),
                rise: json_f64(v, "RISE"),
                current_rank: json_f64(v, "CURRENT_RANK"),
                focus_index: json_f64(v, "FOCUS_INDEX"),
                trade_date: json_str(v, "TRADE_DATE"),
            })
            .collect())
    }

    /// 千股千评-主力控盘-机构参与度
    pub async fn stock_comment_detail_zlkp_jgcyd_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<CommentOrgParticipation>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_DMSK_TS_STOCKEVALUATE",
                "ALL",
                &filter,
                "TRADE_DATE",
                "-1",
                500,
                1,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| CommentOrgParticipation {
                trade_date: json_str(v, "TRADE_DATE"),
                org_participation: json_f64(v, "ORG_PARTICIPATE") * 100.0,
            })
            .collect())
    }

    /// 千股千评-综合评价-历史评分
    pub async fn stock_comment_detail_zhpj_lspf_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<CommentHistScore>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_STOCK_HISTORYMARK",
                "ALL",
                &filter,
                "DIAGNOSE_DATE",
                "1",
                500,
                1,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| CommentHistScore {
                trade_date: json_str(v, "DIAGNOSE_DATE"),
                score: json_f64(v, "TOTAL_SCORE"),
            })
            .collect())
    }

    /// 千股千评-市场热度-用户关注指数
    pub async fn stock_comment_detail_scrd_focus_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<CommentFocusIndex>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_STOCK_FOCUSSCORE",
                "ALL",
                &filter,
                "TRADE_DATE",
                "1",
                500,
                1,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| CommentFocusIndex {
                trade_date: json_str(v, "TRADE_DATE"),
                focus_index: json_f64(v, "FOCUS_INDEX"),
            })
            .collect())
    }

    /// 千股千评-市场热度-市场渴望指数
    pub async fn stock_comment_detail_scrd_desire_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<CommentDesireIndex>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_STOCK_DESIRESCORE",
                "ALL",
                &filter,
                "TRADE_DATE",
                "1",
                500,
                1,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| CommentDesireIndex {
                trade_date: json_str(v, "TRADE_DATE"),
                desire_index: json_f64(v, "DESIRE_INDEX"),
            })
            .collect())
    }
}
