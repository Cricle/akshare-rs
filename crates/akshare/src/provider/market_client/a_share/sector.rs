use anyhow::{Context, bail};

use super::super::MarketDataClient;
use super::super::wire::{EastmoneyKlineEnvelope, EastmoneySectorConstituentEnvelope};
use crate::types::{CapitalFlowPoint, SectorConstituent, SectorSnapshot};

impl MarketDataClient {
    pub(crate) async fn fetch_a_share_sector_rankings_from_sina(
        &self,
        sector_type: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<SectorSnapshot>> {
        let indicator = match sector_type {
            "industry" => "industry",
            "concept" => "concept",
            other => bail!("unsupported sector_type: {}", other),
        };

        let sina_items = self
            .ak
            .stock_sector_spot(indicator)
            .await
            .context("failed to fetch sector rankings from Sina")?;

        let mut items: Vec<SectorSnapshot> = sina_items
            .into_iter()
            .map(|item| SectorSnapshot {
                sector_code: item.label.clone(),
                sector_name: item.sector,
                latest_index: 0.0,
                change_pct: item.change_pct.unwrap_or_default(),
                main_net_inflow: 0.0,
                main_net_inflow_ratio_pct: 0.0,
            })
            .collect();

        // Sort by change_pct descending (Sina doesn't have main_net_inflow)
        items.sort_by(|a, b| {
            b.change_pct
                .partial_cmp(&a.change_pct)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        items.truncate(limit);

        if items.is_empty() {
            bail!("sina returned no sector ranking items");
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
            .query(&crate::util::eastmoney_clist_params(
                &limit.to_string(),
                &[
                    ("fid", "f3"),
                    ("fs", &format!("b:{sector_code}")),
                    ("fields", "f12,f14,f2,f3,f62"),
                ],
            ))
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
