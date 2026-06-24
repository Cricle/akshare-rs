use anyhow::{bail, Context};

use crate::types::CapitalFlowPoint;
use super::super::MarketDataClient;
use super::super::wire::EastmoneyKlineEnvelope;

impl MarketDataClient {
    pub(crate) async fn fetch_a_share_capital_flow(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CapitalFlowPoint>> {
        let secid = self.eastmoney_secid(symbol)?;
        let response = self
            .http
            .get("https://push2his.eastmoney.com/api/qt/stock/fflow/daykline/get")
            .query(&[
                ("secid", secid.as_str()),
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
