//! Box office data from China.
//!
//! Fetches daily/weekly box office figures from the Eastmoney datacenter.
//! The original Python implementation uses endata.com.cn with JS-based
//! decryption; this Rust implementation uses the Eastmoney datacenter as a
//! publicly accessible fallback.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;
use crate::types::wire::EmDatacenterResp;

impl AkShareClient {
    /// Movie box office — cinema daily data (影院票房日榜).
    ///
    /// `date`: format YYYYMMDD
    pub async fn movie_boxoffice_cinema_daily(&self, date: &str) -> Result<Vec<MacroDataPoint>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_MOVIE_CINEMA_DAILY"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "BOX_OFFICE"),
                ("source", "WEB"),
                ("client", "WEB"),
                ("filter", format!("(TRADE_DATE='{date_fmt}')").as_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        Ok(parse_movie_response(resp, "Cinema Daily"))
    }

    /// Movie box office — cinema weekly data (影院票房周榜).
    ///
    /// `date`: format YYYYMMDD
    pub async fn movie_boxoffice_cinema_weekly(&self, date: &str) -> Result<Vec<MacroDataPoint>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_MOVIE_CINEMA_WEEKLY"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "BOX_OFFICE"),
                ("source", "WEB"),
                ("client", "WEB"),
                ("filter", format!("(TRADE_DATE='{date_fmt}')").as_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        Ok(parse_movie_response(resp, "Cinema Weekly"))
    }

    /// Movie box office — daily data (日票房榜).
    ///
    /// `date`: format YYYYMMDD
    pub async fn movie_boxoffice_daily(&self, date: &str) -> Result<Vec<MacroDataPoint>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_MOVIE_DAILY"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "BOX_OFFICE"),
                ("source", "WEB"),
                ("client", "WEB"),
                ("filter", format!("(TRADE_DATE='{date_fmt}')").as_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        Ok(parse_movie_response(resp, "Daily"))
    }

    /// Movie box office — monthly data (月票房榜).
    ///
    /// `date`: format YYYYMM
    pub async fn movie_boxoffice_monthly(&self, date: &str) -> Result<Vec<MacroDataPoint>> {
        let year = &date[..4];
        let month = &date[4..];
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_MOVIE_MONTHLY"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "BOX_OFFICE"),
                ("source", "WEB"),
                ("client", "WEB"),
                (
                    "filter",
                    format!("(YEAR='{year}')(MONTH='{month}')").as_str(),
                ),
            ])
            .send()
            .await?
            .json()
            .await?;

        Ok(parse_movie_response(resp, "Monthly"))
    }

    /// Movie box office — realtime data (实时票房).
    pub async fn movie_boxoffice_realtime(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_MOVIE_REALTIME"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "BOX_OFFICE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        Ok(parse_movie_response(resp, "Realtime"))
    }

    /// Movie box office — weekly data (周票房榜).
    ///
    /// `date`: format YYYYMMDD
    pub async fn movie_boxoffice_weekly(&self, date: &str) -> Result<Vec<MacroDataPoint>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_MOVIE_WEEKLY"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "BOX_OFFICE"),
                ("source", "WEB"),
                ("client", "WEB"),
                ("filter", format!("(TRADE_DATE='{date_fmt}')").as_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        Ok(parse_movie_response(resp, "Weekly"))
    }

    /// Movie box office — yearly data (年票房榜).
    ///
    /// `year`: format YYYY
    pub async fn movie_boxoffice_yearly(&self, year: &str) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_MOVIE_YEARLY"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "BOX_OFFICE"),
                ("source", "WEB"),
                ("client", "WEB"),
                ("filter", format!("(YEAR='{year}')").as_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        Ok(parse_movie_response(resp, "Yearly"))
    }

    /// Movie box office — yearly first week data (首周票房).
    ///
    /// `year`: format YYYY
    pub async fn movie_boxoffice_yearly_first_week(
        &self,
        year: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_MOVIE_YEARLY_FIRST_WEEK"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "BOX_OFFICE"),
                ("source", "WEB"),
                ("client", "WEB"),
                ("filter", format!("(YEAR='{year}')").as_str()),
            ])
            .send()
            .await?
            .json()
            .await?;

        Ok(parse_movie_response(resp, "Yearly First Week"))
    }

    /// China box office data.
    ///
    /// Returns daily box office revenue figures (unit: 10,000 RMB)
    /// from the Eastmoney datacenter.
    pub async fn economy_box_office(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_MOVIE_BOXOFFICE"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        let mut items = Vec::with_capacity(data.len());
        for v in &data {
            let date = v.str_or(&["REPORT_DATE", "DATE", "TRADE_DATE"], "");
            if date.is_empty() {
                continue;
            }
            let value = v.f64_or(
                &["BOX_OFFICE", "INDICATOR_VALUE", "VALUE", "TOTAL_BOX_OFFICE"],
                0.0,
            );
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value,
                name: "Box Office".to_string(),
            });
        }
        Ok(items)
    }
}

/// Helper to parse movie box office response from Eastmoney datacenter.
fn parse_movie_response(resp: EmDatacenterResp, label: &str) -> Vec<MacroDataPoint> {
    let data = resp.result.map(|r| r.data).unwrap_or_default();
    let mut items = Vec::with_capacity(data.len());
    for v in &data {
        let date = v.str_or(&["TRADE_DATE", "REPORT_DATE", "DATE"], "");
        let value = v.f64_or(&["BOX_OFFICE", "INDICATOR_VALUE", "VALUE"], 0.0);
        let name = v.str_or(&["MOVIE_NAME", "NAME"], label);
        items.push(MacroDataPoint {
            date: date.get(..10).unwrap_or(&date).to_string(),
            value,
            name,
        });
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_economy_box_office_response_structure() {
        let json = r#"{
            "result": {
                "data": [
                    {"REPORT_DATE": "2024-04-01T00:00:00", "BOX_OFFICE": 12580.5},
                    {"REPORT_DATE": "2024-03-31T00:00:00", "BOX_OFFICE": 9876.3}
                ]
            }
        }"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert_eq!(data.len(), 2);
        assert_eq!(
            data[0]
                .get("BOX_OFFICE")
                .and_then(serde_json::Value::as_f64),
            Some(12580.5)
        );
    }

    #[test]
    fn test_economy_box_office_empty_response() {
        let json = r#"{"result": {"data": []}}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert!(data.is_empty());
    }

    #[test]
    fn test_economy_box_office_null_result() {
        let json = r#"{"result": null}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.map(|r| r.data).unwrap_or_default();
        assert!(data.is_empty());
    }
}
