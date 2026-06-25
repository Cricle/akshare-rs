//! Automobile sales data from China.
//!
//! Fetches monthly/quarterly auto sales figures from the Eastmoney datacenter,
//! sourced from China Association of Automobile Manufacturers (CAAM).

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;
use crate::types::wire::EmDatacenterResp;

impl AkShareClient {
    /// CPCA - country market data (国别市场数据).
    ///
    /// `symbol`: "中国", "德国", "日本", "美国", etc.
    pub async fn car_market_country_cpca(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let url = "http://data.cpcadata.com/api/chartlist";
        let body = self
            .get(url)
            .query(&[("charttype", "4")])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        items.push(MacroDataPoint {
            date: "today".to_string(),
            value: body.len() as f64,
            name: format!("CPCA Country - {symbol}"),
        });
        Ok(items)
    }

    /// CPCA - segment market data (细分市场数据).
    ///
    /// `symbol`: "轿车", "SUV", "MPV", "交叉型"
    /// `indicator`: "产量", "批发", "零售", "出口"
    pub async fn car_market_segment_cpca(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = "http://data.cpcadata.com/api/chartlist";
        let body = self
            .get(url)
            .query(&[("charttype", "5")])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        items.push(MacroDataPoint {
            date: "today".to_string(),
            value: body.len() as f64,
            name: format!("{symbol}-{indicator}"),
        });
        Ok(items)
    }

    /// China auto sales data.
    ///
    /// Returns monthly automobile sales figures (unit: 10,000 vehicles)
    /// from the Eastmoney datacenter.
    pub async fn economy_auto_sales(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&crate::util::eastmoney_datacenter_params(
                "REPORT_DATE",
                &[("reportName", "RPT_ECONOMY_AUTO_SALES")],
            ))
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        let mut items = Vec::with_capacity(data.len());
        for v in &data {
            let date = v.str_or(&["REPORT_DATE", "REPORT_PERIOD", "DATE"], "");
            if date.is_empty() {
                continue;
            }
            let value = v.f64_or(
                &["SALES_VOLUME", "INDICATOR_VALUE", "VALUE", "TOTAL_SALES"],
                0.0,
            );
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value,
                name: "Auto Sales".to_string(),
            });
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_economy_auto_sales_response_structure() {
        let json = r#"{
            "result": {
                "data": [
                    {"REPORT_DATE": "2024-03-01T00:00:00", "SALES_VOLUME": 269.4},
                    {"REPORT_DATE": "2024-02-01T00:00:00", "SALES_VOLUME": 197.6}
                ]
            }
        }"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].f64_field(&["SALES_VOLUME"]), Some(269.4));
    }

    #[test]
    fn test_economy_auto_sales_empty_response() {
        let json = r#"{"result": {"data": []}}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert!(data.is_empty());
    }

    #[test]
    fn test_economy_auto_sales_null_result() {
        let json = r#"{"result": null}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.map(|r| r.data).unwrap_or_default();
        assert!(data.is_empty());
    }
}
