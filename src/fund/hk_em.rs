//! Hong Kong fund data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundHkRankItem;

impl AkShareClient {
    /// Fetch HK fund ranking (Python: fund_hk_rank_em).
    pub async fn fund_hk_rank_em(&self) -> Result<Vec<FundHkRankItem>> {
        let format_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let response = self
            .get("https://overseas.1234567.com.cn/overseasapi/OpenApiHander.ashx")
            .header("Referer", "https://fund.eastmoney.com/fundguzhi.html")
            .query(&[
                ("api", "HKFDApi"),
                ("m", "MethodFundList"),
                ("action", "1"),
                ("pageindex", "0"),
                ("pagesize", "5000"),
                ("dy", "1"),
                ("date1", format_date.as_str()),
                ("date2", format_date.as_str()),
                ("sortfield", "Y"),
                ("sorttype", "-1"),
                ("isbuy", "0"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("Data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| Error::not_found("no HK fund rank data"))?;

        let mut result = Vec::new();
        for (i, item) in data.iter().enumerate() {
            let arr = match item.as_array() {
                Some(a) => a,
                None => continue,
            };
            if arr.len() < 20 {
                continue;
            }
            let can_buy = arr[6].as_str().unwrap_or("0");
            result.push(FundHkRankItem {
                rank: (i + 1) as i32,
                fund_code: arr[3].as_str().unwrap_or("").to_string(),
                fund_name: arr[5].as_str().unwrap_or("").to_string(),
                currency: arr[19].as_str().unwrap_or("").to_string(),
                date: arr[7].as_str().unwrap_or("").to_string(),
                nav: arr[8].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                change_pct: arr[9].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                week_1: arr[11].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                month_1: arr[12].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                month_3: arr[13].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                month_6: arr[14].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_1: arr[15].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_2: arr[16].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_3: arr[17].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                ytd: arr[18].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                since_found: arr[19].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                can_buy: if can_buy == "1" {
                    "可购买".to_string()
                } else {
                    "不可购买".to_string()
                },
                hk_fund_code: arr[2].as_str().unwrap_or("").to_string(),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found("no HK fund rank data"));
        }
        Ok(result)
    }

    /// Fetch HK fund historical NAV (Python: fund_hk_fund_hist_em).
    ///
    /// `code`: HK fund code (from fund_hk_rank_em).
    /// `symbol`: "历史净值明细" or "分红送配详情".
    pub async fn fund_hk_fund_hist_em(
        &self,
        code: &str,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let action = if symbol == "分红送配详情" {
            "3"
        } else {
            "2"
        };
        let response = self
            .get("https://overseas.1234567.com.cn/overseasapi/OpenApiHander.ashx")
            .header("Referer", "https://fund.eastmoney.com/fundguzhi.html")
            .query(&[
                ("api", "HKFDApi"),
                ("m", "MethodJZ"),
                ("hkfcode", code),
                ("action", action),
                ("pageindex", "0"),
                ("pagesize", "1000"),
                ("date1", ""),
                ("date2", ""),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("Data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| Error::not_found(format!("no HK fund data for {code}")))?;

        if data.is_empty() {
            return Err(Error::not_found(format!("no HK fund data for {code}")));
        }
        Ok(data.clone())
    }
}
