//! Eastmoney hot rankings — stock popularity, keywords, rising stocks.
//!
//! Covers Python functions:
//! - `stock_hot_rank` — Hot stock popularity ranking
//! - `stock_hot_rank_detail` — Historical trend for a stock
//! - `stock_hot_rank_detail_realtime` — Realtime rank changes
//! - `stock_hot_keyword` — Hot keywords for a stock
//! - `stock_hot_up` — Rising stocks (飙升榜)

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Hot rank entry from the popularity ranking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotRankEntry {
    #[serde(default)]
    pub rank: Option<i64>,
    pub symbol: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
}

/// Historical rank trend entry for a stock.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotRankTrendEntry {
    pub time: String,
    pub rank: i64,
    #[serde(default)]
    pub new_fan_rate: Option<f64>,
    #[serde(default)]
    pub old_fan_rate: Option<f64>,
}

/// Realtime rank change entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotRankRealtimeEntry {
    pub time: String,
    pub rank: i64,
}

/// Hot keyword entry for a stock.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotKeywordEntry {
    pub time: String,
    #[serde(default)]
    pub stock_code: Option<String>,
    pub concept_name: String,
    #[serde(default)]
    pub concept_code: Option<String>,
    #[serde(default)]
    pub heat: Option<f64>,
}

/// Hot rising stock entry (飙升榜).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotUpEntry {
    #[serde(default)]
    pub rank_change: Option<i64>,
    #[serde(default)]
    pub rank: Option<i64>,
    pub symbol: String,
    pub name: String,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub change_pct: Option<f64>,
}

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct RankListEnvelope {
    data: Option<Vec<RankItem>>,
}

#[derive(Debug, Deserialize)]
struct RankItem {
    sc: Option<String>,
    rk: Option<i64>,
    #[serde(default)]
    hrc: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct ProfileEnvelope {
    data: Option<Vec<ProfileItem>>,
}

#[derive(Debug, Deserialize)]
struct ProfileItem {
    #[serde(rename = "newUidRate")]
    new_uid_rate: Option<String>,
    #[serde(rename = "oldUidRate")]
    old_uid_rate: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RealtimeEnvelope {
    data: Option<Vec<RealtimeItem>>,
}

#[derive(Debug, Deserialize)]
struct RealtimeItem {
    #[serde(rename = "currentTime")]
    current_time: Option<String>,
    #[serde(rename = "currentRanking")]
    current_ranking: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct KeywordEnvelope {
    data: Option<Vec<KeywordItem>>,
}

#[derive(Debug, Deserialize)]
struct KeywordItem {
    #[serde(rename = "dateTime")]
    date_time: Option<String>,
    #[serde(rename = "securityCode")]
    security_code: Option<String>,
    #[serde(rename = "conceptName")]
    concept_name: Option<String>,
    #[serde(rename = "conceptCode")]
    concept_code: Option<String>,
    #[serde(rename = "hotNum")]
    hot_num: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct UlistEnvelope {
    data: Option<UlistData>,
}

#[derive(Debug, Deserialize)]
struct UlistData {
    diff: Option<Vec<serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get stock hot popularity ranking from Eastmoney.
    ///
    /// Python equivalent: `stock_hot_rank()`
    pub async fn stock_hot_rank(&self, limit: usize) -> Result<Vec<HotRankEntry>> {
        // Step 1: Get rank data
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "marketType": "",
            "pageNo": 1,
            "pageSize": limit,
        });

        let rank_response = self
            .post("https://emappdata.eastmoney.com/stockrank/getAllCurrentList")
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let rank_data: RankListEnvelope = rank_response.json().await.map_err(Error::from)?;
        let rank_items = rank_data.data.unwrap_or_default();

        if rank_items.is_empty() {
            return Err(Error::not_found("eastmoney returned no hot rank items"));
        }

        // Step 2: Build secids for quote lookup
        let secids: Vec<String> = rank_items
            .iter()
            .filter_map(|item| {
                let sc = item.sc.as_ref()?;
                let code = sc.get(2..)?;
                if sc.contains("SZ") {
                    Some(format!("0.{code}"))
                } else {
                    Some(format!("1.{code}"))
                }
            })
            .collect();

        let secids_str = format!("{},?v=08926209912590994", secids.join(","));

        // Step 3: Get quotes
        let ulist_response = self
            .get("https://push2.eastmoney.com/api/qt/ulist.np/get")
            .query(&[
                ("ut", "f057cbcbce2a86e2866ab8877db1d059"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fields", "f14,f3,f12,f2"),
                ("secids", secids_str.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let ulist_data: UlistEnvelope = ulist_response.json().await.map_err(Error::from)?;
        let quotes = ulist_data.data.and_then(|d| d.diff).unwrap_or_default();

        // Step 4: Merge rank + quotes
        let mut entries = Vec::new();
        for (i, rank_item) in rank_items.iter().enumerate() {
            let symbol = rank_item.sc.clone().unwrap_or_default();
            let quote = quotes.get(i);
            let latest_price = quote
                .and_then(|q| q.get("f2"))
                .and_then(serde_json::Value::as_f64);
            let change_pct = quote
                .and_then(|q| q.get("f3"))
                .and_then(serde_json::Value::as_f64);
            let name = quote
                .and_then(|q| q.get("f14"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            entries.push(HotRankEntry {
                rank: rank_item.rk,
                symbol,
                name,
                latest_price,
                change_amount: latest_price.zip(change_pct).map(|(p, c)| p * c / 100.0),
                change_pct,
            });
        }

        Ok(entries)
    }

    /// Get historical trend and fan characteristics for a stock.
    ///
    /// Python equivalent: `stock_hot_rank_detail(symbol)`
    ///
    /// `symbol` uses the Eastmoney format like "SZ000665".
    pub async fn stock_hot_rank_detail(&self, symbol: &str) -> Result<Vec<HotRankTrendEntry>> {
        // Step 1: Get rank history
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "marketType": "",
            "srcSecurityCode": symbol,
            "yearType": "5",
        });

        let rank_response = self
            .post("https://emappdata.eastmoney.com/stockrank/getHisList")
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let rank_data: RankListEnvelope = rank_response.json().await.map_err(Error::from)?;

        // Step 2: Get fan profile
        let profile_response = self
            .post("https://emappdata.eastmoney.com/stockrank/getHisProfileList")
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let profile_data: ProfileEnvelope = profile_response.json().await.map_err(Error::from)?;

        let rank_items = rank_data.data.unwrap_or_default();
        let profile_items = profile_data.data.unwrap_or_default();

        let mut entries = Vec::new();
        for (i, rank_item) in rank_items.iter().enumerate() {
            let time = rank_item.sc.clone().unwrap_or_default();
            let rank = rank_item.rk.unwrap_or(0);
            let new_fan_rate = profile_items
                .get(i)
                .and_then(|p| p.new_uid_rate.as_ref())
                .and_then(|s| s.trim_end_matches('%').parse::<f64>().ok())
                .map(|v| v / 100.0);
            let old_fan_rate = profile_items
                .get(i)
                .and_then(|p| p.old_uid_rate.as_ref())
                .and_then(|s| s.trim_end_matches('%').parse::<f64>().ok())
                .map(|v| v / 100.0);

            entries.push(HotRankTrendEntry {
                time,
                rank,
                new_fan_rate,
                old_fan_rate,
            });
        }

        entries.sort_by(|a, b| a.time.cmp(&b.time));
        Ok(entries)
    }

    /// Get realtime rank changes for a stock.
    ///
    /// Python equivalent: `stock_hot_rank_detail_realtime(symbol)`
    ///
    /// `symbol` uses the Eastmoney format like "SZ000665".
    pub async fn stock_hot_rank_detail_realtime(
        &self,
        symbol: &str,
    ) -> Result<Vec<HotRankRealtimeEntry>> {
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "marketType": "",
            "srcSecurityCode": symbol,
        });

        let response = self
            .post("https://emappdata.eastmoney.com/stockrank/getCurrentList")
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let data: RealtimeEnvelope = response.json().await.map_err(Error::from)?;

        Ok(data
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|item| HotRankRealtimeEntry {
                time: item.current_time.unwrap_or_default(),
                rank: item.current_ranking.unwrap_or(0),
            })
            .collect())
    }

    /// Get hot keywords for a stock.
    ///
    /// Python equivalent: `stock_hot_keyword(symbol)`
    ///
    /// `symbol` uses the Eastmoney format like "SZ000665".
    pub async fn stock_hot_keyword(&self, symbol: &str) -> Result<Vec<HotKeywordEntry>> {
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "srcSecurityCode": symbol,
        });

        let response = self
            .post("https://emappdata.eastmoney.com/stockrank/getHotStockRankList")
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let data: KeywordEnvelope = response.json().await.map_err(Error::from)?;

        Ok(data
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|item| HotKeywordEntry {
                time: item.date_time.unwrap_or_default(),
                stock_code: item.security_code,
                concept_name: item.concept_name.unwrap_or_default(),
                concept_code: item.concept_code,
                heat: item.hot_num,
            })
            .collect())
    }

    /// Get rising stocks (飙升榜) from Eastmoney.
    ///
    /// Python equivalent: `stock_hot_up()`
    pub async fn stock_hot_up(&self, limit: usize) -> Result<Vec<HotUpEntry>> {
        // Step 1: Get rising rank data
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "marketType": "",
            "pageNo": 1,
            "pageSize": limit,
        });

        let rank_response = self
            .post("https://emappdata.eastmoney.com/stockrank/getAllHisRcList")
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let rank_data: RankListEnvelope = rank_response.json().await.map_err(Error::from)?;
        let rank_items = rank_data.data.unwrap_or_default();

        if rank_items.is_empty() {
            return Err(Error::not_found("eastmoney returned no hot up items"));
        }

        // Step 2: Build secids for quote lookup
        let secids: Vec<String> = rank_items
            .iter()
            .filter_map(|item| {
                let sc = item.sc.as_ref()?;
                let code = sc.get(2..)?;
                if sc.contains("SZ") {
                    Some(format!("0.{code}"))
                } else {
                    Some(format!("1.{code}"))
                }
            })
            .collect();

        let secids_str = format!("{},?v=08926209912590994", secids.join(","));

        // Step 3: Get quotes
        let ulist_response = self
            .get("https://push2.eastmoney.com/api/qt/ulist.np/get")
            .query(&[
                ("ut", "f057cbcbce2a86e2866ab8877db1d059"),
                ("fltt", "2"),
                ("invt", "2"),
                ("fields", "f14,f3,f12,f2"),
                ("secids", secids_str.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let ulist_data: UlistEnvelope = ulist_response.json().await.map_err(Error::from)?;
        let quotes = ulist_data.data.and_then(|d| d.diff).unwrap_or_default();

        // Step 4: Merge rank + quotes
        let mut entries = Vec::new();
        for (i, rank_item) in rank_items.iter().enumerate() {
            let symbol = rank_item.sc.clone().unwrap_or_default();
            let quote = quotes.get(i);
            let latest_price = quote
                .and_then(|q| q.get("f2"))
                .and_then(serde_json::Value::as_f64);
            let change_pct = quote
                .and_then(|q| q.get("f3"))
                .and_then(serde_json::Value::as_f64);
            let name = quote
                .and_then(|q| q.get("f14"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            entries.push(HotUpEntry {
                rank_change: rank_item.hrc,
                rank: rank_item.rk,
                symbol,
                name,
                latest_price,
                change_amount: latest_price.zip(change_pct).map(|(p, c)| p * c / 100.0),
                change_pct,
            });
        }

        Ok(entries)
    }

    /// 最新热度排名 (Python: stock_hot_rank_latest)
    pub async fn stock_hot_rank_latest(&self, symbol: &str) -> Result<Vec<HotRankTrendEntry>> {
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "srcSecurityCode": symbol,
        });
        let response = self
            .post("https://emappdata.eastmoney.com/stockrank/getCurrentLatest")
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let data: TrendEnvelope = response.json().await.map_err(Error::from)?;
        Ok(data
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|item| HotRankTrendEntry {
                time: item.date_time.unwrap_or_default(),
                rank: item.rank.unwrap_or(0),
                new_fan_rate: item.new_fan_rate,
                old_fan_rate: item.old_fan_rate,
            })
            .collect())
    }

    /// 相关热度排名 (Python: stock_hot_rank_relate)
    pub async fn stock_hot_rank_relate(&self, symbol: &str) -> Result<Vec<HotRankEntry>> {
        let payload = serde_json::json!({
            "appId": "appId01",
            "globalId": "786e4c21-70dc-435a-93bb-38",
            "srcSecurityCode": symbol,
            "pageNo": 1,
            "pageSize": 10,
        });
        let response = self
            .post("https://emappdata.eastmoney.com/stockrank/getFollowStockRankList")
            .json(&payload)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let data: RankListEnvelope = response.json().await.map_err(Error::from)?;
        Ok(data
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|item| HotRankEntry {
                rank: item.rk,
                symbol: item.sc.unwrap_or_default(),
                name: String::new(),
                latest_price: None,
                change_amount: None,
                change_pct: None,
            })
            .collect())
    }

    /// 百度热搜股票 (Python: stock_hot_search)
    pub async fn stock_hot_search(
        &self,
        symbol: &str,
        _date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let url = format!(
            "https://finance.pae.baidu.com/selfselect/listsugrecomm?srcid=5353&all=1&pointType=string&group=quotation_index&code={symbol}&market_type=ab&newFormat=1&finClientType=pc"
        );
        let response = self
            .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let data: serde_json::Value = response.json().await.map_err(Error::from)?;
        let items = data
            .get("result")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(items)
    }
}

/// Envelope for trend data
#[derive(Debug, Clone, serde::Deserialize)]
struct TrendEnvelope {
    data: Option<Vec<TrendItem>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TrendItem {
    #[serde(default)]
    date_time: Option<String>,
    #[serde(default)]
    rank: Option<i64>,
    #[serde(default)]
    new_fan_rate: Option<f64>,
    #[serde(default)]
    old_fan_rate: Option<f64>,
}
