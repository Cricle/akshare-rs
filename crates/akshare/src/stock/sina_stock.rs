//! Sina Finance stock data — intraday ticks, sector spot.
//!
//! Covers Python functions:
//! - `stock_intraday_sina` — Intraday tick data from Sina
//! - `stock_sector_spot` — Sector spot from Sina

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Intraday tick data from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinaIntradayTick {
    pub ticktime: String,
    pub price: f64,
    pub volume: f64,
    #[serde(default)]
    pub prev_price: Option<f64>,
    #[serde(default)]
    pub buy_or_sell: Option<String>,
}

/// Sector spot data from Sina.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinaSectorSpot {
    pub label: String,
    pub sector: String,
    #[serde(default)]
    pub company_count: Option<i64>,
    #[serde(default)]
    pub avg_price: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
    #[serde(default)]
    pub total_volume: Option<f64>,
    #[serde(default)]
    pub total_amount: Option<f64>,
    #[serde(default)]
    pub leading_symbol: Option<String>,
    #[serde(default)]
    pub leading_change_pct: Option<f64>,
    #[serde(default)]
    pub leading_price: Option<f64>,
    #[serde(default)]
    pub leading_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get intraday tick data from Sina.
    ///
    /// Python equivalent: `stock_intraday_sina(symbol, date)`
    ///
    /// `symbol` uses Sina format like "sz000001" or "sh600000".
    /// `date` is in format "20240321".
    pub async fn stock_intraday_sina(
        &self,
        symbol: &str,
        date: &str,
        limit: usize,
    ) -> Result<Vec<SinaIntradayTick>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);

        // Step 1: Get total count
        let count_url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/CN_Bill.GetBillListCount";
        let count_response = self
                        .get(count_url)
            .query(&[
                ("symbol", symbol),
                ("num", "60"),
                ("page", "1"),
                ("sort", "ticktime"),
                ("asc", "0"),
                ("volume", "0"),
                ("amount", "0"),
                ("type", "0"),
                ("day", date_fmt.as_str()),
            ])
            .header(
                "Referer",
                format!(
                    "https://vip.stock.finance.sina.com.cn/quotes_service/view/cn_bill.php?symbol={symbol}"
                ),
            )
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let count_text = count_response.text().await.map_err(Error::from)?;
        let total_count: i64 = count_text
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse()
            .unwrap_or(0);

        if total_count == 0 {
            return Err(Error::not_found("sina returned no intraday data"));
        }

        let total_pages = ((total_count as f64) / 60.0).ceil() as i64;

        // Step 2: Fetch ticks (limit pages to avoid excessive requests)
        let list_url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/CN_Bill.GetBillList";
        let max_pages = total_pages.min((limit as i64 + 59) / 60).min(10);
        let mut all_ticks = Vec::new();

        for page in 1..=max_pages {
            let page_str = page.to_string();
            let response = self
                                .get(list_url)
                .query(&[
                    ("symbol", symbol),
                    ("num", "60"),
                    ("page", page_str.as_str()),
                    ("sort", "ticktime"),
                    ("asc", "0"),
                    ("volume", "0"),
                    ("amount", "0"),
                    ("type", "0"),
                    ("day", date_fmt.as_str()),
                ])
                .header(
                    "Referer",
                    format!(
                        "https://vip.stock.finance.sina.com.cn/quotes_service/view/cn_bill.php?symbol={symbol}"
                    ),
                )
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let ticks: Vec<serde_json::Value> = response.json().await.map_err(Error::from)?;

            for tick in &ticks {
                all_ticks.push(SinaIntradayTick {
                    ticktime: tick
                        .get("ticktime")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    price: tick
                        .get("price")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse().ok())
                        .or_else(|| tick.get("price").and_then(serde_json::Value::as_f64))
                        .unwrap_or(0.0),
                    volume: tick
                        .get("volume")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse().ok())
                        .or_else(|| tick.get("volume").and_then(serde_json::Value::as_f64))
                        .unwrap_or(0.0),
                    prev_price: tick
                        .get("prev_price")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse().ok())
                        .or_else(|| tick.get("prev_price").and_then(serde_json::Value::as_f64)),
                    buy_or_sell: tick
                        .get("type")
                        .and_then(|v| v.as_str())
                        .map(std::string::ToString::to_string),
                });
            }
        }

        all_ticks.sort_by(|a, b| a.ticktime.cmp(&b.ticktime));
        all_ticks.truncate(limit);

        Ok(all_ticks)
    }

    /// Get sector spot data from Sina.
    ///
    /// Python equivalent: `stock_sector_spot(indicator)`
    ///
    /// `indicator` is one of: "new_sina", "qmx", "concept", "area", "industry".
    pub async fn stock_sector_spot(&self, indicator: &str) -> Result<Vec<SinaSectorSpot>> {
        let url = match indicator {
            "new_sina" => "http://vip.stock.finance.sina.com.cn/q/view/newSinaHy.php".to_string(),
            "qmx" => "http://biz.finance.sina.com.cn/hq/qmxIndustryHq.php".to_string(),
            "concept" => {
                "http://money.finance.sina.com.cn/q/view/newFLJK.php?param=class".to_string()
            }
            "area" => "http://money.finance.sina.com.cn/q/view/newFLJK.php?param=area".to_string(),
            "industry" => {
                "http://money.finance.sina.com.cn/q/view/newFLJK.php?param=industry".to_string()
            }
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported indicator: {indicator}"
                )));
            }
        };

        let response = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;

        // Sina returns JS-like JSON, extract the JSON object
        let json_start = text
            .find('{')
            .ok_or_else(|| Error::decode("sina sector response does not contain JSON object"))?;
        let json_text = &text[json_start..];

        let data: serde_json::Value = serde_json::from_str(json_text)
            .map_err(|e| Error::decode(format!("sina JSON parse: {e}")))?;

        let obj = data
            .as_object()
            .ok_or_else(|| Error::decode("sina sector response is not an object"))?;

        let mut items = Vec::new();
        for (_, val) in obj {
            if let Some(s) = val.as_str() {
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() >= 13 {
                    items.push(SinaSectorSpot {
                        label: parts[0].to_string(),
                        sector: parts[1].to_string(),
                        company_count: parts[2].parse().ok(),
                        avg_price: parts[3].parse().ok(),
                        change_amount: parts[4].parse().ok(),
                        change_pct: parts[5].parse().ok(),
                        total_volume: parts[6].parse().ok(),
                        total_amount: parts[7].parse().ok(),
                        leading_symbol: Some(parts[8].to_string()),
                        leading_change_pct: parts[9].parse().ok(),
                        leading_price: parts[10].parse().ok(),
                        leading_name: Some(parts[12].to_string()),
                    });
                }
            }
        }

        if items.is_empty() {
            return Err(Error::not_found("sina returned no sector spot items"));
        }

        Ok(items)
    }
}
