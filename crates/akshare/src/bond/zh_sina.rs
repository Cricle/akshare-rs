//! Chinese bond spot and daily data from Sina Finance.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::BondSnapshot;

#[derive(Debug, Deserialize)]
struct SinaBondQuote {
    symbol: Option<String>,
    name: Option<String>,
    trade: Option<String>,
    change_pct: Option<String>,
}

impl AkShareClient {
    /// Fetch real-time Chinese bond spot quotes from Sina Finance.
    ///
    /// Returns up to `limit` pages of bond quotes (80 items per page).
    /// Note: heavy scraping may trigger IP bans.
    pub async fn bond_zh_hs_spot(&self, limit: usize) -> Result<Vec<BondSnapshot>> {
        let mut all_items = Vec::new();
        let today = crate::util::today_iso();

        for page in 1..=limit {
            let page_str = page.to_string();
            let resp = self
                                .get("https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData")
                .query(&[
                    ("page", page_str.as_str()),
                    ("num", "80"),
                    ("sort", "changepercent"),
                    ("asc", "0"),
                    ("node", "hs_z"),
                    ("symbol", ""),
                    ("_s_r_a", "page"),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let text = resp.text().await.map_err(Error::from)?;
            if text.is_empty() || text == "null" {
                break;
            }
            let items: Vec<SinaBondQuote> = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(_) => break,
            };

            for item in items {
                let symbol = item.symbol.unwrap_or_default();
                if symbol.is_empty() {
                    continue;
                }
                all_items.push(BondSnapshot {
                    symbol,
                    name: item.name.unwrap_or_default(),
                    date: today.clone(),
                    close: item.trade.unwrap_or_default().parse().unwrap_or(0.0),
                    change_pct: item.change_pct.unwrap_or_default().parse().unwrap_or(0.0),
                    yield_rate: None,
                    credit_rating: None,
                });
            }
        }

        if all_items.is_empty() {
            return Err(Error::not_found("sina returned no bond spot data"));
        }
        Ok(all_items)
    }

    /// Fetch historical daily data for a Chinese bond from Sina Finance.
    ///
    /// `symbol` is in Sina format (e.g. "sh010107").
    pub async fn bond_zh_hs_daily(&self, symbol: &str) -> Result<Vec<BondSnapshot>> {
        let now = chrono::Utc::now().format("%Y_%m_%d").to_string();
        let url = format!("https://hq.sinajs.cn/lb/{symbol}_{now}");
        let resp = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = resp.text().await.map_err(Error::from)?;
        if text.is_empty() {
            return Err(Error::not_found(format!(
                "sina returned no data for {symbol}"
            )));
        }

        // Sina returns encrypted JS; we attempt to parse the data section.
        // For now return the raw response as unsupported.
        Err(Error::decode(format!(
            "sina bond daily data requires JS decryption; use eastmoney_klines instead for symbol {symbol}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sina_bond_quote_deserialize() {
        let j = json!({
            "symbol": "sh010107",
            "name": "21国债(7)",
            "trade": "100.50",
            "change": "0.30",
            "change_pct": "0.30",
            "buy": "100.40",
            "sell": "100.60",
            "settlement": "100.20",
            "open": "100.30",
            "high": "100.80",
            "low": "100.10",
            "volume": "5000",
            "amount": "502500"
        });
        let q: SinaBondQuote = serde_json::from_value(j).unwrap();
        assert_eq!(q.symbol.unwrap(), "sh010107");
        assert_eq!(q.name.unwrap(), "21国债(7)");
    }
}
