//! Money market fund data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FundMoneyRankItem, FundNavHistory, FundSnapshot};
use serde::Deserialize;
use serde_json::{Value, json};

fn is_date_cell(value: &str) -> bool {
    let trimmed = value.trim();
    let bytes = trimmed.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
}

/// Extract a balanced `[...]` literal that follows `key` in a JavaScript blob.
fn extract_js_array<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let key_pos = text.find(key)?;
    let after_key = &text[key_pos + key.len()..];
    let start = after_key.find('[')?;
    let bytes = after_key.as_bytes();
    let mut depth = 0_i32;
    let mut quote: Option<u8> = None;
    let mut escaped = false;
    for (index, byte) in bytes.iter().enumerate().skip(start) {
        if let Some(active) = quote {
            if escaped {
                escaped = false;
            } else if *byte == b'\\' {
                escaped = true;
            } else if *byte == active {
                quote = None;
            }
            continue;
        }
        match *byte {
            b'"' | b'\'' => quote = Some(*byte),
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&after_key[start..=index]);
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_js_string_array(text: &str, keys: &[&str]) -> Vec<String> {
    for key in keys {
        let Some(raw) = extract_js_array(text, key) else {
            continue;
        };
        if let Ok(Value::Array(items)) = serde_json::from_str::<Value>(raw) {
            let values: Vec<String> = items
                .into_iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect();
            if !values.is_empty() {
                return values;
            }
        }
    }
    Vec::new()
}

fn money_column_names(dates: &[String]) -> Vec<String> {
    if dates.len() >= 6 {
        vec![
            "基金代码".to_string(),
            "基金简称".to_string(),
            format!("{}-万份收益", dates[0]),
            format!("{}-7日年化%", dates[1]),
            format!("{}-单位净值", dates[2]),
            format!("{}-万份收益", dates[3]),
            format!("{}-7日年化%", dates[4]),
            format!("{}-单位净值", dates[5]),
            "日涨幅".to_string(),
            "成立日期".to_string(),
            "基金经理".to_string(),
            "手续费".to_string(),
            "可购全部".to_string(),
        ]
    } else {
        vec![
            "基金代码".to_string(),
            "基金简称".to_string(),
            "万份收益".to_string(),
            "7日年化%".to_string(),
            "单位净值".to_string(),
            "万份收益-前一日".to_string(),
            "7日年化%-前一日".to_string(),
            "单位净值-前一日".to_string(),
            "日涨幅".to_string(),
            "成立日期".to_string(),
            "基金经理".to_string(),
            "手续费".to_string(),
            "可购全部".to_string(),
        ]
    }
}

fn clean_fund_name(name: &str) -> String {
    name.replace("基金吧档案", "")
        .replace("基金吧", "")
        .trim()
        .to_string()
}

fn money_rows_from_datas(datas: &[Value], names: &[String]) -> Vec<Value> {
    let mut result = Vec::new();
    for item in datas {
        if let Some(object) = item.as_object() {
            let mut row = serde_json::Map::new();
            for name in names {
                if let Some(value) = object.get(name) {
                    row.insert(name.clone(), value.clone());
                }
            }
            if !row.is_empty() {
                result.push(Value::Object(row));
            }
            continue;
        }
        let Some(values) = item.as_array() else {
            continue;
        };
        let mut offset = 0_usize;
        if values.len() > names.len()
            && values
                .first()
                .and_then(Value::as_i64)
                .is_some_and(|rank| rank > 0)
            && values
                .get(1)
                .and_then(Value::as_str)
                .is_some_and(|code| code.chars().all(|ch| ch.is_ascii_digit()))
        {
            offset = 1;
        }
        let mut row = serde_json::Map::new();
        for (index, name) in names.iter().enumerate() {
            let value = values.get(index + offset).cloned().unwrap_or(Value::Null);
            row.insert(name.clone(), value);
        }
        if let Some(name) = row.get_mut("基金简称")
            && let Some(text) = name.as_str()
        {
            *name = Value::String(clean_fund_name(text));
        }
        if !row.is_empty() {
            result.push(Value::Object(row));
        }
    }
    result
}

#[derive(Debug, Deserialize)]
struct HbRankEnvelope {
    #[serde(rename = "Datas")]
    datas: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ErrCode")]
    err_code: Option<i64>,
    #[serde(rename = "ErrMsg")]
    err_msg: Option<String>,
}

impl AkShareClient {
    /// Fetch money market fund rankings from Eastmoney.
    pub async fn fund_money_market(&self, limit: usize) -> Result<Vec<FundSnapshot>> {
        let pn = limit.max(1).to_string();
        let response = crate::util::send_and_check(
            self.get("https://api.fund.eastmoney.com/FundRank/GetHbRankList")
                .query(&[
                    ("FundType", "0"),
                    ("SortColumn", "SYL_7"),
                    ("Sort", "desc"),
                    ("pageIndex", "1"),
                    ("pageSize", pn.as_str()),
                    ("IsSale", "1"),
                ])
                .header("Referer", "https://fund.eastmoney.com/"),
        )
        .await?;

        let payload: HbRankEnvelope = response.json().await.map_err(Error::from)?;
        if let Some(code) = payload.err_code
            && code != 0
        {
            let msg = payload.err_msg.unwrap_or_else(|| "unknown".to_string());
            return Err(Error::upstream(format!(
                "money fund API error {code}: {msg}"
            )));
        }

        let items = payload.datas.unwrap_or_default();
        let snapshots: Vec<FundSnapshot> = items
            .into_iter()
            .filter_map(|v| {
                let symbol = v.get("FCODE")?.as_str()?.to_string();
                let name = v
                    .get("SHORTNAME")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string();
                let date = v
                    .get("PDATE")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string();
                Some(FundSnapshot {
                    symbol,
                    name,
                    date,
                    nav: v
                        .get("DWJZ")
                        .and_then(|x| x.as_str())
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0.0),
                    acc_nav: v
                        .get("LJJZ")
                        .and_then(|x| x.as_str())
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0.0),
                    change_pct: v
                        .get("RZDF")
                        .and_then(|x| x.as_str())
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0.0),
                    fund_type: Some("money_market".to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found("no money market fund data"));
        }
        Ok(snapshots)
    }

    /// Fetch money fund daily data (Python: `fund_money_fund_daily`).
    ///
    /// The page historically embedded a `rankData` JavaScript object; current
    /// deployments render the same data into the `#oTable` HTML table.  Both
    /// layouts are supported.
    pub async fn fund_money_fund_daily(&self) -> Result<Vec<serde_json::Value>> {
        let text = crate::util::send_and_check_text(
            self.get("https://fund.eastmoney.com/HBJJ_pjsyl.html"),
            Some(encoding_rs::GB18030),
        )
        .await?;

        // Preferred layout: `var rankData = {datas: [...], ...};`.
        if text.contains("rankData")
            && let Some(raw_datas) = extract_js_array(&text, "datas")
            && let Ok(Value::Array(datas)) = serde_json::from_str::<Value>(raw_datas)
            && !datas.is_empty()
        {
            let mut dates = parse_js_string_array(&text, &["showDay", "showDayArr", "dates"]);
            if dates.is_empty() {
                dates = parse_js_string_array(&text, &["dateArr", "day"]);
            }
            let names = money_column_names(&dates);
            let rows = money_rows_from_datas(&datas, &names);
            if !rows.is_empty() {
                return Ok(rows);
            }
        }

        // Fallback: parse the HTML table embedded in the page.
        let tables = crate::util::parse_html_tables(&text);
        for table in tables {
            let Some(header_index) = table
                .iter()
                .position(|row| row.iter().any(|cell| cell.trim().contains("基金代码")))
            else {
                continue;
            };
            let header = &table[header_index];
            let code_index = header
                .iter()
                .position(|cell| cell.trim().contains("基金代码"));
            let name_index = header
                .iter()
                .position(|cell| cell.trim().contains("基金简称"));
            let (Some(code_index), Some(name_index)) = (code_index, name_index) else {
                continue;
            };

            let dates: Vec<String> = header
                .iter()
                .filter(|cell| is_date_cell(cell))
                .map(|cell| cell.trim().to_string())
                .collect();
            let names = money_column_names(&dates);
            let base = code_index.min(name_index);

            let mut rows = Vec::new();
            for row in table.iter().skip(header_index + 1) {
                if row.iter().all(|cell| cell.trim().is_empty()) {
                    continue;
                }
                // Skip the second header row (万份收益 / 7日年化% / 单位净值 …).
                if row.iter().any(|cell| {
                    let value = cell.trim();
                    value == "万份收益" || value == "7日年化%" || value == "单位净值"
                }) {
                    continue;
                }
                let code = row.get(code_index).map(String::as_str).unwrap_or("");
                let name = row.get(name_index).map(String::as_str).unwrap_or("");
                if code.trim().is_empty() && name.trim().is_empty() {
                    continue;
                }

                let mut object = serde_json::Map::new();
                object.insert("基金代码".to_string(), json!(code.trim()));
                object.insert("基金简称".to_string(), json!(clean_fund_name(name)));
                for (offset, column) in names.iter().enumerate().skip(2) {
                    let value = row.get(base + offset).map(|cell| cell.trim()).unwrap_or("");
                    object.insert(column.clone(), json!(value));
                }
                rows.push(Value::Object(object));
            }
            if !rows.is_empty() {
                return Ok(rows);
            }
        }

        Err(Error::not_found("no money fund daily data"))
    }

    /// Fetch money fund info (historical NAV) from Eastmoney.
    ///
    /// `symbol`: money fund code (e.g. "000009").
    pub async fn fund_money_fund_info(&self, symbol: &str) -> Result<Vec<FundNavHistory>> {
        let response = crate::util::send_and_check(
            self.get("https://api.fund.eastmoney.com/f10/lsjz")
                .header(
                    "Referer",
                    format!("https://fundf10.eastmoney.com/jjjz_{symbol}.html"),
                )
                .query(&[
                    ("fundCode", symbol),
                    ("pageIndex", "1"),
                    ("pageSize", "10000"),
                    ("startDate", ""),
                    ("endDate", ""),
                ]),
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let list = payload
            .get("Data")
            .and_then(|d| d.get("LSJZList"))
            .and_then(|l| l.as_array())
            .ok_or_else(|| Error::not_found(format!("no money fund data for {symbol}")))?;

        let mut result = Vec::new();
        for item in list {
            let Some(arr) = item.as_array() else { continue };
            if arr.len() < 10 {
                continue;
            }
            result.push(FundNavHistory {
                date: arr[0].as_str().unwrap_or("").to_string(),
                nav: arr[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                acc_nav: arr[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                change_pct: 0.0,
                subscribe_status: arr[7].as_str().unwrap_or("").to_string(),
                redeem_status: arr[8].as_str().unwrap_or("").to_string(),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found(format!("no money fund data for {symbol}")));
        }
        result.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(result)
    }

    /// Fetch money fund ranking (Python: fund_money_rank).
    pub async fn fund_money_rank(&self) -> Result<Vec<FundMoneyRankItem>> {
        let response = crate::util::send_and_check(
            self.get("https://api.fund.eastmoney.com/FundRank/GetHbRankList")
                .query(&[
                    ("intCompany", "0"),
                    ("MinsgType", ""),
                    ("IsSale", "1"),
                    ("strSortCol", "SYL_1N"),
                    ("orderType", "desc"),
                    ("pageIndex", "1"),
                    ("pageSize", "10000"),
                ])
                .header("Referer", "https://fund.eastmoney.com/fundguzhi.html"),
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("Data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| Error::not_found("no money fund rank data"))?;

        let mut result = Vec::new();
        for (i, item) in data.iter().enumerate() {
            let Some(arr) = item.as_array() else { continue };
            if arr.len() < 20 {
                continue;
            }
            result.push(FundMoneyRankItem {
                rank: (i + 1) as i32,
                fund_code: arr[7].as_str().unwrap_or("").to_string(),
                fund_name: arr[8].as_str().unwrap_or("").to_string(),
                date: arr[9].as_str().unwrap_or("").to_string(),
                yield_per_10k: arr[10].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                annualized_7d: arr[11].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                annualized_14d: arr[13].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                annualized_28d: arr[14].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                month_1: arr[15].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                month_3: arr[16].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                month_6: arr[17].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_1: arr[0].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_2: arr[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_3: arr[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                year_5: arr[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                ytd: arr[18].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                since_found: arr[19].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found("no money fund rank data"));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_hb_rank_envelope_empty() {
        let json = r#"{"Datas": [], "ErrCode": 0}"#;
        let env: HbRankEnvelope = serde_json::from_str(json).unwrap();
        assert!(env.datas.unwrap().is_empty());
    }
}
