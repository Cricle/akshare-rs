#![allow(dead_code)]
//! Sina Finance fundamental data APIs.
//!
//! Covers: financial reports (balance/income/cash flow), financial abstracts,
//! financial analysis indicators, dividends, IPO info, additional stock issuance,
//! restricted releases, shareholders (circulating/fund/main), institutional
//! holdings, and institutional recommendations.

use std::collections::HashMap;
use std::sync::LazyLock;

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

static RE_TABLE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<table[^>]*>(.*?)</table>").unwrap());
static RE_THEAD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<thead[^>]*>(.*?)</thead>").unwrap());
static RE_TR: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<tr[^>]*>(.*?)</tr>").unwrap());
static RE_TH: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<th[^>]*>(.*?)</th>").unwrap());
static RE_TD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<td[^>]*>(.*?)</td>").unwrap());
static RE_HTML_TAG: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<[^>]+>").unwrap());
static RE_YEAR_TABLE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#"id="con02-1".*?<table[^>]*>(.*?)</table>"#).unwrap());
static RE_LINK_YEAR: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#"<a[^>]*>(\d{4})</a>"#).unwrap());
static RE_MENU_LINK: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"<li[^>]*><a[^>]*href="([^"]*)"[^>]*>([^<]*)</a></li>"#).unwrap()
});

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct SinaFinanceReportEnvelope {
    result: Option<SinaFinanceReportResult>,
}

#[derive(Debug, Deserialize)]
struct SinaFinanceReportResult {
    data: Option<SinaFinanceReportData>,
}

#[derive(Debug, Deserialize)]
struct SinaFinanceReportData {
    report_date: Option<Vec<ReportDateItem>>,
    report_list: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct ReportDateItem {
    date_value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SinaInstituteDetailEnvelope {
    data: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// HTML table parsing helper
// ---------------------------------------------------------------------------

/// Parse an HTML table into rows of strings using regex-based extraction.
/// Returns (headers, rows).
fn parse_html_tables(html: &str) -> Vec<(Vec<String>, Vec<Vec<String>>)> {
    let mut tables = Vec::new();

    // Find all <table>...</table> blocks
    for table_cap in RE_TABLE.captures_iter(html) {
        let table_content = &table_cap[1];
        let mut headers = Vec::new();
        let mut rows = Vec::new();

        // Extract headers from <thead> or first <tr> with <th>
        let header_content = RE_THEAD
            .captures(table_content)
            .map_or_else(|| table_content.to_string(), |c| c[1].to_string());

        let mut found_header = false;
        for row_cap in RE_TR.captures_iter(&header_content) {
            let row_content = &row_cap[1];
            let ths: Vec<String> = RE_TH
                .captures_iter(row_content)
                .map(|c| strip_html_tags(&c[1]).trim().to_string())
                .collect();
            if !ths.is_empty() && !found_header {
                headers = ths;
                found_header = true;
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

        // If no headers found in thead, try first row
        if headers.is_empty()
            && let Some(first_row) = rows.first()
        {
            headers = first_row.clone();
            rows.remove(0);
        }

        if !headers.is_empty() || !rows.is_empty() {
            tables.push((headers, rows));
        }
    }

    tables
}

/// Strip HTML tags from a string.
fn strip_html_tags(s: &str) -> String {
    RE_HTML_TAG.replace_all(s, "").to_string()
}

/// Convert HTML table data to a vec of HashMaps.
fn table_to_records(
    headers: &[String],
    rows: &[Vec<String>],
) -> Vec<HashMap<String, serde_json::Value>> {
    rows.iter()
        .filter(|row| row.len() >= headers.len())
        .map(|row| {
            let mut map = HashMap::new();
            for (i, header) in headers.iter().enumerate() {
                if i < row.len() {
                    map.insert(header.clone(), serde_json::Value::String(row[i].clone()));
                }
            }
            map
        })
        .collect()
}

/// Fetch and parse an HTML page containing tables, return the Nth table as records.
async fn fetch_html_table(
    http: &reqwest::Client,
    url: &str,
    table_index: usize,
    referer: Option<&str>,
) -> Result<Vec<HashMap<String, serde_json::Value>>> {
    let mut req = http.get(url);
    if let Some(ref_url) = referer {
        req = req.header("Referer", ref_url);
    }
    let html = req.send().await?.text().await?;
    let tables = parse_html_tables(&html);

    if table_index >= tables.len() {
        return Ok(vec![]);
    }

    let (headers, rows) = &tables[table_index];
    Ok(table_to_records(headers, rows))
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    // -----------------------------------------------------------------------
    // Financial reports from Sina
    // -----------------------------------------------------------------------

    /// Sina financial report (三大报表).
    ///
    /// `stock` is the Sina-format code, e.g. "sh600600".
    /// `symbol` is one of "资产负债表", "利润表", "现金流量表".
    pub async fn stock_financial_report_sina(
        &self,
        stock: &str,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let source = match symbol {
            "资产负债表" => "fzb",
            "利润表" => "lrb",
            "现金流量表" => "llb",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported symbol: {symbol}"
                )));
            }
        };

        let resp = self
                        .get("https://quotes.sina.cn/cn/api/openapi.php/CompanyFinanceService.getFinanceReport2022")
            .query(&[
                ("paperCode", stock),
                ("source", source),
                ("type", "0"),
                ("page", "1"),
                ("num", "1000"),
            ])
            .send()
            .await?
            .error_for_status()?;

        let payload: SinaFinanceReportEnvelope = resp.json().await?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("sina financial report missing data"))?;

        let dates = data.report_date.unwrap_or_default();
        let date_values: Vec<String> = dates.iter().filter_map(|d| d.date_value.clone()).collect();

        let report_list = data.report_list.unwrap_or_default();
        let mut all_rows = Vec::new();

        for date_str in &date_values {
            let report = report_list.get(date_str);
            if let Some(report) = report {
                let items = report
                    .get("data")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut row = HashMap::new();
                row.insert(
                    "报告日".to_string(),
                    serde_json::Value::String(date_str.clone()),
                );

                // Add metadata
                if let Some(ds) = report.get("data_source").and_then(|v| v.as_str()) {
                    row.insert(
                        "数据源".to_string(),
                        serde_json::Value::String(ds.to_string()),
                    );
                }
                if let Some(audit) = report.get("is_audit").and_then(|v| v.as_str()) {
                    row.insert(
                        "是否审计".to_string(),
                        serde_json::Value::String(audit.to_string()),
                    );
                }
                if let Some(pd) = report.get("publish_date").and_then(|v| v.as_str()) {
                    row.insert(
                        "公告日期".to_string(),
                        serde_json::Value::String(pd.to_string()),
                    );
                }
                if let Some(currency) = report.get("rCurrency").and_then(|v| v.as_str()) {
                    row.insert(
                        "币种".to_string(),
                        serde_json::Value::String(currency.to_string()),
                    );
                }
                if let Some(rtype) = report.get("rType").and_then(|v| v.as_str()) {
                    row.insert(
                        "类型".to_string(),
                        serde_json::Value::String(rtype.to_string()),
                    );
                }

                for item in items {
                    if let (Some(title), Some(value)) = (
                        item.get("item_title").and_then(|v| v.as_str()),
                        item.get("item_value"),
                    ) {
                        row.insert(title.to_string(), value.clone());
                    }
                }
                all_rows.push(row);
            }
        }

        Ok(all_rows)
    }

    /// Sina financial abstract (关键指标).
    ///
    /// `symbol` is the stock code, e.g. "600600".
    pub async fn stock_financial_abstract(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let paper_code = if symbol.starts_with("sh") || symbol.starts_with("sz") {
            symbol.to_string()
        } else {
            format!("sh{symbol}")
        };

        let resp = self
                        .get("https://quotes.sina.cn/cn/api/openapi.php/CompanyFinanceService.getFinanceReport2022")
            .query(&[
                ("paperCode", paper_code.as_str()),
                ("source", "gjzb"),
                ("type", "0"),
                ("page", "1"),
                ("num", "1000"),
            ])
            .send()
            .await?
            .error_for_status()?;

        let payload: SinaFinanceReportEnvelope = resp.json().await?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("sina financial abstract missing data"))?;

        let report_list = data.report_list.unwrap_or_default();
        let keys: Vec<String> = if let serde_json::Value::Object(map) = &report_list {
            map.keys().cloned().collect()
        } else {
            return Ok(vec![]);
        };

        // Get the item titles from the first report
        let first_key = match keys.first() {
            Some(k) => k,
            None => return Ok(vec![]),
        };

        let first_items = report_list
            .get(first_key)
            .and_then(|v| v.get("data"))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let titles: Vec<String> = first_items
            .iter()
            .filter_map(|item| {
                item.get("item_title")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string)
            })
            .collect();

        // Build rows: each title is a row, each date is a column
        let mut all_rows = Vec::new();
        for (i, title) in titles.iter().enumerate() {
            let mut row = HashMap::new();
            row.insert("指标".to_string(), serde_json::Value::String(title.clone()));
            for key in &keys {
                let items = report_list
                    .get(key)
                    .and_then(|v| v.get("data"))
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                if let Some(item) = items.get(i)
                    && let Some(val) = item.get("item_value")
                {
                    row.insert(key.clone(), val.clone());
                }
            }
            all_rows.push(row);
        }

        Ok(all_rows)
    }

    // -----------------------------------------------------------------------
    // Historical dividends
    // -----------------------------------------------------------------------

    /// Sina historical dividend summary for all stocks.
    pub async fn stock_history_dividend(&self) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = "https://vip.stock.finance.sina.com.cn/q/go.php/vInvestConsult/kind/lsfh/index.phtml?p=1&num=50000";
        let html = self.get(url).send().await?.text().await?;
        let tables = parse_html_tables(&html);

        // The dividend table is typically the first table
        if tables.is_empty() {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[0];
        Ok(table_to_records(headers, rows))
    }

    /// Sina dividend/rights issue detail for a specific stock.
    ///
    /// `symbol` is the stock code, e.g. "000002".
    /// `indicator` is "分红" or "配股".
    /// `date` is optional, e.g. "1994-12-24" for specific date detail.
    pub async fn stock_history_dividend_detail(
        &self,
        symbol: &str,
        indicator: &str,
        date: Option<&str>,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!(
            "https://vip.stock.finance.sina.com.cn/corp/go.php/vISSUE_ShareBonus/stockid/{symbol}.phtml"
        );
        let html = self.get(&url).send().await?.text().await?;
        let tables = parse_html_tables(&html);

        let table_index = if indicator == "分红" { 12 } else { 13 };
        if table_index >= tables.len() {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[table_index];
        let records = table_to_records(headers, rows);

        // If date specified, fetch detail for that specific date
        if let Some(detail_date) = date {
            let detail_url =
                "https://vip.stock.finance.sina.com.cn/corp/view/vISSUE_ShareBonusDetail.php";
            let detail_html = self
                .get(detail_url)
                .query(&[
                    ("stockid", symbol),
                    ("type", "1"),
                    ("end_date", detail_date),
                ])
                .send()
                .await?
                .text()
                .await?;
            let detail_tables = parse_html_tables(&detail_html);
            if detail_tables.len() > 12 {
                let (h, r) = &detail_tables[12];
                return Ok(table_to_records(h, r));
            }
        }

        Ok(records)
    }

    // -----------------------------------------------------------------------
    // IPO info and additional stock issuance
    // -----------------------------------------------------------------------

    /// Sina IPO information for a stock.
    pub async fn stock_ipo_info(
        &self,
        stock: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!(
            "https://vip.stock.finance.sina.com.cn/corp/go.php/vISSUE_NewStock/stockid/{stock}.phtml"
        );
        fetch_html_table(&self.http, &url, 12, None).await
    }

    /// Sina additional stock issuance (增发) information.
    pub async fn stock_add_stock(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!(
            "https://vip.stock.finance.sina.com.cn/corp/go.php/vISSUE_AddStock/stockid/{symbol}.phtml"
        );
        let html = self.get(&url).send().await?.text().await?;
        let tables = parse_html_tables(&html);

        if tables.len() <= 12 {
            return Ok(vec![]);
        }

        // Check if there's data
        let (_, check_rows) = &tables[12];
        if check_rows.is_empty() {
            return Ok(vec![]);
        }

        // Parse additional stock issuance tables (starting from index 13)
        let mut all_records = Vec::new();
        let mut i = 13;
        while i < tables.len() {
            let (headers, rows) = &tables[i];
            for row in rows {
                if row.len() >= 2 {
                    let mut record = HashMap::new();
                    record.insert(
                        "公告日期".to_string(),
                        serde_json::Value::String(headers.first().cloned().unwrap_or_default()),
                    );
                    record.insert(
                        "发行方式".to_string(),
                        serde_json::Value::String(row.first().cloned().unwrap_or_default()),
                    );
                    record.insert(
                        "发行价格".to_string(),
                        serde_json::Value::String(row.get(1).cloned().unwrap_or_default()),
                    );
                    if row.len() > 2 {
                        record.insert(
                            "实际公司募集资金总额".to_string(),
                            serde_json::Value::String(row.get(2).cloned().unwrap_or_default()),
                        );
                    }
                    if row.len() > 3 {
                        record.insert(
                            "发行费用总额".to_string(),
                            serde_json::Value::String(row.get(3).cloned().unwrap_or_default()),
                        );
                    }
                    if row.len() > 4 {
                        record.insert(
                            "实际发行数量".to_string(),
                            serde_json::Value::String(row.get(4).cloned().unwrap_or_default()),
                        );
                    }
                    all_records.push(record);
                }
            }
            i += 1;
        }

        Ok(all_records)
    }

    // -----------------------------------------------------------------------
    // Restricted release queue (Sina)
    // -----------------------------------------------------------------------

    /// Sina restricted release queue for a stock.
    pub async fn stock_restricted_release_queue_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!(
            "https://vip.stock.finance.sina.com.cn/q/go.php/vInvestConsult/kind/xsjj/index.phtml?symbol={symbol}"
        );
        fetch_html_table(&self.http, &url, 0, None).await
    }

    // -----------------------------------------------------------------------
    // Shareholders
    // -----------------------------------------------------------------------

    /// Sina circulating stock holders (流通股东).
    pub async fn stock_circulate_stock_holder(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!(
            "https://vip.stock.finance.sina.com.cn/corp/go.php/vCI_CirculateStockHolder/stockid/{symbol}.phtml"
        );
        let html = self.get(&url).send().await?.text().await?;
        let tables = parse_html_tables(&html);

        if tables.len() <= 13 {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[13];
        parse_shareholder_table(headers, rows, "截止日期")
    }

    /// Sina fund stock holders (基金持股).
    pub async fn stock_fund_stock_holder(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!(
            "https://vip.stock.finance.sina.com.cn/corp/go.php/vCI_FundStockHolder/stockid/{symbol}.phtml"
        );
        let html = self.get(&url).send().await?.text().await?;
        let tables = parse_html_tables(&html);

        if tables.len() <= 13 {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[13];
        parse_shareholder_table(headers, rows, "截止日期")
    }

    /// Sina main stock holders (主要股东).
    pub async fn stock_main_stock_holder(
        &self,
        stock: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!(
            "https://vip.stock.finance.sina.com.cn/corp/go.php/vCI_StockHolder/stockid/{stock}.phtml"
        );
        let html = self.get(&url).send().await?.text().await?;
        let tables = parse_html_tables(&html);

        if tables.len() <= 13 {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[13];
        parse_shareholder_table(headers, rows, "截至日期")
    }

    // -----------------------------------------------------------------------
    // Financial analysis indicators
    // -----------------------------------------------------------------------

    /// Sina financial analysis indicators (财务指标).
    ///
    /// `symbol` is the stock code, e.g. "600519".
    /// `start_year` is the start year, e.g. "1900" for all years.
    pub async fn stock_financial_analysis_indicator(
        &self,
        symbol: &str,
        start_year: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!(
            "https://money.finance.sina.com.cn/corp/go.php/vFD_FinancialGuideLine/stockid/{symbol}/ctrl/2020/displaytype/4.phtml"
        );
        let html = self.get(&url).send().await?.text().await?;

        // Extract year list from the page
        let mut year_list = Vec::new();
        if let Some(cap) = RE_YEAR_TABLE.captures(&html) {
            let table_content = &cap[1];
            for link_cap in RE_LINK_YEAR.captures_iter(table_content) {
                year_list.push(link_cap[1].to_string());
            }
        }

        // Filter years up to start_year
        if let Some(pos) = year_list.iter().position(|y| y == start_year) {
            year_list.truncate(pos + 1);
        }

        if year_list.is_empty() {
            return Ok(vec![]);
        }

        let mut all_rows = Vec::new();
        for year in &year_list {
            let year_url = format!(
                "https://money.finance.sina.com.cn/corp/go.php/vFD_FinancialGuideLine/stockid/{symbol}/ctrl/{year}/displaytype/4.phtml"
            );
            let year_html = self.get(&year_url).send().await?.text().await?;
            let tables = parse_html_tables(&year_html);

            // The financial table is typically at index 12
            if tables.len() <= 12 {
                continue;
            }

            let (headers, rows) = &tables[12];
            if headers.is_empty() || rows.is_empty() {
                continue;
            }

            // Parse the indicator table structure
            // Headers are report dates (e.g., "2023-12-31", "2023-09-30", etc.)
            let report_dates: Vec<String> = headers[1..].to_vec();

            // Rows contain indicator categories and values
            let mut current_category = String::new();
            for row in rows {
                if row.is_empty() {
                    continue;
                }
                let first = &row[0];

                // Check if this is a category header
                let categories = [
                    "每股指标",
                    "盈利能力",
                    "成长能力",
                    "营运能力",
                    "偿债及资本结构",
                    "现金流量",
                    "其他指标",
                ];
                if categories.contains(&first.as_str()) {
                    current_category = first.clone();
                    continue;
                }

                // This is a data row
                for (j, date) in report_dates.iter().enumerate() {
                    if j + 1 < row.len() {
                        let mut record = HashMap::new();
                        record.insert("日期".to_string(), serde_json::Value::String(date.clone()));
                        record.insert(
                            "类别".to_string(),
                            serde_json::Value::String(current_category.clone()),
                        );
                        record.insert("指标".to_string(), serde_json::Value::String(first.clone()));
                        record.insert(
                            "值".to_string(),
                            serde_json::Value::String(row[j + 1].clone()),
                        );
                        all_rows.push(record);
                    }
                }
            }
        }

        Ok(all_rows)
    }

    // -----------------------------------------------------------------------
    // Institutional holdings
    // -----------------------------------------------------------------------

    /// Sina institutional holdings overview (机构持股一览表).
    ///
    /// `symbol` encodes year and quarter, e.g. "20201" for 2020 Q1.
    /// Quarter: 1=Q1, 2=mid-year, 3=Q3, 4=annual.
    pub async fn stock_institute_hold(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let report_date = &symbol[..symbol.len() - 1];
        let quarter = &symbol[symbol.len() - 1..];

        let url =
            "https://vip.stock.finance.sina.com.cn/q/go.php/vComStockHold/kind/jgcg/index.phtml";
        let html = self
            .get(url)
            .query(&[
                ("p", "1"),
                ("num", "10000"),
                ("reportdate", report_date),
                ("quarter", quarter),
            ])
            .send()
            .await?
            .text()
            .await?;

        let tables = parse_html_tables(&html);
        if tables.is_empty() {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[0];
        Ok(table_to_records(headers, rows))
    }

    /// Sina institutional holdings detail for a specific stock.
    ///
    /// `stock` is the stock code, e.g. "600433".
    /// `quarter` encodes year and quarter, e.g. "20201".
    pub async fn stock_institute_hold_detail(
        &self,
        stock: &str,
        quarter: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = "https://vip.stock.finance.sina.com.cn/q/api/jsonp.php/var%20details=/ComStockHoldService.getJGCGDetail";
        let body = self
            .get(url)
            .query(&[("symbol", stock), ("quarter", quarter)])
            .send()
            .await?
            .text()
            .await?;

        // Parse JSONP: var details=({...});
        let json_start = body.find('{').unwrap_or(0);
        let json_end = body.rfind(')').unwrap_or(body.len());
        if json_start >= json_end {
            return Ok(vec![]);
        }
        let json_str = &body[json_start..json_end];

        let payload: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("failed to parse institute detail JSON: {e}")))?;

        let data = match payload.get("data") {
            Some(serde_json::Value::Object(map)) => map,
            _ => return Ok(vec![]),
        };

        let mut all_records = Vec::new();
        for (_key, inner) in data {
            if let serde_json::Value::Object(items) = inner {
                for (inst_key, values) in items {
                    if inst_key == "total" {
                        continue;
                    }
                    if let serde_json::Value::Object(vals) = values {
                        let mut record = HashMap::new();
                        // Parse institute type from key (e.g., "fund_123456")
                        let inst_type = inst_key.split('_').next().unwrap_or(inst_key);
                        let display_type = match inst_type {
                            "fund" => "基金",
                            "socialSecurity" => "全国社保",
                            "qfii" => "QFII",
                            "insurance" => "保险",
                            other => other,
                        };
                        record.insert(
                            "持股机构类型".to_string(),
                            serde_json::Value::String(display_type.to_string()),
                        );
                        for (field, val) in vals {
                            record.insert(field.clone(), val.clone());
                        }
                        all_records.push(record);
                    }
                }
            }
        }

        Ok(all_records)
    }

    // -----------------------------------------------------------------------
    // Institutional recommendations
    // -----------------------------------------------------------------------

    /// Sina institutional recommendations (机构推荐池).
    ///
    /// `symbol` is one of: "最新投资评级", "上调评级股票", "下调评级股票",
    /// "股票综合评级", "首次评级股票", "目标涨幅排名", "机构关注度",
    /// "行业关注度", "投资评级选股".
    pub async fn stock_institute_recommend(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let base_url = "http://stock.finance.sina.com.cn/stock/go.php/vIR_RatingNewest/index.phtml";

        // First, get the page to find the correct URL for the requested symbol
        let index_html = self
            .get(base_url)
            .query(&[("num", "40"), ("p", "1")])
            .send()
            .await?
            .text()
            .await?;

        // Extract the URL map from the left menu
        // Look for links in the menu that match our symbol
        let mut target_url = None;
        for cap in RE_MENU_LINK.captures_iter(&index_html) {
            let link_text = &cap[2];
            if link_text.trim() == symbol {
                target_url = Some(cap[1].to_string());
                break;
            }
        }

        let url = match target_url {
            Some(u) => u,
            None => {
                return Err(Error::not_found(format!(
                    "recommendation type not found: {symbol}"
                )));
            }
        };

        let html = self
            .get(&url)
            .query(&[("num", "10000"), ("p", "1")])
            .send()
            .await?
            .text()
            .await?;

        let tables = parse_html_tables(&html);
        if tables.is_empty() {
            return Ok(vec![]);
        }

        // Use the first table (index 0)
        let (headers, rows) = &tables[0];
        Ok(table_to_records(headers, rows))
    }

    /// Sina institutional recommendation detail for a specific stock.
    ///
    /// `symbol` is the stock code, e.g. "000001".
    pub async fn stock_institute_recommend_detail(
        &self,
        symbol: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let url = format!(
            "http://stock.finance.sina.com.cn/stock/go.php/vIR_StockSearch/key/{symbol}.phtml"
        );
        let html = self
            .get(&url)
            .query(&[("num", "5000"), ("p", "1")])
            .send()
            .await?
            .text()
            .await?;

        let tables = parse_html_tables(&html);
        if tables.is_empty() {
            return Ok(vec![]);
        }

        let (headers, rows) = &tables[0];
        Ok(table_to_records(headers, rows))
    }
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

/// Parse Sina shareholder tables that have date-grouped sections.
///
/// These tables have a structure where every N rows is a new date group:
/// - Row 0: date info ("截至日期: xxxx-xx-xx")
/// - Row 1: announcement date or column headers
/// - Rows 2+: data rows
fn parse_shareholder_table(
    _headers: &[String],
    rows: &[Vec<String>],
    date_field: &str,
) -> Result<Vec<HashMap<String, serde_json::Value>>> {
    if rows.is_empty() {
        return Ok(vec![]);
    }

    // Simple approach: treat each row as a record with the available columns
    // The actual structure varies by table, so we use a generic approach
    let mut all_records = Vec::new();
    let mut current_date = String::new();
    let mut current_ann_date = String::new();

    // Try to detect the structure
    for row in rows {
        if row.is_empty() {
            continue;
        }

        // Check if this is a date header row
        if row[0].contains("截止日期") || row[0].contains("截至日期") {
            // Next element might be the date
            if row.len() > 1 {
                current_date = row[1].clone();
            }
            continue;
        }
        if row[0].contains("公告日期") {
            if row.len() > 1 {
                current_ann_date = row[1].clone();
            }
            continue;
        }

        // Check if this looks like a column header row
        if row[0] == "编号" || row[0] == "股东名称" || row[0] == "基金名称" {
            continue;
        }

        // Skip rows that are all empty
        if row.iter().all(|s| s.trim().is_empty()) {
            continue;
        }

        let mut record = HashMap::new();
        if !current_date.is_empty() {
            record.insert(
                date_field.to_string(),
                serde_json::Value::String(current_date.clone()),
            );
        }
        if !current_ann_date.is_empty() {
            record.insert(
                "公告日期".to_string(),
                serde_json::Value::String(current_ann_date.clone()),
            );
        }

        // Add all cell values with generic column names
        for (i, cell) in row.iter().enumerate() {
            record.insert(format!("col_{i}"), serde_json::Value::String(cell.clone()));
        }

        all_records.push(record);
    }

    Ok(all_records)
}
