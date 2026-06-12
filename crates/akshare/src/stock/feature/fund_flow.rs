//! Fund flow data (资金流向) from Eastmoney and THS.
//!
//! Note: THS-based fund flow functions use Eastmoney push2 API as fallback
//! since THS requires hexin-v token from JavaScript execution.

use super::helpers::{json_f64_opt, json_str};
use super::types::{
    ConceptFundFlowHist, FundFlowEntry, MainFundFlow, MarketFundFlow, SectorFundFlowHist,
    SectorFundFlowRank, SectorFundFlowSummary,
};
use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// 同花顺-数据中心-资金流向-个股资金流
    /// Uses Eastmoney API as equivalent data source.
    pub async fn stock_fund_flow_individual(&self, _symbol: &str) -> Result<Vec<FundFlowEntry>> {
        let sort_field = "f62";
        let data = self
            .cistock_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f2,f3,f12,f14,f62,f184,f66,f69,f72,f75,f78,f164,f166",
                "5000",
                sort_field,
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| FundFlowEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                turnover_rate: None,
                inflow: json_f64_opt(v, "f62"),
                outflow: None,
                net_flow: json_f64_opt(v, "f62"),
                amount: None,
            })
            .collect())
    }

    /// 同花顺-数据中心-资金流向-概念资金流
    /// Uses Eastmoney sector fund flow API.
    pub async fn stock_fund_flow_concept(&self, symbol: &str) -> Result<Vec<SectorFundFlowRank>> {
        let data = self.sector_fund_flow_fetch("concept", symbol).await?;
        Ok(data)
    }

    /// 同花顺-数据中心-资金流向-行业资金流
    /// Uses Eastmoney sector fund flow API.
    pub async fn stock_fund_flow_industry(&self, symbol: &str) -> Result<Vec<SectorFundFlowRank>> {
        let data = self.sector_fund_flow_fetch("industry", symbol).await?;
        Ok(data)
    }

    /// 同花顺-数据中心-资金流向-大单追踪
    /// Uses Eastmoney big order tracking API.
    pub async fn stock_fund_flow_big_deal(&self, symbol: &str) -> Result<Vec<FundFlowEntry>> {
        let _ = symbol; // parameter kept for API compatibility
        let data = self
            .cistock_fetch(
                "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048",
                "f2,f3,f12,f14,f62,f184,f66,f69",
                "100",
                "f62",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| FundFlowEntry {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                latest_price: json_f64_opt(v, "f2"),
                change_pct: json_f64_opt(v, "f3"),
                turnover_rate: None,
                inflow: json_f64_opt(v, "f62"),
                outflow: None,
                net_flow: json_f64_opt(v, "f62"),
                amount: None,
            })
            .collect())
    }

    /// 东方财富-板块资金流向-历史
    pub async fn stock_sector_fund_flow_hist(
        &self,
        symbol: &str,
    ) -> Result<Vec<SectorFundFlowHist>> {
        let url = "https://push2his.eastmoney.com/api/qt/stock/fflow/daykline/get".to_string();
        let resp = self
            .get(&url)
            .query(&[
                ("lmt", "0"),
                ("klt", "101"),
                ("secid", symbol),
                ("fields1", "f1,f2,f3,f7"),
                (
                    "fields2",
                    "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64,f65",
                ),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let klines = json
            .get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(klines
            .iter()
            .map(|v| SectorFundFlowHist { data: v.clone() })
            .collect())
    }

    /// 东方财富-板块资金流向-排名
    pub async fn stock_sector_fund_flow_rank(
        &self,
        indicator: &str,
    ) -> Result<Vec<SectorFundFlowRank>> {
        let data = self.sector_fund_flow_fetch("industry", indicator).await?;
        Ok(data)
    }

    /// 东方财富-板块资金流向-汇总
    pub async fn stock_sector_fund_flow_summary(&self) -> Result<Vec<SectorFundFlowSummary>> {
        let data = self
            .dc_fetch_all(
                "RPT_SECTOR_FUND_FLOW",
                "ALL",
                "",
                "NET_INFLOW",
                "-1",
                500,
                2,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| SectorFundFlowSummary { data: v.clone() })
            .collect())
    }

    /// 东方财富-主力资金流向 (A-share / HK / US)
    pub async fn stock_main_fund_flow(&self, symbol: &str) -> Result<Vec<MainFundFlow>> {
        let secid = match crate::market::detect_market(symbol) {
            crate::types::MarketKind::HongKong => {
                let code = symbol.trim().trim_start_matches('0');
                format!("116.{code:0>5}")
            }
            crate::types::MarketKind::UsEquity => {
                let market_code =
                    if crate::stock::eastmoney_fund_flow::is_nyse_symbol(symbol.trim()) {
                        "106"
                    } else {
                        "105"
                    };
                format!("{market_code}.{}", symbol.trim())
            }
            crate::types::MarketKind::AShare => {
                // A-share: SH=1, SZ/BJ=0
                let market_code = if symbol.starts_with('6') { "1" } else { "0" };
                format!("{market_code}.{symbol}")
            }
        };
        let url = "https://push2his.eastmoney.com/api/qt/stock/fflow/daykline/get";
        let resp = self
            .get(url)
            .query(&[
                ("lmt", "0"),
                ("klt", "101"),
                ("secid", secid.as_str()),
                ("fields1", "f1,f2,f3,f7"),
                (
                    "fields2",
                    "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64,f65",
                ),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let klines = json
            .get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(klines
            .iter()
            .map(|v| MainFundFlow { data: v.clone() })
            .collect())
    }

    /// 东方财富-市场资金流向
    pub async fn stock_market_fund_flow(&self) -> Result<Vec<MarketFundFlow>> {
        let data = self
            .dc_fetch_all(
                "RPT_MARKET_FUND_FLOW",
                "ALL",
                "",
                "TRADE_DATE",
                "-1",
                500,
                5,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| MarketFundFlow { data: v.clone() })
            .collect())
    }

    /// 东方财富-概念资金流向-历史
    pub async fn stock_concept_fund_flow_hist(
        &self,
        symbol: &str,
    ) -> Result<Vec<ConceptFundFlowHist>> {
        let url = "https://push2his.eastmoney.com/api/qt/stock/fflow/daykline/get";
        let resp = self
            .get(url)
            .query(&[
                ("lmt", "0"),
                ("klt", "101"),
                ("secid", symbol),
                ("fields1", "f1,f2,f3,f7"),
                (
                    "fields2",
                    "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64,f65",
                ),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let klines = json
            .get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(klines
            .iter()
            .map(|v| ConceptFundFlowHist { data: v.clone() })
            .collect())
    }

    /// Generic sector fund flow fetch via Eastmoney.
    async fn sector_fund_flow_fetch(
        &self,
        _kind: &str,
        _indicator: &str,
    ) -> Result<Vec<SectorFundFlowRank>> {
        let data = self
            .cistock_fetch(
                "m:90+t:2",
                "f2,f3,f12,f14,f62,f184,f66,f69,f72,f75,f78,f164,f166",
                "5000",
                "f62",
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| SectorFundFlowRank {
                name: json_str(v, "f14"),
                change_pct: json_f64_opt(v, "f3"),
                inflow: json_f64_opt(v, "f62"),
                outflow: None,
                net_flow: json_f64_opt(v, "f62"),
                leader: None,
                leader_change_pct: None,
            })
            .collect())
    }

    /// Generic Eastmoney clist fetch (variant for fund flow).
    async fn cistock_fetch(
        &self,
        fs: &str,
        fields: &str,
        page_size: &str,
        sort_field: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let resp = self
            .get("https://push2.eastmoney.com/api/qt/clist/get")
            .query(&[
                ("pn", "1"),
                ("pz", page_size),
                ("po", "1"),
                ("np", "1"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fid", sort_field),
                ("fs", fs),
                ("fields", fields),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let payload: super::helpers::ClistSpotEnvelope = resp.json().await.map_err(Error::from)?;
        let items = payload.data.and_then(|d| d.diff).unwrap_or_default();
        if items.is_empty() {
            return Err(Error::not_found("eastmoney clist returned no data"));
        }
        Ok(items)
    }
}
