//! Commodity option commission info from 9qihuo.com.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::parse_f64_safe;

use super::cffex_sina::strip_html_tags;

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// Commodity option symbol entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionCommSymbol {
    /// Symbol name (Chinese).
    pub name: String,
    /// Symbol code.
    pub code: String,
}

/// Commodity option commission info row.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionCommInfoRow {
    /// Contract name.
    pub contract: String,
    /// Current price.
    pub price: f64,
    /// Volume.
    pub volume: f64,
    /// Gross profit per tick.
    pub gross_profit_per_tick: f64,
    /// Net profit per tick.
    pub net_profit_per_tick: f64,
    /// Exchange name.
    pub exchange: String,
    /// Commission update time.
    pub commission_update_time: String,
    /// Price update time.
    pub price_update_time: String,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get available commodity option symbols from 9qihuo.com.
    pub async fn option_comm_symbol(&self) -> Result<Vec<OptionCommSymbol>> {
        let url = "https://www.9qihuo.com/qiquanshouxufei";
        let body = self
                        .get(url)
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Parse the inst_list div for links
        let Some(inst_pos) = body.find("id=\"inst_list\"") else {
            return Err(Error::not_found("inst_list not found on page"));
        };
        let after = &body[inst_pos..];
        let div_end = after.find("</div>").unwrap_or(after.len());
        let div_content = &after[..div_end];

        let mut symbols = Vec::new();
        let mut search = div_content;
        while let Some(a_pos) = search.find("<a ") {
            let after_a = &search[a_pos..];
            if let Some(href_end) = after_a.find('>') {
                let tag = &after_a[..href_end];
                let href = extract_attr(tag, "href").unwrap_or_default();
                let text_start = href_end + 1;
                if let Some(text_end) = after_a[text_start..].find("</a>") {
                    let text = after_a[text_start..text_start + text_end]
                        .trim()
                        .to_string();
                    // Extract code from href like "?heyue=si"
                    let code = href
                        .split('?')
                        .nth(1)
                        .and_then(|q| q.split('=').nth(1))
                        .unwrap_or("")
                        .to_string();
                    if !text.is_empty() {
                        symbols.push(OptionCommSymbol {
                            name: text,
                            code,
                        });
                    }
                }
            }
            search = &search[a_pos + 3..];
        }

        if symbols.is_empty() {
            return Err(Error::not_found("no comm symbols found"));
        }

        Ok(symbols)
    }

    /// Get commodity option commission info from 9qihuo.com.
    ///
    /// `symbol` is the Chinese name, e.g. "\u{5de5}\u{4e1a}\u{7845}\u{9009}\u{6743}".
    pub async fn option_comm_info(&self, symbol: &str) -> Result<Vec<OptionCommInfoRow>> {
        let symbols = self.option_comm_symbol().await?;
        let entry = symbols
            .iter()
            .find(|s| s.name.contains(symbol))
            .ok_or_else(|| Error::not_found(format!("comm symbol not found: {symbol}")))?;

        let url = "https://www.9qihuo.com/qiquanshouxufei";
        let body = self
                        .get(url)
            .query(&[("heyue", entry.code.as_str())])
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Extract exchange name from first row of table
        let exchange = extract_first_row_exchange(&body).unwrap_or_default();

        // Parse HTML table
        let rows = parse_comm_table(&body, &exchange);

        // Extract update times
        let (comm_time, price_time) = extract_update_times(&body);

        let mut result = Vec::with_capacity(rows.len());
        for mut row in rows {
            row.commission_update_time = comm_time.clone();
            row.price_update_time = price_time.clone();
            result.push(row);
        }

        if result.is_empty() {
            return Err(Error::not_found(format!(
                "no commission data for {symbol}"
            )));
        }

        Ok(result)
    }
}

// ---------------------------------------------------------------------------
// HTML parsing helpers
// ---------------------------------------------------------------------------

fn extract_attr(tag: &str, attr_name: &str) -> Option<String> {
    let needle = format!("{attr_name}=\"");
    let pos = tag.find(&needle)?;
    let after = &tag[pos + needle.len()..];
    let end = after.find('"')?;
    Some(after[..end].to_string())
}

fn extract_first_row_exchange(html: &str) -> Option<String> {
    // Find first <td> content which typically has the exchange name
    let table_pos = html.find("<table")?;
    let table = &html[table_pos..];
    let tr_pos = table.find("<tr")?;
    let tr = &table[tr_pos..];
    // Skip the header row, find second <tr>
    let tr_end = tr.find("</tr>")?;
    let after_header = &tr[tr_end + 5..];
    let tr2_pos = after_header.find("<tr")?;
    let tr2 = &after_header[tr2_pos..];
    let td_pos = tr2.find("<td")?;
    let td = &tr2[td_pos..];
    let text_start = td.find('>')? + 1;
    let text_end = td[text_start..].find('<')?;
    Some(td[text_start..text_start + text_end].trim().to_string())
}

fn parse_comm_table(html: &str, exchange: &str) -> Vec<OptionCommInfoRow> {
    let mut rows = Vec::new();
    let table_pos = html.find("<table").unwrap_or(0);
    let table_end = html[table_pos..]
        .find("</table>")
        .map(|i| i + table_pos)
        .unwrap_or(html.len());
    let table = &html[table_pos..table_end];

    let mut search = table;
    let mut header_skipped = false;

    while let Some(tr_pos) = search.find("<tr") {
        let after_tr = &search[tr_pos..];
        if let Some(tr_end) = after_tr.find("</tr>") {
            let tr_content = &after_tr[..tr_end];
            let cells = extract_td_cells(tr_content);

            if !header_skipped {
                header_skipped = true;
                search = &after_tr[tr_end + 5..];
                continue;
            }

            // Skip sub-header rows
            if cells.len() < 4 {
                search = &after_tr[tr_end + 5..];
                continue;
            }

            if cells.len() >= 5 {
                rows.push(OptionCommInfoRow {
                    contract: cells[0].clone(),
                    price: parse_f64_safe(&cells[1]),
                    volume: parse_f64_safe(&cells[2]),
                    gross_profit_per_tick: parse_f64_safe(&cells[3]),
                    net_profit_per_tick: parse_f64_safe(&cells[4]),
                    exchange: exchange.to_string(),
                    commission_update_time: String::new(),
                    price_update_time: String::new(),
                });
            }
            search = &after_tr[tr_end + 5..];
        } else {
            break;
        }
    }

    rows
}

fn extract_update_times(html: &str) -> (String, String) {
    // Look for the link with id="dlink" and extract preceding text
    let dlink_pos = html.find("id=\"dlink\"");
    if let Some(pos) = dlink_pos {
        let before = &html[..pos];
        // Find the text before the <a> tag
        if let Some(text_start) = before.rfind('>') {
            let text = &before[text_start + 1..];
            // Parse: "（手续费更新时间：XXX，价格更新时间：YYY。）"
            let comm_time = text
                .split('\u{ff0c}')
                .next()
                .map(|s| s.trim_start_matches("\u{ff08}\u{624b}\u{7eed}\u{8d39}\u{66f4}\u{65b0}\u{65f6}\u{95f4}\u{effd}"))
                .unwrap_or("")
                .to_string();
            let price_time = text
                .split('\u{ff0c}')
                .nth(1)
                .map(|s| {
                    s.trim_start_matches("\u{4ef7}\u{683c}\u{66f4}\u{65b0}\u{65f6}\u{95f4}\u{effd}")
                        .trim_end_matches("\u{3002}\u{ff09}")
                        .to_string()
                })
                .unwrap_or_default();
            return (comm_time, price_time);
        }
    }
    (String::new(), String::new())
}

fn extract_td_cells(tr_html: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut search = tr_html;
    while let Some(td_pos) = search.find("<td") {
        let after_td = &search[td_pos..];
        if let Some(text_start) = after_td.find('>') {
            let after_gt = &after_td[text_start + 1..];
            if let Some(td_end) = after_gt.find("</td>") {
                let text = strip_html_tags(&after_gt[..td_end]);
                cells.push(text.trim().to_string());
                search = &after_gt[td_end + 5..];
            } else {
                break;
            }
        } else {
            break;
        }
    }
    cells
}
