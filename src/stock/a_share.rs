use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::market::*;
use crate::types::*;
use crate::util::*;

impl AkShareClient {
    /// Get A-share quote with fallback: Tencent -> Sina realtime -> Tushare daily
    pub async fn a_share_quote(&self, symbol: &str) -> Result<QuoteSnapshot> {
        let ts_code = normalize_a_share_symbol(symbol)
            .ok_or_else(|| Error::invalid_input(format!("invalid A-share symbol: {symbol}")))?;

        // Try Tencent first
        if let Ok(quote) = self.tencent_a_share_quote(symbol).await { return Ok(quote) }

        // Fallback to Sina realtime
        if let Ok(quote) = self.sina_a_share_realtime(symbol).await { return Ok(quote) }

        // Fallback to Tushare (convert last candle to quote)
        let candle = self
            .tushare_daily(&ts_code, &days_ago_yyyymmdd(14), &today_yyyymmdd())
            .await?
            .into_iter()
            .last()
            .ok_or_else(|| Error::upstream("all quote providers failed"))?;
        Ok(QuoteSnapshot {
            symbol: ts_code,
            date: candle.trade_date,
            open: candle.open,
            high: candle.high,
            low: candle.low,
            close: candle.close,
            volume: candle.volume,
        })
    }

    /// Get A-share candles with fallback: Tencent klines -> Eastmoney klines -> Tushare daily
    pub async fn a_share_candles(
        &self,
        symbol: &str,
        adjust: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        // Try Tencent
        match self.tencent_a_share_candles(symbol, adjust, limit).await {
            Ok(items) if !items.is_empty() => return Ok(items),
            _ => {}
        }

        // Fallback to Eastmoney
        if let Ok(secid) = eastmoney_secid(symbol) {
            match self.eastmoney_klines(&secid, adjust, limit).await {
                Ok(items) if !items.is_empty() => return Ok(items),
                _ => {}
            }
        }

        // Fallback to Tushare
        let ts_code = normalize_a_share_symbol(symbol)
            .ok_or_else(|| Error::invalid_input(format!("invalid A-share symbol: {symbol}")))?;
        let mut items = self
            .tushare_daily(&ts_code, &days_ago_yyyymmdd(limit as i64 * 2), &today_yyyymmdd())
            .await?;
        if items.len() > limit {
            items = items[items.len() - limit..].to_vec();
        }
        if items.is_empty() {
            return Err(Error::upstream("all candle providers failed"));
        }
        Ok(items)
    }

    /// Search A-share stocks via Eastmoney
    pub async fn a_share_search(
        &self,
        query: &str,
        market: Option<&str>,
        limit: usize,
    ) -> Result<Vec<StockSearchResult>> {
        self.eastmoney_search(query, market, limit).await
    }

    /// Capital flow for A-share stock
    pub async fn a_share_capital_flow(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<CapitalFlowPoint>> {
        let secid = eastmoney_secid(symbol)?;
        self.eastmoney_capital_flow(&secid, limit).await
    }

    /// Sector rankings
    pub async fn a_share_sector_rankings(
        &self,
        sector_type: &str,
        limit: usize,
    ) -> Result<Vec<SectorSnapshot>> {
        self.eastmoney_sector_rankings(sector_type, limit).await
    }

    /// Sector constituents
    pub async fn a_share_sector_constituents(
        &self,
        sector_code: &str,
        limit: usize,
    ) -> Result<Vec<SectorConstituent>> {
        self.eastmoney_sector_constituents(sector_code, limit).await
    }

    /// Sector capital flow
    pub async fn a_share_sector_capital_flow(
        &self,
        sector_code: &str,
        limit: usize,
    ) -> Result<Vec<CapitalFlowPoint>> {
        self.eastmoney_sector_capital_flow(sector_code, limit).await
    }

    /// Billboard (龙虎榜)
    pub async fn a_share_billboard(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<BillboardEntry>> {
        self.eastmoney_billboard(symbol, limit).await
    }

    /// Billboard seats
    pub async fn a_share_billboard_seats(
        &self,
        symbol: &str,
        side: &str,
        limit: usize,
    ) -> Result<Vec<BillboardSeatDetail>> {
        self.eastmoney_billboard_seats(symbol, side, limit).await
    }

    /// Announcements
    pub async fn a_share_announcements(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<AnnouncementItem>> {
        self.eastmoney_announcements(symbol, limit).await
    }

    /// Announcement detail
    pub async fn a_share_announcement_detail(&self, art_code: &str) -> Result<AnnouncementDetail> {
        self.eastmoney_announcement_detail(art_code).await
    }

    /// Trade calendar via Tushare
    pub async fn a_share_trade_calendar(
        &self,
        exchange: &str,
        start: &str,
        end: &str,
    ) -> Result<Vec<TradeCalendarItem>> {
        self.tushare_trade_calendar(exchange, start, end).await
    }
}
