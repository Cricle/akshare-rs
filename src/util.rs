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
