pub fn normalize_trade_date(value: &str) -> String {
    value.get(0..10).unwrap_or(value).to_string()
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

pub fn apply_change_metrics(items: &mut [crate::types::CandlePoint]) {
    for index in 1..items.len() {
        let prev_close = items[index - 1].close;
        if prev_close > 0.0 {
            items[index].change_amount = items[index].close - prev_close;
            items[index].change_pct = (items[index].change_amount / prev_close) * 100.0;
        }
    }
}

pub fn amplitude_pct(high: f64, low: f64) -> f64 {
    if low > 0.0 {
        ((high - low) / low) * 100.0
    } else {
        0.0
    }
}

/// Extract fields from a JSON object into a Row by removing (not cloning) values.
/// The source Value must be an object; non-object values produce an empty Row.
#[allow(dead_code)]
pub fn json_obj_to_row(mut v: serde_json::Value, fields: &[&str]) -> crate::types::Row {
    let mut r = crate::types::Row::new();
    if let Some(obj) = v.as_object_mut() {
        for &field in fields {
            if let Some(val) = obj.remove(field) {
                r.insert(field.to_string(), val);
            }
        }
    }
    r
}

/// Extract a string from a JSON value without cloning (returns owned String).
#[allow(dead_code)]
pub fn json_take_str(v: &serde_json::Value, key: &str) -> String {
    v.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// Extract an f64 from a JSON value.
#[allow(dead_code)]
pub fn json_take_f64(v: &serde_json::Value, key: &str) -> f64 {
    v.get(key)
        .and_then(serde_json::Value::as_f64)
        .unwrap_or(0.0)
}

/// Extract an i64 from a JSON value.
#[allow(dead_code)]
pub fn json_take_i64(v: &serde_json::Value, key: &str) -> i64 {
    v.get(key).and_then(serde_json::Value::as_i64).unwrap_or(0)
}
