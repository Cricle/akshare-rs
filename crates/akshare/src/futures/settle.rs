#![allow(dead_code)]
//! Futures settlement parameters from Chinese exchanges.
//!
//! Covers CFFEX, CZCE, SHFE, INE, and GFEX settlement data.

use std::sync::LazyLock;

use serde_json;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;

static RE_ALPHA: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-zA-Z]+").unwrap());

fn f64_val(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.replace(',', "").parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

fn str_val(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn extract_variety(sym: &str) -> String {
    RE_ALPHA
        .find(sym)
        .map(|m| m.as_str().to_uppercase())
        .unwrap_or_default()
}

impl AkShareClient {
    /// CFFEX settlement parameters.
    pub async fn futures_settle_cffex(&self, date: &str) -> Result<Vec<Row>> {
        let url = format!(
            "http://www.cffex.com.cn/sj/jscs/{}/{}/{}_1.csv",
            &date[..4],
            &date[4..6],
            date
        );
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        if body.trim().is_empty() || body.starts_with('<') {
            return Ok(vec![]);
        }

        let lines: Vec<&str> = body.lines().collect();
        if lines.len() < 3 {
            return Ok(vec![]);
        }

        let mut items = Vec::new();
        for line in &lines[1..] {
            let fields: Vec<&str> = line.split(',').map(str::trim).collect();
            if fields.len() < 6 {
                continue;
            }
            let sym = fields[0].to_string();
            if sym.is_empty() || !sym.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                continue;
            }
            let mut row = Row::new();
            row.insert("date".into(), serde_json::json!(date));
            row.insert("symbol".into(), serde_json::json!(sym));
            row.insert("variety".into(), serde_json::json!(extract_variety(&sym)));
            row.insert("long_margin_ratio".into(), serde_json::json!(fields[1]));
            row.insert("short_margin_ratio".into(), serde_json::json!(fields[2]));
            row.insert("trade_fee_ratio".into(), serde_json::json!(fields[3]));
            row.insert("delivery_fee_ratio".into(), serde_json::json!(fields[4]));
            row.insert("close_today_fee_ratio".into(), serde_json::json!(fields[5]));
            items.push(row);
        }
        Ok(items)
    }

    /// CZCE settlement parameters.
    pub async fn futures_settle_czce(&self, date: &str) -> Result<Vec<Row>> {
        let year = &date[..4];
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{year}/{date}/FutureDataClearParams.txt"
        );
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        if body.is_empty() {
            return Ok(vec![]);
        }

        let lines: Vec<&str> = body.lines().collect();
        if lines.len() < 3 {
            return Ok(vec![]);
        }

        let mut items = Vec::new();
        for line in &lines[2..] {
            let fields: Vec<&str> = line.split('|').map(str::trim).collect();
            if fields.len() < 10 {
                continue;
            }
            let sym = fields[0].to_string();
            if sym.is_empty() || sym.contains("小计") || sym.contains("合计") {
                continue;
            }
            let mut row = Row::new();
            row.insert("date".into(), serde_json::json!(date));
            row.insert("symbol".into(), serde_json::json!(sym));
            row.insert("variety".into(), serde_json::json!(extract_variety(&sym)));
            row.insert("settle_price".into(), serde_json::json!(fields[1]));
            row.insert("is_single_market".into(), serde_json::json!(fields[2]));
            row.insert("single_market_days".into(), serde_json::json!(fields[3]));
            row.insert("margin_ratio".into(), serde_json::json!(fields[4]));
            row.insert("limit_ratio".into(), serde_json::json!(fields[5]));
            row.insert("trade_fee".into(), serde_json::json!(fields[6]));
            row.insert("fee_type".into(), serde_json::json!(fields[7]));
            row.insert("delivery_fee".into(), serde_json::json!(fields[8]));
            row.insert("close_today_fee".into(), serde_json::json!(fields[9]));
            items.push(row);
        }
        Ok(items)
    }

    /// SHFE settlement parameters.
    pub async fn futures_settle_shfe(&self, date: &str) -> Result<Vec<Row>> {
        let url = format!("https://www.shfe.com.cn/data/tradedata/future/dailydata/js{date}.dat");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["o_cursor"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in rows {
            let sym = row
                .get("INSTRUMENTID")
                .or_else(|| row.get("symbol"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if sym.is_empty() {
                continue;
            }
            let mut r = Row::new();
            r.insert("date".into(), serde_json::json!(date));
            r.insert("symbol".into(), serde_json::json!(sym));
            r.insert("variety".into(), serde_json::json!(extract_variety(&sym)));
            for (key, col) in [
                ("TRADEFEERATIO", "trade_fee_ratio"),
                ("TTRADEFEERATIO", "close_today_fee_ratio"),
                ("COMMODITYDELIVFEERATIO", "delivery_fee_ratio"),
                ("SPECLONGMARGINRATIO", "spec_long_margin_ratio"),
                ("HEDGLONGMARGINRATIO", "hedge_long_margin_ratio"),
                ("SPECSHORTMARGINRATIO", "spec_short_margin_ratio"),
                ("HEDGSHORTMARGINRATIO", "hedge_short_margin_ratio"),
                ("SETTLEMENTPRICE", "settle_price"),
            ] {
                if let Some(val) = row.get(key) {
                    r.insert(col.into(), val.clone());
                }
            }
            items.push(r);
        }
        Ok(items)
    }

    /// INE settlement parameters.
    pub async fn futures_settle_ine(&self, date: &str) -> Result<Vec<Row>> {
        let url = format!("https://www.ine.cn/data/tradedata/future/dailydata/js{date}.dat");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["o_cursor"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in rows {
            let sym = row
                .get("INSTRUMENTID")
                .or_else(|| row.get("symbol"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if sym.is_empty() {
                continue;
            }
            let mut r = Row::new();
            r.insert("date".into(), serde_json::json!(date));
            r.insert("symbol".into(), serde_json::json!(sym));
            r.insert("variety".into(), serde_json::json!(extract_variety(&sym)));
            for (key, col) in [
                ("TRADEFEERATIO", "trade_fee_ratio"),
                ("TTRADEFEERATIO", "close_today_fee_ratio"),
                ("COMMODITYDELIVFEERATIO", "delivery_fee_ratio"),
                ("SPECLONGMARGINRATIO", "spec_long_margin_ratio"),
                ("HEDGLONGMARGINRATIO", "hedge_long_margin_ratio"),
                ("SPECSHORTMARGINRATIO", "spec_short_margin_ratio"),
                ("HEDGSHORTMARGINRATIO", "hedge_short_margin_ratio"),
                ("SETTLEMENTPRICE", "settle_price"),
            ] {
                if let Some(val) = row.get(key) {
                    r.insert(col.into(), val.clone());
                }
            }
            items.push(r);
        }
        Ok(items)
    }

    /// GFEX settlement parameters.
    pub async fn futures_settle_gfex(&self, _date: &str) -> Result<Vec<Row>> {
        let url = "http://www.gfex.com.cn/u/interfacesWebTtQueryTradPara/loadDayList";
        let body = self
            .post(url)
            .form(&[("trade_type", "0")])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        if data["code"].as_str() != Some("0") {
            return Ok(vec![]);
        }
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for mut row in rows {
            let sym = row["contractId"].as_str().unwrap_or("").to_string();
            if sym.contains('-') || sym.is_empty() {
                continue;
            }
            let mut r = Row::new();
            r.insert("symbol".into(), serde_json::json!(sym));
            r.insert("variety".into(), serde_json::json!(extract_variety(&sym)));
            r.insert(
                "spec_buy_rate".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("specBuyRate"))
                    .unwrap_or_default(),
            );
            r.insert(
                "hedge_buy_rate".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("hedgeBuyRate"))
                    .unwrap_or_default(),
            );
            r.insert(
                "rise_limit_rate".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("riseLimitRate"))
                    .unwrap_or_default(),
            );
            r.insert(
                "client_buy_posi_quota".into(),
                row["clientBuyPosiQuota"].clone(),
            );
            items.push(r);
        }
        Ok(items)
    }

    /// SHFE JS (结算参数) data for a given symbol.
    ///
    /// Returns settlement parameters from SHFE for the given symbol.
    pub async fn futures_stock_shfe_js(&self, symbol: &str) -> Result<Vec<Row>> {
        // Get today's date for settlement data
        let today = chrono::Utc::now().format("%Y%m%d").to_string();
        let all = self.futures_settle_shfe(&today).await?;
        let filtered: Vec<Row> = all
            .into_iter()
            .filter(|r| {
                r.get("symbol")
                    .and_then(|v| v.as_str())
                    .is_some_and(|s| s.contains(symbol) || symbol.is_empty())
            })
            .collect();
        Ok(filtered)
    }

    /// Unified settlement data across exchanges.
    ///
    /// `market`: "CFFEX", "CZCE", "SHFE", "INE", "GFEX"
    pub async fn futures_settle(&self, date: &str, market: &str) -> Result<Vec<Row>> {
        match market.to_uppercase().as_str() {
            "CFFEX" => self.futures_settle_cffex(date).await,
            "CZCE" => self.futures_settle_czce(date).await,
            "SHFE" => self.futures_settle_shfe(date).await,
            "INE" => self.futures_settle_ine(date).await,
            "GFEX" => self.futures_settle_gfex(date).await,
            _ => Err(Error::invalid_input(format!(
                "unsupported market for settlement: {market}"
            ))),
        }
    }
}
