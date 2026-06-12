//! Sina Finance API helpers.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, QuoteSnapshot};
use crate::util::{amplitude_pct, apply_change_metrics, parse_f64_safe, parse_i64_safe};

impl AkShareClient {
    /// Sina A-share realtime quote.
    pub async fn sina_a_share_realtime(&self, symbol: &str) -> Result<QuoteSnapshot> {
        let normalized = crate::market::normalize_a_share_symbol(symbol)
            .ok_or_else(|| Error::invalid_input(format!("invalid A-share symbol: {symbol}")))?;
        let (code, suffix) = normalized
            .split_once('.')
            .ok_or_else(|| Error::invalid_input(format!("invalid symbol format: {normalized}")))?;
        let prefix = match suffix {
            "SH" => "sh",
            "SZ" => "sz",
            _ => return Err(Error::invalid_input("unsupported exchange")),
        };
        let sina_symbol = format!("{prefix}{code}");
        let url = format!("https://hq.sinajs.cn/list={sina_symbol}");
        let body = self
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        let data = body
            .split_once('=')
            .and_then(|(_, r)| r.trim_matches('"').split_once(';'))
            .map_or("", |(s, _)| s);
        let p: Vec<&str> = data.split(',').collect();
        if p.len() < 32 {
            return Err(Error::decode("sina realtime: insufficient fields"));
        }
        Ok(QuoteSnapshot {
            symbol: symbol.to_string(),
            date: p[30].to_string(),
            open: parse_f64_safe(p[1]),
            high: parse_f64_safe(p[4]),
            low: parse_f64_safe(p[5]),
            close: parse_f64_safe(p[3]),
            volume: parse_i64_safe(p[8]),
        })
    }

    /// Sina US daily candles.
    pub async fn sina_us_daily(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        let url = format!(
            "https://stock.finance.sina.com.cn/usstock/api/jsonp.php/IO.XSRV2.CallbackList/US_MinKService.getDailyK?symbol={symbol}&_var=kline_dayqfq&range={limit}d"
        );
        let body = self
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        let json_str = body
            .split_once('(')
            .and_then(|(_, rest)| rest.rsplit_once(')'))
            .map(|(s, _)| s)
            .ok_or_else(|| Error::decode("invalid sina US kline JSONP"))?;

        #[derive(serde::Deserialize)]
        struct KlineEntry {
            #[serde(default)]
            d: String,
            #[serde(default)]
            o: String,
            #[serde(default)]
            h: String,
            #[serde(default)]
            l: String,
            #[serde(default)]
            c: String,
            #[serde(default)]
            v: String,
        }

        let entries: Vec<KlineEntry> = serde_json::from_str(json_str).map_err(|e| {
            Error::decode(format!(
                "sina US kline JSON parse failed: {e}; raw={}",
                &json_str[..json_str.len().min(200)]
            ))
        })?;
        let mut items = Vec::with_capacity(entries.len());
        for e in &entries {
            items.push(CandlePoint {
                trade_date: e.d.clone(),
                open: parse_f64_safe(&e.o),
                close: parse_f64_safe(&e.c),
                high: parse_f64_safe(&e.h),
                low: parse_f64_safe(&e.l),
                volume: parse_i64_safe(&e.v),
                amount: 0.0,
                amplitude_pct: amplitude_pct(parse_f64_safe(&e.h), parse_f64_safe(&e.l)),
                change_pct: 0.0,
                change_amount: 0.0,
                turnover_pct: 0.0,
            });
        }
        apply_change_metrics(&mut items);
        Ok(items)
    }
}
