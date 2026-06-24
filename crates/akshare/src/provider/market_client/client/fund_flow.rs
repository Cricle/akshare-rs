use super::super::{
    CANDLES_CACHE_TTL_SECS, FundFlowEntry, MARKET_DATA_CACHE_PREFIX, MainFundFlow,
    MarketDataClient, SectorFundFlowRank,
};

impl MarketDataClient {
    // -----------------------------------------------------------------------
    // Fund Flow (资金流向)
    // -----------------------------------------------------------------------

    pub async fn fetch_fund_flow_individual(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<FundFlowEntry>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:fund-flow-individual:{}",
                symbol.trim()
            ),
            CANDLES_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_fund_flow_individual(self, symbol),
        )
        .await
    }

    pub async fn fetch_fund_flow_concept(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<SectorFundFlowRank>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:fund-flow-concept:{}",
                symbol.trim()
            ),
            CANDLES_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_fund_flow_concept(self, symbol),
        )
        .await
    }

    pub async fn fetch_fund_flow_industry(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<SectorFundFlowRank>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:fund-flow-industry:{}",
                symbol.trim()
            ),
            CANDLES_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_fund_flow_industry(self, symbol),
        )
        .await
    }

    pub async fn fetch_main_fund_flow(&self, symbol: &str) -> anyhow::Result<Vec<MainFundFlow>> {
        let normalized = self
            .normalize_a_share_symbol(symbol)
            .ok_or_else(|| anyhow::anyhow!("invalid A-share symbol for main fund flow"))?;
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:main-fund-flow:{}", normalized),
            CANDLES_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_main_fund_flow(self, symbol),
        )
        .await
    }
}
