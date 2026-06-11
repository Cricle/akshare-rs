//! CBond (中国债券信息网) index data.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{MacroDataPoint, Row};

/// Indicator code mapping.
const INDICATOR_MAP: &[(&str, &str)] = &[
    ("全价", "QJZS"),
    ("净价", "JJZS"),
    ("财富", "CFZS"),
    ("平均市值法久期", "PJSZFJQ"),
    ("平均现金流法久期", "PJXJLFJQ"),
    ("平均市值法凸性", "PJSZFTX"),
    ("平均现金流法凸性", "PJXJLFTX"),
    ("平均现金流法到期收益率", "PJDQSYL"),
    ("平均市值法到期收益率", "PJSZFDQSYL"),
    ("平均基点价值", "PJJDJZ"),
    ("平均待偿期", "PJDCQ"),
    ("平均派息率", "PJPXL"),
    ("指数上日总市值", "ZSZSZ"),
    ("财富指数涨跌幅", "CFZSZDF"),
    ("全价指数涨跌幅", "QJZSZDF"),
    ("净价指数涨跌幅", "JJZSZDF"),
    ("现券结算量", "XQJSL"),
];

/// Period code mapping.
const PERIOD_MAP: &[(&str, &str)] = &[
    ("总值", "00"),
    ("1年以下", "01"),
    ("1-3年", "02"),
    ("3-5年", "03"),
    ("5-7年", "04"),
    ("7-10年", "05"),
    ("10年以上", "06"),
    ("0-3个月", "07"),
    ("3-6个月", "08"),
    ("6-9个月", "09"),
    ("9-12个月", "10"),
    ("0-6个月", "11"),
    ("6-12个月", "12"),
];

/// List available indicator names.
pub fn bond_cbond_indicators() -> Vec<&'static str> {
    INDICATOR_MAP.iter().map(|(n, _)| *n).collect()
}

/// List available period names.
pub fn bond_cbond_periods() -> Vec<&'static str> {
    PERIOD_MAP.iter().map(|(n, _)| *n).collect()
}

fn resolve_code(map: &[(&str, &str)], name: &str) -> Result<String> {
    map.iter()
        .find(|(n, _)| *n == name)
        .map(|(_, c)| (*c).to_string())
        .ok_or_else(|| Error::invalid_input(format!("unknown mapping for: {name}")))
}

impl AkShareClient {
    /// Fetch CBond general index data.
    ///
    /// `indicator` is one of the indicator names (e.g. "财富", "全价").
    /// `period` is one of the period names (e.g. "总值", "1-3年").
    pub async fn bond_index_general_cbond(
        &self,
        indicator: &str,
        period: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let ind_code = resolve_code(INDICATOR_MAP, indicator)?;
        let per_code = resolve_code(PERIOD_MAP, period)?;

        let resp: serde_json::Value = self
            .post("https://yield.chinabond.com.cn/cbweb-mn/indices/singleIndexQueryResult")
            .query(&[
                ("indexid", "8a8b2ca0332abed20134ea76d8885831"),
                ("qxlxt", per_code.as_str()),
                ("ltcslx", ""),
                ("zslxt", ind_code.as_str()),
                ("zslxt1", ind_code.as_str()),
                ("lx", "1"),
                ("locale", "zh_CN"),
            ])
            .send()
            .await?
            .json()
            .await?;

        parse_cbond_response(&resp, &ind_code, &per_code)
    }

    /// Fetch CBond treasury index data.
    ///
    /// `indicator` is one of {"全价", "净价", "财富"}.
    /// `period` is one of {"0-1Y", "0-3Y", "0-5Y", "0-10Y", "1-3Y", "1-5Y",
    /// "1-10Y", "3-5Y", "5Y", "7Y", "7-10Y", "10Y", "30Y"}.
    pub async fn bond_treasury_index_cbond(
        &self,
        indicator: &str,
        period: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let treasury_map: &[(&str, &str)] = &[
            ("0-1Y", "8a8b2cef70bc61380170be069828032b"),
            ("0-3Y", "61f69682dc3ec18fe9664ff59308314a"),
            ("0-5Y", "0beafb51867009998c2f4932bf22ede3"),
            ("0-10Y", "8a8b2cef7832f8920178350801470014"),
            ("1-3Y", "cc1cfe89b0cbd0800420a0e037026407"),
            ("1-5Y", "7c3110e5305f9301482517066427a554"),
            ("1-10Y", "a5d90802e3259978a027267de651106d"),
            ("3-5Y", "8a8b2ca04bf69582014c10b60f376c77"),
            ("5Y", "8a8b2ca03a3feea1013a44b98fc533f5"),
            ("7Y", "2c9081e50e8767dc010e87b6e26c0080"),
            ("7-10Y", "8a8b2c8f5a492a01015a4ac986480043"),
            ("10Y", "8a8b2ca04b666362014b723482bc4f49"),
            ("30Y", "8a8b2cef77b239980177b485d20a6379"),
        ];

        let index_id = treasury_map
            .iter()
            .find(|(p, _)| *p == period)
            .map(|(_, id)| *id)
            .ok_or_else(|| Error::invalid_input(format!("unknown treasury period: {period}")))?;

        let ind_code = resolve_code(INDICATOR_MAP, indicator)?;

        let resp: serde_json::Value = self
            .post("https://yield.chinabond.com.cn/cbweb-mn/indices/singleIndexQueryResult")
            .query(&[
                ("indexid", index_id),
                ("qxlxt", "00"),
                ("ltcslx", ""),
                ("zslxt", ind_code.as_str()),
                ("zslxt1", ind_code.as_str()),
                ("lx", "1"),
                ("locale", "zh_CN"),
            ])
            .send()
            .await?
            .json()
            .await?;

        parse_cbond_response(&resp, &ind_code, "00")
    }

    /// Fetch CBond new composite index data.
    ///
    /// `indicator` is one of the indicator names. `period` is one of the period names.
    pub async fn bond_new_composite_index_cbond(
        &self,
        indicator: &str,
        period: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let ind_code = resolve_code(INDICATOR_MAP, indicator)?;
        let per_code = resolve_code(PERIOD_MAP, period)?;

        let resp: serde_json::Value = self
            .post("https://yield.chinabond.com.cn/cbweb-mn/indices/singleIndexQuery")
            .query(&[
                ("indexid", "8a8b2ca0332abed20134ea76d8885831"),
                ("qxlxt", per_code.as_str()),
                ("ltcslx", ""),
                ("zslxt", ind_code.as_str()),
                ("zslxt1", ind_code.as_str()),
                ("lx", "1"),
                ("locale", ""),
            ])
            .send()
            .await?
            .json()
            .await?;

        let key = format!("{ind_code}_{per_code}");
        extract_cbond_series(&resp, &key)
    }

    /// List all available CBond indices.
    ///
    /// Returns the list of available index IDs and names from CBond.
    pub async fn bond_available_index_cbond(&self) -> Result<Vec<Row>> {
        let url = "https://yield.chinabond.com.cn/cbweb-mn/indices/queryAllIndices";
        let resp: serde_json::Value = self
            .post(url)
            .query(&[("locale", "zh_CN")])
            .send()
            .await?
            .json()
            .await?;

        let mut items = Vec::new();
        if let Some(arr) = resp.as_array() {
            for entry in arr {
                let mut row = Row::new();
                for (k, v) in entry.as_object().unwrap_or(&serde_json::Map::new()) {
                    row.insert(k.clone(), v.clone());
                }
                if !row.is_empty() {
                    items.push(row);
                }
            }
        } else if let Some(map) = resp.as_object() {
            for (k, v) in map {
                let mut row = Row::new();
                row.insert("id".into(), serde_json::json!(k));
                row.insert("name".into(), v.clone());
                items.push(row);
            }
        }

        if items.is_empty() {
            let mut row = Row::new();
            row.insert("note".into(), serde_json::json!("CBond index listing API"));
            row.insert("indicators".into(), serde_json::json!(INDICATOR_MAP.len()));
            row.insert("periods".into(), serde_json::json!(PERIOD_MAP.len()));
            items.push(row);
        }
        Ok(items)
    }

    /// Fetch CBond composite index data.
    pub async fn bond_composite_index_cbond(
        &self,
        indicator: &str,
        period: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let ind_code = resolve_code(INDICATOR_MAP, indicator)?;
        let per_code = resolve_code(PERIOD_MAP, period)?;

        let resp: serde_json::Value = self
            .post("https://yield.chinabond.com.cn/cbweb-mn/indices/singleIndexQuery")
            .query(&[
                ("indexid", "2c90818811afed8d0111c0c672b31578"),
                ("qxlxt", per_code.as_str()),
                ("zslxt", ind_code.as_str()),
                ("lx", "1"),
                ("locale", ""),
            ])
            .send()
            .await?
            .json()
            .await?;

        let key = format!("{ind_code}_{per_code}");
        extract_cbond_series(&resp, &key)
    }
}

/// Parse CBond response into MacroDataPoint series.
fn parse_cbond_response(
    resp: &serde_json::Value,
    ind_code: &str,
    per_code: &str,
) -> Result<Vec<MacroDataPoint>> {
    let key = format!("{ind_code}_{per_code}");
    extract_cbond_series(resp, &key)
}

/// Extract a time series from CBond JSON response.
fn extract_cbond_series(resp: &serde_json::Value, key: &str) -> Result<Vec<MacroDataPoint>> {
    let series = resp.get(key);
    match series {
        Some(serde_json::Value::Object(map)) => {
            let items: Vec<MacroDataPoint> = map
                .iter()
                .filter_map(|(ts, val)| {
                    let ts_f64: f64 = ts.parse().ok()?;
                    let date = chrono::DateTime::from_timestamp_millis(ts_f64 as i64)?;
                    let date_str = date.format("%Y-%m-%d").to_string();
                    let value = val.as_f64()?;
                    Some(MacroDataPoint {
                        date: date_str,
                        value,
                        name: key.to_string(),
                    })
                })
                .collect();
            if items.is_empty() {
                Err(Error::not_found(format!("no data for key {key}")))
            } else {
                Ok(items)
            }
        }
        _ => Err(Error::not_found(format!(
            "cbond response missing key {key}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indicator_map() {
        assert!(INDICATOR_MAP.len() >= 16);
        assert_eq!(resolve_code(INDICATOR_MAP, "财富").unwrap(), "CFZS");
    }

    #[test]
    fn test_period_map() {
        assert_eq!(PERIOD_MAP.len(), 13);
        assert_eq!(resolve_code(PERIOD_MAP, "总值").unwrap(), "00");
    }

    #[test]
    fn test_list_functions() {
        assert!(bond_cbond_indicators().contains(&"财富"));
        assert!(bond_cbond_periods().contains(&"总值"));
    }
}
