//! Crypto spot prices from Jin10 (金十数据).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::CryptoSpot;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Jin10CryptoItem {
    #[serde(default)]
    symbol: String,
    #[serde(default)]
    name: String,
    #[serde(default, deserialize_with = "de_f64_from_any")]
    price: f64,
    #[serde(default, deserialize_with = "de_f64_from_any")]
    cny_price: f64,
    #[serde(default, deserialize_with = "de_f64_from_any")]
    change: f64,
    #[serde(default, deserialize_with = "de_f64_from_any")]
    volume: f64,
    #[serde(default, deserialize_with = "de_f64_from_any")]
    market_cap: f64,
}

/// Deserialize f64 from either a number or a string.
#[allow(dead_code)]
fn de_f64_from_any<'de, D>(deserializer: D) -> std::result::Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer)?;
    match v {
        serde_json::Value::Number(n) => Ok(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => Ok(s.parse::<f64>().unwrap_or(0.0)),
        _ => Ok(0.0),
    }
}

impl AkShareClient {
    /// Fetch crypto spot prices from JS (金十数据) — Python-compatible name.
    pub async fn crypto_js_spot(&self) -> Result<Vec<CryptoSpot>> {
        self.crypto_spot().await
    }

    /// Fetch crypto spot prices from Jin10 data center.
    pub async fn crypto_spot(&self) -> Result<Vec<CryptoSpot>> {
        let url = "https://cdn.jin10.com/data_center/reports/exchange_rate.json";
        let resp = self
                        .get(url)
            .header("Referer", "https://www.jin10.com")
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(Error::upstream(format!(
                "jin10 crypto: HTTP {}",
                resp.status()
            )));
        }

        let body: serde_json::Value = resp.json().await?;

        // The response format varies; try to extract an array of crypto items
        let items_raw = body
            .as_array()
            .cloned()
            .or_else(|| {
                body.get("data")
                    .and_then(|d| d.as_array())
                    .cloned()
            })
            .unwrap_or_default();

        let mut items = Vec::with_capacity(items_raw.len());
        for v in &items_raw {
            let symbol = v
                .get("symbol")
                .or_else(|| v.get("code"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            let name = v
                .get("name")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();

            if symbol.is_empty() {
                continue;
            }

            let price_usd = v
                .get("price")
                .or_else(|| v.get("price_usd"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);
            let price_cny = v
                .get("cny_price")
                .or_else(|| v.get("price_cny"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);
            let change_24h_pct = v
                .get("change")
                .or_else(|| v.get("change_24h"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);
            let volume_24h = v
                .get("volume")
                .or_else(|| v.get("volume_24h"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);
            let market_cap = v
                .get("market_cap")
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);

            items.push(CryptoSpot {
                symbol,
                name,
                price_usd,
                price_cny,
                change_24h_pct,
                volume_24h,
                market_cap,
            });
        }
        Ok(items)
    }
}
