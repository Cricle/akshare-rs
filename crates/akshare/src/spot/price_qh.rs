#![allow(dead_code)]
//! Spot price data from 99 QH (99期货).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

#[derive(Debug, Deserialize)]
struct QhTrendResp {
    data: Option<QhTrendData>,
}

#[derive(Debug, Deserialize)]
struct QhTrendData {
    list: Option<Vec<QhTrendItem>>,
}

#[derive(Debug, Deserialize)]
struct QhTrendItem {
    date: Option<String>,
    fp: Option<String>,
    sp: Option<String>,
}

impl AkShareClient {
    /// Fetch spot price trend for a commodity from 99 QH.
    ///
    /// `symbol` is the commodity name (e.g. "螺纹钢").
    /// Returns daily spot prices and futures close prices.
    pub async fn spot_price_qh(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        // First fetch the product list to get the productId
        let products = self.fetch_qh_products().await?;
        let product_id = products
            .iter()
            .find(|(name, _)| name == symbol)
            .map(|(_, id)| id.clone())
            .ok_or_else(|| Error::invalid_input(format!("unknown QH commodity: {symbol}")))?;

        // Get auth token
        let token = self.fetch_qh_token().await?;

        let resp: QhTrendResp = self
            .get("https://centerapi.fx168api.com/app/qh/api/spot/trend")
            .query(&[
                ("productId", product_id.as_str()),
                ("pageNo", "1"),
                ("pageSize", "50000"),
                ("startDate", ""),
                ("endDate", "2050-01-01"),
                ("appCategory", "web"),
            ])
            .header("_pcc", &token)
            .header("Origin", "https://www.99qh.com")
            .header("Referer", "https://www.99qh.com")
            .send()
            .await?
            .json()
            .await?;

        let list = resp.data.and_then(|d| d.list).unwrap_or_default();
        if list.is_empty() {
            return Err(Error::not_found(format!("no spot price data for {symbol}")));
        }

        let items: Vec<MacroDataPoint> = list
            .into_iter()
            .filter_map(|item| {
                let date = item.date?;
                let sp = item.sp?.parse::<f64>().ok()?;
                Some(MacroDataPoint {
                    date: date.get(..10).unwrap_or(&date).to_string(),
                    value: sp,
                    name: symbol.to_string(),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(format!(
                "no valid spot price data for {symbol}"
            )));
        }
        Ok(items)
    }

    /// Fetch the commodity name to product ID mapping table from 99 QH.
    ///
    /// Returns a list of (exchange_name, commodity_name) pairs.
    pub async fn spot_price_table_qh(&self) -> Result<Vec<(String, String)>> {
        let products = self.fetch_qh_products().await?;
        Ok(products
            .into_iter()
            .map(|(name, _id)| name)
            .map(|n| (String::new(), n))
            .collect())
    }

    async fn fetch_qh_products(&self) -> Result<Vec<(String, String)>> {
        let resp = self
            .get("https://www.99qh.com/data/spotTrend")
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Extract __NEXT_DATA__ JSON from the page
        let marker = r#"id="__NEXT_DATA__"#;
        let script_start = resp
            .find(marker)
            .ok_or_else(|| Error::decode("99qh page missing __NEXT_DATA__ script"))?;
        let after_script = &resp[script_start..];
        let json_start = after_script
            .find('>')
            .ok_or_else(|| Error::decode("99qh __NEXT_DATA__ script malformed"))?
            + 1;
        let json_end = after_script
            .find("</script>")
            .ok_or_else(|| Error::decode("99qh __NEXT_DATA__ script not closed"))?;
        let json_str = &after_script[json_start..json_end];

        let data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("99qh JSON parse error: {e}")))?;

        let variety_list = data
            .get("props")
            .and_then(|p| p.get("pageProps"))
            .and_then(|p| p.get("data"))
            .and_then(|d| d.get("varietyListData"))
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("99qh data structure unexpected"))?;

        let mut products = Vec::new();
        for variety in variety_list {
            if let Some(list) = variety.get("productList").and_then(|l| l.as_array()) {
                for product in list {
                    let name = product
                        .get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("")
                        .to_string();
                    let id = product
                        .get("productId")
                        .map(std::string::ToString::to_string)
                        .unwrap_or_default();
                    if !name.is_empty() {
                        products.push((name, id));
                    }
                }
            }
        }

        if products.is_empty() {
            return Err(Error::not_found("99qh returned no product data"));
        }
        Ok(products)
    }

    async fn fetch_qh_token(&self) -> Result<String> {
        let resp = self
            .get("https://centerapi.fx168api.com/app/common/v.js")
            .header("Origin", "https://www.99qh.com")
            .header("Referer", "https://www.99qh.com")
            .send()
            .await
            .map_err(Error::from)?;

        let token = resp
            .headers()
            .get("_pcc")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        if token.is_empty() {
            return Err(Error::decode("99qh token extraction failed"));
        }
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qh_trend_item() {
        let j = serde_json::json!({"date": "2025-01-02", "fp": "3500", "sp": "3480"});
        let item: QhTrendItem = serde_json::from_value(j).unwrap();
        assert_eq!(item.date.unwrap(), "2025-01-02");
        assert_eq!(item.sp.unwrap(), "3480");
    }
}
