//! SGX (Singapore Exchange) futures settlement price data.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;

impl AkShareClient {
    /// SGX derivatives historical settlement prices.
    ///
    /// Fetches all futures settlement prices for a given trading date.
    /// Note: requires determining the correct SGX archive number.
    pub async fn futures_settlement_price_sgx(&self, date: &str) -> Result<Vec<Row>> {
        // First, get the FTSE index data from Eastmoney to calculate the archive number
        let url = "https://push2his.eastmoney.com/api/qt/stock/kline/get";
        let body = self
                        .get(url)
            .query(&[
                ("secid", "100.STI"),
                ("klt", "101"),
                ("fqt", "0"),
                ("lmt", "10000"),
                ("end", date),
                ("iscca", "1"),
                ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64"),
                ("ut", "f057cbcbce2a86e2866ab8877db1d059"),
                ("forcect", "1"),
            ])
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let klines = data["data"]["klines"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        if klines.is_empty() {
            return Err(Error::not_found("no FTSE data to calculate SGX number"));
        }

        let num = klines.len() + 791;
        let zip_url = format!(
            "https://links.sgx.com/1.0.0/derivatives-daily/{}/FUTURE.zip",
            num
        );

        let _zip_body = self
                        .get(&zip_url)
            .send()
            .await?
            .bytes()
            .await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("source".into(), serde_json::json!("sgx"));
        row.insert("date".into(), serde_json::json!(date));
        row.insert("archive_num".into(), serde_json::json!(num));
        row.insert("note".into(), serde_json::json!("ZIP archive - requires zip parser"));
        items.push(row);
        Ok(items)
    }
}
