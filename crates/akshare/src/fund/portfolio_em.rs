//! Fund portfolio data from Eastmoney.
//!
//! The upstream `FundArchivesDatas.aspx` endpoint returns a JavaScript
//! assignment such as:
//!
//! ```text
//! var apidata={ content:"<div>…tables…</div>",arryear:[…],curyear:2024};
//! ```
//!
//! The value of `content` is an HTML fragment containing one or more
//! `<h4 class="t">` section headings followed by the matching tables.  We
//! extract that fragment (handling JavaScript string escapes), split it into
//! sections, and convert the tables into JSON objects with the same column
//! names used by the Python akshare implementation.

use regex::Regex;
use serde_json::{Value, json};

use crate::client::AkShareClient;
use crate::error::{Error, Result};

const ARCHIVES_URL: &str = "https://fundf10.eastmoney.com/FundArchivesDatas.aspx";

/// Decode the small subset of JavaScript string escapes used by Eastmoney.
fn decode_js_string(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.char_indices().peekable();
    while let Some((_, ch)) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        let Some((_, escaped)) = chars.next() else {
            out.push('\\');
            break;
        };
        match escaped {
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            'b' => out.push('\u{0008}'),
            'f' => out.push('\u{000c}'),
            '"' => out.push('"'),
            '\'' => out.push('\''),
            '\\' => out.push('\\'),
            '/' => out.push('/'),
            'u' => {
                let mut hex = String::with_capacity(4);
                for _ in 0..4 {
                    if let Some((_, digit)) = chars.next() {
                        hex.push(digit);
                    }
                }
                if let Ok(code) = u32::from_str_radix(&hex, 16)
                    && let Some(decoded) = char::from_u32(code)
                {
                    out.push(decoded);
                } else {
                    out.push_str("\\u");
                    out.push_str(&hex);
                }
            }
            other => {
                // Unknown escapes are kept verbatim.
                out.push('\\');
                out.push(other);
            }
        }
    }
    out
}

/// Extract the JavaScript string assigned to `content` / `"content"`.
fn extract_content(text: &str) -> Option<String> {
    let search_start = text.find("apidata").unwrap_or(0);
    let slice = &text[search_start..];
    let key_start = slice.find("content")?;
    let after_key = &slice[key_start + "content".len()..];
    let colon = after_key.find(':')?;
    let after_colon = after_key[colon + 1..].trim_start();

    let quote = after_colon.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let body = &after_colon[quote.len_utf8()..];
    let mut escaped = false;
    for (index, ch) in body.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == quote {
            return Some(decode_js_string(&body[..index]));
        }
    }
    None
}

/// Normalize a table header: remove whitespace and use full-width brackets.
fn normalize_header(header: &str) -> String {
    header
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| match ch {
            '(' => '（',
            ')' => '）',
            other => other,
        })
        .collect()
}

fn header_index(headers: &[String], aliases: &[&str]) -> Option<usize> {
    for alias in aliases {
        if let Some(index) = headers.iter().position(|header| header == alias) {
            return Some(index);
        }
    }
    for alias in aliases {
        if let Some(index) = headers.iter().position(|header| header.contains(alias)) {
            return Some(index);
        }
    }
    None
}

fn cell(row: &[String], index: Option<usize>) -> &str {
    index
        .and_then(|index| row.get(index))
        .map(String::as_str)
        .unwrap_or("")
}

fn parse_number(value: &str) -> Value {
    let cleaned = value
        .trim()
        .trim_end_matches('%')
        .replace(',', "")
        .trim()
        .to_string();
    if cleaned.is_empty() || cleaned == "---" || cleaned == "--" {
        return Value::Null;
    }
    match cleaned.parse::<f64>() {
        Ok(number) => json!(number),
        Err(_) => Value::Null,
    }
}

fn is_empty_row(row: &[String]) -> bool {
    row.iter().all(|cell| cell.trim().is_empty())
}

/// Extract the human-readable section label, e.g. `2024年4季度股票投资明细`.
fn section_label(fragment: &str) -> String {
    let text = crate::fund::strip_html(fragment);
    for token in text.split_whitespace() {
        let digit_count = token.chars().filter(char::is_ascii_digit).count();
        if digit_count >= 4
            && token.contains('年')
            && (token.contains("明细") || token.contains("配置") || token.contains("持仓"))
        {
            return token.to_string();
        }
    }
    // Fall back to dropping the leading fund name when no quarter label exists.
    let mut tokens = text.split_whitespace();
    let _ = tokens.next();
    tokens
        .next()
        .map(str::to_string)
        .unwrap_or_else(|| text.clone())
}

/// Split the `content` fragment into `(label, tables)` sections.
fn content_sections(content: &str) -> Vec<(String, Vec<Vec<Vec<String>>>)> {
    let re = Regex::new(r#"(?is)<h4[^>]*class\s*=\s*['"]t['"][^>]*>(.*?)</h4>"#)
        .expect("static h4 regex is valid");
    let captures: Vec<_> = re.captures_iter(content).collect();
    let mut sections = Vec::new();

    for (index, capture) in captures.iter().enumerate() {
        let Some(whole) = capture.get(0) else {
            continue;
        };
        let label = capture
            .get(1)
            .map(|inner| section_label(inner.as_str()))
            .unwrap_or_default();
        let end = captures
            .get(index + 1)
            .and_then(|next| next.get(0))
            .map_or(content.len(), |next| next.start());
        let segment = &content[whole.end()..end];
        let tables = crate::util::parse_html_tables(segment);
        if !tables.is_empty() {
            sections.push((label, tables));
        }
    }

    if sections.is_empty() {
        let tables = crate::util::parse_html_tables(content);
        if !tables.is_empty() {
            sections.push((String::new(), tables));
        }
    }

    sections
}

struct TableRows<'a> {
    headers: Vec<String>,
    rows: &'a [Vec<String>],
}

fn table_rows(table: &[Vec<String>]) -> Option<TableRows<'_>> {
    let (first, rows) = table.split_first()?;
    let headers: Vec<String> = first
        .iter()
        .map(|header| normalize_header(header))
        .collect();
    if headers.iter().all(String::is_empty) {
        return None;
    }
    Some(TableRows { headers, rows })
}

fn hold_rows(
    content: &str,
    code_aliases: &[&str],
    name_aliases: &[&str],
    shares_aliases: &[&str],
) -> Vec<Value> {
    let mut sections = content_sections(content);
    // Python's `fund_portfolio_hold_em` prepends each quarter, which reverses
    // the newest-first order returned by the API.
    sections.reverse();

    let mut result = Vec::new();
    for (label, tables) in sections {
        for table in tables {
            let Some(parsed) = table_rows(&table) else {
                continue;
            };
            let code_index = header_index(&parsed.headers, code_aliases);
            let name_index = header_index(&parsed.headers, name_aliases);
            let ratio_index = header_index(&parsed.headers, &["占净值比例"]);
            let shares_index = header_index(&parsed.headers, shares_aliases);
            let value_index = header_index(
                &parsed.headers,
                &["持仓市值", "持仓市值（万元）", "持仓市值（万元人民币）"],
            );

            for row in parsed.rows {
                if is_empty_row(row) {
                    continue;
                }
                let code = cell(row, code_index);
                if code.is_empty() && cell(row, name_index).is_empty() {
                    continue;
                }
                let sequence = result.len() as i64 + 1;
                result.push(json!({
                    "序号": sequence,
                    "股票代码": code,
                    "股票名称": cell(row, name_index),
                    "占净值比例": parse_number(cell(row, ratio_index)),
                    "持股数": parse_number(cell(row, shares_index)),
                    "持仓市值": parse_number(cell(row, value_index)),
                    "季度": label,
                }));
            }
        }
    }
    result
}

/// Asset-allocation table headers from `https://fundf10.eastmoney.com/zcpz_{code}.html`.
fn asset_allocation_rows(html: &str) -> Vec<Value> {
    let tables = crate::util::parse_html_tables(html);
    let mut result = Vec::new();
    for table in tables {
        let Some(parsed) = table_rows(&table) else {
            continue;
        };
        let date_index = header_index(&parsed.headers, &["报告期"]);
        let stock_index = header_index(&parsed.headers, &["股票占净比"]);
        let bond_index = header_index(&parsed.headers, &["债券占净比"]);
        let cash_index = header_index(&parsed.headers, &["现金占净比"]);
        let asset_index = header_index(&parsed.headers, &["净资产", "净资产（亿元）"]);
        if date_index.is_none() && stock_index.is_none() && bond_index.is_none() {
            continue;
        }

        for row in parsed.rows {
            if is_empty_row(row) {
                continue;
            }
            result.push(json!({
                "报告期": cell(row, date_index),
                "股票占净比": parse_number(cell(row, stock_index)),
                "债券占净比": parse_number(cell(row, bond_index)),
                "现金占净比": parse_number(cell(row, cash_index)),
                "净资产（亿元）": parse_number(cell(row, asset_index)),
            }));
        }
        if !result.is_empty() {
            break;
        }
    }
    result
}

impl AkShareClient {
    /// Fetch the `content` HTML fragment from `FundArchivesDatas.aspx`.
    async fn fund_archives_content(
        &self,
        symbol: &str,
        data_type: &str,
        referer_page: &str,
        extra_params: &[(&str, &str)],
    ) -> Result<String> {
        let rt = "0.1234567890123456";
        let mut params: Vec<(&str, &str)> = vec![("type", data_type), ("code", symbol), ("rt", rt)];
        params.extend_from_slice(extra_params);

        let referer = format!("https://fundf10.eastmoney.com/{referer_page}_{symbol}.html");
        let text = crate::util::send_and_check_text(
            self.get(ARCHIVES_URL)
                .query(&params)
                .header("Referer", referer)
                .header("X-Requested-With", "XMLHttpRequest"),
            Some(encoding_rs::UTF_8),
        )
        .await?;

        let content = extract_content(&text).ok_or_else(|| {
            Error::decode(format!(
                "FundArchivesDatas response for {symbol} has no content field"
            ))
        })?;
        if content.trim().is_empty() {
            return Err(Error::not_found(format!(
                "no {data_type} data for fund {symbol}"
            )));
        }
        Ok(content)
    }

    /// Fetch fund portfolio holdings from Eastmoney (Python: `fund_portfolio_hold_em`).
    ///
    /// `symbol`: fund code, e.g. "000001".
    /// `date`: report year, e.g. "2024"; an empty string returns the latest
    /// available years.
    pub async fn fund_portfolio_hold(
        &self,
        symbol: &str,
        date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let content = self
            .fund_archives_content(
                symbol,
                "jjcc",
                "ccmx",
                &[("topline", "100"), ("year", date), ("month", "")],
            )
            .await?;

        let rows = hold_rows(
            &content,
            &["股票代码"],
            &["股票名称"],
            &["持股数", "持股数（万股）"],
        );
        if rows.is_empty() {
            return Err(Error::not_found(format!(
                "no portfolio holdings for fund {symbol}"
            )));
        }
        Ok(rows)
    }

    /// Fetch fund portfolio holdings for a specific year (alias of
    /// [`fund_portfolio_hold`](Self::fund_portfolio_hold)).
    pub async fn fund_portfolio_hold_em(
        &self,
        symbol: &str,
        date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.fund_portfolio_hold(symbol, date).await
    }

    /// Fetch fund bond holdings from Eastmoney (Python: `fund_portfolio_bond_hold_em`).
    ///
    /// The legacy signature has no year, so the latest available year is used.
    pub async fn fund_portfolio_bond_hold(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        self.fund_portfolio_bond_hold_em(symbol, "").await
    }

    /// Fetch fund bond holdings for a specific year.
    pub async fn fund_portfolio_bond_hold_em(
        &self,
        symbol: &str,
        date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let content = self
            .fund_archives_content(symbol, "zqcc", "ccmx1", &[("year", date)])
            .await?;

        let mut result = Vec::new();
        for (label, tables) in content_sections(&content) {
            for table in tables {
                let Some(parsed) = table_rows(&table) else {
                    continue;
                };
                let code_index = header_index(&parsed.headers, &["债券代码"]);
                let name_index = header_index(&parsed.headers, &["债券名称"]);
                let ratio_index = header_index(&parsed.headers, &["占净值比例"]);
                let value_index = header_index(
                    &parsed.headers,
                    &["持仓市值", "持仓市值（万元）", "持仓市值（万元人民币）"],
                );

                for row in parsed.rows {
                    if is_empty_row(row) {
                        continue;
                    }
                    let code = cell(row, code_index);
                    if code.is_empty() && cell(row, name_index).is_empty() {
                        continue;
                    }
                    let sequence = result.len() as i64 + 1;
                    result.push(json!({
                        "序号": sequence,
                        "债券代码": code,
                        "债券名称": cell(row, name_index),
                        "占净值比例": parse_number(cell(row, ratio_index)),
                        "持仓市值": parse_number(cell(row, value_index)),
                        "季度": label,
                    }));
                }
            }
        }
        if result.is_empty() {
            return Err(Error::not_found(format!(
                "no portfolio bond holdings for fund {symbol}"
            )));
        }
        Ok(result)
    }

    /// Fetch fund asset allocation from Eastmoney.
    ///
    /// Eastmoney publishes this as a static table on
    /// `https://fundf10.eastmoney.com/zcpz_{symbol}.html`.  Some deployments
    /// also expose it through `FundArchivesDatas.aspx?type=zcpz`, so that is
    /// tried first and the static page is used as a fallback.
    pub async fn fund_portfolio_asset_allocation(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        if let Ok(content) = self
            .fund_archives_content(symbol, "zcpz", "zcpz", &[])
            .await
        {
            let rows = asset_allocation_rows(&content);
            if !rows.is_empty() {
                return Ok(rows);
            }
        }

        let url = format!("https://fundf10.eastmoney.com/zcpz_{symbol}.html");
        let html = crate::util::send_and_check_text(
            self.get(&url)
                .header("Referer", "https://fundf10.eastmoney.com/"),
            Some(encoding_rs::UTF_8),
        )
        .await?;

        let rows = asset_allocation_rows(&html);
        if rows.is_empty() {
            return Err(Error::not_found(format!(
                "no asset allocation data for fund {symbol}"
            )));
        }
        Ok(rows)
    }

    /// Fetch fund asset allocation (alias of
    /// [`fund_portfolio_asset_allocation`](Self::fund_portfolio_asset_allocation)).
    pub async fn fund_portfolio_asset_allocation_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.fund_portfolio_asset_allocation(symbol).await
    }

    /// Fetch fund portfolio industry allocation from Eastmoney.
    pub async fn fund_portfolio_industry_allocation(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let response = crate::util::send_and_check_text(
            self.get("https://api.fund.eastmoney.com/f10/HYPZ/")
                .query(&[("fundCode", symbol), ("year", "")])
                .header("Referer", "https://fundf10.eastmoney.com/")
                .header("User-Agent", "Mozilla/5.0"),
            Some(encoding_rs::UTF_8),
        )
        .await?;
        let start = response
            .find('{')
            .ok_or_else(|| Error::decode("industry allocation JSON missing"))?;
        let end = response
            .rfind('}')
            .map(|index| index + 1)
            .unwrap_or(response.len());
        let root: Value = serde_json::from_str(&response[start..end])
            .map_err(|err| Error::decode(format!("industry allocation JSON: {err}")))?;
        let quarters = root["Data"]["QuarterInfos"]
            .as_array()
            .ok_or_else(|| Error::not_found(format!("no industry allocation for fund {symbol}")))?;
        let mut rows = Vec::new();
        for quarter in quarters {
            if let Some(items) = quarter["HYPZInfo"].as_array() {
                rows.extend(items.iter().cloned());
            }
        }
        if rows.is_empty() {
            return Err(Error::not_found(format!(
                "no industry allocation for fund {symbol}"
            )));
        }
        Ok(rows)
    }

    ///
    /// `symbol`: fund code.
    /// `indicator`: "累计买入" or "累计卖出".
    pub async fn fund_portfolio_change(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.fund_portfolio_change_em(symbol, indicator, "").await
    }

    /// Fetch fund portfolio major changes for a specific year.
    pub async fn fund_portfolio_change_em(
        &self,
        symbol: &str,
        indicator: &str,
        date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let zdbd = match indicator {
            "累计买入" => "1",
            "累计卖出" => "2",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported indicator: {indicator}; expected 累计买入 or 累计卖出"
                )));
            }
        };

        let content = self
            .fund_archives_content(symbol, "zdbd", "ccbd", &[("zdbd", zdbd), ("year", date)])
            .await?;

        let mut result = Vec::new();
        for (label, tables) in content_sections(&content) {
            for table in tables {
                let Some(parsed) = table_rows(&table) else {
                    continue;
                };
                let code_index = header_index(&parsed.headers, &["股票代码"]);
                let name_index = header_index(&parsed.headers, &["股票名称"]);
                let amount_index = header_index(
                    &parsed.headers,
                    &["本期累计买入金额", "本期累计买入金额（万元）"],
                );
                let ratio_index = header_index(
                    &parsed.headers,
                    &["占期初基金资产净值比例", "占期初基金资产净值比例（%）"],
                );

                for row in parsed.rows {
                    if is_empty_row(row) {
                        continue;
                    }
                    let code = cell(row, code_index);
                    if code.is_empty() && cell(row, name_index).is_empty() {
                        continue;
                    }
                    let ratio = parse_number(cell(row, ratio_index));
                    let sequence = result.len() as i64 + 1;
                    result.push(json!({
                        "序号": sequence,
                        "股票代码": code,
                        "股票名称": cell(row, name_index),
                        "本期累计买入金额": parse_number(cell(row, amount_index)),
                        "占期初基金资产净值比例": ratio,
                        "季度": label,
                    }));
                }
            }
        }
        Ok(result)
    }
}
