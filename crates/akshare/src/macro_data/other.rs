#![allow(dead_code)]
//! Other macro-economic data — crypto spot, FX sentiment from Jin10.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct CryptoResp {
    data: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct SentimentResp {
    data: Option<SentimentData>,
}

#[derive(Debug, Deserialize)]
struct SentimentData {
    values: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct WsMacroResp {
    data: Option<WsMacroData>,
}

#[derive(Debug, Deserialize)]
struct WsMacroData {
    items: Option<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Crypto spot prices from Jin10 (主流加密货币实时行情).
    /// Returns macro-level crypto price data as MacroDataPoint for economic analysis.
    pub async fn macro_crypto_spot(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-api.jin10.com/crypto_currency/list";
        let resp: CryptoResp = self
            .get(url)
            .header("x-app-id", "rU6QIu7JHe2gOUeR")
            .header("x-csrf-token", "x-csrf-token")
            .header("x-version", "1.0.0")
            .send()
            .await?
            .json()
            .await?;

        let data = resp.data.unwrap_or_default();
        let mut items = Vec::with_capacity(data.len());
        for v in &data {
            let market = v.get("market").and_then(|x| x.as_str()).unwrap_or("");
            let symbol = v.get("symbol").and_then(|x| x.as_str()).unwrap_or("");
            let price = v.get("price").and_then(|x| x.as_f64()).unwrap_or(0.0);
            let updated = v.get("reported_at").and_then(|x| x.as_str()).unwrap_or("");

            items.push(MacroDataPoint {
                date: updated.get(..10).unwrap_or(updated).to_string(),
                value: price,
                name: format!("{} {}", market, symbol),
            });
        }
        Ok(items)
    }

    /// FX sentiment report from Jin10 (外汇投机情绪报告).
    pub async fn macro_fx_sentiment(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        // Convert YYYYMMDD to YYYY-MM-DD
        let start_fmt = format!(
            "{}-{}-{}",
            &start_date[..4],
            &start_date[4..6],
            &start_date[6..8]
        );
        let end_fmt = format!("{}-{}-{}", &end_date[..4], &end_date[4..6], &end_date[6..8]);

        let url = "https://datacenter-api.jin10.com/sentiment/datas";
        let resp: serde_json::Value = self
            .get(url)
            .query(&[
                ("start_date", start_fmt.as_str()),
                ("end_date", end_fmt.as_str()),
                ("currency_pair", ""),
            ])
            .header("x-app-id", "rU6QIu7JHe2gOUeR")
            .header("x-csrf-token", "x-csrf-token")
            .header("x-version", "1.0.0")
            .send()
            .await?
            .json()
            .await?;

        let mut items = Vec::new();
        if let Some(data) = resp.get("data")
            && let Some(values) = data.get("values").and_then(|v| v.as_object())
        {
            for (date, row) in values {
                if let Some(cols) = row.as_object() {
                    for (pair, val) in cols {
                        if let Some(v) = val.as_f64() {
                            items.push(MacroDataPoint {
                                date: date.clone(),
                                value: v,
                                name: format!("FX Sentiment {}", pair),
                            });
                        }
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// Wall Street calendar - macro data (华尔街见闻-日历-宏观).
    pub async fn macro_info_ws(&self, date: &str) -> Result<Vec<MacroDataPoint>> {
        // Convert YYYYMMDD to YYYY-MM-DD HH:MM:SS
        let date_fmt = format!("{}-{}-{} 00:00:00", &date[..4], &date[4..6], &date[6..8]);

        let url = "https://api-one-wscn.awtmt.com/apiv1/finance/macrodatas";
        let resp: WsMacroResp = self
            .get(url)
            .query(&[("start", date_fmt.as_str())])
            .send()
            .await?
            .json()
            .await?;

        let mut items = Vec::new();
        if let Some(data) = resp.data
            && let Some(macro_items) = data.items
        {
            for item in &macro_items {
                let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("");
                let actual = item.get("actual").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let pub_date = item
                    .get("public_date")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);

                // Convert unix timestamp to date string
                let date_str = if pub_date > 0 {
                    chrono::DateTime::from_timestamp(pub_date, 0)
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or_default()
                } else {
                    date[..8].to_string()
                };

                items.push(MacroDataPoint {
                    date: date_str,
                    value: actual,
                    name: title.to_string(),
                });
            }
        }
        Ok(items)
    }

    /// THS stock finance data (同花顺-股票筹资).
    pub async fn macro_stock_finance(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://data.10jqka.com.cn/macro/finance/";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (compatible; akshare-rust/0.1)")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        // Parse HTML table
        let re_rows: Vec<&str> = body.split("<tr").collect();
        for row in &re_rows[1..] {
            let cells: Vec<&str> = row.split("<td").collect();
            if cells.len() >= 3 {
                let date_cell = extract_html_text_other(cells[1]);
                let val_cell = extract_html_text_other(cells[2]);
                if (date_cell.contains("-") || date_cell.contains("/"))
                    && let Ok(val) = val_cell.replace(',', "").parse::<f64>()
                {
                    items.push(MacroDataPoint {
                        date: date_cell,
                        value: val,
                        name: "Stock Finance".to_string(),
                    });
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }
}

/// Strip HTML tags and extract text content.
fn extract_html_text_other(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result.trim().to_string()
}
