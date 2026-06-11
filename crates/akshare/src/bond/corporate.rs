//! Corporate bond issuance data from Eastmoney datacenter.
//!
//! Uses `RPT_BOND_ISSUE` report to fetch corporate bond issuance records.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::BondSnapshot;

impl AkShareClient {
    /// Fetch corporate bond issuance data.
    ///
    /// Returns bond issuance records with yield information from the
    /// Eastmoney datacenter (`RPT_BOND_ISSUE` report).
    pub async fn bond_corporate_yields(&self, limit: usize) -> Result<Vec<BondSnapshot>> {
        let page_size = limit.clamp(1, 500).to_string();

        let response = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_BOND_ISSUE"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", page_size.as_str()),
                ("sortTypes", "-1"),
                ("sortColumns", "ISSUE_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("result")
            .and_then(|r| r.get("data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        if data.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no corporate bond issuance items",
            ));
        }

        let today = crate::util::today_iso();
        let items: Vec<BondSnapshot> = data
            .into_iter()
            .take(limit)
            .filter_map(|v| {
                let symbol =
                    extract_str(&v, &["SECURITY_CODE", "BOND_CODE", "SECCODE"]).unwrap_or_default();
                if symbol.is_empty() {
                    return None;
                }
                let name = extract_str(
                    &v,
                    &["SECURITY_NAME_ABBR", "BOND_NAME", "SECNAME", "SHORT_NAME"],
                )
                .unwrap_or_else(|| symbol.clone());

                let date = extract_str(&v, &["ISSUE_DATE", "DECLAREDATE"])
                    .unwrap_or_else(|| today.clone());

                // Try to extract price or face value
                let close = extract_f64(
                    &v,
                    &[
                        "ISSUE_PRICE",
                        "PAR_VALUE",
                        "FACE_VALUE",
                        "CURRENT_BOND_PRICE",
                    ],
                )
                .unwrap_or(100.0);

                // Try to extract coupon rate / yield
                let yield_rate = extract_f64(
                    &v,
                    &["COUPON_RATE", "INTEREST_RATE", "YIELD_RATE", "ACTUAL_RATE"],
                );

                Some(BondSnapshot {
                    symbol,
                    name,
                    date: date.get(..10).unwrap_or(&date).to_string(),
                    close,
                    change_pct: 0.0,
                    yield_rate,
                    credit_rating: None,
                })
            })
            .collect();

        Ok(items)
    }
}

/// Extract a string from a JSON value by trying multiple possible keys.
fn extract_str(v: &serde_json::Value, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(s) = v.get(*key).and_then(|x| x.as_str()) {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

/// Extract an f64 from a JSON value by trying multiple possible keys.
fn extract_f64(v: &serde_json::Value, keys: &[&str]) -> Option<f64> {
    for key in keys {
        if let Some(n) = v.get(*key).and_then(|x| x.as_f64()) {
            return Some(n);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_str() {
        let v = json!({"SECURITY_CODE": "123456", "BOND_NAME": "Test Bond"});
        assert_eq!(
            extract_str(&v, &["SECURITY_CODE", "BOND_CODE"]),
            Some("123456".to_string())
        );
        assert_eq!(
            extract_str(&v, &["MISSING", "BOND_NAME"]),
            Some("Test Bond".to_string())
        );
        assert_eq!(extract_str(&v, &["MISSING", "ALSO_MISSING"]), None);
    }

    #[test]
    fn test_extract_f64() {
        let v = json!({"COUPON_RATE": 3.5, "ISSUE_PRICE": 100.0});
        assert!((extract_f64(&v, &["COUPON_RATE"]).unwrap() - 3.5).abs() < 0.01);
        assert!((extract_f64(&v, &["MISSING", "ISSUE_PRICE"]).unwrap() - 100.0).abs() < 0.01);
        assert_eq!(extract_f64(&v, &["MISSING"]), None);
    }

    #[test]
    fn test_extract_str_empty_values() {
        let v = json!({"SECURITY_CODE": "", "BOND_NAME": "  "});
        assert_eq!(extract_str(&v, &["SECURITY_CODE"]), None);
        assert_eq!(extract_str(&v, &["BOND_NAME"]), None);
    }
}
