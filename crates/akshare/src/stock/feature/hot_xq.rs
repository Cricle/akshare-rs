//! Xueqiu hot stocks (雪球热度排行).

use super::types::*;
use crate::client::AkShareClient;
use crate::error::Result;

const XQ_BASE: &str = "https://xueqiu.com/service/v5/stock/screener/screen";

impl AkShareClient {
    /// Xueqiu stock screener fetch
    async fn xq_hot_fetch(&self, order_by: &str) -> Result<Vec<HotStockXq>> {
        let mut all = Vec::new();
        for page in 1..=3 {
            let resp = self
                .get(XQ_BASE)
                .query(&[
                    ("category", "CN"),
                    ("size", "200"),
                    ("order", "desc"),
                    ("order_by", order_by),
                    ("only_count", "0"),
                    ("page", &page.to_string()),
                ])
                .header("Accept", "*/*")
                .header("Referer", "https://xueqiu.com/hq")
                .send()
                .await?
                .error_for_status()?
                .json::<serde_json::Value>()
                .await?;
            let data = resp
                .get("data")
                .and_then(|d| d.get("list"))
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default();
            if data.is_empty() {
                break;
            }
            for v in &data {
                all.push(HotStockXq {
                    code: v
                        .get("symbol")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string(),
                    name: v
                        .get("name")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string(),
                    value: v.get(order_by).and_then(|x| x.as_f64()).unwrap_or(0.0),
                    latest_price: v.get("current").and_then(|x| x.as_f64()).unwrap_or(0.0),
                });
            }
        }
        Ok(all)
    }

    /// 雪球-关注排行榜
    pub async fn stock_hot_follow_xq(&self, symbol: &str) -> Result<Vec<HotStockXq>> {
        let order_by = match symbol {
            "本周新增" => "follow7d",
            _ => "follow",
        };
        self.xq_hot_fetch(order_by).await
    }

    /// 雪球-讨论排行榜
    pub async fn stock_hot_tweet_xq(&self, symbol: &str) -> Result<Vec<HotStockXq>> {
        let order_by = match symbol {
            "本周新增" => "tweet7d",
            _ => "tweet",
        };
        self.xq_hot_fetch(order_by).await
    }

    /// 雪球-交易排行榜
    pub async fn stock_hot_deal_xq(&self, symbol: &str) -> Result<Vec<HotStockXq>> {
        let order_by = match symbol {
            "本周新增" => "deal7d",
            _ => "deal",
        };
        self.xq_hot_fetch(order_by).await
    }
}
