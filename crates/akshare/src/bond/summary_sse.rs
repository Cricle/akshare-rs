//! Bond summary data from SSE (上登债券信息网).
//!
//! The SSE endpoints return legacy `.xls` workbooks; they are parsed with the
//! shared calamine-based `read_spreadsheet_rows` helper.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

fn validate_date(date: &str) -> Result<String> {
    let date = date.trim();
    if date.len() != 8 || !date.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(Error::invalid_input(format!(
            "date must be YYYYMMDD, got {date}"
        )));
    }
    Ok(crate::util::fmt_date(date))
}

fn is_numeric(cell: &str) -> bool {
    !cell.trim().is_empty() && cell.trim().replace(',', "").parse::<f64>().is_ok()
}

/// Normalize a date cell that may be text, `YYYYMMDD`, or an Excel serial.
fn normalize_date(cell: &str) -> String {
    let text = cell.trim();
    if text.len() >= 10 && text.as_bytes().get(4) == Some(&b'-') {
        return text[..10].to_string();
    }
    if text.len() == 8 && text.chars().all(|ch| ch.is_ascii_digit()) {
        return format!("{}-{}-{}", &text[0..4], &text[4..6], &text[6..8]);
    }
    if let Ok(serial) = text.parse::<f64>()
        && (1.0..60000.0).contains(&serial)
    {
        let base = chrono::NaiveDate::from_ymd_opt(1899, 12, 30)
            .expect("1899-12-30 is a valid Excel epoch date");
        if let Some(date) = base.checked_add_days(chrono::Days::new(serial as u64)) {
            return date.format("%Y-%m-%d").to_string();
        }
    }
    text.to_string()
}

impl AkShareClient {
    /// Fetch the SSE cash bond market summary for `date` (YYYYMMDD).
    ///
    /// Columns (as in Python `bond_cash_summary_sse`):
    /// 债券现货, 托管只数, 托管市值, 托管面值, 数据日期.
    pub async fn bond_cash_summary(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let trade_date = validate_date(date)?;
        let url = "http://query.sse.com.cn/commonExcelDd.do";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[
                    ("sqlId", "COMMON_SSEBOND_SCSJ_SCTJ_SCGL_ZQXQSCGL_CX_L"),
                    ("TRADE_DATE", trade_date.as_str()),
                ])
                .header("Referer", "http://bond.sse.com.cn/"),
        )
        .await?;
        let bytes = resp.bytes().await.map_err(Error::from)?;
        let rows = crate::util::read_spreadsheet_rows(&bytes)?;

        let mut items = Vec::new();
        for row in rows {
            if row.len() < 5
                || row.iter().all(|cell| cell.trim().is_empty())
                || !is_numeric(&row[1])
            {
                continue;
            }
            items.push(serde_json::json!({
                "债券现货": row[0].trim(),
                "托管只数": crate::bond::parse_number(&row[1]),
                "托管市值": crate::bond::parse_number(&row[2]),
                "托管面值": crate::bond::parse_number(&row[3]),
                "数据日期": normalize_date(&row[4]),
            }));
        }

        if items.is_empty() {
            return Err(Error::not_found(format!(
                "SSE returned no cash summary rows for {date}"
            )));
        }
        Ok(items)
    }

    /// Fetch the SSE bond trading summary for `date` (YYYYMMDD).
    ///
    /// Columns (as in Python `bond_deal_summary_sse`):
    /// 债券类型, 当日成交笔数, 当日成交金额, 当年成交笔数, 当年成交金额, 数据日期.
    pub async fn bond_deal_summary(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let trade_date = validate_date(date)?;
        let url = "http://query.sse.com.cn/commonExcelDd.do";
        let resp = crate::util::send_and_check(
            self.get(url)
                .query(&[
                    ("sqlId", "COMMON_SSEBOND_SCSJ_SCTJ_SCGL_ZQCJGL_CX_L"),
                    ("TRADE_DATE", trade_date.as_str()),
                ])
                .header("Referer", "http://bond.sse.com.cn/"),
        )
        .await?;
        let bytes = resp.bytes().await.map_err(Error::from)?;
        let rows = crate::util::read_spreadsheet_rows(&bytes)?;

        let mut items = Vec::new();
        for row in rows {
            if row.len() < 6 || row.iter().all(|cell| cell.trim().is_empty()) {
                continue;
            }
            // Skip the workbook header row(s).
            if !is_numeric(&row[1]) {
                continue;
            }
            items.push(serde_json::json!({
                "债券类型": row[0].trim(),
                "当日成交笔数": crate::bond::parse_number(&row[1]),
                "当日成交金额": crate::bond::parse_number(&row[2]),
                "当年成交笔数": crate::bond::parse_number(&row[3]),
                "当年成交金额": crate::bond::parse_number(&row[4]),
                "数据日期": normalize_date(&row[5]),
            }));
        }

        if items.is_empty() {
            return Err(Error::not_found(format!(
                "SSE returned no deal summary rows for {date}"
            )));
        }
        Ok(items)
    }
}
