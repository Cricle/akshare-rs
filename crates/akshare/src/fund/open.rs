//! Open-end fund NAV data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FundNavHistory, FundSnapshot};
use crate::util::parse_f64_safe;

impl AkShareClient {
    /// Fetch daily NAV snapshot for all open-end funds from Eastmoney.
    pub async fn fund_open_end_daily(&self, limit: usize) -> Result<Vec<FundSnapshot>> {
        let page = format!("1,{}", limit.max(1));
        let response = self
            .get("https://fund.eastmoney.com/Data/Fund_JJJZ_Data.aspx")
            .query(&[
                ("t", "1"),
                ("lx", "1"),
                ("letter", ""),
                ("gsid", ""),
                ("text", ""),
                ("sort", "zdf,desc"),
                ("page", page.as_str()),
                ("dt", "1580914040623"),
                ("atfc", ""),
                ("onlySale", "0"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        let json_str = text
            .strip_prefix("var db=")
            .ok_or_else(|| Error::decode("unexpected open-end fund response"))?;

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("open-end fund JSON parse: {e}")))?;

        let showday = root
            .get("showday")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("missing showday"))?;
        let date = showday
            .first()
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let datas = root
            .get("datas")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("missing datas"))?;

        let mut snapshots = Vec::new();
        for item in datas {
            let Some(row) = item.as_str() else {
                continue;
            };
            let fields: Vec<&str> = row.split(',').map(str::trim).collect();
            if fields.len() < 9 {
                continue;
            }
            snapshots.push(FundSnapshot {
                symbol: fields[0].to_string(),
                name: fields[1].to_string(),
                date: date.clone(),
                nav: parse_f64_safe(fields[3]),
                acc_nav: parse_f64_safe(fields[4]),
                change_pct: parse_f64_safe(fields[8]),
                fund_type: Some("open_end".to_string()),
            });
        }
        if snapshots.is_empty() {
            return Err(Error::not_found("no open-end fund data"));
        }
        Ok(snapshots)
    }

    /// Fetch NAV history for a specific open-end fund.
    pub async fn fund_open_end_nav(&self, symbol: &str, limit: usize) -> Result<Vec<FundSnapshot>> {
        let page = format!("1,{}", limit.max(1));
        let response = self
            .get("https://fund.eastmoney.com/Data/Fund_JJJZ_Data.aspx")
            .query(&[
                ("t", "1"),
                ("lx", "1"),
                ("letter", ""),
                ("gsid", ""),
                ("text", symbol),
                ("sort", "zdf,desc"),
                ("page", page.as_str()),
                ("dt", "1580914040623"),
                ("atfc", ""),
                ("onlySale", "0"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        let json_str = text
            .strip_prefix("var db=")
            .ok_or_else(|| Error::decode("unexpected response"))?;

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("JSON parse: {e}")))?;

        let showday = root
            .get("showday")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("missing showday"))?;
        let date = showday
            .first()
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let datas = root
            .get("datas")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("missing datas"))?;

        let mut snapshots = Vec::new();
        for item in datas {
            let Some(row) = item.as_str() else {
                continue;
            };
            let fields: Vec<&str> = row.split(',').map(str::trim).collect();
            if fields.len() < 9 {
                continue;
            }
            snapshots.push(FundSnapshot {
                symbol: fields[0].to_string(),
                name: fields[1].to_string(),
                date: date.clone(),
                nav: parse_f64_safe(fields[3]),
                acc_nav: parse_f64_safe(fields[4]),
                change_pct: parse_f64_safe(fields[8]),
                fund_type: Some("open_end".to_string()),
            });
        }
        if snapshots.is_empty() {
            return Err(Error::not_found(format!("no data for {symbol}")));
        }
        Ok(snapshots)
    }

    /// Fetch open-fund daily data (Python: fund_open_fund_daily).
    ///
    /// Returns all open-end fund NAV data for the current trading day.
    pub async fn fund_open_fund_daily(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .get("https://fund.eastmoney.com/Data/Fund_JJJZ_Data.aspx")
            .query(&[
                ("t", "1"),
                ("lx", "1"),
                ("letter", ""),
                ("gsid", ""),
                ("text", ""),
                ("sort", "zdf,desc"),
                ("page", "1,50000"),
                ("dt", "1580914040623"),
                ("atfc", ""),
                ("onlySale", "0"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        let json_str = text
            .strip_prefix("var db=")
            .ok_or_else(|| Error::decode("unexpected response"))?;

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
        for (i, item) in datas.iter().enumerate() {
            let Some(row) = item.as_str() else {
                continue;
            };
            let fields: Vec<&str> = row.split(',').map(str::trim).collect();
            if fields.len() < 9 {
                continue;
            }
            result.push(serde_json::json!({
                "rank": i + 1,
                "fund_code": fields[0],
                "fund_name": fields[1],
                &format!("{day0}-nav"): parse_f64_safe(fields[3]),
                &format!("{day0}-acc_nav"): parse_f64_safe(fields[4]),
                &format!("{day1}-nav"): parse_f64_safe(fields[5]),
                &format!("{day1}-acc_nav"): parse_f64_safe(fields[6]),
                "daily_change": parse_f64_safe(fields[7]),
                "daily_change_pct": parse_f64_safe(fields[8]),
            }));
        }
        if result.is_empty() {
            return Err(Error::not_found("no open fund daily data"));
        }
        Ok(result)
    }

    /// Fetch open-fund info (NAV history) from Eastmoney.
    ///
    /// `symbol`: fund code (e.g. "710001").
    /// `indicator`: "单位净值走势", "累计净值走势", etc.
    /// `period`: "1月", "3月", "6月", "1年", "3年", "5年", "今年来", "成立来".
    pub async fn fund_open_fund_info(
        &self,
        symbol: &str,
        _start_date: &str,
        _end_date: &str,
        indicator: &str,
    ) -> Result<Vec<FundNavHistory>> {
        // Use the lsjz API for NAV history
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
            .ok_or_else(|| Error::not_found(format!("no data for {symbol}")))?;

        let mut result = Vec::new();
        for item in list {
            let Some(arr) = item.as_array() else {
                continue;
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
                "no {indicator} data for {symbol}"
            )));
        }
        result.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::FundSnapshot;

    #[test]
    fn test_fund_snapshot_fields() {
        let snap = FundSnapshot {
            symbol: "000001".to_string(),
            name: "Test Fund".to_string(),
            date: "2024-01-01".to_string(),
            nav: 1.234,
            acc_nav: 2.345,
            change_pct: 0.5,
            fund_type: Some("open_end".to_string()),
        };
        assert_eq!(snap.symbol, "000001");
        assert!((snap.nav - 1.234).abs() < f64::EPSILON);
    }
}
