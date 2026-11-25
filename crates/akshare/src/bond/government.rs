//! China government bond yield data from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::BondSnapshot;
use crate::util::{parse_f64_safe, parse_html_tables, send_and_check_text};

fn chinabond_date(date: &str) -> Result<String> {
    let digits = date
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>();
    if digits.len() != 8 || chrono::NaiveDate::parse_from_str(&digits, "%Y%m%d").is_err() {
        return Err(Error::invalid_input("date must be YYYYMMDD or YYYY-MM-DD"));
    }
    Ok(format!(
        "{}-{}-{}",
        &digits[..4],
        &digits[4..6],
        &digits[6..]
    ))
}

impl AkShareClient {
    /// Fetch China government bond yield curve data.
    ///
    /// The Eastmoney datacenter report `RPT_BOND_GOV_CN_YIELD` has been retired.
    /// The original Python akshare uses ChinaBond website which requires JS rendering.
    /// Returns an error indicating the data source is unavailable.
    pub async fn bond_china_yield(&self, start: &str, end: &str) -> Result<Vec<BondSnapshot>> {
        let start = chinabond_date(start)?;
        let end = chinabond_date(end)?;
        let url = "https://yield.chinabond.com.cn/cbweb-pbc-web/pbc/historyQuery";
        let body = send_and_check_text(
            self.get(url)
                .query(&[
                    ("startDate", start),
                    ("endDate", end),
                    ("gjqx", "0".to_string()),
                    ("qxId", "ycqx".to_string()),
                    ("locale", "cn_ZH".to_string()),
                ])
                .header("User-Agent", "Mozilla/5.0")
                .header("Referer", "https://yield.chinabond.com.cn/"),
            Some(encoding_rs::GB18030),
        )
        .await?;
        let table = parse_html_tables(&body)
            .into_iter()
            .find(|table| {
                table
                    .iter()
                    .any(|row| row.iter().any(|cell| cell.trim() == "日期"))
            })
            .ok_or_else(|| Error::decode("ChinaBond yield table not found"))?;
        let header_index = table
            .iter()
            .position(|row| row.iter().any(|cell| cell.trim() == "日期"))
            .ok_or_else(|| Error::decode("ChinaBond yield header not found"))?;
        let headers = &table[header_index];
        let date_index = headers
            .iter()
            .position(|cell| cell.trim() == "日期")
            .unwrap_or(0);
        let tenors = ["3月", "6月", "1年", "3年", "5年", "7年", "10年", "30年"];
        let mut result = Vec::new();
        for row in table.iter().skip(header_index + 1) {
            let date = row.get(date_index).map(|value| value.trim()).unwrap_or("");
            if date.is_empty() {
                continue;
            }
            for tenor in tenors {
                let Some(index) = headers.iter().position(|header| header.trim() == tenor) else {
                    continue;
                };
                let Some(value) = row.get(index).and_then(|cell| {
                    let value = cell.replace('&', "").replace("nbsp", "");
                    let value = value.trim();
                    (!value.is_empty() && value != "-").then(|| parse_f64_safe(value))
                }) else {
                    continue;
                };
                result.push(BondSnapshot {
                    symbol: tenor.to_string(),
                    name: "中国债券收益率曲线".to_string(),
                    date: date.to_string(),
                    close: value,
                    change_pct: 0.0,
                    yield_rate: Some(value),
                    credit_rating: None,
                });
            }
        }
        if result.is_empty() {
            return Err(Error::not_found("ChinaBond yield returned no rows"));
        }
        Ok(result)
    }
}
