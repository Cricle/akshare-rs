//! Block trade (大宗交易) data from Eastmoney.

use super::helpers::{json_f64, json_i64, json_str};
use crate::util::fmt_date;
use super::types::{DzjyHygtj, DzjyHyyybtj, DzjyMrtj, DzjyYybph};
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 大宗交易-每日统计
    pub async fn stock_dzjy_mrtj(&self, start_date: &str, end_date: &str) -> Result<Vec<DzjyMrtj>> {
        let sd = fmt_date(start_date);
        let ed = fmt_date(end_date);
        let filter = format!("(TRADE_DATE>='{sd}')(TRADE_DATE<='{ed}')");
        let data = self
            .dc_fetch_all(
                "RPT_BLOCKTRADE_DAILYSTATISTIC",
                "ALL",
                &filter,
                "TRADE_DATE,SECURITY_CODE",
                "-1,1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| DzjyMrtj {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                trade_date: json_str(v, "TRADE_DATE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                turnover_rate: json_f64(v, "TURNOVERRATE"),
                block_trade_count: json_i64(v, "BLOCKTRADE_NUM"),
                block_trade_volume: json_f64(v, "BLOCKTRADE_VOLUME"),
                block_trade_amount: json_f64(v, "BLOCKTRADE_AMT"),
                block_trade_premium_ratio: json_f64(v, "PREMIUM_RATIO"),
                circulating_market_cap: json_f64(v, "FREE_MARKET_CAP"),
            })
            .collect())
    }

    /// 大宗交易-行业统计
    pub async fn stock_dzjy_hygtj(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<DzjyHygtj>> {
        let sd = fmt_date(start_date);
        let ed = fmt_date(end_date);
        let filter = format!("(TRADE_DATE>='{sd}')(TRADE_DATE<='{ed}')");
        let data = self
            .dc_fetch_all(
                "RPT_BLOCKTRADE_INDUSTRYSTATISTIC",
                "ALL",
                &filter,
                "BLOCKTRADE_AMT",
                "-1",
                500,
                5,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| DzjyHygtj {
                industry: json_str(v, "INDUSTRY"),
                block_trade_count: json_i64(v, "BLOCKTRADE_NUM"),
                block_trade_volume: json_f64(v, "BLOCKTRADE_VOLUME"),
                block_trade_amount: json_f64(v, "BLOCKTRADE_AMT"),
                avg_premium_ratio: json_f64(v, "PREMIUM_RATIO"),
            })
            .collect())
    }

    /// 大宗交易-行业每日统计
    pub async fn stock_dzjy_hyyybtj(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<DzjyHyyybtj>> {
        let sd = fmt_date(start_date);
        let ed = fmt_date(end_date);
        let filter = format!("(TRADE_DATE>='{sd}')(TRADE_DATE<='{ed}')");
        let data = self
            .dc_fetch_all(
                "RPT_BLOCKTRADE_INDUSTRYDAILY",
                "ALL",
                &filter,
                "TRADE_DATE,INDUSTRY",
                "-1,1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| DzjyHyyybtj {
                industry: json_str(v, "INDUSTRY"),
                trade_date: json_str(v, "TRADE_DATE"),
                block_trade_count: json_i64(v, "BLOCKTRADE_NUM"),
                block_trade_volume: json_f64(v, "BLOCKTRADE_VOLUME"),
                block_trade_amount: json_f64(v, "BLOCKTRADE_AMT"),
                avg_premium_ratio: json_f64(v, "PREMIUM_RATIO"),
            })
            .collect())
    }

    /// 大宗交易-营业部排行
    pub async fn stock_dzjy_yybph(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<DzjyYybph>> {
        let sd = fmt_date(start_date);
        let ed = fmt_date(end_date);
        let filter = format!("(TRADE_DATE>='{sd}')(TRADE_DATE<='{ed}')");
        let data = self
            .dc_fetch_all(
                "RPT_BLOCKTRADE_DEPTSTATISTIC",
                "ALL",
                &filter,
                "BUY_AMT",
                "-1",
                500,
                5,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| DzjyYybph {
                dept_name: json_str(v, "OPERATEDEPT_NAME"),
                buy_count: json_i64(v, "BUY_NUM"),
                buy_amount: json_f64(v, "BUY_AMT"),
                sell_count: json_i64(v, "SELL_NUM"),
                sell_amount: json_f64(v, "SELL_AMT"),
                net_amount: json_f64(v, "NET_AMT"),
            })
            .collect())
    }
}
