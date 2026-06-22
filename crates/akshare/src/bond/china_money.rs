#![allow(dead_code)]
//! ChinaMoney (chinamoney.com.cn) bond data.
//!
//! Close yield curves, swap rates, and bond issuance info.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;

#[derive(Debug, Deserialize)]
struct ChinaMoneyResp {
    records: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct ChinaMoneyPagedResp {
    records: Option<Vec<serde_json::Value>>,
    data: Option<ChinaMoneyPageData>,
}

#[derive(Debug, Deserialize)]
struct ChinaMoneyPageData {
    #[serde(rename = "pageTotalSize")]
    page_total_size: Option<u32>,
}

/// Symbol code mappings for bond yield curve types.
const BOND_TYPE_MAP: &[(&str, &str)] = &[
    ("国债", "CYCC000"),
    ("央行票据", "CYCC001"),
    ("政策性金融债", "CYCC002"),
    ("商业银行债券", "CYCC003"),
    ("企业债券", "CYCC004"),
    ("中期票据", "CYCC005"),
    ("短期融资券", "CYCC006"),
    ("同业存单(AAA)", "CYCC013"),
];

/// List available bond type names for close yield curves.
#[must_use]
pub fn bond_china_close_return_types() -> Vec<&'static str> {
    BOND_TYPE_MAP.iter().map(|(name, _)| *name).collect()
}

impl AkShareClient {
    /// Fetch close yield curve historical data from ChinaMoney.
    ///
    /// `symbol` is the bond type (e.g. "国债", "同业存单(AAA)").
    /// `period` is the tenor interval in years: "0.1", "0.5", or "1".
    /// `start_date` and `end_date` are YYYYMMDD strings; the range should not exceed 1 month.
    pub async fn bond_china_close_return(
        &self,
        symbol: &str,
        period: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let symbol_code = BOND_TYPE_MAP
            .iter()
            .find(|(name, _)| *name == symbol)
            .map(|(_, code)| *code)
            .ok_or_else(|| Error::invalid_input(format!("unknown bond type: {symbol}")))?;

        let sd = format!(
            "{}-{}-{}",
            &start_date[..4],
            &start_date[4..6],
            &start_date[6..8]
        );
        let ed = format!("{}-{}-{}", &end_date[..4], &end_date[4..6], &end_date[6..8]);

        let resp: ChinaMoneyResp = self
            .get("https://www.chinamoney.com.cn/ags/ms/cm-u-bk-currency/ClsYldCurvHis")
            .query(&[
                ("lang", "CN"),
                ("reference", "1,2,3"),
                ("bondType", symbol_code),
                ("startDate", sd.as_str()),
                ("endDate", ed.as_str()),
                ("termId", period),
                ("pageNum", "1"),
                ("pageSize", "50"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let records = resp.records.unwrap_or_default();
        let items: Vec<MacroDataPoint> = records
            .into_iter()
            .filter_map(|v| {
                let date = v.str_field(&["newDateValue", "date"])?;
                let yield_val = v.f64_field(&["yield", "closeYield"])?;
                Some(MacroDataPoint {
                    date: date.get(..10).unwrap_or(date).to_string(),
                    value: yield_val,
                    name: symbol.to_string(),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(format!(
                "chinamoney returned no yield data for {symbol}"
            )));
        }
        Ok(items)
    }

    /// Fetch FR007 interest rate swap curve historical data from ChinaMoney.
    ///
    /// `start_date` and `end_date` are YYYYMMDD strings; the range should not exceed 1 month.
    /// Returns swap rates for various tenors (1M, 3M, 6M, 1Y, 2Y, 5Y, 10Y).
    pub async fn macro_china_swap_rate(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let sd = format!(
            "{}-{}-{}",
            &start_date[..4],
            &start_date[4..6],
            &start_date[6..8]
        );
        let ed = format!("{}-{}-{}", &end_date[..4], &end_date[4..6], &end_date[6..8]);

        let resp: ChinaMoneyResp = self
            .post("https://www.chinamoney.com.cn/ags/ms/cm-u-bk-shibor/IfccHis")
            .form(&[
                ("cfgItemType", "72"),
                ("interestRateType", "0"),
                ("startDate", sd.as_str()),
                ("endDate", ed.as_str()),
                ("bidAskType", ""),
                ("lang", "CN"),
                ("quoteTime", "全部"),
                ("pageSize", "5000"),
                ("pageNum", "1"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let records = resp.records.unwrap_or_default();
        if records.is_empty() {
            return Err(Error::not_found("chinamoney returned no swap rate data"));
        }
        Ok(records)
    }

    /// Fetch bond issuance info from ChinaMoney.
    ///
    /// Returns up to `limit` pages of bond issuance data.
    pub async fn macro_china_bond_public(&self, limit: u32) -> Result<Vec<serde_json::Value>> {
        let mut all_records = Vec::new();
        for page in 1..=limit {
            let resp: ChinaMoneyPagedResp = self
                .post("https://www.chinamoney.com.cn/ags/ms/cm-u-bond-an/bnBondEmit")
                .form(&[
                    ("enty", ""),
                    ("bondType", ""),
                    ("bondNameCode", ""),
                    ("leadUnderwriter", ""),
                    ("pageNo", &page.to_string()),
                    ("pageSize", "10"),
                    ("limit", "1"),
                ])
                .send()
                .await?
                .json()
                .await?;

            if let Some(records) = resp.records {
                if records.is_empty() {
                    break;
                }
                all_records.extend(records);
            } else {
                break;
            }
        }

        if all_records.is_empty() {
            return Err(Error::not_found(
                "chinamoney returned no bond issuance data",
            ));
        }
        Ok(all_records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_type_map() {
        assert_eq!(BOND_TYPE_MAP.len(), 8);
        assert_eq!(
            BOND_TYPE_MAP.iter().find(|(n, _)| *n == "国债").unwrap().1,
            "CYCC000"
        );
    }

    #[test]
    fn test_close_return_types() {
        let types = bond_china_close_return_types();
        assert!(types.contains(&"国债"));
        assert!(types.contains(&"同业存单(AAA)"));
    }
}
