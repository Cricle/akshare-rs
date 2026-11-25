//! Fund data: ETF, open-end, money market, graded, LOF, Xueqiu, QDII,
//! plus ranking, portfolio, rating, scale, report, announcement, AUM, etc.

pub mod announcement_em;
pub mod aum_em;
pub mod cf_em;
pub mod em;
pub mod etf;
pub mod etf_em;
pub mod etf_sina;
pub mod fee_em;
pub mod fhsp_em;
pub mod financial_em;
pub mod graded;
pub mod hk_em;
pub mod hold_em;
pub mod info_ths;
pub mod init_em;
pub mod lcx_em;
pub mod lof;
pub mod lof_ths;
pub mod manager;
pub mod money;
pub mod open;
pub mod overview_em;
pub mod portfolio_em;
pub mod position_lg;
pub mod private_fund;
pub mod qdii;
pub mod rank_em;
pub mod rating;
pub mod report_cninfo;
pub mod scale_em;
pub mod scale_sina;
pub mod scale_szse;
pub mod value_em;
pub mod xueqiu;

// ---------------------------------------------------------------------------
// Shared, dependency-free HTML table helpers
// ---------------------------------------------------------------------------

/// Extract every `<table>...</table>` block from an HTML document as rows of
/// normalized cell text.
///
/// The parser is deliberately small: it tolerates omitted `</td>`/`</th>`
/// closers (common on Eastmoney pages) and ignores rowspan/colspan, which is
/// sufficient for the fixed-layout tables used by the fund data sources.
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

/// Remove HTML tags from a fragment, decode the common entities, and collapse
/// whitespace (including `&nbsp;`) into single spaces.
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
