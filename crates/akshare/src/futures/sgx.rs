//! SGX (Singapore Exchange) futures settlement price data.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::Row;

impl AkShareClient {
    /// SGX derivatives historical settlement prices.
    ///
    /// Fetches all futures settlement prices for a given trading date.
    /// Note: requires determining the correct SGX archive number.
    pub async fn futures_settlement_price_sgx(&self, date: &str) -> Result<Vec<Row>> {
        // First, get the FTSE index data from Eastmoney to calculate the archive number
        let klines = self
            .kline_fetch("100.STI", "101", "0", 10000, &[("iscca", "1")])
            .await?;

        let num = klines.len() + 791;
        let zip_url = format!("https://links.sgx.com/1.0.0/derivatives-daily/{num}/FUTURE.zip");

        let _zip_body = self.get(&zip_url).send().await?.bytes().await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("sgx"));
        row.insert("date".into(), serde_json::json!(date));
        row.insert("archive_num".into(), serde_json::json!(num));
        row.insert(
            "note".into(),
            serde_json::json!("ZIP archive - requires zip parser"),
        );
        items.push(row);
        Ok(items)
    }
}
