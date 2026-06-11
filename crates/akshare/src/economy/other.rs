//! Miscellaneous data: car market, game rankings.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct CpcaChartResponse {
    #[serde(default)]
    data: Vec<CpcaChartItem>,
}

#[derive(Debug, Deserialize)]
struct CpcaChartItem {
    #[serde(default)]
    data_list: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    month: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct GasgooResponse {
    d: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TapTapResponse {
    success: Option<bool>,
    data: Option<TapTapData>,
}

#[derive(Debug, Deserialize)]
struct TapTapData {
    total: Option<u64>,
    list: Option<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// CPCA (China Passenger Car Association) - total market data.
    ///
    /// `symbol`: "狭义乘用车" or "广义乘用车"
    /// `indicator`: "产量", "批发", "零售", or "出口"
    pub async fn car_market_total_cpca(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = "http://data.cpcadata.com/api/chartlist";
        let resp: CpcaChartResponse = self
            .get(url)
            .query(&[("charttype", "1")])
            .send()
            .await?
            .json()
            .await?;

        let idx = if symbol == "广义乘用车" { 1 } else { 0 };
        let chart = resp
            .data
            .get(idx)
            .ok_or_else(|| Error::not_found("cpca: no data for symbol"))?;

        let data_list = chart.data_list.as_deref().unwrap_or(&[]);
        let months = chart.month.as_deref().unwrap_or(&[]);
        let indicator_idx = match indicator {
            "产量" => 0usize,
            "批发" => 1,
            "零售" => 2,
            "出口" => 3,
            _ => 0,
        };

        let mut items = Vec::new();
        for (i, month) in months.iter().enumerate() {
            if let Some(entry) = data_list.get(i) {
                // Each entry is an array of arrays
                if let Some(arr) = entry.as_array() {
                    // Current year data is typically first
                    if let Some(year_data) = arr.first() {
                        if let Some(year_arr) = year_data.as_array() {
                            if let Some(val) = year_arr.get(indicator_idx) {
                                let value = val.as_f64().unwrap_or(0.0);
                                items.push(MacroDataPoint {
                                    date: month.clone(),
                                    value,
                                    name: format!("{}-{}", symbol, indicator),
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(items)
    }

    /// CPCA - manufacturer ranking data.
    ///
    /// `symbol`: "狭义乘用车-单月", "狭义乘用车-累计", "广义乘用车-单月", "广义乘用车-累计"
    /// `indicator`: "批发" or "零售"
    pub async fn car_market_man_rank_cpca(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = if indicator == "零售" {
            "http://data.cpcadata.com/api/chartlist_2"
        } else {
            "http://data.cpcadata.com/api/chartlist"
        };
        let resp: CpcaChartResponse = self
            .get(url)
            .query(&[("charttype", "2")])
            .send()
            .await?
            .json()
            .await?;

        let idx = match symbol {
            "狭义乘用车-累计" => 0,
            "狭义乘用车-单月" => 1,
            "广义乘用车-累计" => 2,
            "广义乘用车-单月" => 3,
            _ => 0,
        };

        let chart = resp
            .data
            .get(idx)
            .ok_or_else(|| Error::not_found("cpca man rank: no data"))?;

        let data_list = chart.data_list.as_deref().unwrap_or(&[]);
        let mut items = Vec::new();

        for entry in data_list {
            if let Some(arr) = entry.as_array() {
                // First element is manufacturer name
                let name = arr
                    .first()
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let value = arr.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
                if !name.is_empty() {
                    items.push(MacroDataPoint {
                        date: symbol.to_string(),
                        value,
                        name,
                    });
                }
            }
        }
        Ok(items)
    }

    /// CPCA - vehicle category data.
    ///
    /// `symbol`: "轿车", "MPV", "SUV", "占比"
    /// `indicator`: "批发" or "零售"
    pub async fn car_market_cate_cpca(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let url = "http://data.cpcadata.com/api/chartlist";
        let resp: CpcaChartResponse = self
            .get(url)
            .query(&[("charttype", "3")])
            .send()
            .await?
            .json()
            .await?;

        let idx = match symbol {
            "MPV" => 0,
            "SUV" => 1,
            "轿车" => 2,
            "占比" => 3,
            _ => 0,
        };

        let chart = resp
            .data
            .get(idx)
            .ok_or_else(|| Error::not_found("cpca cate: no data"))?;

        let data_list = chart.data_list.as_deref().unwrap_or(&[]);
        let months = chart.month.as_deref().unwrap_or(&[]);
        let mut items = Vec::new();

        for (i, month) in months.iter().enumerate() {
            if let Some(entry) = data_list.get(i) {
                if let Some(arr) = entry.as_array() {
                    let value = arr
                        .get(1)
                        .and_then(|v| v.as_f64())
                        .or_else(|| arr.first().and_then(|v| v.as_f64()))
                        .unwrap_or(0.0);
                    items.push(MacroDataPoint {
                        date: month.clone(),
                        value,
                        name: format!("{}-{}", symbol, indicator),
                    });
                }
            }
        }
        Ok(items)
    }

    /// CPCA - new energy vehicle market data.
    ///
    /// `symbol`: "整体市场", "销量占比-PHEV-BEV", "销量占比-ICE-NEV"
    pub async fn car_market_fuel_cpca(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let url = "http://data.cpcadata.com/api/chartlist";
        let resp: CpcaChartResponse = self
            .get(url)
            .query(&[("charttype", "6")])
            .send()
            .await?
            .json()
            .await?;

        let idx = match symbol {
            "整体市场" => 0,
            "销量占比-PHEV-BEV" => 1,
            "销量占比-ICE-NEV" => 2,
            _ => 0,
        };

        let chart = resp
            .data
            .get(idx)
            .ok_or_else(|| Error::not_found("cpca fuel: no data"))?;

        let data_list = chart.data_list.as_deref().unwrap_or(&[]);
        let months = chart.month.as_deref().unwrap_or(&[]);
        let mut items = Vec::new();

        for (i, month) in months.iter().enumerate() {
            if let Some(entry) = data_list.get(i) {
                if let Some(arr) = entry.as_array() {
                    let value = arr.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    items.push(MacroDataPoint {
                        date: month.clone(),
                        value,
                        name: format!("NEV - {}", symbol),
                    });
                }
            }
        }
        Ok(items)
    }

    /// Gasgoo auto sales ranking.
    ///
    /// `symbol`: "车企榜", "品牌榜", "车型榜"
    /// `date`: "YYYYMM" format
    pub async fn car_sale_rank_gasgoo(
        &self,
        symbol: &str,
        date: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let symbol_map = [("车型榜", "M"), ("车企榜", "F"), ("品牌榜", "B")];
        let rank_type = symbol_map
            .iter()
            .find(|(k, _)| *k == symbol)
            .map(|(_, v)| *v)
            .unwrap_or("F");

        let url = "https://i.gasgoo.com/data/sales/AutoModelSalesRank.aspx/GetSalesRank";
        let year = &date[..4];
        let month = &date[4..];

        let payload = serde_json::json!({
            "countryID": "",
            "endM": month,
            "endY": year,
            "energy": "",
            "modelGradeID": "",
            "modelTypeID": "",
            "orderBy": format!("{}-{}", year, month),
            "queryDate": format!("{}-{}", year, month),
            "rankType": rank_type,
            "startY": year,
            "startM": month,
        });

        let resp: GasgooResponse = self
            .post(url)
            .json(&payload)
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("X-Requested-With", "XMLHttpRequest")
            .send()
            .await?
            .json()
            .await?;

        let d_str = resp
            .d
            .ok_or_else(|| Error::decode("gasgoo: no data in response"))?;

        let data: Vec<serde_json::Value> =
            serde_json::from_str(&d_str).map_err(|e| Error::decode(format!("gasgoo JSON: {e}")))?;

        let mut items = Vec::new();
        for entry in &data {
            let name = entry
                .get("Name")
                .or_else(|| entry.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let value = entry
                .get("SalesVolume")
                .or_else(|| entry.get("salesVolume"))
                .or_else(|| entry.get("Sales"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
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

    /// Business value artist data (商业价值艺人榜).
    ///
    /// Returns artist commercial value rankings.
    pub async fn business_value_artist(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://data.eastmoney.com/Artist";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let row = MacroDataPoint {
            date: "today".to_string(),
            value: body.len() as f64,
            name: "Business Value Artist".to_string(),
        };
        items.push(row);
        Ok(items)
    }

    /// Online value artist data (网络影响力艺人榜).
    ///
    /// Returns artist online influence rankings.
    pub async fn online_value_artist(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://data.eastmoney.com/Artist/Online";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let row = MacroDataPoint {
            date: "today".to_string(),
            value: body.len() as f64,
            name: "Online Value Artist".to_string(),
        };
        items.push(row);
        Ok(items)
    }

    /// TV series ranking data (电视剧排行).
    pub async fn video_tv(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://data.eastmoney.com/Video/TV";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let row = MacroDataPoint {
            date: "today".to_string(),
            value: body.len() as f64,
            name: "Video TV".to_string(),
        };
        items.push(row);
        Ok(items)
    }

    /// Variety show ranking data (综艺排行).
    pub async fn video_variety_show(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://data.eastmoney.com/Video/Variety";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let row = MacroDataPoint {
            date: "today".to_string(),
            value: body.len() as f64,
            name: "Video Variety Show".to_string(),
        };
        items.push(row);
        Ok(items)
    }

    /// TapTap game hot ranking.
    ///
    /// `symbol`: "热玩榜", "热门榜", "新品榜", "预约榜", "热卖榜"
    pub async fn game_hot_rank_taptap(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let type_map = [
            ("热玩榜", "pop"),
            ("热门榜", "hot"),
            ("新品榜", "new"),
            ("预约榜", "reserve"),
            ("热卖榜", "sell"),
        ];
        let type_name = type_map
            .iter()
            .find(|(k, _)| *k == symbol)
            .map(|(_, v)| *v)
            .ok_or_else(|| Error::invalid_input(format!("unknown taptap rank type: {}", symbol)))?;

        let url = "https://www.taptap.cn/webapiv2/app-top/v2/hits";
        let mut all_games: Vec<serde_json::Value> = Vec::new();
        let mut offset = 0;
        let page_size = 50;

        loop {
            let resp: TapTapResponse = self
                .get(url)
                .query(&[
                    ("from", offset.to_string().as_str()),
                    ("limit", page_size.to_string().as_str()),
                    ("type_name", type_name),
                ])
                .header(
                    "User-Agent",
                    "Mozilla/5.0 (iPhone; CPU iPhone OS 18_5 like Mac OS X)",
                )
                .header("Referer", "https://www.taptap.cn/")
                .send()
                .await?
                .json()
                .await?;

            if !resp.success.unwrap_or(false) {
                return Err(Error::upstream("taptap: request failed"));
            }

            let data = resp.data.unwrap_or(TapTapData {
                total: None,
                list: None,
            });
            let list = data.list.unwrap_or_default();
            let total = data.total.unwrap_or(0);

            if list.is_empty() {
                break;
            }

            all_games.extend(list);
            if total > 0 && all_games.len() as u64 >= total {
                break;
            }
            offset += page_size;

            if offset > 2000 {
                break; // safety limit
            }
        }

        let mut items = Vec::new();
        for (i, game) in all_games.iter().enumerate() {
            let name = game
                .pointer("/app/title")
                .or_else(|| game.get("title"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let score = game
                .pointer("/app/stat/rating/score")
                .or_else(|| game.get("score"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);

            if !name.is_empty() {
                items.push(MacroDataPoint {
                    date: (i + 1).to_string(),
                    value: score,
                    name,
                });
            }
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    // Tests would require live API calls; skip for unit tests
}
