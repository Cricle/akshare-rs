//! Eastmoney datacenter-based fundamental data APIs.
//!
//! Covers: financial analysis indicators, HK/US financial reports,
//! IPO registration, restricted releases, IPO declare/review/tutor,
//! profit forecasts, share capital structure, main business composition,
//! and stock notices.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::Deserialize;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct PagedEnvelope {
    result: Option<PagedResult>,
}

#[derive(Debug, Deserialize)]
struct PagedResult {
    data: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pages: u64,
}

#[derive(Debug, Deserialize)]
struct SingleEnvelope {
    result: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Generic helpers
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
/// Fetch a single page from the Eastmoney web datacenter API.
async fn fetch_datacenter_page(
    http: &reqwest::Client,
    report_name: &str,
    columns: &str,
    filter: &str,
    page: u64,
    page_size: u64,
    sort_columns: &str,
    sort_types: &str,
    source: &str,
) -> Result<(Vec<serde_json::Value>, u64)> {
    let ps = page_size.to_string();
    let pn = page.to_string();
    let mut params = vec![
        ("reportName", report_name),
        ("columns", columns),
        ("pageNumber", pn.as_str()),
        ("pageSize", ps.as_str()),
        ("sortTypes", sort_types),
        ("sortColumns", sort_columns),
        ("source", source),
        ("client", "WEB"),
    ];
    if !filter.is_empty() {
        params.push(("filter", filter));
    }

    let resp = http
        .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
        .query(&params)
        .send()
        .await?
        .error_for_status()?;

    let payload: PagedEnvelope = resp.json().await?;
    let result = payload
        .result
        .ok_or_else(|| Error::upstream("eastmoney datacenter response missing result"))?;
    let data = result.data.unwrap_or_default();
    let pages = result.pages;
    Ok((data, pages))
}

/// Fetch all pages from the Eastmoney web datacenter API.
async fn fetch_datacenter_all(
    http: &reqwest::Client,
    report_name: &str,
    columns: &str,
    filter: &str,
    sort_columns: &str,
    sort_types: &str,
    source: &str,
) -> Result<Vec<serde_json::Value>> {
    let page_size: u64 = 500;
    let (first_data, total_pages) = fetch_datacenter_page(
        http,
        report_name,
        columns,
        filter,
        1,
        page_size,
        sort_columns,
        sort_types,
        source,
    )
    .await?;

    let mut all = first_data;
    for page in 2..=total_pages {
        let (data, _) = fetch_datacenter_page(
            http,
            report_name,
            columns,
            filter,
            page,
            page_size,
            sort_columns,
            sort_types,
            source,
        )
        .await?;
        all.extend(data);
    }
    Ok(all)
}

#[allow(clippy::too_many_arguments)]
/// Fetch a single page from the Eastmoney securities API (used for HK/US financials).
async fn fetch_securities_page(
    http: &reqwest::Client,
    report_name: &str,
    columns: &str,
    filter: &str,
    page: u64,
    page_size: u64,
    sort_columns: &str,
    sort_types: &str,
) -> Result<Vec<serde_json::Value>> {
    let ps = page_size.to_string();
    let pn = page.to_string();
    let mut params = vec![
        ("reportName", report_name),
        ("columns", columns),
        ("pageNumber", pn.as_str()),
        ("pageSize", ps.as_str()),
        ("sortTypes", sort_types),
        ("sortColumns", sort_columns),
        ("source", "F10"),
        ("client", "PC"),
    ];
    if !filter.is_empty() {
        params.push(("filter", filter));
    }

    let resp = http
        .get("https://datacenter.eastmoney.com/securities/api/data/v1/get")
        .query(&params)
        .send()
        .await?
        .error_for_status()?;

    let payload: PagedEnvelope = resp.json().await?;
    let result = payload
        .result
        .ok_or_else(|| Error::upstream("eastmoney securities response missing result"))?;
    Ok(result.data.unwrap_or_default())
}

/// Fetch from the Eastmoney securities API with a `type` parameter (older format).
async fn fetch_securities_type(
    http: &reqwest::Client,
    data_type: &str,
    sty: &str,
    filter: &str,
) -> Result<Vec<serde_json::Value>> {
    let params = vec![
        ("type", data_type),
        ("sty", sty),
        ("quoteColumns", ""),
        ("filter", filter),
        ("p", "1"),
        ("ps", "200"),
        ("sr", "-1"),
        ("st", "REPORT_DATE"),
        ("source", "HSF10"),
        ("client", "PC"),
    ];

    let resp = http
        .get("https://datacenter.eastmoney.com/securities/api/data/get")
        .query(&params)
        .send()
        .await?
        .error_for_status()?;

    let payload: SingleEnvelope = resp.json().await?;
    match payload.result {
        Some(serde_json::Value::Array(arr)) => Ok(arr),
        Some(other) => Ok(vec![other]),
        None => Ok(vec![]),
    }
}

// ---------------------------------------------------------------------------
// A-share financial analysis indicators
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Eastmoney A-share financial analysis main indicators.
    ///
    /// `symbol` is in SECUCODE format, e.g. "301389.SZ".
    /// `indicator` is "按报告期" or "按单季度".
    pub async fn stock_financial_analysis_indicator_em(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let data = if indicator == "按报告期" {
            fetch_securities_type(
                &self.http,
                "RPT_F10_FINANCE_MAINFINADATA",
                "APP_F10_MAINFINADATA",
                &format!(r#"(SECUCODE="{symbol}")"#),
            )
            .await?
        } else {
            fetch_securities_page(
                &self.http,
                "RPT_F10_QTR_MAINFINADATA",
                "ALL",
                &format!(r#"(SECUCODE="{symbol}")"#),
                1,
                200,
                "REPORT_DATE",
                "-1",
            )
            .await?
        };

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    // -----------------------------------------------------------------------
    // HK financial reports
    // -----------------------------------------------------------------------

    /// Eastmoney HK stock financial reports (balance/income/cash flow).
    ///
    /// `stock` is the HK stock code, e.g. "00700".
    /// `symbol` is one of "资产负债表", "利润表", "现金流量表".
    /// `indicator` is "年度" or "报告期".
    pub async fn stock_financial_hk_report_em(
        &self,
        stock: &str,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        // Step 1: Get available report dates
        let filter = format!(r#"(SECUCODE="{stock}.HK")"#);
        let summary = fetch_securities_page(
            &self.http,
            "RPT_CUSTOM_HKSK_APPFN_CASHFLOW_SUMMARY",
            "SECUCODE,SECURITY_CODE,SECURITY_NAME_ABBR,START_DATE,REPORT_DATE,FISCAL_YEAR,CURRENCY,ACCOUNT_STANDARD,REPORT_TYPE",
            &filter,
            1, 100, "", "",
        )
        .await?;

        if summary.is_empty() {
            return Ok(vec![]);
        }

        // Extract REPORT_LIST from first record
        let report_list = summary[0]
            .get("REPORT_LIST")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // Filter by indicator
        let filtered: Vec<&serde_json::Value> = if indicator == "年度" {
            report_list
                .iter()
                .filter(|r| {
                    r.get("REPORT_TYPE")
                        .and_then(|v| v.as_str())
                        .is_some_and(|s| s == "年报")
                })
                .collect()
        } else {
            report_list.iter().collect()
        };

        let year_list: Vec<String> = filtered
            .iter()
            .filter_map(|r| {
                r.get("REPORT_DATE")
                    .and_then(|v| v.as_str())
                    .map(|s| s.split(' ').next().unwrap_or(s).to_string())
            })
            .collect();

        if year_list.is_empty() {
            return Ok(vec![]);
        }

        let years_joined = year_list
            .iter()
            .map(|y| format!("'{y}'"))
            .collect::<Vec<_>>()
            .join(",");

        let report_name = match symbol {
            "资产负债表" => "RPT_HKF10_FN_BALANCE_PC",
            "利润表" => "RPT_HKF10_FN_INCOME_PC",
            "现金流量表" => "RPT_HKF10_FN_CASHFLOW_PC",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported symbol: {symbol}"
                )));
            }
        };

        let cols = match symbol {
            "资产负债表" => {
                "SECUCODE,SECURITY_CODE,SECURITY_NAME_ABBR,ORG_CODE,REPORT_DATE,DATE_TYPE_CODE,FISCAL_YEAR,STD_ITEM_CODE,STD_ITEM_NAME,AMOUNT,STD_REPORT_DATE"
            }
            _ => {
                "SECUCODE,SECURITY_CODE,SECURITY_NAME_ABBR,ORG_CODE,REPORT_DATE,DATE_TYPE_CODE,FISCAL_YEAR,START_DATE,STD_ITEM_CODE,STD_ITEM_NAME,AMOUNT"
            }
        };

        let filter = format!(r#"(SECUCODE="{stock}.HK")(REPORT_DATE in ({years_joined}))"#);
        let data = fetch_securities_page(
            &self.http,
            report_name,
            cols,
            &filter,
            1,
            0, // empty pageSize means all
            "REPORT_DATE,STD_ITEM_CODE",
            "-1,1",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    /// Eastmoney HK stock financial analysis main indicators.
    ///
    /// `symbol` is the HK stock code, e.g. "00700".
    /// `indicator` is "年度" or "报告期".
    pub async fn stock_financial_hk_analysis_indicator_em(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let filter = if indicator == "年度" {
            format!(r#"(SECUCODE="{symbol}.HK")(DATE_TYPE_CODE="001")"#)
        } else {
            format!(r#"(SECUCODE="{symbol}.HK")"#)
        };

        let data = fetch_securities_page(
            &self.http,
            "RPT_HKF10_FN_MAININDICATOR",
            "HKF10_FN_MAININDICATOR",
            &filter,
            1,
            9,
            "STD_REPORT_DATE",
            "-1",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    // -----------------------------------------------------------------------
    // US financial reports
    // -----------------------------------------------------------------------

    /// Resolve US stock SECUCODE from the security code.
    async fn us_resolve_secucode(&self, symbol: &str) -> Result<String> {
        let data = fetch_securities_page(
            &self.http,
            "RPT_USF10_INFO_ORGPROFILE",
            "SECUCODE,SECURITY_CODE,ORG_CODE,SECURITY_INNER_CODE,ORG_NAME,ORG_EN_ABBR,BELONG_INDUSTRY,FOUND_DATE,CHAIRMAN,REG_PLACE,ADDRESS,EMP_NUM,ORG_TEL,ORG_FAX,ORG_EMAIL,ORG_WEB,ORG_PROFILE",
            &format!(r#"(SECURITY_CODE="{symbol}")"#),
            1, 200, "", "",
        )
        .await?;

        data.first()
            .and_then(|v| v.get("SECUCODE"))
            .and_then(|v| v.as_str())
            .map(std::string::ToString::to_string)
            .ok_or_else(|| Error::not_found(format!("US stock {symbol} not found")))
    }

    /// Eastmoney US stock financial reports.
    ///
    /// `stock` is the US stock code, e.g. "TSLA".
    /// `symbol` is one of "资产负债表", "综合损益表", "现金流量表".
    /// `indicator` is "年报", "单季报", or "累计季报".
    pub async fn stock_financial_us_report_em(
        &self,
        stock: &str,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let secucode = self.us_resolve_secucode(stock).await?;

        // Step 1: Get available reports to determine report names
        let report_name = match symbol {
            "资产负债表" => "RPT_USF10_FN_BALANCE",
            "综合损益表" => "RPT_USF10_FN_INCOME",
            "现金流量表" => "RPT_USSK_FN_CASHFLOW",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported symbol: {symbol}"
                )));
            }
        };

        let reports = fetch_securities_page(
            &self.http,
            report_name,
            "SECUCODE,SECURITY_CODE,SECURITY_NAME_ABBR,REPORT,REPORT_DATE,FISCAL_YEAR,CURRENCY,ACCOUNT_STANDARD,REPORT_TYPE,DATE_TYPE_CODE",
            &format!(r#"(SECUCODE="{secucode}")"#),
            1, 0, "REPORT_DATE", "-1",
        )
        .await?;

        // Extract unique REPORT values
        let report_set: std::collections::HashSet<String> = reports
            .iter()
            .filter_map(|r| {
                r.get("REPORT")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string)
            })
            .collect();

        // Filter by indicator type
        let filtered: Vec<String> = match indicator {
            "年报" => report_set
                .into_iter()
                .filter(|r| r.contains("FY"))
                .collect(),
            "单季报" => report_set
                .into_iter()
                .filter(|r| {
                    r.contains("Q1") || r.contains("Q2") || r.contains("Q3") || r.contains("Q4")
                })
                .collect(),
            "累计季报" => report_set
                .into_iter()
                .filter(|r| r.contains("Q6") || r.contains("Q9"))
                .collect(),
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported indicator: {indicator}"
                )));
            }
        };

        if filtered.is_empty() {
            return Ok(vec![]);
        }

        let mut sorted = filtered;
        sorted.sort_by(|a, b| b.cmp(a)); // reverse sort
        let reports_joined = sorted
            .iter()
            .map(|r| format!("\"{r}\""))
            .collect::<Vec<_>>()
            .join(",");

        // Step 2: Fetch actual data
        let filter = format!(r#"(SECUCODE="{secucode}")(REPORT in ({reports_joined}))"#);
        let data = fetch_securities_page(
            &self.http,
            report_name,
            "SECUCODE,SECURITY_CODE,SECURITY_NAME_ABBR,REPORT_DATE,REPORT_TYPE,REPORT,STD_ITEM_CODE,AMOUNT,ITEM_NAME",
            &filter,
            1, 0, "STD_ITEM_CODE,REPORT_DATE", "1,-1",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    /// Eastmoney US stock financial analysis main indicators.
    ///
    /// `symbol` is the US stock code, e.g. "TSLA".
    /// `indicator` is "年报", "单季报", or "累计季报".
    pub async fn stock_financial_us_analysis_indicator_em(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let secucode = self.us_resolve_secucode(symbol).await?;

        let is_insurance = secucode.contains('_');
        let report_name = if is_insurance {
            "RPT_USF10_FN_IMAININDICATOR"
        } else {
            "RPT_USF10_FN_GMAININDICATOR"
        };

        let columns = if is_insurance {
            "ORG_CODE,SECURITY_CODE,SECUCODE,SECURITY_NAME_ABBR,SECURITY_INNER_CODE,STD_REPORT_DATE,REPORT_DATE,DATE_TYPE,DATE_TYPE_CODE,REPORT_TYPE,REPORT_DATA_TYPE,FISCAL_YEAR,START_DATE,NOTICE_DATE,ACCOUNT_STANDARD,ACCOUNT_STANDARD_NAME,CURRENCY,CURRENCY_NAME,ORGTYPE,TOTAL_INCOME,TOTAL_INCOME_YOY,PREMIUM_INCOME,PREMIUM_INCOME_YOY,PARENT_HOLDER_NETPROFIT,PARENT_HOLDER_NETPROFIT_YOY,BASIC_EPS_CS,BASIC_EPS_CS_YOY,DILUTED_EPS_CS,PAYOUT_RATIO,CAPITIAL_RATIO,ROE,ROE_YOY,ROA,ROA_YOY,DEBT_RATIO,DEBT_RATIO_YOY,EQUITY_RATIO"
        } else {
            "USF10_FN_GMAININDICATOR"
        };

        let filter = match indicator {
            "年报" => format!(r#"(SECUCODE="{secucode}")(DATE_TYPE_CODE="001")"#),
            "单季报" => {
                format!(r#"(SECUCODE="{secucode}")(DATE_TYPE_CODE in ("003","006","007","008"))"#)
            }
            "累计季报" => {
                format!(r#"(SECUCODE="{secucode}")(DATE_TYPE_CODE in ("002","004"))"#)
            }
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported indicator: {indicator}"
                )));
            }
        };

        let data = fetch_securities_page(
            &self.http,
            report_name,
            columns,
            &filter,
            1,
            0,
            "REPORT_DATE",
            "-1",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    // -----------------------------------------------------------------------
    // IPO registration
    // -----------------------------------------------------------------------

    const IPO_REGISTER_COLUMNS: &'static str = "SECURITY_CODE,STATE,REG_ADDRESS,INFO_CODE,CSRC_INDUSTRY,ACCEPT_DATE,DECLARE_ORG,PREDICT_LISTING_MARKET,LAW_FIRM,ACCOUNT_FIRM,ORG_CODE,UPDATE_DATE,RECOMMEND_ORG,IS_REGISTRATION";

    /// Eastmoney IPO registration data.
    ///
    /// `market` is one of: "全部", "科创板", "创业板", "北交所", "沪主板", "深主板", "达标企业".
    pub async fn stock_register_em(
        &self,
        market: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        if market == "达标企业" {
            return self.stock_register_db_em().await;
        }

        let filter = match market {
            "全部" => "",
            "科创板" => r#"(PREDICT_LISTING_MARKET="科创板")"#,
            "创业板" => r#"(PREDICT_LISTING_MARKET="创业板")"#,
            "北交所" => r#"(PREDICT_LISTING_MARKET="北交所")"#,
            "沪主板" => r#"(PREDICT_LISTING_MARKET="沪主板")"#,
            "深主板" => r#"(PREDICT_LISTING_MARKET="深主板")"#,
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported market: {market}"
                )));
            }
        };

        let data = fetch_datacenter_all(
            &self.http,
            "RPT_IPO_INFOALLNEW",
            Self::IPO_REGISTER_COLUMNS,
            filter,
            "UPDATE_DATE,ORG_CODE",
            "-1,-1",
            "WEB",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    async fn stock_register_db_em(&self) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let data = fetch_datacenter_all(
            &self.http,
            "RPT_KCB_IPO",
            "KCB_LB",
            r#"(ORG_TYPE_CODE="03")"#,
            "NOTICE_DATE,SECURITY_CODE",
            "-1,-1",
            "WEB",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    // -----------------------------------------------------------------------
    // Restricted releases
    // -----------------------------------------------------------------------

    /// Eastmoney restricted release market summary.
    ///
    /// `market` is one of: "全部股票", "沪市A股", "科创板", "深市A股", "创业板", "京市A股".
    /// `start_date` and `end_date` are in "YYYYMMDD" format.
    pub async fn stock_restricted_release_summary_em(
        &self,
        market: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let index_code = match market {
            "全部股票" => "000300",
            "沪市A股" => "000001",
            "科创板" => "000688",
            "深市A股" => "399001",
            "创业板" => "399001",
            "京市A股" => "999999",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported market: {market}"
                )));
            }
        };

        let sd = format!(
            "{}-{}-{}",
            &start_date[..4],
            &start_date[4..6],
            &start_date[6..8]
        );
        let ed = format!("{}-{}-{}", &end_date[..4], &end_date[4..6], &end_date[6..8]);

        let filter =
            format!(r#"(INDEX_CODE="{index_code}")(FREE_DATE>='{sd}')(FREE_DATE<='{ed}')"#);
        let data = fetch_datacenter_all(
            &self.http,
            "RPT_LIFTDAY_STA",
            "ALL",
            &filter,
            "FREE_DATE",
            "1",
            "WEB",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    /// Eastmoney restricted release detail list.
    ///
    /// `start_date` and `end_date` are in "YYYYMMDD" format.
    pub async fn stock_restricted_release_detail_em(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let sd = format!(
            "{}-{}-{}",
            &start_date[..4],
            &start_date[4..6],
            &start_date[6..8]
        );
        let ed = format!("{}-{}-{}", &end_date[..4], &end_date[4..6], &end_date[6..8]);

        let filter = format!(r#"(FREE_DATE>='{sd}')(FREE_DATE<='{ed}')"#);
        let data = fetch_datacenter_all(
            &self.http,
            "RPT_LIFT_STAGE",
            "SECURITY_CODE,SECURITY_NAME_ABBR,FREE_DATE,CURRENT_FREE_SHARES,ABLE_FREE_SHARES,LIFT_MARKET_CAP,FREE_RATIO,NEW,B20_ADJCHRATE,A20_ADJCHRATE,FREE_SHARES_TYPE,TOTAL_RATIO,NON_FREE_SHARES,BATCH_HOLDER_NUM",
            &filter,
            "FREE_DATE,CURRENT_FREE_SHARES",
            "1,1",
            "WEB",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    /// Eastmoney individual stock restricted release queue.
    pub async fn stock_restricted_release_queue_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let data = fetch_datacenter_all(
            &self.http,
            "RPT_LIFT_STAGE",
            "SECURITY_CODE,SECURITY_NAME_ABBR,FREE_DATE,CURRENT_FREE_SHARES,ABLE_FREE_SHARES,LIFT_MARKET_CAP,FREE_RATIO,NEW,B20_ADJCHRATE,A20_ADJCHRATE,FREE_SHARES_TYPE,TOTAL_RATIO,NON_FREE_SHARES,BATCH_HOLDER_NUM",
            &format!(r#"(SECURITY_CODE="{symbol}")"#),
            "FREE_DATE",
            "-1",
            "WEB",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    /// Eastmoney individual stock restricted release stockholder details.
    ///
    /// `symbol` is the stock code, e.g. "600000".
    /// `date` is in "YYYYMMDD" format (obtained from `stock_restricted_release_queue_em`).
    pub async fn stock_restricted_release_stockholder_em(
        &self,
        symbol: &str,
        date: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let date_str = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        let filter = format!(r#"(SECURITY_CODE="{symbol}")(FREE_DATE='{date_str}')"#);
        let data = fetch_datacenter_all(
            &self.http,
            "RPT_LIFT_GD",
            "LIMITED_HOLDER_NAME,ADD_LISTING_SHARES,ACTUAL_LISTED_SHARES,ADD_LISTING_CAP,LOCK_MONTH,RESIDUAL_LIMITED_SHARES,FREE_SHARES_TYPE,PLAN_FEATURE",
            &filter,
            "ADD_LISTING_SHARES",
            "-1",
            "WEB",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    // -----------------------------------------------------------------------
    // IPO declare / review / tutor
    // -----------------------------------------------------------------------

    /// Eastmoney IPO declaration (first-time filing) information.
    pub async fn stock_ipo_declare_em(&self) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let data = fetch_datacenter_all(
            &self.http,
            "RPT_IPO_DECORGNEWEST",
            "DECLARE_ORG,STATE,REG_ADDRESS,RECOMMEND_ORG,LAW_FIRM,ACCOUNT_FIRM,IS_SUBMIT,PREDICT_LISTING_MARKET,END_DATE,INFO_CODE,SECURITY_CODE,ORG_CODE,IS_REGISTER,STATE_CODE,DERIVE_SECURITY_CODE,ORG_CODE_OLD,IS_STATE",
            "",
            "END_DATE,SECURITY_CODE",
            "-1,-1",
            "WEB",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    /// Eastmoney IPO review (listing committee) information.
    pub async fn stock_ipo_review_em(&self) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let data = fetch_datacenter_all(
            &self.http,
            "RPT_IPO_REVIEW",
            "ALL",
            "",
            "REVIEW_DATE,ORG_CODE",
            "-1,-1",
            "WEB",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    /// Eastmoney IPO tutor (coaching/filing) information.
    pub async fn stock_ipo_tutor_em(&self) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let data = fetch_datacenter_all(
            &self.http,
            "RPT_IPO_TUTRECORD",
            "TUTOR_OBJECT,ORG_CODE,TUTOR_ORG_CODE,TUTOR_ORG,TUTOR_PROCESS_STATE,REPORT_TYPE,DISPATCH_ORG,REPORT_TITLE,RECORD_DATE",
            "",
            "RECORD_DATE,TUTOR_OBJECT",
            "-1,-1",
            "WEB",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    // -----------------------------------------------------------------------
    // Profit forecast
    // -----------------------------------------------------------------------

    /// Eastmoney profit forecast data.
    ///
    /// `industry` is an optional industry board name, e.g. "船舶制造".
    /// Pass "" for all industries.
    pub async fn stock_profit_forecast_em(
        &self,
        industry: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let filter = if industry.is_empty() {
            String::new()
        } else {
            format!(r#"(INDUSTRY_BOARD="{industry}")"#)
        };

        let data = fetch_datacenter_all(
            &self.http,
            "RPT_WEB_RESPREDICT",
            "WEB_RESPREDICT",
            &filter,
            "RATING_ORG_NUM",
            "-1",
            "WEB",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    // -----------------------------------------------------------------------
    // Share capital structure
    // -----------------------------------------------------------------------

    /// Eastmoney A-share share capital structure (股本结构).
    ///
    /// `symbol` is in SECUCODE format, e.g. "603392.SH".
    pub async fn stock_zh_a_gbjg_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let data = fetch_securities_page(
            &self.http,
            "RPT_F10_EH_EQUITY",
            "SECUCODE,SECURITY_CODE,END_DATE,TOTAL_SHARES,LIMITED_SHARES,LIMITED_OTHARS,LIMITED_DOMESTIC_NATURAL,LIMITED_STATE_LEGAL,LIMITED_OVERSEAS_NOSTATE,LIMITED_OVERSEAS_NATURAL,UNLIMITED_SHARES,LISTED_A_SHARES,B_FREE_SHARE,H_FREE_SHARE,FREE_SHARES,LIMITED_A_SHARES,NON_FREE_SHARES,LIMITED_B_SHARES,OTHER_FREE_SHARES,LIMITED_STATE_SHARES,LIMITED_DOMESTIC_NOSTATE,LOCK_SHARES,LIMITED_FOREIGN_SHARES,LIMITED_H_SHARES,SPONSOR_SHARES,STATE_SPONSOR_SHARES,SPONSOR_SOCIAL_SHARES,RAISE_SHARES,RAISE_STATE_SHARES,RAISE_DOMESTIC_SHARES,RAISE_OVERSEAS_SHARES,CHANGE_REASON",
            &format!(r#"(SECUCODE="{symbol}")"#),
            1, 20, "END_DATE", "-1",
        )
        .await?;

        Ok(data
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    // -----------------------------------------------------------------------
    // Main business composition
    // -----------------------------------------------------------------------

    /// Eastmoney main business composition (主营构成).
    ///
    /// `symbol` is in SECUCODE format, e.g. "SH688041" or "688041.SH".
    pub async fn stock_zygc_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let resp = self
            .get("https://emweb.securities.eastmoney.com/PC_HSF10/BusinessAnalysis/PageAjax")
            .query(&[("code", symbol)])
            .send()
            .await?
            .error_for_status()?;

        let payload: serde_json::Value = resp.json().await?;
        let arr = payload
            .get("zygcfx")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        Ok(arr
            .into_iter()
            .filter_map(|v| {
                if let serde_json::Value::Object(map) = v {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            })
            .collect())
    }

    // -----------------------------------------------------------------------
    // Stock notices
    // -----------------------------------------------------------------------

    /// Eastmoney stock notice report (公告大全).
    ///
    /// `report_type` is one of: "全部", "财务报告", "融资公告", "风险提示",
    /// "信息变更", "重大事项", "资产重组", "持股变动".
    /// `date` is in "YYYYMMDD" format.
    pub async fn stock_notice_report(
        &self,
        report_type: &str,
        date: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let begin_date = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        self.fetch_notices(None, report_type, Some(&begin_date), Some(&begin_date))
            .await
    }

    /// Eastmoney individual stock notice report.
    ///
    /// `security` is the stock code, e.g. "300237".
    /// `report_type` is one of: "全部", "财务报告", etc.
    /// `begin_date` and `end_date` are optional, in "YYYYMMDD" format.
    pub async fn stock_individual_notice_report(
        &self,
        security: &str,
        report_type: &str,
        begin_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        self.fetch_notices(Some(security), report_type, begin_date, end_date)
            .await
    }

    async fn fetch_notices(
        &self,
        security: Option<&str>,
        report_type: &str,
        begin_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let report_map: HashMap<&str, &str> = [
            ("全部", "0"),
            ("财务报告", "1"),
            ("融资公告", "2"),
            ("风险提示", "3"),
            ("信息变更", "4"),
            ("重大事项", "5"),
            ("资产重组", "6"),
            ("持股变动", "7"),
        ]
        .into_iter()
        .collect();

        let f_node = report_map.get(report_type).copied().unwrap_or("0");

        let mut all_items = Vec::new();
        let mut page = 1_u64;

        loop {
            let ps = "100";
            let pn = page.to_string();
            let mut params = vec![
                ("sr", "-1"),
                ("page_size", ps),
                ("page_index", pn.as_str()),
                ("ann_type", "A"),
                ("client_source", "web"),
                ("f_node", f_node),
                ("s_node", "0"),
            ];

            let stock_list;
            if let Some(sec) = security {
                stock_list = sec.to_string();
                params.push(("stock_list", stock_list.as_str()));
            }

            let bd_str;
            let ed_str;
            if let Some(bd) = begin_date {
                bd_str = format!("{}-{}-{}", &bd[..4], &bd[4..6], &bd[6..8]);
                params.push(("begin_time", bd_str.as_str()));
            }
            if let Some(ed) = end_date {
                ed_str = format!("{}-{}-{}", &ed[..4], &ed[4..6], &ed[6..8]);
                params.push(("end_time", ed_str.as_str()));
            }

            let resp = self
                .get("https://np-anotice-stock.eastmoney.com/api/security/ann")
                .query(&params)
                .send()
                .await?
                .error_for_status()?;

            let payload: serde_json::Value = resp.json().await?;
            let total_hits = payload
                .pointer("/data/total_hits")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);

            let list = payload
                .pointer("/data/list")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            for item in list {
                if let serde_json::Value::Object(mut map) = item {
                    // Extract stock code from codes array
                    let codes = map.remove("codes").unwrap_or_default();
                    let stock_code = if let Some(arr) = codes.as_array() {
                        arr.first()
                            .and_then(|c| c.get("stock_code"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string()
                    } else {
                        String::new()
                    };

                    // Extract column name
                    let columns = map.remove("columns").unwrap_or_default();
                    let column_name = if let Some(arr) = columns.as_array() {
                        arr.first()
                            .and_then(|c| c.get("column_name"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string()
                    } else {
                        String::new()
                    };

                    let mut row: HashMap<String, serde_json::Value> = map.into_iter().collect();
                    row.insert(
                        "stock_code".to_string(),
                        serde_json::Value::String(stock_code),
                    );
                    row.insert(
                        "column_name".to_string(),
                        serde_json::Value::String(column_name),
                    );
                    all_items.push(row);
                }
            }

            let total_pages = total_hits.div_ceil(100);
            if page >= total_pages || total_hits == 0 {
                break;
            }
            page += 1;
        }

        Ok(all_items)
    }
}
