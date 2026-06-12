#![allow(dead_code)]
//! Eastmoney miscellaneous stock data — block trades, repurchase, company events,
//! fund holdings, shareholder changes, summaries, industry, forecasts, comparisons.
//!
//! Covers Python functions:
//! - `stock_dzjy_sctj` — Block trade market statistics
//! - `stock_dzjy_mrmx` — Block trade daily details
//! - `stock_repurchase_em` — Stock repurchase data
//! - `stock_gsrl_gsdt_em` — Company events calendar
//! - `stock_report_fund_hold` — Fund holdings
//! - `stock_share_hold_change_sse` — Shareholder changes (SSE)
//! - `stock_share_hold_change_szse` — Shareholder changes (SZSE)
//! - `stock_szse_summary` — SZSE market summary
//! - `stock_sse_summary` — SSE market summary
//! - `stock_sector_spot` — Sector spot (Sina)
//! - `stock_rank_forecast_cninfo` — Analyst forecasts
//! - `stock_zh_growth_comparison_em` — Growth comparison
//! - `stock_zh_valuation_comparison_em` — Valuation comparison
//! - `stock_hk_growth_comparison_em` — HK growth comparison
//! - `stock_hk_valuation_comparison_em` — HK valuation comparison

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct DatacenterEnvelope {
    result: Option<DatacenterResult>,
}

#[derive(Debug, Deserialize)]
struct DatacenterResult {
    data: Option<Vec<serde_json::Value>>,
    pages: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct DatacenterEnvelopeGeneric<T> {
    result: Option<DatacenterResultGeneric<T>>,
}

#[derive(Debug, Deserialize)]
struct DatacenterResultGeneric<T> {
    data: Option<Vec<T>>,
    pages: Option<i64>,
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Block trade market statistics entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockTradeStat {
    #[serde(default)]
    pub trade_date: Option<String>,
    #[serde(default)]
    pub sh_index: Option<f64>,
    #[serde(default)]
    pub sh_change_rate: Option<f64>,
    #[serde(default)]
    pub blocktrade_deal_amt: Option<f64>,
    #[serde(default)]
    pub premium_deal_amt: Option<f64>,
    #[serde(default)]
    pub premium_ratio: Option<f64>,
    #[serde(default)]
    pub discount_deal_amt: Option<f64>,
    #[serde(default)]
    pub discount_ratio: Option<f64>,
}

/// Block trade daily detail entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockTradeDetail {
    #[serde(default)]
    pub trade_date: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub close_price: Option<f64>,
    #[serde(default)]
    pub change_rate: Option<f64>,
    #[serde(default)]
    pub deal_price: Option<f64>,
    #[serde(default)]
    pub deal_volume: Option<f64>,
    #[serde(default)]
    pub deal_amount: Option<f64>,
    #[serde(default)]
    pub premium_ratio: Option<f64>,
    #[serde(default)]
    pub buyer: Option<String>,
    #[serde(default)]
    pub seller: Option<String>,
}

/// Stock repurchase entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepurchaseEntry {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub latest_price: Option<f64>,
    #[serde(default)]
    pub repurchase_price_cap: Option<f64>,
    #[serde(default)]
    pub repurchase_num_lower: Option<f64>,
    #[serde(default)]
    pub repurchase_num_cap: Option<f64>,
    #[serde(default)]
    pub repurchase_amount_lower: Option<f64>,
    #[serde(default)]
    pub repurchase_amount_cap: Option<f64>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub progress: Option<String>,
    #[serde(default)]
    pub repurchased_num: Option<f64>,
    #[serde(default)]
    pub repurchased_amount: Option<f64>,
    #[serde(default)]
    pub update_date: Option<String>,
}

/// Company event calendar entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyEvent {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub event_type: Option<String>,
    #[serde(default)]
    pub event_content: Option<String>,
    #[serde(default)]
    pub trade_date: Option<String>,
}

/// Fund holdings entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundHoldEntry {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub holder_count: Option<i64>,
    #[serde(default)]
    pub hold_shares: Option<f64>,
    #[serde(default)]
    pub hold_market_value: Option<f64>,
    #[serde(default)]
    pub change: Option<String>,
    #[serde(default)]
    pub change_amount: Option<f64>,
    #[serde(default)]
    pub change_ratio: Option<f64>,
}

/// Shareholder change entry (SSE format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderChange {
    #[serde(default)]
    pub company_code: Option<String>,
    #[serde(default)]
    pub company_name: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub stock_type: Option<String>,
    #[serde(default)]
    pub change_date: Option<String>,
    #[serde(default)]
    pub change_num: Option<f64>,
    #[serde(default)]
    pub avg_price: Option<f64>,
    #[serde(default)]
    pub hold_after: Option<f64>,
    #[serde(default)]
    pub reason: Option<String>,
}

/// Market summary entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSummary {
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub trade_amount: Option<f64>,
    #[serde(default)]
    pub total_market_cap: Option<f64>,
    #[serde(default)]
    pub float_market_cap: Option<f64>,
}

/// Analyst forecast entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalystForecast {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub publish_date: Option<String>,
    #[serde(default)]
    pub institution: Option<String>,
    #[serde(default)]
    pub analyst: Option<String>,
    #[serde(default)]
    pub rating: Option<String>,
    #[serde(default)]
    pub is_first: Option<bool>,
    #[serde(default)]
    pub rating_change: Option<String>,
    #[serde(default)]
    pub prev_rating: Option<String>,
    #[serde(default)]
    pub target_price_lower: Option<f64>,
    #[serde(default)]
    pub target_price_upper: Option<f64>,
}

/// Peer comparison entry (growth or valuation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerComparison {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten)]
    pub metrics: std::collections::HashMap<String, serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    // -- Block trades -------------------------------------------------------

    /// Get block trade market statistics from Eastmoney.
    ///
    /// Python equivalent: `stock_dzjy_sctj()`
    pub async fn stock_dzjy_sctj(&self, limit: usize) -> Result<Vec<BlockTradeStat>> {
        let page_size = limit.to_string();
        let response = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("sortColumns", "TRADE_DATE"),
                ("sortTypes", "-1"),
                ("pageSize", page_size.as_str()),
                ("pageNumber", "1"),
                ("reportName", "PRT_BLOCKTRADE_MARKET_STA"),
                (
                    "columns",
                    "TRADE_DATE,SZ_INDEX,SZ_CHANGE_RATE,BLOCKTRADE_DEAL_AMT,PREMIUM_DEAL_AMT,\
                     PREMIUM_RATIO,DISCOUNT_DEAL_AMT,DISCOUNT_RATIO",
                ),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: DatacenterEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("eastmoney block trade stats missing data"))?;

        let items: Vec<BlockTradeStat> = data
            .into_iter()
            .map(|v| BlockTradeStat {
                trade_date: v
                    .get("TRADE_DATE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                sh_index: v.get("SZ_INDEX").and_then(serde_json::Value::as_f64),
                sh_change_rate: v.get("SZ_CHANGE_RATE").and_then(serde_json::Value::as_f64),
                blocktrade_deal_amt: v
                    .get("BLOCKTRADE_DEAL_AMT")
                    .and_then(serde_json::Value::as_f64),
                premium_deal_amt: v
                    .get("PREMIUM_DEAL_AMT")
                    .and_then(serde_json::Value::as_f64),
                premium_ratio: v.get("PREMIUM_RATIO").and_then(serde_json::Value::as_f64),
                discount_deal_amt: v
                    .get("DISCOUNT_DEAL_AMT")
                    .and_then(serde_json::Value::as_f64),
                discount_ratio: v.get("DISCOUNT_RATIO").and_then(serde_json::Value::as_f64),
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no block trade stats"));
        }
        Ok(items)
    }

    /// Get block trade daily details from Eastmoney.
    ///
    /// Python equivalent: `stock_dzjy_mrmx(symbol, start_date, end_date)`
    ///
    /// `asset_type` is one of: "astock", "bstock", "fund", "bond".
    pub async fn stock_dzjy_mrmx(
        &self,
        asset_type: &str,
        start_date: &str,
        end_date: &str,
        limit: usize,
    ) -> Result<Vec<BlockTradeDetail>> {
        let asset_code = match asset_type {
            "astock" => "1",
            "bstock" => "2",
            "fund" => "3",
            "bond" => "4",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported asset type: {asset_type}"
                )));
            }
        };

        let sd = format!(
            "{}-{}-{}",
            &start_date[..4],
            &start_date[4..6],
            &start_date[6..8]
        );
        let ed = format!("{}-{}-{}", &end_date[..4], &end_date[4..6], &end_date[6..8]);
        let filter =
            format!("(MARKET_TYPE=\"{asset_code}\")(TRADE_DATE>='{sd}')(TRADE_DATE<='{ed}')");
        let page_size = limit.min(5000).to_string();

        let response = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("sortColumns", "SECURITY_CODE"),
                ("sortTypes", "1"),
                ("pageSize", page_size.as_str()),
                ("pageNumber", "1"),
                ("reportName", "RPT_DATA_BLOCKTRADE"),
                (
                    "columns",
                    "TRADE_DATE,SECURITY_CODE,SECURITY_NAME_ABBR,CHANGE_RATE,CLOSE_PRICE,\
                     DEAL_PRICE,DEAL_VOLUME,DEAL_AMT,PREMIUM_RATIO,BUYER_NAME,SELLER_NAME",
                ),
                ("filter", filter.as_str()),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: DatacenterEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("eastmoney block trade details missing data"))?;

        let items: Vec<BlockTradeDetail> = data
            .into_iter()
            .map(|v| BlockTradeDetail {
                trade_date: v
                    .get("TRADE_DATE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                symbol: v
                    .get("SECURITY_CODE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                name: v
                    .get("SECURITY_NAME_ABBR")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                close_price: v.get("CLOSE_PRICE").and_then(serde_json::Value::as_f64),
                change_rate: v.get("CHANGE_RATE").and_then(serde_json::Value::as_f64),
                deal_price: v.get("DEAL_PRICE").and_then(serde_json::Value::as_f64),
                deal_volume: v.get("DEAL_VOLUME").and_then(serde_json::Value::as_f64),
                deal_amount: v.get("DEAL_AMT").and_then(serde_json::Value::as_f64),
                premium_ratio: v.get("PREMIUM_RATIO").and_then(serde_json::Value::as_f64),
                buyer: v
                    .get("BUYER_NAME")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                seller: v
                    .get("SELLER_NAME")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no block trade details",
            ));
        }
        Ok(items)
    }

    // -- Repurchase ---------------------------------------------------------

    /// Get stock repurchase data from Eastmoney.
    ///
    /// Python equivalent: `stock_repurchase_em()`
    pub async fn stock_repurchase_em(&self, limit: usize) -> Result<Vec<RepurchaseEntry>> {
        let page_size = limit.min(500).to_string();
        let response = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("sortColumns", "UPD,DIM_DATE,DIM_SCODE"),
                ("sortTypes", "-1,-1,-1"),
                ("pageSize", page_size.as_str()),
                ("pageNumber", "1"),
                ("reportName", "RPTA_WEB_GETHGLIST_NEW"),
                ("columns", "ALL"),
                ("source", "WEB"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: DatacenterEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("eastmoney repurchase missing data"))?;

        let items: Vec<RepurchaseEntry> = data
            .into_iter()
            .map(|v| RepurchaseEntry {
                symbol: v
                    .get("DIM_SCODE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                name: v
                    .get("SECURITYSHORTNAME")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                latest_price: v.get("NEWPRICE").and_then(serde_json::Value::as_f64),
                repurchase_price_cap: v.get("REPURPRICECAP").and_then(serde_json::Value::as_f64),
                repurchase_num_lower: v.get("REPURNUMLOWER").and_then(serde_json::Value::as_f64),
                repurchase_num_cap: v.get("REPURNUMCAP").and_then(serde_json::Value::as_f64),
                repurchase_amount_lower: v.get("JEXX").and_then(serde_json::Value::as_f64),
                repurchase_amount_cap: v.get("JESX").and_then(serde_json::Value::as_f64),
                start_date: v
                    .get("DIM_TRADEDATE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                progress: v
                    .get("REPURPROGRESS")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                repurchased_num: v.get("REPURNUM").and_then(serde_json::Value::as_f64),
                repurchased_amount: v.get("REPURAMOUNT").and_then(serde_json::Value::as_f64),
                update_date: v
                    .get("UPDATEDATE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no repurchase items"));
        }
        Ok(items)
    }

    // -- Company events calendar --------------------------------------------

    /// Get company events calendar from Eastmoney.
    ///
    /// Python equivalent: `stock_gsrl_gsdt_em(date)`
    pub async fn stock_gsrl_gsdt_em(&self, date: &str) -> Result<Vec<CompanyEvent>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        let filter = format!("(TRADE_DATE='{date_fmt}')");
        let response = self
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("sortColumns", "SECURITY_CODE"),
                ("sortTypes", "1"),
                ("pageSize", "5000"),
                ("pageNumber", "1"),
                (
                    "columns",
                    "SECURITY_CODE,SECUCODE,SECURITY_NAME_ABBR,EVENT_TYPE,EVENT_CONTENT,TRADE_DATE",
                ),
                ("source", "WEB"),
                ("client", "WEB"),
                ("reportName", "RPT_ORGOP_ALL"),
                ("filter", filter.as_str()),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: DatacenterEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("eastmoney company events missing data"))?;

        let items: Vec<CompanyEvent> = data
            .into_iter()
            .map(|v| CompanyEvent {
                symbol: v
                    .get("SECURITY_CODE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                name: v
                    .get("SECURITY_NAME_ABBR")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                event_type: v
                    .get("EVENT_TYPE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                event_content: v
                    .get("EVENT_CONTENT")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                trade_date: v
                    .get("TRADE_DATE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no company events"));
        }
        Ok(items)
    }

    // -- Fund holdings ------------------------------------------------------

    /// Get fund holdings data from Eastmoney.
    ///
    /// Python equivalent: `stock_report_fund_hold(symbol, date)`
    ///
    /// `holder_type` is one of: "fund", "qfii", "social", "broker", "insurance", "trust".
    /// `date` is in format "20210331".
    pub async fn stock_report_fund_hold(
        &self,
        holder_type: &str,
        date: &str,
        limit: usize,
    ) -> Result<Vec<FundHoldEntry>> {
        let type_code = match holder_type {
            "fund" => "1",
            "qfii" => "2",
            "social" => "3",
            "broker" => "4",
            "insurance" => "5",
            "trust" => "6",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported holder type: {holder_type}"
                )));
            }
        };
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        let page_size = limit.min(500).to_string();

        let response = self
            .get("http://data.eastmoney.com/dataapi/zlsj/list")
            .query(&[
                ("date", date_fmt.as_str()),
                ("type", type_code),
                ("zjc", "0"),
                ("sortField", "HOULD_NUM"),
                ("sortDirec", "1"),
                ("pageNum", "1"),
                ("pageSize", page_size.as_str()),
                ("p", "1"),
                ("pageNo", "1"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("data")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::upstream("eastmoney fund hold missing data"))?;

        let items: Vec<FundHoldEntry> = data
            .iter()
            .take(limit)
            .map(|v| FundHoldEntry {
                symbol: v
                    .get("SECURITY_CODE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string)
                    .or_else(|| {
                        v.get("SCODE")
                            .and_then(|v| v.as_str())
                            .map(std::string::ToString::to_string)
                    }),
                name: v
                    .get("SECURITY_NAME_ABBR")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string)
                    .or_else(|| {
                        v.get("SNAME")
                            .and_then(|v| v.as_str())
                            .map(std::string::ToString::to_string)
                    }),
                holder_count: v.get("HOULD_NUM").and_then(serde_json::Value::as_i64),
                hold_shares: v.get("HOLD_NUM").and_then(serde_json::Value::as_f64),
                hold_market_value: v.get("HOLD_MARKET_CAP").and_then(serde_json::Value::as_f64),
                change: v
                    .get("HOLD_CHANGE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                change_amount: v.get("HOLDCHANGE").and_then(serde_json::Value::as_f64),
                change_ratio: v
                    .get("HOLD_RATIO_CHANGE")
                    .and_then(serde_json::Value::as_f64),
            })
            .collect();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no fund hold items"));
        }
        Ok(items)
    }

    // -- Market summary -----------------------------------------------------

    /// Get SSE market summary from Eastmoney.
    ///
    /// Python equivalent: `stock_sse_summary(date)`
    pub async fn stock_sse_summary(&self, date: &str) -> Result<Vec<MarketSummary>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..8]);
        let response = self
            .get("https://query.sse.com.cn/commonQuery.do")
            .query(&[
                ("isPagination", "false"),
                ("sqlId", "COMMON_SSE_XXPL_LSSJL_S"),
                ("STAT_DATE", date_fmt.as_str()),
            ])
            .header("Referer", "https://www.sse.com.cn/")
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let data = payload
            .get("result")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::upstream("SSE summary missing data"))?;

        let items: Vec<MarketSummary> = data
            .iter()
            .map(|v| MarketSummary {
                category: v
                    .get("STAT_NAME")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string),
                count: v.get("STAT_NUM").and_then(serde_json::Value::as_i64),
                trade_amount: v.get("TRADE_AMOUNT").and_then(serde_json::Value::as_f64),
                total_market_cap: v
                    .get("TOTAL_MARKET_CAP")
                    .and_then(serde_json::Value::as_f64),
                float_market_cap: v
                    .get("FLOAT_MARKET_CAP")
                    .and_then(serde_json::Value::as_f64),
            })
            .collect();

        Ok(items)
    }

    // -- Peer comparison ----------------------------------------------------

    /// Get A-share growth comparison from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_growth_comparison_em(symbol)`
    ///
    /// `symbol` uses the format "SZ000895" or "SH600000".
    pub async fn stock_zh_growth_comparison_em(&self, symbol: &str) -> Result<Vec<PeerComparison>> {
        let secucode = if symbol.len() >= 8 {
            let (prefix, code) = symbol.split_at(2);
            format!("{code}.{prefix}")
        } else {
            return Err(Error::invalid_input("symbol must be in format SZ000895"));
        };
        let filter = format!("(SECUCODE=\"{secucode}\")");
        self.fetch_peer_comparison("RPT_PCF10_INDUSTRY_GROWTH", &filter, "HSF10")
            .await
    }

    /// Get A-share valuation comparison from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_valuation_comparison_em(symbol)`
    pub async fn stock_zh_valuation_comparison_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<PeerComparison>> {
        let secucode = if symbol.len() >= 8 {
            let (prefix, code) = symbol.split_at(2);
            format!("{code}.{prefix}")
        } else {
            return Err(Error::invalid_input("symbol must be in format SZ000895"));
        };
        let filter = format!("(SECUCODE=\"{secucode}\")");
        self.fetch_peer_comparison("RPT_PCF10_INDUSTRY_CVALUE", &filter, "HSF10")
            .await
    }

    /// Get HK growth comparison from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_growth_comparison_em(symbol)`
    pub async fn stock_hk_growth_comparison_em(&self, symbol: &str) -> Result<Vec<PeerComparison>> {
        let filter = format!("(SECUCODE=\"{symbol}.HK\")(CORRE_SECUCODE=\"{symbol}.HK\")");
        self.fetch_peer_comparison("RPT_PCF10_INDUSTRY_HKGROWTH", &filter, "F10")
            .await
    }

    /// Get HK valuation comparison from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_valuation_comparison_em(symbol)`
    pub async fn stock_hk_valuation_comparison_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<PeerComparison>> {
        let filter = format!("(SECUCODE=\"{symbol}.HK\")(CORRE_SECUCODE=\"{symbol}.HK\")");
        self.fetch_peer_comparison("RPT_PCF10_INDUSTRY_HKCVALUE", &filter, "F10")
            .await
    }

    // -- Private helpers ----------------------------------------------------

    async fn fetch_peer_comparison(
        &self,
        report_name: &str,
        filter: &str,
        source: &str,
    ) -> Result<Vec<PeerComparison>> {
        let response = self
            .get("https://datacenter.eastmoney.com/securities/api/data/v1/get")
            .query(&[
                ("reportName", report_name),
                ("columns", "ALL"),
                ("quoteColumns", ""),
                ("filter", filter),
                ("pageNumber", ""),
                ("pageSize", ""),
                ("sortTypes", "1"),
                ("sortColumns", "PAIMING"),
                ("source", source),
                ("client", "PC"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;

        let data = match payload.get("result").and_then(|r| r.get("data")) {
            Some(val) if val.is_array() => val.as_array().unwrap(),
            _ => return Ok(vec![]),
        };

        let items: Vec<PeerComparison> = data
            .iter()
            .map(|v| {
                let symbol = v
                    .get("CORRE_SECURITY_CODE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string);
                let name = v
                    .get("CORRE_SECURITY_NAME")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string);

                let mut metrics = std::collections::HashMap::new();
                if let Some(obj) = v.as_object() {
                    for (key, val) in obj {
                        if key != "CORRE_SECURITY_CODE" && key != "CORRE_SECURITY_NAME" {
                            metrics.insert(key.clone(), val.clone());
                        }
                    }
                }

                PeerComparison {
                    symbol,
                    name,
                    metrics,
                }
            })
            .collect();

        Ok(items)
    }
}
