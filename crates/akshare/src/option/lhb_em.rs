//! Option billboard (dragon-tiger list) from Eastmoney.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct DatacenterEnvelope {
    result: Option<DatacenterResult>,
}

#[derive(Debug, Deserialize)]
struct DatacenterResult {
    #[serde(default)]
    data: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// Option billboard entry from Eastmoney.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionLhbEntry {
    /// Trade type description.
    pub trade_type: String,
    /// Trade date.
    pub trade_date: String,
    /// Security code.
    pub security_code: String,
    /// Target name.
    pub target_name: String,
    /// Rank.
    pub rank: i64,
    /// Institution name.
    pub institution: String,
    /// Volume or position (depends on indicator).
    pub value: f64,
    /// Change.
    pub change: f64,
    /// Net value.
    pub net_value: f64,
    /// Ratio of total.
    pub ratio: f64,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Option billboard from Eastmoney.
    ///
    /// `symbol` is the underlying code, e.g. "510050", "510300", "159919".
    /// `indicator` is one of:
    /// - "\u{9009}\u{6743}\u{4ea4}\u{6613}\u{60c5}\u{51b5}-\u{8ba4}\u{6cbd}\u{4ea4}\u{6613}\u{91cf}"
    /// - "\u{9009}\u{6743}\u{6301}\u{4ed3}\u{60c5}\u{51b5}-\u{8ba4}\u{6cbd}\u{6301}\u{4ed3}\u{91cf}"
    /// - "\u{9009}\u{6743}\u{4ea4}\u{6613}\u{60c5}\u{51b5}-\u{8ba4}\u{8d2d}\u{4ea4}\u{6613}\u{91cf}"
    /// - "\u{9009}\u{6743}\u{6301}\u{4ed3}\u{60c5}\u{51b5}-\u{8ba4}\u{8d2d}\u{6301}\u{4ed3}\u{91cf}"
    ///   `trade_date` is "YYYYMMDD".
    pub async fn option_lhb(
        &self,
        symbol: &str,
        indicator: &str,
        trade_date: &str,
    ) -> Result<Vec<OptionLhbEntry>> {
        let date_formatted = if trade_date.len() >= 8 {
            format!(
                "{}-{}-{}",
                &trade_date[..4],
                &trade_date[4..6],
                &trade_date[6..8]
            )
        } else {
            trade_date.to_string()
        };

        let filter = format!("(SECURITY_CODE=\"{symbol}\")(TRADE_DATE='{date_formatted}')");

        let resp: DatacenterEnvelope = self
            .get("https://datacenter-web.eastmoney.com/api/data/get")
            .query(&[
                ("type", "RPT_IF_BILLBOARD_TD"),
                ("sty", "ALL"),
                ("filter", filter.as_str()),
                ("p", "1"),
                ("pss", "200"),
                ("source", "IFBILLBOARD"),
                ("client", "WEB"),
                ("ut", "b2884a393a59ad64002292a3e90d46a5"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();

        // Each request returns 4 groups of 7 rows:
        // [0..7]: put volume, [7..14]: put position, [14..21]: call volume, [21..]: call position
        let (start, end, value_key, net_key) = match indicator {
            "\u{9009}\u{6743}\u{4ea4}\u{6613}\u{60c5}\u{51b5}-\u{8ba4}\u{6cbd}\u{4ea4}\u{6613}\u{91cf}" => {
                (0, 7, 8, 10)
            }
            "\u{9009}\u{6743}\u{6301}\u{4ed3}\u{60c5}\u{51b5}-\u{8ba4}\u{6cbd}\u{6301}\u{4ed3}\u{91cf}" => {
                (7, 14, 13, 15)
            }
            "\u{9009}\u{6743}\u{4ea4}\u{6613}\u{60c5}\u{51b5}-\u{8ba4}\u{8d2d}\u{4ea4}\u{6613}\u{91cf}" => {
                (14, 21, 17, 19)
            }
            "\u{9009}\u{6743}\u{6301}\u{4ed3}\u{60c5}\u{51b5}-\u{8ba4}\u{8d2d}\u{6301}\u{4ed3}\u{91cf}" => {
                (21, usize::MAX, 12, 14)
            }
            other => {
                return Err(Error::invalid_input(format!(
                    "unsupported LHB indicator: {other}"
                )));
            }
        };

        let end = end.min(data.len());
        let mut rows = Vec::new();

        for item in data.iter().take(end).skip(start) {
            let arr = item.as_array();
            let get_str = |idx: usize| -> String {
                arr.and_then(|a| a.get(idx))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            };
            let get_f64 = |idx: usize| -> f64 {
                arr.and_then(|a| a.get(idx))
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0)
            };

            // For the last group (call position), use named fields
            if indicator
                == "\u{9009}\u{6743}\u{6301}\u{4ed3}\u{60c5}\u{51b5}-\u{8ba4}\u{8d2d}\u{6301}\u{4ed3}\u{91cf}"
            {
                rows.push(OptionLhbEntry {
                    trade_type: get_str(0),
                    trade_date: get_str(1),
                    security_code: get_str(2),
                    target_name: get_str(3),
                    rank: get_f64(6) as i64,
                    institution: get_str(5),
                    value: get_f64(12),
                    change: get_f64(13),
                    net_value: get_f64(14),
                    ratio: get_f64(15),
                });
            } else {
                rows.push(OptionLhbEntry {
                    trade_type: get_str(0),
                    trade_date: get_str(1),
                    security_code: get_str(2),
                    target_name: get_str(3),
                    rank: get_f64(7) as i64,
                    institution: get_str(6),
                    value: get_f64(value_key),
                    change: get_f64(value_key + 1),
                    net_value: get_f64(net_key),
                    ratio: get_f64(net_key + 1),
                });
            }
        }

        Ok(rows)
    }
}
