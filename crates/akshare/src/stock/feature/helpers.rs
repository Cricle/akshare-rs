#![allow(dead_code)]
//! Generic helpers for Eastmoney datacenter and other API patterns.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use serde::Deserialize;

/// Generic Eastmoney datacenter response envelope.
#[derive(Debug, Deserialize)]
pub(crate) struct DcEnvelope {
    pub result: Option<DcResult>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DcResult {
    pub data: Option<Vec<serde_json::Value>>,
    pub pages: Option<i64>,
    pub count: Option<i64>,
}

/// Generic Eastmoney push2ex response envelope.
#[derive(Debug, Deserialize)]
pub(crate) struct Push2exEnvelope {
    pub data: Option<serde_json::Value>,
}

/// Generic Eastmoney clist response for spot data.
#[derive(Debug, Deserialize)]
pub(crate) struct ClistSpotEnvelope {
    pub data: Option<ClistSpotData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ClistSpotData {
    pub diff: Option<Vec<serde_json::Value>>,
    pub total: Option<i64>,
}

/// Format a date string from "YYYYMMDD" to "YYYY-MM-DD".
pub(crate) fn fmt_date(date: &str) -> String {
    if date.len() >= 8 {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        date.to_string()
    }
}

/// Get a string field from a JSON value, returning default if missing.
pub(crate) fn json_str(v: &serde_json::Value, key: &str) -> String {
    v.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// Get a string field as Option from a JSON value.
pub(crate) fn json_str_opt(v: &serde_json::Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|v| v.as_str())
        .map(std::string::ToString::to_string)
}

/// Get a f64 field from a JSON value, returning 0.0 if missing.
pub(crate) fn json_f64(v: &serde_json::Value, key: &str) -> f64 {
    v.get(key)
        .and_then(serde_json::Value::as_f64)
        .unwrap_or(0.0)
}

/// Get a f64 field as Option from a JSON value.
pub(crate) fn json_f64_opt(v: &serde_json::Value, key: &str) -> Option<f64> {
    v.get(key).and_then(serde_json::Value::as_f64)
}

/// Get an i64 field from a JSON value, returning 0 if missing.
pub(crate) fn json_i64(v: &serde_json::Value, key: &str) -> i64 {
    v.get(key).and_then(serde_json::Value::as_i64).unwrap_or(0)
}

/// Get an i64 field as Option from a JSON value.
pub(crate) fn json_i64_opt(v: &serde_json::Value, key: &str) -> Option<i64> {
    v.get(key).and_then(serde_json::Value::as_i64)
}

impl AkShareClient {
    /// Generic Eastmoney datacenter fetch with pagination support.
    ///
    #[allow(clippy::too_many_arguments)]
    /// Fetches all pages and returns combined raw JSON values.
    pub(crate) async fn dc_fetch_all(
        &self,
        report_name: &str,
        columns: &str,
        filter: &str,
        sort_columns: &str,
        sort_types: &str,
        page_size: i64,
        max_pages: i64,
        extra_params: &[(&str, &str)],
    ) -> Result<Vec<serde_json::Value>> {
        let mut all_data = Vec::new();
        let ps = page_size.to_string();

        let mut page = 1_i64;
        loop {
            let pn = page.to_string();
            let mut builder = self
                .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("reportName", report_name),
                    ("columns", columns),
                    ("filter", filter),
                    ("pageNumber", &pn),
                    ("pageSize", &ps),
                    ("sortTypes", sort_types),
                    ("sortColumns", sort_columns),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]);
            for &(k, v) in extra_params {
                builder = builder.query(&[(k, v)]);
            }

            let resp = builder
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;
            let payload: DcEnvelope = resp.json().await.map_err(Error::from)?;
            let result = payload
                .result
                .ok_or_else(|| Error::upstream("eastmoney datacenter missing result"))?;

            if let Some(data) = result.data {
                all_data.extend(data);
            }

            let total_pages = result.pages.unwrap_or(1);
            if page >= total_pages || page >= max_pages {
                break;
            }
            page += 1;
        }

        if all_data.is_empty() {
            return Err(Error::not_found("eastmoney datacenter returned no data"));
        }
        Ok(all_data)
    }

    /// Generic Eastmoney push2ex fetch (for limit-up/down pools, order book changes).
    pub(crate) async fn push2ex_fetch(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        let url = format!("https://push2ex.eastmoney.com/{path}");
        let resp = self
            .get(&url)
            .query(params)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let payload: serde_json::Value = resp.json().await.map_err(Error::from)?;
        Ok(payload)
    }

    /// Generic Eastmoney push2 clist fetch (for spot market data).
    pub(crate) async fn clist_spot_fetch(
        &self,
        fs: &str,
        fields: &str,
        page_size: &str,
        sort_field: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let resp = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", page_size),
                ("po", "1"),
                ("np", "1"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", sort_field),
                ("fs", fs),
                ("fields", fields),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: ClistSpotEnvelope = resp.json().await.map_err(Error::from)?;
        let items = payload.data.and_then(|d| d.diff).unwrap_or_default();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney clist returned no data"));
        }
        Ok(items)
    }

    /// Generic Eastmoney emweb financial report fetch.
    /// Used for per-stock financial statements (balance sheet, profit, cash flow).
    pub(crate) async fn emweb_financial_fetch(
        &self,
        code: &str,
        report_type: &str,
        date_type: &str,
    ) -> Result<Vec<serde_json::Value>> {
        // First get company type
        let url = "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/Index";
        let resp = self
            .get(url)
            .query(&[("type", "web"), ("code", &code.to_lowercase())])
            .send()
            .await
            .map_err(Error::from)?;
        let html = resp.text().await.map_err(Error::from)?;

        // Extract company type from HTML
        let company_type = if html.contains("hidctype") {
            // Try to extract from hidden input
            if let Some(start) = html.find(r#"id="hidctype""#) {
                if let Some(val_start) = html[start..].find("value=\"") {
                    let val_start = start + val_start + 7;
                    if let Some(val_end) = html[val_start..].find('"') {
                        html[val_start..val_start + val_end].to_string()
                    } else {
                        "4".to_string()
                    }
                } else {
                    "4".to_string()
                }
            } else {
                "4".to_string()
            }
        } else {
            "4".to_string()
        };

        // Get date list
        let date_url = format!(
            "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/{report_type}DateAjaxNew"
        );
        let code_lower = code.to_lowercase();
        let resp = self
            .get(&date_url)
            .query(&[
                ("companyType", company_type.as_str()),
                ("reportDateType", date_type),
                ("code", code_lower.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let date_json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let dates = date_json
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        if dates.is_empty() {
            return Err(Error::not_found("no financial report dates available"));
        }

        // Fetch data in batches of 5 dates
        let mut all_data = Vec::new();
        let date_strs: Vec<String> = dates
            .iter()
            .filter_map(|d| {
                d.get("REPORT_DATE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string)
            })
            .collect();

        for chunk in date_strs.chunks(5) {
            let dates_param = chunk.join(",");
            let data_url = format!(
                "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/{report_type}AjaxNew"
            );
            let resp = self
                .get(&data_url)
                .query(&[
                    ("companyType", company_type.as_str()),
                    ("reportDateType", date_type),
                    ("reportType", "1"),
                    ("dates", dates_param.as_str()),
                    ("code", code_lower.as_str()),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
            if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                all_data.extend(data.clone());
            }
        }

        Ok(all_data)
    }
}
