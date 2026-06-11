//! IPO subscription yield (打新收益率) and new stock subscription (新股申购) from Eastmoney.

use crate::client::AkShareClient;
use crate::error::Result;
use super::helpers::*;
use super::types::*;

impl AkShareClient {
    /// 打新收益率
    pub async fn stock_dxsyl_em(&self) -> Result<Vec<Dxsyl>> {
        let filter = "((APPLY_DATE>'2010-01-01')(|@APPLY_DATE=\"NULL\"))((LISTING_DATE>'2010-01-01')(|@LISTING_DATE=\"NULL\"))(TRADE_MARKET_CODE!=\"069001017\")";
        let data = self.dc_fetch_all("RPTA_APP_IPOAPPLY", "ALL", filter, "LISTING_DATE,SECURITY_CODE", "-1", 5000, 5, &[]).await?;
        Ok(data.iter().map(|v| Dxsyl {
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "f14"),
            issue_price: json_f64(v, "ISSUE_PRICE"),
            latest_price: json_f64_opt(v, "LATELY_PRICE"),
            online_lottery_rate: json_f64(v, "ONLINE_ISSUE_LWR"),
            online_valid_shares: json_f64(v, "ONLINE_VA_SHARES"),
            online_valid_accounts: json_f64(v, "ONLINE_VA_NUM"),
            online_oversubscribe_ratio: json_f64(v, "ONLINE_ES_MULTIPLE"),
            offline_lottery_rate: json_f64(v, "OFFLINE_VAP_RATIO"),
            offline_valid_shares: json_f64(v, "OFFLINE_VATS"),
            offline_valid_accounts: json_f64(v, "OFFLINE_VAP_OBJECT"),
            offline_oversubscribe_ratio: json_f64(v, "OFFLINE_VAS_MULTIPLE"),
            total_issue_shares: json_f64(v, "ISSUE_NUM"),
            open_premium: json_f64(v, "LD_OPEN_PREMIUM"),
            first_day_change: json_f64(v, "LD_CLOSE_CHANGE"),
            listing_date: json_str_opt(v, "LISTING_DATE"),
        }).collect())
    }

    /// 新股申购与中签查询
    pub async fn stock_xgsglb_em(&self, market: &str) -> Result<Vec<Xgsglb>> {
        let filter = match market {
            "沪市主板" => "(APPLY_DATE>'2010-01-01')(SECURITY_TYPE_CODE in (\"058001001\",\"058001008\"))(TRADE_MARKET_CODE in (\"069001001001\",\"069001001003\",\"069001001006\"))",
            "科创板" => "(APPLY_DATE>'2010-01-01')(SECURITY_TYPE_CODE in (\"058001001\",\"058001008\"))(TRADE_MARKET_CODE=\"069001001006\")",
            "深市主板" => "(APPLY_DATE>'2010-01-01')(SECURITY_TYPE_CODE=\"058001001\")(TRADE_MARKET_CODE in (\"069001002001\",\"069001002002\",\"069001002003\",\"069001002005\"))",
            "创业板" => "(APPLY_DATE>'2010-01-01')(SECURITY_TYPE_CODE=\"058001001\")(TRADE_MARKET_CODE=\"069001002002\")",
            _ => "(APPLY_DATE>'2010-01-01')",
        };
        let data = self.dc_fetch_all("RPTA_APP_IPOAPPLY", "ALL", filter, "APPLY_DATE", "-1", 500, 10, &[]).await?;
        Ok(data.iter().map(|v| Xgsglb {
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            apply_code: json_str_opt(v, "APPLY_CODE"),
            issue_price: json_f64_opt(v, "ISSUE_PRICE"),
            latest_price: json_f64_opt(v, "LATELY_PRICE"),
            issue_pe_ratio: json_f64_opt(v, "ISSUE_PE_RATIO"),
            apply_date: json_str_opt(v, "APPLY_DATE"),
            listing_date: json_str_opt(v, "LISTING_DATE"),
            online_lottery_rate: json_f64_opt(v, "ONLINE_ISSUE_LWR"),
            first_day_close: json_f64_opt(v, "CLOSE_PRICE"),
            first_day_change: json_f64_opt(v, "LD_CLOSE_CHANGE"),
            first_day_turnover: json_f64_opt(v, "TURNOVERRATE"),
            main_business: json_str_opt(v, "MAIN_BUSINESS"),
        }).collect())
    }
}
