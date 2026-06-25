//! Shenwan (申万宏源) research index data — history, realtime, analysis, components.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{SwAnalysisPoint, SwResearchComponent, SwResearchHistPoint, SwResearchRealtime};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct SwTrendEnvelope {
    data: Option<Vec<SwTrendItem>>,
}

#[derive(Debug, Deserialize)]
struct SwTrendItem {
    #[serde(default)]
    swindexcode: String,
    #[serde(default)]
    bargaindate: String,
    #[serde(default)]
    openindex: Option<f64>,
    #[serde(default)]
    maxindex: Option<f64>,
    #[serde(default)]
    minindex: Option<f64>,
    #[serde(default)]
    closeindex: Option<f64>,
    #[serde(default)]
    bargainamount: Option<f64>,
    #[serde(default)]
    bargainsum: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct SwComponentEnvelope {
    data: Option<SwComponentData>,
}

#[derive(Debug, Deserialize)]
struct SwComponentData {
    results: Option<Vec<SwComponentItem>>,
}

#[derive(Debug, Deserialize)]
struct SwComponentItem {
    #[serde(default)]
    stockcode: String,
    #[serde(default)]
    stockname: String,
    #[serde(default)]
    newweight: Option<f64>,
    #[serde(default)]
    beginningdate: String,
}

#[derive(Debug, Deserialize)]
struct SwCurrentEnvelope {
    data: Option<SwCurrentData>,
}

#[derive(Debug, Deserialize)]
struct SwCurrentData {
    results: Option<Vec<Vec<serde_json::Value>>>,
}

#[derive(Debug, Deserialize)]
struct SwAnalysisEnvelope {
    data: Option<SwAnalysisData>,
}

#[derive(Debug, Deserialize)]
struct SwAnalysisData {
    results: Option<Vec<SwAnalysisItem>>,
}

#[derive(Debug, Deserialize)]
struct SwAnalysisItem {
    #[serde(default)]
    swindexcode: String,
    #[serde(default)]
    swindexname: String,
    #[serde(default)]
    bargaindate: String,
    #[serde(default)]
    closeindex: Option<f64>,
    #[serde(default)]
    bargainamount: Option<f64>,
    #[serde(default)]
    markup: Option<f64>,
    #[serde(default)]
    turnoverrate: Option<f64>,
    #[serde(default)]
    pe: Option<f64>,
    #[serde(default)]
    pb: Option<f64>,
    #[serde(default)]
    meanprice: Option<f64>,
    #[serde(default)]
    bargainsumrate: Option<f64>,
    #[serde(default)]
    negotiablessharesum1: Option<f64>,
    #[serde(default)]
    negotiablessharesum2: Option<f64>,
    #[serde(default)]
    dp: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct SwPageListEnvelope {
    data: Option<SwPageListData>,
}

#[derive(Debug, Deserialize)]
struct SwPageListData {
    list: Option<Vec<SwPageListItem>>,
}

#[derive(Debug, Deserialize)]
struct SwPageListItem {
    #[serde(default, rename = "swIndexCode")]
    sw_index_code: String,
    #[serde(default, rename = "swIndexName")]
    sw_index_name: String,
    #[serde(default, rename = "lastCloseIndex")]
    last_close_index: Option<f64>,
    #[serde(default, rename = "lastMarkup")]
    last_markup: Option<f64>,
    #[serde(default, rename = "yearMarkup")]
    year_markup: Option<f64>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 申万宏源研究 — 指数历史数据.
    ///
    /// `period` is "day", "week", or "month".
    pub async fn index_hist_sw(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<SwResearchHistPoint>> {
        let period_upper = match period {
            "day" => "DAY",
            "week" => "WEEK",
            "month" => "MONTH",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported period: {period}"
                )));
            }
        };

        let response = crate::util::send_and_check(
            self.get("https://www.swsresearch.com/institute-sw/api/index_publish/trend/")
                .query(&[("swindexcode", symbol), ("period", period_upper)]),
        )
        .await?;

        let payload: SwTrendEnvelope = response.json().await.map_err(Error::from)?;
        let items = payload.data.unwrap_or_default();

        let points: Vec<SwResearchHistPoint> = items
            .into_iter()
            .map(|item| SwResearchHistPoint {
                code: item.swindexcode,
                date: item.bargaindate,
                open: item.openindex.unwrap_or(0.0),
                close: item.closeindex.unwrap_or(0.0),
                high: item.maxindex.unwrap_or(0.0),
                low: item.minindex.unwrap_or(0.0),
                volume: item.bargainamount.unwrap_or(0.0),
                amount: item.bargainsum.unwrap_or(0.0),
            })
            .collect();

        if points.is_empty() {
            return Err(Error::not_found("swsresearch returned no history data"));
        }
        Ok(points)
    }

    /// 申万宏源研究 — 指数分时数据.
    pub async fn index_min_sw(&self, symbol: &str) -> Result<Vec<SwMinPoint>> {
        #[derive(Deserialize)]
        struct MinEnvelope {
            data: Option<Vec<MinItem>>,
        }
        #[derive(Deserialize)]
        struct MinItem {
            #[serde(default)]
            l1: String,
            #[serde(default)]
            l2: String,
            #[serde(default)]
            l8: Option<f64>,
            #[serde(default)]
            trading_date: String,
            #[serde(default)]
            trading_time: String,
        }

        let response = crate::util::send_and_check(
            self.get(
                "https://www.swsresearch.com/institute-sw/api/index_publish/details/timelines/",
            )
            .query(&[("swindexcode", symbol)]),
        )
        .await?;

        let payload: MinEnvelope = response.json().await.map_err(Error::from)?;
        let items = payload.data.unwrap_or_default();

        let points: Vec<SwMinPoint> = items
            .into_iter()
            .map(|item| SwMinPoint {
                code: item.l1,
                name: item.l2,
                price: item.l8.unwrap_or(0.0),
                date: item.trading_date,
                time: item.trading_time,
            })
            .collect();

        if points.is_empty() {
            return Err(Error::not_found("swsresearch returned no intraday data"));
        }
        Ok(points)
    }

    /// 申万宏源研究 — 指数成分股.
    pub async fn index_component_sw(&self, symbol: &str) -> Result<Vec<SwResearchComponent>> {
        let response = crate::util::send_and_check(
            self.get("https://www.swsresearch.com/institute-sw/api/index_publish/details/component_stocks/")
                .query(&[("swindexcode", symbol), ("page", "1"), ("page_size", "10000")])
        )
        .await?;

        let payload: SwComponentEnvelope = response.json().await.map_err(Error::from)?;
        let items = payload.data.and_then(|d| d.results).unwrap_or_default();

        let components: Vec<SwResearchComponent> = items
            .into_iter()
            .map(|item| SwResearchComponent {
                symbol: item.stockcode,
                name: item.stockname,
                weight: item.newweight.unwrap_or(0.0),
                date: item.beginningdate,
            })
            .collect();

        if components.is_empty() {
            return Err(Error::not_found("swsresearch returned no component data"));
        }
        Ok(components)
    }

    /// 申万宏源研究 — 指数系列实时行情.
    ///
    /// `symbol` is one of: "市场表征", "一级行业", "二级行业", "风格指数".
    pub async fn index_realtime_sw(&self, symbol: &str) -> Result<Vec<SwResearchRealtime>> {
        if symbol == "大类风格指数" || symbol == "金创指数" {
            return self.index_realtime_sw_page_list(symbol).await;
        }

        let response = crate::util::send_and_check(
            self.get("https://www.swsresearch.com/institute-sw/api/index_publish/current/")
                .query(&[("page", "1"), ("page_size", "200"), ("indextype", symbol)]),
        )
        .await?;

        let payload: SwCurrentEnvelope = response.json().await.map_err(Error::from)?;
        let results = payload.data.and_then(|d| d.results).unwrap_or_default();

        let items: Vec<SwResearchRealtime> = results
            .into_iter()
            .filter_map(|row| {
                if row.len() < 9 {
                    return None;
                }
                Some(SwResearchRealtime {
                    code: row[0].as_str()?.to_string(),
                    name: row[1].as_str().unwrap_or("").to_string(),
                    prev_close: row[2].as_f64().unwrap_or(0.0),
                    change_pct: 0.0, // Not directly available in this format
                    year_change_pct: 0.0,
                })
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("swsresearch returned no realtime data"));
        }
        Ok(items)
    }

    /// 申万宏源研究 — 指数分析 (日报告).
    ///
    /// `symbol` is one of: "市场表征", "一级行业", "二级行业", "风格指数".
    pub async fn index_analysis_daily_sw(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<SwAnalysisPoint>> {
        let sd = format_dash_date(start_date);
        let ed = format_dash_date(end_date);

        let response = crate::util::send_and_check(
            self.get("https://www.swsresearch.com/institute-sw/api/index_analysis/index_analysis_report/")
                .query(&[
                ("page", "1"),
                ("page_size", "50"),
                ("index_type", symbol),
                ("start_date", sd.as_str()),
                ("end_date", ed.as_str()),
                ("type", "DAY"),
                ("swindexcode", "all"),
                ])
        )
        .await?;

        fetch_sw_analysis(self, &response.text().await.map_err(Error::from)?).await
    }

    /// 申万宏源研究 — 周/月报表日期序列.
    pub async fn index_analysis_week_month_sw(&self, symbol: &str) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct DateEnvelope {
            data: Option<Vec<DateItem>>,
        }
        #[derive(Deserialize)]
        struct DateItem {
            #[serde(default)]
            bargaindate: String,
        }

        let period = symbol.to_uppercase();

        let response = crate::util::send_and_check(
            self.get(
                "https://www.swsresearch.com/institute-sw/api/index_analysis/week_month_datetime/",
            )
            .query(&[("type", period.as_str())]),
        )
        .await?;

        let payload: DateEnvelope = response.json().await.map_err(Error::from)?;
        let dates: Vec<String> = payload
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|d| d.bargaindate)
            .collect();

        if dates.is_empty() {
            return Err(Error::not_found("swsresearch returned no date series"));
        }
        Ok(dates)
    }

    /// 申万宏源研究 — 指数分析 (周报告).
    pub async fn index_analysis_weekly_sw(
        &self,
        symbol: &str,
        date: &str,
    ) -> Result<Vec<SwAnalysisPoint>> {
        let d = format_dash_date(date);

        let response = crate::util::send_and_check(
            self.get("https://www.swsresearch.com/institute-sw/api/index_analysis/index_analysis_reports/")
                .query(&[
                ("page", "1"),
                ("page_size", "50"),
                ("index_type", symbol),
                ("bargaindate", d.as_str()),
                ("type", "WEEK"),
                ("swindexcode", "all"),
                ])
        )
        .await?;

        fetch_sw_analysis(self, &response.text().await.map_err(Error::from)?).await
    }

    /// 申万宏源研究 — 指数分析 (月报告).
    pub async fn index_analysis_monthly_sw(
        &self,
        symbol: &str,
        date: &str,
    ) -> Result<Vec<SwAnalysisPoint>> {
        let d = format_dash_date(date);

        let response = crate::util::send_and_check(
            self.get("https://www.swsresearch.com/institute-sw/api/index_analysis/index_analysis_reports/")
                .query(&[
                ("page", "1"),
                ("page_size", "50"),
                ("index_type", symbol),
                ("bargaindate", d.as_str()),
                ("type", "MONTH"),
                ("swindexcode", "all"),
                ])
        )
        .await?;

        fetch_sw_analysis(self, &response.text().await.map_err(Error::from)?).await
    }

    // Private: page list for 大类风格/金创
    async fn index_realtime_sw_page_list(&self, symbol: &str) -> Result<Vec<SwResearchRealtime>> {
        let response = crate::util::send_and_check(
            self.post("https://www.swsresearch.com/insWechatSw/dflgOrJcIndex/pageList")
                .json(&serde_json::json!({
                "pageNo": 1,
                "pageSize": 50,
                "indexTypeName": symbol,
                "sortField": "",
                "rule": "",
                "indexType": 1,
                })),
        )
        .await?;

        let payload: SwPageListEnvelope = response.json().await.map_err(Error::from)?;
        let items = payload.data.and_then(|d| d.list).unwrap_or_default();

        let result: Vec<SwResearchRealtime> = items
            .into_iter()
            .map(|item| SwResearchRealtime {
                code: item.sw_index_code,
                name: item.sw_index_name,
                prev_close: item.last_close_index.unwrap_or(0.0),
                change_pct: item.last_markup.unwrap_or(0.0),
                year_change_pct: item.year_markup.unwrap_or(0.0),
            })
            .collect();

        if result.is_empty() {
            return Err(Error::not_found("swsresearch returned no page list data"));
        }
        Ok(result)
    }
}

// ---------------------------------------------------------------------------
// Supporting types
// ---------------------------------------------------------------------------

/// Shenwan minute data point.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SwMinPoint {
    pub code: String,
    pub name: String,
    pub price: f64,
    pub date: String,
    pub time: String,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn fetch_sw_analysis(_client: &AkShareClient, body: &str) -> Result<Vec<SwAnalysisPoint>> {
    let payload: SwAnalysisEnvelope = serde_json::from_str(body)?;
    let items = payload.data.and_then(|d| d.results).unwrap_or_default();

    let points: Vec<SwAnalysisPoint> = items
        .into_iter()
        .map(|item| SwAnalysisPoint {
            code: item.swindexcode,
            name: item.swindexname,
            date: item.bargaindate,
            close: item.closeindex.unwrap_or(0.0),
            volume: item.bargainamount.unwrap_or(0.0),
            change_pct: item.markup.unwrap_or(0.0),
            turnover_rate: item.turnoverrate.unwrap_or(0.0),
            pe: item.pe.unwrap_or(0.0),
            pb: item.pb.unwrap_or(0.0),
            avg_price: item.meanprice.unwrap_or(0.0),
            amount_ratio: item.bargainsumrate.unwrap_or(0.0),
            circ_mv: item.negotiablessharesum1.unwrap_or(0.0),
            avg_circ_mv: item.negotiablessharesum2.unwrap_or(0.0),
            dividend_yield: item.dp.unwrap_or(0.0),
        })
        .collect();

    if points.is_empty() {
        return Err(Error::not_found("swsresearch returned no analysis data"));
    }
    Ok(points)
}

fn format_dash_date(d: &str) -> String {
    let d = d.trim();
    if d.len() == 8 && d.chars().all(|c| c.is_ascii_digit()) {
        format!("{}-{}-{}", &d[..4], &d[4..6], &d[6..8])
    } else {
        d.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_dash_date() {
        assert_eq!(format_dash_date("20241025"), "2024-10-25");
        assert_eq!(format_dash_date("2024-10-25"), "2024-10-25");
    }
}
