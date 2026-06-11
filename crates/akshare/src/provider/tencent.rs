//! Tencent Finance API helpers.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::*;
use crate::util::*;

impl AkShareClient {
    /// Tencent A-share realtime quote.
    pub(crate) async fn tencent_a_share_quote(&self, symbol: &str) -> Result<QuoteSnapshot> {
        let ts = crate::market::tencent_market_symbol(symbol)?;
        let url = format!("https://qt.gtimg.cn/q={ts}");
        let body = self.get(&url).send().await?.text().await?;
        let line = body
            .lines()
            .find(|l| l.contains("v_"))
            .ok_or_else(|| Error::upstream("empty tencent response"))?;
        let data = line
            .split_once('=')
            .and_then(|(_, r)| r.trim_matches('"').split_once(';'))
            .map(|(s, _)| s)
            .unwrap_or("");
        let p: Vec<&str> = data.split('~').collect();
        if p.len() < 45 {
            return Err(Error::decode("tencent quote: insufficient fields"));
        }
        Ok(QuoteSnapshot {
            symbol: symbol.to_string(),
            date: normalize_trade_date(p[30]),
            open: parse_f64_safe(p[5]),
            high: parse_f64_safe(p[33]),
            low: parse_f64_safe(p[34]),
            close: parse_f64_safe(p[3]),
            volume: parse_i64_safe(p[6]),
        })
    }

    /// Tencent A-share kline (candlestick).
    pub(crate) async fn tencent_a_share_candles(
        &self,
        symbol: &str,
        adjust: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let ts = crate::market::tencent_market_symbol(symbol)?;
        let period = "day";
        let url = format!(
            "https://web.ifzq.gtimg.cn/appstock/app/fqkline/get?param={ts},{period},,,{limit},{adjust}"
        );

        #[derive(serde::Deserialize)]
        struct Resp {
            data: Option<serde_json::Value>,
        }

        let resp: Resp = self.get(&url).send().await?.json().await?;
        let data = resp
            .data
            .ok_or_else(|| Error::upstream("empty tencent kline data"))?;

        let symbol_lower = ts.to_lowercase();
        let kline_key = match adjust {
            "qfq" => "qfqday",
            "hfq" => "hfqday",
            _ => "day",
        };
        let klines = data
            .get(&symbol_lower)
            .and_then(|v| v.get(kline_key).or_else(|| v.get("day")))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::with_capacity(klines.len());
        for entry in &klines {
            let arr = match entry.as_array() {
                Some(a) => a,
                None => continue,
            };
            if arr.len() < 6 {
                continue;
            }
            let trade_date = arr[0].as_str().unwrap_or("").to_string();
            let open = arr[1].as_f64().unwrap_or(0.0);
            let close = arr[2].as_f64().unwrap_or(0.0);
            let high = arr[3].as_f64().unwrap_or(0.0);
            let low = arr[4].as_f64().unwrap_or(0.0);
            let volume = arr[5].as_f64().unwrap_or(0.0) as i64;
            items.push(CandlePoint {
                trade_date,
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
        Ok(items)
    }

    /// Tencent HK realtime quote.
    pub(crate) async fn tencent_hk_quote(&self, symbol: &str) -> Result<QuoteSnapshot> {
        let code = crate::market::normalize_hk_symbol(symbol)
            .ok_or_else(|| Error::invalid_input(format!("invalid HK symbol: {symbol}")))?;
        let url = format!("https://qt.gtimg.cn/q=r_hk{code}");
        let body = self.get(&url).send().await?.text().await?;
        let line = body
            .lines()
            .find(|l| l.contains("v_"))
            .ok_or_else(|| Error::upstream("empty tencent HK response"))?;
        let data = line
            .split_once('=')
            .and_then(|(_, r)| r.trim_matches('"').split_once(';'))
            .map(|(s, _)| s)
            .unwrap_or("");
        let p: Vec<&str> = data.split('~').collect();
        if p.len() < 30 {
            return Err(Error::decode("tencent HK quote: insufficient fields"));
        }
        Ok(QuoteSnapshot {
            symbol: symbol.to_string(),
            date: normalize_trade_date(p.get(17).unwrap_or(&"")),
            open: parse_f64_safe(p.get(2).unwrap_or(&"")),
            high: parse_f64_safe(p.get(3).unwrap_or(&"")),
            low: parse_f64_safe(p.get(4).unwrap_or(&"")),
            close: parse_f64_safe(p.get(1).unwrap_or(&"")),
            volume: parse_i64_safe(p.get(6).unwrap_or(&"")),
        })
    }

    /// Tencent HK kline (candlestick).
    pub(crate) async fn tencent_hk_candles(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let code = crate::market::normalize_hk_symbol(symbol)
            .ok_or_else(|| Error::invalid_input(format!("invalid HK symbol: {symbol}")))?;
        let hk_symbol = format!("r_hk{code}");
        let url = format!(
            "https://web.ifzq.gtimg.cn/appstock/app/fqkline/get?param={hk_symbol},day,,,{limit},qfq"
        );

        #[derive(serde::Deserialize)]
        struct Resp {
            data: Option<serde_json::Value>,
        }

        let resp: Resp = self.get(&url).send().await?.json().await?;
        let data = resp
            .data
            .ok_or_else(|| Error::upstream("empty tencent HK kline data"))?;

        let klines = data
            .get(&hk_symbol)
            .and_then(|v| v.get("qfqday").or_else(|| v.get("day")))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::with_capacity(klines.len());
        for entry in &klines {
            let arr = match entry.as_array() {
                Some(a) => a,
                None => continue,
            };
            if arr.len() < 6 {
                continue;
            }
            items.push(CandlePoint {
                trade_date: arr[0].as_str().unwrap_or("").to_string(),
                open: arr[1].as_f64().unwrap_or(0.0),
                close: arr[2].as_f64().unwrap_or(0.0),
                high: arr[3].as_f64().unwrap_or(0.0),
                low: arr[4].as_f64().unwrap_or(0.0),
                volume: arr[5].as_f64().unwrap_or(0.0) as i64,
                amount: 0.0,
                amplitude_pct: 0.0,
                change_pct: 0.0,
                change_amount: 0.0,
                turnover_pct: 0.0,
            });
        }
        apply_change_metrics(&mut items);
        Ok(items)
    }
}
