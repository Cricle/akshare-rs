//! Eastmoney miscellaneous stock data — block trades, repurchase, company events,
//! fund holdings, shareholder changes, summaries, industry, forecasts, comparisons.
//!
//! Covers Python functions:
//! - `stock_dzjy_sctj` — Block trade market statistics
//! - `stock_dzjy_mrmx` — Block trade daily details
//! - `stock_repurchase` — Stock repurchase data
//! - `stock_gsrl_gsdt` — Company events calendar
//! - `stock_report_fund_hold` — Fund holdings
//! - `stock_share_hold_change_sse` — Shareholder changes (SSE)
//! - `stock_share_hold_change_szse` — Shareholder changes (SZSE)
//! - `stock_szse_summary` — SZSE market summary
//! - `stock_sse_summary` — SSE market summary
//! - `stock_sector_spot` — Sector spot (Sina)
//! - `stock_rank_forecast` — Analyst forecasts
//! - `stock_zh_growth_comparison` — Growth comparison
//! - `stock_zh_valuation_comparison` — Valuation comparison
//! - `stock_hk_growth_comparison` — HK growth comparison
//! - `stock_hk_valuation_comparison` — HK valuation comparison

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::value_ext::ValueExt;

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
                    .str_field(&["TRADE_DATE"])
                    .map(std::string::ToString::to_string),
                sh_index: v.f64_field(&["SZ_INDEX"]),
                sh_change_rate: v.f64_field(&["SZ_CHANGE_RATE"]),
                blocktrade_deal_amt: v.f64_field(&["BLOCKTRADE_DEAL_AMT"]),
                premium_deal_amt: v.f64_field(&["PREMIUM_DEAL_AMT"]),
                premium_ratio: v.f64_field(&["PREMIUM_RATIO"]),
                discount_deal_amt: v.f64_field(&["DISCOUNT_DEAL_AMT"]),
                discount_ratio: v.f64_field(&["DISCOUNT_RATIO"]),
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
                    .str_field(&["TRADE_DATE"])
                    .map(std::string::ToString::to_string),
                symbol: v
                    .str_field(&["SECURITY_CODE"])
                    .map(std::string::ToString::to_string),
                name: v
                    .str_field(&["SECURITY_NAME_ABBR"])
                    .map(std::string::ToString::to_string),
                close_price: v.f64_field(&["CLOSE_PRICE"]),
                change_rate: v.f64_field(&["CHANGE_RATE"]),
                deal_price: v.f64_field(&["DEAL_PRICE"]),
                deal_volume: v.f64_field(&["DEAL_VOLUME"]),
                deal_amount: v.f64_field(&["DEAL_AMT"]),
                premium_ratio: v.f64_field(&["PREMIUM_RATIO"]),
                buyer: v
                    .str_field(&["BUYER_NAME"])
                    .map(std::string::ToString::to_string),
                seller: v
                    .str_field(&["SELLER_NAME"])
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
    /// Python equivalent: `stock_repurchase()`
    pub async fn stock_repurchase(&self, limit: usize) -> Result<Vec<RepurchaseEntry>> {
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
                    .str_field(&["DIM_SCODE"])
                    .map(std::string::ToString::to_string),
                name: v
                    .str_field(&["SECURITYSHORTNAME"])
                    .map(std::string::ToString::to_string),
                latest_price: v.f64_field(&["NEWPRICE"]),
                repurchase_price_cap: v.f64_field(&["REPURPRICECAP"]),
                repurchase_num_lower: v.f64_field(&["REPURNUMLOWER"]),
                repurchase_num_cap: v.f64_field(&["REPURNUMCAP"]),
                repurchase_amount_lower: v.f64_field(&["JEXX"]),
                repurchase_amount_cap: v.f64_field(&["JESX"]),
                start_date: v
                    .str_field(&["DIM_TRADEDATE"])
                    .map(std::string::ToString::to_string),
                progress: v
                    .str_field(&["REPURPROGRESS"])
                    .map(std::string::ToString::to_string),
                repurchased_num: v.f64_field(&["REPURNUM"]),
                repurchased_amount: v.f64_field(&["REPURAMOUNT"]),
                update_date: v
                    .str_field(&["UPDATEDATE"])
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
    /// Python equivalent: `stock_gsrl_gsdt(date)`
    pub async fn stock_gsrl_gsdt(&self, date: &str) -> Result<Vec<CompanyEvent>> {
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
                    .str_field(&["SECURITY_CODE"])
                    .map(std::string::ToString::to_string),
                name: v
                    .str_field(&["SECURITY_NAME_ABBR"])
                    .map(std::string::ToString::to_string),
                event_type: v
                    .str_field(&["EVENT_TYPE"])
                    .map(std::string::ToString::to_string),
                event_content: v
                    .str_field(&["EVENT_CONTENT"])
                    .map(std::string::ToString::to_string),
                trade_date: v
                    .str_field(&["TRADE_DATE"])
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
                    .str_field(&["SECURITY_CODE"])
                    .or_else(|| v.str_field(&["SCODE"]))
                    .map(std::string::ToString::to_string),
                name: v
                    .str_field(&["SECURITY_NAME_ABBR"])
                    .or_else(|| v.str_field(&["SNAME"]))
                    .map(std::string::ToString::to_string),
                holder_count: v.i64_field(&["HOULD_NUM"]),
                hold_shares: v.f64_field(&["HOLD_NUM"]),
                hold_market_value: v.f64_field(&["HOLD_MARKET_CAP"]),
                change: v
                    .str_field(&["HOLD_CHANGE"])
                    .map(std::string::ToString::to_string),
                change_amount: v.f64_field(&["HOLDCHANGE"]),
                change_ratio: v.f64_field(&["HOLD_RATIO_CHANGE"]),
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
                    .str_field(&["STAT_NAME"])
                    .map(std::string::ToString::to_string),
                count: v.i64_field(&["STAT_NUM"]),
                trade_amount: v.f64_field(&["TRADE_AMOUNT"]),
                total_market_cap: v.f64_field(&["TOTAL_MARKET_CAP"]),
                float_market_cap: v.f64_field(&["FLOAT_MARKET_CAP"]),
            })
            .collect();

        Ok(items)
    }

    // -- Peer comparison ----------------------------------------------------

    /// Get A-share growth comparison from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_growth_comparison(symbol)`
    ///
    /// `symbol` uses the format "SZ000895" or "SH600000".
    pub async fn stock_zh_growth_comparison(&self, symbol: &str) -> Result<Vec<PeerComparison>> {
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
    /// Python equivalent: `stock_zh_valuation_comparison(symbol)`
    pub async fn stock_zh_valuation_comparison(
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
    /// Python equivalent: `stock_hk_growth_comparison(symbol)`
    pub async fn stock_hk_growth_comparison(&self, symbol: &str) -> Result<Vec<PeerComparison>> {
        let filter = format!("(SECUCODE=\"{symbol}.HK\")(CORRE_SECUCODE=\"{symbol}.HK\")");
        self.fetch_peer_comparison("RPT_PCF10_INDUSTRY_HKGROWTH", &filter, "F10")
            .await
    }

    /// Get HK valuation comparison from Eastmoney.
    ///
    /// Python equivalent: `stock_hk_valuation_comparison(symbol)`
    pub async fn stock_hk_valuation_comparison(
        &self,
        symbol: &str,
    ) -> Result<Vec<PeerComparison>> {
        let filter = format!("(SECUCODE=\"{symbol}.HK\")(CORRE_SECUCODE=\"{symbol}.HK\")");
        self.fetch_peer_comparison("RPT_PCF10_INDUSTRY_HKCVALUE", &filter, "F10")
            .await
    }

    /// Get US growth comparison from Eastmoney.
    ///
    /// Python equivalent: `stock_us_growth_comparison(symbol)`
    pub async fn stock_us_growth_comparison(&self, symbol: &str) -> Result<Vec<PeerComparison>> {
        let filter = format!("(SECUCODE=\"{symbol}.OQ\")(CORRE_SECUCODE=\"{symbol}.OQ\")");
        self.fetch_peer_comparison("RPT_PCF10_INDUSTRY_USGROWTH", &filter, "F10")
            .await
    }

    /// Get US valuation comparison from Eastmoney.
    ///
    /// Python equivalent: `stock_us_valuation_comparison(symbol)`
    pub async fn stock_us_valuation_comparison(
        &self,
        symbol: &str,
    ) -> Result<Vec<PeerComparison>> {
        let filter = format!("(SECUCODE=\"{symbol}.OQ\")(CORRE_SECUCODE=\"{symbol}.OQ\")");
        self.fetch_peer_comparison("RPT_PCF10_INDUSTRY_USCVALUE", &filter, "F10")
            .await
    }

    /// Get A-share financial indicators from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_a_financial_indicator(symbol)`
    ///
    /// `symbol` uses the format "SZ000895" or "SH600000".
    pub async fn stock_zh_a_financial_indicator(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        #[derive(Deserialize)]
        struct Env {
            result: Option<EnvResult>,
        }
        #[derive(Deserialize)]
        struct EnvResult {
            data: Option<Vec<serde_json::Value>>,
        }
        let filter = if symbol.len() >= 2 {
            let (prefix, code) = symbol.split_at(2);
            format!("(SECUCODE=\"{code}.{prefix}\")")
        } else {
            return Err(Error::invalid_input("symbol must be like SZ000895"));
        };

        let url = "https://datacenter.eastmoney.com/securities/api/data/v1/get";
        let response = self
            .get(url)
            .query(&[
                ("reportName", "RPT_F10_FN_MAINFINADATA"),
                ("columns", "ALL"),
                ("quoteColumns", ""),
                ("filter", filter.as_str()),
                ("pageNumber", "1"),
                ("pageSize", ""),
                ("sortTypes", "-1"),
                ("sortColumns", "REPORT_DATE"),
                ("source", "HSF10"),
                ("client", "PC"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("A-share financial indicators missing data"))?;

        if data.is_empty() {
            return Err(Error::not_found(
                "A-share financial indicators returned no data",
            ));
        }
        Ok(data)
    }

    /// Get A-share dividend payout from Eastmoney.
    ///
    /// Python equivalent: `stock_zh_a_dividend_payout(symbol)`
    ///
    /// `symbol` uses the format "SZ000895" or "SH600000".
    pub async fn stock_zh_a_dividend_payout(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        #[derive(Deserialize)]
        struct Env {
            result: Option<EnvResult>,
        }
        #[derive(Deserialize)]
        struct EnvResult {
            data: Option<Vec<serde_json::Value>>,
        }
        let filter = if symbol.len() >= 2 {
            let (prefix, code) = symbol.split_at(2);
            format!("(SECUCODE=\"{code}.{prefix}\")")
        } else {
            return Err(Error::invalid_input("symbol must be like SZ000895"));
        };

        let url = "https://datacenter.eastmoney.com/securities/api/data/v1/get";
        let response = self
            .get(url)
            .query(&[
                ("reportName", "RPT_F10_EH_DIVIDEND"),
                ("columns", "ALL"),
                ("quoteColumns", ""),
                ("filter", filter.as_str()),
                ("pageNumber", "1"),
                ("pageSize", ""),
                ("sortTypes", "-1"),
                ("sortColumns", "EX_DIVIDEND_DATE"),
                ("source", "HSF10"),
                ("client", "PC"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: Env = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .and_then(|r| r.data)
            .ok_or_else(|| Error::upstream("A-share dividend payout missing data"))?;

        if data.is_empty() {
            return Err(Error::not_found("A-share dividend payout returned no data"));
        }
        Ok(data)
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

        let Some(data) = payload
            .get("result")
            .and_then(|r| r.get("data"))
            .and_then(|v| v.as_array())
        else {
            return Ok(vec![]);
        };

        let items: Vec<PeerComparison> = data
            .iter()
            .map(|v| {
                let symbol = v
                    .str_field(&["CORRE_SECURITY_CODE"])
                    .map(std::string::ToString::to_string);
                let name = v
                    .str_field(&["CORRE_SECURITY_NAME"])
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
