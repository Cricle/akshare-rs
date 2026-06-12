//! HSGT (沪深港通) data from Eastmoney.

use super::helpers::{fmt_date, json_f64, json_f64_opt, json_i64, json_str, json_str_opt};
use super::types::{
    HsgtBoardRank, HsgtFundFlowSummary, HsgtHist, HsgtHoldStock, HsgtIndividualDetail,
    HsgtInstitutionStatistic, HsgtStockStatistic, SpotQuote,
};
use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// 沪深港通资金流向
    pub async fn stock_hsgt_fund_flow_summary_em(&self) -> Result<Vec<HsgtFundFlowSummary>> {
        let data = self.dc_fetch_all(
            "RPT_MUTUAL_QUOTA",
            "TRADE_DATE,MUTUAL_TYPE,BOARD_TYPE,MUTUAL_TYPE_NAME,FUNDS_DIRECTION,INDEX_CODE,INDEX_NAME,BOARD_CODE",
            "",
            "MUTUAL_TYPE",
            "1",
            2000, 1, &[],
        ).await?;

        Ok(data
            .iter()
            .map(|v| HsgtFundFlowSummary {
                trade_date: json_str(v, "TRADE_DATE"),
                mutual_type: json_str(v, "MUTUAL_TYPE_NAME"),
                board_type: json_str_opt(v, "BOARD_TYPE"),
                fund_direction: json_str_opt(v, "FUNDS_DIRECTION"),
                trade_status: None,
                net_buy_amount: None,
                fund_net_inflow: None,
                remaining_quota: None,
                up_count: None,
                flat_count: None,
                down_count: None,
                index_change_pct: None,
            })
            .collect())
    }

    /// 港股通成份股
    pub async fn stock_hk_ggt_components_em(&self) -> Result<Vec<SpotQuote>> {
        let items = self
            .clist_spot_fetch(
                "b:DLMK0146,b:DLMK0144",
                "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f24,f25",
                "5000",
                "f12",
            )
            .await?;

        Ok(items
            .iter()
            .filter_map(|v| {
                let code = v.get("f12")?.as_str()?.to_string();
                let name = v.get("f14")?.as_str()?.to_string();
                Some(SpotQuote {
                    code,
                    name,
                    latest_price: json_f64(v, "f2"),
                    change_pct: json_f64(v, "f3"),
                    change_amount: json_f64(v, "f4"),
                    volume: json_f64(v, "f5"),
                    amount: json_f64(v, "f6"),
                    amplitude_pct: json_f64(v, "f7"),
                    turnover_rate: json_f64(v, "f8"),
                    pe_dynamic: json_f64(v, "f9"),
                    volume_ratio: json_f64(v, "f10"),
                    high: json_f64(v, "f15"),
                    low: json_f64(v, "f16"),
                    open: json_f64(v, "f17"),
                    prev_close: json_f64(v, "f18"),
                    total_market_cap: json_f64(v, "f20"),
                    circulating_market_cap: json_f64(v, "f21"),
                    speed: json_f64(v, "f22"),
                    pb: json_f64(v, "f23"),
                    change_60d: json_f64(v, "f24"),
                    change_ytd: json_f64(v, "f25"),
                    change_5min: 0.0,
                })
            })
            .collect())
    }

    /// 沪深港通持股-个股排行
    pub async fn stock_hsgt_hold_stock_em(
        &self,
        market: &str,
        indicator: &str,
    ) -> Result<Vec<HsgtHoldStock>> {
        let indicator_type = match indicator {
            "今日排行" => "1",
            "3日排行" => "3",
            "5日排行" => "5",
            "10日排行" => "10",
            "月排行" => "M",
            "季排行" => "Q",
            "年排行" => "Y",
            _ => "1",
        };
        let mut filter = format!("(INTERVAL_TYPE=\"{indicator_type}\")");
        match market {
            "沪股通" => filter.push_str("(MUTUAL_TYPE=\"001\")"),
            "深股通" => filter.push_str("(MUTUAL_TYPE=\"003\")"),
            _ => {}
        }
        let data = self
            .dc_fetch_all(
                "RPT_MUTUAL_STOCK_NORTHSTA",
                "ALL",
                &filter,
                "ADD_MARKET_CAP",
                "-1",
                50000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| HsgtHoldStock {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                holding_shares: json_f64(v, "HOLD_SHARES"),
                holding_market_cap: json_f64(v, "HOLD_MARKET_CAP"),
                holding_circulating_ratio: json_f64(v, "A_SHARES_RATIO"),
                holding_total_ratio: json_f64(v, "TOTAL_SHARES_RATIO"),
                add_shares: json_f64_opt(v, "ADD_SHARES"),
                add_market_cap: json_f64_opt(v, "ADD_MARKET_CAP"),
                add_market_cap_change: json_f64_opt(v, "ADD_MARKET_CAP_CHR"),
                add_circulating_ratio: json_f64_opt(v, "ADD_SHARES_RATIO"),
                add_total_ratio: json_f64_opt(v, "ADD_SHARES_RATIO_TOTAL"),
                sector: json_str_opt(v, "BOARD_NAME"),
                trade_date: json_str(v, "TRADE_DATE"),
            })
            .collect())
    }

    /// 沪深港通持股-每日个股统计
    pub async fn stock_hsgt_stock_statistics_em(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HsgtStockStatistic>> {
        let (sd, ed) = (fmt_date(start_date), fmt_date(end_date));
        let report = if symbol == "南向持股" {
            "RPT_MUTUAL_STOCK_HOLDRANKS"
        } else {
            "RPT_MUTUAL_STOCK_NORTHSTA"
        };
        let filter = if sd == ed {
            format!("(INTERVAL_TYPE=\"1\")(RN=1)(TRADE_DATE='{sd}')")
        } else {
            format!("(INTERVAL_TYPE=\"1\")(RN=1)(TRADE_DATE>='{sd}')(TRADE_DATE<='{ed}')")
        };
        let data = self
            .dc_fetch_all(report, "ALL", &filter, "TRADE_DATE", "-1", 1000, 10, &[])
            .await?;

        Ok(data
            .iter()
            .map(|v| HsgtStockStatistic {
                trade_date: json_str(v, "TRADE_DATE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                code: json_str(v, "SECURITY_CODE"),
                holding_market_cap: json_f64(v, "HOLD_MARKET_CAP"),
                holding_shares: json_f64(v, "HOLD_SHARES"),
                holding_circulating_ratio: json_f64(v, "A_SHARES_RATIO"),
                holding_total_ratio: json_f64(v, "TOTAL_SHARES_RATIO"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
            })
            .collect())
    }

    /// 沪深港通持股-机构统计
    pub async fn stock_hsgt_institution_statistics_em(
        &self,
        _symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HsgtInstitutionStatistic>> {
        let (sd, ed) = (fmt_date(start_date), fmt_date(end_date));
        let filter = format!("(TRADE_DATE>='{sd}')(TRADE_DATE<='{ed}')");
        let data = self
            .dc_fetch_all(
                "RPT_MUTUAL_INSTITUTION",
                "ALL",
                &filter,
                "TRADE_DATE",
                "-1",
                5000,
                10,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| HsgtInstitutionStatistic {
                trade_date: json_str(v, "TRADE_DATE"),
                institution_name: json_str(v, "INSTITUTION_NAME"),
                holding_market_cap: json_f64(v, "HOLD_MARKET_CAP"),
                holding_shares: json_f64(v, "HOLD_SHARES"),
                holding_count: json_i64(v, "HOLD_NUM"),
            })
            .collect())
    }

    /// 沪深港通历史数据
    pub async fn stock_hsgt_hist_em(&self, _symbol: &str) -> Result<Vec<HsgtHist>> {
        let data = self.dc_fetch_all(
            "RPT_MUTUAL_QUOTA",
            "TRADE_DATE,MUTUAL_TYPE,BOARD_TYPE,MUTUAL_TYPE_NAME,FUNDS_DIRECTION,INDEX_CODE,INDEX_NAME,BOARD_CODE",
            "",
            "TRADE_DATE",
            "-1",
            5000, 10, &[],
        ).await?;

        Ok(data
            .iter()
            .map(|v| HsgtHist {
                trade_date: json_str(v, "TRADE_DATE"),
                fund_net_inflow: 0.0,
                fund_buy_amount: 0.0,
                fund_sell_amount: 0.0,
                index_close: 0.0,
                index_change_pct: 0.0,
            })
            .collect())
    }

    /// 沪深港通板块排行
    pub async fn stock_hsgt_board_rank_em(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HsgtBoardRank>> {
        let indicator_type = match indicator {
            "今日排行" => "1",
            "3日排行" => "3",
            "5日排行" => "5",
            "10日排行" => "10",
            "月排行" => "M",
            "季排行" => "Q",
            "年排行" => "Y",
            _ => "1",
        };
        let mut filter = format!("(INTERVAL_TYPE=\"{indicator_type}\")");
        match symbol {
            "沪股通" => filter.push_str("(MUTUAL_TYPE=\"001\")"),
            "深股通" => filter.push_str("(MUTUAL_TYPE=\"003\")"),
            _ => {}
        }
        let data = self
            .dc_fetch_all(
                "RPT_MUTUAL_BOARD_RANK",
                "ALL",
                &filter,
                "ADD_MARKET_CAP",
                "-1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| HsgtBoardRank {
                board_name: json_str(v, "BOARD_NAME"),
                holding_market_cap: json_f64(v, "HOLD_MARKET_CAP"),
                holding_shares: json_f64(v, "HOLD_SHARES"),
                holding_count: json_i64(v, "HOLD_NUM"),
                change_pct: json_f64_opt(v, "CHANGE_RATE"),
            })
            .collect())
    }

    /// 沪深港通个股详情
    pub async fn stock_hsgt_individual_detail_em(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HsgtIndividualDetail>> {
        let indicator_type = match indicator {
            "今日排行" => "1",
            "3日排行" => "3",
            "5日排行" => "5",
            "10日排行" => "10",
            _ => "1",
        };
        let filter = format!("(SECURITY_CODE=\"{symbol}\")(INTERVAL_TYPE=\"{indicator_type}\")");
        let data = self
            .dc_fetch_all(
                "RPT_MUTUAL_STOCK_NORTHSTA",
                "ALL",
                &filter,
                "TRADE_DATE",
                "-1",
                5000,
                10,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| HsgtIndividualDetail {
                trade_date: json_str(v, "TRADE_DATE"),
                holding_shares: json_f64(v, "HOLD_SHARES"),
                holding_market_cap: json_f64(v, "HOLD_MARKET_CAP"),
                holding_circulating_ratio: json_f64(v, "A_SHARES_RATIO"),
                holding_total_ratio: json_f64(v, "TOTAL_SHARES_RATIO"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
            })
            .collect())
    }

    /// 沪深港通资金流向-分钟数据 (Python: stock_hsgt_fund_min_em)
    pub async fn stock_hsgt_fund_min_em(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let market = match symbol {
            "沪股通" => "1",
            "深股通" => "2",
            "北向" => "3",
            "港股通沪" => "1",
            "港股通深" => "2",
            "南向" => "3",
            _ => "1",
        };
        let url = format!(
            "https://push2.eastmoney.com/api/qt/kamtbs.rtmin/get?fields1=f1,f2,f3,f4&fields2=f51,f52,f53,f54,f55,f56&ut=b2884a393a59ad64002292a3e90d46a5&klt={market}"
        );
        let resp = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let data: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let items = data
            .get("data")
            .and_then(|d| d.get("s2n"))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(items)
    }

    /// 沪深港通持股-个股 (Python: stock_hsgt_individual_em)
    pub async fn stock_hsgt_individual_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<HsgtIndividualDetail>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_MUTUAL_STOCK_NORTHSTA",
                "ALL",
                &filter,
                "TRADE_DATE",
                "-1",
                5000,
                10,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| HsgtIndividualDetail {
                trade_date: json_str(v, "TRADE_DATE"),
                holding_shares: json_f64(v, "HOLD_SHARES"),
                holding_market_cap: json_f64(v, "HOLD_MARKET_CAP"),
                holding_circulating_ratio: json_f64(v, "A_SHARES_RATIO"),
                holding_total_ratio: json_f64(v, "TOTAL_SHARES_RATIO"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
            })
            .collect())
    }
}
