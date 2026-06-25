//! Bond spot rate / yield curve data from Eastmoney.
//!
//! Fetches China and US treasury yield curve data using the
//! `RPTA_WEB_TREASURYYIELD` report from the Eastmoney datacenter.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::BondSnapshot;
use crate::types::value_ext::ValueExt;

impl AkShareClient {
    /// Bond spot deal data (现券成交数据).
    ///
    /// Returns bond spot trading data from the ChinaMoney interbank market.
    pub async fn bond_spot_deal(&self) -> Result<Vec<crate::types::Row>> {
        let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bk-currency/SptDl?lang=CN&pageNo=1&pageSize=200";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let records = resp["records"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        let empty_map = serde_json::Map::new();
        for rec in &records {
            let mut row = crate::types::Row::new();
            for (k, v) in rec.as_object().unwrap_or(&empty_map) {
                row.insert(k.clone(), v.clone());
            }
            if !row.is_empty() {
                items.push(row);
            }
        }
        Ok(items)
    }

    /// Bond spot quote data (现券报价数据).
    ///
    /// Returns bond spot quotation data from the ChinaMoney interbank market.
    pub async fn bond_spot_quote(&self) -> Result<Vec<crate::types::Row>> {
        let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bk-currency/SptQut?lang=CN&pageNo=1&pageSize=200";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let records = resp["records"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        let empty_map = serde_json::Map::new();
        for rec in &records {
            let mut row = crate::types::Row::new();
            for (k, v) in rec.as_object().unwrap_or(&empty_map) {
                row.insert(k.clone(), v.clone());
            }
            if !row.is_empty() {
                items.push(row);
            }
        }
        Ok(items)
    }

    /// Fetch bond spot rates (yield curve data).
    ///
    /// Returns China treasury bond yields for multiple tenors (2Y, 5Y, 10Y,
    /// 30Y) from the Eastmoney datacenter. Each tenor is returned as a
    /// separate `BondSnapshot` with the `yield_rate` field populated.
    ///
    /// Up to `limit` date-rows are fetched; each row may expand into
    /// multiple tenor snapshots.
    pub async fn bond_spot_rates(&self, limit: usize) -> Result<Vec<BondSnapshot>> {
        let page_size = limit.clamp(1, 500).to_string();

        let response = crate::util::send_and_check(
            self.get("https://datacenter.eastmoney.com/api/data/get")
                .query(&[
                ("type", "RPTA_WEB_TREASURYYIELD"),
                ("sty", "ALL"),
                ("st", "SOLAR_DATE"),
                ("sr", "-1"),
                ("token", "894050c76af8597a853f5b408b759f5d"),
                ("p", "1"),
                ("ps", page_size.as_str()),
                ("pageNo", "1"),
                ("pageNum", "1"),
                ])
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .nested(&["result", "data"])
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        if data.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no bond yield curve data",
            ));
        }

        // Yield tenor columns from the Eastmoney API
        let tenors: [(&str, &str); 4] = [
            ("EMM00588704", "2Y"),
            ("EMM00166462", "5Y"),
            ("EMM00166466", "10Y"),
            ("EMM00166469", "30Y"),
        ];

        let mut items = Vec::new();
        for v in &data {
            let date = v.str_or(&["SOLAR_DATE"], "");
            if date.is_empty() {
                continue;
            }
            let date_short = date.get(..10).unwrap_or(&date).to_string();

            for (field, tenor_label) in &tenors {
                let yield_rate = v.f64_field(&[*field]);
                if let Some(rate) = yield_rate {
                    items.push(BondSnapshot {
                        symbol: format!("CNBD{tenor_label}"),
                        name: format!("China Bond {tenor_label}"),
                        date: date_short.clone(),
                        close: 0.0,
                        change_pct: 0.0,
                        yield_rate: Some(rate),
                        credit_rating: None,
                    });
                }
            }
        }

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no bond spot rate items",
            ));
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::value_ext::ValueExt;

    #[test]
    fn test_yield_extraction() {
        // Simulate an Eastmoney response row
        let row = json!({
            "SOLAR_DATE": "2025-01-15 00:00:00",
            "EMM00588704": 1.85,
            "EMM00166462": 2.10,
            "EMM00166466": 2.45,
            "EMM00166469": 2.80
        });

        let date = row.str_or(&["SOLAR_DATE"], "");
        assert_eq!(date.get(..10).unwrap_or(&date), "2025-01-15");

        let tenors = [
            ("EMM00588704", "2Y"),
            ("EMM00166462", "5Y"),
            ("EMM00166466", "10Y"),
            ("EMM00166469", "30Y"),
        ];

        for (field, label) in &tenors {
            let rate = row.f64_field(&[*field]);
            assert!(rate.is_some(), "missing yield for {label}");
            assert!(rate.unwrap() > 0.0);
        }
    }

    #[test]
    fn test_missing_date_skipped() {
        let row = json!({
            "SOLAR_DATE": "",
            "EMM00588704": 1.85
        });
        let date = row.str_or(&["SOLAR_DATE"], "");
        assert!(date.is_empty());
    }
}
