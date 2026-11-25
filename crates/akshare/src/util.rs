// Shared helpers are intentionally broader than any single feature set (HTML
// tables, CSV, XLS/XLSX). Callers live behind different `mod-*` features, so a
// helper can be unused in a given build without being dead.
#![allow(dead_code)]

use std::io::Cursor;

use calamine::{Data, Reader, Xls, Xlsx};

async fn send_checked(builder: reqwest::RequestBuilder) -> crate::Result<reqwest::Response> {
    builder
        .send()
        .await
        .map_err(crate::Error::from)?
        .error_for_status()
        .map_err(crate::Error::from)
}

/// Read the first worksheet from an XLS/XLSX response as string rows.
pub(crate) fn read_spreadsheet_rows(bytes: &[u8]) -> crate::Result<Vec<Vec<String>>> {
    if bytes.starts_with(b"PK") {
        let mut workbook: Xlsx<Cursor<Vec<u8>>> = Xlsx::new(Cursor::new(bytes.to_vec()))
            .map_err(|e| crate::Error::decode(format!("failed to open XLSX: {e}")))?;
        let sheet = workbook
            .sheet_names()
            .first()
            .cloned()
            .ok_or_else(|| crate::Error::decode("XLSX has no worksheets"))?;
        let range = workbook
            .worksheet_range(&sheet)
            .map_err(|e| crate::Error::decode(format!("failed to read XLSX worksheet: {e}")))?;
        Ok(range
            .rows()
            .map(|row| row.iter().map(ToString::to_string).collect())
            .collect())
    } else {
        let mut workbook: Xls<Cursor<Vec<u8>>> = Xls::new(Cursor::new(bytes.to_vec()))
            .map_err(|e| crate::Error::decode(format!("failed to open XLS: {e}")))?;
        let sheet = workbook
            .sheet_names()
            .first()
            .cloned()
            .ok_or_else(|| crate::Error::decode("XLS has no worksheets"))?;
        let range = workbook
            .worksheet_range(&sheet)
            .map_err(|e| crate::Error::decode(format!("failed to read XLS worksheet: {e}")))?;
        Ok(range
            .rows()
            .map(|row| row.iter().map(ToString::to_string).collect())
            .collect())
    }
}

/// Normalize a security code while preserving six digits.
pub(crate) fn zero_pad_code(value: &str) -> String {
    let code = value.trim().split('.').next().unwrap_or_default().trim();
    if code.is_empty() || code == "nan" {
        String::new()
    } else {
        format!("{code:0>6}")
    }
}

///
/// Many Chinese market-data endpoints still return GBK/GB18030 while omitting a
/// charset declaration. Valid UTF-8 always wins; otherwise GB18030 is used so
/// callers never receive replacement characters from `from_utf8_lossy`.
pub(crate) async fn response_text(
    response: reqwest::Response,
    fallback: Option<&'static encoding_rs::Encoding>,
) -> crate::Result<String> {
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let bytes = response.bytes().await.map_err(crate::Error::from)?;
    Ok(decode_text(&bytes, content_type.as_deref(), fallback))
}

/// Send a request, reject non-success HTTP statuses, and decode its body.
pub(crate) async fn send_and_check_text(
    builder: reqwest::RequestBuilder,
    fallback: Option<&'static encoding_rs::Encoding>,
) -> crate::Result<String> {
    let response = send_checked(builder).await?;
    response_text(response, fallback).await
}

/// Send a request, reject non-success HTTP statuses, and parse JSON.
pub(crate) async fn send_and_check_json<T: serde::de::DeserializeOwned>(
    builder: reqwest::RequestBuilder,
) -> crate::Result<T> {
    let response = send_checked(builder).await?;
    let bytes = response.bytes().await.map_err(crate::Error::from)?;
    match serde_json::from_slice(&bytes) {
        Ok(value) => Ok(value),
        Err(utf8_or_json_error) => {
            let text = decode_text(&bytes, None, Some(encoding_rs::GB18030));
            serde_json::from_str(&text).map_err(|_| crate::Error::from(utf8_or_json_error))
        }
    }
}

/// Decode text with an explicit HTTP charset, UTF-8 detection, and fallback.
pub(crate) fn decode_text(
    bytes: &[u8],
    content_type: Option<&str>,
    fallback: Option<&'static encoding_rs::Encoding>,
) -> String {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return String::from_utf8_lossy(&bytes[3..]).into_owned();
    }
    if let Some(label) = content_type.and_then(content_type_charset)
        && let Some(encoding) = encoding_rs::Encoding::for_label(label.as_bytes())
    {
        return encoding.decode(bytes).0.into_owned();
    }
    if let Ok(text) = std::str::from_utf8(bytes) {
        return text.to_owned();
    }
    fallback
        .unwrap_or(encoding_rs::GB18030)
        .decode(bytes)
        .0
        .into_owned()
}

fn content_type_charset(content_type: &str) -> Option<&str> {
    content_type.split(';').skip(1).find_map(|part| {
        let (key, value) = part.trim().split_once('=')?;
        (key.trim().eq_ignore_ascii_case("charset"))
            .then(|| value.trim().trim_matches(|c| c == '"' || c == '\''))
    })
}

/// Format a date string from "YYYYMMDD" to "YYYY-MM-DD".
pub fn fmt_date(date: &str) -> String {
    if date.len() >= 8 && date.is_ascii() {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        date.to_string()
    }
}

/// Normalize a trade date string to "YYYY-MM-DD" format.
pub fn normalize_trade_date(value: &str) -> String {
    if value.len() >= 8 {
        fmt_date(value)
    } else {
        value.get(0..10).unwrap_or(value).to_string()
    }
}

/// Parse a string to f64, returning 0.0 on failure.
pub fn parse_f64_safe(value: &str) -> f64 {
    value.trim().parse::<f64>().unwrap_or(0.0)
}

/// Parse a string to i64, returning 0 on failure.
pub fn parse_i64_safe(value: &str) -> i64 {
    value.trim().parse::<i64>().unwrap_or(0)
}

/// Get today's date in "YYYYMMDD" format.
pub fn today_yyyymmdd() -> String {
    chrono::Utc::now().format("%Y%m%d").to_string()
}

/// Get today's date in "YYYY-MM-DD" format.
pub fn today_iso() -> String {
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}

/// Get the date N days ago in "YYYYMMDD" format.
pub fn days_ago_yyyymmdd(days: i64) -> String {
    (chrono::Utc::now() - chrono::Duration::days(days))
        .format("%Y%m%d")
        .to_string()
}

/// Split a CSV line by comma and trim whitespace from each field.
pub fn parse_csv_line(line: &str) -> Vec<&str> {
    line.split(',').map(str::trim).collect()
}

/// Parse an Eastmoney candle CSV line into a `CandlePoint`.
///
/// Format: `date,open,close,high,low,volume,amount,amplitude_pct,change_pct,change_amount,turnover_pct`
pub fn parse_candle_line(line: &str) -> crate::Result<crate::types::CandlePoint> {
    let f = parse_csv_line(line);
    if f.len() < 11 {
        return Err(crate::Error::decode(format!(
            "unexpected eastmoney candle format: {line}"
        )));
    }
    Ok(crate::types::CandlePoint {
        trade_date: f[0].to_string(),
        open: parse_f64_safe(f[1]),
        close: parse_f64_safe(f[2]),
        high: parse_f64_safe(f[3]),
        low: parse_f64_safe(f[4]),
        volume: parse_f64_safe(f[5]).round() as i64,
        amount: parse_f64_safe(f[6]),
        amplitude_pct: parse_f64_safe(f[7]),
        change_pct: parse_f64_safe(f[8]),
        change_amount: parse_f64_safe(f[9]),
        turnover_pct: parse_f64_safe(f[10]),
    })
}

/// Compute change_amount and change_pct from consecutive candle closes.
pub fn apply_change_metrics(items: &mut [crate::types::CandlePoint]) {
    for index in 1..items.len() {
        let prev_close = items[index - 1].close;
        if prev_close > 0.0 {
            items[index].change_amount = items[index].close - prev_close;
            items[index].change_pct = (items[index].change_amount / prev_close) * 100.0;
        }
    }
}

/// Sort candle items by `trade_date` and keep only the last `limit` items.
///
/// This is a common post-processing step after fetching kline data from
/// providers that may return more rows than requested or in arbitrary order.
pub fn sort_and_limit(
    mut items: Vec<crate::types::CandlePoint>,
    limit: usize,
) -> Vec<crate::types::CandlePoint> {
    items.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
    if items.len() > limit {
        items[items.len() - limit..].to_vec()
    } else {
        items
    }
}

/// Calculate amplitude percentage: (high - low) / low * 100.
pub fn amplitude_pct(high: f64, low: f64) -> f64 {
    if low > 0.0 {
        ((high - low) / low) * 100.0
    } else {
        0.0
    }
}

/// Send a request builder and check for HTTP errors.
///
/// This replaces the common pattern:
/// ```ignore
/// .send().await.map_err(Error::from)?
///     .error_for_status().map_err(Error::from)?
/// ```
pub(crate) async fn send_and_check(
    builder: reqwest::RequestBuilder,
) -> crate::Result<reqwest::Response> {
    send_checked(builder).await
}

/// Build Eastmoney clist query parameters.
///
/// Returns the common parameter block used by `push2.eastmoney.com/api/qt/clist/get`.
/// Callers pass `pz` (page size) and any extra fields specific to their endpoint.
pub(crate) fn eastmoney_clist_params<'a>(
    pz: &'a str,
    extra: &[(&'a str, &'a str)],
) -> Vec<(&'a str, &'a str)> {
    let mut params = vec![
        ("pn", "1"),
        ("pz", pz),
        ("po", "1"),
        ("np", "1"),
        ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
        ("fltt", "2"),
        ("invt", "2"),
    ];
    params.extend_from_slice(extra);
    params
}

/// Build Eastmoney datacenter query parameters.
///
/// Returns the common parameter block used by
/// `datacenter-web.eastmoney.com/api/data/v1/get`.
/// Callers pass `sort_columns` and any extra fields specific to their endpoint.
pub fn eastmoney_datacenter_params<'a>(
    sort_columns: &'a str,
    extra: &[(&'a str, &'a str)],
) -> Vec<(&'a str, &'a str)> {
    let mut params = vec![
        ("columns", "ALL"),
        ("pageNumber", "1"),
        ("pageSize", "500"),
        ("sortTypes", "-1"),
        ("sortColumns", sort_columns),
        ("source", "WEB"),
        ("client", "WEB"),
    ];
    params.extend_from_slice(extra);
    params
}

/// Build Eastmoney F10/HSF10 query parameters.
pub(crate) fn eastmoney_f10_params(source: &str) -> Vec<(&str, &str)> {
    vec![("source", source), ("client", "PC")]
}

/// Extract lines between `<table>` and `</table>` tags from HTML.
///
/// Returns trimmed lines that appear between the first `<table` and the
/// matching `</table>` tag. Outer markup and content outside the table are
/// discarded.
pub fn extract_table_lines(html: &str) -> Vec<String> {
    let mut in_table = false;
    let mut lines = Vec::new();
    for line in html.lines() {
        let trimmed = line.trim();
        if trimmed.contains("<table") {
            in_table = true;
            continue;
        }
        if trimmed.contains("</table>") {
            in_table = false;
            continue;
        }
        if in_table {
            lines.push(trimmed.to_string());
        }
    }
    lines
}

/// Extract text content from `<td>...</td>` cells in an HTML fragment.
///
/// Returns a `Vec<String>` of trimmed cell contents found in `html`.
pub fn extract_table_cells(html: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut remaining = html;
    while let Some(start) = remaining.find("<td") {
        let after_td = &remaining[start..];
        if let Some(content_start) = after_td.find('>') {
            let content = &after_td[content_start + 1..];
            if let Some(content_end) = content.find("</td>") {
                let cell_text = content[..content_end].trim().to_string();
                cells.push(cell_text);
                remaining = &content[content_end + 5..];
            } else {
                break;
            }
        } else {
            break;
        }
    }
    cells
}

// ---------------------------------------------------------------------------
// Shared HTML table parser
// ---------------------------------------------------------------------------

/// One raw cell collected from a `<tr>` before column positions are resolved.
struct RawCell {
    text: String,
    colspan: usize,
    rowspan: usize,
}

/// A `rowspan` cell that must be re-inserted into following rows.
struct PendingCell {
    col: usize,
    remaining: usize,
    text: String,
}

/// Parse every `<table>` element in an HTML document.
///
/// Returns `tables → rows → cells` in document order:
/// - the outer `Vec` contains one entry per `<table>`,
/// - each table is a `Vec` of rows (one per `<tr>`),
/// - each row is a `Vec` of cell strings (`<th>` and `<td>`).
///
/// Nested markup is stripped, common HTML entities are decoded, `<br>` becomes
/// a space, `colspan` duplicates the text across the spanned columns and
/// `rowspan` cells are carried into following rows so columns stay aligned.
/// Script/style bodies are ignored.
///
/// Unlike [`extract_table_cells`], this parser does not assume one cell per
/// line and therefore works with minified HTML.
pub(crate) fn parse_html_tables(html: &str) -> Vec<Vec<Vec<String>>> {
    let mut tables: Vec<Vec<Vec<String>>> = Vec::new();
    let mut current_table: Option<Vec<Vec<String>>> = None;
    let mut depth = 0_usize;
    let mut row: Option<Vec<RawCell>> = None;
    let mut cell: Option<String> = None;
    let mut cell_colspan = 1_usize;
    let mut cell_rowspan = 1_usize;
    let mut carry: Vec<PendingCell> = Vec::new();
    let mut skip_tag: Option<String> = None;

    let mut pos = 0;
    while pos < html.len() {
        let Some(rel) = html[pos..].find('<') else {
            if let Some(text) = cell.as_mut() {
                text.push_str(&decode_html_entities(&html[pos..]));
            }
            break;
        };

        let text = &html[pos..pos + rel];
        if !text.is_empty()
            && skip_tag.is_none()
            && let Some(cell_text) = cell.as_mut()
        {
            cell_text.push_str(&decode_html_entities(text));
        }

        let tag_start = pos + rel;
        if html[tag_start..].starts_with("<!--") {
            pos = html[tag_start + 4..]
                .find("-->")
                .map_or(html.len(), |i| tag_start + 4 + i + 3);
            continue;
        }

        let Some(tag_rel_end) = html[tag_start..].find('>') else {
            break;
        };
        let raw_tag = &html[tag_start + 1..tag_start + tag_rel_end];
        pos = tag_start + tag_rel_end + 1;

        let raw_tag = raw_tag.trim();
        if raw_tag.is_empty() || raw_tag.starts_with('!') || raw_tag.starts_with('?') {
            continue;
        }

        let is_end = raw_tag.starts_with('/');
        let body = if is_end { &raw_tag[1..] } else { raw_tag };
        let body = body.trim_end_matches('/').trim();
        let name_end = body
            .find(|c: char| c.is_whitespace() || c == '=')
            .unwrap_or(body.len());
        let name = body[..name_end].to_ascii_lowercase();
        let attrs = &body[name_end..];

        if let Some(tag) = &skip_tag {
            if is_end && name == *tag {
                skip_tag = None;
            }
            continue;
        }

        match (is_end, name.as_str()) {
            (false, "script" | "style") => skip_tag = Some(name),
            (false, "table") => {
                if depth == 0 {
                    current_table = Some(Vec::new());
                    carry.clear();
                }
                depth += 1;
            }
            (true, "table") => {
                if depth == 1 {
                    take_cell(&mut row, &mut cell, &mut cell_colspan, &mut cell_rowspan);
                    take_row(&mut current_table, &mut row, &mut carry);
                    if let Some(table) = current_table.take() {
                        tables.push(table);
                    }
                }
                depth = depth.saturating_sub(1);
            }
            (false, "tr") if depth == 1 => {
                take_cell(&mut row, &mut cell, &mut cell_colspan, &mut cell_rowspan);
                take_row(&mut current_table, &mut row, &mut carry);
                row = Some(Vec::new());
            }
            (true, "tr") if depth == 1 => {
                take_cell(&mut row, &mut cell, &mut cell_colspan, &mut cell_rowspan);
                take_row(&mut current_table, &mut row, &mut carry);
            }
            (false, "td" | "th") if depth == 1 => {
                take_cell(&mut row, &mut cell, &mut cell_colspan, &mut cell_rowspan);
                cell = Some(String::new());
                cell_colspan = attr_value(attrs, "colspan")
                    .and_then(|v| v.trim().parse::<usize>().ok())
                    .unwrap_or(1)
                    .clamp(1, 1_000);
                cell_rowspan = attr_value(attrs, "rowspan")
                    .and_then(|v| v.trim().parse::<usize>().ok())
                    .unwrap_or(1)
                    .clamp(1, 1_000);
            }
            (true, "td" | "th") if depth == 1 => {
                take_cell(&mut row, &mut cell, &mut cell_colspan, &mut cell_rowspan);
            }
            (false, "br") => {
                if let Some(cell_text) = cell.as_mut() {
                    cell_text.push(' ');
                }
            }
            _ => {}
        }
    }

    // Be forgiving about documents that omit the closing `</table>`.
    if depth > 0 {
        take_cell(&mut row, &mut cell, &mut cell_colspan, &mut cell_rowspan);
        take_row(&mut current_table, &mut row, &mut carry);
        if let Some(table) = current_table.take() {
            tables.push(table);
        }
    }

    tables
}

/// Parse the first HTML table, returning its rows of cells.
///
/// Convenience wrapper around [`parse_html_tables`] for pages with a single
/// relevant table.
pub(crate) fn parse_first_html_table(html: &str) -> Vec<Vec<String>> {
    parse_html_tables(html)
        .into_iter()
        .next()
        .unwrap_or_default()
}

fn take_cell(
    row: &mut Option<Vec<RawCell>>,
    cell: &mut Option<String>,
    colspan: &mut usize,
    rowspan: &mut usize,
) {
    if let Some(text) = cell.take()
        && let Some(cells) = row.as_mut()
    {
        cells.push(RawCell {
            text: normalize_cell_text(&text),
            colspan: (*colspan).max(1),
            rowspan: (*rowspan).max(1),
        });
    }
    *colspan = 1;
    *rowspan = 1;
}

fn take_row(
    table: &mut Option<Vec<Vec<String>>>,
    row: &mut Option<Vec<RawCell>>,
    carry: &mut Vec<PendingCell>,
) {
    if let Some(cells) = row.take()
        && let Some(rows) = table.as_mut()
    {
        rows.push(finalize_row(cells, carry));
    }
}

/// Place rowspan carry-over cells and this row's cells into column slots.
fn finalize_row(cells: Vec<RawCell>, carry: &mut Vec<PendingCell>) -> Vec<String> {
    let mut placed: std::collections::BTreeMap<usize, String> = std::collections::BTreeMap::new();

    for pending in carry.iter_mut() {
        if pending.remaining > 0 {
            placed.insert(pending.col, pending.text.clone());
            pending.remaining -= 1;
        }
    }
    carry.retain(|pending| pending.remaining > 0);

    let mut next_pending = Vec::new();
    let mut col = 0;
    for cell in cells {
        let colspan = cell.colspan.clamp(1, 1_000);
        let rowspan = cell.rowspan.clamp(1, 1_000);
        for _ in 0..colspan {
            while placed.contains_key(&col) {
                col += 1;
            }
            placed.insert(col, cell.text.clone());
            if rowspan > 1 {
                next_pending.push(PendingCell {
                    col,
                    remaining: rowspan - 1,
                    text: cell.text.clone(),
                });
            }
            col += 1;
        }
    }
    carry.extend(next_pending);

    let width = placed.keys().next_back().map_or(0, |col| col + 1);
    let mut out = vec![String::new(); width];
    for (col, text) in placed {
        if let Some(slot) = out.get_mut(col) {
            *slot = text;
        }
    }
    out
}

fn normalize_cell_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Decode the HTML entities commonly found in market-data tables.
fn decode_html_entities(text: &str) -> String {
    if !text.contains('&') {
        return text.to_string();
    }

    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        let tail = &rest[amp..];
        let window_end = tail.len().min(12);
        let Some(semi) = tail[..window_end].find(';') else {
            out.push('&');
            rest = &tail[1..];
            continue;
        };
        let entity = &tail[1..semi];
        let decoded = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some(' '),
            _ if entity.starts_with('#') => {
                let digits = &entity[1..];
                let code = if let Some(hex) = digits
                    .strip_prefix('x')
                    .or_else(|| digits.strip_prefix('X'))
                {
                    u32::from_str_radix(hex, 16).ok()
                } else {
                    digits.parse::<u32>().ok()
                };
                code.and_then(char::from_u32)
            }
            _ => None,
        };
        if let Some(ch) = decoded {
            out.push(ch);
            rest = &tail[semi + 1..];
        } else {
            out.push('&');
            rest = &tail[1..];
        }
    }
    out.push_str(rest);
    out
}

/// Fetch a named attribute value from a raw tag attribute string.
fn attr_value<'a>(attrs: &'a str, key: &str) -> Option<&'a str> {
    let mut rest = attrs;
    loop {
        let trimmed = rest.trim_start();
        if trimmed.is_empty() {
            return None;
        }
        let name_end = trimmed
            .find(|c: char| c.is_whitespace() || c == '=')
            .unwrap_or(trimmed.len());
        let name = &trimmed[..name_end];
        let after_name = trimmed[name_end..].trim_start();
        if let Some(value_part) = after_name.strip_prefix('=') {
            let value_part = value_part.trim_start();
            let (value, next) = if let Some(quote) = value_part
                .chars()
                .next()
                .filter(|c| *c == '"' || *c == '\'')
            {
                let inner = &value_part[1..];
                match inner.find(quote) {
                    Some(end) => (&inner[..end], &inner[end + 1..]),
                    None => (inner, ""),
                }
            } else {
                let end = value_part
                    .find(char::is_whitespace)
                    .unwrap_or(value_part.len());
                (&value_part[..end], &value_part[end..])
            };
            if name.eq_ignore_ascii_case(key) {
                return Some(value);
            }
            rest = next;
        } else {
            if name.eq_ignore_ascii_case(key) {
                return Some("");
            }
            rest = after_name;
        }
    }
}

// ---------------------------------------------------------------------------
// CSV helpers
// ---------------------------------------------------------------------------

/// Parse CSV/TSV-like text into records.
///
/// Supports quoted fields with embedded delimiters, doubled quotes (`""`),
/// CRLF/LF line endings and blank trailing lines. Fields are trimmed.
pub(crate) fn parse_csv_records(text: &str, delimiter: char) -> Vec<Vec<String>> {
    let mut records = Vec::new();
    let mut record = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(ch);
            }
            continue;
        }

        match ch {
            '"' if field.is_empty() => in_quotes = true,
            c if c == delimiter => {
                record.push(field.trim().to_string());
                field.clear();
            }
            '\r' | '\n' => {
                if ch == '\r' && chars.peek() == Some(&'\n') {
                    chars.next();
                }
                record.push(field.trim().to_string());
                field.clear();
                if !(record.len() == 1 && record[0].is_empty()) {
                    records.push(std::mem::take(&mut record));
                } else {
                    record.clear();
                }
            }
            c => field.push(c),
        }
    }

    if !field.is_empty() || !record.is_empty() {
        record.push(field.trim().to_string());
        records.push(record);
    }
    records
}

// ---------------------------------------------------------------------------
// XLS / XLSX helpers
// ---------------------------------------------------------------------------

/// True when the payload starts with a ZIP local-file header (XLSX/XLSB or a
/// zipped archive). `calamine` opens ZIP-based XLSX directly.
pub(crate) fn is_zip_container(bytes: &[u8]) -> bool {
    bytes.starts_with(b"PK\x03\x04")
}

/// Names of all worksheets in an XLS/XLSX workbook, in workbook order.
pub(crate) fn spreadsheet_sheet_names(bytes: &[u8]) -> crate::Result<Vec<String>> {
    if bytes.starts_with(b"PK") {
        let workbook: Xlsx<Cursor<Vec<u8>>> = Xlsx::new(Cursor::new(bytes.to_vec()))
            .map_err(|e| crate::Error::decode(format!("failed to open XLSX workbook: {e}")))?;
        Ok(workbook.sheet_names().to_vec())
    } else {
        let workbook: Xls<Cursor<Vec<u8>>> = Xls::new(Cursor::new(bytes.to_vec()))
            .map_err(|e| crate::Error::decode(format!("failed to open XLS workbook: {e}")))?;
        Ok(workbook.sheet_names().to_vec())
    }
}

/// Read one worksheet (default: the first) as string rows.
///
/// Date and time cells are rendered as `YYYY-MM-DD` or
/// `YYYY-MM-DD HH:MM:SS`, which makes XLSX exports from NIFD/CNIndex usable
/// without a Python/Excel runtime. Booking dates that are stored as plain
/// numbers are left untouched.
pub(crate) fn read_spreadsheet_sheet_rows(
    bytes: &[u8],
    sheet: Option<&str>,
) -> crate::Result<Vec<Vec<String>>> {
    if bytes.starts_with(b"PK") {
        let mut workbook: Xlsx<Cursor<Vec<u8>>> = Xlsx::new(Cursor::new(bytes.to_vec()))
            .map_err(|e| crate::Error::decode(format!("failed to open XLSX workbook: {e}")))?;
        read_worksheet(&mut workbook, sheet)
    } else {
        let mut workbook: Xls<Cursor<Vec<u8>>> = Xls::new(Cursor::new(bytes.to_vec()))
            .map_err(|e| crate::Error::decode(format!("failed to open XLS workbook: {e}")))?;
        read_worksheet(&mut workbook, sheet)
    }
}

fn read_worksheet<R>(workbook: &mut R, sheet: Option<&str>) -> crate::Result<Vec<Vec<String>>>
where
    R: Reader<Cursor<Vec<u8>>>,
    R::Error: std::fmt::Display,
{
    let names = workbook.sheet_names().to_vec();
    let selected = match sheet {
        Some(wanted) => names
            .iter()
            .find(|name| name.trim().eq_ignore_ascii_case(wanted.trim()))
            .or_else(|| names.iter().find(|name| name.trim() == wanted.trim()))
            .cloned()
            .ok_or_else(|| {
                crate::Error::decode(format!(
                    "worksheet `{wanted}` not found (available: {})",
                    names.join(", ")
                ))
            })?,
        None => names
            .first()
            .cloned()
            .ok_or_else(|| crate::Error::decode("workbook has no worksheets"))?,
    };

    let range = workbook
        .worksheet_range(&selected)
        .map_err(|e| crate::Error::decode(format!("failed to read worksheet `{selected}`: {e}")))?;
    Ok(range
        .rows()
        .map(|row| row.iter().map(spreadsheet_cell_text).collect())
        .collect())
}

fn spreadsheet_cell_text(cell: &Data) -> String {
    match cell {
        Data::Empty => String::new(),
        Data::DateTime(dt) if dt.is_datetime() => excel_serial_to_iso(dt.as_f64()),
        Data::DateTimeIso(value) | Data::DurationIso(value) => value.clone(),
        other => other.to_string(),
    }
}

/// Convert an Excel 1900-date-system serial number to ISO text.
///
/// The 1900 leap-year bug is absorbed by using 1899-12-30 as the epoch, which
/// is the same convention used by Excel-compatible readers for modern dates.
pub(crate) fn excel_serial_to_iso(serial: f64) -> String {
    if !serial.is_finite() {
        return serial.to_string();
    }
    let Some(base) = chrono::NaiveDate::from_ymd_opt(1899, 12, 30) else {
        return serial.to_string();
    };
    let whole_days = serial.trunc();
    let Some(date) = base.checked_add_signed(chrono::Duration::days(whole_days as i64)) else {
        return serial.to_string();
    };
    let seconds = ((serial - whole_days) * 86_400.0).round() as i64;
    if seconds == 0 {
        return date.format("%Y-%m-%d").to_string();
    }
    let Some(midnight) = date.and_hms_opt(0, 0, 0) else {
        return date.format("%Y-%m-%d").to_string();
    };
    (midnight + chrono::Duration::seconds(seconds))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_gbk_without_charset() {
        let (encoded, _, _) = encoding_rs::GBK.encode("浦发银行");
        let decoded = decode_text(&encoded, None, Some(encoding_rs::GB18030));
        assert_eq!(decoded, "浦发银行");
    }

    #[test]
    fn decode_declared_gbk() {
        let (encoded, _, _) = encoding_rs::GBK.encode("小计");
        let decoded = decode_text(&encoded, Some("text/plain; charset=GBK"), None);
        assert_eq!(decoded, "小计");
    }

    #[test]
    fn decode_utf8_without_charset() {
        assert_eq!(decode_text("贵州茅台".as_bytes(), None, None), "贵州茅台");
    }

    #[test]
    fn non_ascii_date_does_not_panic() {
        assert_eq!(fmt_date("2024年01月02日"), "2024年01月02日");
    }

    #[test]
    fn parse_html_tables_handles_minified_input() {
        let html = "<html><body><table class=x><tr><th>A</th><th>B</th></tr>\
                    <tr><td>1</td><td>2</td></tr></table></body></html>";
        let tables = parse_html_tables(html);
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0][0], vec!["A", "B"]);
        assert_eq!(tables[0][1], vec!["1", "2"]);
    }

    #[test]
    fn parse_html_tables_multiple_tables_and_nested_markup() {
        let html = "<table><tr><td><b>name</b>&nbsp;x</td><td>1</td></tr></table>\
                    <script>var s = '<table><tr><td>ignored</td></tr></table>';</script>\
                    <table><tr><td>second</td></tr></table>";
        let tables = parse_html_tables(html);
        assert_eq!(tables.len(), 2);
        assert_eq!(tables[0][0], vec!["name x", "1"]);
        assert_eq!(tables[1][0], vec!["second"]);
    }

    #[test]
    fn parse_html_tables_colspan_and_rowspan_align_columns() {
        let html = "<table>\
            <tr><th colspan=2>head</th><th>c</th></tr>\
            <tr><td rowspan=2>a</td><td>b</td><td>c1</td></tr>\
            <tr><td>d</td><td>c2</td></tr>\
            </table>";
        let tables = parse_html_tables(html);
        assert_eq!(tables[0][0], vec!["head", "head", "c"]);
        assert_eq!(tables[0][1], vec!["a", "b", "c1"]);
        assert_eq!(tables[0][2], vec!["a", "d", "c2"]);
    }

    #[test]
    fn parse_html_tables_decodes_numeric_entities() {
        let html = "<table><tr><td>&#26085;&#26399;</td><td>&amp;</td></tr></table>";
        let tables = parse_html_tables(html);
        assert_eq!(tables[0][0], vec!["日期", "&"]);
    }

    #[test]
    fn parse_first_html_table_returns_empty_without_table() {
        assert!(parse_first_html_table("<div>no table</div>").is_empty());
    }

    #[test]
    fn parse_csv_records_handles_quotes_and_crlf() {
        let text = "a,\"b,c\",\"d\"\"e\"\r\n1,2,3\r\n";
        let records = parse_csv_records(text, ',');
        assert_eq!(
            records,
            vec![
                vec!["a".to_string(), "b,c".to_string(), "d\"e".to_string()],
                vec!["1".to_string(), "2".to_string(), "3".to_string()],
            ]
        );
    }

    #[test]
    fn excel_serial_to_iso_converts_1900_system_dates() {
        assert_eq!(excel_serial_to_iso(38_442.0), "2005-03-31");
        assert_eq!(excel_serial_to_iso(45_000.0), "2023-03-15");
        assert_eq!(excel_serial_to_iso(38_442.5), "2005-03-31 12:00:00");
    }

    #[test]
    fn zip_magic_detection() {
        assert!(is_zip_container(b"PK\x03\x04rest"));
        assert!(!is_zip_container(b"\xD0\xCF\x11\xE0"));
    }
}
