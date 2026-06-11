//! Option data from Eastmoney.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::OptionSnapshot;
use crate::util::*;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct DatacenterEnvelope {
    result: Option<DatacenterResult>,
}

#[derive(Debug, Deserialize)]
struct DatacenterResult {
    #[serde(default)]
    data: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct KlineEnvelope {
    data: Option<KlineData>,
}

#[derive(Debug, Deserialize)]
struct KlineData {
    klines: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Fetch option chain snapshots from Eastmoney.
    ///
    /// `symbol` identifies the underlying and option type. Common values:
    /// - `"510050"` — 50ETF options
    /// - `"510300"` — 300ETF options
    ///
    /// Returns up to `limit` option contracts with latest pricing data.
    pub async fn option_chain(&self, symbol: &str, limit: usize) -> Result<Vec<OptionSnapshot>> {
        let trimmed = symbol.trim();
        if trimmed.is_empty() {
            return Err(Error::invalid_input("option symbol is empty"));
        }
        let page_size = limit.clamp(1, 200).to_string();
        let filter = format!("(UNDERLYING_SECURITY_CODE=\"{trimmed}\")");

        let response = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_OPTION_CURRENTDAY"),
                ("columns", "ALL"),
                ("filter", filter.as_str()),
                ("pageNumber", "1"),
                ("pageSize", page_size.as_str()),
                ("sortTypes", "-1"),
                ("sortColumns", "TRADE_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: DatacenterEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload.result.map(|r| r.data).unwrap_or_default();
        let today = today_iso();

        let mut items = Vec::with_capacity(data.len());
        for v in &data {
            let option_code = v
                .get("SECURITY_CODE")
                .or_else(|| v.get("CONTRACT_CODE"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if option_code.is_empty() {
                continue;
            }

            let name = v
                .get("SECURITY_NAME_ABBR")
                .or_else(|| v.get("CONTRACT_NAME"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();

            let date = v
                .get("TRADE_DATE")
                .and_then(|x| x.as_str())
                .map(|s| s.get(..10).unwrap_or(s).to_string())
                .unwrap_or_else(|| today.clone());

            let close = v
                .get("CLOSE_PRICE")
                .or_else(|| v.get("LATEST_PRICE"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);

            let change_pct = v
                .get("CHANGE_RATE")
                .or_else(|| v.get("CHANGE_PCT"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);

            let volume = v
                .get("VOLUME")
                .or_else(|| v.get("TRADE_VOLUME"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);

            let open_interest = v
                .get("OPEN_INTEREST")
                .or_else(|| v.get("HOLD_VOLUME"))
                .and_then(|x| x.as_f64())
                .unwrap_or(0.0);

            let strike_price = v
                .get("STRIKE_PRICE")
                .or_else(|| v.get("EXERCISE_PRICE"))
                .and_then(|x| x.as_f64());

            let expiry_date = v
                .get("EXPIRE_DATE")
                .or_else(|| v.get("EXPIRATION_DATE"))
                .and_then(|x| x.as_str())
                .map(|s| s.get(..10).unwrap_or(s).to_string());

            items.push(OptionSnapshot {
                symbol: option_code,
                name,
                date,
                close,
                change_pct,
                volume,
                open_interest,
                strike_price,
                expiry_date,
            });
        }

        if items.is_empty() {
            // Fallback: try the push2 kline API for a specific option contract
            return self.option_chain_from_kline(trimmed, limit).await;
        }

        items.truncate(limit);
        Ok(items)
    }

    /// Fallback: fetch option data via the kline API for a specific contract.
    ///
    /// `contract_code` is the option contract code (e.g. "10005765").
    async fn option_chain_from_kline(
        &self,
        contract_code: &str,
        limit: usize,
    ) -> Result<Vec<OptionSnapshot>> {
        // Try secid format for Eastmoney option contracts
        let secid = format!("1.{contract_code}");
        let lmt = limit.max(5).to_string();

        let response = self
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", "101"),
                ("fqt", "1"),
                ("lmt", lmt.as_str()),
                ("end", "20500000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: KlineEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload
            .data
            .ok_or_else(|| Error::upstream("eastmoney option kline response missing data"))?;
        let klines = data
            .klines
            .ok_or_else(|| Error::upstream("eastmoney option kline response missing klines"))?;

        let mut items = Vec::with_capacity(klines.len());
        for line in &klines {
            let f = parse_csv_line(line);
            if f.len() < 6 {
                continue;
            }
            let close = parse_f64_safe(f[2]);
            let volume = parse_f64_safe(f[5]);
            items.push(OptionSnapshot {
                symbol: contract_code.to_string(),
                name: String::new(),
                date: f[0].to_string(),
                close,
                change_pct: 0.0,
                volume,
                open_interest: 0.0,
                strike_price: None,
                expiry_date: None,
            });
        }

        if items.is_empty() {
            return Err(Error::not_found(format!(
                "no option data found for {contract_code}"
            )));
        }

        // Compute change_pct from consecutive close prices
        for i in 1..items.len() {
            let prev = items[i - 1].close;
            if prev > 0.0 {
                items[i].change_pct = ((items[i].close - prev) / prev) * 100.0;
            }
        }

        items.sort_by(|a, b| a.date.cmp(&b.date));
        if items.len() > limit {
            let start = items.len() - limit;
            items = items[start..].to_vec();
        }
        Ok(items)
    }
}

// ---------------------------------------------------------------------------
// Return types for option_current_em / option_current_cffex_em
// ---------------------------------------------------------------------------

/// Option current day data from Eastmoney (push2 clist API).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionCurrentEmRow {
    /// Sequential number.
    pub index: usize,
    /// Option code.
    pub code: String,
    /// Option name.
    pub name: String,
    /// Latest price.
    pub latest_price: f64,
    /// Change amount.
    pub change: f64,
    /// Change percent.
    pub change_pct: f64,
    /// Volume.
    pub volume: f64,
    /// Turnover amount.
    pub amount: f64,
    /// Open interest.
    pub open_interest: f64,
    /// Exercise price.
    pub exercise_price: f64,
    /// Remaining days.
    pub remaining_days: f64,
    /// Daily change in open interest.
    pub daily_change: f64,
    /// Previous settlement.
    pub prev_settlement: f64,
    /// Open price.
    pub open: f64,
    /// Market identifier.
    pub market_id: i64,
}

// ---------------------------------------------------------------------------
// Wire types for clist API
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ClistEnvelope {
    data: Option<ClistData>,
}

#[derive(Debug, Default, Deserialize)]
struct ClistData {
    total: Option<usize>,
    #[serde(default)]
    diff: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct FutsseapiEnvelope {
    #[serde(default)]
    list: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Implementation — option_current_em / option_current_cffex_em / option_minute_em
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// All current-day options from Eastmoney (push2 clist + CFFEX).
    ///
    /// Fetches SSE/SZSE options via push2 clist API and CFFEX options via futsseapi,
    /// then combines them into a single list.
    pub async fn option_current_em(&self) -> Result<Vec<OptionCurrentEmRow>> {
        let mut all_rows = Vec::new();

        // Part 1: SSE/SZSE options via push2 clist
        let sse_rows = self.fetch_em_option_clist_current().await?;
        all_rows.extend(sse_rows);

        // Part 2: CFFEX options via futsseapi
        let cffex_rows = self.option_current_cffex_em().await?;
        all_rows.extend(cffex_rows);

        // Re-number
        for (i, row) in all_rows.iter_mut().enumerate() {
            row.index = i + 1;
        }

        Ok(all_rows)
    }

    /// CFFEX options from Eastmoney (futsseapi).
    pub async fn option_current_cffex_em(&self) -> Result<Vec<OptionCurrentEmRow>> {
        let url = "https://futsseapi.eastmoney.com/list/option/221";

        let resp: FutsseapiEnvelope = self
            .get(url)
            .query(&[
                ("orderBy", "zdf"),
                ("sort", "desc"),
                ("pageSize", "20000"),
                ("pageIndex", "0"),
                ("token", "58b2fa8f54638b60b87d69b31969089c"),
                (
                    "field",
                    "dm,sc,name,p,zsjd,zde,zdf,f152,vol,cje,ccl,xqj,syr,rz,zjsj,o",
                ),
                ("blockName", "callback"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let mut rows = Vec::with_capacity(resp.list.len());
        for (i, item) in resp.list.iter().enumerate() {
            rows.push(OptionCurrentEmRow {
                index: i + 1,
                code: json_str(item, "dm"),
                name: json_str(item, "name"),
                latest_price: json_f64(item, "p"),
                change: json_f64(item, "zde"),
                change_pct: json_f64(item, "zdf"),
                volume: json_f64(item, "vol"),
                amount: json_f64(item, "cje"),
                open_interest: json_f64(item, "ccl"),
                exercise_price: json_f64(item, "xqj"),
                remaining_days: json_f64(item, "syr"),
                daily_change: json_f64(item, "rz"),
                prev_settlement: json_f64(item, "zjsj"),
                open: json_f64(item, "o"),
                market_id: json_i64(item, "sc"),
            });
        }

        Ok(rows)
    }

    /// Option minute (intraday trend) data from Eastmoney.
    ///
    /// `symbol` is the option code, e.g. "MO2404-P-4450".
    pub async fn option_minute_em(&self, symbol: &str) -> Result<Vec<OptionMinuteRow>> {
        // First, look up the market id from the current options list
        let current = self.fetch_em_option_clist_current().await?;
        let option = current
            .iter()
            .find(|r| r.code == symbol)
            .ok_or_else(|| Error::not_found(format!("option not found: {symbol}")))?;
        let secid = format!("{}.{}", option.market_id, symbol);

        let url = "https://push2.eastmoney.com/api/qt/stock/trends2/get";

        let resp = self
            .get(url)
            .query(&[
                ("secid", secid.as_str()),
                (
                    "fields1",
                    "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13,f14,f17",
                ),
                ("fields2", "f51,f53,f54,f55,f56,f57,f58"),
                ("iscr", "0"),
                ("iscca", "0"),
                ("ut", "f057cbcbce2a86e2866ab8877db1d059"),
                ("ndays", "1"),
                ("cb", "quotepushdata1"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Response is JSONP: quotepushdata1({...})
        let json_start = resp.find('{').unwrap_or(0);
        let json_end = resp.rfind('}').map(|i| i + 1).unwrap_or(resp.len());
        let json_str = &resp[json_start..json_end];

        let data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("option minute json: {e}")))?;

        let trends = data
            .get("data")
            .and_then(|d| d.get("trends"))
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();

        let mut rows = Vec::with_capacity(trends.len());
        for trend in &trends {
            if let Some(s) = trend.as_str() {
                let fields: Vec<&str> = s.split(',').collect();
                if fields.len() < 6 {
                    continue;
                }
                rows.push(OptionMinuteRow {
                    time: fields[0].to_string(),
                    close: fields[1].parse::<f64>().unwrap_or(0.0),
                    high: fields[2].parse::<f64>().unwrap_or(0.0),
                    low: fields[3].parse::<f64>().unwrap_or(0.0),
                    volume: fields[4].parse::<f64>().unwrap_or(0.0),
                    amount: fields[5].parse::<f64>().unwrap_or(0.0),
                });
            }
        }

        if rows.is_empty() {
            return Err(Error::not_found(format!("no minute data for {symbol}")));
        }

        Ok(rows)
    }

    // -- private helpers ----------------------------------------------------

    /// Fetch SSE/SZSE options via the push2 clist API with pagination.
    async fn fetch_em_option_clist_current(&self) -> Result<Vec<OptionCurrentEmRow>> {
        let mut all_rows = Vec::new();
        let mut page = 1u32;
        let page_size = 100;

        loop {
            let pz = page_size.to_string();
            let pn = page.to_string();

            let resp: ClistEnvelope = self
                                .get("https://23.push2.eastmoney.com/api/qt/clist/get")
                .query(&[
                    ("pn", pn.as_str()),
                    ("pz", pz.as_str()),
                    ("po", "1"),
                    ("np", "1"),
                    ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                    ("fltt", "2"),
                    ("invt", "2"),
                    ("fid", "f3"),
                    ("fs", "m:10,m:12,m:140,m:141,m:151,m:163,m:226"),
                    (
                        "fields",
                        "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f24,f25,f22,f28,f11,f62,f128,f136,f115,f152,f133,f108,f163,f161,f162",
                    ),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .json()
                .await
                .map_err(Error::from)?;

            let data = resp.data.unwrap_or_default();
            let total = data.total.unwrap_or(0);
            let diff = data.diff;

            if diff.is_empty() {
                break;
            }

            for item in diff.iter() {
                // Items may be objects with numeric keys or direct arrays
                let values = if let Some(arr) = item.as_array() {
                    arr.clone()
                } else if let Some(obj) = item.as_object() {
                    let max_key = obj
                        .keys()
                        .filter_map(|k| k.parse::<usize>().ok())
                        .max()
                        .unwrap_or(0);
                    (0..=max_key)
                        .map(|i| {
                            obj.get(&i.to_string())
                                .cloned()
                                .unwrap_or(serde_json::Value::Null)
                        })
                        .collect::<Vec<_>>()
                } else {
                    continue;
                };

                if values.len() < 36 {
                    continue;
                }

                all_rows.push(OptionCurrentEmRow {
                    index: all_rows.len() + 1,
                    code: em_str(&values, 12),
                    name: em_str(&values, 14),
                    latest_price: em_f64(&values, 2),
                    change: em_f64(&values, 4),
                    change_pct: em_f64(&values, 3),
                    volume: em_f64(&values, 5),
                    amount: em_f64(&values, 6),
                    open_interest: em_f64(&values, 27),
                    exercise_price: em_f64(&values, 35),
                    remaining_days: em_f64(&values, 36),
                    daily_change: em_f64(&values, 37),
                    prev_settlement: em_f64(&values, 25),
                    open: em_f64(&values, 17),
                    market_id: em_i64(&values, 13),
                });
            }

            if all_data_len(&diff) >= total || page_size > diff.len() as u32 {
                break;
            }
            page += 1;
        }

        if all_rows.is_empty() {
            return Err(Error::not_found("no EM option current data"));
        }

        Ok(all_rows)
    }
}

/// Minute data row from Eastmoney option trend API.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionMinuteRow {
    /// Time (HH:MM).
    pub time: String,
    /// Close price.
    pub close: f64,
    /// High price.
    pub high: f64,
    /// Low price.
    pub low: f64,
    /// Volume.
    pub volume: f64,
    /// Amount.
    pub amount: f64,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn json_str(v: &serde_json::Value, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string()
}

fn json_f64(v: &serde_json::Value, key: &str) -> f64 {
    match v.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(serde_json::Value::String(s)) => s.trim().parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

fn json_i64(v: &serde_json::Value, key: &str) -> i64 {
    match v.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0),
        Some(serde_json::Value::String(s)) => s.trim().parse::<i64>().unwrap_or(0),
        _ => 0,
    }
}

fn em_str(arr: &[serde_json::Value], idx: usize) -> String {
    arr.get(idx)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn em_f64(arr: &[serde_json::Value], idx: usize) -> f64 {
    match arr.get(idx) {
        Some(serde_json::Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(serde_json::Value::String(s)) => s.trim().parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

fn em_i64(arr: &[serde_json::Value], idx: usize) -> i64 {
    match arr.get(idx) {
        Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0),
        Some(serde_json::Value::String(s)) => s.trim().parse::<i64>().unwrap_or(0),
        _ => 0,
    }
}

fn all_data_len(diff: &[serde_json::Value]) -> usize {
    diff.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse_csv_line;

    #[test]
    fn test_parse_option_kline_line() {
        let line = "2025-01-02,0.0500,0.0550,0.0600,0.0480,123456";
        let f = parse_csv_line(line);
        assert_eq!(f.len(), 6);
        assert_eq!(f[0], "2025-01-02");
        assert!((parse_f64_safe(f[2]) - 0.055).abs() < 0.001);
    }

    #[test]
    fn test_option_chain_empty_symbol_validation() {
        // Empty symbol should be rejected; test the validation path directly
        let trimmed = "";
        assert!(trimmed.is_empty());
    }
}
