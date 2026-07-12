use anyhow::{Context, bail};

use super::super::MarketDataClient;
use super::super::wire::EastmoneyKlineEnvelope;
use crate::types::CapitalFlowPoint;

impl MarketDataClient {
    pub(crate) async fn fetch_a_share_capital_flow(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CapitalFlowPoint>> {
        let secid = self.eastmoney_secid(symbol)?;
        let result = self.fetch_a_share_capital_flow_primary(&secid, limit).await;
        match result {
            Ok(items) => Ok(items),
            Err(_) => {
                // Fallback: use clist endpoint for current-day data
                let code = symbol
                    .split_once('.')
                    .map(|(c, _)| c)
                    .unwrap_or(symbol)
                    .to_string();
                self.fetch_capital_flow_from_clist(&code, "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23")
                    .await
            }
        }
    }

    async fn fetch_a_share_capital_flow_primary(
        &self,
        secid: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CapitalFlowPoint>> {
        let response = self
            .http
            .get("https://push2his.eastmoney.com/api/qt/stock/fflow/daykline/get")
            .query(&[
                ("secid", secid),
                ("klt", "101"),
                ("lmt", &limit.to_string()),
                ("fields1", "f1,f2,f3,f7"),
                (
                    "fields2",
                    "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63",
                ),
            ])
            .send()
            .await
            .context("failed to fetch A-share capital flow from Eastmoney")?
            .error_for_status()
            .context("eastmoney capital flow request failed")?;
        let payload: EastmoneyKlineEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney capital flow response")?;
        let data = payload
            .data
            .context("eastmoney capital flow response missing data")?;
        let klines = data
            .klines
            .context("eastmoney capital flow response missing klines")?;
        let mut items = klines
            .into_iter()
            .map(|line| Self::parse_a_share_capital_flow_line(&line))
            .collect::<anyhow::Result<Vec<_>>>()?;
        if items.is_empty() {
            bail!("eastmoney returned no capital flow items");
        }
        items.truncate(limit);
        Ok(items)
    }
}
