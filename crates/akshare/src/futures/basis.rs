//! Futures basis (基差) and spot price data from 100ppi.com (生意社).
//!
//! Fetches commodity spot prices and basis calculations.
//!
//! Upstream reference: `akshare/futures/futures_basis.py`.
//!
//! 100ppi pages are requested with an explicit HTML `Accept` header, matching
//! the upstream Python request and avoiding browser-only challenge handling.

use std::sync::LazyLock;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;
use crate::util::send_and_check_text;

static RE_TABLE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<table[^>]*>(.*?)</table>").unwrap());
static RE_TR: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<tr[^>]*>(.*?)</tr>").unwrap());
static RE_TD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?is)<td[^>]*>(.*?)</td>").unwrap());
static RE_TAGS: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"<[^>]+>").unwrap());
static RE_TRAILING_DIGITS: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(\d+)$").unwrap());

/// Inner-table marker so nested cells stay inside their owning outer cell.
const INNER_CELL_SEP: char = '\u{1e}';
const INNER_MARK: char = '\u{1f}';

/// Find top-level `<table>...</table>` blocks, honouring nested tables.
fn find_outer_tables(html: &str) -> Vec<String> {
    let lower = html.to_ascii_lowercase();
    let mut tables = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = lower[cursor..].find("<table") {
        let start = cursor + rel;
        let mut depth = 0usize;
        let mut scan = start;
        let mut end = None;
        while scan < lower.len() {
            let next_open = lower[scan..].find("<table").map(|pos| scan + pos);
            let next_close = lower[scan..].find("</table>").map(|pos| scan + pos);
            match (next_open, next_close) {
                (Some(open), Some(close)) if open < close => {
                    depth += 1;
                    scan = open + 6;
                }
                (_, Some(close)) => {
                    depth = depth.saturating_sub(1);
                    scan = close + 8;
                    if depth == 0 {
                        end = Some(scan);
                        break;
                    }
                }
                _ => break,
            }
        }
        match end {
            Some(end) => {
                tables.push(html[start..end].to_string());
                cursor = end;
            }
            None => break,
        }
    }
    tables
}

/// Replace nested tables with a marker that keeps their cell text in one cell.
fn collapse_inner_tables(content: &str) -> String {
    RE_TABLE
        .replace_all(content, |caps: &regex::Captures<'_>| {
            let inner = RE_TD
                .captures_iter(&caps[1])
                .map(|cap| strip_tags(&cap[1]))
                .collect::<Vec<_>>()
                .join(&INNER_CELL_SEP.to_string());
            format!("{INNER_MARK}{inner}{INNER_MARK}")
        })
        .into_owned()
}

fn strip_tags(value: &str) -> String {
    RE_TAGS
        .replace_all(value, " ")
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Parse all rows of one outer table into trimmed text cells.
fn parse_table_rows(table: &str) -> Vec<Vec<String>> {
    // Drop the table's own opening/closing tags before collapsing children.
    let body = table.split_once('>').map(|(_, rest)| rest).unwrap_or(table);
    let body = body.rfind("</table>").map_or(body, |pos| &body[..pos]);
    let body = collapse_inner_tables(body);
    RE_TR
        .captures_iter(&body)
        .map(|row| {
            RE_TD
                .captures_iter(&row[1])
                .map(|cell| strip_tags(&cell[1]))
                .collect::<Vec<String>>()
        })
        .filter(|row| !row.is_empty())
        .collect()
}

fn parse_number(value: &str) -> Option<f64> {
    let cleaned = value
        .trim()
        .trim_matches(|c: char| c == '\u{1f}' || c == '+' || c == ',' || c == '%')
        .trim();
    cleaned.parse::<f64>().ok()
}

fn normalize_date(date: &str) -> Result<String> {
    let digits: String = date.chars().filter(char::is_ascii_digit).collect();
    if digits.len() != 8 {
        return Err(Error::invalid_input(format!(
            "date must be YYYYMMDD or YYYY-MM-DD, got {date}"
        )));
    }
    Ok(digits)
}

fn fmt_dash(date: &str) -> String {
    format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
}

async fn fetch_100ppi(client: &AkShareClient, url: &str) -> Result<String> {
    send_and_check_text(
        client
            .get(url)
            .header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            )
            .header("User-Agent", "Mozilla/5.0"),
        None,
    )
    .await
}

fn chinese_only(value: &str) -> String {
    value
        .chars()
        .filter(|c| ('\u{4e00}'..='\u{9fa5}').contains(c))
        .collect()
}

/// Upstream `akshare.futures.symbol_var.chinese_to_english`.
fn chinese_to_english(name: &str) -> Option<&'static str> {
    let name = name.trim();
    Some(match name {
        "橡胶" | "天然橡胶" => "RU",
        "石油沥青"
        | "石油沥青(仓库)"
        | "石油沥青(厂库)"
        | "沥青"
        | "沥青仓库"
        | "沥青(仓库)"
        | "石油沥青厂库"
        | "石油沥青仓库" => "BU",
        "沥青厂库" | "沥青(厂库)" => "BU2",
        "热轧卷板"
        | "热轧卷板厂库"
        | "热轧卷板仓库"
        | "热轧卷板(厂库)"
        | "热轧卷板(仓库)"
        | "热轧板卷" => "HC",
        "燃料油" => "FU",
        "白银" => "AG",
        "线材" => "WR",
        "螺纹钢" | "螺纹钢(仓库)" | "螺纹钢(厂库)" | "螺纹钢仓库" | "螺纹钢厂库" => {
            "RB"
        }
        "铅" => "PB",
        "铜" => "CU",
        "铝" => "AL",
        "锌" => "ZN",
        "黄金" | "钯金" => "AU",
        "锡" => "SN",
        "镍" => "NI",
        "纸浆" | "纸浆(仓库)" | "纸浆(厂库)" | "纸浆仓库" | "纸浆厂库" => "SP",
        "豆一" | "大豆" => "A",
        "豆二" => "B",
        "胶合板" => "BB",
        "玉米" => "C",
        "玉米淀粉" => "CS",
        "聚乙烯" | "LLDPE" | "LDPE" => "L",
        "豆粕" => "M",
        "豆油" | "大豆油" => "Y",
        "棕榈油" => "P",
        "纤维板" => "FB",
        "鸡蛋" => "JD",
        "聚氯乙烯" | "PVC" => "V",
        "聚丙烯" | "PP" => "PP",
        "焦炭" => "J",
        "焦煤" => "JM",
        "铁矿石" => "I",
        "乙二醇" => "EG",
        "强麦" | "强筋小麦" => "WH",
        "硬冬白麦" | "普麦" | "硬白小麦" | "硬白小麦（）" => "PM",
        "皮棉" | "棉花" | "一号棉" => "CF",
        "白糖" => "SR",
        "PTA" => "TA",
        "菜籽油" | "菜油" => "OI",
        "早籼稻" => "RI",
        "早籼" => "ER",
        "甲醇" | "柴油" => "MA",
        "玻璃" => "FG",
        "油菜籽" | "菜籽" => "RS",
        "菜籽粕" | "菜粕" => "RM",
        "动力煤" => "ZC",
        "粳稻" => "JR",
        "晚籼稻" | "晚籼" => "LR",
        "硅铁" => "SF",
        "锰硅" => "SM",
        "硬麦" => "WT",
        "棉纱" => "CY",
        "苹果" => "AP",
        "原油" | "中质含硫原油" => "SC",
        "尿素" => "UR",
        "20号胶" | "20号胶20" | "20号胶(仓库)" | "20号胶(厂库)" => "NR",
        "苯乙烯" => "EB",
        "不锈钢" | "不锈钢仓库" | "不锈钢厂库" | "不锈钢(厂库)" | "不锈钢(仓库)" => {
            "SS"
        }
        "粳米" => "RR",
        "红枣" => "CJ",
        "纯碱" => "SA",
        "液化石油气" => "PG",
        "低硫燃料油"
        | "低硫燃料油仓库"
        | "低硫燃料油厂库"
        | "低硫燃料油(仓库)"
        | "低硫燃料油(厂库)" => "LU",
        "短纤" | "涤纶短纤" => "PF",
        "生猪" => "LH",
        "花生" => "PK",
        "工业硅" => "SI",
        "氧化铝" | "氧化铝仓库" | "氧化铝厂库" | "氧化铝(仓库)" | "氧化铝(厂库)" => {
            "AO"
        }
        "丁二烯橡胶"
        | "丁二烯橡胶仓库"
        | "丁二烯橡胶厂库"
        | "丁二烯橡胶(仓库)"
        | "丁二烯橡胶(厂库)" => "BR",
        "碳酸锂" => "LC",
        "烧碱" => "SH",
        "PX" => "PX",
        "原木" => "LG",
        "瓶片期货" | "瓶片" => "PR",
        "纯苯" => "BZ",
        "多晶硅" => "PS",
        "铸造铝合金" => "AD",
        "铜(BC)" => "BC",
        "胶版印刷纸(仓库)" | "胶版印刷纸(厂库)" => "OP",
        "丙烯期货" | "丙烯" => "PL",
        _ => return None,
    })
}

fn is_shfe_or_dce(prefix: &str) -> bool {
    matches!(
        prefix,
        "CU" | "AL"
            | "ZN"
            | "PB"
            | "NI"
            | "SN"
            | "AU"
            | "AG"
            | "RB"
            | "WR"
            | "HC"
            | "FU"
            | "BU"
            | "RU"
            | "SC"
            | "NR"
            | "SP"
            | "SS"
            | "LU"
            | "BC"
            | "AO"
            | "BR"
            | "EC"
            | "AD"
            | "OP"
            | "C"
            | "CS"
            | "A"
            | "B"
            | "M"
            | "Y"
            | "P"
            | "FB"
            | "BB"
            | "JD"
            | "L"
            | "V"
            | "PP"
            | "J"
            | "JM"
            | "I"
            | "EG"
            | "RR"
            | "EB"
            | "PG"
            | "LH"
            | "LG"
            | "BZ"
    )
}

fn is_czce(prefix: &str) -> bool {
    matches!(
        prefix,
        "WH" | "PM"
            | "CF"
            | "SR"
            | "TA"
            | "OI"
            | "RI"
            | "MA"
            | "ME"
            | "FG"
            | "RS"
            | "RM"
            | "ZC"
            | "JR"
            | "LR"
            | "SF"
            | "SM"
            | "WT"
            | "TC"
            | "GN"
            | "RO"
            | "ER"
            | "SRX"
            | "SRY"
            | "WSX"
            | "WSY"
            | "CY"
            | "AP"
            | "UR"
            | "CJ"
            | "SA"
            | "PK"
            | "PF"
            | "PX"
            | "SH"
            | "PR"
            | "PL"
    )
}

fn contract_symbol(code: &str, month: &str) -> String {
    let digits = RE_TRAILING_DIGITS
        .captures(month)
        .map(|caps| caps[1].to_string())
        .unwrap_or_default();
    let month = digits
        .parse::<i64>()
        .map(|v| v.to_string())
        .unwrap_or(digits);
    let mut symbol = format!("{code}{month}");
    let prefix_len = symbol.len().saturating_sub(4);
    let prefix = symbol[..prefix_len].to_string();
    if is_shfe_or_dce(&prefix) {
        symbol = symbol.to_lowercase();
    } else if is_czce(&prefix) {
        symbol = format!("{}{}", &symbol[..prefix_len], &symbol[symbol.len() - 3..]);
    }
    symbol
}

fn insert_number(row: &mut Row, key: &str, value: f64) {
    row.insert(key.to_string(), serde_json::json!(value));
}

impl AkShareClient {
    /// Futures spot price and basis data for a given date.
    ///
    /// Fetches from 100ppi.com (生意社) for commodity spot prices and basis calculations.
    /// Data available from 20110104.
    pub async fn futures_spot_price(&self, date: &str) -> Result<Vec<Row>> {
        let date = normalize_date(date)?;
        if date.as_str() < "20110104" {
            return Err(Error::invalid_input(
                "100ppi data starts at 20110104, please use a later date",
            ));
        }
        let url = format!("https://www.100ppi.com/sf/day-{}.html", fmt_dash(&date));
        let body = fetch_100ppi(self, &url).await?;
        let tables = find_outer_tables(&body);
        // Upstream uses read_html(...)[1]: the second table is `id="fdata"`.
        let table = tables
            .iter()
            .find(|table| table.contains("id=\"fdata\""))
            .or_else(|| tables.get(1))
            .ok_or_else(|| Error::decode("100ppi page has no basis table"))?;
        let rows = parse_table_rows(table);

        let mut items = Vec::new();
        for row in rows {
            if row.len() < 7 {
                continue;
            }
            let raw_name = strip_tags(&row[0]);
            let name = if chinese_only(&raw_name).is_empty() {
                raw_name.trim().to_string()
            } else {
                chinese_only(&raw_name)
            };
            if name.is_empty()
                || matches!(
                    name.as_str(),
                    "商品"
                        | "价格"
                        | "上海期货交易所"
                        | "郑州商品交易所"
                        | "大连商品交易所"
                        | "广州期货交易所"
                        | "暂无数据"
                )
            {
                continue;
            }
            let Some(code) = chinese_to_english(&name) else {
                continue;
            };
            let Some(spot_price) = parse_number(&row[1]) else {
                continue;
            };
            let Some(near_price) = parse_number(&row[3]) else {
                continue;
            };
            let Some(dom_price) = parse_number(&row[6]) else {
                continue;
            };
            let spot_price = match code {
                "JD" => spot_price * 500.0,
                "FG" => spot_price * 80.0,
                "LH" => spot_price * 1000.0,
                _ => spot_price,
            };
            if spot_price == 0.0 {
                continue;
            }
            let near_contract = contract_symbol(code, &row[2]);
            let dom_contract = contract_symbol(code, &row[5]);

            let mut item = Row::new();
            item.insert("symbol".into(), serde_json::json!(code));
            insert_number(&mut item, "spot_price", spot_price);
            item.insert("near_contract".into(), serde_json::json!(near_contract));
            insert_number(&mut item, "near_contract_price", near_price);
            item.insert("dominant_contract".into(), serde_json::json!(dom_contract));
            insert_number(&mut item, "dominant_contract_price", dom_price);
            insert_number(&mut item, "near_basis", near_price - spot_price);
            insert_number(&mut item, "dom_basis", dom_price - spot_price);
            insert_number(&mut item, "near_basis_rate", near_price / spot_price - 1.0);
            insert_number(&mut item, "dom_basis_rate", dom_price / spot_price - 1.0);
            item.insert("date".into(), serde_json::json!(date));
            items.push(item);
        }
        Ok(items)
    }

    /// Futures spot price daily range.
    ///
    /// Fetches basis data for a date range.
    pub async fn futures_spot_price_daily(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<Row>> {
        let start = normalize_date(start_date)?;
        let end = normalize_date(end_date)?;
        let start_naive = chrono::NaiveDate::parse_from_str(&start, "%Y%m%d")
            .map_err(|_| Error::invalid_input(format!("invalid start_date: {start_date}")))?;
        let end_naive = chrono::NaiveDate::parse_from_str(&end, "%Y%m%d")
            .map_err(|_| Error::invalid_input(format!("invalid end_date: {end_date}")))?;
        if end_naive < start_naive {
            return Err(Error::invalid_input(
                "end_date must not be earlier than start_date",
            ));
        }
        let mut items = Vec::new();
        let mut day = start_naive;
        while day <= end_naive {
            let date = day.format("%Y%m%d").to_string();
            // Non-trading days have no table; upstream skips them rather than
            // failing the whole range.
            if let Ok(rows) = self.futures_spot_price(&date).await {
                items.extend(rows);
            }
            day += chrono::Duration::days(1);
        }
        Ok(items)
    }

    /// Historical spot price and basis from 100ppi (sf2 format).
    pub async fn futures_spot_price_previous(&self, date: &str) -> Result<Vec<Row>> {
        let date = normalize_date(date)?;
        if date.as_str() < "20110104" {
            return Err(Error::invalid_input(
                "100ppi data starts at 20110104, please use a later date",
            ));
        }
        let url = format!("https://www.100ppi.com/sf2/day-{}.html", fmt_dash(&date));
        let body = fetch_100ppi(self, &url).await?;
        let tables = find_outer_tables(&body);
        let table = tables
            .iter()
            .find(|table| table.contains("id=\"fdata\""))
            .or_else(|| tables.get(1))
            .ok_or_else(|| Error::decode("100ppi sf2 page has no basis table"))?;
        let rows = parse_table_rows(table);

        let mut items = Vec::new();
        for row in rows {
            if row.len() < 8 {
                continue;
            }
            let name = strip_tags(&row[0]);
            if name.is_empty()
                || matches!(
                    name.as_str(),
                    "商品"
                        | "价格"
                        | "上海期货交易所"
                        | "郑州商品交易所"
                        | "大连商品交易所"
                        | "广州期货交易所"
                        | "暂无数据"
                )
            {
                continue;
            }
            let Some(spot_price) = parse_number(&row[1]) else {
                continue;
            };
            let Some(dom_price) = parse_number(&row[3]) else {
                continue;
            };
            let nested: Vec<&str> = row[4]
                .trim_matches(INNER_MARK)
                .split(INNER_CELL_SEP)
                .collect();
            if nested.len() < 2 {
                continue;
            }
            let Some(basis) = parse_number(nested[0]) else {
                continue;
            };
            let pct = nested[1].trim().trim_end_matches('%').trim().to_string();
            let Some(high) = parse_number(&row[5]) else {
                continue;
            };
            let Some(low) = parse_number(&row[6]) else {
                continue;
            };
            let Some(avg) = parse_number(&row[7]) else {
                continue;
            };

            let mut item = Row::new();
            item.insert("商品".into(), serde_json::json!(name));
            insert_number(&mut item, "现货价格", spot_price);
            item.insert(
                "主力合约代码".into(),
                serde_json::json!(strip_tags(&row[2])),
            );
            insert_number(&mut item, "主力合约价格", dom_price);
            insert_number(&mut item, "主力合约基差", basis);
            item.insert(
                "主力合约变动百分比".into(),
                serde_json::json!(pct.parse::<f64>().unwrap_or(0.0)),
            );
            insert_number(&mut item, "180日内主力基差最高", high);
            insert_number(&mut item, "180日内主力基差最低", low);
            insert_number(&mut item, "180日内主力基差平均", avg);
            items.push(item);
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nested_100ppi_table() {
        let html = r#"<table id="fdata"><tr><td>商品</td><td>价格</td><td>代码</td>
            <td>价格</td><td><table><tr><td>-586</td><td>-0.72%</td></tr></table></td>
            <td>2406</td><td>82180</td><td></td></tr>
            <tr><td><a>铜</a></td><td>81593.33&nbsp;</td><td>2405</td><td>82340</td>
            <td><table><tr><td>-746</td><td>-0.91%</td></tr></table></td>
            <td>2406</td><td>82180</td><td></td></tr></table>"#;
        let tables = find_outer_tables(html);
        assert_eq!(tables.len(), 1);
        let rows = parse_table_rows(&tables[0]);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1][0], "铜");
        assert_eq!(rows[1].len(), 8);
        assert!(rows[1][4].contains(INNER_CELL_SEP));
    }

    #[test]
    fn maps_chinese_commodity_names() {
        assert_eq!(chinese_to_english("铜"), Some("CU"));
        assert_eq!(chinese_to_english("玻璃"), Some("FG"));
        assert_eq!(chinese_to_english("不存在"), None);
    }

    #[test]
    fn builds_contract_symbols_per_exchange() {
        assert_eq!(contract_symbol("CU", "2405"), "cu2405");
        assert_eq!(contract_symbol("RB", "2410"), "rb2410");
        assert_eq!(contract_symbol("TA", "405"), "TA405");
        assert_eq!(contract_symbol("SR", "2405"), "SR405");
    }

    #[test]
    fn parses_trailing_numbers() {
        assert_eq!(parse_number("81593.33\u{a0}"), Some(81593.33));
        assert_eq!(parse_number("-0.72%"), Some(-0.72));
        assert_eq!(parse_number("abc"), None);
    }
}
