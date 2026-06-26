//! Energy commodity data: oil prices and carbon trading from Eastmoney.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;
use crate::types::wire::EmDatacenterResp;

impl AkShareClient {
    /// Historical oil price data (Brent, WTI).
    ///
    /// The Eastmoney datacenter report `RPT_ECONOMY_OIL_PRICE` has been retired.
    /// Uses Sina's commodity API to fetch current oil prices.
    pub async fn energy_oil_hist(&self) -> Result<Vec<MacroDataPoint>> {
        let sina_symbols = [("hf_CL", "WTI原油"), ("hf_OIL", "布伦特原油")];
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
            if fields.is_empty() {
                continue;
            }
            let (_, name) = sina_symbols[i];
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

        if items.is_empty() {
            return Err(crate::error::Error::not_found(
                "sina returned no oil price data",
            ));
        }
        Ok(items)
    }

    /// China domestic carbon trading prices from Eastmoney.
    pub async fn energy_carbon_domestic(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&crate::util::eastmoney_datacenter_params(
                "TRADE_DATE",
                &[("reportName", "RPT_CARBON_TRADING")],
            ))
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        let mut items = Vec::with_capacity(data.len());
        for v in &data {
            let date = v.str_or(&["TRADE_DATE"], "");
            if date.is_empty() {
                continue;
            }
            let value = v.f64_or(&["CLOSE_PRICE", "PRICE"], 0.0);
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value,
                name: "Carbon Trading".to_string(),
            });
        }
        Ok(items)
    }
}
