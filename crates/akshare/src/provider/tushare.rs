//! Tushare Pro API helpers.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, TradeCalendarItem};

#[derive(Debug, Deserialize)]
struct TushareResponse {
    code: Option<i64>,
    msg: Option<String>,
    data: Option<TushareData>,
}

#[derive(Debug, Deserialize)]
struct TushareData {
    #[serde(default)]
    fields: Vec<String>,
    #[serde(default)]
    items: Vec<Vec<serde_json::Value>>,
}

impl AkShareClient {
    /// Generic Tushare API call.
    async fn tushare_call(
        &self,
        api_name: &str,
        params: serde_json::Value,
        fields: &str,
    ) -> Result<TushareData> {
        let token = self
            .tushare_token
            .as_ref()
            .ok_or_else(|| Error::missing_credentials("tushare_token not set"))?;

        let body = serde_json::json!({
            "api_name": api_name,
            "token": token,
            "params": params,
            "fields": fields,
        });

        let resp: TushareResponse = self
            .post("https://api.tushare.pro")
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        if resp.code != Some(0) {
            let msg = resp
                .msg
                .unwrap_or_else(|| "unknown tushare error".to_string());
            return Err(Error::upstream(format!("tushare: {msg}")));
        }

        resp.data
            .ok_or_else(|| Error::upstream("tushare: empty response"))
    }

    /// Get daily candles from Tushare.
    pub(crate) async fn tushare_daily(
        &self,
        ts_code: &str,
        start: &str,
        end: &str,
    ) -> Result<Vec<CandlePoint>> {
        let params = serde_json::json!({
            "ts_code": ts_code,
            "start_date": start,
            "end_date": end,
        });
        let data = self
            .tushare_call(
                "daily",
                params,
                "trade_date,open,high,low,close,vol,amount,pct_chg,change,turnover_rate",
            )
            .await?;

        let fields = &data.fields;
        let mut items = Vec::with_capacity(data.items.len());
        for row in &data.items {
            let get = |name: &str| -> &serde_json::Value {
                fields
                    .iter()
                    .position(|f| f == name)
                    .and_then(|i| row.get(i))
                    .unwrap_or(&serde_json::Value::Null)
            };
            items.push(CandlePoint {
                trade_date: get("trade_date").as_str().unwrap_or("").to_string(),
                open: get("open").as_f64().unwrap_or(0.0),
                close: get("close").as_f64().unwrap_or(0.0),
                high: get("high").as_f64().unwrap_or(0.0),
                low: get("low").as_f64().unwrap_or(0.0),
                volume: get("vol").as_f64().unwrap_or(0.0) as i64,
                amount: get("amount").as_f64().unwrap_or(0.0),
                amplitude_pct: 0.0,
                change_pct: get("pct_chg").as_f64().unwrap_or(0.0),
                change_amount: get("change").as_f64().unwrap_or(0.0),
                turnover_pct: get("turnover_rate").as_f64().unwrap_or(0.0),
            });
        }
        Ok(items)
    }

    /// Trade calendar from Tushare.
    pub(crate) async fn tushare_trade_calendar(
        &self,
        exchange: &str,
        start: &str,
        end: &str,
    ) -> Result<Vec<TradeCalendarItem>> {
        let params = serde_json::json!({
            "exchange": exchange,
            "start_date": start,
            "end_date": end,
        });
        let data = self
            .tushare_call(
                "trade_cal",
                params,
                "exchange,cal_date,is_open,pretrade_date",
            )
            .await?;

        let fields = &data.fields;
        let mut items = Vec::with_capacity(data.items.len());
        for row in &data.items {
            let get = |name: &str| -> &serde_json::Value {
                fields
                    .iter()
                    .position(|f| f == name)
                    .and_then(|i| row.get(i))
                    .unwrap_or(&serde_json::Value::Null)
            };
            items.push(TradeCalendarItem {
                exchange: get("exchange").as_str().unwrap_or("").to_string(),
                calendar_date: get("cal_date").as_str().unwrap_or("").to_string(),
                is_open: get("is_open").as_i64().unwrap_or(0) == 1,
                previous_trade_date: get("pretrade_date")
                    .as_str()
                    .map(std::string::ToString::to_string),
            });
        }
        Ok(items)
    }
}
