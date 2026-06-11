//! 中国柯桥纺织指数.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 中国柯桥纺织指数.
    ///
    /// `symbol` is one of: "价格指数", "景气指数", "外贸指数".
    pub async fn index_kq_fz(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let index_type = match symbol {
            "价格指数" => "1_1",
            "景气指数" => "1_2",
            "外贸指数" => "2",
            _ => return Err(Error::invalid_input(format!("unsupported KQ FZ symbol: {symbol}"))),
        };

        // First request to get page count
        let resp = self
                        .get("http://www.kqindex.cn/flzs/table_data")
            .query(&[
                ("category", "0"),
                ("start", ""),
                ("end", ""),
                ("indexType", index_type),
                ("pageindex", "1"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        #[derive(Deserialize)]
        struct PageEnvelope {
            page: Option<i64>,
            result: Option<Vec<serde_json::Value>>,
        }

        let first: PageEnvelope = resp.json().await.map_err(Error::from)?;
        let page_count = first.page.unwrap_or(1);
        let mut all_rows: Vec<serde_json::Value> = first.result.unwrap_or_default();

        // Fetch remaining pages
        for page in 2..=page_count {
            let resp = self
                                .get("http://www.kqindex.cn/flzs/table_data")
                .query(&[
                    ("category", "0"),
                    ("start", ""),
                    ("end", ""),
                    ("indexType", index_type),
                    ("pageindex", &page.to_string()),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let page_data: PageEnvelope = resp.json().await.map_err(Error::from)?;
            if let Some(rows) = page_data.result {
                all_rows.extend(rows);
            }
        }

        if all_rows.is_empty() {
            return Err(Error::not_found("kqindex returned no data"));
        }
        Ok(all_rows)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // KQ FZ functions require network access.
    }
}
