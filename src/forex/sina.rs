//! Forex rates from Sina Finance.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::ForexRate;
use crate::util::parse_f64_safe;

/// Sina forex symbols for major currency pairs against CNY.
const SINA_FOREX_SYMBOLS: &[&str] = &[
    "fx_susdcny", "fx_seurcny", "fx_sgbpcny", "fx_sjpycny", "fx_shkdcny",
    "fx_saudcny", "fx_scadcny", "fx_schfcny", "fx_nzdcny", "fx_ssgdcny",
];

/// Human-readable currency pair labels matching `SINA_FOREX_SYMBOLS`.
const FOREX_PAIR_LABELS: &[&str] = &[
    "USD/CNY", "EUR/CNY", "GBP/CNY", "JPY/CNY", "HKD/CNY",
    "AUD/CNY", "CAD/CNY", "CHF/CNY", "NZD/CNY", "SGD/CNY",
];

impl AkShareClient {
    /// Forex realtime rates from Sina Finance.
    ///
    /// Fetches buy/sell/middle rates for major currency pairs against CNY
    /// from the Sina `hq.sinajs.cn` API.
    ///
    /// Response format: `var hq_str_fx_susdcny="name,buy,sell,middle,ts1,ts2,date,time,...";`
    pub async fn forex_sina_rates(&self) -> Result<Vec<ForexRate>> {
        let symbols_csv = SINA_FOREX_SYMBOLS.join(",");
        let url = format!("https://hq.sinajs.cn/list={symbols_csv}");

        let body = self
                        .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        for (idx, line) in body.lines().enumerate() {
            // Each line: var hq_str_fx_susdcny="field1,field2,...";
            let data = line
                .split_once('=')
                .and_then(|(_, r)| r.trim_matches('"').split_once(';'))
                .map(|(s, _)| s)
                .unwrap_or("");
            if data.is_empty() {
                continue;
            }
            let fields: Vec<&str> = data.split(',').collect();
            // Sina forex format typically:
            //   [0] pair name, [1] buy_rate, [2] sell_rate, [3] middle_rate,
            //   [4-5] timestamps, [6] date, [7] time, ...
            if fields.len() < 7 {
                continue;
            }
            let buy_rate = parse_f64_safe(fields[1]);
            let sell_rate = parse_f64_safe(fields[2]);
            let middle_rate = parse_f64_safe(fields[3]);
            let date = fields[6].to_string();
            if middle_rate == 0.0 && buy_rate == 0.0 {
                continue;
            }
            let currency_pair = FOREX_PAIR_LABELS
                .get(idx)
                .unwrap_or(&"Unknown/CNY")
                .to_string();
            items.push(ForexRate {
                currency_pair,
                buy_rate,
                sell_rate,
                middle_rate,
                date,
                change_pct: None,
            });
        }

        if items.is_empty() {
            return Err(Error::not_found("sina returned no forex rates"));
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sina_forex_symbols_and_labels_match() {
        assert_eq!(SINA_FOREX_SYMBOLS.len(), FOREX_PAIR_LABELS.len());
    }

    #[test]
    fn test_parse_sina_forex_line() {
        // Simulate a Sina response line
        let line = r#"var hq_str_fx_susdcny="美元兑人民币,7.0950,7.0980,7.0965,2025-01-02 10:00:00,2025-01-02,2025-01-02,10:00:00";"#;
        let data = line
            .split_once('=')
            .and_then(|(_, r)| r.trim_matches('"').split_once(';'))
            .map(|(s, _)| s)
            .unwrap_or("");
        let fields: Vec<&str> = data.split(',').collect();
        assert!(fields.len() >= 7);
        let buy = parse_f64_safe(fields[1]);
        let sell = parse_f64_safe(fields[2]);
        let middle = parse_f64_safe(fields[3]);
        assert!((buy - 7.095).abs() < 0.001);
        assert!((sell - 7.098).abs() < 0.001);
        assert!((middle - 7.0965).abs() < 0.001);
        assert_eq!(fields[6], "2025-01-02");
    }
}
