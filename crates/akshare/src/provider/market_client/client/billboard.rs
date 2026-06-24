use super::super::{
    BillboardEntry, BillboardSeatDetail, DataError, DataErrorKind, MarketDataClient, MarketKind,
    INSIDER_CACHE_TTL_SECS, MARKET_DATA_CACHE_PREFIX, NEWS_CACHE_TTL_SECS,
};

use super::super::{
    BillboardActiveBranch, BillboardBranchDetail, BillboardBranchRanking, BillboardDetail,
    BillboardOrgSeatTracking, BillboardOrgTradeSummary, BillboardStockDetail,
    BillboardStockDetailDate, BillboardStockStatistic, BillboardTraderStatistic,
};

impl MarketDataClient {
    pub async fn fetch_billboard_entries(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<BillboardEntry>> {
        if self.normalize_a_share_symbol(symbol).is_some() {
            let normalized = self.cache_symbol(symbol, MarketKind::AShare);
            return self
                .cached_fetch(
                    &format!(
                        "{MARKET_DATA_CACHE_PREFIX}:billboard-entries:{}:{}",
                        normalized, limit
                    ),
                    NEWS_CACHE_TTL_SECS,
                    || self.fetch_a_share_billboard_entries(symbol, limit),
                )
                .await;
        }

        Err(DataError::new(
            DataErrorKind::UnsupportedMarket,
            format!("billboard is unsupported for symbol {symbol}"),
        )
        .into())
    }

    pub async fn fetch_billboard_seats(
        &self,
        symbol: &str,
        side: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<BillboardSeatDetail>> {
        if self.normalize_a_share_symbol(symbol).is_some() {
            let normalized = self.cache_symbol(symbol, MarketKind::AShare);
            return self
                .cached_fetch(
                    &format!(
                        "{MARKET_DATA_CACHE_PREFIX}:billboard-seats:{}:{}:{}",
                        normalized,
                        side.trim().to_lowercase(),
                        limit
                    ),
                    NEWS_CACHE_TTL_SECS,
                    || self.fetch_a_share_billboard_seats(symbol, side, limit),
                )
                .await;
        }

        Err(DataError::new(
            DataErrorKind::UnsupportedMarket,
            format!("billboard seats are unsupported for symbol {symbol}"),
        )
        .into())
    }

    // -----------------------------------------------------------------------
    // Billboard / Dragon Tiger List (龙虎榜)
    // -----------------------------------------------------------------------

    pub async fn fetch_lhb_detail(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> anyhow::Result<Vec<BillboardDetail>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:lhb-detail:{}:{}", start_date, end_date),
            INSIDER_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_billboard_detail(self, start_date, end_date),
        )
        .await
    }

    pub async fn fetch_lhb_stock_statistic(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<BillboardStockStatistic>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:lhb-stock-stat:{}", symbol.trim()),
            INSIDER_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_billboard_stock_statistic(self, symbol),
        )
        .await
    }

    pub async fn fetch_lhb_jgmmtj(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> anyhow::Result<Vec<BillboardOrgTradeSummary>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:lhb-jgmmtj:{}:{}", start_date, end_date),
            INSIDER_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_billboard_jgmmtj(self, start_date, end_date),
        )
        .await
    }

    pub async fn fetch_lhb_jgstatistic(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<BillboardOrgSeatTracking>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:lhb-jgstat:{}", symbol.trim()),
            INSIDER_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_billboard_jgstatistic(self, symbol),
        )
        .await
    }

    pub async fn fetch_lhb_hyyyb(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> anyhow::Result<Vec<BillboardActiveBranch>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:lhb-hyyyb:{}:{}", start_date, end_date),
            INSIDER_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_billboard_hyyyb(self, start_date, end_date),
        )
        .await
    }

    pub async fn fetch_lhb_yybph(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<BillboardBranchRanking>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:lhb-yybph:{}", symbol.trim()),
            INSIDER_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_billboard_yybph(self, symbol),
        )
        .await
    }

    pub async fn fetch_lhb_trader_statistic(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<BillboardTraderStatistic>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:lhb-trader:{}", symbol.trim()),
            INSIDER_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_billboard_trader_statistic(self, symbol),
        )
        .await
    }

    pub async fn fetch_lhb_stock_detail_date(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<BillboardStockDetailDate>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:lhb-stock-date:{}", symbol.trim()),
            INSIDER_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_billboard_stock_detail_date(self, symbol),
        )
        .await
    }

    pub async fn fetch_lhb_stock_detail(
        &self,
        symbol: &str,
        date: &str,
        flag: &str,
    ) -> anyhow::Result<Vec<BillboardStockDetail>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:lhb-stock-detail:{}:{}:{}",
                symbol.trim(),
                date,
                flag
            ),
            INSIDER_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_billboard_stock_detail(self, symbol, date, flag),
        )
        .await
    }

    pub async fn fetch_lhb_yyb_detail(
        &self,
        symbol: &str,
    ) -> anyhow::Result<Vec<BillboardBranchDetail>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:lhb-yyb-detail:{}", symbol.trim()),
            INSIDER_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_billboard_yyb_detail(self, symbol),
        )
        .await
    }
}
