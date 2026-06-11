//! Futures roll yield (展期收益率) calculations.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FuturesDailyBar, Row};

impl AkShareClient {
    /// Calculate roll yield between two futures contracts.
    ///
    /// Returns (roll_yield, near_by_contract, deferred_contract).
    /// If `symbol1` and `symbol2` are not provided, uses the top 2 by open interest
    /// for the given variety.
    pub async fn get_roll_yield(
        &self,
        date: &str,
        variety: &str,
        symbol1: Option<&str>,
        symbol2: Option<&str>,
    ) -> Result<(f64, String, String)> {
        // Get daily data from all exchanges
        let mut all_bars: Vec<FuturesDailyBar> = Vec::new();
        for market in &["CFFEX", "SHFE", "DCE", "CZCE", "GFEX", "INE"] {
            if let Ok(bars) = self.get_futures_daily(date, market).await {
                all_bars.extend(bars);
            }
        }

        // Filter for the variety
        let mut variety_bars: Vec<&FuturesDailyBar> = all_bars
            .iter()
            .filter(|b| b.variety == variety && !b.symbol.contains("efp"))
            .collect();
        variety_bars.sort_by(|a, b| {
            b.open_interest
                .partial_cmp(&a.open_interest)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if variety_bars.len() < 2 {
            return Err(Error::not_found(format!(
                "not enough contracts for variety {}",
                variety
            )));
        }

        let (s1, c1) = if let Some(s) = symbol1 {
            let bar = variety_bars
                .iter()
                .find(|b| b.symbol == s)
                .ok_or_else(|| Error::not_found(format!("contract {} not found", s)))?;
            (s.to_string(), bar.close)
        } else {
            (variety_bars[0].symbol.clone(), variety_bars[0].close)
        };

        let (s2, c2) = if let Some(s) = symbol2 {
            let bar = variety_bars
                .iter()
                .find(|b| b.symbol == s)
                .ok_or_else(|| Error::not_found(format!("contract {} not found", s)))?;
            (s.to_string(), bar.close)
        } else {
            (variety_bars[1].symbol.clone(), variety_bars[1].close)
        };

        if c1 == 0.0 || c2 == 0.0 {
            return Err(Error::upstream("close price is zero"));
        }

        // Extract month codes
        let a: String = s1.chars().filter(|c| c.is_ascii_digit()).collect();
        let b: String = s2.chars().filter(|c| c.is_ascii_digit()).collect();
        let a_1: i32 = a[..a.len() - 2].parse().unwrap_or(0);
        let a_2: i32 = a[a.len() - 2..].parse().unwrap_or(0);
        let b_1: i32 = b[..b.len() - 2].parse().unwrap_or(0);
        let b_2: i32 = b[b.len() - 2..].parse().unwrap_or(0);
        let c = (a_1 - b_1) * 12 + (a_2 - b_2);

        let ry = (c2 / c1).ln() / c as f64 * 12.0;

        if c > 0 {
            Ok((ry, s2, s1))
        } else {
            Ok((ry, s1, s2))
        }
    }

    /// Roll yield cross-section for all varieties on a given date.
    pub async fn futures_roll_yield_bar(
        &self,
        date: &str,
    ) -> Result<Vec<Row>> {
        let mut all_bars: Vec<FuturesDailyBar> = Vec::new();
        for market in &["CFFEX", "SHFE", "DCE", "CZCE", "GFEX", "INE"] {
            if let Ok(bars) = self.get_futures_daily(date, market).await {
                all_bars.extend(bars);
            }
        }

        let varieties: std::collections::HashSet<String> =
            all_bars.iter().map(|b| b.variety.clone()).collect();

        let mut items = Vec::new();
        for var in &varieties {
            // Skip options
            if ["IO", "MO", "HO"].contains(&var.as_str()) {
                continue;
            }
            if let Ok((ry, near, deferred)) = self.get_roll_yield(date, var, None, None).await {
                let mut row = Row::new();
                row.insert("variety".into(), serde_json::json!(var));
                row.insert("roll_yield".into(), serde_json::json!(ry));
                row.insert("near_by".into(), serde_json::json!(near));
                row.insert("deferred".into(), serde_json::json!(deferred));
                row.insert("date".into(), serde_json::json!(date));
                items.push(row);
            }
        }
        Ok(items)
    }
}
