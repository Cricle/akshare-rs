#![allow(dead_code)]
//! FX data: FX swap rates, pair quotes, Baidu FX quotes.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct BaiduFxResponse {
    #[serde(default)]
    data: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// China Foreign Exchange Trade System (CFETS) - FX currency pair map.
    ///
    /// Returns the mapping of currency pair names to codes from CFETS.
    pub async fn currency_pair_map(&self) -> Result<Vec<MacroDataPoint>> {
        // Use Eastmoney datacenter for currency pair mapping
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: serde_json::Value = self
            .get(url)
            .query(&[
                ("reportName", "RPT_FE_QUOTATION_BOCCN"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "50"),
                ("sortTypes", "-1"),
                ("sortColumns", "DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp
            .nested(&["result", "data"])
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for v in &data {
            let name = v.str_or(&["CURRENCY_NAME"], "");
            let code = v.str_or(&["CURRENCY_CODE"], "");
            if !name.is_empty() {
                items.push(MacroDataPoint {
                    date: code,
                    value: 0.0,
                    name,
                });
            }
        }
        Ok(items)
    }

    /// CFETS FX C-Swap (Currency Swap) rates from China Money.
    ///
    /// Returns FX swap benchmark rates published by CFETS.
    pub async fn fx_c_swap_cm(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bk-ccs/CcsSwpBm";
        let body: serde_json::Value = self
            .get(url)
            .query(&[("lang", "CN")])
            .send()
            .await?
            .json()
            .await?;

        let records = body
            .get("records")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for record in &records {
            let date = record.str_or(&["startDate"], "");
            let pair = record.str_or(&["ccyPair"], "");

            // Extract swap points for different tenors
            for tenor in &["ON", "1W", "2W", "1M", "3M", "6M", "1Y"] {
                let key = format!("swapPoint{tenor}");
                if let Some(val) = record.get(&key).and_then(serde_json::Value::as_f64) {
                    items.push(MacroDataPoint {
                        date: date.clone(),
                        value: val,
                        name: format!("FX C-Swap {pair} {tenor}"),
                    });
                }
            }
        }
        Ok(items)
    }

    /// FX pair quote from China Money.
    ///
    /// Returns FX spot/forward quotes for a specific currency pair.
    pub async fn fx_pair_quote(&self, pair: &str) -> Result<Vec<MacroDataPoint>> {
        let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bk-ccs/CcsMktQuotation";
        let body: serde_json::Value = self
            .get(url)
            .query(&[("lang", "CN"), ("ccyPair", pair)])
            .send()
            .await?
            .json()
            .await?;

        let records = body
            .get("records")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for record in &records {
            let date = record.str_or(&["startDate"], "");
            let bid = record.f64_or(&["bidPips"], 0.0);
            items.push(MacroDataPoint {
                date,
                value: bid,
                name: format!("FX Quote {pair}"),
            });
        }
        Ok(items)
    }

    /// FX spot quote from China Money.
    ///
    /// Returns FX spot market quotes.
    pub async fn fx_spot_quote(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bk-ccs/CcsSpotQuotation";
        let body: serde_json::Value = self
            .get(url)
            .query(&[("lang", "CN")])
            .send()
            .await?
            .json()
            .await?;

        let records = body
            .get("records")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for record in &records {
            let date = record.str_or(&["startDate"], "");
            let pair = record.str_or(&["ccyPair"], "");
            let spot = record.f64_or(&["spotMid"], 0.0);
            items.push(MacroDataPoint {
                date,
                value: spot,
                name: format!("FX Spot {pair}"),
            });
        }
        Ok(items)
    }

    /// FX swap quote from China Money.
    ///
    /// Returns FX swap market quotes.
    pub async fn fx_swap_quote(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bk-ccs/CcsSwapQuotation";
        let body: serde_json::Value = self
            .get(url)
            .query(&[("lang", "CN")])
            .send()
            .await?
            .json()
            .await?;

        let records = body
            .get("records")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for record in &records {
            let date = record.str_or(&["startDate"], "");
            let pair = record.str_or(&["ccyPair"], "");
            let swap_pips = record.f64_or(&["swapPoint"], 0.0);
            items.push(MacroDataPoint {
                date,
                value: swap_pips,
                name: format!("FX Swap {pair}"),
            });
        }
        Ok(items)
    }

    /// Baidu Finance FX quote.
    ///
    /// Fetches real-time FX quotes from Baidu Finance.
    pub async fn fx_quote_baidu(&self, pair: &str) -> Result<Vec<MacroDataPoint>> {
        let url = "https://finance.pae.baidu.com/api/getbondprice";
        let body: serde_json::Value = self
            .get(url)
            .query(&[("code", pair), ("pointType", "string")])
            .header("User-Agent", "Mozilla/5.0 (compatible; akshare-rust/0.1)")
            .send()
            .await?
            .json()
            .await?;

        let mut items = Vec::new();
        if let Some(result) = body.get("Result").or_else(|| body.get("result"))
            && let Some(arr) = result.as_array()
        {
            for entry in arr {
                let name = entry.str_or(&["name"], "");
                let price = entry.f64_or(&["price", "currentPrice"], 0.0);
                if !name.is_empty() {
                    items.push(MacroDataPoint {
                        date: pair.to_string(),
                        value: price,
                        name,
                    });
                }
            }
        }

        if items.is_empty() {
            return Err(Error::not_found(format!("baidu fx: no data for {pair}")));
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    // Tests would require live API calls
}
