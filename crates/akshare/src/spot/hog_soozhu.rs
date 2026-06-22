//! Hog spot price data from Soozhu (搜猪).

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

#[derive(Debug, Deserialize)]
struct SoozhuPriceResp {
    vlist: Option<Vec<SoozhuValueItem>>,
    nationlist: Option<Vec<Vec<serde_json::Value>>>,
    datalist: Option<Vec<Vec<serde_json::Value>>>,
}

#[derive(Debug, Deserialize)]
struct SoozhuValueItem {
    name: Option<String>,
    value: Option<Vec<serde_json::Value>>,
}

impl AkShareClient {
    /// Fetch hog spot prices by province from Soozhu.
    ///
    /// Returns current average prices and daily change percentages per province.
    pub async fn spot_hog_soozhu(&self) -> Result<Vec<MacroDataPoint>> {
        let session = self.fetch_soozhu_session().await?;
        let resp: SoozhuPriceResp = self
            .post("https://www.soozhu.com/price/data/center/")
            .form(&[("act", "mapdata"), ("csrfmiddlewaretoken", &session)])
            .send()
            .await?
            .json()
            .await?;

        let vlist = resp.vlist.unwrap_or_default();
        if vlist.is_empty() {
            return Err(Error::not_found("soozhu returned no hog price data"));
        }

        let items: Vec<MacroDataPoint> = vlist
            .into_iter()
            .filter_map(|item| {
                let name = item.name?;
                let vals = item.value?;
                let price = vals.first()?.as_f64()?;
                Some(MacroDataPoint {
                    date: crate::util::today_iso(),
                    value: price,
                    name,
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("no valid hog price data parsed"));
        }
        Ok(items)
    }

    /// Fetch year-to-date national hog average price trend from Soozhu.
    pub async fn spot_hog_year_trend_soozhu(&self) -> Result<Vec<MacroDataPoint>> {
        self.fetch_soozhu_trend("yeartrend", "全国出栏均价").await
    }

    /// Fetch national lean hog price trend from Soozhu.
    pub async fn spot_hog_lean_price_soozhu(&self) -> Result<Vec<MacroDataPoint>> {
        self.fetch_soozhu_price_trend("", "瘦肉型肉猪").await
    }

    /// Fetch national three-way piglet price trend from Soozhu.
    pub async fn spot_hog_three_way_soozhu(&self) -> Result<Vec<MacroDataPoint>> {
        self.fetch_soozhu_price_trend("4", "三元仔猪").await
    }

    /// Fetch national crossbred sow price trend from Soozhu.
    pub async fn spot_hog_crossbred_soozhu(&self) -> Result<Vec<MacroDataPoint>> {
        self.fetch_soozhu_price_trend("6", "后备二元母猪").await
    }

    /// Fetch national corn price trend from Soozhu.
    pub async fn spot_corn_price_soozhu(&self) -> Result<Vec<MacroDataPoint>> {
        self.fetch_soozhu_price_trend("8", "玉米价格").await
    }

    /// Fetch national soybean meal price trend from Soozhu.
    pub async fn spot_soybean_price_soozhu(&self) -> Result<Vec<MacroDataPoint>> {
        self.fetch_soozhu_price_trend("9", "豆粕价格").await
    }

    /// Fetch national mixed feed price trend from Soozhu.
    pub async fn spot_mixed_feed_soozhu(&self) -> Result<Vec<MacroDataPoint>> {
        self.fetch_soozhu_price_trend("11", "育肥猪合料").await
    }

    async fn fetch_soozhu_session(&self) -> Result<String> {
        // Soozhu requires CSRF token from page load
        let resp = self
            .get("https://www.soozhu.com/price/data/center/")
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Extract CSRF token from HTML
        let token = extract_csrf_token(&resp).unwrap_or_default();
        Ok(token)
    }

    async fn fetch_soozhu_trend(&self, act: &str, label: &str) -> Result<Vec<MacroDataPoint>> {
        let session = self.fetch_soozhu_session().await?;
        let resp: SoozhuPriceResp = self
            .post("https://www.soozhu.com/price/data/center/")
            .form(&[("act", act), ("csrfmiddlewaretoken", &session)])
            .send()
            .await?
            .json()
            .await?;

        let list = resp.nationlist.unwrap_or_default();
        if list.is_empty() {
            return Err(Error::not_found(format!("soozhu returned no {label} data")));
        }

        let items: Vec<MacroDataPoint> = list
            .into_iter()
            .filter_map(|row| {
                if row.len() < 2 {
                    return None;
                }
                let date = row[0].as_str()?.to_string();
                let price = row[1].as_f64()?;
                Some(MacroDataPoint {
                    date: date.get(..10).unwrap_or(&date).to_string(),
                    value: price,
                    name: label.to_string(),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(format!("no valid {label} data")));
        }
        Ok(items)
    }

    async fn fetch_soozhu_price_trend(
        &self,
        indid: &str,
        label: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let session = self.fetch_soozhu_session().await?;
        let resp: SoozhuPriceResp = self
            .post("https://www.soozhu.com/price/data/center/")
            .form(&[
                ("act", "pricetrend"),
                ("indid", indid),
                ("csrfmiddlewaretoken", &session),
            ])
            .send()
            .await?
            .json()
            .await?;

        let list = resp.datalist.unwrap_or_default();
        if list.is_empty() {
            return Err(Error::not_found(format!("soozhu returned no {label} data")));
        }

        let items: Vec<MacroDataPoint> = list
            .into_iter()
            .filter_map(|row| {
                if row.len() < 2 {
                    return None;
                }
                let date = row[0].as_str()?.to_string();
                let price = row[1].as_f64()?;
                Some(MacroDataPoint {
                    date: date.get(..10).unwrap_or(&date).to_string(),
                    value: price,
                    name: label.to_string(),
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(format!("no valid {label} data")));
        }
        Ok(items)
    }
}

fn extract_csrf_token(html: &str) -> Option<String> {
    let marker = "csrfmiddlewaretoken";
    let pos = html.find(marker)?;
    let after = &html[pos..];
    let value_start = after.find("value=\"")? + 7;
    let after_val = &after[value_start..];
    let value_end = after_val.find('"')?;
    Some(after_val[..value_end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_csrf_token() {
        let html = r#"<input name="csrfmiddlewaretoken" value="abc123def">"#;
        assert_eq!(extract_csrf_token(html), Some("abc123def".to_string()));
    }

    #[test]
    fn test_extract_csrf_token_missing() {
        assert_eq!(extract_csrf_token("<html>no token</html>"), None);
    }
}
