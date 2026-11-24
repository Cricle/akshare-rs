//! Fund info from THS (同花顺).

use regex::Regex;
use serde_json::{Value, json};

use crate::client::AkShareClient;
use crate::error::{Error, Result};

fn span_text(html: &str, class: &str) -> Option<String> {
    let pattern =
        format!(r#"(?is)<span[^>]*class\s*=\s*['"][^'"]*\b{class}\b[^'"]*['"][^>]*>(.*?)</span>"#);
    let re = Regex::new(&pattern).ok()?;
    re.captures(html)
        .and_then(|capture| capture.get(1))
        .map(|inner| crate::fund::strip_html(inner.as_str()))
        .filter(|text| !text.is_empty())
}

impl AkShareClient {
    /// Fetch fund basic information from THS (Python: `fund_info_ths`).
    ///
    /// `symbol`: fund code, e.g. "161130".
    pub async fn fund_info_ths(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let symbol = symbol.trim();
        if symbol.is_empty() {
            return Err(Error::invalid_input("fund code must not be empty"));
        }

        let url = format!("https://fund.10jqka.com.cn/{symbol}/interduce.html");
        let text = crate::util::send_and_check_text(
            self.get(&url)
                .header("Referer", "https://fund.10jqka.com.cn/"),
            Some(encoding_rs::UTF_8),
        )
        .await?;

        let dialog_re = Regex::new(
            r#"(?is)<ul[^>]*class\s*=\s*['"][^'"]*\bg-dialog\b[^'"]*['"][^>]*>(.*?)</ul>"#,
        )
        .map_err(|error| Error::decode(format!("fund info dialog regex: {error}")))?;
        let Some(dialog) = dialog_re
            .captures(&text)
            .and_then(|capture| capture.get(1))
            .map(|matched| matched.as_str())
        else {
            return Err(Error::not_found(format!(
                "no fund information found for {symbol}; the THS page layout may have changed"
            )));
        };

        let li_re = Regex::new(r"(?is)<li[^>]*>(.*?)</li>")
            .map_err(|error| Error::decode(format!("fund info list regex: {error}")))?;
        let mut result = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for capture in li_re.captures_iter(dialog) {
            let Some(item) = capture.get(1) else {
                continue;
            };
            let Some(key) = span_text(item.as_str(), "key") else {
                continue;
            };
            let Some(value) = span_text(item.as_str(), "value") else {
                continue;
            };
            if !seen.insert(key.clone()) {
                continue;
            }
            result.push(json!({ "字段": key, "值": value }));
        }

        if result.is_empty() {
            return Err(Error::not_found(format!(
                "no fund information found for {symbol}"
            )));
        }
        Ok(result)
    }

    /// Legacy placeholder: the THS endpoint requires a fund code.
    ///
    /// Use [`fund_info_ths`](Self::fund_info_ths) instead.
    pub async fn fund_info(&self, _limit: usize) -> Result<Vec<Value>> {
        Err(Error::unavailable(
            "THS fund_info requires a fund selector; use fund_info_ths(symbol)",
        ))
    }
}
