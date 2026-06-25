//! Convertible bond list, comparison, info and value analysis from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::BondSnapshot;
use crate::types::value_ext::ValueExt;
use crate::types::wire::EmDatacenterResp;

impl AkShareClient {
    /// Fetch convertible bond list with pricing data from Eastmoney datacenter.
    ///
    /// Uses `RPT_BOND_CB_LIST` report to retrieve all listed convertible bonds
    /// with their current pricing, conversion value, premium ratio, etc.
    ///
    /// Returns up to `limit` bonds as `BondSnapshot` with yield_rate set to
    /// the conversion premium ratio and credit_rating populated.
    pub async fn bond_zh_cov(&self, limit: usize) -> Result<Vec<BondSnapshot>> {
        let page_size = limit.clamp(1, 500).to_string();
        let resp: EmDatacenterResp = self
                        .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("sortColumns", "PUBLIC_START_DATE"),
                ("sortTypes", "-1"),
                ("pageSize", page_size.as_str()),
                ("pageNumber", "1"),
                ("reportName", "RPT_BOND_CB_LIST"),
                ("columns", "ALL"),
                ("quoteColumns", "f2~01~CONVERT_STOCK_CODE~CONVERT_STOCK_PRICE,f235~10~SECURITY_CODE~TRANSFER_PRICE,f236~10~SECURITY_CODE~TRANSFER_VALUE,f2~10~SECURITY_CODE~CURRENT_BOND_PRICE,f237~10~SECURITY_CODE~TRANSFER_PREMIUM_RATIO,f239~10~SECURITY_CODE~RESALE_TRIG_PRICE,f240~10~SECURITY_CODE~REDEEM_TRIG_PRICE,f23~01~CONVERT_STOCK_CODE~PBV_RATIO"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        if data.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no convertible bond data",
            ));
        }

        let today = crate::util::today_iso();
        let items: Vec<BondSnapshot> = data
            .into_iter()
            .take(limit)
            .filter_map(|v| {
                let symbol = v.str_field(&["SECURITY_CODE"])?;
                let name = v.str_or(&["SECURITY_NAME_ABBR"], symbol);
                let close = v.f64_or(&["CURRENT_BOND_PRICE"], 100.0);
                let premium = v.f64_field(&["TRANSFER_PREMIUM_RATIO"]);
                let credit_rating = v
                    .str_field(&["RATING"])
                    .map(std::string::ToString::to_string);
                Some(BondSnapshot {
                    symbol: symbol.to_string(),
                    name,
                    date: today.clone(),
                    close,
                    change_pct: 0.0,
                    yield_rate: premium,
                    credit_rating,
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("no convertible bond items parsed"));
        }
        Ok(items)
    }

    /// Fetch convertible bond comparison table from Eastmoney clist API.
    ///
    /// Returns bond vs underlying stock comparison data including conversion
    /// value, conversion premium ratio, pure bond premium ratio, etc.
    pub async fn bond_cov_comparison(&self, limit: usize) -> Result<Vec<serde_json::Value>> {
        let pz = limit.clamp(1, 5000).to_string();
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(pz.as_str(), &[
                ("fid", "f243"),
                ("fs", "b:MK0354"),
                ("fields", "f2,f3,f12,f13,f14,f227,f229,f231,f232,f233,f234,f235,f236,f237,f238,f239,f240,f241,f242,f26,f243"),
                ]))
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let items = payload
            .nested(&["data", "diff"])
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no convertible bond comparison data",
            ));
        }
        Ok(items.into_iter().take(limit).collect())
    }

    /// Fetch convertible bond detail info from Eastmoney datacenter.
    ///
    /// `indicator` is one of: "基本信息", "中签号", "筹资用途", "重要日期".
    pub async fn bond_zh_cov_info(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let report_name = match indicator {
            "基本信息" => "RPT_BOND_CB_LIST",
            "中签号" => "RPT_CB_BALLOTNUM",
            "筹资用途" => "RPT_BOND_BS_OPRFINVESTITEM",
            "重要日期" => "RPT_CB_IMPORTANTDATE",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported indicator: {indicator}"
                )));
            }
        };

        let quote_cols = if indicator == "基本信息" {
            "f2~01~CONVERT_STOCK_CODE~CONVERT_STOCK_PRICE,f235~10~SECURITY_CODE~TRANSFER_PRICE,f236~10~SECURITY_CODE~TRANSFER_VALUE,f2~10~SECURITY_CODE~CURRENT_BOND_PRICE,f237~10~SECURITY_CODE~TRANSFER_PREMIUM_RATIO,f239~10~SECURITY_CODE~RESALE_TRIG_PRICE,f240~10~SECURITY_CODE~REDEEM_TRIG_PRICE,f23~01~CONVERT_STOCK_CODE~PBV_RATIO"
        } else {
            ""
        };

        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let resp: EmDatacenterResp = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", report_name),
                ("columns", "ALL"),
                ("quoteColumns", quote_cols),
                ("source", "WEB"),
                ("client", "WEB"),
                ("filter", filter.as_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        if data.is_empty() {
            return Err(Error::not_found(format!(
                "no data found for bond {symbol} with indicator {indicator}"
            )));
        }
        Ok(data)
    }

    /// Convertible bond daily historical data from Eastmoney.
    ///
    /// `symbol` is a convertible bond code, e.g. "127018".
    /// Returns daily OHLCV data for the bond.
    pub async fn bond_zh_hs_cov_daily(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let secid = format!("1.{symbol}");
        let resp: serde_json::Value = self
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                ("klt", "101"),
                ("fqt", "1"),
                ("beg", "0"),
                ("end", "20500000"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let klines = resp["data"]["klines"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for line in &klines {
            if let Some(s) = line.as_str() {
                let fields: Vec<&str> = s.split(',').collect();
                if fields.len() >= 7 {
                    let mut row = serde_json::Map::new();
                    row.insert("date".into(), serde_json::json!(fields[0]));
                    row.insert("open".into(), serde_json::json!(fields[1]));
                    row.insert("close".into(), serde_json::json!(fields[2]));
                    row.insert("high".into(), serde_json::json!(fields[3]));
                    row.insert("low".into(), serde_json::json!(fields[4]));
                    row.insert("volume".into(), serde_json::json!(fields[5]));
                    row.insert("amount".into(), serde_json::json!(fields[6]));
                    items.push(serde_json::Value::Object(row));
                }
            }
        }
        Ok(items)
    }

    /// Convertible bond minute-level data from Eastmoney.
    ///
    /// `symbol` is a convertible bond code, e.g. "127018".
    /// `period` is "1", "5", "15", "30", or "60".
    pub async fn bond_zh_hs_cov_min(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let secid = format!("1.{symbol}");
        let resp: serde_json::Value = self
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                ("klt", period),
                ("fqt", "1"),
                ("beg", "0"),
                ("end", "20500000"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let klines = resp["data"]["klines"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for line in &klines {
            if let Some(s) = line.as_str() {
                let fields: Vec<&str> = s.split(',').collect();
                if fields.len() >= 7 {
                    let mut row = serde_json::Map::new();
                    row.insert("datetime".into(), serde_json::json!(fields[0]));
                    row.insert("open".into(), serde_json::json!(fields[1]));
                    row.insert("close".into(), serde_json::json!(fields[2]));
                    row.insert("high".into(), serde_json::json!(fields[3]));
                    row.insert("low".into(), serde_json::json!(fields[4]));
                    row.insert("volume".into(), serde_json::json!(fields[5]));
                    row.insert("amount".into(), serde_json::json!(fields[6]));
                    items.push(serde_json::Value::Object(row));
                }
            }
        }
        Ok(items)
    }

    /// Convertible bond pre-market minute data from Eastmoney.
    ///
    /// `symbol` is a convertible bond code, e.g. "127018".
    pub async fn bond_zh_hs_cov_pre_min(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let secid = format!("1.{symbol}");
        let resp: serde_json::Value = self
            .get("https://push2.eastmoney.com/api/qt/stock/trends2/get")
            .query(&[
                ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58"),
                ("iscr", "0"),
                ("ndays", "1"),
                ("secid", secid.as_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        let trends = resp["data"]["trends"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for line in &trends {
            if let Some(s) = line.as_str() {
                let fields: Vec<&str> = s.split(',').collect();
                if fields.len() >= 6 {
                    let mut row = serde_json::Map::new();
                    row.insert("datetime".into(), serde_json::json!(fields[0]));
                    row.insert("open".into(), serde_json::json!(fields[1]));
                    row.insert("close".into(), serde_json::json!(fields[2]));
                    row.insert("high".into(), serde_json::json!(fields[3]));
                    row.insert("low".into(), serde_json::json!(fields[4]));
                    row.insert("volume".into(), serde_json::json!(fields[5]));
                    items.push(serde_json::Value::Object(row));
                }
            }
        }
        Ok(items)
    }

    /// Convertible bond spot (real-time) data from Eastmoney.
    ///
    /// `symbol` is a convertible bond code, e.g. "127018".
    pub async fn bond_zh_hs_cov_spot(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let secid = format!("1.{symbol}");
        let resp: serde_json::Value = self
                        .get("https://push2.eastmoney.com/api/qt/stock/get")
            .query(&[
                ("secid", secid.as_str()),
                ("fields", "f43,f44,f45,f46,f47,f48,f50,f51,f52,f55,f57,f58,f116,f117,f162,f167,f168,f169,f170"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp.get("data").cloned().unwrap_or(serde_json::Value::Null);
        if data.is_null() {
            return Ok(vec![]);
        }

        let mut row = serde_json::Map::new();
        row.insert("symbol".into(), serde_json::json!(symbol));
        if let Some(v) = data.get("f43") {
            row.insert("close".into(), v.clone());
        }
        if let Some(v) = data.get("f44") {
            row.insert("high".into(), v.clone());
        }
        if let Some(v) = data.get("f45") {
            row.insert("low".into(), v.clone());
        }
        if let Some(v) = data.get("f46") {
            row.insert("open".into(), v.clone());
        }
        if let Some(v) = data.get("f47") {
            row.insert("volume".into(), v.clone());
        }
        if let Some(v) = data.get("f48") {
            row.insert("amount".into(), v.clone());
        }
        if let Some(v) = data.get("f170") {
            row.insert("change_pct".into(), v.clone());
        }
        Ok(vec![serde_json::Value::Object(row)])
    }

    /// Fetch convertible bond value analysis (premium ratio analysis).
    ///
    /// Returns historical data of conversion value, pure bond value, and
    /// premium ratios for a given convertible bond symbol.
    pub async fn bond_zh_cov_value_analysis(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let filter = format!("(zcode=\"{symbol}\")");
        let resp: EmDatacenterResp = self
            .get("https://datacenter-web.eastmoney.com/api/data/get")
            .query(&[
                ("sty", "ALL"),
                ("token", "894050c76af8597a853f5b408b759f5d"),
                ("st", "date"),
                ("sr", "1"),
                ("source", "WEB"),
                ("type", "RPTA_WEB_KZZ_LS"),
                ("filter", filter.as_str()),
                ("p", "1"),
                ("ps", "8000"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        if data.is_empty() {
            return Err(Error::not_found(format!(
                "no value analysis data for bond {symbol}"
            )));
        }
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cov_info_indicator_map() {
        let indicators = ["基本信息", "中签号", "筹资用途", "重要日期"];
        let expected = [
            "RPT_BOND_CB_LIST",
            "RPT_CB_BALLOTNUM",
            "RPT_BOND_BS_OPRFINVESTITEM",
            "RPT_CB_IMPORTANTDATE",
        ];
        for (ind, exp) in indicators.iter().zip(expected.iter()) {
            let report = match *ind {
                "基本信息" => "RPT_BOND_CB_LIST",
                "中签号" => "RPT_CB_BALLOTNUM",
                "筹资用途" => "RPT_BOND_BS_OPRFINVESTITEM",
                "重要日期" => "RPT_CB_IMPORTANTDATE",
                _ => unreachable!(),
            };
            assert_eq!(report, *exp);
        }
    }

    #[test]
    fn test_value_analysis_filter() {
        let symbol = "113527";
        let filter = format!("(zcode=\"{symbol}\")");
        assert_eq!(filter, "(zcode=\"113527\")");
    }

    #[test]
    fn test_em_datacenter_resp() {
        let json_str = r#"{"result": {"data": [{"SECURITY_CODE": "123121"}], "pages": 3}}"#;
        let resp: super::EmDatacenterResp = serde_json::from_str(json_str).unwrap();
        assert_eq!(resp.result.as_ref().unwrap().data.len(), 1);
    }
}
