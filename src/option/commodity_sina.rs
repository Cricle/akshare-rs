//! Commodity options from Sina Finance.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::parse_f64_safe;

use super::cffex_sina::{CffexOptionSpotRow, strip_html_tags};

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

/// Commodity option contract list entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommodityContractEntry {
    /// Sequential number (1-based).
    pub index: usize,
    /// Contract code.
    pub contract: String,
}

/// A single row of commodity option history from Sina.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommodityHistRow {
    /// Date.
    pub date: String,
    /// Open.
    pub open: f64,
    /// High.
    pub high: f64,
    /// Low.
    pub low: f64,
    /// Close.
    pub close: f64,
    /// Volume.
    pub volume: f64,
}

// ---------------------------------------------------------------------------
// Sina commodity symbol URL mapping
// ---------------------------------------------------------------------------

/// Build the product/exchange info from the commodity symbol page.
/// Returns (product, exchange) for the OptionService.getOptionData API.
fn commodity_product_exchange(symbol: &str) -> Option<(&'static str, &'static str)> {
    match symbol {
        "\u{8c46}\u{7c95}\u{9009}\u{6743}" => Some(("m", "dce")),
        "\u{7389}\u{7c73}\u{9009}\u{6743}" => Some(("c", "dce")),
        "\u{94c1}\u{77ff}\u{77f3}\u{9009}\u{6743}" => Some(("i", "dce")),
        "\u{68c9}\u{82b1}\u{9009}\u{6743}" => Some(("CF", "czce")),
        "\u{767d}\u{7cd6}\u{9009}\u{6743}" => Some(("SR", "czce")),
        "PTA\u{9009}\u{6743}" => Some(("TA", "czce")),
        "\u{7532}\u{9187}\u{9009}\u{6743}" => Some(("MA", "czce")),
        "\u{6a61}\u{80f6}\u{9009}\u{6743}" => Some(("ru", "shfe")),
        "\u{6caa}\u{94dc}\u{9009}\u{6743}" => Some(("cu", "shfe")),
        "\u{9ec4}\u{91d1}\u{9009}\u{6743}" => Some(("au", "shfe")),
        "\u{83dc}\u{7c7d}\u{7c95}\u{9009}\u{6743}" => Some(("RM", "czce")),
        "\u{6db2}\u{5316}\u{77f3}\u{6cb9}\u{6c14}\u{9009}\u{6743}" => Some(("pg", "dce")),
        "\u{52a8}\u{529b}\u{7164}\u{9009}\u{6743}" => Some(("ZC", "czce")),
        "\u{83dc}\u{7c7d}\u{6cb9}\u{9009}\u{6743}" => Some(("OI", "czce")),
        "\u{82b1}\u{751f}\u{9009}\u{6743}" => Some(("PK", "czce")),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Commodity option contract list from Sina.
    ///
    /// Scrapes the Sina options page to get available contracts for a given symbol.
    pub async fn option_commodity_contract_sina(
        &self,
        symbol: &str,
    ) -> Result<Vec<CommodityContractEntry>> {
        // Fetch the options page to get the list of commodities
        let base_url = "https://stock.finance.sina.com.cn/futures/view/optionsDP.php/pg_o/dce";
        let body = self
            .get(base_url)
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Extract the URL for the given symbol from the page
        // Look for links with class="active" that match the symbol
        let commodity_url = extract_commodity_url(&body, symbol)
            .ok_or_else(|| Error::not_found(format!("commodity symbol not found: {symbol}")))?;

        // Fetch the commodity page
        let full_url = format!("https://stock.finance.sina.com.cn{commodity_url}");
        let body2 = self
            .get(&full_url)
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Extract contracts from id="option_suffix" li elements
        let contracts = extract_li_all(&body2, "option_suffix");

        let entries: Vec<CommodityContractEntry> = contracts
            .into_iter()
            .enumerate()
            .map(|(i, c)| CommodityContractEntry {
                index: i + 1,
                contract: c,
            })
            .collect();

        if entries.is_empty() {
            return Err(Error::not_found(format!("no contracts found for {symbol}")));
        }

        Ok(entries)
    }

    /// Commodity option contract table (call/put pairs) from Sina.
    ///
    /// Returns the real-time option chain for a specific commodity contract.
    pub async fn option_commodity_contract_table_sina(
        &self,
        symbol: &str,
        contract: &str,
    ) -> Result<Vec<CffexOptionSpotRow>> {
        let (product, exchange) = commodity_product_exchange(symbol).ok_or_else(|| {
            Error::invalid_input(format!("unsupported commodity symbol: {symbol}"))
        })?;

        let url =
            "https://stock.finance.sina.com.cn/futures/api/openapi.php/OptionService.getOptionData";
        let body = self
            .get(url)
            .query(&[
                ("type", "futures"),
                ("product", product),
                ("exchange", exchange),
                ("pinzhong", contract),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        let json_start = body.find('{').unwrap_or(0);
        let json_end = body.rfind('}').map(|i| i + 1).unwrap_or(body.len());
        let json_str = &body[json_start..json_end];

        let data: SinaOptionDataResponse = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("sina commodity table json: {e}")))?;

        let result = data
            .result
            .ok_or_else(|| Error::upstream("sina commodity table: missing result"))?;
        let option_data = result
            .data
            .ok_or_else(|| Error::upstream("sina commodity table: missing data"))?;

        let up = &option_data.up;
        let down = &option_data.down;
        let max_len = up.len().max(down.len());
        let mut rows = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let call_row = up.get(i);
            let put_row = down.get(i);

            rows.push(CffexOptionSpotRow {
                call_bid_qty: call_row
                    .and_then(|r| r.first())
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                call_bid_price: call_row
                    .and_then(|r| r.get(1))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                call_latest_price: call_row
                    .and_then(|r| r.get(2))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                call_ask_price: call_row
                    .and_then(|r| r.get(3))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                call_ask_qty: call_row
                    .and_then(|r| r.get(4))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                call_open_interest: call_row
                    .and_then(|r| r.get(5))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                call_change: call_row
                    .and_then(|r| r.get(6))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                strike_price: call_row
                    .and_then(|r| r.get(7))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                call_id: call_row
                    .and_then(|r| r.get(8))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                put_bid_qty: put_row
                    .and_then(|r| r.first())
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                put_bid_price: put_row
                    .and_then(|r| r.get(1))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                put_latest_price: put_row
                    .and_then(|r| r.get(2))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                put_ask_price: put_row
                    .and_then(|r| r.get(3))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                put_ask_qty: put_row
                    .and_then(|r| r.get(4))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                put_open_interest: put_row
                    .and_then(|r| r.get(5))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                put_change: put_row
                    .and_then(|r| r.get(6))
                    .map(json_to_f64)
                    .unwrap_or(0.0),
                put_id: put_row
                    .and_then(|r| r.get(7))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            });
        }

        if rows.is_empty() {
            return Err(Error::not_found(format!(
                "no commodity option data for {symbol} {contract}"
            )));
        }

        Ok(rows)
    }

    /// Commodity option daily history from Sina.
    ///
    /// `symbol` is the contract code including call/put, e.g. "au2012C392".
    pub async fn option_commodity_hist_sina(&self, symbol: &str) -> Result<Vec<CommodityHistRow>> {
        let now = chrono::Utc::now();
        let url = format!(
            "https://stock.finance.sina.com.cn/futures/api/jsonp.php/var%20_m{}C3000{}_{}_{}=/FutureOptionAllService.getOptionDayline",
            symbol,
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

        // Extract JSON array from JSONP
        let arr_start = body.find('[').unwrap_or(0);
        let arr_end = body.rfind(']').map(|i| i + 1).unwrap_or(body.len());
        if arr_start >= arr_end {
            return Err(Error::decode("sina commodity hist: no JSON array found"));
        }
        let json_str = &body[arr_start..arr_end];

        let raw: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("sina commodity hist json: {e}")))?;

        let rows: Vec<CommodityHistRow> = raw
            .iter()
            .filter(|item| item.len() >= 6)
            .map(|item| CommodityHistRow {
                open: json_to_f64(&item[0]),
                high: json_to_f64(&item[1]),
                low: json_to_f64(&item[2]),
                close: json_to_f64(&item[3]),
                volume: json_to_f64(&item[4]),
                date: item[5].as_str().unwrap_or("").to_string(),
            })
            .collect();

        if rows.is_empty() {
            return Err(Error::not_found(format!(
                "no commodity hist data for {symbol}"
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

/// Extract the commodity URL for a given symbol from the HTML page.
fn extract_commodity_url(html: &str, symbol: &str) -> Option<String> {
    // Look for links containing the symbol text within active list items
    let search_text = format!(">{symbol}<");
    let pos = html.find(&search_text)?;
    // Search backwards for the <a href="
    let before = &html[..pos];
    let href_pos = before.rfind("href=\"")? + 6;
    let href_end = before[href_pos..].find('"')? + href_pos;
    Some(before[href_pos..href_end].to_string())
}

/// Extract all `<li>` text from an element with the given id.
fn extract_li_all(html: &str, id: &str) -> Vec<String> {
    let needle = format!("id=\"{id}\"");
    let Some(pos) = html.find(&needle) else {
        return vec![];
    };
    let after = &html[pos..];
    let mut results = Vec::new();
    let mut search = after;
    while let Some(li_pos) = search.find("<li") {
        let after_li = &search[li_pos..];
        if let Some(end) = after_li.find("</li>") {
            let content = &after_li[..end];
            let text = strip_html_tags(content);
            results.push(text.trim().to_string());
            search = &after_li[end + 5..];
        } else {
            break;
        }
    }
    results
}
