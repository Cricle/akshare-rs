//! Drewry World Container Index.
//!
//! Upstream `index_wci_drewry` scrapes the public Infogram embed and reads a
//! `window.infographicData` JSON blob. This module implements the same
//! extraction in pure Rust: locate the marker, take the balanced JSON object,
//! sanitize JavaScript-only literals, and read the requested chart series.

use serde_json::Value;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

const INFOGRAM_URL: &str = "https://infogram.com/world-container-index-1h17493095xl4zj";
const CHART_ENTITY_ID: &str = "7a55585f-3fb3-44e6-9b54-beea1cd20b4d";

impl AkShareClient {
    /// Drewry 集装箱指数.
    ///
    /// `symbol` accepts the upstream names: `composite`, `shanghai-rotterdam`,
    /// `rotterdam-shanghai`, `shanghai-los angeles`, `los angeles-shanghai`,
    /// `shanghai-genoa`, `new york-rotterdam`, `rotterdam-new york`.
    ///
    /// Returns objects with `date` (`YYYY-MM-DD`) and `wci` (numeric).
    pub async fn drewry_wci_index(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let series_index = drewry_series_index(symbol)
            .ok_or_else(|| Error::invalid_input(format!("unknown Drewry WCI symbol `{symbol}`")))?;

        let html =
            crate::util::send_and_check_text(self.get(INFOGRAM_URL), Some(encoding_rs::UTF_8))
                .await?;
        let payload = extract_infographic_payload(&html)?;
        drewry_rows(&payload, series_index)
    }
}

/// Map an upstream symbol to the Infogram chart-series index.
fn drewry_series_index(symbol: &str) -> Option<usize> {
    match symbol.trim().to_ascii_lowercase().as_str() {
        "composite" | "world container index" | "wci" => Some(0),
        "shanghai-rotterdam" => Some(1),
        "rotterdam-shanghai" => Some(2),
        "shanghai-los angeles" => Some(3),
        "los angeles-shanghai" => Some(4),
        "shanghai-genoa" => Some(5),
        "new york-rotterdam" => Some(6),
        "rotterdam-new york" => Some(7),
        _ => None,
    }
}

/// Extract and parse the `window.infographicData=...` JSON blob from a page.
fn extract_infographic_payload(html: &str) -> Result<Value> {
    const MARKER: &str = "window.infographicData=";
    let marker = html
        .find(MARKER)
        .ok_or_else(|| Error::decode("Infogram page is missing `window.infographicData`"))?;
    let rest = html[marker + MARKER.len()..].trim_start();
    let object = balanced_json_object(rest)
        .ok_or_else(|| Error::decode("Infogram `window.infographicData` is not a JSON object"))?;
    serde_json::from_str(&sanitize_jsonish(object))
        .map_err(|e| Error::decode(format!("failed to parse Infogram payload: {e}")))
}

/// Return the first balanced `{...}` object in `text`, ignoring braces inside
/// JSON strings.
fn balanced_json_object(text: &str) -> Option<&str> {
    let start = text.find('{')?;
    let bytes = text.as_bytes();
    let mut depth = 0_usize;
    let mut in_string = false;
    let mut escaped = false;

    for (offset, byte) in bytes.iter().enumerate().skip(start) {
        if in_string {
            if escaped {
                escaped = false;
            } else if *byte == b'\\' {
                escaped = true;
            } else if *byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&text[start..=offset]);
                }
            }
            _ => {}
        }
    }
    None
}

/// Replace JavaScript-only literals (`NaN`, `Infinity`, `undefined`) with
/// `null` and drop trailing commas outside strings, so `serde_json` accepts
/// the Infogram payload.
fn sanitize_jsonish(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.char_indices().peekable();
    let mut in_string = false;
    let mut escaped = false;

    while let Some((index, ch)) = chars.next() {
        if in_string {
            out.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => {
                in_string = true;
                out.push(ch);
            }
            ',' => {
                let mut lookahead = chars.clone();
                let next = lookahead.by_ref().find(|(_, c)| !c.is_whitespace());
                if !matches!(next.map(|(_, c)| c), Some('}' | ']')) {
                    out.push(',');
                }
            }
            _ => {
                let rest = &input[index..];
                let keyword = [
                    ("NaN", 3_usize),
                    ("-Infinity", 9),
                    ("Infinity", 8),
                    ("undefined", 9),
                ]
                .into_iter()
                .find(|(token, _)| rest.starts_with(token));
                let Some((_, length)) = keyword else {
                    out.push(ch);
                    continue;
                };
                let boundary_ok = rest[length..]
                    .chars()
                    .next()
                    .is_none_or(|next| !next.is_alphanumeric() && next != '_');
                if boundary_ok {
                    out.push_str("null");
                    // `ch` itself is already consumed, so skip the remainder.
                    for _ in 1..length {
                        chars.next();
                    }
                } else {
                    out.push(ch);
                }
            }
        }
    }
    out
}

/// Read one chart series into `{date, wci}` objects.
fn drewry_rows(payload: &Value, series_index: usize) -> Result<Vec<serde_json::Value>> {
    let series = payload
        .pointer("/elements/content/content/entities")
        .and_then(Value::as_object)
        .and_then(|entities| entities.get(CHART_ENTITY_ID))
        .and_then(|entity| entity.get("data"))
        .and_then(Value::as_array)
        .and_then(|series| series.get(series_index))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            Error::upstream(format!(
                "Drewry Infogram payload has no chart series at index {series_index}"
            ))
        })?;

    let mut rows = Vec::new();
    for item in series.iter().skip(1) {
        let Some(cells) = item.as_array() else {
            continue;
        };
        let Some(raw_date) = cells
            .first()
            .and_then(|cell| cell.get("value"))
            .and_then(Value::as_str)
        else {
            continue;
        };
        let value = cells
            .get(1)
            .and_then(|cell| cell.get("value"))
            .and_then(jsonish_f64);
        let (Some(date), Some(value)) = (parse_wci_date(raw_date), value) else {
            continue;
        };
        rows.push(serde_json::json!({"date": date, "wci": value}));
    }

    if rows.is_empty() {
        return Err(Error::not_found(
            "Drewry Infogram payload contained no WCI rows",
        ));
    }
    Ok(rows)
}

fn jsonish_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64(),
        Value::String(text) => text.trim().replace(',', "").parse().ok(),
        _ => None,
    }
}

/// Parse Infogram's `%d-%b-%y` dates (e.g. `10-Mar-16`) to ISO text.
fn parse_wci_date(raw: &str) -> Option<String> {
    chrono::NaiveDate::parse_from_str(raw.trim(), "%d-%b-%y")
        .ok()
        .map(|date| date.format("%Y-%m-%d").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_payload() -> Value {
        serde_json::json!({
            "elements": {
                "content": {
                    "content": {
                        "entities": {
                            CHART_ENTITY_ID: {
                                "data": [
                                    [
                                        [null, {"value": "Composite"}],
                                        [{"value": "10-Mar-16"}, {"value": "700.57"}],
                                        [{"value": "17-Mar-16"}, {"value": "674.41"}],
                                        [{"value": "24-Mar-16"}, null]
                                    ]
                                ]
                            }
                        }
                    }
                }
            }
        })
    }

    #[test]
    fn symbol_mapping_matches_upstream() {
        assert_eq!(drewry_series_index("composite"), Some(0));
        assert_eq!(drewry_series_index("World Container Index"), Some(0));
        assert_eq!(drewry_series_index("shanghai-los angeles"), Some(3));
        assert_eq!(drewry_series_index("rotterdam-new york"), Some(7));
        assert_eq!(drewry_series_index("nope"), None);
    }

    #[test]
    fn extract_payload_handles_js_literals_and_trailing_commas() {
        let html = "<script>window.infographicData={\"a\":NaN,\"b\":[1,2,],};</script>";
        let payload = extract_infographic_payload(html).expect("payload should parse");
        assert!(payload["a"].is_null());
        assert_eq!(payload["b"], serde_json::json!([1, 2]));
    }

    #[test]
    fn extract_payload_respects_braces_inside_strings() {
        let html = "window.infographicData={\"text\":\"a}b{c\"}";
        let payload = extract_infographic_payload(html).expect("payload should parse");
        assert_eq!(payload["text"], "a}b{c");
    }

    #[test]
    fn drewry_rows_skip_header_and_incomplete_rows() {
        let rows = drewry_rows(&sample_payload(), 0).expect("series should parse");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["date"], "2016-03-10");
        assert_eq!(rows[0]["wci"], 700.57);
        assert_eq!(rows[1]["date"], "2016-03-17");
        assert_eq!(rows[1]["wci"], 674.41);
    }

    #[test]
    fn missing_series_is_reported() {
        let payload = sample_payload();
        let error = drewry_rows(&payload, 3).expect_err("series 3 is absent");
        assert!(error.to_string().contains("no chart series"));
    }

    #[test]
    fn parse_wci_date_formats_iso() {
        assert_eq!(parse_wci_date("10-Mar-16").as_deref(), Some("2016-03-10"));
        assert_eq!(parse_wci_date("not a date"), None);
    }
}
