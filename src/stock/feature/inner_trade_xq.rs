//! Insider trading (内部交易) from Xueqiu.

use crate::client::AkShareClient;
use crate::error::Result;
use super::helpers::*;
use super::types::*;

const INNER_TRADE_URL: &str = "https://xueqiu.com/service/v5/stock/f10/cn/skholderchg";

impl AkShareClient {
    /// 雪球-内部交易
    pub async fn stock_inner_trade_xq(&self) -> Result<Vec<InnerTradeXq>> {
        let resp = self.get(INNER_TRADE_URL)
            .query(&[
                ("size", "100000"),
                ("page", "1"),
                ("extend", "true"),
            ])
            .header("Accept", "*/*")
            .header("Referer", "https://xueqiu.com/hq")
            .send().await?
            .error_for_status()?
            .json::<serde_json::Value>().await?;

        let data = resp.get("data").and_then(|d| d.get("list")).and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(data.iter().map(|v| InnerTradeXq {
            code: json_str(v, "symbol"),
            name: json_str(v, "name"),
            change_date: json_str(v, "changedate"),
            changer: json_str(v, "changer"),
            change_shares: json_f64(v, "changecount"),
            avg_price: json_f64(v, "avgprice"),
            holding_after_change: json_f64(v, "holdcount"),
            relationship: json_str(v, "relationship"),
            position: json_str(v, "position"),
        }).collect())
    }
}
