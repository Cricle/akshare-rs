//! Shared helpers for fetching macro-economic data from Eastmoney and Jin10.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub(crate) struct EmDatacenterResp {
    pub result: Option<EmResult>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EmResult {
    #[serde(default)]
    pub data: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Jin10Resp {
    pub data: Option<Jin10Data>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Jin10Data {
    #[serde(default)]
    pub values: Vec<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Eastmoney datacenter helpers
// ---------------------------------------------------------------------------

/// Fetch a macro-economic report from Eastmoney datacenter and extract
/// date + indicator value into `MacroDataPoint`.
pub(crate) async fn fetch_em_report(
    client: &AkShareClient,
    report_name: &str,
    sort_column: &str,
    name_label: &str,
) -> Result<Vec<MacroDataPoint>> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
    let resp: EmDatacenterResp = client
        .get(url)
        .query(&[
            ("reportName", report_name),
            ("columns", "ALL"),
            ("pageNumber", "1"),
            ("pageSize", "500"),
            ("sortTypes", "-1"),
            ("sortColumns", sort_column),
            ("source", "WEB"),
            ("client", "WEB"),
        ])
        .send()
        .await?
        .json()
        .await?;

    let data = resp.result.map(|r| r.data).unwrap_or_default();
    let mut items = Vec::with_capacity(data.len());
    for v in &data {
        let date = v
            .get("REPORT_DATE")
            .or_else(|| v.get("REPORT_PERIOD"))
            .or_else(|| v.get("DATE"))
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string();

        let value = v
            .get("INDICATOR_VALUE")
            .or_else(|| v.get("VALUE"))
            .or_else(|| v.get("GDP"))
            .or_else(|| v.get("CPI"))
            .or_else(|| v.get("PPI"))
            .or_else(|| v.get("PMI"))
            .or_else(|| v.get("M2"))
            .and_then(|x| x.as_f64())
            .unwrap_or(0.0);

        if date.is_empty() {
            continue;
        }

        items.push(MacroDataPoint {
            date: date.get(..10).unwrap_or(&date).to_string(),
            value,
            name: name_label.to_string(),
        });
    }
    Ok(items)
}

/// Fetch a macro-economic report from Eastmoney datacenter with an INDICATOR_ID filter.
/// Used for country-specific economic indicators (UK, HK, Germany, etc.).
pub(crate) async fn fetch_em_indicator(
    client: &AkShareClient,
    report_name: &str,
    indicator_id: &str,
    name_label: &str,
) -> Result<Vec<MacroDataPoint>> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
    let filter = format!(r#"(INDICATOR_ID="{}")"#, indicator_id);
    let resp: EmDatacenterResp = client
        .get(url)
        .query(&[
            ("reportName", report_name),
            ("columns", "ALL"),
            ("filter", filter.as_str()),
            ("pageNumber", "1"),
            ("pageSize", "5000"),
            ("sortTypes", "-1"),
            ("sortColumns", "REPORT_DATE"),
            ("source", "WEB"),
            ("client", "WEB"),
        ])
        .send()
        .await?
        .json()
        .await?;

    let data = resp.result.map(|r| r.data).unwrap_or_default();
    let mut items = Vec::with_capacity(data.len());
    for v in &data {
        let date = v
            .get("REPORT_DATE")
            .or_else(|| v.get("REPORT_DATE_CH"))
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string();

        let value = v
            .get("VALUE")
            .or_else(|| v.get("INDICATOR_VALUE"))
            .and_then(|x| x.as_f64())
            .unwrap_or(0.0);

        if date.is_empty() {
            continue;
        }

        items.push(MacroDataPoint {
            date: date.get(..10).unwrap_or(&date).to_string(),
            value,
            name: name_label.to_string(),
        });
    }
    Ok(items)
}

/// Fetch an Eastmoney industry index with a specific INDICATOR_ID filter.
pub(crate) async fn fetch_em_industry_index(
    client: &AkShareClient,
    indicator_id: &str,
    name_label: &str,
) -> Result<Vec<MacroDataPoint>> {
    fetch_em_indicator(client, "RPT_INDUSTRY_INDEX", indicator_id, name_label).await
}

// ---------------------------------------------------------------------------
// Jin10 datacenter helpers
// ---------------------------------------------------------------------------

/// Fetch macro-economic data from Jin10 datacenter API.
/// Returns (date, current_value, forecast, previous_value) tuples.
pub(crate) async fn fetch_jin10_report(
    client: &AkShareClient,
    attr_id: &str,
    name_label: &str,
) -> Result<Vec<MacroDataPoint>> {
    let url = "https://datacenter-api.jin10.com/reports/list_v2";
    let resp: Jin10Resp = client
        .get(url)
        .query(&[("max_date", ""), ("category", "ec"), ("attr_id", attr_id)])
        .header("x-app-id", "rU6QIu7JHe2gOUeR")
        .header("x-csrf-token", "x-csrf-token")
        .header("x-version", "1.0.0")
        .send()
        .await?
        .json()
        .await?;

    let values = resp.data.map(|d| d.values).unwrap_or_default();
    let mut items = Vec::with_capacity(values.len());
    for row in &values {
        if row.len() < 2 {
            continue;
        }
        let date = row[0].as_str().unwrap_or("").to_string();
        let value = row[1].as_f64().unwrap_or(0.0);
        if date.is_empty() {
            continue;
        }
        items.push(MacroDataPoint {
            date: date.get(..10).unwrap_or(&date).to_string(),
            value,
            name: name_label.to_string(),
        });
    }
    Ok(items)
}

/// Fetch Jin10 interest rate decision data.
pub(crate) async fn fetch_jin10_interest_rate(
    client: &AkShareClient,
    attr_id: &str,
    name_label: &str,
) -> Result<Vec<MacroDataPoint>> {
    fetch_jin10_report(client, attr_id, name_label).await
}
