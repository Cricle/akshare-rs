use serde_json::{Value, json};

use super::TradingToolbox;

impl TradingToolbox {
    pub(super) fn summarize_stock_data_output(output: &str) -> String {
        let Ok(value) = serde_json::from_str::<Value>(output) else {
            return output.to_string();
        };
        let symbol = value
            .get("symbol")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let market_type = value
            .get("market_type")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let start_date = value
            .get("start_date")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let end_date = value
            .get("end_date")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let rows = value
            .get("rows")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if rows.is_empty() {
            return format!(
                "{{\"symbol\":\"{symbol}\",\"market_type\":\"{market_type}\",\"start_date\":\"{start_date}\",\"end_date\":\"{end_date}\",\"row_count\":0}}"
            );
        }
        let first = rows.first().cloned().unwrap_or(Value::Null);
        let last = rows.last().cloned().unwrap_or(Value::Null);
        let summary = json!({
            "symbol": symbol,
            "market_type": market_type,
            "start_date": start_date,
            "end_date": end_date,
            "row_count": rows.len(),
            "first_trade_date": first.get("trade_date").cloned().unwrap_or(Value::Null),
            "last_trade_date": last.get("trade_date").cloned().unwrap_or(Value::Null),
            "first_close": first.get("close").cloned().unwrap_or(Value::Null),
            "last_close": last.get("close").cloned().unwrap_or(Value::Null),
            "high_max": rows.iter().filter_map(|row| row.get("high").and_then(Value::as_f64)).reduce(f64::max),
            "low_min": rows.iter().filter_map(|row| row.get("low").and_then(Value::as_f64)).reduce(f64::min),
            "volume_sum": rows.iter().filter_map(|row| row.get("volume").and_then(Value::as_f64)).sum::<f64>(),
            "data_gap": value.get("data_gap").cloned().unwrap_or(Value::Null),
        });
        serde_json::to_string_pretty(&summary).unwrap_or_else(|_| output.to_string())
    }

    pub(super) fn summarize_indicator_output(output: &str, meta: &Value) -> String {
        let payload = serde_json::from_str::<Value>(output)
            .ok()
            .or_else(|| meta.get("payload").cloned());
        let Some(payload) = payload else {
            return output.to_string();
        };
        let summary = json!({
            "symbol": payload.get("symbol").cloned().unwrap_or(Value::Null),
            "start_date": payload.get("start_date").cloned().unwrap_or(Value::Null),
            "end_date": payload.get("end_date").cloned().unwrap_or(Value::Null),
            "history_candle_count": payload.get("history_candle_count").cloned().unwrap_or(Value::Null),
            "requested_window_candle_count": payload.get("requested_window_candle_count").cloned().unwrap_or(Value::Null),
            "indicators": payload.get("indicators").cloned().unwrap_or(Value::Array(Vec::new())),
            "data_gap": payload.get("data_gap").cloned().unwrap_or(Value::Null),
        });
        serde_json::to_string_pretty(&summary).unwrap_or_else(|_| output.to_string())
    }

    pub(super) fn summarize_json_object_output(output: &str, max_keys: usize) -> String {
        let Ok(value) = serde_json::from_str::<Value>(output) else {
            return output.to_string();
        };
        let Some(object) = value.as_object() else {
            return output.to_string();
        };
        let mut summary = serde_json::Map::new();
        for (index, (key, value)) in object.iter().enumerate() {
            if index >= max_keys {
                break;
            }
            summary.insert(key.clone(), value.clone());
        }
        serde_json::to_string_pretty(&Value::Object(summary)).unwrap_or_else(|_| output.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_stock_data_empty_rows() {
        let input = json!({
            "symbol": "AAPL",
            "market_type": "US",
            "start_date": "2026-01-01",
            "end_date": "2026-06-01",
            "rows": []
        });
        let result = TradingToolbox::summarize_stock_data_output(&input.to_string());
        assert!(result.contains("AAPL"));
        assert!(result.contains("row_count"));
    }

    #[test]
    fn summarize_stock_data_with_rows() {
        let input = json!({
            "symbol": "AAPL",
            "market_type": "US",
            "start_date": "2026-01-01",
            "end_date": "2026-01-03",
            "rows": [
                {"trade_date": "2026-01-01", "close": 100.0, "high": 105.0, "low": 95.0, "volume": 1000},
                {"trade_date": "2026-01-02", "close": 110.0, "high": 115.0, "low": 105.0, "volume": 2000}
            ]
        });
        let result = TradingToolbox::summarize_stock_data_output(&input.to_string());
        assert!(result.contains("row_count"));
        assert!(result.contains("high_max"));
    }

    #[test]
    fn summarize_stock_data_invalid_json() {
        let result = TradingToolbox::summarize_stock_data_output("not json");
        assert_eq!(result, "not json");
    }

    #[test]
    fn summarize_indicator_output_from_string() {
        let input = json!({
            "symbol": "AAPL",
            "start_date": "2026-01-01",
            "indicators": ["rsi", "macd"]
        });
        let result = TradingToolbox::summarize_indicator_output(&input.to_string(), &json!({}));
        assert!(result.contains("AAPL"));
        assert!(result.contains("rsi"));
    }

    #[test]
    fn summarize_indicator_output_from_meta() {
        let meta = json!({
            "payload": {
                "symbol": "GOOGL",
                "indicators": ["sma"]
            }
        });
        let result = TradingToolbox::summarize_indicator_output("invalid", &meta);
        assert!(result.contains("GOOGL"));
    }

    #[test]
    fn summarize_json_object_output_normal() {
        let input = json!({"a": 1, "b": 2, "c": 3});
        let result = TradingToolbox::summarize_json_object_output(&input.to_string(), 2);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert!(parsed.get("a").is_some());
        assert!(parsed.get("b").is_some());
        assert!(parsed.get("c").is_none());
    }

    #[test]
    fn summarize_json_object_output_invalid_json() {
        let result = TradingToolbox::summarize_json_object_output("not json", 10);
        assert_eq!(result, "not json");
    }

    #[test]
    fn summarize_json_object_output_not_object() {
        let result = TradingToolbox::summarize_json_object_output("[1,2,3]", 10);
        assert_eq!(result, "[1,2,3]");
    }
}
