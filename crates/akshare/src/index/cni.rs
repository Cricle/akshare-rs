//! CNIndex (国证指数) data — index listing, history, and constituents.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CniHistPoint, CniIndexItem};
use crate::util::{read_spreadsheet_sheet_rows, zero_pad_code};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct CniListEnvelope {
    data: Option<CniListData>,
}

#[derive(Debug, Deserialize)]
struct CniListData {
    rows: Option<Vec<Vec<serde_json::Value>>>,
}

#[derive(Debug, Deserialize)]
struct CniHistEnvelope {
    data: Option<CniHistData>,
}

#[derive(Debug, Deserialize)]
struct CniHistData {
    data: Option<Vec<Vec<serde_json::Value>>>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 国证指数 — 最近交易日的所有指数.
    pub async fn index_all_cni(&self) -> Result<Vec<CniIndexItem>> {
        let response = crate::util::send_and_check(
            self.get("https://www.cnindex.com.cn/index/indexList")
                .query(&[("channelCode", "-1"), ("rows", "2000"), ("pageNum", "1")]),
        )
        .await?;

        let payload: CniListEnvelope = response.json().await.map_err(Error::from)?;
        let rows = payload.data.and_then(|d| d.rows).unwrap_or_default();

        let items: Vec<CniIndexItem> = rows
            .into_iter()
            .filter_map(|row| {
                if row.len() < 25 {
                    return None;
                }
                let code = row[2].as_str()?.to_string();
                let name = row[8].as_str().unwrap_or("").to_string();
                let sample_count = row[12].as_f64().unwrap_or(0.0);
                let close = row[13].as_f64().unwrap_or(0.0);
                let change_pct = row[14].as_f64().unwrap_or(0.0);
                let pe_ttm = row[16].as_f64().unwrap_or(0.0);
                let volume = row[18].as_f64().unwrap_or(0.0) / 100_000.0;
                let amount = row[19].as_f64().unwrap_or(0.0) / 100_000_000.0;
                let total_mv = row[20].as_f64().unwrap_or(0.0) / 100_000_000.0;
                let free_circ_mv = row[21].as_f64().unwrap_or(0.0) / 100_000_000.0;
                Some(CniIndexItem {
                    code,
                    name,
                    sample_count,
                    close,
                    change_pct,
                    pe_ttm,
                    volume,
                    amount,
                    total_mv,
                    free_circ_mv,
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("cnindex returned no index items"));
        }
        Ok(items)
    }

    /// 国证指数 — 历史行情数据.
    pub async fn index_hist_cni(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<CniHistPoint>> {
        // Format: YYYYMMDD -> YYYY-MM-DD
        let start = format_date_hyphen(start_date);
        let end = format_date_hyphen(end_date);

        let response = crate::util::send_and_check(
            self.get("http://hq.cnindex.com.cn/market/market/getIndexDailyDataWithDataFormat")
                .query(&[
                    ("indexCode", symbol),
                    ("startDate", &start),
                    ("endDate", &end),
                    ("frequency", "day"),
                ]),
        )
        .await?;

        let payload: CniHistEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload.data.and_then(|d| d.data).unwrap_or_default();

        let mut points: Vec<CniHistPoint> = data
            .into_iter()
            .filter_map(|row| {
                if row.len() < 11 {
                    return None;
                }
                let date = row[0].as_str()?.to_string();
                let high = row[2].as_f64().unwrap_or(0.0);
                let open = row[3].as_f64().unwrap_or(0.0);
                let low = row[4].as_f64().unwrap_or(0.0);
                let close = row[5].as_f64().unwrap_or(0.0);
                let change_pct_raw = row[7].as_str().unwrap_or("0");
                let change_pct = change_pct_raw
                    .trim_end_matches('%')
                    .parse::<f64>()
                    .unwrap_or(0.0)
                    / 100.0;
                let amount = row[8].as_f64().unwrap_or(0.0);
                let volume = row[9].as_f64().unwrap_or(0.0);
                Some(CniHistPoint {
                    date,
                    open,
                    high,
                    low,
                    close,
                    change_pct,
                    volume,
                    amount,
                })
            })
            .collect();

        points.sort_by(|a, b| a.date.cmp(&b.date));
        if points.is_empty() {
            return Err(Error::not_found("cnindex returned no history data"));
        }
        Ok(points)
    }

    /// 国证指数 — 样本详情 (constituents).
    ///
    /// Returns a JSON array of objects with fields: `date`, `code`, `name`,
    /// `industry`, `total_mv`, `weight`.
    ///
    /// Upstream `index_detail_cni` downloads an XLSX workbook (despite the
    /// endpoint name). The workbook header row is `日期 / 样本代码 / 样本简称 /
    /// 所属行业 / 总市值(亿元) / 权重（%）`. JSON is passed through unchanged
    /// when the server happens to return it.
    pub async fn index_detail_cni(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let response = crate::util::send_and_check(
            self.get("https://www.cnindex.com.cn/sample-detail/download-history")
                .query(&[("indexcode", symbol)]),
        )
        .await?;

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let bytes = response.bytes().await.map_err(Error::from)?;

        if content_type.contains("json") {
            let data: Vec<serde_json::Value> =
                serde_json::from_slice(&bytes).map_err(Error::from)?;
            Ok(data)
        } else {
            let rows = read_spreadsheet_sheet_rows(&bytes, None)?;
            let items = cni_detail_rows_to_json(&rows);
            if items.is_empty() {
                return Err(Error::not_found(
                    "cnindex detail workbook returned no constituent rows",
                ));
            }
            Ok(items)
        }
    }

    /// 国证指数 — 详细历史行情数据 (with date range).
    ///
    /// Same as `index_hist_cni` but with the Python-compatible name.
    pub async fn index_detail_hist_cni(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<CniHistPoint>> {
        self.index_hist_cni(symbol, start_date, end_date).await
    }

    /// 国证指数 — 历史调样.
    ///
    /// Returns a JSON array of objects with fields: `start_date`, `end_date`,
    /// `code`, `name`, `industry`, `adjust_type`.
    ///
    /// The upstream endpoint returns an XLSX workbook whose header row is
    /// `开始日期 / 结束日期 / 样本代码 / 样本简称 / 所属行业 / 调整类型`.
    pub async fn index_detail_hist_adjust_cni(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let response = crate::util::send_and_check(
            self.get("https://www.cnindex.com.cn/sample-detail/download-adjustment")
                .query(&[("indexcode", symbol)]),
        )
        .await?;

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let bytes = response.bytes().await.map_err(Error::from)?;

        if content_type.contains("json") {
            let data: Vec<serde_json::Value> =
                serde_json::from_slice(&bytes).map_err(Error::from)?;
            Ok(data)
        } else {
            let rows = read_spreadsheet_sheet_rows(&bytes, None)?;
            let items = cni_adjust_rows_to_json(&rows);
            if items.is_empty() {
                return Err(Error::not_found(
                    "cnindex adjustment workbook returned no adjustment rows",
                ));
            }
            Ok(items)
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn format_date_hyphen(d: &str) -> String {
    let d = d.trim();
    if d.len() == 8 && d.chars().all(|c| c.is_ascii_digit()) {
        format!("{}-{}-{}", &d[..4], &d[4..6], &d[6..8])
    } else {
        d.to_string()
    }
}

/// Collapse whitespace (including the full-width space) in workbook headers.
fn normalize_header(value: &str) -> String {
    value
        .replace('\u{3000}', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Locate the header row of a CNIndex detail workbook.
fn find_header_row(rows: &[Vec<String>]) -> Option<usize> {
    rows.iter().take(20).position(|row| {
        row.iter().any(|cell| {
            let header = normalize_header(cell);
            header.contains("样本代码") || header.contains("开始日期") || header == "日期"
        })
    })
}

/// Find a column whose header contains every `needle` and none of `exclude`.
fn find_column(headers: &[String], needles: &[&str], exclude: &[&str]) -> Option<usize> {
    headers.iter().position(|cell| {
        let header = normalize_header(cell);
        needles.iter().all(|needle| header.contains(needle))
            && !exclude.iter().any(|skip| header.contains(skip))
    })
}

fn cell_at(row: &[String], index: Option<usize>) -> String {
    index
        .and_then(|index| row.get(index))
        .map(|value| value.trim().to_string())
        .unwrap_or_default()
}

fn json_number(raw: &str) -> serde_json::Value {
    let cleaned = raw
        .trim()
        .trim_end_matches(['%', '％'])
        .trim()
        .replace(',', "");
    cleaned
        .parse::<f64>()
        .ok()
        .and_then(serde_json::Number::from_f64)
        .map_or_else(
            || serde_json::Value::String(raw.trim().to_string()),
            serde_json::Value::Number,
        )
}

/// Map a CNIndex `download-history` workbook to `{date, code, name, industry,
/// total_mv, weight}` objects.
fn cni_detail_rows_to_json(rows: &[Vec<String>]) -> Vec<serde_json::Value> {
    let Some(header_index) = find_header_row(rows) else {
        return Vec::new();
    };
    let headers = &rows[header_index];
    let date_col = find_column(headers, &["日期"], &["开始", "结束"]);
    let code_col = find_column(headers, &["样本代码"], &[]);
    let name_col = find_column(headers, &["样本简称"], &[]);
    let industry_col = find_column(headers, &["所属行业"], &[]);
    let total_mv_col = find_column(headers, &["总市值"], &[]);
    let weight_col = find_column(headers, &["权重"], &[]);

    rows[header_index + 1..]
        .iter()
        .filter_map(|row| {
            let date = cell_at(row, date_col);
            let raw_code = cell_at(row, code_col);
            if date.is_empty() && raw_code.is_empty() {
                return None;
            }
            let code = zero_pad_code(&raw_code);
            let code = if code.is_empty() { raw_code } else { code };
            let mut object = serde_json::Map::new();
            object.insert("date".to_string(), serde_json::Value::String(date));
            object.insert("code".to_string(), serde_json::Value::String(code));
            object.insert(
                "name".to_string(),
                serde_json::Value::String(cell_at(row, name_col)),
            );
            object.insert(
                "industry".to_string(),
                serde_json::Value::String(cell_at(row, industry_col)),
            );
            object.insert(
                "total_mv".to_string(),
                json_number(&cell_at(row, total_mv_col)),
            );
            object.insert("weight".to_string(), json_number(&cell_at(row, weight_col)));
            Some(serde_json::Value::Object(object))
        })
        .collect()
}

/// Map a CNIndex `download-adjustment` workbook to `{start_date, end_date,
/// code, name, industry, adjust_type}` objects.
fn cni_adjust_rows_to_json(rows: &[Vec<String>]) -> Vec<serde_json::Value> {
    let Some(header_index) = find_header_row(rows) else {
        return Vec::new();
    };
    let headers = &rows[header_index];
    let start_col = find_column(headers, &["开始日期"], &[]);
    let end_col = find_column(headers, &["结束日期"], &[]);
    let code_col = find_column(headers, &["样本代码"], &[]);
    let name_col = find_column(headers, &["样本简称"], &[]);
    let industry_col = find_column(headers, &["所属行业"], &[]);
    let adjust_col = find_column(headers, &["调整类型"], &[]);

    rows[header_index + 1..]
        .iter()
        .filter_map(|row| {
            let raw_code = cell_at(row, code_col);
            let start_date = cell_at(row, start_col);
            if raw_code.is_empty() && start_date.is_empty() {
                return None;
            }
            let code = zero_pad_code(&raw_code);
            let code = if code.is_empty() { raw_code } else { code };
            let mut object = serde_json::Map::new();
            object.insert(
                "start_date".to_string(),
                serde_json::Value::String(start_date),
            );
            object.insert(
                "end_date".to_string(),
                serde_json::Value::String(cell_at(row, end_col)),
            );
            object.insert("code".to_string(), serde_json::Value::String(code));
            object.insert(
                "name".to_string(),
                serde_json::Value::String(cell_at(row, name_col)),
            );
            object.insert(
                "industry".to_string(),
                serde_json::Value::String(cell_at(row, industry_col)),
            );
            object.insert(
                "adjust_type".to_string(),
                serde_json::Value::String(cell_at(row, adjust_col)),
            );
            Some(serde_json::Value::Object(object))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date_hyphen() {
        assert_eq!(format_date_hyphen("20230114"), "2023-01-14");
        assert_eq!(format_date_hyphen("2023-01-14"), "2023-01-14");
    }

    #[test]
    fn detail_rows_map_to_json_objects() {
        let rows = vec![
            vec![
                "日期".to_string(),
                "样本代码".to_string(),
                "样本简称".to_string(),
                "所属行业".to_string(),
                "总市值(亿元)".to_string(),
                "权重（%）".to_string(),
            ],
            vec![
                "2026-10-30".to_string(),
                "937".to_string(),
                "冀中能源".to_string(),
                "能源".to_string(),
                "166.36".to_string(),
                "0.04".to_string(),
            ],
            vec![
                "2026-10-30".to_string(),
                "600000".to_string(),
                "浦发银行".to_string(),
                "金融".to_string(),
                "1,234.5".to_string(),
                "1.2".to_string(),
            ],
        ];

        let items = cni_detail_rows_to_json(&rows);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0]["date"], "2026-10-30");
        assert_eq!(items[0]["code"], "000937");
        assert_eq!(items[0]["name"], "冀中能源");
        assert_eq!(items[0]["industry"], "能源");
        assert_eq!(items[0]["total_mv"], 166.36);
        assert_eq!(items[0]["weight"], 0.04);
        assert_eq!(items[1]["code"], "600000");
        assert_eq!(items[1]["total_mv"], 1234.5);
    }

    #[test]
    fn adjust_rows_map_to_json_objects() {
        let rows = vec![
            vec![
                "开始日期".to_string(),
                "结束日期".to_string(),
                "样本代码".to_string(),
                "样本简称".to_string(),
                "所属行业".to_string(),
                "调整类型".to_string(),
            ],
            vec![
                "2026-06-15".to_string(),
                "2026-12-11".to_string(),
                "000937".to_string(),
                "冀中能源".to_string(),
                "能源".to_string(),
                "OLD".to_string(),
            ],
        ];

        let items = cni_adjust_rows_to_json(&rows);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["start_date"], "2026-06-15");
        assert_eq!(items[0]["end_date"], "2026-12-11");
        assert_eq!(items[0]["code"], "000937");
        assert_eq!(items[0]["name"], "冀中能源");
        assert_eq!(items[0]["industry"], "能源");
        assert_eq!(items[0]["adjust_type"], "OLD");
    }

    #[test]
    fn headerless_workbook_returns_no_rows() {
        let rows = vec![vec!["foo".to_string(), "bar".to_string()]];
        assert!(cni_detail_rows_to_json(&rows).is_empty());
        assert!(cni_adjust_rows_to_json(&rows).is_empty());
    }
}
