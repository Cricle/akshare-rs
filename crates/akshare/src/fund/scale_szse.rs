//! Fund scale data from SZSE.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::{read_spreadsheet_rows, zero_pad_code};

impl AkShareClient {
    /// Fetch SZSE fund scale daily data (Python: fund_scale_daily).
    ///
    /// `start_date` / `end_date`: format "YYYYMMDD".
    /// `symbol`: "ETF", "LOF", or "REITS".
    pub async fn fund_scale_daily(
        &self,
        start_date: &str,
        end_date: &str,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        if !valid_date(start_date) || !valid_date(end_date) {
            return Err(Error::invalid_input(
                "start_date and end_date must use YYYYMMDD",
            ));
        }
        if start_date > end_date {
            return Err(Error::invalid_input("start_date cannot exceed end_date"));
        }
        let (jjlb, referer) = match symbol {
            "ETF" => (
                "ETF",
                "https://www.szse.cn/market/fund/volume/etf/index.html",
            ),
            "LOF" => (
                "LOF",
                "https://www.szse.cn/market/fund/volume/lof/index.html",
            ),
            "REITS" => (
                "不动产基金",
                "https://www.szse.cn/market/fund/volume/reits/index.html",
            ),
            _ => return Err(Error::invalid_input("symbol must be ETF, LOF, or REITS")),
        };
        let bytes = self
            .get("https://www.szse.cn/api/report/ShowReport")
            .query(&[
                ("SHOWTYPE", "xlsx"),
                ("CATALOGID", "scsj_fund_jjgm"),
                ("TABKEY", "tab1"),
                ("txtStart", &format_date(start_date)),
                ("txtEnd", &format_date(end_date)),
                ("jjlb", jjlb),
                ("random", "0.9184251620553985"),
            ])
            .header("Referer", referer)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        let rows = read_spreadsheet_rows(&bytes)?;
        if rows.is_empty() {
            return Ok(Vec::new());
        }
        let headers = rows[0]
            .iter()
            .map(|value| value.trim().to_string())
            .collect::<Vec<_>>();
        let mut result = Vec::new();
        for row in rows.into_iter().skip(1) {
            if row.iter().all(|value| value.trim().is_empty()) {
                continue;
            }
            let mut item = serde_json::Map::new();
            for (index, header) in headers.iter().enumerate() {
                if header.is_empty() {
                    continue;
                }
                let value = row.get(index).map(String::as_str).unwrap_or("").trim();
                let key = if header == "基金规模(份)" {
                    "基金份额"
                } else {
                    header.as_str()
                };
                let json_value = if key == "基金代码" {
                    serde_json::json!(zero_pad_code(value))
                } else if key == "基金份额" {
                    serde_json::json!(value.replace(',', "").parse::<f64>().unwrap_or(0.0))
                } else {
                    serde_json::json!(value)
                };
                item.insert(key.to_string(), json_value);
            }
            result.push(serde_json::Value::Object(item));
        }
        Ok(result)
    }
}

fn valid_date(value: &str) -> bool {
    value.len() == 8
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && chrono::NaiveDate::parse_from_str(value, "%Y%m%d").is_ok()
}

fn format_date(value: &str) -> String {
    format!("{}-{}-{}", &value[..4], &value[4..6], &value[6..8])
}
