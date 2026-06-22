//! Tonghuashun (THS / 10jqka) fundamental data APIs.
//!
//! Covers: financial statements (abstract, balance, income, cash flow),
//! management/shareholder changes, IPO data, profit forecasts,
//! and main business introduction.

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::value_ext::ValueExt;

static RE_HTML_TAG: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<[^>]+>").expect("valid regex: html tag"));
static RE_TABLE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?is)<table[^>]*>(.*?)</table>").expect("valid regex: table")
});
static RE_TR: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<tr[^>]*>(.*?)</tr>").expect("valid regex: tr"));
static RE_TH: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<th[^>]*>(.*?)</th>").expect("valid regex: th"));
static RE_TD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<td[^>]*>(.*?)</td>").expect("valid regex: td"));
static RE_JSON_P: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"(?s)<p\s+id="main"[^>]*>(.*?)</p>"#).expect("valid regex: json p")
});
static RE_TABLE_DATA_HL: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r#"(?is)<table[^>]*class="[^"]*data_table_1\s+m_table\s+m_hl[^"]*"[^>]*>(.*?)</table>"#,
    )
    .expect("valid regex: data_table_1 m_table m_hl")
});
static RE_TABLE_M_DATA_HL: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r#"(?is)<table[^>]*class="[^"]*m_table\s+data_table_1\s+m_hl[^"]*"[^>]*>(.*?)</table>"#,
    )
    .expect("valid regex: m_table data_table_1 m_hl")
});
static RE_TABLE_MAINTABLE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"(?is)<table[^>]*id="maintable"[^>]*>(.*?)</table>"#)
        .expect("valid regex: maintable")
});
static RE_TABLE_M_TABLE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"(?is)<table[^>]*class="[^"]*m_table[^"]*"[^>]*>(.*?)</table>"#)
        .expect("valid regex: m_table")
});
static RE_UL_MAIN_INTRO: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"(?is)<ul[^>]*class="[^"]*main_intro_list[^"]*"[^>]*>(.*?)</ul>"#)
        .expect("valid regex: main_intro_list")
});
static RE_LI: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<li[^>]*>(.*?)</li>").expect("valid regex: li"));

// ---------------------------------------------------------------------------
// Shared THS headers
// ---------------------------------------------------------------------------

fn ths_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"
            .parse()
            .expect("valid User-Agent header value"),
    );
    headers
}

// ---------------------------------------------------------------------------
// Market code helper for THS new API
// ---------------------------------------------------------------------------

fn ths_market_code(stock_code: &str) -> i32 {
    let code = stock_code.trim();
    if code.starts_with("000")
        || code.starts_with("001")
        || code.starts_with("002")
        || code.starts_with("003")
        || code.starts_with("300")
    {
        return 33; // Shenzhen
    }
    if code.starts_with("600")
        || code.starts_with("601")
        || code.starts_with("603")
        || code.starts_with("605")
        || code.starts_with("688")
    {
        return 17; // Shanghai
    }
    if code.starts_with("920") {
        return 151; // Beijing
    }
    0
}

// ---------------------------------------------------------------------------
// Period mapping for THS new API
// ---------------------------------------------------------------------------

fn ths_period_code(indicator: &str) -> &str {
    match indicator {
        "按报告期" => "0",
        "一季度" => "1",
        "二季度" => "2",
        "三季度" => "3",
        _ => "4",
    }
}

// ---------------------------------------------------------------------------
// HTML table parsing (reused from sina module pattern)
// ---------------------------------------------------------------------------

fn strip_html_tags(s: &str) -> String {
    RE_HTML_TAG.replace_all(s, "").to_string()
}

fn parse_html_tables(html: &str) -> Vec<(Vec<String>, Vec<Vec<String>>)> {
    let mut tables = Vec::new();
    for table_cap in RE_TABLE.captures_iter(html) {
        let table_content = &table_cap[1];

        let mut rows = Vec::new();
        for row_cap in RE_TR.captures_iter(table_content) {
            let row_content = &row_cap[1];
            let ths: Vec<String> = RE_TH
                .captures_iter(row_content)
                .map(|c| strip_html_tags(&c[1]).trim().to_string())
                .collect();
            if !ths.is_empty() {
                rows.push(ths);
                continue;
            }
            let tds: Vec<String> = RE_TD
                .captures_iter(row_content)
                .map(|c| strip_html_tags(&c[1]).trim().to_string())
                .collect();
            if !tds.is_empty() {
                rows.push(tds);
            }
        }

        if rows.is_empty() {
            continue;
        }

        let headers = rows[0].clone();
        let data_rows = rows[1..].to_vec();
        tables.push((headers, data_rows));
    }
    tables
}

fn table_to_records(
    headers: &[String],
    rows: &[Vec<String>],
) -> Vec<HashMap<String, serde_json::Value>> {
    rows.iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            let mut map = HashMap::new();
            for (i, header) in headers.iter().enumerate() {
                let val = row.get(i).cloned().unwrap_or_default();
                map.insert(header.clone(), serde_json::Value::String(val));
            }
            map
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    // -----------------------------------------------------------------------
    // THS old-style financial APIs (JSON embedded in HTML)
    // -----------------------------------------------------------------------

    /// THS financial abstract (主要指标) - old API.
    ///
    /// `symbol` is the stock code, e.g. "000063".
    /// `indicator` is one of "按报告期", "按年度", "按单季度".
    pub async fn stock_financial_abstract_ths(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!("https://basic.10jqka.com.cn/new/{symbol}/finance.html");
        let html = self
            .get(&url)
            .headers(ths_headers())
            .send()
            .await?
            .text()
            .await?;

        // Extract JSON from <p id="main">...</p>
        let json_str = RE_JSON_P
            .captures(&html)
            .map(|c| c[1].trim().to_string())
            .ok_or_else(|| Error::upstream("THS finance page: main JSON not found"))?;

        let data_json: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| Error::decode(format!("THS finance JSON parse error: {e}")))?;

        Ok(Self::parse_ths_finance_json(
            &data_json, indicator, "report",
        ))
    }

    /// THS balance sheet (资产负债表) - old API.
    pub async fn stock_financial_debt(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!("https://basic.10jqka.com.cn/api/stock/finance/{symbol}_debt.json");
        self.fetch_ths_json_table(&url, indicator, false).await
    }

    /// THS income statement (利润表) - old API.
    pub async fn stock_financial_benefit(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!("https://basic.10jqka.com.cn/api/stock/finance/{symbol}_benefit.json");
        self.fetch_ths_json_table(&url, indicator, true).await
    }

    /// THS cash flow statement (现金流量表) - old API.
    pub async fn stock_financial_cash(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!("https://basic.10jqka.com.cn/api/stock/finance/{symbol}_cash.json");
        self.fetch_ths_json_table(&url, indicator, true).await
    }

    async fn fetch_ths_json_table(
        &self,
        url: &str,
        indicator: &str,
        has_simple: bool,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let resp = self
            .get(url)
            .headers(ths_headers())
            .send()
            .await?
            .error_for_status()?;

        let outer: serde_json::Value = resp.json().await?;
        let flash_data_str = outer
            .get("flashData")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::upstream("THS response missing flashData"))?;

        // flashData is a JSON string that needs to be parsed again
        let inner_str = if flash_data_str.starts_with('"') {
            // Double-encoded JSON string
            serde_json::from_str::<String>(flash_data_str)
                .map_err(|e| Error::decode(format!("THS flashData decode error: {e}")))?
        } else {
            flash_data_str.to_string()
        };

        let data_json: serde_json::Value = serde_json::from_str(&inner_str)
            .map_err(|e| Error::decode(format!("THS flashData JSON parse error: {e}")))?;

        let default_key = if has_simple { "simple" } else { "report" };
        Ok(Self::parse_ths_finance_json(
            &data_json,
            indicator,
            default_key,
        ))
    }

    fn parse_ths_finance_json(
        data_json: &serde_json::Value,
        indicator: &str,
        default_key: &str,
    ) -> Vec<HashMap<String, serde_json::Value>> {
        // Extract title (row labels)
        let titles: Vec<String> = data_json
            .get("title")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|item| {
                        if let Some(s) = item.as_str() {
                            s.to_string()
                        } else if let Some(arr) = item.as_array() {
                            arr.first()
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string()
                        } else {
                            String::new()
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Select the data key based on indicator
        let data_key = match indicator {
            "按报告期" => "report",
            "按单季度" => {
                if data_json.get("simple").is_some() {
                    "simple"
                } else {
                    default_key
                }
            }
            "按年度" => "year",
            _ => default_key,
        };

        let table_data = data_json
            .get(data_key)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if table_data.is_empty() {
            return vec![];
        }

        // First row is column headers (report dates)
        let column_headers: Vec<String> = table_data
            .first()
            .and_then(|row| row.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect()
            })
            .unwrap_or_default();

        // Data rows start from index 1
        let mut all_records = Vec::new();
        for (row_idx, row_val) in table_data.iter().enumerate().skip(1) {
            let cells: Vec<String> = row_val
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect()
                })
                .unwrap_or_default();

            let row_label = titles.get(row_idx).cloned().unwrap_or_default();

            // Each column (after the first) is a report period
            for (col_idx, col_header) in column_headers.iter().enumerate().skip(1) {
                let mut record = HashMap::new();
                record.insert(
                    "报告期".to_string(),
                    serde_json::Value::String(col_header.clone()),
                );
                record.insert(
                    "指标".to_string(),
                    serde_json::Value::String(row_label.clone()),
                );
                if let Some(val) = cells.get(col_idx) {
                    record.insert("值".to_string(), serde_json::Value::String(val.clone()));
                }
                all_records.push(record);
            }
        }

        all_records
    }

    // -----------------------------------------------------------------------
    // THS new-style financial APIs
    // -----------------------------------------------------------------------

    /// THS new financial abstract (重要指标).
    ///
    /// `symbol` is the stock code, e.g. "000063".
    /// `indicator` is one of "按报告期", "一季度", "二季度", "三季度", "四季度", "按年度".
    pub async fn stock_financial_abstract_new(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        self.fetch_ths_new_api(symbol, "client_stock_importance", indicator)
            .await
    }

    /// THS new balance sheet (资产负债表).
    pub async fn stock_financial_debt_new(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let ind = if indicator == "按报告期" {
            "按报告期"
        } else {
            "按年度"
        };
        self.fetch_ths_new_api(symbol, "client_stock_debt", ind)
            .await
    }

    /// THS new income statement (利润表).
    pub async fn stock_financial_benefit_new(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        self.fetch_ths_new_api(symbol, "client_stock_benefit", indicator)
            .await
    }

    /// THS new cash flow statement (现金流量表).
    pub async fn stock_financial_cash_new(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        self.fetch_ths_new_api(symbol, "client_stock_cash", indicator)
            .await
    }

    async fn fetch_ths_new_api(
        &self,
        symbol: &str,
        api_id: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = "https://basic.10jqka.com.cn/basicapi/finance/index/v1/app_data/";
        let market = ths_market_code(symbol).to_string();
        let period = ths_period_code(indicator);

        let resp = self
            .get(url)
            .headers(ths_headers())
            .query(&[
                ("code", symbol),
                ("id", api_id),
                ("market", market.as_str()),
                ("type", "stock"),
                ("page", "1"),
                ("size", "50"),
                ("period", period),
            ])
            .send()
            .await?
            .error_for_status()?;

        let payload: serde_json::Value = resp.json().await?;
        let financial_data = payload
            .pointer("/data/data")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut all_records = Vec::new();
        for report in &financial_data {
            let report_date = report.str_or(&["date"], "");
            let report_name = report.str_or(&["report_name"], "");
            let report_period = report.str_or(&["report"], "");
            let quarter_name = report.str_or(&["quarter_name"], "");

            let Some(serde_json::Value::Object(index_list)) = report.get("index_list") else {
                continue;
            };

            for (metric_name, metric_values) in index_list {
                let mut record = HashMap::new();
                record.insert(
                    "report_date".to_string(),
                    serde_json::Value::String(report_date.clone()),
                );
                record.insert(
                    "report_name".to_string(),
                    serde_json::Value::String(report_name.clone()),
                );
                record.insert(
                    "report_period".to_string(),
                    serde_json::Value::String(report_period.clone()),
                );
                record.insert(
                    "quarter_name".to_string(),
                    serde_json::Value::String(quarter_name.clone()),
                );
                record.insert(
                    "metric_name".to_string(),
                    serde_json::Value::String(metric_name.clone()),
                );

                if let serde_json::Value::Object(vals) = metric_values {
                    for (field, value) in vals {
                        record.insert(field.clone(), value.clone());
                    }
                } else {
                    record.insert("value".to_string(), metric_values.clone());
                }

                all_records.push(record);
            }
        }

        Ok(all_records)
    }

    // -----------------------------------------------------------------------
    // Management / shareholder changes
    // -----------------------------------------------------------------------

    /// THS management shareholding changes (高管持股变动).
    pub async fn stock_management_change(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!("https://basic.10jqka.com.cn/new/{symbol}/event.html");
        let html = self
            .get(&url)
            .headers(ths_headers())
            .send()
            .await?
            .text()
            .await?;

        // Parse the management change table
        // Look for table with class "data_table_1 m_table m_hl"
        let table_content = match RE_TABLE_DATA_HL.captures(&html) {
            Some(cap) => cap[1].to_string(),
            None => return Ok(vec![]),
        };

        let tables = parse_html_tables(&format!("<table>{table_content}</table>"));
        if tables.is_empty() {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[0];
        Ok(table_to_records(headers, rows))
    }

    /// THS shareholder shareholding changes (股东持股变动).
    pub async fn stock_shareholder_change(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!("https://basic.10jqka.com.cn/new/{symbol}/event.html");
        let html = self
            .get(&url)
            .headers(ths_headers())
            .send()
            .await?
            .text()
            .await?;

        // Look for table with class "m_table data_table_1 m_hl"
        let table_content = match RE_TABLE_M_DATA_HL.captures(&html) {
            Some(cap) => cap[1].to_string(),
            None => return Ok(vec![]),
        };

        let tables = parse_html_tables(&format!("<table>{table_content}</table>"));
        if tables.is_empty() {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[0];
        Ok(table_to_records(headers, rows))
    }

    // -----------------------------------------------------------------------
    // THS IPO data
    // -----------------------------------------------------------------------

    /// THS IPO subscription and allotment data (新股申购与中签).
    ///
    /// `market` is one of: "全部A股", "沪市主板", "深市主板", "创业板", "科创板", "京市主板".
    pub async fn stock_ipo(&self, market: &str) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let market_map: HashMap<&str, &str> = [
            ("全部A股", "all"),
            ("沪市主板", "hszb"),
            ("深市主板", "sszb"),
            ("创业板", "cyb"),
            ("科创板", "kcbsg"),
            ("京市主板", "bjzb"),
        ]
        .into_iter()
        .collect();

        let path = market_map
            .get(market)
            .copied()
            .ok_or_else(|| Error::invalid_input(format!("unsupported market: {market}")))?;

        let url = format!("https://data.10jqka.com.cn/ipo/xgsgyzq/{path}/");
        let html = self
            .get(&url)
            .headers(ths_headers())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        // Parse the main table
        let table_content = RE_TABLE_MAINTABLE
            .captures(&html)
            .or_else(|| RE_TABLE_M_TABLE.captures(&html))
            .map(|c| c[1].to_string());

        let Some(content) = table_content else {
            return Ok(vec![]);
        };

        let tables = parse_html_tables(&format!("<table>{content}</table>"));
        if tables.is_empty() {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[0];
        Ok(table_to_records(headers, rows))
    }

    /// THS HK IPO subscription and allotment data (港股新股申购与中签).
    pub async fn stock_ipo_hk(&self) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = "https://data.10jqka.com.cn/ipo/xgsgyzq/hkstock/";
        let html = self
            .get(url)
            .headers(ths_headers())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let table_content = RE_TABLE_MAINTABLE
            .captures(&html)
            .or_else(|| RE_TABLE_M_TABLE.captures(&html))
            .map(|c| c[1].to_string());

        let Some(content) = table_content else {
            return Ok(vec![]);
        };

        let tables = parse_html_tables(&format!("<table>{content}</table>"));
        if tables.is_empty() {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[0];
        Ok(table_to_records(headers, rows))
    }

    // -----------------------------------------------------------------------
    // THS profit forecast
    // -----------------------------------------------------------------------

    /// THS profit forecast (盈利预测).
    ///
    /// `symbol` is the stock code, e.g. "600519".
    /// `indicator` is one of: "预测年报每股收益", "预测年报净利润",
    /// "业绩预测详表-机构", "业绩预测详表-详细指标预测".
    pub async fn stock_profit_forecast_ths(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!("https://basic.10jqka.com.cn/new/{symbol}/worth.html");
        let resp = self.get(&url).headers(ths_headers()).send().await?;

        let bytes = resp.bytes().await?;
        let html = String::from_utf8_lossy(&bytes).to_string();

        let tables = parse_html_tables(&html);

        // Check if there's a "no forecast" message
        if html.contains("本年度暂无机构做出业绩预测") {
            match indicator {
                "业绩预测详表-机构" => {
                    if tables.is_empty() {
                        return Ok(vec![]);
                    }
                    let (headers, rows) = &tables[0];
                    return Ok(table_to_records(headers, rows));
                }
                "业绩预测详表-详细指标预测" => {
                    if tables.len() < 2 {
                        return Ok(vec![]);
                    }
                    let (headers, rows) = &tables[1];
                    return Ok(table_to_records(headers, rows));
                }
                _ => return Ok(vec![]),
            }
        }

        let table_idx = match indicator {
            "预测年报每股收益" => 0,
            "预测年报净利润" => 1,
            "业绩预测详表-机构" => 2,
            "业绩预测详表-详细指标预测" => 3,
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported indicator: {indicator}"
                )));
            }
        };

        if table_idx >= tables.len() {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[table_idx];
        Ok(table_to_records(headers, rows))
    }

    // -----------------------------------------------------------------------
    // THS main business introduction
    // -----------------------------------------------------------------------

    /// THS main business introduction (主营介绍).
    ///
    /// `symbol` is the stock code, e.g. "000066".
    pub async fn stock_zyjs(&self, symbol: &str) -> Result<HashMap<String, serde_json::Value>> {
        let url = format!("https://basic.10jqka.com.cn/new/{symbol}/operate.html");
        let resp = self.get(&url).headers(ths_headers()).send().await?;

        let bytes = resp.bytes().await?;
        let html = String::from_utf8_lossy(&bytes).to_string();

        // Extract the main_intro_list items
        let mut result = HashMap::new();
        result.insert(
            "股票代码".to_string(),
            serde_json::Value::String(symbol.to_string()),
        );

        if let Some(list_cap) = RE_UL_MAIN_INTRO.captures(&html) {
            let list_content = &list_cap[1];
            for li_cap in RE_LI.captures_iter(list_content) {
                let text = strip_html_tags(&li_cap[1])
                    .replace(['\t', '\n'], "")
                    .replace("  ", "")
                    .trim()
                    .to_string();

                if let Some((key, value)) = text.split_once('：') {
                    result.insert(
                        key.trim().to_string(),
                        serde_json::Value::String(value.trim().to_string()),
                    );
                } else if let Some((key, value)) = text.split_once(':') {
                    result.insert(
                        key.trim().to_string(),
                        serde_json::Value::String(value.trim().to_string()),
                    );
                }
            }
        }

        Ok(result)
    }

    // -----------------------------------------------------------------------
    // HK profit forecast from ET Net
    // -----------------------------------------------------------------------

    /// HK stock profit forecast from ET Net (经济通).
    ///
    /// `symbol` is the HK stock code, e.g. "09999".
    /// `indicator` is one of: "评级总览", "去年度业绩表现", "综合盈利预测", "盈利预测概览".
    pub async fn stock_hk_profit_forecast_et(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let code = symbol.trim_start_matches('0');
        let code = if code.is_empty() { "0" } else { code };

        let url = "https://www.etnet.com.hk/www/sc/stocks/realtime/quote_profit.php";
        let html = self
            .get(url)
            .query(&[("code", code)])
            .send()
            .await?
            .text()
            .await?;

        let tables = parse_html_tables(&html);

        match indicator {
            "评级总览" => {
                if tables.is_empty() {
                    return Ok(vec![]);
                }
                let (headers, rows) = &tables[0];
                Ok(table_to_records(headers, rows))
            }
            "去年度业绩表现" => {
                if tables.len() < 3 {
                    return Ok(vec![]);
                }
                let (headers, rows) = &tables[2];
                Ok(table_to_records(headers, rows))
            }
            "综合盈利预测" => {
                if tables.len() < 4 {
                    return Ok(vec![]);
                }
                let (headers, rows) = &tables[3];
                Ok(table_to_records(headers, rows))
            }
            "盈利预测概览" => {
                if tables.len() < 5 {
                    return Ok(vec![]);
                }
                let (headers, rows) = &tables[4];
                Ok(table_to_records(headers, rows))
            }
            _ => Err(Error::invalid_input(format!(
                "unsupported indicator: {indicator}"
            ))),
        }
    }
}
