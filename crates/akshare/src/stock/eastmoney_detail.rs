//! Eastmoney detailed stock data — bid/ask, intraday, individual info, profiles.
//!
//! Covers Python functions:
//! - `stock_bid_ask` / `stock_ask_bid_em` — Bid/ask data
//! - `stock_intraday_em` — Intraday tick data
//! - `stock_individual_info` — Individual stock info
//! - `stock_hk_security_profile` — HK security profile
//! - `stock_hk_company_profile` — HK company profile

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::value_ext::ValueExt;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct DatacenterProfileEnvelope<T> {
    result: Option<DatacenterProfileResult<T>>,
}

#[derive(Debug, Deserialize)]
struct DatacenterProfileResult<T> {
    data: Option<Vec<T>>,
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Bid/ask data for a stock.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidAskData {
    pub symbol: String,
    pub name: String,
    #[serde(default)]
    pub latest: Option<f64>,
    #[serde(default)]
    pub avg_price: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub turnover_rate: Option<f64>,
    #[serde(default)]
    pub volume_ratio: Option<f64>,
    #[serde(default)]
    pub high: Option<f64>,
    #[serde(default)]
    pub low: Option<f64>,
    #[serde(default)]
    pub open: Option<f64>,
    #[serde(default)]
    pub prev_close: Option<f64>,
    #[serde(default)]
    pub limit_up: Option<f64>,
    #[serde(default)]
    pub limit_down: Option<f64>,
    #[serde(default)]
    pub buy_1_price: Option<f64>,
    #[serde(default)]
    pub buy_1_vol: Option<f64>,
    #[serde(default)]
    pub buy_2_price: Option<f64>,
    #[serde(default)]
    pub buy_2_vol: Option<f64>,
    #[serde(default)]
    pub buy_3_price: Option<f64>,
    #[serde(default)]
    pub buy_3_vol: Option<f64>,
    #[serde(default)]
    pub buy_4_price: Option<f64>,
    #[serde(default)]
    pub buy_4_vol: Option<f64>,
    #[serde(default)]
    pub buy_5_price: Option<f64>,
    #[serde(default)]
    pub buy_5_vol: Option<f64>,
    #[serde(default)]
    pub sell_1_price: Option<f64>,
    #[serde(default)]
    pub sell_1_vol: Option<f64>,
    #[serde(default)]
    pub sell_2_price: Option<f64>,
    #[serde(default)]
    pub sell_2_vol: Option<f64>,
    #[serde(default)]
    pub sell_3_price: Option<f64>,
    #[serde(default)]
    pub sell_3_vol: Option<f64>,
    #[serde(default)]
    pub sell_4_price: Option<f64>,
    #[serde(default)]
    pub sell_4_vol: Option<f64>,
    #[serde(default)]
    pub sell_5_price: Option<f64>,
    #[serde(default)]
    pub sell_5_vol: Option<f64>,
    #[serde(default)]
    pub outer_vol: Option<f64>,
    #[serde(default)]
    pub inner_vol: Option<f64>,
}

/// Intraday tick data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntradayTick {
    pub time: String,
    pub price: f64,
    pub volume: f64,
    /// "buy" (买盘), "sell" (卖盘), or "neutral" (中性盘).
    pub side: String,
}

/// Individual stock info key-value pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockInfoItem {
    pub item: String,
    pub value: serde_json::Value,
}

/// HK security profile data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkSecurityProfile {
    #[serde(default)]
    pub security_code: Option<String>,
    #[serde(default)]
    pub security_name: Option<String>,
    #[serde(default)]
    pub listing_date: Option<String>,
    #[serde(default)]
    pub security_type: Option<String>,
    #[serde(default)]
    pub issue_price: Option<f64>,
    #[serde(default)]
    pub issue_num: Option<f64>,
    #[serde(default)]
    pub trade_unit: Option<i64>,
    #[serde(default)]
    pub par_value: Option<f64>,
    #[serde(default)]
    pub trade_market: Option<String>,
    #[serde(default)]
    pub board: Option<String>,
    #[serde(default)]
    pub year_settle_day: Option<String>,
    #[serde(default)]
    pub isin_code: Option<String>,
    #[serde(default)]
    pub is_hgt_target: Option<bool>,
    #[serde(default)]
    pub is_sgt_target: Option<bool>,
}

/// HK company profile data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkCompanyProfile {
    #[serde(default)]
    pub company_name: Option<String>,
    #[serde(default)]
    pub company_name_en: Option<String>,
    #[serde(default)]
    pub registered_capital: Option<f64>,
    #[serde(default)]
    pub chairman: Option<String>,
    #[serde(default)]
    pub general_manager: Option<String>,
    #[serde(default)]
    pub secretary: Option<String>,
    #[serde(default)]
    pub established_date: Option<String>,
    #[serde(default)]
    pub listing_date: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub business_scope: Option<String>,
    #[serde(default)]
    pub main_business: Option<String>,
    #[serde(default)]
    pub employees: Option<i64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get bid/ask data for an A-share stock from Eastmoney.
    ///
    /// Python equivalent: `stock_bid_ask(symbol)` / `stock_ask_bid_em(symbol)`
    pub async fn stock_bid_ask(&self, symbol: &str) -> Result<BidAskData> {
        let secid = crate::market::eastmoney_secid(symbol)?;
        let response = self
            .get("https://push2.eastmoney.com/api/qt/stock/get")
            .query(&[
                ("fltt", "2"),
                ("invt", "2"),
                (
                    "fields",
                    "f43,f44,f45,f46,f47,f48,f49,f50,f51,f52,f55,f57,f58,f59,f60,f62,\
                     f71,f116,f117,f161,f162,f163,f164,f167,f168,f169,f170,f171,f120,f121,\
                     f122,f174,f175,f11,f12,f13,f14,f15,f16,f17,f18,f19,f20,f31,f32,f33,\
                     f34,f35,f36,f37,f38,f39,f40",
                ),
                ("secid", secid.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("data")
            .ok_or_else(|| Error::upstream("eastmoney bid/ask missing data"))?;

        Ok(BidAskData {
            symbol: symbol.to_string(),
            name: data.str_or(&["f58"], ""),
            latest: data.f64_field(&["f43"]),
            avg_price: data.f64_field(&["f71"]),
            change_pct: data.f64_field(&["f170"]),
            change_amount: data.f64_field(&["f169"]),
            volume: data.f64_field(&["f47"]),
            amount: data.f64_field(&["f48"]),
            turnover_rate: data.f64_field(&["f168"]),
            volume_ratio: data.f64_field(&["f50"]),
            high: data.f64_field(&["f44"]),
            low: data.f64_field(&["f45"]),
            open: data.f64_field(&["f46"]),
            prev_close: data.f64_field(&["f60"]),
            limit_up: data.f64_field(&["f51"]),
            limit_down: data.f64_field(&["f52"]),
            buy_1_price: data.f64_field(&["f19"]),
            buy_1_vol: data.f64_field(&["f20"]).map(|v| v * 100.0),
            buy_2_price: data.f64_field(&["f17"]),
            buy_2_vol: data.f64_field(&["f18"]).map(|v| v * 100.0),
            buy_3_price: data.f64_field(&["f15"]),
            buy_3_vol: data.f64_field(&["f16"]).map(|v| v * 100.0),
            buy_4_price: data.f64_field(&["f13"]),
            buy_4_vol: data.f64_field(&["f14"]).map(|v| v * 100.0),
            buy_5_price: data.f64_field(&["f11"]),
            buy_5_vol: data.f64_field(&["f12"]).map(|v| v * 100.0),
            sell_1_price: data.f64_field(&["f39"]),
            sell_1_vol: data.f64_field(&["f40"]).map(|v| v * 100.0),
            sell_2_price: data.f64_field(&["f37"]),
            sell_2_vol: data.f64_field(&["f38"]).map(|v| v * 100.0),
            sell_3_price: data.f64_field(&["f35"]),
            sell_3_vol: data.f64_field(&["f36"]).map(|v| v * 100.0),
            sell_4_price: data.f64_field(&["f33"]),
            sell_4_vol: data.f64_field(&["f34"]).map(|v| v * 100.0),
            sell_5_price: data.f64_field(&["f31"]),
            sell_5_vol: data.f64_field(&["f32"]).map(|v| v * 100.0),
            outer_vol: data.f64_field(&["f49"]),
            inner_vol: data.f64_field(&["f161"]),
        })
    }

    /// Get intraday tick data for an A-share stock from Eastmoney.
    ///
    /// Python equivalent: `stock_intraday_em(symbol)`
    pub async fn stock_intraday_em(&self, symbol: &str) -> Result<Vec<IntradayTick>> {
        let secid = crate::market::eastmoney_secid(symbol)?;
        let response = self
            .get("https://push2.eastmoney.com/api/qt/stock/details/sse")
            .query(&[
                ("fields1", "f1,f2,f3,f4"),
                ("fields2", "f51,f52,f53,f54,f55"),
                ("mpi", "2000"),
                ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
                ("fltt", "2"),
                ("pos", "-0"),
                ("secid", secid.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;

        // The response is SSE (Server-Sent Events) format
        let mut ticks = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if !line.starts_with("data:") {
                continue;
            }
            let json_str = line.strip_prefix("data:").unwrap_or(line).trim();
            if json_str.is_empty() {
                continue;
            }
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
                if let Some(details) = val
                    .get("data")
                    .and_then(|d| d.get("details"))
                    .and_then(|d| d.as_array())
                {
                    for detail in details {
                        if let Some(s) = detail.as_str() {
                            let parts: Vec<&str> = s.split(',').collect();
                            if parts.len() >= 5 {
                                ticks.push(IntradayTick {
                                    time: parts[0].to_string(),
                                    price: parts[1].parse().unwrap_or(0.0),
                                    volume: parts[2].parse().unwrap_or(0.0),
                                    side: match parts.get(4) {
                                        Some(&"2") => "buy".to_string(),
                                        Some(&"1") => "sell".to_string(),
                                        _ => "neutral".to_string(),
                                    },
                                });
                            }
                        }
                    }
                }
                break; // We only need the first event
            }
        }

        Ok(ticks)
    }

    /// Get individual stock info from Eastmoney.
    ///
    /// Python equivalent: `stock_individual_info(symbol)`
    pub async fn stock_individual_info(&self, symbol: &str) -> Result<Vec<StockInfoItem>> {
        let secid = crate::market::eastmoney_secid(symbol)?;
        let response = self
            .get("https://push2.eastmoney.com/api/qt/stock/get")
            .query(&[
                ("fltt", "2"),
                ("invt", "2"),
                ("fields", "f57,f58,f84,f85,f127,f116,f117,f189,f43"),
                ("secid", secid.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;

        // Fields are nested inside the "data" object
        let data = payload.get("data").unwrap_or(&payload);

        let code_name_map = [
            ("f57", "股票代码"),
            ("f58", "股票简称"),
            ("f84", "总股本"),
            ("f85", "流通股"),
            ("f127", "行业"),
            ("f116", "总市值"),
            ("f117", "流通市值"),
            ("f189", "上市时间"),
            ("f43", "最新价"),
        ];

        let mut items = Vec::new();
        for (key, label) in &code_name_map {
            if let Some(val) = data.get(key)
                && !val.is_null()
            {
                items.push(StockInfoItem {
                    item: label.to_string(),
                    value: val.clone(),
                });
            }
        }

        Ok(items)
    }

    /// Get individual stock info from Eastmoney using a raw secid.
    ///
    /// Unlike `stock_individual_info`, this accepts a pre-formatted secid
    /// (e.g. "105.AAPL" for NASDAQ, "106.AAPL" for NYSE, "116.00700" for HK).
    pub async fn stock_individual_info_em_by_secid(
        &self,
        secid: &str,
    ) -> Result<Vec<StockInfoItem>> {
        let response = self
            .get("https://push2.eastmoney.com/api/qt/stock/get")
            .query(&[
                ("fltt", "2"),
                ("invt", "2"),
                ("fields", "f57,f58,f84,f85,f127,f116,f117,f189,f43"),
                ("secid", secid),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;

        let data = payload.get("data").unwrap_or(&payload);

        let code_name_map = [
            ("f57", "股票代码"),
            ("f58", "股票简称"),
            ("f84", "总股本"),
            ("f85", "流通股"),
            ("f127", "行业"),
            ("f116", "总市值"),
            ("f117", "流通市值"),
            ("f189", "上市时间"),
            ("f43", "最新价"),
        ];

        let mut items = Vec::new();
        for (key, label) in &code_name_map {
            if let Some(val) = data.get(key)
                && !val.is_null()
            {
                items.push(StockInfoItem {
                    item: label.to_string(),
                    value: val.clone(),
                });
            }
        }

        Ok(items)
    }

    /// Get industry for any stock by raw Eastmoney secid (e.g. "105.AAPL", "116.00700").
    pub async fn stock_info_by_secid(&self, secid: &str) -> Result<Option<String>> {
        let response = self
            .get("https://push2.eastmoney.com/api/qt/stock/get")
            .query(&[
                ("fltt", "2"),
                ("invt", "2"),
                ("fields", "f127"),
                ("secid", secid),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let industry = payload
            .nested(&["data", "f127"])
            .and_then(|v| v.as_str())
            .filter(|s| !s.trim().is_empty())
            .map(std::string::ToString::to_string);
        Ok(industry)
    }

    /// Get HK security profile from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_security_profile(symbol)`
    pub async fn stock_hk_security_profile(&self, symbol: &str) -> Result<Vec<HkSecurityProfile>> {
        let filter = format!("(SECUCODE=\"{symbol}.HK\")");
        self.fetch_hk_profile("RPT_HKF10_INFO_SECURITYINFO", &filter)
            .await
    }

    /// Get HK company profile from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_company_profile(symbol)`
    pub async fn stock_hk_company_profile(&self, symbol: &str) -> Result<Vec<HkCompanyProfile>> {
        let filter = format!("(SECUCODE=\"{symbol}.HK\")");
        self.fetch_hk_profile("RPT_HKF10_INFO_COMPANYINFO", &filter)
            .await
    }

    // -- Private helpers ----------------------------------------------------

    async fn fetch_hk_profile<T>(&self, report_name: &str, filter: &str) -> Result<Vec<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self
            .get("https://datacenter.eastmoney.com/securities/api/data/v1/get")
            .query(&[
                ("reportName", report_name),
                ("columns", "ALL"),
                ("quoteColumns", ""),
                ("filter", filter),
                ("pageNumber", "1"),
                ("pageSize", "200"),
                ("sortTypes", ""),
                ("sortColumns", ""),
                ("source", "F10"),
                ("client", "PC"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: DatacenterProfileEnvelope<T> = response.json().await.map_err(Error::from)?;
        Ok(payload.result.and_then(|r| r.data).unwrap_or_default())
    }
}
