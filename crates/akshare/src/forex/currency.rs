//! Currency data from CurrencyBeacon / currencyscoop.com API.
//!
//! These functions require an API key from currencyscoop.com.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Latest exchange rates from CurrencyBeacon.
    ///
    /// `base` is the base currency (e.g., "USD").
    /// `symbols` is a comma-separated list of target currencies.
    /// `api_key` is the CurrencyBeacon API key.
    pub async fn currency_latest(
        &self,
        base: &str,
        symbols: &str,
        api_key: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = "https://api.currencyscoop.com/v1/latest";
        let resp: serde_json::Value = self
            .get(url)
            .query(&[("base", base), ("symbols", symbols), ("api_key", api_key)])
            .send()
            .await?
            .json()
            .await?;

        let response = resp
            .get("response")
            .ok_or_else(|| Error::decode("currency_latest: no response field"))?;

        let rates = response
            .get("rates")
            .and_then(|r| r.as_object())
            .ok_or_else(|| Error::decode("currency_latest: no rates"))?;

        let date = response.str_or(&["date"], "");

        let mut items = Vec::new();
        for (currency, rate) in rates {
            items.push(MacroDataPoint {
                date: date.clone(),
                value: rate.as_f64().unwrap_or(0.0),
                name: format!("{base}/{currency}"),
            });
        }
        Ok(items)
    }

    /// Historical exchange rates from CurrencyBeacon.
    ///
    /// `date` is in "YYYY-MM-DD" format.
    pub async fn currency_history(
        &self,
        base: &str,
        date: &str,
        symbols: &str,
        api_key: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = "https://api.currencyscoop.com/v1/historical";
        let resp: serde_json::Value = self
            .get(url)
            .query(&[
                ("base", base),
                ("date", date),
                ("symbols", symbols),
                ("api_key", api_key),
            ])
            .send()
            .await?
            .json()
            .await?;

        let response = resp
            .get("response")
            .ok_or_else(|| Error::decode("currency_history: no response field"))?;

        let rates = response
            .get("rates")
            .and_then(|r| r.as_object())
            .ok_or_else(|| Error::decode("currency_history: no rates"))?;

        let mut items = Vec::new();
        for (currency, rate) in rates {
            items.push(MacroDataPoint {
                date: date.to_string(),
                value: rate.as_f64().unwrap_or(0.0),
                name: format!("{base}/{currency}"),
            });
        }
        Ok(items)
    }

    /// Time-series exchange rates from CurrencyBeacon.
    ///
    /// Requires special API access.
    pub async fn currency_time_series(
        &self,
        base: &str,
        start_date: &str,
        end_date: &str,
        symbols: &str,
        api_key: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = "https://api.currencyscoop.com/v1/timeseries";
        let resp: serde_json::Value = self
            .get(url)
            .query(&[
                ("base", base),
                ("start_date", start_date),
                ("end_date", end_date),
                ("symbols", symbols),
                ("api_key", api_key),
            ])
            .send()
            .await?
            .json()
            .await?;

        let response = resp
            .get("response")
            .ok_or_else(|| Error::decode("currency_time_series: no response"))?;

        let rates = response
            .get("rates")
            .and_then(|r| r.as_object())
            .ok_or_else(|| Error::decode("currency_time_series: no rates"))?;

        let mut items = Vec::new();
        for (date, day_rates) in rates {
            if let Some(obj) = day_rates.as_object() {
                for (currency, rate) in obj {
                    items.push(MacroDataPoint {
                        date: date.clone(),
                        value: rate.as_f64().unwrap_or(0.0),
                        name: format!("{base}/{currency}"),
                    });
                }
            }
        }
        Ok(items)
    }

    /// List all supported currencies from CurrencyBeacon.
    pub async fn currency_currencies(
        &self,
        c_type: &str,
        api_key: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = "https://api.currencyscoop.com/v1/currencies";
        let resp: serde_json::Value = self
            .get(url)
            .query(&[("type", c_type), ("api_key", api_key)])
            .send()
            .await?
            .json()
            .await?;

        let response = resp
            .get("response")
            .and_then(|r| r.get("fiat"))
            .or_else(|| resp.get("response"))
            .ok_or_else(|| Error::decode("currency_currencies: no response"))?;

        let mut items = Vec::new();
        if let Some(obj) = response.as_object() {
            for (code, info) in obj {
                let name = info.str_or(&["currency_name"], "");
                items.push(MacroDataPoint {
                    date: code.clone(),
                    value: 0.0,
                    name,
                });
            }
        }
        Ok(items)
    }

    /// Currency conversion via CurrencyBeacon.
    pub async fn currency_convert(
        &self,
        from: &str,
        to: &str,
        amount: f64,
        api_key: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = "https://api.currencyscoop.com/v1/convert";
        let resp: serde_json::Value = self
            .get(url)
            .query(&[
                ("from", from),
                ("to", to),
                ("amount", &amount.to_string()),
                ("api_key", api_key),
            ])
            .send()
            .await?
            .json()
            .await?;

        let response = resp
            .get("response")
            .ok_or_else(|| Error::decode("currency_convert: no response"))?;

        let value = response.f64_or(&["value"], 0.0);

        let timestamp = response
            .get("timestamp")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(0);

        let date = chrono::DateTime::from_timestamp(timestamp, 0)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_default();

        Ok(vec![MacroDataPoint {
            date,
            value,
            name: format!("{from}/{to}"),
        }])
    }

    /// Bank of China historical rates via Sina Finance.
    ///
    /// `symbol`: currency name in Chinese (e.g., "美元", "英镑", "欧元").
    /// `start_date` and `end_date` in "YYYYMMDD" format.
    pub async fn currency_boc(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        // Map Chinese currency names to Sina codes
        let money_code = match symbol {
            "美元" => "USD",
            "英镑" => "GBP",
            "欧元" => "EUR",
            "港币" => "HKD",
            "日元" => "JPY",
            "加拿大元" => "CAD",
            "澳大利亚元" => "AUD",
            "新西兰元" => "NZD",
            "新加坡元" => "SGD",
            "瑞士法郎" => "CHF",
            "瑞典克朗" => "SEK",
            "丹麦克朗" => "DKK",
            "挪威克朗" => "NOK",
            "澳门元" => "MOP",
            "泰国铢" => "THB",
            "菲律宾比索" => "PHP",
            "韩国元" => "KRW",
            _ => {
                return Err(Error::invalid_input(format!("unknown currency: {symbol}")));
            }
        };

        let url = "http://biz.finance.sina.com.cn/forex/forex.php";
        let formatted_start = format!(
            "{}-{}-{}",
            &start_date[..4],
            &start_date[4..6],
            &start_date[6..8]
        );
        let formatted_end = format!("{}-{}-{}", &end_date[..4], &end_date[4..6], &end_date[6..8]);

        let body = self
            .get(url)
            .query(&[
                ("money_code", money_code),
                ("type", "0"),
                ("startdate", &formatted_start),
                ("enddate", &formatted_end),
                ("page", "1"),
                ("call_type", "ajax"),
            ])
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        // Parse HTML table rows
        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.contains("<td") {
                let cells = extract_table_cells(trimmed);
                if cells.len() >= 5 {
                    let date = cells[0].clone();
                    let buy_rate: f64 = cells[1].parse().unwrap_or(0.0);
                    items.push(MacroDataPoint {
                        date,
                        value: buy_rate,
                        name: format!("BOC {symbol}"),
                    });
                }
            }
        }
        Ok(items)
    }

    /// SAFE (State Administration of Foreign Exchange) RMB central parity rates.
    ///
    /// Returns the latest RMB exchange rate central parity published by SAFE.
    pub async fn currency_boc_safe(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://www.safe.gov.cn/AppStructured/hlw/RMBQuery.do";
        let body = self
            .post(url)
            .form(&[
                ("startDate", "2020-01-01"),
                (
                    "endDate",
                    &chrono::Utc::now().format("%Y-%m-%d").to_string(),
                ),
                ("queryYN", "true"),
            ])
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        // Parse HTML table
        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.contains("<td") {
                let cells = extract_table_cells(trimmed);
                if cells.len() >= 2 {
                    let date = cells[0].clone();
                    for (i, cell) in cells.iter().enumerate().skip(1) {
                        if let Ok(val) = cell.parse::<f64>() {
                            items.push(MacroDataPoint {
                                date: date.clone(),
                                value: val,
                                name: format!("SAFE Rate Col{i}"),
                            });
                        }
                    }
                }
            }
        }
        Ok(items)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_table_cells(html: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut remaining = html;
    while let Some(start) = remaining.find("<td") {
        let after_td = &remaining[start..];
        if let Some(content_start) = after_td.find('>') {
            let content = &after_td[content_start + 1..];
            if let Some(content_end) = content.find("</td>") {
                let cell_text = content[..content_end].trim().to_string();
                cells.push(cell_text);
                remaining = &content[content_end + 5..];
            } else {
                break;
            }
        } else {
            break;
        }
    }
    cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_table_cells() {
        let html = "<td>2023-01-01</td><td>6.7234</td><td>6.7500</td>";
        let cells = extract_table_cells(html);
        assert_eq!(cells.len(), 3);
        assert_eq!(cells[0], "2023-01-01");
        assert_eq!(cells[1], "6.7234");
    }
}
