#![allow(dead_code)]
//! Open-end fund data from Eastmoney: purchase status, fund names, index info.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundSnapshot;

/// Wire type for fund search data (var r = [...] pattern).
#[derive(Debug, Deserialize)]
struct FundSearchItem {
    #[serde(rename = "0")]
    code: String,
    #[serde(rename = "2")]
    name: String,
    #[serde(rename = "3")]
    fund_type: String,
}

impl AkShareClient {
    /// Fetch fund purchase/redemption status from Eastmoney.
    ///
    /// Returns up to `limit` funds with their current purchase and redemption status.
    pub async fn fund_purchase(&self, limit: usize) -> Result<Vec<serde_json::Value>> {
        let page = format!("1,{}", limit.max(1));
        let resp = self
            .get("https://fund.eastmoney.com/Data/Fund_JJJZ_Data.aspx")
            .query(&[
                ("t", "8"),
                ("page", page.as_str()),
                ("js", "reData"),
                ("sort", "fcode,asc"),
            ])
            .header("Referer", "https://fund.eastmoney.com/")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = resp.text().await.map_err(Error::from)?;
        let json_str = text
            .strip_prefix("var reData=")
            .ok_or_else(|| Error::decode("unexpected fund purchase response format"))?;

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("fund purchase JSON parse: {e}")))?;

        let datas = root
            .get("datas")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("fund purchase response missing datas"))?
            .clone();

        if datas.is_empty() {
            return Err(Error::not_found("eastmoney returned no fund purchase data"));
        }
        Ok(datas)
    }

    /// Fetch all fund names and types from Eastmoney.
    ///
    /// Returns a list of fund code, name, and type for all available funds.
    pub async fn fund_name(&self) -> Result<Vec<serde_json::Value>> {
        let resp = self
            .get("https://fund.eastmoney.com/js/fundcode_search.js")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = resp.text().await.map_err(Error::from)?;
        let json_str = text
            .strip_prefix("var r = ")
            .and_then(|s| s.strip_suffix(';'))
            .ok_or_else(|| Error::decode("unexpected fund name JS format"))?;

        let data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("fund name JSON parse: {e}")))?;

        let items = data
            .as_array()
            .ok_or_else(|| Error::decode("fund name data is not an array"))?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no fund name data"));
        }
        Ok(items.clone())
    }

    /// Fetch index fund info from Eastmoney.
    ///
    /// `symbol` filters by index category (e.g. "沪深指数", "行业主题").
    /// `indicator` filters by fund type ("被动指数型" or "增强指数型").
    pub async fn fund_info_index(
        &self,
        symbol: &str,
        indicator: &str,
        limit: usize,
    ) -> Result<Vec<FundSnapshot>> {
        let symbol_map: &[(&str, &str)] = &[
            ("全部", ""),
            ("沪深指数", "053"),
            ("行业主题", "054"),
            ("大盘指数", "01"),
            ("中盘指数", "02"),
            ("小盘指数", "03"),
        ];
        let indicator_map: &[(&str, &str)] =
            &[("全部", ""), ("被动指数型", "051"), ("增强指数型", "052")];

        let s_code = symbol_map
            .iter()
            .find(|(n, _)| *n == symbol)
            .map_or("", |(_, c)| *c);
        let i_code = indicator_map
            .iter()
            .find(|(n, _)| *n == indicator)
            .map_or("", |(_, c)| *c);

        let pn = limit.max(1).to_string();
        let resp = self
            .get("https://api.fund.eastmoney.com/FundTradeRank/GetRankList")
            .query(&[
                ("ft", "zs"),
                ("sc", "1n"),
                ("st", "desc"),
                ("pi", "1"),
                ("pn", pn.as_str()),
                ("cp", ""),
                ("ct", ""),
                ("cd", ""),
                ("ms", ""),
                ("fr", ""),
                ("plevel", ""),
                ("fst", ""),
                ("ftype", ""),
                ("fr1", s_code),
                ("fr2", i_code),
                ("fl", "0"),
                ("is498", "1"),
            ])
            .header("Referer", "https://fund.eastmoney.com/")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = resp.text().await.map_err(Error::from)?;
        // Response is JS callback, extract JSON
        let json_start = text.find('{').unwrap_or(0);
        let json_end = text.rfind('}').map_or(text.len(), |i| i + 1);
        let json_str = &text[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("fund index info JSON parse: {e}")))?;

        let datas = root
            .get("datas")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("fund index info missing datas"))?;

        let today = crate::util::today_iso();
        let snapshots: Vec<FundSnapshot> = datas
            .iter()
            .take(limit)
            .filter_map(|item| {
                let arr = item.as_array()?;
                if arr.len() < 5 {
                    return None;
                }
                Some(FundSnapshot {
                    symbol: arr[0].as_str().unwrap_or("").to_string(),
                    name: arr[1].as_str().unwrap_or("").to_string(),
                    date: today.clone(),
                    nav: arr[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    acc_nav: arr[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    change_pct: 0.0,
                    fund_type: Some("index".to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found("no index fund data"));
        }
        Ok(snapshots)
    }
}
