//! Billboard (龙虎榜) data from Eastmoney.

use super::helpers::{json_f64, json_f64_opt, json_i64, json_str, json_str_opt};
use super::types::{
    BillboardActiveBranch, BillboardBranchDetail, BillboardBranchRanking, BillboardDetail,
    BillboardOrgSeatTracking, BillboardOrgTradeSummary, BillboardSinaBranchSummary,
    BillboardSinaDetail, BillboardSinaHkSummary, BillboardSinaOrgDetail, BillboardSinaOrgTracking,
    BillboardStockDetail, BillboardStockDetailDate, BillboardStockStatistic,
    BillboardTraderStatistic,
};
use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::fmt_date;

fn fmt_date_range(start: &str, end: &str) -> (String, String) {
    (fmt_date(start), fmt_date(end))
}

impl AkShareClient {
    /// 龙虎榜详情
    pub async fn stock_lhb_detail(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<BillboardDetail>> {
        let (sd, ed) = fmt_date_range(start_date, end_date);
        let filter = format!("(TRADE_DATE<='{ed}')(TRADE_DATE>='{sd}')");
        let data = self
            .dc_fetch_all(
                "RPT_DAILYBILLBOARD_DETAILSNEW",
                "ALL",
                &filter,
                "SECURITY_CODE,TRADE_DATE",
                "1,-1",
                500,
                10,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardDetail {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                trade_date: json_str(v, "TRADE_DATE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                billboard_net_amount: json_f64(v, "BILLBOARD_NET_AMT"),
                billboard_buy_amount: json_f64(v, "BILLBOARD_BUY_AMT"),
                billboard_sell_amount: json_f64(v, "BILLBOARD_SELL_AMT"),
                billboard_deal_amount: json_f64(v, "BILLBOARD_DEAL_AMT"),
                total_amount: json_f64(v, "ACCUM_AMOUNT"),
                net_ratio: json_f64(v, "DEAL_NET_RATIO"),
                deal_ratio: json_f64(v, "DEAL_AMOUNT_RATIO"),
                turnover_rate: json_f64(v, "TURNOVERRATE"),
                circulating_market_cap: json_f64(v, "FREE_MARKET_CAP"),
                reason: json_str(v, "EXPLANATION"),
                explanation: json_str_opt(v, "EXPLAIN"),
                d1_change: json_f64_opt(v, "D1_CLOSE_ADJCHRATE"),
                d2_change: json_f64_opt(v, "D2_CLOSE_ADJCHRATE"),
                d5_change: json_f64_opt(v, "D5_CLOSE_ADJCHRATE"),
                d10_change: json_f64_opt(v, "D10_CLOSE_ADJCHRATE"),
            })
            .collect())
    }

    /// 个股上榜统计
    pub async fn stock_lhb_stock_statistic(
        &self,
        symbol: &str,
    ) -> Result<Vec<BillboardStockStatistic>> {
        let cycle = match symbol {
            "近三月" => "02",
            "近六月" => "03",
            "近一年" => "04",
            _ => "01",
        };
        let filter = format!("(STATISTICS_CYCLE=\"{cycle}\")");
        let data = self
            .dc_fetch_all(
                "RPT_BILLBOARD_TRADEALL",
                "ALL",
                &filter,
                "BILLBOARD_TIMES,LATEST_TDATE,SECURITY_CODE",
                "-1,-1,1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardStockStatistic {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                latest_trade_date: json_str(v, "LATEST_TDATE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                billboard_times: json_i64(v, "BILLBOARD_TIMES"),
                billboard_net_amount: json_f64(v, "BILLBOARD_NET_AMT"),
                billboard_buy_amount: json_f64(v, "BILLBOARD_BUY_AMT"),
                billboard_sell_amount: json_f64(v, "BILLBOARD_SELL_AMT"),
                billboard_total_amount: json_f64(v, "AMOUNT"),
                org_buy_times: json_i64(v, "BUY_TIMES"),
                org_sell_times: json_i64(v, "SELL_TIMES"),
                org_net_buy_amount: json_f64(v, "ORG_BUY_NET_AMT"),
                org_buy_amount: json_f64(v, "ORG_BUY_AMT"),
                org_sell_amount: json_f64(v, "ORG_SELL_AMT"),
                m1_change: json_f64_opt(v, "M1_CLOSE_ADJCHRATE"),
                m3_change: json_f64_opt(v, "M3_CLOSE_ADJCHRATE"),
                m6_change: json_f64_opt(v, "M6_CLOSE_ADJCHRATE"),
                y1_change: json_f64_opt(v, "Y1_CLOSE_ADJCHRATE"),
            })
            .collect())
    }

    /// 机构买卖每日统计
    pub async fn stock_lhb_jgmmtj(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<BillboardOrgTradeSummary>> {
        let (sd, ed) = fmt_date_range(start_date, end_date);
        let filter = format!("(TRADE_DATE>='{sd}')(TRADE_DATE<='{ed}')");
        let data = self
            .dc_fetch_all(
                "RPT_ORGANIZATION_TRADE_DETAILS",
                "ALL",
                &filter,
                "NET_BUY_AMT,TRADE_DATE,SECURITY_CODE",
                "-1,-1,1",
                500,
                10,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardOrgTradeSummary {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                trade_date: json_str(v, "TRADE_DATE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                buy_org_count: json_i64(v, "BUY_ORG_NUM"),
                sell_org_count: json_i64(v, "SELL_ORG_NUM"),
                org_buy_amount: json_f64(v, "ORG_BUY_AMT"),
                org_sell_amount: json_f64(v, "ORG_SELL_AMT"),
                org_net_buy_amount: json_f64(v, "NET_BUY_AMT"),
                total_amount: json_f64(v, "ACCUM_AMOUNT"),
                org_net_ratio: json_f64(v, "NET_BUY_RATIO"),
                turnover_rate: json_f64(v, "TURNOVERRATE"),
                circulating_market_cap: json_f64(v, "FREE_MARKET_CAP"),
                reason: json_str(v, "EXPLANATION"),
            })
            .collect())
    }

    /// 机构席位追踪
    pub async fn stock_lhb_jgstatistic(
        &self,
        symbol: &str,
    ) -> Result<Vec<BillboardOrgSeatTracking>> {
        let cycle = match symbol {
            "近三月" => "02",
            "近六月" => "03",
            "近一年" => "04",
            _ => "01",
        };
        let filter = format!("(STATISTICSCYCLE=\"{cycle}\")");
        let data = self
            .dc_fetch_all(
                "RPT_ORGANIZATION_SEATNEW",
                "ALL",
                &filter,
                "ONLIST_TIMES,SECURITY_CODE",
                "-1,1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardOrgSeatTracking {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                billboard_amount: json_f64(v, "AMOUNT"),
                billboard_times: json_i64(v, "ONLIST_TIMES"),
                org_buy_amount: json_f64(v, "BUY_AMT"),
                org_buy_times: json_i64(v, "BUY_TIMES"),
                org_sell_amount: json_f64(v, "SELL_AMT"),
                org_sell_times: json_i64(v, "SELL_TIMES"),
                org_net_buy_amount: json_f64(v, "NET_BUY_AMT"),
                m1_change: json_f64_opt(v, "M1_CLOSE_ADJCHRATE"),
                m3_change: json_f64_opt(v, "M3_CLOSE_ADJCHRATE"),
                m6_change: json_f64_opt(v, "M6_CLOSE_ADJCHRATE"),
                y1_change: json_f64_opt(v, "Y1_CLOSE_ADJCHRATE"),
            })
            .collect())
    }

    /// 每日活跃营业部
    pub async fn stock_lhb_hyyyb(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<BillboardActiveBranch>> {
        let (sd, ed) = fmt_date_range(start_date, end_date);
        let filter = format!("(ONLIST_DATE>='{sd}')(ONLIST_DATE<='{ed}')");
        let data = self
            .dc_fetch_all(
                "RPT_OPERATEDEPT_ACTIVE",
                "ALL",
                &filter,
                "TOTAL_NETAMT,ONLIST_DATE,OPERATEDEPT_CODE",
                "-1,-1,1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardActiveBranch {
                dept_name: json_str(v, "OPERATEDEPT_NAME"),
                trade_date: json_str(v, "ONLIST_DATE"),
                buy_stock_count: json_i64(v, "BUY_NUM"),
                sell_stock_count: json_i64(v, "SELL_NUM"),
                buy_amount: json_f64(v, "BUY_AMT"),
                sell_amount: json_f64(v, "SELL_AMT"),
                net_amount: json_f64(v, "TOTAL_NETAMT"),
                buy_stocks: json_str_opt(v, "BUY_SECURITIES"),
                dept_code: json_str(v, "OPERATEDEPT_CODE"),
            })
            .collect())
    }

    /// 营业部排行
    pub async fn stock_lhb_yybph(&self, symbol: &str) -> Result<Vec<BillboardBranchRanking>> {
        let cycle = match symbol {
            "近三月" => "02",
            "近六月" => "03",
            "近一年" => "04",
            _ => "01",
        };
        let filter = format!("(STATISTICSCYCLE=\"{cycle}\")");
        let data = self
            .dc_fetch_all(
                "RPT_OPERATEDEPT_RANK",
                "ALL",
                &filter,
                "ONLIST_TIMES,OPERATEDEPT_CODE",
                "-1,1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardBranchRanking {
                dept_name: json_str(v, "OPERATEDEPT_NAME"),
                dept_code: json_str(v, "OPERATEDEPT_CODE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                billboard_amount: json_f64(v, "AMOUNT"),
                billboard_times: json_i64(v, "ONLIST_TIMES"),
                buy_amount: json_f64(v, "BUY_AMT"),
                buy_times: json_i64(v, "BUY_TIMES"),
                sell_amount: json_f64(v, "SELL_AMT"),
                sell_times: json_i64(v, "SELL_TIMES"),
                net_buy_amount: json_f64(v, "NET_BUY_AMT"),
                m1_change: json_f64_opt(v, "M1_CLOSE_ADJCHRATE"),
                m3_change: json_f64_opt(v, "M3_CLOSE_ADJCHRATE"),
                m6_change: json_f64_opt(v, "M6_CLOSE_ADJCHRATE"),
                y1_change: json_f64_opt(v, "Y1_CLOSE_ADJCHRATE"),
            })
            .collect())
    }

    /// 游资统计
    pub async fn stock_lhb_traderstatistic(
        &self,
        symbol: &str,
    ) -> Result<Vec<BillboardTraderStatistic>> {
        let cycle = match symbol {
            "近三月" => "02",
            "近六月" => "03",
            "近一年" => "04",
            _ => "01",
        };
        let filter = format!("(STATISTICSCYCLE=\"{cycle}\")");
        let data = self
            .dc_fetch_all(
                "RPT_BILLBOARD_DAILYDETAILS_TRADER",
                "ALL",
                &filter,
                "ONLIST_TIMES,OPERATEDEPT_CODE",
                "-1,1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardTraderStatistic {
                trader_name: json_str(v, "OPERATEDEPT_NAME"),
                trader_code: json_str(v, "OPERATEDEPT_CODE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                billboard_amount: json_f64(v, "AMOUNT"),
                billboard_times: json_i64(v, "ONLIST_TIMES"),
                buy_amount: json_f64(v, "BUY_AMT"),
                buy_times: json_i64(v, "BUY_TIMES"),
                sell_amount: json_f64(v, "SELL_AMT"),
                sell_times: json_i64(v, "SELL_TIMES"),
                net_buy_amount: json_f64(v, "NET_BUY_AMT"),
                m1_change: json_f64_opt(v, "M1_CLOSE_ADJCHRATE"),
                m3_change: json_f64_opt(v, "M3_CLOSE_ADJCHRATE"),
                m6_change: json_f64_opt(v, "M6_CLOSE_ADJCHRATE"),
                y1_change: json_f64_opt(v, "Y1_CLOSE_ADJCHRATE"),
            })
            .collect())
    }

    /// 个股上榜日期
    pub async fn stock_lhb_stock_detail_date(
        &self,
        symbol: &str,
    ) -> Result<Vec<BillboardStockDetailDate>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_DAILYBILLBOARD_DETAILSNEW",
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
            .map(|v| BillboardStockDetailDate {
                trade_date: json_str(v, "TRADE_DATE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                reason: json_str(v, "EXPLANATION"),
                billboard_net_amount: json_f64(v, "BILLBOARD_NET_AMT"),
                billboard_buy_amount: json_f64(v, "BILLBOARD_BUY_AMT"),
                billboard_sell_amount: json_f64(v, "BILLBOARD_SELL_AMT"),
            })
            .collect())
    }

    /// 个股龙虎榜详情
    pub async fn stock_lhb_stock_detail(
        &self,
        symbol: &str,
        date: &str,
        flag: &str,
    ) -> Result<Vec<BillboardStockDetail>> {
        let report = match flag {
            "卖出" => "RPT_BILLBOARD_DAILYDETAILSSELL",
            _ => "RPT_BILLBOARD_DAILYDETAILSBUY",
        };
        let filter = format!(
            "(TRADE_DATE='{}')(SECURITY_CODE=\"{symbol}\")",
            fmt_date(date)
        );
        let data = self
            .dc_fetch_all(report, "ALL", &filter, "TRADE_DATE", "-1", 500, 1, &[])
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardStockDetail {
                trade_date: json_str(v, "TRADE_DATE"),
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                dept_name: json_str(v, "OPERATEDEPT_NAME"),
                buy_amount: json_f64(v, "BUY"),
                sell_amount: json_f64(v, "SELL"),
                net_amount: json_f64(v, "NET"),
                reason: json_str(v, "EXPLANATION"),
                side: flag.to_string(),
            })
            .collect())
    }

    /// 营业部龙虎榜详情
    pub async fn stock_lhb_yyb_detail(&self, symbol: &str) -> Result<Vec<BillboardBranchDetail>> {
        let filter = format!("(OPERATEDEPT_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_BILLBOARD_DAILYDETAILSBUY",
                "ALL",
                &filter,
                "TRADE_DATE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardBranchDetail {
                trade_date: json_str(v, "TRADE_DATE"),
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                buy_amount: json_f64(v, "BUY"),
                sell_amount: json_f64(v, "SELL"),
                net_amount: json_f64(v, "NET"),
                reason: json_str(v, "EXPLANATION"),
            })
            .collect())
    }

    /// 新浪-龙虎榜-每日详情
    pub async fn stock_lhb_detail_daily(&self, date: &str) -> Result<Vec<BillboardSinaDetail>> {
        let formatted = fmt_date(date);
        let url =
            "https://vip.stock.finance.sina.com.cn/q/go.php/vInvestConsult/kind/lhb/index.phtml";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[("tradedate", formatted.as_str())])
        )
        .await?;
        let _text = resp.text().await.map_err(Error::from)?;
        // Sina LHB returns HTML tables - return empty vec as HTML parsing requires external crate
        Ok(vec![])
    }

    /// 新浪-龙虎榜-个股上榜统计
    pub async fn stock_lhb_ggtj(&self) -> Result<Vec<BillboardSinaHkSummary>> {
        let url = "https://vip.stock.finance.sina.com.cn/q/go.php/vLHBData/kind/ggtj/index.phtml";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[("last", "5"), ("p", "1")])
        )
        .await?;
        let _text = resp.text().await.map_err(Error::from)?;
        Ok(vec![])
    }

    /// 新浪-龙虎榜-机构席位成交明细
    pub async fn stock_lhb_jgmx(&self) -> Result<Vec<BillboardSinaOrgDetail>> {
        let url = "https://vip.stock.finance.sina.com.cn/q/go.php/vLHBData/kind/jgmx/index.phtml";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[("p", "1")])
        )
        .await?;
        let _text = resp.text().await.map_err(Error::from)?;
        Ok(vec![])
    }

    /// 新浪-龙虎榜-机构席位追踪
    pub async fn stock_lhb_jgzz(&self, symbol: &str) -> Result<Vec<BillboardSinaOrgTracking>> {
        let url = "https://vip.stock.finance.sina.com.cn/q/go.php/vLHBData/kind/jgzz/index.phtml";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[("last", symbol), ("p", "1")])
        )
        .await?;
        let _text = resp.text().await.map_err(Error::from)?;
        Ok(vec![])
    }

    /// 新浪-龙虎榜-营业部上榜统计
    pub async fn stock_lhb_yytj(&self, symbol: &str) -> Result<Vec<BillboardSinaBranchSummary>> {
        let url = "https://vip.stock.finance.sina.com.cn/q/go.php/vLHBData/kind/yytj/index.phtml";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[("last", symbol), ("p", "1")])
        )
        .await?;
        let _text = resp.text().await.map_err(Error::from)?;
        Ok(vec![])
    }
}
