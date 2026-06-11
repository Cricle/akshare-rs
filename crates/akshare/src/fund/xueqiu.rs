//! Fund data from Xueqiu (雪球) / Danjuan (蛋卷基金).

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{
    FundSnapshot, XqAchievementItem, XqAnalysisItem, XqBasicInfo, XqDetailHoldItem,
    XqProfitProbabilityItem,
};

impl AkShareClient {
    /// Fetch fund information from Danjuan API.
    pub async fn fund_xueqiu_info(&self, symbol: &str) -> Result<FundSnapshot> {
        let url = format!("https://danjuanfunds.com/djapi/fund/{symbol}");
        let response = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let root: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = root
            .get("data")
            .ok_or_else(|| Error::decode("xueqiu fund response missing data"))?;

        let fd_code = data
            .get("fd_code")
            .and_then(|v| v.as_str())
            .unwrap_or(symbol);
        let fd_name = data
            .get("fd_name")
            .and_then(|v| v.as_str())
            .unwrap_or("未知基金");
        let update_date = data
            .get("update_date")
            .or_else(|| data.get("found_date"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let nav = data
            .get("nav")
            .or_else(|| data.get("unit_nav"))
            .and_then(|v| v.as_f64())
            .or_else(|| {
                data.get("nav")
                    .or_else(|| data.get("unit_nav"))
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<f64>().ok())
            })
            .unwrap_or(0.0);
        let acc_nav = data
            .get("acc_nav")
            .and_then(|v| v.as_f64())
            .or_else(|| {
                data.get("acc_nav")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse().ok())
            })
            .unwrap_or(0.0);
        let change_pct = data
            .get("percent")
            .or_else(|| data.get("rzdf"))
            .and_then(|v| v.as_f64())
            .or_else(|| {
                data.get("percent")
                    .or_else(|| data.get("rzdf"))
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse().ok())
            })
            .unwrap_or(0.0);

        Ok(FundSnapshot {
            symbol: fd_code.to_string(),
            name: fd_name.to_string(),
            date: update_date,
            nav,
            acc_nav,
            change_pct,
            fund_type: Some("xueqiu".to_string()),
        })
    }

    /// Fetch fund performance/achievement data from Danjuan API.
    pub async fn fund_xueqiu_achievement(&self, symbol: &str) -> Result<serde_json::Value> {
        let url = format!("https://danjuanfunds.com/djapi/fundx/base/fund/achievement/{symbol}");
        let response = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let root: serde_json::Value = response.json().await.map_err(Error::from)?;
        root.get("data")
            .cloned()
            .ok_or_else(|| Error::decode("xueqiu achievement response missing data"))
    }

    /// Fetch fund basic info from Xueqiu (Python: fund_individual_basic_info_xq).
    pub async fn fund_individual_basic_info_xq(&self, symbol: &str) -> Result<Vec<XqBasicInfo>> {
        let url = format!("https://danjuanfunds.com/djapi/fund/{symbol}");
        let response = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let root: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = root
            .get("data")
            .ok_or_else(|| Error::decode("missing data"))?;

        let fields = [
            ("fd_code", "基金代码"),
            ("fd_name", "基金名称"),
            ("fd_full_name", "基金全称"),
            ("found_date", "成立时间"),
            ("totshare", "最新规模"),
            ("keeper_name", "基金公司"),
            ("manager_name", "基金经理"),
            ("trup_name", "托管银行"),
            ("type_desc", "基金类型"),
            ("rating_source", "评级机构"),
            ("rating_desc", "基金评级"),
            ("invest_orientation", "投资策略"),
            ("invest_target", "投资目标"),
            ("performance_bench_mark", "业绩比较基准"),
        ];

        let mut result = Vec::new();
        for (key, label) in &fields {
            let value = data
                .get(*key)
                .map(|v| {
                    if let Some(s) = v.as_str() {
                        s.to_string()
                    } else {
                        v.to_string()
                    }
                })
                .unwrap_or_default();
            result.push(XqBasicInfo {
                item: label.to_string(),
                value,
            });
        }
        Ok(result)
    }

    /// Fetch fund achievement from Xueqiu (Python: fund_individual_achievement_xq).
    pub async fn fund_individual_achievement_xq(
        &self,
        symbol: &str,
    ) -> Result<Vec<XqAchievementItem>> {
        let data = self.fund_xueqiu_achievement(symbol).await?;
        let mut result = Vec::new();

        for (key, type_name) in &[
            ("annual_performance_list", "年度业绩"),
            ("stage_performance_list", "阶段业绩"),
        ] {
            let list = data
                .get(*key)
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            for item in list {
                result.push(XqAchievementItem {
                    achievement_type: type_name.to_string(),
                    period: item
                        .get("period_time")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    return_pct: item
                        .get("self_nav")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .replace('%', "")
                        .parse()
                        .unwrap_or(0.0),
                    max_drawdown: item
                        .get("self_max_draw_down")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .replace('%', "")
                        .parse()
                        .unwrap_or(0.0),
                    rank: item
                        .get("self_nav_rank")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                });
            }
        }
        Ok(result)
    }

    /// Fetch fund analysis from Xueqiu (Python: fund_individual_analysis_xq).
    pub async fn fund_individual_analysis_xq(&self, symbol: &str) -> Result<Vec<XqAnalysisItem>> {
        let url =
            format!("https://danjuanfunds.com/djapi/fund/base/quote/data/index/analysis/{symbol}");
        let response = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let root: serde_json::Value = response.json().await.map_err(Error::from)?;
        let list = root
            .get("data")
            .and_then(|d| d.get("index_data_list"))
            .and_then(|l| l.as_array())
            .ok_or_else(|| Error::not_found(format!("no analysis data for {symbol}")))?;

        let mut result = Vec::new();
        for item in list {
            result.push(XqAnalysisItem {
                period: item
                    .get("index_time_period")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                risk_return_ratio: item
                    .get("investment_cost_performance")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                risk_control: item
                    .get("risk_control")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                volatility: item
                    .get("self_index")
                    .and_then(|s| s.get("volatility_rank"))
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                sharpe: item
                    .get("self_index")
                    .and_then(|s| s.get("sharpe_rank"))
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                max_drawdown: item
                    .get("self_index")
                    .and_then(|s| s.get("max_draw_down"))
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
            });
        }
        Ok(result)
    }

    /// Fetch profit probability from Xueqiu (Python: fund_individual_profit_probability_xq).
    pub async fn fund_individual_profit_probability_xq(
        &self,
        symbol: &str,
    ) -> Result<Vec<XqProfitProbabilityItem>> {
        let url = format!("https://danjuanfunds.com/djapi/fundx/base/fund/profit/ratio/{symbol}");
        let response = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let root: serde_json::Value = response.json().await.map_err(Error::from)?;
        let list = root
            .get("data")
            .and_then(|d| d.get("data_list"))
            .and_then(|l| l.as_array())
            .ok_or_else(|| Error::not_found(format!("no profit probability for {symbol}")))?;

        let mut result = Vec::new();
        for item in list {
            result.push(XqProfitProbabilityItem {
                holding_period: item
                    .get("holding_time")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                profit_ratio: item
                    .get("profit_ratio")
                    .and_then(|v| v.as_str())
                    .unwrap_or("0")
                    .replace('%', "")
                    .parse()
                    .unwrap_or(0.0),
                avg_return: item
                    .get("average_income")
                    .and_then(|v| v.as_str())
                    .unwrap_or("0")
                    .replace('%', "")
                    .parse()
                    .unwrap_or(0.0),
            });
        }
        Ok(result)
    }

    /// Fetch fund detail info (trading rules) from Xueqiu (Python: fund_individual_detail_info_xq).
    pub async fn fund_individual_detail_info_xq(&self, symbol: &str) -> Result<serde_json::Value> {
        let url = format!("https://danjuanfunds.com/djapi/fund/detail/{symbol}");
        let response = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let root: serde_json::Value = response.json().await.map_err(Error::from)?;
        root.get("data")
            .cloned()
            .ok_or_else(|| Error::not_found(format!("no detail info for {symbol}")))
    }

    /// Fetch fund detail holdings from Xueqiu (Python: fund_individual_detail_hold_xq).
    pub async fn fund_individual_detail_hold_xq(
        &self,
        symbol: &str,
        date: &str,
    ) -> Result<Vec<XqDetailHoldItem>> {
        let date_fmt = format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8]);
        let url = "https://danjuanfunds.com/djapi/fundx/base/fund/record/asset/percent";
        let response = self
            .get(url)
            .query(&[("fund_code", symbol), ("report_date", date_fmt.as_str())])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let root: serde_json::Value = response.json().await.map_err(Error::from)?;
        let list = root
            .get("data")
            .and_then(|d| d.get("chart_list"))
            .and_then(|l| l.as_array())
            .ok_or_else(|| Error::not_found(format!("no hold data for {symbol}")))?;

        let mut result = Vec::new();
        for item in list {
            result.push(XqDetailHoldItem {
                asset_type: item
                    .get("type_desc")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                percent: item.get("percent").and_then(|v| v.as_f64()).unwrap_or(0.0),
            });
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::types::FundSnapshot;

    #[test]
    fn test_fund_snapshot_xueqiu_fields() {
        let snap = FundSnapshot {
            symbol: "000001".to_string(),
            name: "test".to_string(),
            date: "2024-01-01".to_string(),
            nav: 1.5,
            acc_nav: 3.2,
            change_pct: -0.3,
            fund_type: Some("xueqiu".to_string()),
        };
        assert_eq!(snap.symbol, "000001");
    }
}
