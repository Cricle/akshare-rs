//! Extension methods for `serde_json::Value` to reduce boilerplate
//! when extracting fields from JSON responses.

use serde_json::Value;

/// Extension trait for ergonomic JSON field extraction.
///
/// Instead of writing:
/// ```ignore
/// v.get("REPORT_DATE")
///     .or_else(|| v.get("DATE"))
///     .and_then(|x| x.as_str())
///     .unwrap_or("")
/// ```
///
/// Write:
/// ```ignore
/// v.str_or(&["REPORT_DATE", "DATE"], "")
/// ```
pub trait ValueExt {
    /// Get the first matching string field.
    fn str_field(&self, keys: &[&str]) -> Option<&str>;

    /// Get the first matching f64 field.
    fn f64_field(&self, keys: &[&str]) -> Option<f64>;

    /// Get the first matching i64 field.
    fn i64_field(&self, keys: &[&str]) -> Option<i64>;

    /// Get the first matching string field, or a default.
    fn str_or(&self, keys: &[&str], default: &str) -> String;

    /// Get the first matching f64 field, or a default.
    fn f64_or(&self, keys: &[&str], default: f64) -> f64;

    /// Get the first matching i64 field, or a default.
    fn i64_or(&self, keys: &[&str], default: i64) -> i64;

    /// Get a nested value by path (e.g., `["data", "items"]`).
    fn nested(&self, path: &[&str]) -> Option<&Value>;
}

impl ValueExt for Value {
    fn str_field(&self, keys: &[&str]) -> Option<&str> {
        for k in keys {
            if let Some(s) = self.get(k).and_then(Self::as_str) {
                return Some(s);
            }
        }
        None
    }

    fn f64_field(&self, keys: &[&str]) -> Option<f64> {
        for k in keys {
            if let Some(v) = self.get(k).and_then(Self::as_f64) {
                return Some(v);
            }
        }
        None
    }

    fn i64_field(&self, keys: &[&str]) -> Option<i64> {
        for k in keys {
            if let Some(v) = self.get(k).and_then(Self::as_i64) {
                return Some(v);
            }
        }
        None
    }

    fn str_or(&self, keys: &[&str], default: &str) -> String {
        self.str_field(keys).unwrap_or(default).to_string()
    }

    fn f64_or(&self, keys: &[&str], default: f64) -> f64 {
        self.f64_field(keys).unwrap_or(default)
    }

    fn i64_or(&self, keys: &[&str], default: i64) -> i64 {
        self.i64_field(keys).unwrap_or(default)
    }

    fn nested(&self, path: &[&str]) -> Option<&Value> {
        let mut current = self;
        for k in path {
            current = current.get(k)?;
        }
        Some(current)
    }
}
