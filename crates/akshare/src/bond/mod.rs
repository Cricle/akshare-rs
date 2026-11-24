//! Bond data: government, convertible, corporate, spot, buyback,
//! CBond indices, ChinaMoney yields, Sina/THS sources, SSE summaries,
//! CNINFO issuance, NAFMII, and Eastmoney rates.

pub mod buyback;
pub mod cb_sina;
pub mod cb_ths;
pub mod cbond_index;
pub mod china_money;
pub mod convertible;
pub mod corporate;
pub mod em_rate;
pub mod gb_sina;
pub mod government;
pub mod issue_cninfo;
pub mod jsl;
pub mod nafmii;
pub mod repo;
pub mod spot;
pub mod summary_sse;
pub mod zh_cov;
pub mod zh_sina;

// ---------------------------------------------------------------------------
// Shared, dependency-free HTML table helpers
// ---------------------------------------------------------------------------

/// Extract every `<table>...</table>` block as normalized rows of cell text.
pub(crate) fn html_tables(html: &str) -> Vec<Vec<Vec<String>>> {
    let lower = html.to_ascii_lowercase();
    let mut tables = Vec::new();
    let mut pos = 0;
    while let Some(rel) = lower[pos..].find("<table") {
        let start = pos + rel;
        let Some(gt) = lower[start..].find('>') else {
            break;
        };
        let body_start = start + gt + 1;
        let Some(end_rel) = lower[body_start..].find("</table>") else {
            break;
        };
        let body_end = body_start + end_rel;
        tables.push(html_rows(&html[body_start..body_end]));
        pos = body_end + "</table>".len();
    }
    tables
}

fn html_rows(fragment: &str) -> Vec<Vec<String>> {
    let lower = fragment.to_ascii_lowercase();
    let mut rows = Vec::new();
    let mut pos = 0;
    while let Some(rel) = lower[pos..].find("<tr") {
        let start = pos + rel;
        let Some(gt) = lower[start..].find('>') else {
            break;
        };
        let body_start = start + gt + 1;
        let body_end = lower[body_start..]
            .find("</tr>")
            .map_or(lower.len(), |offset| body_start + offset);
        let cells = split_cells(&fragment[body_start..body_end]);
        if !cells.is_empty() {
            rows.push(cells);
        }
        if body_end + 5 >= lower.len() {
            break;
        }
        pos = body_end + 5;
    }
    rows
}

fn split_cells(row: &str) -> Vec<String> {
    let lower = row.to_ascii_lowercase();
    let mut cells = Vec::new();
    let mut pos = 0;
    while let Some(rel) = lower[pos..].find('<') {
        let lt = pos + rel;
        let is_cell = lower[lt..].starts_with("<td") || lower[lt..].starts_with("<th");
        if !is_cell {
            pos = lt + 1;
            continue;
        }
        let Some(gt) = lower[lt..].find('>') else {
            break;
        };
        let content_start = lt + gt + 1;
        let mut end = lower.len();
        for pat in ["</td>", "</th>", "<td", "<th"] {
            if let Some(offset) = lower[content_start..].find(pat) {
                end = end.min(content_start + offset);
            }
        }
        cells.push(strip_html(&row[content_start..end]));
        pos = end;
    }
    cells
}

/// Remove tags, decode common entities and collapse whitespace.
pub(crate) fn strip_html(raw: &str) -> String {
    let mut without_tags = String::with_capacity(raw.len());
    let mut in_tag = false;
    let mut tag = String::new();
    for ch in raw.chars() {
        if in_tag {
            if ch == '>' {
                let name = tag.trim().to_ascii_lowercase();
                if name == "br" || name == "br/" || name == "br /" {
                    without_tags.push(' ');
                }
                tag.clear();
                in_tag = false;
            } else {
                tag.push(ch);
            }
        } else if ch == '<' {
            in_tag = true;
        } else {
            without_tags.push(ch);
        }
    }
    let decoded = decode_entities(&without_tags);
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn decode_entities(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(idx) = rest.find('&') {
        out.push_str(&rest[..idx]);
        let tail = &rest[idx..];
        let Some(semi) = tail.find(';') else {
            out.push_str(tail);
            return out;
        };
        let entity = &tail[1..semi];
        let replacement = match entity {
            "nbsp" => Some('\u{a0}'),
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" | "#39" => Some('\''),
            _ => entity
                .strip_prefix("#x")
                .or_else(|| entity.strip_prefix("#X"))
                .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                .or_else(|| {
                    entity
                        .strip_prefix('#')
                        .and_then(|dec| dec.parse::<u32>().ok())
                })
                .and_then(char::from_u32),
        };
        if let Some(ch) = replacement {
            out.push(ch);
        } else {
            out.push_str(&tail[..=semi]);
        }
        rest = &tail[semi + 1..];
    }
    out.push_str(rest);
    out
}

/// Parse a display number, tolerating thousands separators and percent signs.
pub(crate) fn parse_number(raw: &str) -> Option<f64> {
    let cleaned = raw.trim().trim_end_matches('%').replace(',', "");
    if cleaned.is_empty()
        || cleaned == "--"
        || cleaned == "---"
        || cleaned.eq_ignore_ascii_case("nan")
    {
        None
    } else {
        cleaned.parse::<f64>().ok()
    }
}
