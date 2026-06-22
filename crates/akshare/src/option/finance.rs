//! Option finance board data from SSE, SZSE, and CFFEX.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Constants from Python cons.py
// ---------------------------------------------------------------------------

const SH_OPTION_URL_50: &str = "http://yunhq.sse.com.cn:32041/v1/sh1/list/self/510050";
const SH_OPTION_URL_300: &str = "http://yunhq.sse.com.cn:32041/v1/sh1/list/self/510300";
const SH_OPTION_URL_500: &str = "http://yunhq.sse.com.cn:32041/v1/sh1/list/self/510500";
const SH_OPTION_URL_KC_50: &str = "http://yunhq.sse.com.cn:32041/v1/sh1/list/self/588000";
const SH_OPTION_URL_KC_50_YFD: &str = "http://yunhq.sse.com.cn:32041/v1/sh1/list/self/588080";

const SH_OPTION_KING_50: &str = "http://yunhq.sse.com.cn:32041/v1/sho/list/tstyle/510050_{}";
const SH_OPTION_KING_300: &str = "http://yunhq.sse.com.cn:32041/v1/sho/list/tstyle/510300_{}";
const SH_OPTION_KING_500: &str = "http://yunhq.sse.com.cn:32041/v1/sho/list/tstyle/510500_{}";
const SH_OPTION_KC_KING_50: &str = "http://yunhq.sse.com.cn:32041/v1/sho/list/tstyle/588000_{}";
const SH_OPTION_KING_50_YFD: &str = "http://yunhq.sse.com.cn:32041/v1/sho/list/tstyle/588080_{}";

const CFFEX_OPTION_URL_IO: &str = "http://www.cffex.com.cn/quote_IO.txt";
const CFFEX_OPTION_URL_MO: &str = "http://www.cffex.com.cn/quote_MO.txt";
const CFFEX_OPTION_URL_HO: &str = "http://www.cffex.com.cn/quote_HO.txt";

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct SseListResponse {
    date: Option<String>,
    time: Option<String>,
    total: Option<usize>,
    list: Option<Vec<Vec<serde_json::Value>>>,
}

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// SSE option underlying daily data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionFinanceSseUnderlying {
    /// Underlying code.
    pub code: String,
    /// Underlying name.
    pub name: String,
    /// Last price.
    pub last_price: f64,
    /// Change.
    pub change: f64,
    /// Change percent.
    pub change_pct: f64,
    /// Amplitude percent.
    pub amplitude_pct: f64,
    /// Volume (lots).
    pub volume: f64,
    /// Amount (10k yuan).
    pub amount: f64,
    /// Update datetime.
    pub update_time: String,
}

/// SSE/SZSE/CFFEX option finance board row.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionFinanceBoardRow {
    /// Date/time string.
    pub datetime: String,
    /// Contract trading code.
    pub contract_code: String,
    /// Current price.
    pub current_price: f64,
    /// Change percent.
    pub change_pct: f64,
    /// Previous settlement price.
    pub prev_settlement: f64,
    /// Exercise price.
    pub exercise_price: f64,
    /// Number of contracts (SSE only).
    pub count: Option<f64>,
    /// Extra fields from CFFEX (instrument, etc.).
    pub extra: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// SSE option underlying daily data.
    ///
    /// `symbol` is one of:
    /// - "\u{534e}\u{590f}\u{4e0a}\u{8bc1}50ETF\u{9009}\u{6743}"
    /// - "\u{534e}\u{6cf0}\u{67cf}\u{745e}\u{6caa}\u{6df1}300ETF\u{9009}\u{6743}"
    /// - "\u{5357}\u{65b9}\u{4e2d}\u{8bc1}500ETF\u{9009}\u{6743}"
    /// - "\u{534e}\u{590f}\u{79d1}\u{521b}50ETF\u{9009}\u{6743}"
    /// - "\u{6613}\u{65b9}\u{8fbe}\u{79d1}\u{521b}50ETF\u{9009}\u{6743}"
    pub async fn option_finance_sse_underlying(
        &self,
        symbol: &str,
    ) -> Result<Vec<OptionFinanceSseUnderlying>> {
        let url = match symbol {
            "\u{534e}\u{590f}\u{4e0a}\u{8bc1}50ETF\u{9009}\u{6743}" => SH_OPTION_URL_50,
            "\u{534e}\u{6cf0}\u{67cf}\u{745e}\u{6caa}\u{6df1}300ETF\u{9009}\u{6743}" => {
                SH_OPTION_URL_300
            }
            "\u{5357}\u{65b9}\u{4e2d}\u{8bc1}500ETF\u{9009}\u{6743}" => SH_OPTION_URL_500,
            "\u{534e}\u{590f}\u{79d1}\u{521b}50ETF\u{9009}\u{6743}" => SH_OPTION_URL_KC_50,
            "\u{6613}\u{65b9}\u{8fbe}\u{79d1}\u{521b}50ETF\u{9009}\u{6743}" => {
                SH_OPTION_URL_KC_50_YFD
            }
            other => {
                return Err(Error::invalid_input(format!(
                    "unsupported SSE underlying symbol: {other}"
                )));
            }
        };

        let resp: SseListResponse = self
            .get(url)
            .query(&[(
                "select",
                "select: code,name,last,change,chg_rate,amp_rate,volume,amount,prev_close",
            )])
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let date = resp.date.unwrap_or_default();
        let time = resp.time.unwrap_or_default();
        let update_time = format!("{date}{time}");

        let list = resp.list.unwrap_or_default();
        let mut rows = Vec::with_capacity(list.len());

        for item in &list {
            if item.len() < 9 {
                continue;
            }
            rows.push(OptionFinanceSseUnderlying {
                code: item[0].as_str().unwrap_or("").to_string(),
                name: item[1].as_str().unwrap_or("").to_string(),
                last_price: json_to_f64(&item[2]),
                change: json_to_f64(&item[3]),
                change_pct: json_to_f64(&item[4]),
                amplitude_pct: json_to_f64(&item[5]),
                volume: json_to_f64(&item[6]),
                amount: json_to_f64(&item[7]),
                update_time: update_time.clone(),
            });
        }

        Ok(rows)
    }

    /// Option finance board data.
    ///
    /// `symbol` is one of the supported option symbols:
    /// - "\u{534e}\u{590f}\u{4e0a}\u{8bc1}50ETF\u{9009}\u{6743}"
    /// - "\u{534e}\u{6cf0}\u{67cf}\u{745e}\u{6caa}\u{6df1}300ETF\u{9009}\u{6743}"
    /// - "\u{5357}\u{65b9}\u{4e2d}\u{8bc1}500ETF\u{9009}\u{6743}"
    /// - "\u{534e}\u{590f}\u{79d1}\u{521b}50ETF\u{9009}\u{6743}"
    /// - "\u{6613}\u{65b9}\u{8fbe}\u{79d1}\u{521b}50ETF\u{9009}\u{6743}"
    /// - "\u{5609}\u{5b9e}\u{6caa}\u{6df1}300ETF\u{9009}\u{6743}"
    /// - "\u{6caa}\u{6df1}300\u{80a1}\u{6307}\u{9009}\u{6743}"
    /// - "\u{4e2d}\u{8bc1}1000\u{80a1}\u{6307}\u{9009}\u{6743}"
    /// - "\u{4e0a}\u{8bc1}50\u{80a1}\u{6307}\u{9009}\u{6743}"
    ///
    /// `end_month` is the expiry month, e.g. "2306".
    pub async fn option_finance_board(
        &self,
        symbol: &str,
        end_month: &str,
    ) -> Result<Vec<OptionFinanceBoardRow>> {
        let month_suffix = if end_month.len() >= 2 {
            &end_month[end_month.len() - 2..]
        } else {
            end_month
        };

        match symbol {
            "\u{534e}\u{590f}\u{4e0a}\u{8bc1}50ETF\u{9009}\u{6743}" => {
                self.fetch_sse_option_board(SH_OPTION_KING_50, month_suffix)
                    .await
            }
            "\u{534e}\u{6cf0}\u{67cf}\u{745e}\u{6caa}\u{6df1}300ETF\u{9009}\u{6743}" => {
                self.fetch_sse_option_board(SH_OPTION_KING_300, month_suffix)
                    .await
            }
            "\u{5357}\u{65b9}\u{4e2d}\u{8bc1}500ETF\u{9009}\u{6743}" => {
                self.fetch_sse_option_board(SH_OPTION_KING_500, month_suffix)
                    .await
            }
            "\u{534e}\u{590f}\u{79d1}\u{521b}50ETF\u{9009}\u{6743}" => {
                self.fetch_sse_option_board(SH_OPTION_KC_KING_50, month_suffix)
                    .await
            }
            "\u{6613}\u{65b9}\u{8fbe}\u{79d1}\u{521b}50ETF\u{9009}\u{6743}" => {
                self.fetch_sse_option_board(SH_OPTION_KING_50_YFD, month_suffix)
                    .await
            }
            "\u{5609}\u{5b9e}\u{6caa}\u{6df1}300ETF\u{9009}\u{6743}" => {
                self.fetch_szse_option_board(month_suffix).await
            }
            "\u{6caa}\u{6df1}300\u{80a1}\u{6307}\u{9009}\u{6743}" => {
                self.fetch_cffex_option_board(CFFEX_OPTION_URL_IO, month_suffix)
                    .await
            }
            "\u{4e2d}\u{8bc1}1000\u{80a1}\u{6307}\u{9009}\u{6743}" => {
                self.fetch_cffex_option_board(CFFEX_OPTION_URL_MO, month_suffix)
                    .await
            }
            "\u{4e0a}\u{8bc1}50\u{80a1}\u{6307}\u{9009}\u{6743}" => {
                self.fetch_cffex_option_board(CFFEX_OPTION_URL_HO, month_suffix)
                    .await
            }
            other => Err(Error::invalid_input(format!(
                "unsupported finance board symbol: {other}"
            ))),
        }
    }

    // -- private helpers ----------------------------------------------------

    async fn fetch_sse_option_board(
        &self,
        url_template: &str,
        month_suffix: &str,
    ) -> Result<Vec<OptionFinanceBoardRow>> {
        let url = url_template.replace("{}", month_suffix);

        let resp: SseListResponse = self
            .get(&url)
            .query(&[("select", "contractid,last,chg_rate,presetpx,exepx")])
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let date = resp.date.unwrap_or_default();
        let time = resp.time.unwrap_or_default();
        let datetime = format!("{date}{time}");
        let total = resp.total.unwrap_or(0);
        let list = resp.list.unwrap_or_default();

        let mut rows = Vec::with_capacity(list.len());
        for item in &list {
            if item.len() < 5 {
                continue;
            }
            rows.push(OptionFinanceBoardRow {
                datetime: datetime.clone(),
                contract_code: item[0].as_str().unwrap_or("").to_string(),
                current_price: json_to_f64(&item[1]),
                change_pct: json_to_f64(&item[2]),
                prev_settlement: json_to_f64(&item[3]),
                exercise_price: json_to_f64(&item[4]),
                count: Some(total as f64),
                extra: None,
            });
        }

        Ok(rows)
    }

    async fn fetch_szse_option_board(
        &self,
        month_suffix: &str,
    ) -> Result<Vec<OptionFinanceBoardRow>> {
        let url = "http://www.szse.cn/api/report/ShowReport/data";
        let mut all_rows = Vec::new();

        // First request to get page count
        let body = self
            .get(url)
            .query(&[
                ("SHOWTYPE", "JSON"),
                ("CATALOGID", "ysplbrb"),
                ("TABKEY", "tab1"),
                ("PAGENO", "1"),
                ("random", "0.10642298535346595"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        let json_val: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| Error::decode(format!("szse board json: {e}")))?;

        let page_count = json_val
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|v| v.get("metadata"))
            .and_then(|m| m.get("pagecount"))
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1) as u64;

        // Fetch all pages
        for page in 1..=page_count {
            let page_str = page.to_string();
            let body = self
                .get(url)
                .query(&[
                    ("SHOWTYPE", "JSON"),
                    ("CATALOGID", "ysplbrb"),
                    ("TABKEY", "tab1"),
                    ("PAGENO", page_str.as_str()),
                    ("random", "0.10642298535346595"),
                ])
                .send()
                .await
                .map_err(Error::from)?
                .text()
                .await
                .map_err(Error::from)?;

            let json_val: serde_json::Value = serde_json::from_str(&body)
                .map_err(|e| Error::decode(format!("szse board page json: {e}")))?;

            let data = json_val
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|v| v.get("data"))
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default();

            for item in &data {
                if let Some(arr) = item.as_array() {
                    if arr.len() < 8 {
                        continue;
                    }
                    // Filter by end_month
                    let exercise_date = arr[6].as_str().unwrap_or("");
                    if let Some(mm) = extract_month(exercise_date)
                        && mm != month_suffix
                    {
                        continue;
                    }

                    all_rows.push(OptionFinanceBoardRow {
                        datetime: String::new(),
                        contract_code: arr[0].as_str().unwrap_or("").to_string(),
                        current_price: 0.0,
                        change_pct: 0.0,
                        prev_settlement: 0.0,
                        exercise_price: json_to_f64(&arr[4]),
                        count: None,
                        extra: Some(serde_json::json!({
                            "contract_name": arr[1].as_str().unwrap_or(""),
                            "underlying": arr[2].as_str().unwrap_or(""),
                            "type": arr[3].as_str().unwrap_or(""),
                            "contract_unit": json_to_f64(&arr[5]),
                            "exercise_date": exercise_date,
                            "delivery_date": arr[7].as_str().unwrap_or(""),
                        })),
                    });
                }
            }
        }

        Ok(all_rows)
    }

    async fn fetch_cffex_option_board(
        &self,
        url: &str,
        month_suffix: &str,
    ) -> Result<Vec<OptionFinanceBoardRow>> {
        let body = self
                        .get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        // Parse CSV-like data
        let lines: Vec<&str> = body.lines().collect();
        if lines.is_empty() {
            return Ok(vec![]);
        }

        let headers: Vec<&str> = lines[0].split(',').map(str::trim).collect();
        let instrument_idx = headers.iter().position(|h| *h == "instrument");

        let mut rows = Vec::new();
        for line in lines.iter().skip(1) {
            let fields: Vec<&str> = line.split(',').map(str::trim).collect();
            if fields.len() < headers.len() {
                continue;
            }

            // Filter by end_month
            if let Some(idx) = instrument_idx {
                let instrument = fields[idx];
                // Extract month from instrument code like "IO2306-C-4000" -> "06"
                if let Some(code_part) = instrument.split('-').next()
                    && code_part.len() >= 6
                {
                    let mm = &code_part[code_part.len() - 2..];
                    if mm != month_suffix {
                        continue;
                    }
                }
            }

            let extra = serde_json::to_value(
                headers
                    .iter()
                    .zip(fields.iter())
                    .map(|(h, v)| (h.to_string(), v.to_string()))
                    .collect::<Vec<_>>(),
            )
            .ok();

            rows.push(OptionFinanceBoardRow {
                datetime: String::new(),
                contract_code: fields.first().unwrap_or(&"").to_string(),
                current_price: 0.0,
                change_pct: 0.0,
                prev_settlement: 0.0,
                exercise_price: 0.0,
                count: None,
                extra,
            });
        }

        Ok(rows)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn json_to_f64(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.trim().parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

/// Extract the 2-digit month from a date string like "2023-06-15" or "20230615".
fn extract_month(date_str: &str) -> Option<&str> {
    if date_str.len() >= 7 && date_str.as_bytes()[4] == b'-' {
        Some(&date_str[5..7])
    } else if date_str.len() >= 6 {
        Some(&date_str[4..6])
    } else {
        None
    }
}
