use super::super::{
    MarginAccountInfo, MarginRatioPa, MarginSseDetail, MarginSseSummary, MarginSzseDetail,
    MarginSzseSummary, MarketDataClient, FUNDAMENTALS_CACHE_TTL_SECS, INSIDER_CACHE_TTL_SECS,
    MARKET_DATA_CACHE_PREFIX,
};

impl MarketDataClient {
    // -----------------------------------------------------------------------
    // Margin Trading (融资融券)
    // -----------------------------------------------------------------------

    pub async fn fetch_margin_account_info(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> anyhow::Result<Vec<MarginAccountInfo>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:margin-account:{}:{}",
                start_date, end_date
            ),
            INSIDER_CACHE_TTL_SECS,
            || {
                super::super::akshare_rust::a_share::fetch_margin_account_info(
                    self, start_date, end_date,
                )
            },
        )
        .await
    }

    pub async fn fetch_margin_sse_detail(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<MarginSseDetail>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:margin-sse-detail:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_margin_sse_detail(self, date),
        )
        .await
    }

    pub async fn fetch_margin_szse_detail(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<MarginSzseDetail>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:margin-szse-detail:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_margin_szse_detail(self, date),
        )
        .await
    }

    pub async fn fetch_margin_ratio_pa(
        &self,
        symbol: &str,
        date: &str,
    ) -> anyhow::Result<Vec<MarginRatioPa>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:margin-ratio-pa:{}:{}",
                symbol.trim(),
                date
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_margin_ratio_pa(self, symbol, date),
        )
        .await
    }

    pub async fn fetch_margin_sse_summary(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> anyhow::Result<Vec<MarginSseSummary>> {
        self.cached_fetch(
            &format!(
                "{MARKET_DATA_CACHE_PREFIX}:margin-sse-summary:{}:{}",
                start_date, end_date
            ),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || {
                super::super::akshare_rust::a_share::fetch_margin_sse_summary(
                    self, start_date, end_date,
                )
            },
        )
        .await
    }

    pub async fn fetch_margin_szse_summary(
        &self,
        date: &str,
    ) -> anyhow::Result<Vec<MarginSzseSummary>> {
        self.cached_fetch(
            &format!("{MARKET_DATA_CACHE_PREFIX}:margin-szse-summary:{}", date),
            FUNDAMENTALS_CACHE_TTL_SECS,
            || super::super::akshare_rust::a_share::fetch_margin_szse_summary(self, date),
        )
        .await
    }
}
