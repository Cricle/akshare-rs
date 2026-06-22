//! Futures news from Shanghai Metals Market (上海金属网).

use std::sync::LazyLock;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::Row;

static UTC_PLUS_8: LazyLock<chrono::FixedOffset> = LazyLock::new(|| {
    chrono::FixedOffset::east_opt(8 * 3600).expect("valid UTC+8 offset")
});

impl AkShareClient {
    /// SHMET news flash (上海金属网快讯).
    ///
    /// `category`: "全部", "要闻", "VIP", "财经", "铜", "铝", "铅", "锌",
    ///             "镍", "锡", "贵金属", "小金属"
    pub async fn futures_news_shmet(&self, category: &str) -> Result<Vec<Row>> {
        let url = "https://www.shmet.com/api/rest/news/queryNewsflashList";

        let symbol_map = [
            ("全部", ""),
            ("要闻", "0"),
            ("VIP", "100"),
            ("财经", "999"),
            ("铜", "1002"),
            ("铝", "1003"),
            ("铅", "1005"),
            ("锌", "1004"),
            ("镍", "1006"),
            ("锡", "1007"),
            ("贵金属", "1008"),
            ("小金属", "1009"),
        ];

        let flash_tag = symbol_map
            .iter()
            .find(|(k, _)| *k == category)
            .map_or("", |(_, v)| *v);

        let payload = if flash_tag.is_empty() {
            serde_json::json!({
                "currentPage": 1,
                "pageSize": 100
            })
        } else {
            serde_json::json!({
                "currentPage": 1,
                "pageSize": 2000,
                "content": "",
                "flashTag": flash_tag
            })
        };

        let body = self.post(url).json(&payload).send().await?.text().await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"]["dataList"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let arr = row.as_array().cloned().unwrap_or_default();
            if arr.len() < 6 {
                continue;
            }
            let timestamp_ms = arr[3].as_i64().unwrap_or(0);
            let content = arr[5].as_str().unwrap_or("");

            let datetime = chrono::DateTime::from_timestamp_millis(timestamp_ms)
                .map(|dt| {
                    dt.with_timezone(&*UTC_PLUS_8)
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                })
                .unwrap_or_default();

            let mut r = Row::new();
            r.insert("published_at".into(), serde_json::json!(datetime));
            r.insert("content".into(), serde_json::json!(content));
            items.push(r);
        }
        Ok(items)
    }
}
