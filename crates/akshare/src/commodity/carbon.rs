//! Carbon trading data from various Chinese carbon exchanges.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Beijing carbon emission trading data.
    ///
    /// Fetches data from Beijing Carbon Emission Trading Center.
    pub async fn energy_carbon_bj(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://www.bjets.com.cn/article/jyxx/";
        let body = self.get(url).send().await?.text().await?;

        let mut items = Vec::new();
        // Parse HTML table for carbon trading data
        for line in &crate::util::extract_table_lines(&body) {
            if line.contains("<td") {
                let cells = crate::util::extract_table_cells(line);
                if cells.len() >= 3 {
                    let date = cells[0].clone();
                    let _volume: f64 = cells[1].parse().unwrap_or(0.0);
                    let avg_price: f64 = cells[2].parse().unwrap_or(0.0);
                    if !date.is_empty() {
                        items.push(MacroDataPoint {
                            date,
                            value: avg_price,
                            name: "Beijing Carbon".to_string(),
                        });
                    }
                }
            }
        }
        Ok(items)
    }

    /// Shenzhen carbon emission trading data (domestic).
    ///
    /// Fetches domestic carbon market data from Shenzhen Carbon Exchange.
    pub async fn energy_carbon_sz(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "http://www.cerx.cn/dailynewsCN/index.htm";
        let body = self.get(url).send().await?.text().await?;

        let mut items = Vec::new();
        parse_carx_table(&body, &mut items, "Shenzhen Carbon Domestic");
        Ok(items)
    }

    /// Shenzhen carbon emission trading data (international).
    ///
    /// Fetches international carbon market data from Shenzhen Carbon Exchange.
    pub async fn energy_carbon_eu(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "http://www.cerx.cn/dailynewsOuter/index.htm";
        let body = self.get(url).send().await?.text().await?;

        let mut items = Vec::new();
        parse_carx_table(&body, &mut items, "Shenzhen Carbon International");
        Ok(items)
    }

    /// Hubei carbon emission trading data.
    ///
    /// Fetches data from Hubei Carbon Emission Exchange.
    pub async fn energy_carbon_hb(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://www.hbets.cn/";
        let body = self.get(url).send().await?.text().await?;

        let mut items = Vec::new();
        // Look for JSON data in script tags
        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.contains("cjj") && trimmed.contains("riqi") {
                // Try to extract JSON array
                if let Some(start) = trimmed.find("'[")
                    && let Some(end) = trimmed.rfind("]'")
                {
                    let json_str = &trimmed[start + 1..=end];
                    if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(json_str) {
                        for entry in &arr {
                            let date = entry.str_or(&["riqi"], "");
                            let price = entry.f64_or(&["cjj"], 0.0);
                            if !date.is_empty() {
                                items.push(MacroDataPoint {
                                    date: date.get(..10).unwrap_or(&date).to_string(),
                                    value: price,
                                    name: "Hubei Carbon".to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(items)
    }

    /// Guangzhou carbon emission trading data.
    ///
    /// Fetches data from Guangzhou Carbon Emission Exchange.
    pub async fn energy_carbon_gz(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "http://ets.cnemission.com/carbon/portalIndex/markethistory";
        let body = self
            .get(url)
            .query(&[
                ("Top", "1"),
                ("beginTime", "2010-01-01"),
                ("endTime", "2030-12-31"),
            ])
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        parse_html_table(&body, &mut items, "Guangzhou Carbon");
        Ok(items)
    }

    /// Historical oil price detail by region.
    ///
    /// Returns oil prices for all Chinese regions on a specific date.
    /// `date` is in "YYYYMMDD" format.
    pub async fn energy_oil_detail(&self, date: &str) -> Result<Vec<MacroDataPoint>> {
        let formatted_date = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);

        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: serde_json::Value = self
            .get(url)
            .query(&[
                ("reportName", "RPTA_WEB_YJ_JH"),
                ("columns", "ALL"),
                ("filter", &format!("(dim_date='{formatted_date}')")),
                ("sortColumns", "cityname"),
                ("sortTypes", "1"),
                ("token", "894050c76af8597a853f5b408b759f5d"),
                ("pageNumber", "1"),
                ("pageSize", "1000"),
                ("source", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp
            .nested(&["result", "data"])
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for v in &data {
            let region = v.str_or(&["cityname"], "");
            let price_92 = v.f64_or(&["v_92", "V_92"], 0.0);
            if !region.is_empty() {
                items.push(MacroDataPoint {
                    date: formatted_date.clone(),
                    value: price_92,
                    name: format!("Oil 92# {region}"),
                });
            }
        }
        Ok(items)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_carx_table(body: &str, items: &mut Vec<MacroDataPoint>, name: &str) {
    for line in &crate::util::extract_table_lines(body) {
        if line.contains("<td") {
            let cells = crate::util::extract_table_cells(line);
            if cells.len() >= 5 {
                let date = cells[0].clone();
                let close: f64 = cells[4].parse().unwrap_or(0.0);
                if !date.is_empty() {
                    items.push(MacroDataPoint {
                        date: date.get(..10).unwrap_or(&date).to_string(),
                        value: close,
                        name: name.to_string(),
                    });
                }
            }
        }
    }
}

fn parse_html_table(body: &str, items: &mut Vec<MacroDataPoint>, name: &str) {
    for line in &crate::util::extract_table_lines(body) {
        if line.contains("<td") {
            let cells = crate::util::extract_table_cells(line);
            if cells.len() >= 4 {
                let date = cells[0].clone();
                let close: f64 = cells[3].parse().unwrap_or(0.0);
                if !date.is_empty() {
                    items.push(MacroDataPoint {
                        date: date.get(..10).unwrap_or(&date).to_string(),
                        value: close,
                        name: name.to_string(),
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::extract_table_cells;

    #[test]
    fn test_extract_table_cells() {
        let html = "<td>2024-01-01</td><td>1000</td><td>50.0</td>";
        let cells = extract_table_cells(html);
        assert_eq!(cells.len(), 3);
        assert_eq!(cells[0], "2024-01-01");
    }
}
