//! Foreign commodity realtime data from Sina Finance.
//!
//! Provides realtime quotes for international futures (LME, COMEX, NYMEX, etc.)

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::{ForeignCommodityQuote, Row};

fn parse_f64(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap_or(0.0)
}

/// Exchange symbol map: Chinese name -> Sina code
fn exchange_symbol_map() -> Vec<(&'static str, &'static str)> {
    vec![
        ("新加坡铁矿石", "FEF"),
        ("马棕油", "FCPO"),
        ("日橡胶", "RSS3"),
        ("美国原糖", "RS"),
        ("CME比特币期货", "BTC"),
        ("NYBOT-棉花", "CT"),
        ("LME镍3个月", "NID"),
        ("LME铅3个月", "PBD"),
        ("LME锡3个月", "SND"),
        ("LME锌3个月", "ZSD"),
        ("LME铝3个月", "AHD"),
        ("LME铜3个月", "CAD"),
        ("CBOT-黄豆", "S"),
        ("CBOT-小麦", "W"),
        ("CBOT-玉米", "C"),
        ("CBOT-黄豆油", "BO"),
        ("CBOT-黄豆粉", "SM"),
        ("日本橡胶", "TRB"),
        ("COMEX铜", "HG"),
        ("NYMEX天然气", "NG"),
        ("NYMEX原油", "CL"),
        ("COMEX白银", "SI"),
        ("COMEX黄金", "GC"),
        ("CME-瘦肉猪", "LHC"),
        ("布伦特原油", "OIL"),
        ("伦敦金", "XAU"),
        ("伦敦银", "XAG"),
        ("伦敦铂金", "XPT"),
        ("伦敦钯金", "XPD"),
        ("欧洲碳排放", "EUA"),
    ]
}

impl AkShareClient {
    /// List of foreign commodity subscribe exchange symbols (Python-compatible name).
    ///
    /// Returns symbol-code pairs for all supported foreign futures.
    pub fn futures_foreign_commodity_subscribe_exchange_symbol(&self) -> Vec<Row> {
        self.futures_hq_subscribe_exchange_symbol()
    }

    /// List of foreign commodity subscribe exchange symbols.
    ///
    /// Returns symbol-code pairs for all supported foreign futures.
    pub fn futures_hq_subscribe_exchange_symbol(&self) -> Vec<Row> {
        exchange_symbol_map()
            .into_iter()
            .map(|(name, code)| {
                let mut row = Row::new();
                row.insert("symbol".into(), serde_json::json!(name));
                row.insert("code".into(), serde_json::json!(code));
                row
            })
            .collect()
    }

    /// List of all foreign commodity Sina subscription codes.
    pub fn futures_foreign_commodity_subscribe_codes(&self) -> Vec<String> {
        exchange_symbol_map()
            .into_iter()
            .map(|(_, code)| code.to_string())
            .collect()
    }

    /// Foreign commodity realtime quotes from Sina.
    ///
    /// `symbols` is a list of Sina codes (e.g., ["CL", "GC", "XAU"]).
    pub async fn futures_foreign_commodity_realtime(
        &self,
        symbols: &[&str],
    ) -> Result<Vec<ForeignCommodityQuote>> {
        let subscribe_list: Vec<String> = symbols.iter().map(|s| format!("hf_{s}")).collect();
        let list_param = subscribe_list.join(",");
        let url = format!("https://hq.sinajs.cn/?list={list_param}");

        let body = self
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn/")
            .header("Host", "hq.sinajs.cn")
            .send()
            .await?
            .text()
            .await?;

        let code_to_name: std::collections::HashMap<&str, &str> =
            exchange_symbol_map().into_iter().collect();

        let mut items = Vec::new();
        for line in body.split(';') {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let Some(eq_pos) = line.find('=') else {
                continue;
            };
            let var_part = &line[..eq_pos];
            let code = var_part.split('_').next_back().unwrap_or("");
            let data_part = line[eq_pos + 1..].trim_matches('"');
            let fields: Vec<&str> = data_part.split(',').collect();

            if fields.len() < 14 {
                continue;
            }

            let name = code_to_name.get(code).unwrap_or(&code);
            let current_price = parse_f64(fields[0]);
            let bid = parse_f64(fields[2]);
            let ask = parse_f64(fields[3]);
            let high = parse_f64(fields[4]);
            let low = parse_f64(fields[5]);
            let time_str = fields[6].to_string();
            let last_settle = parse_f64(fields[7]);
            let open = parse_f64(fields[8]);
            let hold = parse_f64(fields[9]);
            let date = fields.get(12).unwrap_or(&"").to_string();
            let current_price_rmb = fields.get(14).map_or(0.0, |s| parse_f64(s));

            let change_amount = current_price - last_settle;
            let change_pct = if last_settle > 0.0 {
                change_amount / last_settle * 100.0
            } else {
                0.0
            };

            items.push(ForeignCommodityQuote {
                symbol: name.to_string(),
                current_price,
                current_price_rmb,
                change_amount,
                change_pct,
                open,
                high,
                low,
                last_settle_price: last_settle,
                hold,
                bid,
                ask,
                time: time_str,
                date,
            });
        }
        Ok(items)
    }
}
