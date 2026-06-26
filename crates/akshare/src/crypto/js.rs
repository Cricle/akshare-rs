//! Crypto spot prices from Jin10 (金十数据).

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::CryptoSpot;
use crate::types::value_ext::ValueExt;

impl AkShareClient {
    /// Fetch crypto spot prices from JS (金十数据) — Python-compatible name.
    pub async fn crypto_js_spot(&self) -> Result<Vec<CryptoSpot>> {
        self.crypto_spot().await
    }

    /// Fetch crypto spot prices from Jin10 data center.
    pub async fn crypto_spot(&self) -> Result<Vec<CryptoSpot>> {
        let url = "https://datacenter-api.jin10.com/crypto_currency/list";
        let resp = self
            .get(url)
            .header("Referer", "https://www.jin10.com")
            .header("x-app-id", "rU6QIu7JHe2gOUeR")
            .header("x-csrf-token", "x-csrf-token")
            .header("x-version", "1.0.0")
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(Error::upstream(format!(
                "jin10 crypto: HTTP {}",
                resp.status()
            )));
        }

        let body: serde_json::Value = resp.json().await?;

        let items_raw = body
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .or_else(|| body.as_array().cloned())
            .unwrap_or_default();

        let mut items = Vec::with_capacity(items_raw.len());
        for v in &items_raw {
            let symbol = v.str_or(&["symbol", "currency_pair"], "");
            let name = v.str_or(&["name", "bourse", "market"], "");

            if symbol.is_empty() {
                continue;
            }

            let price_usd = v.f64_or(&["price", "price_usd"], 0.0);
            let price_cny = v.f64_or(&["cny_price", "price_cny"], 0.0);
            let change_24h_pct = v.f64_or(
                &["change", "change_24h", "change_percent", "up_down_rate"],
                0.0,
            );
            let volume_24h = v.f64_or(&["volume", "volume_24h"], 0.0);
            let market_cap = v.f64_or(&["market_cap"], 0.0);

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
