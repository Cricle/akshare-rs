//! Fund fee data from Eastmoney.

use regex::Regex;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

fn section_tables(html: &str) -> Vec<(String, Vec<Vec<Vec<String>>>)> {
    let re =
        Regex::new(r#"(?is)<h4[^>]*class="t"[^>]*>(.*?)</h4>"#).expect("static h4 regex is valid");
    let matches: Vec<_> = re.captures_iter(html).collect();
    let mut sections = Vec::new();
    for (index, capture) in matches.iter().enumerate() {
        let Some(whole) = capture.get(0) else {
            continue;
        };
        let title = capture
            .get(1)
            .map(|m| crate::fund::strip_html(m.as_str()))
            .unwrap_or_default();
        let title = title
            .split('◆')
            .next()
            .unwrap_or_default()
            .trim()
            .to_string();
        if title.is_empty() {
            continue;
        }
        let end = matches
            .get(index + 1)
            .and_then(|next| next.get(0))
            .map_or(html.len(), |m| m.start());
        let segment = &html[whole.end()..end];
        sections.push((title, crate::fund::html_tables(segment)));
    }
    sections
}

fn rows_to_objects(rows: &[Vec<String>]) -> Vec<serde_json::Map<String, serde_json::Value>> {
    let Some(headers) = rows.first() else {
        return Vec::new();
    };
    let mut keys = Vec::with_capacity(headers.len());
    for (index, header) in headers.iter().enumerate() {
        let base = if header.is_empty() {
            format!("column_{index}")
        } else {
            header.clone()
        };
        let mut key = base.clone();
        let mut suffix = 2;
        while keys.contains(&key) {
            key = format!("{base}_{suffix}");
            suffix += 1;
        }
        keys.push(key);
    }

    let mut objects = Vec::new();
    for row in rows.iter().skip(1) {
        if row.iter().all(String::is_empty) {
            continue;
        }
        let mut object = serde_json::Map::new();
        for (index, key) in keys.iter().enumerate() {
            let value = row.get(index).cloned().unwrap_or_default();
            object.insert(key.clone(), serde_json::Value::String(value));
        }
        objects.push(object);
    }
    objects
}

fn split_column(
    object: &mut serde_json::Map<String, serde_json::Value>,
    column: &str,
    targets: &[&str],
) {
    let Some(value) = object.remove(column) else {
        return;
    };
    let text = value.as_str().unwrap_or_default();
    let parts: Vec<&str> = text.split('|').map(str::trim).collect();
    let original = parts.first().copied().unwrap_or_default().to_string();
    for target in targets {
        let piece = if *target == targets[0] {
            original.clone()
        } else {
            parts
                .get(targets.iter().position(|t| t == target).unwrap_or(0))
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
                .unwrap_or_else(|| original.clone())
        };
        object.insert((*target).to_string(), serde_json::Value::String(piece));
    }
}

fn apply_indicator_transform(
    indicator: &str,
    rows: &mut [serde_json::Map<String, serde_json::Value>],
) {
    match indicator {
        "认购费率（前端）" => {
            for row in rows.iter_mut() {
                split_column(
                    row,
                    "原费率|天天基金优惠费率",
                    &["原费率", "天天基金优惠费率"],
                );
            }
            if let Some(row) = rows.get_mut(3)
                && let Some(original) = row.get("原费率").cloned()
            {
                row.insert("天天基金优惠费率".to_string(), original);
            }
        }
        "申购费率（前端）" => {
            for row in rows.iter_mut() {
                split_column(
                    row,
                    "原费率|天天基金优惠费率 银行卡购买|活期宝购买",
                    &[
                        "原费率",
                        "天天基金优惠费率-银行卡购买",
                        "天天基金优惠费率-活期宝购买",
                    ],
                );
            }
        }
        indicator if indicator.starts_with("赎回费率") => {
            for row in rows.iter_mut() {
                split_column(
                    row,
                    "原费率|天天基金优惠费率",
                    &["原费率", "天天基金优惠费率"],
                );
            }
        }
        _ => {}
    }
}

impl AkShareClient {
    /// Fetch fund purchase-fee rules from Eastmoney (Python: `fund_fee_em`).
    ///
    /// `symbol`: fund code, e.g. "015641".
    /// `indicator`: a section title exactly as published on
    /// `https://fundf10.eastmoney.com/jjfl_{symbol}.html`, for example
    /// "交易状态", "申购与赎回金额", "交易确认日", "运作费用", "认购费率",
    /// "申购费率" or "赎回费率".
    pub async fn fund_fee_em(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let symbol = symbol.trim();
        if symbol.is_empty() {
            return Err(Error::invalid_input("fund code must not be empty"));
        }
        let url = format!("https://fundf10.eastmoney.com/jjfl_{symbol}.html");
        let text =
            crate::util::send_and_check_text(self.get(&url), Some(encoding_rs::GB18030)).await?;

        let sections = section_tables(&text);
        if sections.is_empty() {
            return Err(Error::not_found(format!(
                "no fee sections found for fund {symbol}"
            )));
        }

        let Some((_, tables)) = sections.iter().find(|(title, _)| title == indicator) else {
            let available = sections
                .iter()
                .map(|(title, _)| title.as_str())
                .collect::<Vec<_>>()
                .join("、");
            return Err(Error::invalid_input(format!(
                "fund {symbol} has no \"{indicator}\" section; available indicators: {available}"
            )));
        };

        let mut objects = Vec::new();
        if indicator == "申购与赎回金额" {
            for table in tables {
                objects.extend(rows_to_objects(table));
            }
        } else if let Some(table) = tables.first() {
            objects = rows_to_objects(table);
        }

        if objects.is_empty() {
            return Err(Error::not_found(format!(
                "no rows in \"{indicator}\" section for fund {symbol}"
            )));
        }
        apply_indicator_transform(indicator, &mut objects);
        Ok(objects.into_iter().map(serde_json::Value::Object).collect())
    }

    /// Legacy placeholder: the upstream API requires a fund code and indicator.
    ///
    /// Use [`fund_fee_em`](Self::fund_fee_em) instead.
    pub async fn fund_fee(&self, _limit: usize) -> Result<Vec<serde_json::Value>> {
        Err(Error::invalid_input(
            "fund_fee requires a fund code and indicator; use fund_fee_em(symbol, indicator)",
        ))
    }
}
