//! Option contract info from openctp.cn.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// Option contract info from openctp.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptionContractInfoCtp {
    /// Exchange ID.
    pub exchange_id: String,
    /// Instrument ID.
    pub instrument_id: String,
    /// Instrument name.
    pub instrument_name: String,
    /// Product class.
    pub product_class: String,
    /// Product ID.
    pub product_id: String,
    /// Volume multiple.
    pub volume_multiple: f64,
    /// Minimum price tick.
    pub price_tick: f64,
    /// Long margin ratio by money.
    pub long_margin_ratio: f64,
    /// Short margin ratio by money.
    pub short_margin_ratio: f64,
    /// Long margin per lot.
    pub long_margin_per_lot: f64,
    /// Short margin per lot.
    pub short_margin_per_lot: f64,
    /// Open fee ratio by money.
    pub open_fee_ratio: f64,
    /// Open fee per lot.
    pub open_fee_per_lot: f64,
    /// Close fee ratio by money.
    pub close_fee_ratio: f64,
    /// Close fee per lot.
    pub close_fee_per_lot: f64,
    /// Close-today fee ratio by money.
    pub close_today_fee_ratio: f64,
    /// Close-today fee per lot.
    pub close_today_fee_per_lot: f64,
    /// Delivery year.
    pub delivery_year: i64,
    /// Delivery month.
    pub delivery_month: i64,
    /// Open date.
    pub open_date: String,
    /// Expire date.
    pub expire_date: String,
    /// Delivery date.
    pub delivery_date: String,
    /// Underlying instrument ID.
    pub underlying_instrument_id: String,
    /// Underlying multiple.
    pub underlying_multiple: f64,
    /// Options type (call/put).
    pub options_type: String,
    /// Strike price.
    pub strike_price: f64,
    /// Instrument life phase.
    pub inst_life_phase: String,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get all option contract info from openctp.
    pub async fn option_contract_info_ctp(&self) -> Result<Vec<OptionContractInfoCtp>> {
        let url = "http://dict.openctp.cn/instruments?types=option";

        let resp: serde_json::Value = self
            .get(url)
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let data = resp
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut rows = Vec::with_capacity(data.len());
        for item in &data {
            rows.push(OptionContractInfoCtp {
                exchange_id: json_str(item, "ExchangeID"),
                instrument_id: json_str(item, "InstrumentID"),
                instrument_name: json_str(item, "InstrumentName"),
                product_class: json_str(item, "ProductClass"),
                product_id: json_str(item, "ProductID"),
                volume_multiple: json_f64(item, "VolumeMultiple"),
                price_tick: json_f64(item, "PriceTick"),
                long_margin_ratio: json_f64(item, "LongMarginRatioByMoney"),
                short_margin_ratio: json_f64(item, "ShortMarginRatioByMoney"),
                long_margin_per_lot: json_f64(item, "LongMarginRatioByVolume"),
                short_margin_per_lot: json_f64(item, "ShortMarginRatioByVolume"),
                open_fee_ratio: json_f64(item, "OpenRatioByMoney"),
                open_fee_per_lot: json_f64(item, "OpenRatioByVolume"),
                close_fee_ratio: json_f64(item, "CloseRatioByMoney"),
                close_fee_per_lot: json_f64(item, "CloseRatioByVolume"),
                close_today_fee_ratio: json_f64(item, "CloseTodayRatioByMoney"),
                close_today_fee_per_lot: json_f64(item, "CloseTodayRatioByVolume"),
                delivery_year: json_i64(item, "DeliveryYear"),
                delivery_month: json_i64(item, "DeliveryMonth"),
                open_date: json_str(item, "OpenDate"),
                expire_date: json_str(item, "ExpireDate"),
                delivery_date: json_str(item, "DeliveryDate"),
                underlying_instrument_id: json_str(item, "UnderlyingInstrID"),
                underlying_multiple: json_f64(item, "UnderlyingMultiple"),
                options_type: json_str(item, "OptionsType"),
                strike_price: json_f64(item, "StrikePrice"),
                inst_life_phase: json_str(item, "InstLifePhase"),
            });
        }

        Ok(rows)
    }
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
