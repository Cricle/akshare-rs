//! Stooq CSV data provider (fallback for US/global stocks).

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::*;
use crate::util::*;

impl AkShareClient {
    /// Fetch daily candles from Stooq CSV.
    pub(crate) async fn stooq_candles(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let stooq_sym = symbol.to_lowercase();
        let url = format!("https://stooq.com/q/d/l/?s={stooq_sym}&i=d");

        let body = self
                        .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        for line in body.lines().skip(1) {
            let p: Vec<&str> = line.split(',').collect();
            if p.len() < 6 {
                continue;
            }
            let open = parse_f64_safe(p[1]);
            let high = parse_f64_safe(p[2]);
            let low = parse_f64_safe(p[3]);
            let close = parse_f64_safe(p[4]);
            let volume = parse_i64_safe(p[5]);

            items.push(CandlePoint {
                trade_date: normalize_trade_date(p[0]),
                open,
                close,
                high,
                low,
                volume,
                amount: 0.0,
                amplitude_pct: amplitude_pct(high, low),
                change_pct: 0.0,
                change_amount: 0.0,
                turnover_pct: 0.0,
            });
        }

        apply_change_metrics(&mut items);

        if items.len() > limit {
            items = items[items.len() - limit..].to_vec();
        }
        if items.is_empty() {
            return Err(Error::upstream("stooq: no data returned"));
        }
        Ok(items)
    }
}
