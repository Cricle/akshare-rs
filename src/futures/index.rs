//! Futures commodity index data from CCIDX (中证商品指数).

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;

impl AkShareClient {
    /// CCIDX futures commodity index (中证商品指数).
    ///
    /// `symbol`: "中证商品期货指数" or "中证商品期货价格指数"
    pub async fn futures_index_ccidx(&self, symbol: &str) -> Result<Vec<Row>> {
        let index_id = match symbol {
            "中证商品期货指数" => "100001.CCI",
            "中证商品期货价格指数" => "000001.CCI",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unknown CCIDX index: {}",
                    symbol
                )));
            }
        };

        let url = "http://www.ccidx.com/CCI-ZZZS/index/getDateLine";
        let body = self
                        .get(url)
            .query(&[("indexId", index_id)])
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let date_line = data["data"]["dateLineJson"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for entry in &date_line {
            let mut row = Row::new();
            row.insert("date".into(), entry["tradeDate"].clone());
            row.insert("index_id".into(), entry["indexId"].clone());
            row.insert("close_price".into(), entry["closingPrice"].clone());
            row.insert("settle_price".into(), entry["settlePrice"].clone());
            row.insert("change".into(), entry["dailyIncreaseAndDecrease"].clone());
            row.insert("change_pct".into(), entry["dailyIncreaseAndDecreasePercentage"].clone());
            items.push(row);
        }
        Ok(items)
    }
}
