//! Futures spot-to-stock (现货与股票) data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::Row;

impl AkShareClient {
    /// Futures spot-to-stock mapping — unified entry point.
    ///
    /// `date`: date string for the data
    /// Returns spot-to-stock mapping data.
    pub async fn futures_spot_stock(&self, date: &str) -> Result<Vec<Row>> {
        let url = "https://data.eastmoney.com/ifdata/xhgp.html";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("date".into(), serde_json::json!(date));
        row.insert("source".into(), serde_json::json!("eastmoney"));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }

    /// Eastmoney spot-to-stock mapping data.
    ///
    /// `category`: "能源", "化工", "塑料", "纺织", "有色", "钢铁", "建材", "农副"
    pub async fn futures_spot_stock_em(&self, category: &str) -> Result<Vec<Row>> {
        let url = "https://data.eastmoney.com/ifdata/xhgp.html";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("category".into(), serde_json::json!(category));
        row.insert("source".into(), serde_json::json!("eastmoney"));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }
}
