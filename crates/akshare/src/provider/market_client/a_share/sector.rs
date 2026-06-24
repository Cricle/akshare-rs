use anyhow::{Context, bail};

use super::super::MarketDataClient;
use super::super::wire::{
    EastmoneyKlineEnvelope, EastmoneySectorConstituentEnvelope, EastmoneySectorRankingEnvelope,
};
use crate::types::{CapitalFlowPoint, SectorConstituent, SectorSnapshot};

impl MarketDataClient {
    pub(crate) async fn fetch_a_share_sector_rankings_from_eastmoney(
        &self,
        sector_type: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<SectorSnapshot>> {
        let fs = match sector_type {
            "industry" => "m:90+t:2",
            "concept" => "m:90+t:3",
            other => bail!("unsupported sector_type: {}", other),
        };
        let response = self
            .http
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", &limit.to_string()),
                ("po", "1"),
                ("np", "1"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f62"),
                ("fs", fs),
                ("fields", "f12,f14,f2,f3,f62,f184"),
            ])
            .send()
            .await
            .context("failed to fetch A-share sector rankings from Eastmoney")?
            .error_for_status()
            .context("eastmoney sector ranking request failed")?;
        let payload: EastmoneySectorRankingEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney sector ranking response")?;
        let items = payload
            .data
            .and_then(|data| data.diff)
            .unwrap_or_default()
            .into_iter()
            .map(|item| SectorSnapshot {
                sector_code: item.sector_code.unwrap_or_default(),
                sector_name: item.sector_name.unwrap_or_else(|| "未知板块".to_string()),
                latest_index: item.latest_index.unwrap_or_default(),
                change_pct: item.change_pct.unwrap_or_default(),
                main_net_inflow: item.main_net_inflow.unwrap_or_default(),
                main_net_inflow_ratio_pct: item.main_net_inflow_ratio_pct.unwrap_or_default(),
            })
            .collect::<Vec<_>>();
        if items.is_empty() {
            bail!("eastmoney returned no sector ranking items");
        }
        Ok(items)
    }

    pub(crate) async fn fetch_a_share_sector_constituents_from_eastmoney(
        &self,
        sector_code: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<SectorConstituent>> {
        let response = self
            .http
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", &limit.to_string()),
                ("po", "1"),
                ("np", "1"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", "f3"),
                ("fs", &format!("b:{sector_code}")),
                ("fields", "f12,f14,f2,f3,f62"),
            ])
            .send()
            .await
            .context("failed to fetch A-share sector constituents from Eastmoney")?
            .error_for_status()
            .context("eastmoney sector constituent request failed")?;
        let payload: EastmoneySectorConstituentEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney sector constituent response")?;
        let items = payload
            .data
            .and_then(|data| data.diff)
            .unwrap_or_default()
            .into_iter()
            .map(|item| SectorConstituent {
                symbol: item.symbol.unwrap_or_default(),
                name: item.name.unwrap_or_else(|| "未知成分股".to_string()),
                latest_price: item.latest_price.unwrap_or_default(),
                change_pct: item.change_pct.unwrap_or_default(),
                main_net_inflow: item.main_net_inflow,
            })
            .collect::<Vec<_>>();
        if items.is_empty() {
            bail!("eastmoney returned no sector constituent items");
        }
        Ok(items)
    }

    pub(crate) async fn fetch_a_share_sector_capital_flow_from_eastmoney(
        &self,
        sector_code: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CapitalFlowPoint>> {
        let secid = format!("90.{}", sector_code.trim().to_uppercase());
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
            .context("failed to fetch A-share sector capital flow from Eastmoney")?
            .error_for_status()
            .context("eastmoney sector capital flow request failed")?;
        let payload: EastmoneyKlineEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney sector capital flow response")?;
        let data = payload
            .data
            .context("eastmoney sector capital flow response missing data")?;
        let klines = data
            .klines
            .context("eastmoney sector capital flow response missing klines")?;
        let items = klines
            .into_iter()
            .map(|line| Self::parse_a_share_capital_flow_line(&line))
            .collect::<anyhow::Result<Vec<_>>>()?;
        if items.is_empty() {
            bail!("eastmoney returned no sector capital flow items");
        }
        Ok(items)
    }
}
