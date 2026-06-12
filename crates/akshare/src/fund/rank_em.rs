//! Fund ranking data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FundExchangeRankItem, FundSnapshot};
use crate::util::parse_f64_safe;

impl AkShareClient {
    /// Fetch open-end fund rankings from Eastmoney.
    pub async fn fund_open_fund_rank_em(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<FundSnapshot>> {
        let type_map: &[(&str, &str)] = &[
            ("全部", "all"),
            ("股票型", "gp"),
            ("混合型", "hh"),
            ("债券型", "zq"),
            ("指数型", "zs"),
            ("QDII", "qdii"),
            ("LOF", "lof"),
            ("FOF", "fof"),
        ];
        let ft = type_map
            .iter()
            .find(|(n, _)| *n == symbol)
            .map_or("all", |(_, c)| *c);

        let now = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let pn = limit.max(1).to_string();

        let resp = self
            .get("https://fund.eastmoney.com/data/rankhandler.aspx")
            .header("Referer", "https://fund.eastmoney.com/fundguzhi.html")
            .query(&[
                ("op", "ph"),
                ("dt", "kf"),
                ("ft", ft),
                ("rs", ""),
                ("gs", "0"),
                ("sc", "1nzf"),
                ("st", "desc"),
                ("sd", now.as_str()),
                ("ed", now.as_str()),
                ("qdii", ""),
                ("tabSubtype", ",,,,,"),
                ("pi", "1"),
                ("pn", pn.as_str()),
                ("dx", "1"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = resp.text().await.map_err(Error::from)?;
        let json_start = text.find('{').unwrap_or(0);
        let json_end = text.rfind('}').map_or(text.len(), |i| i + 1);
        let json_str = &text[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("fund rank JSON parse: {e}")))?;

        let datas = root
            .get("datas")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("fund rank missing datas"))?;

        let today = crate::util::today_iso();
        let snapshots: Vec<FundSnapshot> = datas
            .iter()
            .take(limit)
            .filter_map(|item| {
                let arr = item.as_array()?;
                if arr.len() < 8 {
                    return None;
                }
                Some(FundSnapshot {
                    symbol: arr[0].as_str().unwrap_or("").to_string(),
                    name: arr[1].as_str().unwrap_or("").to_string(),
                    date: today.clone(),
                    nav: arr[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    acc_nav: arr[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    change_pct: arr[7].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    fund_type: Some(symbol.to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found("no fund rank data"));
        }
        Ok(snapshots)
    }

    /// Fetch exchange fund ranking (Python: fund_exchange_rank_em).
    pub async fn fund_exchange_rank_em(&self) -> Result<Vec<FundExchangeRankItem>> {
        let response = self
            .get("https://fund.eastmoney.com/data/rankhandler.aspx")
            .header("Referer", "https://fund.eastmoney.com/fundguzhi.html")
            .query(&[
                ("op", "ph"),
                ("dt", "fb"),
                ("ft", "ct"),
                ("rs", ""),
                ("gs", "0"),
                ("sc", "1nzf"),
                ("st", "desc"),
                ("pi", "1"),
                ("pn", "30000"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        let json_start = text.find('{').unwrap_or(0);
        let json_end = text.rfind('}').map_or(text.len(), |i| i + 1);
        let json_str = &text[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("exchange rank JSON parse: {e}")))?;

        let datas = root
            .get("datas")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("exchange rank missing datas"))?;

        let mut result = Vec::new();
        for (i, item) in datas.iter().enumerate() {
            let s = item.as_str().unwrap_or("");
            let fields: Vec<&str> = s.split(',').collect();
            if fields.len() < 23 {
                continue;
            }
            result.push(FundExchangeRankItem {
                rank: (i + 1) as i32,
                fund_code: fields[0].to_string(),
                fund_name: fields[1].to_string(),
                fund_type: fields[22].to_string(),
                date: fields[4].to_string(),
                nav: parse_f64_safe(fields[5]),
                acc_nav: parse_f64_safe(fields[6]),
                week_1: parse_f64_safe(fields[7]),
                month_1: parse_f64_safe(fields[8]),
                month_3: parse_f64_safe(fields[9]),
                month_6: parse_f64_safe(fields[10]),
                year_1: parse_f64_safe(fields[11]),
                year_2: parse_f64_safe(fields[12]),
                year_3: parse_f64_safe(fields[13]),
                ytd: parse_f64_safe(fields[14]),
                since_found: parse_f64_safe(fields[15]),
                found_date: fields[16].to_string(),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found("no exchange fund rank data"));
        }
        Ok(result)
    }
}
