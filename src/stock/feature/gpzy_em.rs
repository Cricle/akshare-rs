//! Pledge data (股权质押) from Eastmoney.

use crate::client::AkShareClient;
use crate::error::Result;
use super::helpers::*;
use super::types::*;

impl AkShareClient {
    /// 股权质押市场概况
    pub async fn stock_gpzy_profile_em(&self) -> Result<Vec<GpzyProfile>> {
        let data = self.dc_fetch_all("RPT_CSDC_LIST", "ALL", "", "TRADE_DATE", "-1", 500, 2, &[]).await?;
        Ok(data.iter().map(|v| GpzyProfile {
            trade_date: json_str(v, "TRADE_DATE"),
            pledge_ratio: json_f64(v, "PLEDGE_RATIO"),
            company_count: json_i64(v, "COMPANY_COUNT"),
            pledge_count: json_i64(v, "PLEDGE_COUNT"),
            total_shares: json_f64(v, "TOTAL_SHARES"),
            total_market_cap: json_f64(v, "TOTAL_MARKET_CAP"),
            hs300_index: json_f64(v, "HS300_INDEX"),
            change_pct: json_f64(v, "CHANGE_PCT"),
        }).collect())
    }

    /// 上市公司质押比例
    pub async fn stock_gpzy_pledge_ratio_em(&self) -> Result<Vec<GpzyPledgeRatio>> {
        let data = self.dc_fetch_all("RPT_CSDC_LIST2", "ALL", "", "PLEDGE_RATIO", "-1", 500, 5, &[]).await?;
        Ok(data.iter().map(|v| GpzyPledgeRatio {
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            trade_date: json_str(v, "TRADE_DATE"),
            industry: json_str_opt(v, "INDUSTRY"),
            pledge_ratio: json_f64(v, "PLEDGE_RATIO"),
            pledge_shares: json_f64(v, "PLEDGE_SHARES"),
            pledge_market_cap: json_f64(v, "PLEDGE_MARKET_CAP"),
            pledge_count: json_i64(v, "PLEDGE_COUNT"),
            untradable_pledge: json_f64(v, "UNTRADABLE_PLEDGE"),
            tradable_pledge: json_f64(v, "TRADABLE_PLEDGE"),
            y1_change: json_f64_opt(v, "Y1_CHANGE"),
            industry_code: json_str_opt(v, "INDUSTRY_CODE"),
        }).collect())
    }

    /// 重要股东股权质押明细
    pub async fn stock_gpzy_pledge_detail_em(&self) -> Result<Vec<GpzyPledgeDetail>> {
        let data = self.dc_fetch_all("RPT_CSDC_DETAIL", "ALL", "", "NOTICE_DATE", "-1", 500, 5, &[]).await?;
        Ok(data.iter().map(|v| GpzyPledgeDetail {
            notice_date: json_str(v, "NOTICE_DATE"),
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            holder_name: json_str(v, "HOLDER_NAME"),
            pledge_shares: json_f64(v, "PLEDGE_SHARES"),
            pledge_ratio: json_f64(v, "PLEDGE_RATIO"),
            pledge_market_cap: json_f64(v, "PLEDGE_MARKET_CAP"),
            pledge_type: json_str_opt(v, "PLEDGE_TYPE"),
            receiver: json_str_opt(v, "RECEIVER"),
        }).collect())
    }

    /// 质押机构分布统计-银行
    pub async fn stock_gpzy_distribute_statistics_bank_em(&self) -> Result<Vec<GpzyDistributeEntry>> {
        let data = self.dc_fetch_all(
            "RPT_GDZY_ZYJG_SUM",
            "ALL",
            "(PFORG_TYPE=\"银行\")",
            "ORG_NUM",
            "-1",
            500,
            2,
            &[],
        ).await?;
        Ok(data.iter().enumerate().map(|(i, v)| GpzyDistributeEntry {
            rank: (i + 1) as i64,
            org_name: json_str(v, "PFORG_NAME"),
            company_count: json_i64(v, "COMPANY_NUM"),
            pledge_count: json_i64(v, "PLEDGE_NUM"),
            pledge_shares: json_f64(v, "PLEDGE_SHARES"),
            below_warning_ratio: json_f64_opt(v, "BELOW_WARNING_RATIO"),
            warning_not_sell_ratio: json_f64_opt(v, "WARNING_NOT_SELL_RATIO"),
            sell_line_ratio: json_f64_opt(v, "SELL_LINE_RATIO"),
        }).collect())
    }

    /// 质押机构分布统计-证券公司
    pub async fn stock_gpzy_distribute_statistics_company_em(&self) -> Result<Vec<GpzyDistributeEntry>> {
        let data = self.dc_fetch_all(
            "RPT_GDZY_ZYJG_SUM",
            "ALL",
            "(PFORG_TYPE=\"证券\")",
            "ORG_NUM",
            "-1",
            500,
            2,
            &[],
        ).await?;
        Ok(data.iter().enumerate().map(|(i, v)| GpzyDistributeEntry {
            rank: (i + 1) as i64,
            org_name: json_str(v, "PFORG_NAME"),
            company_count: json_i64(v, "COMPANY_NUM"),
            pledge_count: json_i64(v, "PLEDGE_NUM"),
            pledge_shares: json_f64(v, "PLEDGE_SHARES"),
            below_warning_ratio: json_f64_opt(v, "BELOW_WARNING_RATIO"),
            warning_not_sell_ratio: json_f64_opt(v, "WARNING_NOT_SELL_RATIO"),
            sell_line_ratio: json_f64_opt(v, "SELL_LINE_RATIO"),
        }).collect())
    }

    /// 个股股权质押明细
    pub async fn stock_gpzy_individual_pledge_ratio_detail_em(&self, symbol: &str) -> Result<Vec<GpzyPledgeRatioDetail>> {
        let filter = format!("(SECURITY_CODE=\"{}\")", symbol);
        let data = self.dc_fetch_all(
            "RPTA_APP_ACCUMDETAILS",
            "ALL",
            &filter,
            "NOTICE_DATE",
            "-1",
            500,
            10,
            &[],
        ).await?;
        Ok(data.iter().enumerate().map(|(i, v)| GpzyPledgeRatioDetail {
            rank: (i + 1) as i64,
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            holder_name: json_str(v, "HOLDER_NAME"),
            pledge_shares: json_f64(v, "PLEDGE_SHARES"),
            holding_ratio: json_f64(v, "HOLDING_RATIO"),
            total_ratio: json_f64(v, "TOTAL_RATIO"),
            org_name: json_str_opt(v, "ORG_NAME"),
            latest_price: json_f64_opt(v, "LATEST_PRICE"),
            pledge_close_price: json_f64_opt(v, "PLEDGE_CLOSE_PRICE"),
            estimated_sell_price: json_f64_opt(v, "ESTIMATED_SELL_PRICE"),
            start_date: json_str_opt(v, "START_DATE"),
            end_date: json_str_opt(v, "END_DATE"),
            status: json_str_opt(v, "STATUS"),
            notice_date: json_str(v, "NOTICE_DATE"),
        }).collect())
    }

    /// 行业质押数据
    pub async fn stock_gpzy_industry_data_em(&self) -> Result<Vec<GpzyIndustry>> {
        let data = self.dc_fetch_all(
            "RPT_CSDC_INDUSTRY_STATISTICS",
            "ALL",
            "",
            "AVERAGE_PLEDGE_RATIO",
            "-1",
            500,
            2,
            &[],
        ).await?;
        Ok(data.iter().map(|v| GpzyIndustry {
            industry: json_str(v, "INDUSTRY"),
            pledge_ratio: json_f64(v, "AVERAGE_PLEDGE_RATIO"),
            company_count: json_i64(v, "ORG_NUM"),
            pledge_count: json_i64(v, "PLEDGE_TOTAL_NUM"),
            total_shares: json_f64(v, "TOTAL_PLEDGE_SHARES"),
            total_market_cap: json_f64(v, "PLEDGE_TOTAL_MARKETCAP"),
        }).collect())
    }

    /// 重要股东股权质押明细（全部）
    pub async fn stock_gpzy_pledge_ratio_detail_em(&self) -> Result<Vec<GpzyPledgeRatioDetail>> {
        let data = self.dc_fetch_all(
            "RPTA_APP_ACCUMDETAILS",
            "ALL",
            "",
            "NOTICE_DATE",
            "-1",
            500,
            10,
            &[],
        ).await?;
        Ok(data.iter().enumerate().map(|(i, v)| GpzyPledgeRatioDetail {
            rank: (i + 1) as i64,
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            holder_name: json_str(v, "HOLDER_NAME"),
            pledge_shares: json_f64(v, "PLEDGE_SHARES"),
            holding_ratio: json_f64(v, "HOLDING_RATIO"),
            total_ratio: json_f64(v, "TOTAL_RATIO"),
            org_name: json_str_opt(v, "ORG_NAME"),
            latest_price: json_f64_opt(v, "LATEST_PRICE"),
            pledge_close_price: json_f64_opt(v, "PLEDGE_CLOSE_PRICE"),
            estimated_sell_price: json_f64_opt(v, "ESTIMATED_SELL_PRICE"),
            start_date: json_str_opt(v, "START_DATE"),
            end_date: json_str_opt(v, "END_DATE"),
            status: json_str_opt(v, "STATUS"),
            notice_date: json_str(v, "NOTICE_DATE"),
        }).collect())
    }
}
