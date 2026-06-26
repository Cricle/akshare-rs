//! Commodity spot prices from Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

impl AkShareClient {
    /// Fetch commodity spot prices.
    ///
    /// The Eastmoney datacenter report `RPT_SPOT_COMMODITY` has been retired.
    /// Uses Sina's commodity API to fetch real-time prices for major
    /// commodities (gold, silver, copper, crude oil, etc.).
    pub async fn commodity_spot_prices(&self, limit: usize) -> Result<Vec<MacroDataPoint>> {
        if limit == 0 {
            return Ok(Vec::new());
        }

        // Sina commodity symbols: gold, silver, copper, crude oil, etc.
        let sina_symbols = [
            ("hf_GC", "COMEX黄金"),
            ("hf_SI", "COMEX白银"),
            ("hf_HG", "COMEX铜"),
            ("hf_CL", "WTI原油"),
            ("hf_OIL", "布伦特原油"),
            ("hf_NG", "天然气"),
            ("hf_PL", "铂金"),
            ("hf_PA", "钯金"),
        ];

        let symbols_csv: Vec<&str> = sina_symbols.iter().map(|(s, _)| *s).collect();
        let url = format!("https://hq.sinajs.cn/list={}", symbols_csv.join(","));

        let body = self
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        let today = crate::util::today_iso();
        let mut items = Vec::new();
        for (i, line) in body.lines().enumerate() {
            if i >= sina_symbols.len() {
                break;
            }
            let data = line
                .split_once('=')
                .and_then(|(_, r)| r.trim_matches('"').split_once(';'))
                .map_or("", |(s, _)| s);
            if data.is_empty() {
                continue;
            }
            let fields: Vec<&str> = data.split(',').collect();
            if fields.len() < 10 {
                continue;
            }
            let (_, name) = sina_symbols[i];
            // Sina commodity format: [0]=name, [1]=price, ...
            let price = fields[0].parse::<f64>().unwrap_or(0.0);
            if price == 0.0 {
                continue;
            }
            items.push(MacroDataPoint {
                date: today.clone(),
                value: price,
                name: name.to_string(),
            });
        }

        items.truncate(limit);
        if items.is_empty() {
            return Err(crate::error::Error::not_found(
                "sina returned no commodity spot data",
            ));
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_spot_response() {
        // Simulate Eastmoney datacenter response.
        let json_str = r#"{
            "result": {
                "data": [
                    {
                        "REPORT_DATE": "2025-06-01 00:00:00",
                        "CLOSE_PRICE": 580.50,
                        "COMMODITY_NAME": "Au99.99"
                    },
                    {
                        "REPORT_DATE": "2025-06-01 00:00:00",
                        "CLOSE_PRICE": 7650.0,
                        "COMMODITY_NAME": "Ag(T+D)"
                    }
                ]
            }
        }"#;
        let resp: EmDatacenterResp = serde_json::from_str(json_str).unwrap();
        let data = resp.result.unwrap().data;
        assert_eq!(data.len(), 2);

        let first = &data[0];
        let date = first.str_field(&["REPORT_DATE"]).unwrap();
        assert_eq!(&date[..10], "2025-06-01");
        assert_eq!(first.f64_field(&["CLOSE_PRICE"]), Some(580.50));
    }

    #[test]
    fn test_parse_empty_response() {
        let json_str = r#"{"result": null}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json_str).unwrap();
        let data = resp.result.map(|r| r.data).unwrap_or_default();
        assert!(data.is_empty());
    }

    #[test]
    fn test_parse_fallback_fields() {
        // Ensure we fall back to alternative field names.
        let json_str = r#"{
            "result": {
                "data": [
                    {
                        "TRADE_DATE": "2025-05-30",
                        "LATEST_PRICE": 1024.0,
                        "NAME": "Copper"
                    }
                ]
            }
        }"#;
        let resp: EmDatacenterResp = serde_json::from_str(json_str).unwrap();
        let data = resp.result.unwrap().data;
        let v = &data[0];

        let date = v.str_or(&["REPORT_DATE", "TRADE_DATE"], "");
        assert_eq!(&date[..10], "2025-05-30");

        let value = v.f64_or(&["CLOSE_PRICE", "LATEST_PRICE"], 0.0);
        assert!((value - 1024.0).abs() < f64::EPSILON);

        let name = v.str_or(&["COMMODITY_NAME", "NAME"], "Commodity Spot");
        assert_eq!(name, "Copper");
    }
}
