//! Option margin data from iweiai.com.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::parse_f64_safe;

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// Option margin symbol entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionMarginSymbol {
    /// Symbol name (Chinese).
    pub symbol: String,
    /// URL path.
    pub url: String,
}

/// Option margin data row.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionMarginRow {
    /// Contract code.
    pub contract: String,
    /// Settlement price.
    pub settlement_price: f64,
    /// Trading multiplier.
    pub multiplier: f64,
    /// Buyer premium.
    pub buyer_premium: f64,
    /// Seller margin.
    pub seller_margin: f64,
    /// Opening fee.
    pub open_fee: f64,
    /// Close-today fee.
    pub close_today_fee: f64,
    /// Close-yesterday fee.
    pub close_yesterday_fee: f64,
    /// Total fee (open + close today).
    pub total_fee: f64,
    /// Update time.
    pub update_time: String,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get available commodity option margin symbols.
    pub async fn option_margin_symbol(&self) -> Result<Vec<OptionMarginSymbol>> {
        let url = "https://www.iweiai.com/qiquan/yuanyou";
        let body = self
            .get(url)
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Parse links containing "qiquan" in href
        let mut symbols = Vec::new();
        let mut search = body.as_str();
        while let Some(pos) = search.find("qiquan") {
            // Find the enclosing <a> tag
            let before = &search[..pos];
            let a_start = before.rfind("<a ");

            if let Some(start) = a_start {
                let tag_area = &search[start..];
                if let Some(href_end) = tag_area.find('>') {
                    let href_tag = &tag_area[..href_end];
                    let href = extract_attr(href_tag, "href").unwrap_or_default();
                    let text_start = href_end + 1;
                    if let Some(text_end) = tag_area[text_start..].find("</a>") {
                        let text = tag_area[text_start..text_start + text_end].trim();
                        if !text.is_empty() && !href.is_empty() {
                            symbols.push(OptionMarginSymbol {
                                symbol: text.to_string(),
                                url: if href.starts_with("http") {
                                    href
                                } else {
                                    format!("https://www.iweiai.com{href}")
                                },
                            });
                        }
                    }
                }
            }
            search = &search[pos + 6..];
        }

        if symbols.is_empty() {
            return Err(Error::not_found("no margin symbols found"));
        }

        Ok(symbols)
    }

    /// Get commodity option margin data for a specific symbol.
    pub async fn option_margin(&self, symbol: &str) -> Result<Vec<OptionMarginRow>> {
        // First get the symbol list to find the URL
        let symbols = self.option_margin_symbol().await?;
        let entry = symbols
            .iter()
            .find(|s| s.symbol == symbol)
            .ok_or_else(|| Error::not_found(format!("margin symbol not found: {symbol}")))?;

        let url = &entry.url;
        let body = self
            .get(url)
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Extract update time from <small> tag
        let update_time = extract_small_text(&body).unwrap_or_default();

        // Parse HTML table
        let rows = parse_html_table(&body, &update_time);

        if rows.is_empty() {
            return Err(Error::not_found(format!("no margin data for {symbol}")));
        }

        Ok(rows)
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

fn extract_small_text(html: &str) -> Option<String> {
    let start = html.find("<small")?;
    let after = &html[start..];
    let text_start = after.find('>')? + 1;
    let text_end = after.find("</small>")?;
    let text = &after[text_start..text_end];
    // Strip prefix
    Some(
        text.trim_start_matches("\u{6700}\u{8fd1}\u{66f4}\u{65b0}\u{effd}")
            .to_string(),
    )
}

fn parse_html_table(html: &str, update_time: &str) -> Vec<OptionMarginRow> {
    // Find all <tr> rows in the first <table>
    let table_start = html.find("<table").unwrap_or(0);
    let table_end = html[table_start..]
        .find("</table>")
        .map_or(html.len(), |i| i + table_start);
    let table = &html[table_start..table_end];

    let mut rows = Vec::new();
    let mut search = table;
    let mut header_found = false;

    while let Some(tr_pos) = search.find("<tr") {
        let after_tr = &search[tr_pos..];
        if let Some(tr_end) = after_tr.find("</tr>") {
            let tr_content = &after_tr[..tr_end];
            let cells = extract_td_cells(tr_content);

            // Skip header row
            if !header_found {
                if cells.iter().any(|c| {
                    c.contains("\u{7ed3}\u{7b97}\u{4ef7}") || c.contains("\u{5408}\u{7ea6}")
                }) {
                    header_found = true;
                }
                search = &after_tr[tr_end + 5..];
                continue;
            }

            if cells.len() >= 8 {
                rows.push(OptionMarginRow {
                    contract: cells[0].clone(),
                    settlement_price: parse_f64_safe(&cells[1]),
                    multiplier: parse_f64_safe(&cells[2]),
                    buyer_premium: parse_f64_safe(&cells[3]),
                    seller_margin: parse_f64_safe(&cells[4]),
                    open_fee: parse_f64_safe(&cells[5]),
                    close_today_fee: parse_f64_safe(&cells[6]),
                    close_yesterday_fee: parse_f64_safe(&cells[7]),
                    total_fee: cells.get(8).map_or(0.0, |s| parse_f64_safe(s)),
                    update_time: update_time.to_string(),
                });
            }
            search = &after_tr[tr_end + 5..];
        } else {
            break;
        }
    }

    rows
}

fn extract_td_cells(tr_html: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut search = tr_html;
    while let Some(td_pos) = search.find("<td") {
        let after_td = &search[td_pos..];
        if let Some(text_start) = after_td.find('>') {
            let after_gt = &after_td[text_start + 1..];
            if let Some(td_end) = after_gt.find("</td>") {
                let text = after_gt[..td_end].trim();
                cells.push(text.to_string());
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
