//! Futures data from Sina Finance.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, Row};
use crate::util::{amplitude_pct, apply_change_metrics};

impl AkShareClient {
    /// Fetch main contract futures candles from Sina Finance.
    ///
    /// `symbol` should be a Sina futures symbol, e.g.:
    /// - `"nf_AG0"` — Silver main contract
    /// - `"nf_CU0"` — Copper main contract
    /// - `"nf_SC0"` — Crude oil (Shanghai INE)
    /// - `"nf_RB0"` — Rebar
    /// - `"nf_IF0"` — CSI 300 index futures
    pub async fn futures_main(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        let url = format!(
            "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_=/InnerFuturesNewService.getDailyKLine?symbol={symbol}&_={}",
            chrono::Utc::now().timestamp_millis()
        );
        let body = self
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        // Extract JSON array from JSONP callback: var _=([[...]])
        let json_str = body
            .find("[[")
            .and_then(|start| {
                body[start..]
                    .rfind("]]")
                    .map(|end| &body[start..start + end + 2])
            })
            .ok_or_else(|| Error::decode("sina futures: invalid JSONP response"))?;

        // Parse as Vec<Vec<...>> where each inner is [date, open, high, low, close, volume, ...]
        let rows: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str).unwrap_or_default();

        let mut items = Vec::with_capacity(rows.len());
        for row in &rows {
            if row.len() < 6 {
                continue;
            }
            let date = row[0].as_str().unwrap_or("").to_string();
            let open = row[1].as_f64().unwrap_or(0.0);
            let high = row[2].as_f64().unwrap_or(0.0);
            let low = row[3].as_f64().unwrap_or(0.0);
            let close = row[4].as_f64().unwrap_or(0.0);
            let volume = row[5].as_f64().unwrap_or(0.0) as i64;

            items.push(CandlePoint {
                trade_date: date,
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
            return Err(Error::upstream("sina futures: no data returned"));
        }
        Ok(items)
    }

    /// Futures symbol-to-mark mapping from Sina.
    ///
    /// Returns a list of (exchange, symbol, mark) triples for all Chinese futures
    /// symbols available on Sina Finance.
    pub async fn futures_symbol_mark(&self) -> Result<Vec<Row>> {
        // The upstream Sina JS file is GBK-encoded with Chinese commodity names,
        // making it unreliable to parse. Instead, we use a static mapping of
        // English futures variety symbols to Sina node marks.
        // Source: vip.stock.finance.sina.com.cn/quotes_service/view/js/qihuohangqing.js
        let mapping: Vec<(&str, &str, &str)> = vec![
            // SHFE (上海期货交易所)
            ("shfe", "CU", "tong_qh"),
            ("shfe", "AL", "lv_qh"),
            ("shfe", "ZN", "xing_qh"),
            ("shfe", "PB", "qian_qh"),
            ("shfe", "NI", "ni_qh"),
            ("shfe", "SN", "xi_qh"),
            ("shfe", "AU", "hj_qh"),
            ("shfe", "AG", "by_qh"),
            ("shfe", "RB", "lwg_qh"),
            ("shfe", "HC", "rzjb_qh"),
            ("shfe", "SS", "bxg_qh"),
            ("shfe", "BU", "lq_qh"),
            ("shfe", "RU", "xj_qh"),
            ("shfe", "FU", "ry_qh"),
            ("shfe", "SP", "zj_qh"),
            ("shfe", "SC", "yy_qh"),
            ("shfe", "NR", "ehj_qh"),
            ("shfe", "LU", "lu_qh"),
            ("shfe", "BC", "bc_qh"),
            ("shfe", "AO", "ao_qh"),
            ("shfe", "BR", "br_qh"),
            ("shfe", "WR", "xc_qh"),
            ("shfe", "LC", "lc_qh"),
            ("shfe", "SI", "ps_qh"),
            ("shfe", "PT", "pt_qh"),
            ("shfe", "PD", "pd_qh"),
            // DCE (大连商品交易所)
            ("dce", "A", "dd_qh"),
            ("dce", "B", "de_qh"),
            ("dce", "M", "dp_qh"),
            ("dce", "Y", "dy_qh"),
            ("dce", "P", "zly_qh"),
            ("dce", "C", "hym_qh"),
            ("dce", "CS", "ymdf_qh"),
            ("dce", "I", "tks_qh"),
            ("dce", "J", "jt_qh"),
            ("dce", "JM", "jm_qh"),
            ("dce", "L", "lldpe_qh"),
            ("dce", "PP", "jbx_qh"),
            ("dce", "V", "pvc_qh"),
            ("dce", "EG", "yec_qh"),
            ("dce", "EB", "byx_qh"),
            ("dce", "PG", "pg_qh"),
            ("dce", "JD", "jd_qh"),
            ("dce", "RR", "gm_qh"),
            ("dce", "LH", "lh_qh"),
            ("dce", "FB", "xwb_qh"),
            ("dce", "BB", "jhb_qh"),
            ("dce", "LG", "lg_qh"),
            ("dce", "BZ", "bz_qh"),
            // CZCE (郑州商品交易所)
            ("czce", "TA", "pta_qh"),
            ("czce", "MA", "czy_qh"),
            ("czce", "SR", "bst_qh"),
            ("czce", "CF", "mh_qh"),
            ("czce", "OI", "czy_qh"),
            ("czce", "RM", "czp_qh"),
            ("czce", "FG", "bl_qh"),
            ("czce", "SA", "cj_qh"),
            ("czce", "UR", "ns_qh"),
            ("czce", "PK", "pk_qh"),
            ("czce", "PF", "pf_qh"),
            ("czce", "AP", "xpg_qh"),
            ("czce", "CJ", "hz_qh"),
            ("czce", "CY", "ms_qh"),
            ("czce", "SF", "gt_qh"),
            ("czce", "SM", "mg_qh"),
            ("czce", "ZC", "dlm_qh"),
            ("czce", "WH", "qm_qh"),
            ("czce", "PM", "qm_qh"),
            ("czce", "JR", "jdm_qh"),
            ("czce", "LR", "zxd_qh"),
            ("czce", "SH", "sh_qh"),
            ("czce", "PX", "px_qh"),
            // CFFEX (中国金融期货交易所)
            ("cffex", "IF", "if_qh"),
            ("cffex", "IH", "ih_qh"),
            ("cffex", "IC", "ic_qh"),
            ("cffex", "IM", "im_qh"),
            ("cffex", "T", "t_qh"),
            ("cffex", "TF", "tf_qh"),
            ("cffex", "TS", "ts_qh"),
            ("cffex", "TL", "tl_qh"),
            // GFEX (广州期货交易所)
            ("gfex", "SI2", "si2_qh"),
            ("gfex", "LC2", "lc2_qh"),
        ];

        let items: Vec<Row> = mapping
            .into_iter()
            .map(|(exchange, symbol, mark)| {
                let mut r = Row::new();
                r.insert("exchange".into(), serde_json::json!(exchange));
                r.insert("symbol".into(), serde_json::json!(symbol));
                r.insert("mark".into(), serde_json::json!(mark));
                r
            })
            .collect();
        Ok(items)
    }

    /// Realtime quotes for all contracts of a given futures variety.
    ///
    /// `symbol` is the variety name like "PTA", "RB", "CU".
    /// Returns realtime data for all tradeable contracts of that variety.
    pub async fn futures_zh_realtime(&self, symbol: &str) -> Result<Vec<Row>> {
        // First get the symbol mark mapping
        let marks = self.futures_symbol_mark().await?;
        let mark = marks
            .iter()
            .find(|r| r.get("symbol").and_then(|v| v.as_str()) == Some(symbol))
            .and_then(|r| {
                r.get("mark")
                    .and_then(|v| v.as_str().map(std::string::ToString::to_string))
            })
            .ok_or_else(|| Error::not_found(format!("sina: unknown futures symbol: {symbol}")))?;

        let url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQFuturesData";
        let body = self
            .get(url)
            .query(&[
                ("page", "1"),
                ("sort", "position"),
                ("asc", "0"),
                ("node", mark.as_str()),
                ("base", "futures"),
            ])
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .send()
            .await?
            .text()
            .await?;

        let data: Vec<serde_json::Value> = serde_json::from_str(&body)
            .map_err(|_| Error::decode("sina realtime: JSON parse error"))?;

        let mut items = Vec::new();
        for entry in &data {
            let mut r = Row::new();
            let empty = serde_json::Map::new();
            for (key, val) in entry.as_object().unwrap_or(&empty) {
                r.insert(key.clone(), val.clone());
            }
            items.push(r);
        }
        Ok(items)
    }

    /// Realtime spot quotes for specific futures contracts from Sina hq.sinajs.cn.
    ///
    /// `symbols` is a comma-separated list of contract codes like "V2309,V2401".
    /// `market` is "CF" for commodity futures, "FF" for financial futures.
    pub async fn futures_zh_spot(&self, symbols: &str, market: &str) -> Result<Vec<Row>> {
        let subscribe_list: Vec<String> = symbols
            .split(',')
            .map(|s| format!("nf_{}", s.trim()))
            .collect();
        let list_str = subscribe_list.join(",");
        let rn = format!("{:x}", chrono::Utc::now().timestamp_millis());

        let url = format!("https://hq.sinajs.cn/rn={rn}&list={list_str}");
        let body = self
            .get(&url)
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        for line in body.split(';') {
            let line = line.trim();
            if line.is_empty() || !line.contains('=') {
                continue;
            }
            // Extract symbol from var hq_str_nf_XXX=
            let symbol_part = line.split('=').next().unwrap_or("");
            let symbol_name = symbol_part.split("hq_str_").last().unwrap_or("");
            let nf_symbol = symbol_name.strip_prefix("nf_").unwrap_or(symbol_name);

            // Extract the comma-separated values between quotes
            let value_part = line.split('=').nth(1).unwrap_or("");
            let values: Vec<&str> = value_part.trim_matches('"').split(',').collect();

            if values.len() < 15 {
                continue;
            }

            let mut r = Row::new();
            r.insert("symbol".into(), serde_json::json!(nf_symbol));

            if market == "CF" {
                // Commodity futures format
                r.insert("time".into(), serde_json::json!(values[0]));
                r.insert("open".into(), serde_json::json!(values[1]));
                r.insert("high".into(), serde_json::json!(values[2]));
                r.insert("low".into(), serde_json::json!(values[3]));
                r.insert("last_close".into(), serde_json::json!(values[4]));
                r.insert("bid_price".into(), serde_json::json!(values[5]));
                r.insert("ask_price".into(), serde_json::json!(values[6]));
                r.insert("current_price".into(), serde_json::json!(values[7]));
                r.insert("avg_price".into(), serde_json::json!(values[8]));
                r.insert("last_settle_price".into(), serde_json::json!(values[9]));
                r.insert("buy_vol".into(), serde_json::json!(values[10]));
                r.insert("sell_vol".into(), serde_json::json!(values[11]));
                r.insert("hold".into(), serde_json::json!(values[12]));
                r.insert("volume".into(), serde_json::json!(values[13]));
            } else {
                // Financial futures format (FF) - different field positions
                r.insert("open".into(), serde_json::json!(values[0]));
                r.insert("high".into(), serde_json::json!(values[1]));
                r.insert("low".into(), serde_json::json!(values[2]));
                r.insert("current_price".into(), serde_json::json!(values[3]));
                r.insert("volume".into(), serde_json::json!(values[4]));
                r.insert("amount".into(), serde_json::json!(values[5]));
                r.insert("hold".into(), serde_json::json!(values[6]));
                // time is at a different position for financial futures
                if values.len() > 32 {
                    r.insert("time".into(), serde_json::json!(values[32]));
                }
            }
            items.push(r);
        }
        Ok(items)
    }

    /// Minute-frequency kline data for a futures contract from Sina.
    ///
    /// `symbol`: contract code like "IF2008", "RB0"
    /// `period`: "1", "5", "15", "30", "60" for 1/5/15/30/60 minute bars
    pub async fn futures_zh_minute(&self, symbol: &str, period: &str) -> Result<Vec<Row>> {
        let url = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/=/InnerFuturesNewService.getFewMinLine";
        let body = self
            .get(url)
            .query(&[("symbol", symbol), ("type", period)])
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        // Extract JSON from JSONP callback: =([...]);
        let json_str = body
            .find("=(")
            .and_then(|start| {
                body[start..]
                    .rfind(");")
                    .map(|end| &body[start + 2..start + end])
            })
            .ok_or_else(|| Error::decode("sina minute kline: invalid JSONP response"))?;

        let data: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|_| Error::decode("sina minute kline: JSON parse error"))?;

        let mut items = Vec::new();
        for row in &data {
            if row.len() < 7 {
                continue;
            }
            let mut r = Row::new();
            r.insert("datetime".into(), row[0].clone());
            r.insert("open".into(), row[1].clone());
            r.insert("high".into(), row[2].clone());
            r.insert("low".into(), row[3].clone());
            r.insert("close".into(), row[4].clone());
            r.insert("volume".into(), row[5].clone());
            r.insert("hold".into(), row[6].clone());
            items.push(r);
        }
        Ok(items)
    }

    /// Daily kline data for a specific futures contract from Sina.
    ///
    /// `symbol`: contract code like "RB0", "V2105"
    pub async fn futures_zh_daily(&self, symbol: &str) -> Result<Vec<Row>> {
        let date = chrono::Utc::now().format("%Y%m%d").to_string();
        let date_formatted = format!("{}_{}_{}", &date[..4], &date[4..6], &date[6..8]);
        let url = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_dummy=/InnerFuturesNewService.getDailyKLine";
        let body = self
            .get(url)
            .query(&[("symbol", symbol), ("type", &date_formatted)])
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        // Extract JSON from JSONP callback
        let json_str = body
            .find("=(")
            .and_then(|start| {
                body[start..]
                    .rfind(");")
                    .map(|end| &body[start + 2..start + end])
            })
            .ok_or_else(|| Error::decode("sina daily kline: invalid JSONP response"))?;

        let data: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|_| Error::decode("sina daily kline: JSON parse error"))?;

        let mut items = Vec::new();
        for row in &data {
            if row.len() < 8 {
                continue;
            }
            let mut r = Row::new();
            r.insert("date".into(), row[0].clone());
            r.insert("open".into(), row[1].clone());
            r.insert("high".into(), row[2].clone());
            r.insert("low".into(), row[3].clone());
            r.insert("close".into(), row[4].clone());
            r.insert("volume".into(), row[5].clone());
            r.insert("hold".into(), row[6].clone());
            r.insert("settle".into(), row[7].clone());
            items.push(r);
        }
        Ok(items)
    }
}
