//! CFFEX index options from Sina Finance (SZ50, HS300, ZZ1000).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::parse_f64_safe;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct SinaOptionDataResponse {
    result: Option<SinaOptionResult>,
}

#[derive(Debug, Deserialize)]
struct SinaOptionResult {
    data: Option<SinaOptionResultData>,
}

#[derive(Debug, Deserialize)]
struct SinaOptionResultData {
    #[serde(default)]
    up: Vec<Vec<serde_json::Value>>,
    #[serde(default)]
    down: Vec<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// A single row of CFFEX option spot data (call + put pair at a strike).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CffexOptionSpotRow {
    /// Call bid quantity.
    pub call_bid_qty: f64,
    /// Call bid price.
    pub call_bid_price: f64,
    /// Call latest price.
    pub call_latest_price: f64,
    /// Call ask price.
    pub call_ask_price: f64,
    /// Call ask quantity.
    pub call_ask_qty: f64,
    /// Call open interest.
    pub call_open_interest: f64,
    /// Call change.
    pub call_change: f64,
    /// Strike price.
    pub strike_price: f64,
    /// Call identifier.
    pub call_id: String,
    /// Put bid quantity.
    pub put_bid_qty: f64,
    /// Put bid price.
    pub put_bid_price: f64,
    /// Put latest price.
    pub put_latest_price: f64,
    /// Put ask price.
    pub put_ask_price: f64,
    /// Put ask quantity.
    pub put_ask_qty: f64,
    /// Put open interest.
    pub put_open_interest: f64,
    /// Put change.
    pub put_change: f64,
    /// Put identifier.
    pub put_id: String,
}

/// A single row of CFFEX option daily kline data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CffexOptionDailyRow {
    /// Date (YYYY-MM-DD).
    pub date: String,
    /// Open price.
    pub open: f64,
    /// High price.
    pub high: f64,
    /// Low price.
    pub low: f64,
    /// Close price.
    pub close: f64,
    /// Volume.
    pub volume: f64,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// CFFEX SZ50 index option contract list from Sina.
    ///
    /// Returns a map of index name to contract list (first contract is the main contract).
    pub async fn option_cffex_sz50_list_sina(&self) -> Result<Vec<(String, Vec<String>)>> {
        let url = "https://stock.finance.sina.com.cn/futures/view/optionsCffexDP.php/ho/cffex";
        self.fetch_cffex_list_sina(url, 0).await
    }

    /// CFFEX HS300 index option contract list from Sina.
    pub async fn option_cffex_hs300_list_sina(&self) -> Result<Vec<(String, Vec<String>)>> {
        let url = "https://stock.finance.sina.com.cn/futures/view/optionsCffexDP.php";
        self.fetch_cffex_list_sina(url, 1).await
    }

    /// CFFEX ZZ1000 index option contract list from Sina.
    pub async fn option_cffex_zz1000_list_sina(&self) -> Result<Vec<(String, Vec<String>)>> {
        let url = "https://stock.finance.sina.com.cn/futures/view/optionsCffexDP.php/mo/cffex";
        self.fetch_cffex_list_sina(url, 2).await
    }

    /// CFFEX SZ50 index option spot (real-time call/put pairs) from Sina.
    pub async fn option_cffex_sz50_spot_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<CffexOptionSpotRow>> {
        self.fetch_cffex_spot_sina("ho", "cffex", symbol).await
    }

    /// CFFEX HS300 index option spot (real-time call/put pairs) from Sina.
    pub async fn option_cffex_hs300_spot_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<CffexOptionSpotRow>> {
        self.fetch_cffex_spot_sina("io", "cffex", symbol).await
    }

    /// CFFEX ZZ1000 index option spot (real-time call/put pairs) from Sina.
    pub async fn option_cffex_zz1000_spot_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<CffexOptionSpotRow>> {
        self.fetch_cffex_spot_sina("mo", "cffex", symbol).await
    }

    /// CFFEX SZ50 index option daily kline from Sina.
    ///
    /// `symbol` is the contract code including call/put identifier, e.g. "ho2303P2350".
    pub async fn option_cffex_sz50_daily_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<CffexOptionDailyRow>> {
        self.fetch_cffex_daily_sina(symbol).await
    }

    /// CFFEX HS300 index option daily kline from Sina.
    pub async fn option_cffex_hs300_daily_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<CffexOptionDailyRow>> {
        self.fetch_cffex_daily_sina(symbol).await
    }

    /// CFFEX ZZ1000 index option daily kline from Sina.
    pub async fn option_cffex_zz1000_daily_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<CffexOptionDailyRow>> {
        self.fetch_cffex_daily_sina(symbol).await
    }

    // -- private helpers ----------------------------------------------------

    async fn fetch_cffex_list_sina(
        &self,
        url: &str,
        symbol_index: usize,
    ) -> Result<Vec<(String, Vec<String>)>> {
        let body = self
            .get(url)
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Parse HTML to extract symbol and contract list
        // Look for id="option_symbol" <li> elements and id="option_suffix" <li> elements
        let symbol =
            extract_html_li_by_id(&body, "option_symbol", symbol_index).unwrap_or_default();
        let contracts = extract_html_li_all_by_id(&body, "option_suffix");

        if symbol.is_empty() {
            return Err(Error::upstream("sina cffex list: no symbol found"));
        }

        Ok(vec![(symbol, contracts)])
    }

    async fn fetch_cffex_spot_sina(
        &self,
        product: &str,
        exchange: &str,
        symbol: &str,
    ) -> Result<Vec<CffexOptionSpotRow>> {
        let url =
            "https://stock.finance.sina.com.cn/futures/api/openapi.php/OptionService.getOptionData";
        let body = self
            .get(url)
            .query(&[
                ("type", "futures"),
                ("product", product),
                ("exchange", exchange),
                ("pinzhong", symbol),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Response is JSONP-like: extract JSON from the response
        let json_start = body.find('{').unwrap_or(0);
        let json_end = body.rfind('}').map(|i| i + 1).unwrap_or(body.len());
        let json_str = &body[json_start..json_end];

        let data: SinaOptionDataResponse = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("sina cffex spot json: {e}")))?;

        let result = data
            .result
            .ok_or_else(|| Error::upstream("sina cffex spot: missing result"))?;
        let option_data = result
            .data
            .ok_or_else(|| Error::upstream("sina cffex spot: missing data"))?;

        let up = &option_data.up;
        let down = &option_data.down;

        let mut rows = Vec::with_capacity(up.len().max(down.len()));
        let max_len = up.len().max(down.len());

        for i in 0..max_len {
            let call_row = up.get(i);
            let put_row = down.get(i);

            let call_bid_qty = call_row
                .and_then(|r| r.first())
                .map(json_to_f64)
                .unwrap_or(0.0);
            let call_bid_price = call_row
                .and_then(|r| r.get(1))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let call_latest_price = call_row
                .and_then(|r| r.get(2))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let call_ask_price = call_row
                .and_then(|r| r.get(3))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let call_ask_qty = call_row
                .and_then(|r| r.get(4))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let call_open_interest = call_row
                .and_then(|r| r.get(5))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let call_change = call_row
                .and_then(|r| r.get(6))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let strike_price = call_row
                .and_then(|r| r.get(7))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let call_id = call_row
                .and_then(|r| r.get(8))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let put_bid_qty = put_row
                .and_then(|r| r.first())
                .map(json_to_f64)
                .unwrap_or(0.0);
            let put_bid_price = put_row
                .and_then(|r| r.get(1))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let put_latest_price = put_row
                .and_then(|r| r.get(2))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let put_ask_price = put_row
                .and_then(|r| r.get(3))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let put_ask_qty = put_row
                .and_then(|r| r.get(4))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let put_open_interest = put_row
                .and_then(|r| r.get(5))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let put_change = put_row
                .and_then(|r| r.get(6))
                .map(json_to_f64)
                .unwrap_or(0.0);
            let put_id = put_row
                .and_then(|r| r.get(7))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            rows.push(CffexOptionSpotRow {
                call_bid_qty,
                call_bid_price,
                call_latest_price,
                call_ask_price,
                call_ask_qty,
                call_open_interest,
                call_change,
                strike_price,
                call_id,
                put_bid_qty,
                put_bid_price,
                put_latest_price,
                put_ask_price,
                put_ask_qty,
                put_open_interest,
                put_change,
                put_id,
            });
        }

        if rows.is_empty() {
            return Err(Error::not_found(format!("no cffex spot data for {symbol}")));
        }

        Ok(rows)
    }

    async fn fetch_cffex_daily_sina(&self, symbol: &str) -> Result<Vec<CffexOptionDailyRow>> {
        let now = chrono::Utc::now();
        let url = format!(
            "https://stock.finance.sina.com.cn/futures/api/jsonp.php/var%20_{symbol}{}_{}_{}/FutureOptionAllService.getOptionDayline",
            now.format("%Y"),
            now.format("%m"),
            now.format("%d"),
        );

        let body = self
            .get(&url)
            .query(&[("symbol", symbol)])
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Extract JSON array from JSONP response
        let arr_start = body.find('[').unwrap_or(0);
        let arr_end = body.rfind(']').map(|i| i + 1).unwrap_or(body.len());
        if arr_start >= arr_end {
            return Err(Error::decode("sina cffex daily: no JSON array found"));
        }
        let json_str = &body[arr_start..arr_end];

        // Data format: [[open, high, low, close, volume, date], ...]
        let raw: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("sina cffex daily json: {e}")))?;

        let mut rows = Vec::with_capacity(raw.len());
        for item in &raw {
            if item.len() < 6 {
                continue;
            }
            rows.push(CffexOptionDailyRow {
                open: json_to_f64(&item[0]),
                high: json_to_f64(&item[1]),
                low: json_to_f64(&item[2]),
                close: json_to_f64(&item[3]),
                volume: json_to_f64(&item[4]),
                date: item[5].as_str().unwrap_or("").to_string(),
            });
        }

        if rows.is_empty() {
            return Err(Error::not_found(format!(
                "no cffex daily data for {symbol}"
            )));
        }

        Ok(rows)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn json_to_f64(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => parse_f64_safe(s),
        _ => 0.0,
    }
}

/// Extract text of the Nth `<li>` child inside an element with the given `id`.
fn extract_html_li_by_id(html: &str, id: &str, index: usize) -> Option<String> {
    let needle = format!("id=\"{id}\"");
    let pos = html.find(&needle)?;
    let after = &html[pos..];
    let mut li_texts = Vec::new();
    let mut search = after;
    while let Some(li_pos) = search.find("<li") {
        let after_li = &search[li_pos..];
        if let Some(end) = after_li.find("</li>") {
            let content = &after_li[..end];
            // strip tags
            let text = strip_html_tags(content);
            li_texts.push(text.trim().to_string());
            search = &after_li[end + 5..];
        } else {
            break;
        }
    }
    li_texts.get(index).cloned()
}

/// Extract all `<li>` text inside an element with the given `id`.
fn extract_html_li_all_by_id(html: &str, id: &str) -> Vec<String> {
    let needle = format!("id=\"{id}\"");
    let Some(pos) = html.find(&needle) else {
        return vec![];
    };
    let after = &html[pos..];
    let mut li_texts = Vec::new();
    let mut search = after;
    while let Some(li_pos) = search.find("<li") {
        let after_li = &search[li_pos..];
        if let Some(end) = after_li.find("</li>") {
            let content = &after_li[..end];
            let text = strip_html_tags(content);
            li_texts.push(text.trim().to_string());
            search = &after_li[end + 5..];
        } else {
            break;
        }
    }
    li_texts
}

/// Strip HTML tags from a string.
pub(crate) fn strip_html_tags(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_tag = false;
    for ch in s.chars() {
        if ch == '<' {
            in_tag = true;
        } else if ch == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_html_tags() {
        assert_eq!(strip_html_tags("<b>hello</b>"), "hello");
        assert_eq!(strip_html_tags("plain text"), "plain text");
    }

    #[test]
    fn test_json_to_f64() {
        assert!((json_to_f64(&serde_json::json!(42.5)) - 42.5).abs() < 0.01);
        assert!((json_to_f64(&serde_json::json!("3.14")) - std::f64::consts::PI).abs() < 0.01);
        assert!((json_to_f64(&serde_json::json!(null)) - 0.0).abs() < 0.01);
    }
}
