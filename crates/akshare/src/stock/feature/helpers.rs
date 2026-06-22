//! Generic helpers for Eastmoney datacenter and other API patterns.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::wire::{ClistResp, EmDatacenterResp, KlineResp};

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
            let payload: EmDatacenterResp = resp.json().await.map_err(Error::from)?;
            let result = payload
                .result
                .ok_or_else(|| Error::upstream("eastmoney datacenter missing result"))?;

            all_data.extend(result.data);

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

        let payload: ClistResp = resp.json().await.map_err(Error::from)?;
        let items = payload.data.and_then(|d| d.diff).unwrap_or_default();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney clist returned no data"));
        }
        Ok(items)
    }

    /// Generic Eastmoney kline fetch from push2his.
    ///
    /// Returns raw kline strings (comma-separated OHLCV etc.) from the Eastmoney
    /// kline API. Each string needs to be parsed by the caller with `parse_csv_line`.
    ///
    /// # Parameters
    /// - `secid`: Eastmoney security id (e.g. `"1.600000"`, `"133.USDCNY"`)
    /// - `klt`: Kline period (1=1min, 5=5min, 15=15min, 30=30min, 60=60min, 101=daily, 102=weekly, 103=monthly)
    /// - `fqt`: Forward adjust (0=none, 1=forward, 2=backwards)
    /// - `limit`: Number of bars to return
    /// - `extra`: Additional query parameters (e.g. `[("iscca", "1")]`)
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn kline_fetch(
        &self,
        secid: &str,
        klt: &str,
        fqt: &str,
        limit: usize,
        extra: &[(&str, &str)],
    ) -> Result<Vec<String>> {
        let lmt = limit.max(1).to_string();
        let mut builder = self
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", klt),
                ("fqt", fqt),
                ("lmt", lmt.as_str()),
                ("end", "20500000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ]);
        for &(k, v) in extra {
            builder = builder.query(&[(k, v)]);
        }

        let resp = builder
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: KlineResp = resp.json().await.map_err(Error::from)?;
        let klines = payload.data.and_then(|d| d.klines).unwrap_or_default();

        if klines.is_empty() {
            return Err(Error::not_found("eastmoney kline returned no data"));
        }
        Ok(klines)
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
