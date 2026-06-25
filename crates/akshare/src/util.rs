/// Format a date string from "YYYYMMDD" to "YYYY-MM-DD".
pub fn fmt_date(date: &str) -> String {
    if date.len() >= 8 {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        date.to_string()
    }
}

pub fn normalize_trade_date(value: &str) -> String {
    if value.len() >= 8 {
        fmt_date(value)
    } else {
        value.get(0..10).unwrap_or(value).to_string()
    }
}

pub fn parse_f64_safe(value: &str) -> f64 {
    value.trim().parse::<f64>().unwrap_or(0.0)
}

pub fn parse_i64_safe(value: &str) -> i64 {
    value.trim().parse::<i64>().unwrap_or(0)
}

pub fn today_yyyymmdd() -> String {
    chrono::Utc::now().format("%Y%m%d").to_string()
}

pub fn today_iso() -> String {
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}

pub fn days_ago_yyyymmdd(days: i64) -> String {
    (chrono::Utc::now() - chrono::Duration::days(days))
        .format("%Y%m%d")
        .to_string()
}

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
    builder
        .send()
        .await
        .map_err(crate::Error::from)?
        .error_for_status()
        .map_err(crate::Error::from)
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
pub(crate) fn eastmoney_f10_params<'a>(source: &'a str) -> Vec<(&'a str, &'a str)> {
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
