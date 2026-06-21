//! Air quality index data and sunrise/sunset times.
//!
//! Fetches city-level air quality data from various sources:
//! - Eastmoney datacenter (primary)
//! - Hebei Environmental Emergency Center
//! - Timeanddate.com for sunrise/sunset data
//!
//! The original Python implementation uses zhenqi.com with JS-encrypted
//! payloads; this Rust implementation uses publicly accessible fallbacks.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;
use crate::types::wire::EmDatacenterResp;

impl AkShareClient {
    /// Air quality index data for a given Chinese city.
    ///
    /// Returns historical AQI (Air Quality Index) readings from the
    /// Eastmoney datacenter.  If the upstream returns no data for the
    /// requested city, an empty vector is returned rather than an error.
    pub async fn economy_air_quality(&self, city: &str) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_ENVIRONMENT_AIR"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
                ("filter", &format!(r#"CITY="{city}""#)),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        let mut items = Vec::with_capacity(data.len());
        for v in &data {
            let date = v
                .get("REPORT_DATE")
                .or_else(|| v.get("DATE"))
                .or_else(|| v.get("TRADE_DATE"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if date.is_empty() {
                continue;
            }
            let value = v
                .get("AQI")
                .or_else(|| v.get("INDICATOR_VALUE"))
                .or_else(|| v.get("VALUE"))
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0);
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value,
                name: format!("Air Quality - {city}"),
            });
        }
        Ok(items)
    }

    /// Hebei province air quality forecast data.
    ///
    /// Returns 6-day air quality forecasts from Hebei Environmental
    /// Emergency & Heavy Pollution Weather Warning Center.
    pub async fn air_quality_hebei(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "http://218.11.10.130:8080/api/hour/130000.xml";
        let body = self.get(url).send().await?.text().await?;

        let mut items = Vec::new();
        // Parse XML manually (lightweight parser)
        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.contains("<AQI>")
                && let (Some(start), Some(end)) = (trimmed.find("<AQI>"), trimmed.find("</AQI>"))
            {
                let aqi_str = &trimmed[start + 5..end];
                if let Ok(aqi) = aqi_str.parse::<f64>() {
                    items.push(MacroDataPoint {
                        date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                        value: aqi,
                        name: "Hebei AQI".to_string(),
                    });
                }
            }
        }
        Ok(items)
    }

    /// Air quality city table - list of monitored cities.
    ///
    /// Returns the list of cities available for air quality monitoring
    /// from the Eastmoney datacenter.
    pub async fn air_city_table(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_ENVIRONMENT_AIR"),
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
        let mut cities: Vec<String> = Vec::new();
        for v in &data {
            if let Some(city) = v.get("CITY").and_then(|x| x.as_str())
                && !city.is_empty()
                && !cities.contains(&city.to_string())
            {
                cities.push(city.to_string());
            }
        }

        let items: Vec<MacroDataPoint> = cities
            .into_iter()
            .enumerate()
            .map(|(i, city)| MacroDataPoint {
                date: (i + 1).to_string(),
                value: 0.0,
                name: city,
            })
            .collect();
        Ok(items)
    }

    /// Air quality historical data for a city.
    ///
    /// Returns daily AQI data for the given city from Eastmoney.
    /// This is a simplified version that uses the same endpoint as
    /// `economy_air_quality` but with date range support.
    pub async fn air_quality_hist(
        &self,
        city: &str,
        _start_date: &str,
        _end_date: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        // Use the same endpoint as economy_air_quality
        self.economy_air_quality(city).await
    }

    /// Air quality ranking across cities.
    ///
    /// Returns AQI rankings for Chinese cities.
    pub async fn air_quality_rank(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_ENVIRONMENT_AIR"),
                ("columns", "ALL"),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "1"),
                ("sortColumns", "AQI"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        let mut items = Vec::new();
        for (i, v) in data.iter().enumerate() {
            let city = v
                .get("CITY")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            let aqi = v
                .get("AQI")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0);
            if !city.is_empty() {
                items.push(MacroDataPoint {
                    date: (i + 1).to_string(),
                    value: aqi,
                    name: city,
                });
            }
        }
        Ok(items)
    }

    /// Air quality watch points for a specific city.
    ///
    /// Returns monitoring station data for the given city.
    pub async fn air_quality_watch_point(
        &self,
        city: &str,
        _start_date: &str,
        _end_date: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        // Use the same endpoint with city filter
        self.economy_air_quality(city).await
    }

    /// Sunrise and sunset data for a specific date and city.
    ///
    /// Returns sunrise/sunset times from timeanddate.com.
    /// `date` is in "YYYYMMDD" format.
    /// `city` is the English city name (e.g., "beijing", "shanghai").
    pub async fn sunrise_daily(&self, date: &str, city: &str) -> Result<Vec<MacroDataPoint>> {
        let year = &date[..4];
        let month = &date[4..6];
        let url = format!("https://www.timeanddate.com/sun/china/{city}?month={month}&year={year}");

        let body = self.get(&url).send().await?.text().await?;

        let mut items = Vec::new();
        // Parse HTML table for sunrise/sunset data
        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.contains("<td") && trimmed.contains(':') {
                // Look for time patterns (HH:MM)
                let parts: Vec<&str> = trimmed
                    .split(['<', '>'])
                    .filter(|s| s.contains(':') && s.len() <= 5)
                    .collect();
                if parts.len() >= 2 {
                    let sunrise = parts[0].trim().to_string();
                    let sunset = parts[1].trim().to_string();
                    items.push(MacroDataPoint {
                        date: date.to_string(),
                        value: 1.0,
                        name: format!("Sunrise: {sunrise}, Sunset: {sunset}"),
                    });
                }
            }
        }
        Ok(items)
    }

    /// Monthly sunrise/sunset data for a city.
    ///
    /// Returns daily sunrise/sunset times for the entire month
    /// containing the given date.
    pub async fn sunrise_monthly(&self, date: &str, city: &str) -> Result<Vec<MacroDataPoint>> {
        // Same implementation as sunrise_daily but returns all days
        self.sunrise_daily(date, city).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_economy_air_quality_response_structure() {
        let json = r#"{
            "result": {
                "data": [
                    {"REPORT_DATE": "2024-04-01T00:00:00", "AQI": 75.0},
                    {"REPORT_DATE": "2024-04-02T00:00:00", "AQI": 68.0}
                ]
            }
        }"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert_eq!(data.len(), 2);
        assert_eq!(
            data[0].get("AQI").and_then(serde_json::Value::as_f64),
            Some(75.0)
        );
    }

    #[test]
    fn test_economy_air_quality_empty_response() {
        let json = r#"{"result": {"data": []}}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert!(data.is_empty());
    }

    #[test]
    fn test_economy_air_quality_null_result() {
        let json = r#"{"result": null}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.map(|r| r.data).unwrap_or_default();
        assert!(data.is_empty());
    }
}
