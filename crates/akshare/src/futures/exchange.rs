//! Exchange-level get_* wrapper functions for futures data.
//!
//! These provide Python-compatible function names that delegate to the
//! underlying exchange-specific implementations.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::{FuturesDailyBar, FuturesPositionRank, Row};

impl AkShareClient {
    /// Get CFFEX daily bar data for a given date.
    pub async fn get_cffex_daily(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        self.futures_daily_cffex(date).await
    }

    /// Get CFFEX rank table (position ranking) for a given date and symbol.
    pub async fn get_cffex_rank_table(
        &self,
        date: &str,
        _symbol: &str,
    ) -> Result<Vec<FuturesPositionRank>> {
        self.futures_cffex_position_rank(date).await
    }

    /// Get CZCE daily bar data for a given date.
    pub async fn get_czce_daily(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        self.futures_daily_czce(date).await
    }

    /// Get DCE daily bar data for a given date.
    ///
    /// `vars_list` is an optional filter for varieties (currently unused;
    /// returns all varieties).
    pub async fn get_dce_daily(
        &self,
        date: &str,
        _vars_list: Option<&str>,
    ) -> Result<Vec<FuturesDailyBar>> {
        self.futures_daily_dce(date).await
    }

    /// Get DCE rank table (position ranking) for a given date and symbol.
    pub async fn get_dce_rank_table(
        &self,
        date: &str,
        _symbol: &str,
    ) -> Result<Vec<FuturesPositionRank>> {
        self.futures_dce_position_rank(date).await
    }

    /// Get GFEX daily bar data for a given date.
    pub async fn get_gfex_daily(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        self.futures_daily_gfex(date).await
    }

    /// Get INE daily bar data for a given date.
    pub async fn get_ine_daily(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        self.futures_daily_ine(date).await
    }

    /// Get SHFE daily bar data for a given date.
    pub async fn get_shfe_daily(&self, date: &str) -> Result<Vec<FuturesDailyBar>> {
        self.futures_daily_shfe(date).await
    }

    /// Get SHFE rank table (position ranking) for a given date and symbol.
    pub async fn get_shfe_rank_table(
        &self,
        date: &str,
        _symbol: &str,
    ) -> Result<Vec<FuturesPositionRank>> {
        self.futures_shfe_position_rank(date).await
    }

    /// Get CZCE rank table (position ranking) for a given date and variety list.
    pub async fn get_rank_table_czce(
        &self,
        date: &str,
        _var_list: Option<&str>,
    ) -> Result<Vec<FuturesPositionRank>> {
        self.futures_czce_position_rank(date).await
    }

    /// Get roll yield bar (cross-section of roll yields for all varieties).
    pub async fn get_roll_yield_bar(
        &self,
        date: &str,
        _var: Option<&str>,
        _start_date: Option<&str>,
        _end_date: Option<&str>,
    ) -> Result<Vec<Row>> {
        self.futures_roll_yield_bar(date).await
    }

    /// Get rank sum — aggregated position ranking across exchanges.
    pub async fn get_rank_sum(&self, date: &str) -> Result<Vec<Row>> {
        let mut all_ranks: Vec<Row> = Vec::new();

        // SHFE
        if let Ok(ranks) = self.futures_shfe_position_rank(date).await {
            for r in &ranks {
                let mut row = Row::new();
                row.insert("exchange".into(), serde_json::json!("SHFE"));
                row.insert("rank".into(), serde_json::json!(r.rank));
                row.insert("symbol".into(), serde_json::json!(r.symbol));
                row.insert("variety".into(), serde_json::json!(r.variety));
                row.insert("vol_party_name".into(), serde_json::json!(r.vol_party_name));
                row.insert("vol".into(), serde_json::json!(r.vol));
                row.insert(
                    "long_party_name".into(),
                    serde_json::json!(r.long_party_name),
                );
                row.insert(
                    "long_open_interest".into(),
                    serde_json::json!(r.long_open_interest),
                );
                row.insert(
                    "short_party_name".into(),
                    serde_json::json!(r.short_party_name),
                );
                row.insert(
                    "short_open_interest".into(),
                    serde_json::json!(r.short_open_interest),
                );
                all_ranks.push(row);
            }
        }

        // CZCE
        if let Ok(ranks) = self.futures_czce_position_rank(date).await {
            for r in &ranks {
                let mut row = Row::new();
                row.insert("exchange".into(), serde_json::json!("CZCE"));
                row.insert("rank".into(), serde_json::json!(r.rank));
                row.insert("symbol".into(), serde_json::json!(r.symbol));
                row.insert("variety".into(), serde_json::json!(r.variety));
                row.insert("vol_party_name".into(), serde_json::json!(r.vol_party_name));
                row.insert("vol".into(), serde_json::json!(r.vol));
                row.insert(
                    "long_party_name".into(),
                    serde_json::json!(r.long_party_name),
                );
                row.insert(
                    "long_open_interest".into(),
                    serde_json::json!(r.long_open_interest),
                );
                row.insert(
                    "short_party_name".into(),
                    serde_json::json!(r.short_party_name),
                );
                row.insert(
                    "short_open_interest".into(),
                    serde_json::json!(r.short_open_interest),
                );
                all_ranks.push(row);
            }
        }

        // CFFEX
        if let Ok(ranks) = self.futures_cffex_position_rank(date).await {
            for r in &ranks {
                let mut row = Row::new();
                row.insert("exchange".into(), serde_json::json!("CFFEX"));
                row.insert("rank".into(), serde_json::json!(r.rank));
                row.insert("symbol".into(), serde_json::json!(r.symbol));
                row.insert("variety".into(), serde_json::json!(r.variety));
                row.insert("vol_party_name".into(), serde_json::json!(r.vol_party_name));
                row.insert("vol".into(), serde_json::json!(r.vol));
                row.insert(
                    "long_party_name".into(),
                    serde_json::json!(r.long_party_name),
                );
                row.insert(
                    "long_open_interest".into(),
                    serde_json::json!(r.long_open_interest),
                );
                row.insert(
                    "short_party_name".into(),
                    serde_json::json!(r.short_party_name),
                );
                row.insert(
                    "short_open_interest".into(),
                    serde_json::json!(r.short_open_interest),
                );
                all_ranks.push(row);
            }
        }

        Ok(all_ranks)
    }

    /// Get rank sum daily — daily aggregated position ranking data.
    pub async fn get_rank_sum_daily(&self, date: &str) -> Result<Vec<Row>> {
        self.get_rank_sum(date).await
    }

    /// Get receipt (warehouse receipt) data for a given date and variety list.
    ///
    /// Tries DCE and SHFE receipt data and merges the results.
    pub async fn get_receipt(&self, date: &str, _var_list: Option<&str>) -> Result<Vec<Row>> {
        let mut items = Vec::new();

        if let Ok(dce) = self.get_dce_receipt(date).await {
            items.extend(dce);
        }
        if let Ok(shfe) = self.get_shfe_receipt(date).await {
            items.extend(shfe);
        }

        Ok(items)
    }

    /// Get token — authenticate with Tushare Pro.
    ///
    /// `user`: Tushare account email
    /// `password`: Tushare account password
    pub async fn get_token(&self, user: &str, password: &str) -> Result<String> {
        self.pro_api(user, password).await
    }
}
