//! Fund ranking data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FundExchangeRankItem, FundSnapshot};
use crate::util::parse_f64_safe;

impl AkShareClient {
    /// Fetch open-end fund rankings from Eastmoney.
    pub async fn fund_open_fund_rank(
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

        let now = chrono::Utc::now();
        let week_ago = now - chrono::Duration::days(7);
        let ed = now.format("%Y-%m-%d").to_string();
        let sd = week_ago.format("%Y-%m-%d").to_string();
        let pn = limit.max(1).to_string();

        let resp = crate::util::send_and_check(
            self.get("https://fund.eastmoney.com/data/rankhandler.aspx")
                .header("Referer", "https://fund.eastmoney.com/fundguzhi.html")
                .query(&[
                    ("op", "ph"),
                    ("dt", "kf"),
                    ("ft", ft),
                    ("rs", ""),
                    ("gs", "0"),
                    ("sc", "1nzf"),
                    ("st", "desc"),
                    ("sd", sd.as_str()),
                    ("ed", ed.as_str()),
                    ("qdii", ""),
                    ("tabSubtype", ",,,,,"),
                    ("pi", "1"),
                    ("pn", pn.as_str()),
                    ("dx", "1"),
                ]),
        )
        .await?;

        let text = resp.text().await.map_err(Error::from)?;
        if text.is_empty() {
            return Err(Error::upstream("fund rank: empty response"));
        }
        let json_start = text.find('{').unwrap_or(0);
        let json_end = text.rfind('}').map_or(text.len(), |i| i + 1);
        let json_str = &text[json_start..json_end];

        // The response is JS, not JSON — unquoted keys like `datas:`, `allRecords:`.
        // Extract just the `datas` array directly to avoid full JS-to-JSON conversion.
        let datas_start = json_str
            .find("datas:")
            .map(|i| i + "datas:".len())
            .ok_or_else(|| {
                Error::decode(format!(
                    "fund rank missing datas field, json_str={}",
                    &json_str[..json_str.len().min(100)]
                ))
            })?;
        // Find matching ']' for the array
        let bracket_start = json_str[datas_start..]
            .find('[')
            .map(|i| datas_start + i)
            .ok_or_else(|| Error::decode("fund rank datas not an array"))?;
        let mut depth = 0i32;
        let mut bracket_end = bracket_start;
        for (i, c) in json_str[bracket_start..].char_indices() {
            match c {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        bracket_end = bracket_start + i;
                        break;
                    }
                }
                _ => {}
            }
        }
        let datas_str = &json_str[bracket_start..=bracket_end];
        let datas: Vec<serde_json::Value> = serde_json::from_str(datas_str)
            .map_err(|e| Error::decode(format!("fund rank datas parse: {e}")))?;

        let today = crate::util::today_iso();
        let snapshots: Vec<FundSnapshot> = datas
            .iter()
            .take(limit)
            .filter_map(|item| {
                // datas items can be CSV strings or JSON arrays
                let fields: Vec<&str> = if let Some(s) = item.as_str() {
                    s.split(',').collect()
                } else if let Some(arr) = item.as_array() {
                    arr.iter().map(|v| v.as_str().unwrap_or("")).collect()
                } else {
                    return None;
                };
                if fields.len() < 8 {
                    return None;
                }
                Some(FundSnapshot {
                    symbol: fields[0].to_string(),
                    name: fields[1].to_string(),
                    date: today.clone(),
                    nav: fields[3].parse().unwrap_or(0.0),
                    acc_nav: fields[4].parse().unwrap_or(0.0),
                    change_pct: fields[7].parse().unwrap_or(0.0),
                    fund_type: Some(symbol.to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found("no fund rank data"));
        }
        Ok(snapshots)
    }

    /// Fetch exchange fund ranking (Python: fund_exchange_rank).
    pub async fn fund_exchange_rank(&self) -> Result<Vec<FundExchangeRankItem>> {
        let response = crate::util::send_and_check(
            self.get("https://fund.eastmoney.com/data/rankhandler.aspx")
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
                ]),
        )
        .await?;

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
