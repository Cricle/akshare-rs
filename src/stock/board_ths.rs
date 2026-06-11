//! THS (Tonghuashun) board data — concept and industry boards.
//!
//! Covers Python functions:
//! - `stock_board_concept_name_ths` — Concept board names from THS
//! - `stock_board_concept_info_ths` — Concept board info from THS
//! - `stock_board_concept_index_ths` — Concept board index from THS
//! - `stock_board_concept_summary_ths` — Concept board summary from THS
//! - `stock_board_industry_name_ths` — Industry board names from THS
//! - `stock_board_industry_info_ths` — Industry board info from THS
//! - `stock_board_industry_index_ths` — Industry board index from THS
//! - `stock_board_industry_summary_ths` — Industry board summary from THS

use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use crate::client::AkShareClient;
use crate::error::{Error, Result};

static RE_BOARD_HREF: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"href="[^"]*?(?:gn|thshy)/detail/code/(\d+)/"[^>]*>([^<]+)</a>"#).unwrap()
});
static RE_BOARD_HREF_FALLBACK: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"<a[^>]+href="[^"]*?/code/(\d+)/?"[^>]*>([^<]+)</a>"#).unwrap()
});
static RE_DT: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<dt[^>]*>([^<]+)</dt>").unwrap());
static RE_DD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<dd[^>]*>([\s\S]*?)</dd>").unwrap());
static RE_DATE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(\d{4}-\d{2}-\d{2})").unwrap());

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// THS board name entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThsBoardName {
    pub name: String,
    pub code: String,
}

/// THS board info entry (key-value pair).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThsBoardInfo {
    pub item: String,
    pub value: String,
}

/// THS board index kline point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThsBoardIndexPoint {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub amount: f64,
}

/// THS board summary entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThsBoardSummary {
    pub date: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub stock_count: Option<i64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    // -- Concept board THS ---------------------------------------------------

    /// Get concept board names from THS.
    ///
    /// Python equivalent: `stock_board_concept_name_ths()`
    ///
    /// Returns name and code pairs for all concept boards.
    pub async fn stock_board_concept_name_ths(&self) -> Result<Vec<ThsBoardName>> {
        self.fetch_ths_board_names("https://q.10jqka.com.cn/gn/detail/code/307822/")
            .await
    }

    /// Get concept board info from THS.
    ///
    /// Python equivalent: `stock_board_concept_info_ths(symbol)`
    ///
    /// `symbol` is the board name, e.g. "阿里巴巴概念".
    pub async fn stock_board_concept_info_ths(&self, symbol: &str) -> Result<Vec<ThsBoardInfo>> {
        let boards = self.stock_board_concept_name_ths().await?;
        let code = boards
            .iter()
            .find(|b| b.name == symbol)
            .map(|b| b.code.as_str())
            .ok_or_else(|| Error::not_found(format!("THS concept board not found: {symbol}")))?;

        self.fetch_ths_board_info(&format!("https://q.10jqka.com.cn/gn/detail/code/{code}/"))
            .await
    }

    /// Get concept board index data from THS.
    ///
    /// Python equivalent: `stock_board_concept_index_ths(symbol, start_date, end_date)`
    ///
    /// Returns daily OHLCV data for the concept board index.
    /// Note: THS requires JavaScript execution for full data; this returns
    /// data from the THS kline API directly.
    pub async fn stock_board_concept_index_ths(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<ThsBoardIndexPoint>> {
        let boards = self.stock_board_concept_name_ths().await?;
        let code = boards
            .iter()
            .find(|b| b.name == symbol)
            .map(|b| b.code.as_str())
            .ok_or_else(|| Error::not_found(format!("THS concept board not found: {symbol}")))?;

        self.fetch_ths_board_index(code, start_date, end_date).await
    }

    /// Get concept board summary from THS.
    ///
    /// Python equivalent: `stock_board_concept_summary_ths(symbol)`
    ///
    /// Returns concept board timeline/summary data.
    pub async fn stock_board_concept_summary_ths(
        &self,
        symbol: &str,
    ) -> Result<Vec<ThsBoardSummary>> {
        let boards = self.stock_board_concept_name_ths().await?;
        let code = boards
            .iter()
            .find(|b| b.name == symbol)
            .map(|b| b.code.as_str())
            .ok_or_else(|| Error::not_found(format!("THS concept board not found: {symbol}")))?;

        self.fetch_ths_board_summary(&format!("https://q.10jqka.com.cn/gn/detail/code/{code}/"))
            .await
    }

    // -- Industry board THS --------------------------------------------------

    /// Get industry board names from THS.
    ///
    /// Python equivalent: `stock_board_industry_name_ths()`
    pub async fn stock_board_industry_name_ths(&self) -> Result<Vec<ThsBoardName>> {
        self.fetch_ths_board_names("https://q.10jqka.com.cn/thshy/detail/code/881272/")
            .await
    }

    /// Get industry board info from THS.
    ///
    /// Python equivalent: `stock_board_industry_info_ths(symbol)`
    pub async fn stock_board_industry_info_ths(&self, symbol: &str) -> Result<Vec<ThsBoardInfo>> {
        let boards = self.stock_board_industry_name_ths().await?;
        let code = boards
            .iter()
            .find(|b| b.name == symbol)
            .map(|b| b.code.as_str())
            .ok_or_else(|| Error::not_found(format!("THS industry board not found: {symbol}")))?;

        self.fetch_ths_board_info(&format!(
            "https://q.10jqka.com.cn/thshy/detail/code/{code}/"
        ))
        .await
    }

    /// Get industry board index data from THS.
    ///
    /// Python equivalent: `stock_board_industry_index_ths(symbol, start_date, end_date)`
    pub async fn stock_board_industry_index_ths(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<ThsBoardIndexPoint>> {
        let boards = self.stock_board_industry_name_ths().await?;
        let code = boards
            .iter()
            .find(|b| b.name == symbol)
            .map(|b| b.code.as_str())
            .ok_or_else(|| Error::not_found(format!("THS industry board not found: {symbol}")))?;

        self.fetch_ths_board_index(code, start_date, end_date).await
    }

    /// Get industry board summary from THS.
    ///
    /// Python equivalent: `stock_board_industry_summary_ths(symbol)`
    pub async fn stock_board_industry_summary_ths(
        &self,
        symbol: &str,
    ) -> Result<Vec<ThsBoardSummary>> {
        let boards = self.stock_board_industry_name_ths().await?;
        let code = boards
            .iter()
            .find(|b| b.name == symbol)
            .map(|b| b.code.as_str())
            .ok_or_else(|| Error::not_found(format!("THS industry board not found: {symbol}")))?;

        self.fetch_ths_board_summary(&format!(
            "https://q.10jqka.com.cn/thshy/detail/code/{code}/"
        ))
        .await
    }

    // -- Private helpers -----------------------------------------------------

    /// Fetch board names from a THS category page.
    async fn fetch_ths_board_names(&self, url: &str) -> Result<Vec<ThsBoardName>> {
        let response = self
                        .get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let html = response.text().await.map_err(Error::from)?;

        // Parse HTML to extract board names and codes from href links
        let mut boards = Vec::new();
        // Look for links with pattern /gn/detail/code/XXXXXX/ or /thshy/detail/code/XXXXXX/
        for cap in RE_BOARD_HREF.captures_iter(&html) {
            let code = cap[1].to_string();
            let name = cap[2].trim().to_string();
            if !name.is_empty() {
                boards.push(ThsBoardName { name, code });
            }
        }

        // Also try parsing from the cate_inner div pattern
        if boards.is_empty() {
            // Fallback: look for data in the page's JavaScript or structured elements
            for cap in RE_BOARD_HREF_FALLBACK.captures_iter(&html) {
                let code = cap[1].to_string();
                let name = cap[2].trim().to_string();
                if !name.is_empty() && code.len() >= 4 {
                    boards.push(ThsBoardName { name, code });
                }
            }
        }

        if boards.is_empty() {
            return Err(Error::not_found("THS returned no board names"));
        }
        Ok(boards)
    }

    /// Fetch board info from a THS detail page.
    async fn fetch_ths_board_info(&self, url: &str) -> Result<Vec<ThsBoardInfo>> {
        let response = self
                        .get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let html = response.text().await.map_err(Error::from)?;

        let mut items = Vec::new();

        // Parse dt/dd pairs from board-infos div
        let dt_matches: Vec<&str> = RE_DT
            .captures_iter(&html)
            .map(|c| c.get(1).unwrap().as_str().trim())
            .collect();
        let dd_matches: Vec<String> = RE_DD
            .captures_iter(&html)
            .map(|c| {
                c.get(1)
                    .unwrap()
                    .as_str()
                    .replace("<br>", "/")
                    .replace("<br/>", "/")
                    .trim()
                    .to_string()
            })
            .collect();

        for (i, item_name) in dt_matches.iter().enumerate() {
            if let Some(val) = dd_matches.get(i) {
                items.push(ThsBoardInfo {
                    item: item_name.to_string(),
                    value: val.clone(),
                });
            }
        }

        if items.is_empty() {
            return Err(Error::not_found("THS board info not found"));
        }
        Ok(items)
    }

    /// Fetch board index kline data from THS.
    async fn fetch_ths_board_index(
        &self,
        board_code: &str,
        _start_date: &str,
        _end_date: &str,
    ) -> Result<Vec<ThsBoardIndexPoint>> {
        // THS kline data is served from d.10jqka.com.cn
        // The inner_code is typically the same as the board_code for THS boards
        let _current_year = chrono::Utc::now().format("%Y").to_string();
        let mut all_points = Vec::new();

        // Fetch kline data for each year
        let start_year: i32 = _start_date[..4].parse().unwrap_or(2020);
        let end_year: i32 = _end_date[..4].parse().unwrap_or(2025);

        for year in start_year..=end_year {
            let url = format!(
                "https://d.10jqka.com.cn/v4/line/bk_{}/01/{}.js",
                board_code, year
            );

            let response = match self
                                .get(&url)
                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .header("Referer", "http://q.10jqka.com.cn")
                .send()
                .await
            {
                Ok(r) => r,
                Err(_) => continue,
            };

            let text = match response.text().await {
                Ok(t) => t,
                Err(_) => continue,
            };

            // Parse JS response: find JSON object
            let json_start = match text.find('{') {
                Some(pos) => pos,
                None => continue,
            };
            let json_text = &text[json_start..text.len().saturating_sub(1)];

            let data: serde_json::Value = match serde_json::from_str(json_text) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let data_str = match data.get("data").and_then(|v| v.as_str()) {
                Some(s) => s,
                None => continue,
            };

            // Each record is semicolon-separated, fields are comma-separated
            for record in data_str.split(';') {
                let parts: Vec<&str> = record.split(',').collect();
                if parts.len() < 7 {
                    continue;
                }
                all_points.push(ThsBoardIndexPoint {
                    date: parts[0].to_string(),
                    open: parts[1].parse().unwrap_or(0.0),
                    high: parts[3].parse().unwrap_or(0.0),
                    low: parts[4].parse().unwrap_or(0.0),
                    close: parts[2].parse().unwrap_or(0.0),
                    volume: parts[5].parse().unwrap_or(0.0),
                    amount: parts[6].parse().unwrap_or(0.0),
                });
            }
        }

        if all_points.is_empty() {
            return Err(Error::not_found("THS board index returned no data"));
        }
        Ok(all_points)
    }

    /// Fetch board summary data from THS.
    async fn fetch_ths_board_summary(&self, url: &str) -> Result<Vec<ThsBoardSummary>> {
        let response = self
                        .get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let html = response.text().await.map_err(Error::from)?;

        let mut summaries = Vec::new();

        // Parse table data - look for date patterns and associated data
        for cap in RE_DATE.captures_iter(&html) {
            let date = cap[1].to_string();
            summaries.push(ThsBoardSummary {
                date,
                name: None,
                code: None,
                stock_count: None,
            });
        }

        if summaries.is_empty() {
            return Err(Error::not_found("THS board summary returned no data"));
        }
        Ok(summaries)
    }
}
