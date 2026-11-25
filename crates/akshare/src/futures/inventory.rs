//! Futures inventory (库存) data.
//!
//! Sources: Eastmoney (东方财富网), 99qh (99期货网)

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;
use crate::util::today_iso;

impl AkShareClient {
    /// Eastmoney futures inventory data.
    ///
    /// `symbol`: variety name in Chinese (e.g., "铝") or code (e.g., "a")
    pub async fn futures_inventory(&self, symbol: &str) -> Result<Vec<Row>> {
        // Step 1: Get product code mapping
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let body = self
            .get(url)
            .query(&[
                ("reportName", "RPT_FUTU_POSITIONCODE"),
                ("columns", "TRADE_MARKET_CODE,TRADE_CODE,TRADE_TYPE"),
                ("filter", r#"(IS_MAINCODE="1")"#),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["result"]["data"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        // Build symbol -> code mapping
        let mut code_map = std::collections::HashMap::new();
        for row in &rows {
            let trade_type = row["TRADE_TYPE"].as_str().unwrap_or("");
            let trade_code = row["TRADE_CODE"].as_str().unwrap_or("");
            if !trade_type.is_empty() && !trade_code.is_empty() {
                code_map.insert(trade_type.to_string(), trade_code.to_string());
            }
        }

        let product_id = code_map
            .get(symbol)
            .ok_or_else(|| Error::invalid_input(format!("unknown inventory symbol: {symbol}")))?;

        // Step 2: Fetch inventory data
        let filter = format!(r#"(SECURITY_CODE="{product_id}")(TRADE_DATE>='2020-10-28')"#);
        let body2 = self
            .get(url)
            .query(&[
                ("reportName", "RPT_FUTU_STOCKDATA"),
                (
                    "columns",
                    "SECURITY_CODE,TRADE_DATE,ON_WARRANT_NUM,ADDCHANGE",
                ),
                ("filter", filter.as_str()),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "TRADE_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .text()
            .await?;

        let data2: serde_json::Value = serde_json::from_str(&body2)?;
        let rows2 = data2["result"]["data"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows2 {
            let Some(arr) = row.as_array() else {
                continue;
            };
            if arr.len() < 4 {
                continue;
            }
            let mut r = Row::new();
            r.insert("date".into(), arr[1].clone());
            r.insert("inventory".into(), arr[2].clone());
            r.insert("change".into(), arr[3].clone());
            items.push(r);
        }
        Ok(items)
    }

    /// 99qh futures inventory data.
    ///
    /// `symbol`: Chinese name (e.g., "豆一")
    pub async fn futures_inventory_99(&self, symbol: &str) -> Result<Vec<Row>> {
        let page = self
            .get("https://www.99qh.com/data/stockIn")
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;
        let marker = r#"id="__NEXT_DATA__""#;
        let start = page
            .find(marker)
            .and_then(|index| page[index..].find('>').map(|offset| index + offset + 1))
            .ok_or_else(|| Error::decode("99qh stock inventory missing __NEXT_DATA__"))?;
        let end = page[start..]
            .find("</script>")
            .map(|offset| start + offset)
            .ok_or_else(|| Error::decode("99qh stock inventory malformed __NEXT_DATA__"))?;
        let root: serde_json::Value = serde_json::from_str(&page[start..end])
            .map_err(|err| Error::decode(format!("99qh inventory JSON: {err}")))?;
        let products = root["props"]["pageProps"]["data"]["varietyListData"]
            .as_array()
            .ok_or_else(|| Error::decode("99qh inventory product data missing"))?;
        let product_id = products
            .iter()
            .flat_map(|variety| variety["productList"].as_array().into_iter().flatten())
            .find(|product| product["name"].as_str() == Some(symbol))
            .and_then(|product| {
                product["productId"]
                    .as_str()
                    .map(str::to_string)
                    .or_else(|| product["productId"].as_i64().map(|value| value.to_string()))
            })
            .ok_or_else(|| {
                Error::invalid_input(format!("unknown 99qh inventory symbol: {symbol}"))
            })?;
        let token = self
            .get("https://centerapi.fx168api.com/app/common/v.js")
            .header("Origin", "https://www.99qh.com")
            .header("Referer", "https://www.99qh.com")
            .send()
            .await?
            .headers()
            .get("_pcc")
            .and_then(|value| value.to_str().ok())
            .map(str::to_string)
            .ok_or_else(|| Error::decode("99qh inventory token missing"))?;
        let today = today_iso();
        let root: serde_json::Value = self
            .get("https://centerapi.fx168api.com/app/qh/api/stock/trend")
            .query(&[
                ("productId", product_id.as_str()),
                ("type", "1"),
                ("pageNo", "1"),
                ("pageSize", "5000"),
                ("startDate", ""),
                ("endDate", today.as_str()),
                ("appCategory", "web"),
            ])
            .header("_pcc", token)
            .header("site-channel", "003")
            .header("Origin", "https://www.99qh.com")
            .header("Referer", "https://www.99qh.com")
            .send()
            .await?
            .json()
            .await?;
        let rows = root["data"]["list"].as_array().cloned().unwrap_or_default();
        let mut result = Vec::new();
        for item in rows {
            let Some(values) = item.as_array() else {
                continue;
            };
            if values.len() < 3 {
                continue;
            }
            let mut row = Row::new();
            row.insert("日期".into(), values[0].clone());
            row.insert("收盘价".into(), values[1].clone());
            row.insert("库存".into(), values[2].clone());
            result.push(row);
        }
        result.sort_by(|left, right| left["日期"].to_string().cmp(&right["日期"].to_string()));
        if result.is_empty() {
            return Err(Error::not_found(format!(
                "no 99qh inventory data for {symbol}"
            )));
        }
        Ok(result)
    }
}
