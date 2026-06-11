//! Baidu migration data: area-level and scale-level migration indices.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct MigrationAreaResponse {
    data: Option<MigrationAreaData>,
}

#[derive(Debug, Deserialize)]
struct MigrationAreaData {
    list: Option<Vec<MigrationAreaItem>>,
}

#[derive(Debug, Deserialize)]
struct MigrationAreaItem {
    city_name: Option<String>,
    province_name: Option<String>,
    value: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct MigrationScaleResponse {
    data: Option<MigrationScaleData>,
}

#[derive(Debug, Deserialize)]
struct MigrationScaleData {
    list: Option<std::collections::HashMap<String, f64>>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Baidu Migration - area-level migration details.
    ///
    /// Returns the top 100 origin/destination areas for the given city/province.
    /// `area` must be a full name (e.g., "重庆市", "杭州市").
    /// `indicator` is "move_in" or "move_out".
    /// `date` is "YYYYMMDD" format.
    pub async fn migration_area_baidu(
        &self,
        area: &str,
        indicator: &str,
        date: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        // Determine if it's a province or city
        let dt_flag = if is_province(area) {
            "province"
        } else {
            "city"
        };
        let area_id = get_area_id(area)
            .ok_or_else(|| Error::invalid_input(format!("unknown area: {}", area)))?;

        let url = "https://huiyan.baidu.com/migration/cityrank.jsonp";
        let body = self
            .get(url)
            .query(&[
                ("dt", dt_flag),
                ("id", &area_id),
                ("type", indicator),
                ("date", date),
            ])
            .send()
            .await?
            .text()
            .await?;

        // Parse JSONP: ({ ... })
        let json_str = extract_jsonp(&body)?;
        let resp: MigrationAreaResponse = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("migration area JSON: {e}")))?;

        let list = resp.data.and_then(|d| d.list).unwrap_or_default();

        let mut items = Vec::new();
        for entry in &list {
            let name = entry
                .city_name
                .as_deref()
                .or(entry.province_name.as_deref())
                .unwrap_or("")
                .to_string();
            let value = entry.value.unwrap_or(0.0);
            if !name.is_empty() {
                items.push(MacroDataPoint {
                    date: date.to_string(),
                    value,
                    name,
                });
            }
        }
        Ok(items)
    }

    /// Baidu Migration - migration scale index time series.
    ///
    /// Returns the historical migration scale index for the given area.
    /// `area` must be a full name (e.g., "广州市").
    /// `indicator` is "move_in" or "move_out".
    pub async fn migration_scale_baidu(
        &self,
        area: &str,
        indicator: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let dt_flag = if is_province(area) {
            "province"
        } else {
            "city"
        };
        let area_id = get_area_id(area)
            .ok_or_else(|| Error::invalid_input(format!("unknown area: {}", area)))?;

        let url = "https://huiyan.baidu.com/migration/historycurve.jsonp";
        let body = self
            .get(url)
            .query(&[("dt", dt_flag), ("id", &area_id), ("type", indicator)])
            .send()
            .await?
            .text()
            .await?;

        let json_str = extract_jsonp(&body)?;
        let resp: MigrationScaleResponse = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("migration scale JSON: {e}")))?;

        let list = resp.data.and_then(|d| d.list).unwrap_or_default();

        let mut items = Vec::new();
        for (date_str, value) in &list {
            items.push(MacroDataPoint {
                date: date_str.clone(),
                value: *value,
                name: format!("Migration Scale - {}", area),
            });
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_jsonp(body: &str) -> Result<&str> {
    let start = body
        .find("({")
        .map(|i| i + 1)
        .or_else(|| body.find('{'))
        .ok_or_else(|| Error::decode("jsonp: opening not found"))?;
    let end = body
        .rfind("})")
        .or_else(|| body.rfind('}'))
        .ok_or_else(|| Error::decode("jsonp: closing not found"))?;
    Ok(&body[start..=end])
}

fn is_province(area: &str) -> bool {
    // Simplified check - if it ends with province suffixes
    area.ends_with("省")
        || area.ends_with("自治区")
        || area == "北京市"
        || area == "天津市"
        || area == "上海市"
        || area == "重庆市"
}

fn get_area_id(area: &str) -> Option<String> {
    // Common city/province to Baidu area ID mapping
    // In practice, this would be a comprehensive lookup table
    let known: &[(&str, &str)] = &[
        ("北京市", "110000"),
        ("天津市", "120000"),
        ("上海市", "310000"),
        ("重庆市", "500000"),
        ("广州市", "440100"),
        ("深圳市", "440300"),
        ("杭州市", "330100"),
        ("南京市", "320100"),
        ("成都市", "510100"),
        ("武汉市", "420100"),
        ("西安市", "610100"),
        ("苏州市", "320500"),
        ("郑州市", "410100"),
        ("长沙市", "430100"),
        ("青岛市", "370200"),
        ("大连市", "210200"),
        ("宁波市", "330200"),
        ("厦门市", "350200"),
        ("济南市", "370100"),
        ("哈尔滨市", "230100"),
        ("沈阳市", "210100"),
        ("长春市", "220100"),
        ("昆明市", "530100"),
        ("贵阳市", "520100"),
        ("南昌市", "360100"),
        ("福州市", "350100"),
        ("合肥市", "340100"),
        ("太原市", "140100"),
        ("石家庄市", "130100"),
        ("兰州市", "620100"),
        ("乌鲁木齐市", "650100"),
        ("呼和浩特市", "150100"),
        ("南宁市", "450100"),
        ("海口市", "460100"),
        ("银川市", "640100"),
        ("西宁市", "630100"),
        ("拉萨市", "540100"),
        ("广东省", "440000"),
        ("浙江省", "330000"),
        ("江苏省", "320000"),
        ("山东省", "370000"),
        ("河南省", "410000"),
        ("四川省", "510000"),
        ("湖北省", "420000"),
        ("湖南省", "430000"),
        ("福建省", "350000"),
        ("安徽省", "340000"),
        ("河北省", "130000"),
        ("辽宁省", "210000"),
        ("陕西省", "610000"),
        ("江西省", "360000"),
        ("黑龙江省", "230000"),
        ("吉林省", "220000"),
        ("云南省", "530000"),
        ("贵州省", "520000"),
        ("山西省", "140000"),
        ("甘肃省", "620000"),
        ("海南省", "460000"),
        ("青海省", "630000"),
        ("内蒙古自治区", "150000"),
        ("广西壮族自治区", "450000"),
        ("西藏自治区", "540000"),
        ("宁夏回族自治区", "640000"),
        ("新疆维吾尔自治区", "650000"),
    ];
    known
        .iter()
        .find(|(name, _)| *name == area)
        .map(|(_, id)| id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_jsonp() {
        let body = r#"jQuery123({"data":{"list":[]}});"#;
        let json = extract_jsonp(body).unwrap();
        assert_eq!(json, r#"{"data":{"list":[]}}"#);
    }

    #[test]
    fn test_is_province() {
        assert!(is_province("广东省"));
        assert!(is_province("北京市"));
        assert!(!is_province("广州市"));
    }

    #[test]
    fn test_get_area_id() {
        assert_eq!(get_area_id("广州市"), Some("440100".to_string()));
        assert_eq!(get_area_id("广东省"), Some("440000".to_string()));
        assert_eq!(get_area_id("Unknown"), None);
    }
}
