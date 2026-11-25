//! Futures delivery (交割) and futures-to-spot (期转现) data.
//!
//! Covers SHFE, DCE, and CZCE delivery statistics.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;

impl AkShareClient {
    /// SHFE futures-to-spot (期转现) data.
    pub async fn futures_to_spot_shfe(&self, date: &str) -> Result<Vec<Row>> {
        let url = format!("https://tsite.shfe.com.cn/data/instrument/ExchangeDelivery{date}.dat");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["ExchangeDelivery"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let Some(arr) = row.as_array() else {
                continue;
            };
            if arr.len() < 6 {
                continue;
            }
            let mut r = Row::new();
            r.insert("date".into(), arr.get(1).cloned().unwrap_or_default());
            r.insert("contract".into(), arr.get(5).cloned().unwrap_or_default());
            r.insert(
                "delivery_volume".into(),
                arr.get(2).cloned().unwrap_or_default(),
            );
            r.insert(
                "to_spot_volume".into(),
                arr.get(4).cloned().unwrap_or_default(),
            );
            items.push(r);
        }
        Ok(items)
    }

    /// SHFE delivery statistics by month.
    pub async fn futures_delivery_shfe(&self, date: &str) -> Result<Vec<Row>> {
        let url =
            format!("https://tsite.shfe.com.cn/data/dailydata/{date}monthvarietystatistics.dat");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["o_curdelivery"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let arr = row.as_array().cloned().unwrap_or_default();
            if arr.len() < 7 {
                continue;
            }
            let mut r = Row::new();
            r.insert("variety_name".into(), arr[0].clone());
            r.insert("variety_code".into(), arr[1].clone());
            r.insert("delivery_volume_month".into(), arr[3].clone());
            r.insert("delivery_volume_ratio".into(), arr[4].clone());
            r.insert("delivery_volume_ytd".into(), arr[5].clone());
            r.insert("delivery_volume_yoy".into(), arr[6].clone());
            items.push(r);
        }
        Ok(items)
    }

    /// DCE delivery statistics.
    pub async fn futures_delivery_dce(&self, _date: &str) -> Result<Vec<Row>> {
        Err(Error::unavailable(
            "DCE delivery endpoint requires HTML table parsing",
        ))
    }

    /// DCE futures-to-spot (期转现) data.
    pub async fn futures_to_spot_dce(&self, _date: &str) -> Result<Vec<Row>> {
        Err(Error::unavailable(
            "DCE futures-to-spot endpoint requires HTML table parsing",
        ))
    }

    /// CZCE delivery matching data.
    pub async fn futures_delivery_match_czce(&self, date: &str) -> Result<Vec<Row>> {
        let year = &date[..4];
        let ext = if date > "20251101" { "xlsx" } else { "xls" };
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{year}/{date}/FutureDataDelsettle.{ext}"
        );
        let bytes = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        let rows = crate::util::read_spreadsheet_rows(&bytes)?;
        Ok(rows
            .into_iter()
            .skip(1)
            .filter(|row| !row.iter().all(|cell| cell.trim().is_empty()))
            .map(|row| {
                let mut out = Row::new();
                for (index, value) in row.into_iter().enumerate() {
                    out.insert(format!("column_{index}"), serde_json::json!(value));
                }
                out.insert("date".into(), serde_json::json!(date));
                out
            })
            .collect())
    }

    /// CZCE monthly delivery data.
    pub async fn futures_delivery_czce(&self, date: &str) -> Result<Vec<Row>> {
        let year = &date[..4];
        let ext = if date > "20251101" { "xlsx" } else { "xls" };
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{year}/{date}/FutureDataSettlematched.{ext}"
        );
        let bytes = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        let rows = crate::util::read_spreadsheet_rows(&bytes)?;
        Ok(rows
            .into_iter()
            .skip(1)
            .filter(|row| !row.iter().all(|cell| cell.trim().is_empty()))
            .map(|row| {
                let mut out = Row::new();
                for (index, value) in row.into_iter().enumerate() {
                    out.insert(format!("column_{index}"), serde_json::json!(value));
                }
                out.insert("date".into(), serde_json::json!(date));
                out
            })
            .collect())
    }

    /// DCE delivery match data for a given date.
    ///
    /// Returns delivery matching (交割配对) data from DCE.
    pub async fn futures_delivery_match_dce(&self, _date: &str) -> Result<Vec<Row>> {
        Err(Error::unavailable(
            "DCE delivery matching endpoint requires HTML table parsing",
        ))
    }

    /// CZCE futures-to-spot statistics.
    pub async fn futures_to_spot_czce(&self, date: &str) -> Result<Vec<Row>> {
        let year = &date[..4];
        let ext = if date > "20251101" { "xlsx" } else { "xls" };
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{year}/{date}/FutureDataTrdtrades.{ext}"
        );
        let bytes = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        let rows = crate::util::read_spreadsheet_rows(&bytes)?;
        Ok(rows
            .into_iter()
            .skip(1)
            .filter(|row| !row.iter().all(|cell| cell.trim().is_empty()))
            .map(|row| {
                let mut out = Row::new();
                for (index, value) in row.into_iter().enumerate() {
                    out.insert(format!("column_{index}"), serde_json::json!(value));
                }
                out.insert("date".into(), serde_json::json!(date));
                out
            })
            .collect())
    }
}
