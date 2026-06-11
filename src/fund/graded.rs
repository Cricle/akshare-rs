//! Graded fund (分级基金) data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FundNavHistory, FundSnapshot};
use crate::util::{parse_f64_safe, today_iso};

impl AkShareClient {
    /// Fetch graded fund daily snapshot from Eastmoney.
    pub async fn fund_graded(&self, limit: usize) -> Result<Vec<FundSnapshot>> {
        let pz = limit.max(1).to_string();
        let response = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", pz.as_str()),
                ("po", "1"),
                ("np", "1"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fs", "b:MK0021,b:MK0022,b:MK0023"),
                ("fields", "f12,f14,f2,f3"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let items = payload
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let date = today_iso();
        let snapshots: Vec<FundSnapshot> = items
            .into_iter()
            .filter_map(|v| {
                Some(FundSnapshot {
                    symbol: v.get("f12")?.as_str()?.to_string(),
                    name: v
                        .get("f14")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string(),
                    date: date.clone(),
                    nav: v.get("f2").and_then(|x| x.as_f64()).unwrap_or(0.0),
                    acc_nav: v.get("f2").and_then(|x| x.as_f64()).unwrap_or(0.0),
                    change_pct: v.get("f3").and_then(|x| x.as_f64()).unwrap_or(0.0),
                    fund_type: Some("graded".to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found("no graded fund data"));
        }
        Ok(snapshots)
    }

    /// Fetch graded fund daily data (Python: fund_graded_fund_daily_em).
    pub async fn fund_graded_fund_daily_em(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .get("https://fund.eastmoney.com/Data/Fund_JJJZ_Data.aspx")
            .header("Referer", "https://fund.eastmoney.com/fjjj.html")
            .query(&[
                ("t", "1"),
                ("lx", "9"),
                ("letter", ""),
                ("gsid", "0"),
                ("text", ""),
                ("sort", "zdf,desc"),
                ("page", "1,10000"),
                ("dt", "1580914040623"),
                ("atfc", ""),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        let json_str = text
            .strip_prefix("var db=")
            .ok_or_else(|| Error::decode("unexpected graded fund response"))?;

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("JSON parse: {e}")))?;

        let showday = root
            .get("showday")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("missing showday"))?;
        let day0 = showday.first().and_then(|v| v.as_str()).unwrap_or("");
        let day1 = showday.get(1).and_then(|v| v.as_str()).unwrap_or("");

        let datas = root
            .get("datas")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("missing datas"))?;

        let mut result = Vec::new();
        for item in datas {
            let row = match item.as_str() {
                Some(s) => s,
                None => continue,
            };
            let fields: Vec<&str> = row.split(',').map(str::trim).collect();
            if fields.len() < 11 {
                continue;
            }
            result.push(serde_json::json!({
                "fund_code": fields[0],
                "fund_name": fields[1],
                &format!("{day0}-nav"): parse_f64_safe(fields[3]),
                &format!("{day0}-acc_nav"): parse_f64_safe(fields[4]),
                &format!("{day1}-nav"): parse_f64_safe(fields[5]),
                &format!("{day1}-acc_nav"): parse_f64_safe(fields[6]),
                "daily_change": parse_f64_safe(fields[7]),
                "daily_change_pct": parse_f64_safe(fields[8]),
                "market_price": parse_f64_safe(fields[9]),
                "discount_rate": parse_f64_safe(fields[10]),
            }));
        }
        if result.is_empty() {
            return Err(Error::not_found("no graded fund daily data"));
        }
        Ok(result)
    }

    /// Fetch graded fund info (historical NAV) from Eastmoney.
    ///
    /// `symbol`: graded fund code (e.g. "150232").
    pub async fn fund_graded_fund_info_em(&self, symbol: &str) -> Result<Vec<FundNavHistory>> {
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
            .ok_or_else(|| Error::not_found(format!("no graded fund data for {symbol}")))?;

        let mut result = Vec::new();
        for item in list {
            let arr = match item.as_array() {
                Some(a) => a,
                None => continue,
            };
            if arr.len() < 10 {
                continue;
            }
            result.push(FundNavHistory {
                date: arr[0].as_str().unwrap_or("").to_string(),
                nav: arr[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                acc_nav: arr[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                change_pct: arr[6].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                subscribe_status: arr[7].as_str().unwrap_or("").to_string(),
                redeem_status: arr[8].as_str().unwrap_or("").to_string(),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found(format!(
                "no graded fund data for {symbol}"
            )));
        }
        result.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graded_fund_snapshot() {
        let snap = FundSnapshot {
            symbol: "150001".to_string(),
            name: "test".to_string(),
            date: "2024-01-01".to_string(),
            nav: 1.0,
            acc_nav: 1.0,
            change_pct: 0.0,
            fund_type: Some("graded".to_string()),
        };
        assert_eq!(snap.fund_type.unwrap(), "graded");
    }
}
