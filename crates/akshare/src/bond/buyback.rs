//! Bond buyback (质押式回购) data from Eastmoney.
//!
//! Shanghai and Shenzhen exchange buyback lists and historical data.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::BondSnapshot;
use crate::types::wire::ClistResp;

impl AkShareClient {
    /// Fetch Shanghai exchange bond buyback (质押式回购) list.
    ///
    /// Returns up to `limit` instruments from `m:1+b:MK0356`.
    pub async fn bond_sh_buy_back_em(&self, limit: usize) -> Result<Vec<BondSnapshot>> {
        self.fetch_buyback_list("m:1+b:MK0356", limit).await
    }

    /// Fetch Shenzhen exchange bond buyback (质押式回购) list.
    ///
    /// Returns up to `limit` instruments from `m:0+b:MK0356`.
    pub async fn bond_sz_buy_back_em(&self, limit: usize) -> Result<Vec<BondSnapshot>> {
        self.fetch_buyback_list("m:0+b:MK0356", limit).await
    }

    /// Fetch historical daily candles for a bond buyback instrument.
    ///
    /// Uses Eastmoney kline API. `symbol` is the 6-digit code (e.g. "204001").
    pub async fn bond_buy_back_hist_em(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<crate::types::CandlePoint>> {
        let s = symbol.trim();
        let market_id = if s.starts_with('1') { "0" } else { "1" };
        let secid = format!("{market_id}.{s}");
        self.eastmoney_klines(&secid, "qfq", limit).await
    }

    async fn fetch_buyback_list(&self, fs: &str, limit: usize) -> Result<Vec<BondSnapshot>> {
        let pz = limit.max(1).to_string();
        let resp = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("np", "1"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fs", fs),
                (
                    "fields",
                    "f12,f13,f14,f1,f2,f4,f3,f152,f17,f18,f15,f16,f5,f6",
                ),
                ("fid", "f6"),
                ("pn", "1"),
                ("pz", pz.as_str()),
                ("po", "1"),
                ("dect", "1"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: ClistResp = resp.json().await.map_err(Error::from)?;
        let items = payload.data.and_then(|d| d.diff).unwrap_or_default();
        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no buyback items"));
        }

        let today = crate::util::today_iso();
        let snapshots: Vec<BondSnapshot> = items
            .into_iter()
            .take(limit)
            .filter_map(|v| {
                let code = v.get("f12")?.as_str()?.to_string();
                let name = v
                    .get("f14")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string();
                let price = v
                    .get("f2")
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0);
                let change_pct = v
                    .get("f3")
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0);
                Some(BondSnapshot {
                    symbol: code,
                    name,
                    date: today.clone(),
                    close: price,
                    change_pct,
                    yield_rate: None,
                    credit_rating: None,
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found("no valid buyback items parsed"));
        }
        Ok(snapshots)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_market_inference() {
        let s = "204001";
        let mid = if s.starts_with('1') { "0" } else { "1" };
        assert_eq!(mid, "1");
        let s2 = "131810";
        let mid2 = if s2.starts_with('1') { "0" } else { "1" };
        assert_eq!(mid2, "0");
    }
}
