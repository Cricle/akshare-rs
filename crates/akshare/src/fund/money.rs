//! Money market fund data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FundMoneyRankItem, FundNavHistory, FundSnapshot};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HbRankEnvelope {
    #[serde(rename = "Datas")]
    datas: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ErrCode")]
    err_code: Option<i64>,
    #[serde(rename = "ErrMsg")]
    err_msg: Option<String>,
}

impl AkShareClient {
    /// Fetch money market fund rankings from Eastmoney.
    pub async fn fund_money_market(&self, limit: usize) -> Result<Vec<FundSnapshot>> {
        let pn = limit.max(1).to_string();
        let response = self
            .get("https://api.fund.eastmoney.com/FundRank/GetHbRankList")
            .query(&[
                ("FundType", "0"),
                ("SortColumn", "SYL_7"),
                ("Sort", "desc"),
                ("pageIndex", "1"),
                ("pageSize", pn.as_str()),
                ("IsSale", "1"),
            ])
            .header("Referer", "https://fund.eastmoney.com/")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: HbRankEnvelope = response.json().await.map_err(Error::from)?;
        if let Some(code) = payload.err_code
            && code != 0
        {
            let msg = payload.err_msg.unwrap_or_else(|| "unknown".to_string());
            return Err(Error::upstream(format!(
                "money fund API error {code}: {msg}"
            )));
        }

        let items = payload.datas.unwrap_or_default();
        let snapshots: Vec<FundSnapshot> = items
            .into_iter()
            .filter_map(|v| {
                let symbol = v.get("FCODE")?.as_str()?.to_string();
                let name = v
                    .get("SHORTNAME")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string();
                let date = v
                    .get("PDATE")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string();
                Some(FundSnapshot {
                    symbol,
                    name,
                    date,
                    nav: v
                        .get("DWJZ")
                        .and_then(|x| x.as_str())
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0.0),
                    acc_nav: v
                        .get("LJJZ")
                        .and_then(|x| x.as_str())
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0.0),
                    change_pct: v
                        .get("RZDF")
                        .and_then(|x| x.as_str())
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0.0),
                    fund_type: Some("money_market".to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found("no money market fund data"));
        }
        Ok(snapshots)
    }

    /// Fetch money fund daily data (Python: fund_money_fund_daily).
    pub async fn fund_money_fund_daily(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .get("https://fund.eastmoney.com/HBJJ_pjsyl.html")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        // This endpoint returns HTML; we need to parse the table.
        // Return raw HTML extraction attempt or error.
        if text.is_empty() {
            return Err(Error::not_found("no money fund daily data"));
        }
        // Extract data from HTML table using simple parsing
        // The HTML contains a table with fund data; we return a placeholder error
        // since full HTML parsing requires a dedicated library.
        Err(Error::decode(
            "money fund daily data requires HTML table parsing",
        ))
    }

    /// Fetch money fund info (historical NAV) from Eastmoney.
    ///
    /// `symbol`: money fund code (e.g. "000009").
    pub async fn fund_money_fund_info(&self, symbol: &str) -> Result<Vec<FundNavHistory>> {
        let response = self
            .get("https://api.fund.eastmoney.com/f10/lsjz")
            .header(
                "Referer",
                format!("https://fundf10.eastmoney.com/jjjz_{symbol}.html"),
            )
            .query(&[
                ("fundCode", symbol),
                ("pageIndex", "1"),
                ("pageSize", "10000"),
                ("startDate", ""),
                ("endDate", ""),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let list = payload
            .get("Data")
            .and_then(|d| d.get("LSJZList"))
            .and_then(|l| l.as_array())
            .ok_or_else(|| Error::not_found(format!("no money fund data for {symbol}")))?;

        let mut result = Vec::new();
        for item in list {
            let Some(arr) = item.as_array() else { continue };
            if arr.len() < 10 {
                continue;
            }
            result.push(FundNavHistory {
                date: arr[0].as_str().unwrap_or("").to_string(),
                nav: arr[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                acc_nav: arr[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                change_pct: 0.0,
                subscribe_status: arr[7].as_str().unwrap_or("").to_string(),
                redeem_status: arr[8].as_str().unwrap_or("").to_string(),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found(format!("no money fund data for {symbol}")));
        }
        result.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(result)
    }

    /// Fetch money fund ranking (Python: fund_money_rank).
    pub async fn fund_money_rank(&self) -> Result<Vec<FundMoneyRankItem>> {
        let response = self
            .get("https://api.fund.eastmoney.com/FundRank/GetHbRankList")
            .query(&[
                ("intCompany", "0"),
                ("MinsgType", ""),
                ("IsSale", "1"),
                ("strSortCol", "SYL_1N"),
                ("orderType", "desc"),
                ("pageIndex", "1"),
                ("pageSize", "10000"),
            ])
            .header("Referer", "https://fund.eastmoney.com/fundguzhi.html")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("Data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| Error::not_found("no money fund rank data"))?;

        let mut result = Vec::new();
        for (i, item) in data.iter().enumerate() {
            let Some(arr) = item.as_array() else { continue };
            if arr.len() < 20 {
                continue;
            }
            result.push(FundMoneyRankItem {
                rank: (i + 1) as i32,
                fund_code: arr[7].as_str().unwrap_or("").to_string(),
                fund_name: arr[8].as_str().unwrap_or("").to_string(),
                date: arr[9].as_str().unwrap_or("").to_string(),
                yield_per_10k: arr[10].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                annualized_7d: arr[11].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                annualized_14d: arr[13].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                annualized_28d: arr[14].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                month_1: arr[15].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                month_3: arr[16].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                month_6: arr[17].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_1: arr[0].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_2: arr[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_3: arr[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_5: arr[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                ytd: arr[18].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                since_found: arr[19].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found("no money fund rank data"));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_hb_rank_envelope_empty() {
        let json = r#"{"Datas": [], "ErrCode": 0}"#;
        let env: HbRankEnvelope = serde_json::from_str(json).unwrap();
        assert!(env.datas.unwrap().is_empty());
    }
}
