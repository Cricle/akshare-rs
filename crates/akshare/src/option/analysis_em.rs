//! Option analysis data from Eastmoney (premium, value, risk).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ClistEnvelope {
    data: Option<ClistData>,
}

#[derive(Debug, Default, Deserialize)]
struct ClistData {
    total: Option<usize>,
    #[serde(default)]
    diff: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// Option premium analysis row from Eastmoney.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionPremiumAnalysisRow {
    /// Option code.
    pub option_code: String,
    /// Option name.
    pub option_name: String,
    /// Latest price.
    pub latest_price: f64,
    /// Change percent.
    pub change_pct: f64,
    /// Exercise price.
    pub exercise_price: f64,
    /// Premium/discount rate.
    pub premium_rate: f64,
    /// Underlying name.
    pub underlying_name: String,
    /// Underlying latest price.
    pub underlying_price: f64,
    /// Underlying change percent.
    pub underlying_change_pct: f64,
    /// Breakeven price.
    pub breakeven_price: f64,
    /// Expiry date.
    pub expiry_date: String,
}

/// Option value analysis row from Eastmoney.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionValueAnalysisRow {
    /// Option code.
    pub option_code: String,
    /// Option name.
    pub option_name: String,
    /// Latest price.
    pub latest_price: f64,
    /// Time value.
    pub time_value: f64,
    /// Intrinsic value.
    pub intrinsic_value: f64,
    /// Implied volatility.
    pub implied_volatility: f64,
    /// Theoretical price.
    pub theoretical_price: f64,
    /// Underlying name.
    pub underlying_name: String,
    /// Underlying latest price.
    pub underlying_price: f64,
    /// Underlying 1-year volatility.
    pub underlying_volatility: f64,
    /// Expiry date.
    pub expiry_date: String,
}

/// Option risk analysis row from Eastmoney.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionRiskAnalysisRow {
    /// Option code.
    pub option_code: String,
    /// Option name.
    pub option_name: String,
    /// Latest price.
    pub latest_price: f64,
    /// Change percent.
    pub change_pct: f64,
    /// Leverage ratio.
    pub leverage_ratio: f64,
    /// Effective leverage ratio.
    pub effective_leverage: f64,
    /// Delta.
    pub delta: f64,
    /// Gamma.
    pub gamma: f64,
    /// Vega.
    pub vega: f64,
    /// Rho.
    pub rho: f64,
    /// Theta.
    pub theta: f64,
    /// Expiry date.
    pub expiry_date: String,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Option premium analysis from Eastmoney.
    pub async fn option_premium_analysis_em(&self) -> Result<Vec<OptionPremiumAnalysisRow>> {
        let resp = self
            .fetch_em_option_clist(
                "f250",
                "m:10",
                "f1,f2,f3,f12,f13,f14,f161,f250,f330,f331,f332,f333,f334,f335,f337,f301,f152",
            )
            .await?;

        let mut rows = Vec::with_capacity(resp.len());
        for item in &resp {
            if item.len() < 18 {
                continue;
            }
            rows.push(OptionPremiumAnalysisRow {
                option_code: json_str_idx(item, 12),
                option_name: json_str_idx(item, 14),
                latest_price: json_f64_idx(item, 2),
                change_pct: json_f64_idx(item, 3),
                exercise_price: json_f64_idx(item, 7),
                premium_rate: json_f64_idx(item, 8),
                underlying_name: json_str_idx(item, 13),
                underlying_price: json_f64_idx(item, 15),
                underlying_change_pct: json_f64_idx(item, 16),
                breakeven_price: json_f64_idx(item, 17),
                expiry_date: json_str_idx(item, 9),
            });
        }

        if rows.is_empty() {
            return Err(Error::not_found("no premium analysis data"));
        }

        Ok(rows)
    }

    /// Option value analysis from Eastmoney.
    pub async fn option_value_analysis_em(&self) -> Result<Vec<OptionValueAnalysisRow>> {
        let resp = self
            .fetch_em_option_clist(
                "f301",
                "m:10",
                "f1,f2,f3,f12,f13,f14,f298,f299,f249,f300,f330,f331,f332,f333,f334,f335,f336,f301,f152",
            )
            .await?;

        let mut rows = Vec::with_capacity(resp.len());
        for item in &resp {
            if item.len() < 19 {
                continue;
            }
            rows.push(OptionValueAnalysisRow {
                option_code: json_str_idx(item, 12),
                option_name: json_str_idx(item, 14),
                latest_price: json_f64_idx(item, 2),
                time_value: json_f64_idx(item, 8),
                intrinsic_value: json_f64_idx(item, 9),
                implied_volatility: json_f64_idx(item, 7),
                theoretical_price: json_f64_idx(item, 10),
                underlying_name: json_str_idx(item, 13),
                underlying_price: json_f64_idx(item, 16),
                underlying_volatility: json_f64_idx(item, 18),
                expiry_date: json_str_idx(item, 11),
            });
        }

        if rows.is_empty() {
            return Err(Error::not_found("no value analysis data"));
        }

        Ok(rows)
    }

    /// Option risk analysis from Eastmoney.
    pub async fn option_risk_analysis_em(&self) -> Result<Vec<OptionRiskAnalysisRow>> {
        let resp = self
            .fetch_em_option_clist(
                "f12",
                "m:10",
                "f1,f2,f3,f12,f13,f14,f302,f303,f325,f326,f327,f329,f328,f301,f152,f154",
            )
            .await?;

        let mut rows = Vec::with_capacity(resp.len());
        for item in &resp {
            if item.len() < 16 {
                continue;
            }
            rows.push(OptionRiskAnalysisRow {
                option_code: json_str_idx(item, 12),
                option_name: json_str_idx(item, 14),
                latest_price: json_f64_idx(item, 2),
                change_pct: json_f64_idx(item, 3),
                leverage_ratio: json_f64_idx(item, 9),
                effective_leverage: json_f64_idx(item, 10),
                delta: json_f64_idx(item, 11),
                gamma: json_f64_idx(item, 12),
                vega: json_f64_idx(item, 13),
                rho: json_f64_idx(item, 14),
                theta: json_f64_idx(item, 15),
                expiry_date: json_str_idx(item, 8),
            });
        }

        if rows.is_empty() {
            return Err(Error::not_found("no risk analysis data"));
        }

        Ok(rows)
    }

    // -- private helper -----------------------------------------------------

    /// Fetch all pages from Eastmoney clist API for option data.
    async fn fetch_em_option_clist(
        &self,
        fid: &str,
        fs: &str,
        fields: &str,
    ) -> Result<Vec<Vec<serde_json::Value>>> {
        let mut all_data = Vec::new();
        let mut page = 1_u32;
        let page_size = 100;

        loop {
            let pz = page_size.to_string();
            let pn = page.to_string();

            let resp: ClistEnvelope = self
                .get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&[
                    ("fid", fid),
                    ("po", "1"),
                    ("pz", pz.as_str()),
                    ("pn", pn.as_str()),
                    ("np", "1"),
                    ("fltt", "2"),
                    ("invt", "2"),
                    ("ut", "b2884a393a59ad64002292a3e90d46a5"),
                    ("fields", fields),
                    ("fs", fs),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .json()
                .await
                .map_err(Error::from)?;

            let data = resp.data.unwrap_or_default();
            let total = data.total.unwrap_or(0);
            let diff = data.diff;

            if diff.is_empty() {
                break;
            }

            // Extract arrays from the diff items
            let diff_len = diff.len();
            for item in &diff {
                if let Some(arr) = item.as_array() {
                    all_data.push(arr.clone());
                } else if let Some(obj) = item.as_object() {
                    let mut arr = Vec::new();
                    let max_key = obj
                        .keys()
                        .filter_map(|k| k.parse::<usize>().ok())
                        .max()
                        .unwrap_or(0);
                    for i in 0..=max_key {
                        let key = i.to_string();
                        arr.push(obj.get(&key).cloned().unwrap_or(serde_json::Value::Null));
                    }
                    all_data.push(arr);
                }
            }

            if all_data.len() >= total || page_size > diff_len as u32 {
                break;
            }
            page += 1;
        }

        Ok(all_data)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn json_str_idx(arr: &[serde_json::Value], idx: usize) -> String {
    arr.get(idx)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn json_f64_idx(arr: &[serde_json::Value], idx: usize) -> f64 {
    match arr.get(idx) {
        Some(serde_json::Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(serde_json::Value::String(s)) => s.trim().parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}
