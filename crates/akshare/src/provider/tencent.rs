//! Tencent Finance API helpers.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, HkFinancialSnapshot, QuoteSnapshot};
use crate::util::{
    amplitude_pct, apply_change_metrics, normalize_trade_date, parse_f64_safe, parse_i64_safe,
};

/// Parse a JSON Value to f64, handling both number and string types.
fn value_to_f64(v: &serde_json::Value) -> f64 {
    v.as_f64()
        .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
        .unwrap_or(0.0)
}

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
            .map_or("", |(s, _)| s);
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
        #[derive(serde::Deserialize)]
        struct Resp {
            data: Option<serde_json::Value>,
        }

        let ts = crate::market::tencent_market_symbol(symbol)?;
        let period = "day";
        let url = format!(
            "https://web.ifzq.gtimg.cn/appstock/app/fqkline/get?param={ts},{period},,,{limit},{adjust}"
        );

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
            let Some(arr) = entry.as_array() else {
                continue;
            };
            if arr.len() < 6 {
                continue;
            }
            let trade_date = arr[0].as_str().unwrap_or("").to_string();
            let open = value_to_f64(&arr[1]);
            let close = value_to_f64(&arr[2]);
            let high = value_to_f64(&arr[3]);
            let low = value_to_f64(&arr[4]);
            let volume = value_to_f64(&arr[5]) as i64;
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
            .map_or("", |(s, _)| s);
        let p: Vec<&str> = data.split('~').collect();
        if p.len() < 35 {
            return Err(Error::decode("tencent HK quote: insufficient fields"));
        }
        // Fields: 0:market, 1:name, 2:symbol, 3:current_price, 4:prev_close, 5:open,
        //         6:volume, ..., 30:date, 31:change, 32:change_pct, 33:high, 34:low
        Ok(QuoteSnapshot {
            symbol: symbol.to_string(),
            date: normalize_trade_date(p.get(30).unwrap_or(&"")),
            open: parse_f64_safe(p.get(5).unwrap_or(&"")),
            high: parse_f64_safe(p.get(33).unwrap_or(&"")),
            low: parse_f64_safe(p.get(34).unwrap_or(&"")),
            close: parse_f64_safe(p.get(3).unwrap_or(&"")),
            volume: parse_i64_safe(p.get(6).unwrap_or(&"")),
        })
    }

    /// Tencent HK financial data (PE, PB, EPS, BVPS, market cap).
    ///
    /// Fields from `qt.gtimg.cn`:
    /// - [1] name, [3] price, [39] PE_TTM, [44] market_cap (亿),
    /// - [57] EPS, [58] BVPS, [72] PB
    pub(crate) async fn tencent_hk_financial(&self, symbol: &str) -> Result<HkFinancialSnapshot> {
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
            .map_or("", |(s, _)| s);
        let p: Vec<&str> = data.split('~').collect();
        if p.len() < 60 {
            return Err(Error::decode("tencent HK financial: insufficient fields"));
        }
        let parse = |i: usize| -> Option<f64> {
            p.get(i)
                .and_then(|s| s.trim().parse::<f64>().ok())
                .filter(|v| *v != 0.0)
        };
        Ok(HkFinancialSnapshot {
            symbol: symbol.to_string(),
            name: p.get(1).unwrap_or(&"").to_string(),
            pe_ttm: parse(39),
            pb: parse(72),
            eps: parse(57),
            bvps: parse(58),
            market_cap_hkd: parse(44).map(|v| v * 100_000_000.0), // 亿 → 元
            amount_hkd: parse(37),
        })
    }

    /// Tencent HK kline (candlestick).
    pub(crate) async fn tencent_hk_candles(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        #[derive(serde::Deserialize)]
        struct Resp {
            data: Option<serde_json::Value>,
        }

        let code = crate::market::normalize_hk_symbol(symbol)
            .ok_or_else(|| Error::invalid_input(format!("invalid HK symbol: {symbol}")))?;
        let hk_symbol = format!("hk{code}");
        let url = format!(
            "https://web.ifzq.gtimg.cn/appstock/app/hkfqkline/get?_var=kline_dayqfq&param={hk_symbol},day,,,{limit},qfq"
        );

        let resp_text = self.get(&url).send().await?.text().await?;
        // Parse JSONP response: kline_dayqfq={...}
        let json_str = if let Some(start) = resp_text.find('{') {
            &resp_text[start..]
        } else {
            return Err(Error::upstream("invalid tencent HK kline response"));
        };
        let resp: Resp = serde_json::from_str(json_str)?;
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
            let Some(arr) = entry.as_array() else {
                continue;
            };
            if arr.len() < 6 {
                continue;
            }
            items.push(CandlePoint {
                trade_date: arr[0].as_str().unwrap_or("").to_string(),
                open: value_to_f64(&arr[1]),
                close: value_to_f64(&arr[2]),
                high: value_to_f64(&arr[3]),
                low: value_to_f64(&arr[4]),
                volume: value_to_f64(&arr[5]) as i64,
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
