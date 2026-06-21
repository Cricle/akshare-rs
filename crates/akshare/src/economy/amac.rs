//! AMAC (China Securities Investment Fund Industry Association) fund industry data.
//!
//! Fetches fund industry AUM and statistics from the Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::wire::EmDatacenterResp;
use crate::types::{MacroDataPoint, Row};

/// Helper to fetch AMAC data from Eastmoney datacenter.
async fn fetch_amac_data(
    http: &reqwest::Client,
    report_name: &str,
) -> Result<Vec<serde_json::Value>> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
    let resp: EmDatacenterResp = http
        .get(url)
        .query(&[
            ("reportName", report_name),
            ("columns", "ALL"),
            ("pageNumber", "1"),
            ("pageSize", "2000"),
            ("sortTypes", "-1"),
            ("sortColumns", "REPORT_DATE"),
            ("source", "WEB"),
            ("client", "WEB"),
        ])
        .send()
        .await?
        .json()
        .await?;

    Ok(resp.result.map(|r| r.data).unwrap_or_default())
}

/// Helper to convert AMAC JSON rows to Row type.
fn amac_rows_to_rows(data: &[serde_json::Value]) -> Vec<Row> {
    data.iter()
        .map(|v| {
            let mut row = Row::new();
            let empty = serde_json::Map::new();
            for (k, val) in v.as_object().unwrap_or(&empty) {
                row.insert(k.clone(), val.clone());
            }
            row
        })
        .collect()
}

impl AkShareClient {
    /// AMAC fund industry statistics.
    ///
    /// Returns fund industry AUM, product counts, and manager statistics
    /// from the Eastmoney datacenter (sourced from AMAC disclosures).
    pub async fn economy_amac_stats(&self) -> Result<Vec<MacroDataPoint>> {
        let data = fetch_amac_data(&self.http, "RPT_FUND_INDUSTRY_STAT").await?;
        let mut items = Vec::with_capacity(data.len());
        for v in &data {
            let date = v
                .get("REPORT_DATE")
                .or_else(|| v.get("REPORT_PERIOD"))
                .or_else(|| v.get("DATE"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if date.is_empty() {
                continue;
            }
            let value = v
                .get("INDICATOR_VALUE")
                .or_else(|| v.get("VALUE"))
                .or_else(|| v.get("AUM"))
                .or_else(|| v.get("TOTAL_AUM"))
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0);
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value,
                name: "AMAC Fund Industry".to_string(),
            });
        }
        Ok(items)
    }

    /// AMAC manager info (私募基金管理人信息).
    ///
    /// Returns information about fund managers registered with AMAC.
    pub async fn amac_manager_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_MANAGER_INFO").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC manager classify info (私募基金管理人分类信息).
    pub async fn amac_manager_classify_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_MANAGER_CLASSIFY").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC manager cancelled info (已注销私募基金管理人信息).
    pub async fn amac_manager_cancelled_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_MANAGER_CANCELLED").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC fund info (私募基金产品信息).
    pub async fn amac_fund_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_FUND_INFO").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC fund ABS info (资产证券化产品信息).
    pub async fn amac_fund_abs(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_FUND_ABS").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC fund sub info (私募基金产品投资子基金信息).
    pub async fn amac_fund_sub_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_FUND_SUB").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC fund account info (基金专户管理人产品信息).
    pub async fn amac_fund_account_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_FUND_ACCOUNT").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC member info (会员机构信息).
    pub async fn amac_member_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_MEMBER_INFO").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC member sub info (会员机构子公司信息).
    pub async fn amac_member_sub_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_MEMBER_SUB").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC securities info (证券期货经营机构信息).
    pub async fn amac_securities_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_SECURITIES").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC futures info (期货经营机构信息).
    pub async fn amac_futures_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_FUTURES").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC AOIN info (从业人员信息).
    pub async fn amac_aoin_info(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_AOIN").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC person bond org list (债券从业人员机构列表).
    pub async fn amac_person_bond_org_list(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_PERSON_BOND_ORG").await?;
        Ok(amac_rows_to_rows(&data))
    }

    /// AMAC person fund org list (基金从业人员机构列表).
    pub async fn amac_person_fund_org_list(&self) -> Result<Vec<Row>> {
        let data = fetch_amac_data(&self.http, "RPT_AMAC_PERSON_FUND_ORG").await?;
        Ok(amac_rows_to_rows(&data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_economy_amac_stats_response_structure() {
        let json = r#"{
            "result": {
                "data": [
                    {"REPORT_DATE": "2024-03-31T00:00:00", "INDICATOR_VALUE": 275800.0},
                    {"REPORT_DATE": "2023-12-31T00:00:00", "INDICATOR_VALUE": 268900.0}
                ]
            }
        }"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert_eq!(data.len(), 2);
        assert_eq!(
            data[0]
                .get("INDICATOR_VALUE")
                .and_then(serde_json::Value::as_f64),
            Some(275_800.0)
        );
    }

    #[test]
    fn test_economy_amac_stats_empty_response() {
        let json = r#"{"result": {"data": []}}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert!(data.is_empty());
    }

    #[test]
    fn test_economy_amac_stats_null_result() {
        let json = r#"{"result": null}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.map(|r| r.data).unwrap_or_default();
        assert!(data.is_empty());
    }
}
