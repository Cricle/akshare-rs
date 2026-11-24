//! Trading department rankings (营业部排名) from THS.
//!
//! Upstream reference: `akshare/stock/flow/stock_lh_yybpm.py`.
//!
//! These pages are reachable with a browser User-Agent and Referer. THS only
//! requires the `hexin-v` cookie for its `data.10jqka.com.cn` ranking APIs when
//! accessed without a browser context; the `/ifmarket/lhbyyb/` HTML tables used
//! here are served without it.

use std::sync::LazyLock;

use super::types::BillboardBranchRankingEntry;
use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::{decode_text, parse_f64_safe, parse_html_tables};

static RE_ORG_CODE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"orgcode/([0-9a-zA-Z]+)/").unwrap());
static RE_ORG_LINK: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#"orgcode/([^/\"']+)[^>]*>\s*([^<]+)"#).unwrap());
static RE_PAGE_INFO: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#"page_info"[^>]*>\s*\d+\s*/\s*(\d+)"#).unwrap());

const THS_REFERER: &str = "https://data.10jqka.com.cn/market/longhu/";
const THS_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

async fn ths_html(client: &AkShareClient, url: &str) -> Result<String> {
    let bytes = client
        .get(url)
        .header("User-Agent", THS_UA)
        .header("Referer", THS_REFERER)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    Ok(decode_text(&bytes, None, Some(encoding_rs::GB18030)))
}

fn total_pages(html: &str) -> usize {
    RE_PAGE_INFO
        .captures(html)
        .and_then(|caps| caps[1].parse::<usize>().ok())
        .unwrap_or(1)
        .clamp(1, 100)
}

/// Parse one THS ranking table.
///
/// Column 0 is the rank, column 1 the department name, column 2 the metric the
/// page is sorted by; the remaining columns depend on the tab and are kept as a
/// JSON object in `extra`.
fn parse_yyb_rows(html: &str) -> Vec<BillboardBranchRankingEntry> {
    let tables = parse_html_tables(html);
    let Some(table) = tables.first() else {
        return Vec::new();
    };
    let Some(header_idx) = table
        .iter()
        .position(|row| row.iter().any(|cell| cell.contains("营业部名称")))
    else {
        return Vec::new();
    };
    let headers: Vec<String> = table[header_idx]
        .iter()
        .map(|cell| cell.trim().to_string())
        .collect();
    let mut items = Vec::new();
    for row in table.iter().skip(header_idx + 1) {
        if row.len() < 3 {
            continue;
        }
        let dept_name = row[1].trim().to_string();
        if dept_name.is_empty() || row[0].trim().is_empty() {
            continue;
        }
        let dept_code = RE_ORG_LINK
            .captures_iter(html)
            .find(|caps| caps[2].trim() == dept_name)
            .map(|caps| caps[1].to_string())
            .or_else(|| {
                RE_ORG_CODE
                    .captures(&row[1])
                    .map(|caps| caps[1].to_string())
            });
        let mut extra = serde_json::Map::new();
        for (index, value) in row.iter().enumerate().skip(3) {
            if let Some(header) = headers.get(index)
                && !header.is_empty()
            {
                extra.insert(header.clone(), serde_json::json!(value.trim()));
            }
        }
        items.push(BillboardBranchRankingEntry {
            dept_name,
            dept_code,
            metric_value: parse_f64_safe(&row[2]),
            extra: if extra.is_empty() {
                None
            } else {
                Some(serde_json::Value::Object(extra).to_string())
            },
        });
    }
    items
}

async fn fetch_yyb(
    client: &AkShareClient,
    base_url: &str,
) -> Result<Vec<BillboardBranchRankingEntry>> {
    let first = ths_html(client, &format!("{base_url}page/1/")).await?;
    let pages = total_pages(&first);
    let mut items = parse_yyb_rows(&first);
    for page in 2..=pages {
        let html = ths_html(client, &format!("{base_url}page/{page}/")).await?;
        items.extend(parse_yyb_rows(&html));
    }
    if items.is_empty() {
        return Err(Error::not_found(
            "THS billboard branch ranking returned no rows",
        ));
    }
    Ok(items)
}

impl AkShareClient {
    /// 同花顺-营业部排名-上榜次数最多
    pub async fn stock_lh_yyb_most(&self) -> Result<Vec<BillboardBranchRankingEntry>> {
        fetch_yyb(
            self,
            "https://data.10jqka.com.cn/ifmarket/lhbyyb/type/1/tab/sbcs/field/sbcs/sort/desc/",
        )
        .await
    }

    /// 同花顺-营业部排名-资金实力最强
    pub async fn stock_lh_yyb_capital(&self) -> Result<Vec<BillboardBranchRankingEntry>> {
        fetch_yyb(
            self,
            "https://data.10jqka.com.cn/ifmarket/lhbyyb/type/1/tab/zjsl/field/zgczje/sort/desc/",
        )
        .await
    }

    /// 同花顺-营业部排名-抱团操作实力
    pub async fn stock_lh_yyb_control(&self) -> Result<Vec<BillboardBranchRankingEntry>> {
        fetch_yyb(
            self,
            "https://data.10jqka.com.cn/ifmarket/lhbyyb/type/1/tab/btcz/field/xsjs/sort/desc/",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_page_info() {
        assert_eq!(total_pages(r#"<span class="page_info">1/12</span>"#), 12);
        assert_eq!(total_pages("no pagination"), 1);
    }

    #[test]
    fn parses_yyb_table_and_org_code() {
        let html = r#"<table><thead><tr>
            <th>序号</th><th>营业部名称</th><th>上榜次数</th><th>合计动用资金</th>
            </tr></thead><tbody><tr>
            <td>1</td><td><a href="http://data.10jqka.com.cn/market/lhbyyb/orgcode/a3f4/">深股通专用</a></td>
            <td>14</td><td>26.24亿</td></tr></tbody></table>"#;
        let rows = parse_yyb_rows(html);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].dept_name, "深股通专用");
        assert_eq!(rows[0].dept_code.as_deref(), Some("a3f4"));
        assert_eq!(rows[0].metric_value, 14.0);
        assert!(rows[0].extra.as_deref().unwrap_or("").contains("26.24亿"));
    }
}
