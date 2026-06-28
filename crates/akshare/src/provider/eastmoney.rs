//! Eastmoney provider — A-share stock search, klines, sector rankings,
//! sector constituents, capital flow, billboard, announcements, and more.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::wire::{ClistResp, EmDatacenterResp, KlineResp};
use crate::types::{
    AnnouncementDetail, AnnouncementItem, BillboardEntry, BillboardSeatDetail, CandlePoint,
    CapitalFlowPoint, SectorConstituent, SectorSnapshot, StockSearchResult,
};
use crate::util::{parse_candle_line, parse_csv_line, parse_f64_safe};

use serde::Deserialize;

// ---------------------------------------------------------------------------
// Wire types — private to this module
// ---------------------------------------------------------------------------

// 1. Stock search

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SearchEnvelope {
    quotation_code_table: Option<SearchTable>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SearchTable {
    data: Option<Vec<SearchItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SearchItem {
    code: Option<String>,
    name: Option<String>,
    #[serde(rename = "JYS")]
    exchange: Option<String>,
    classify: Option<String>,
    #[serde(rename = "SecurityTypeName")]
    security_type_name: Option<String>,
}

// 2. Sector rankings / Sector constituents (clist API)

#[derive(Debug, Deserialize)]
struct ClistItem {
    #[serde(rename = "f12")]
    code: Option<String>,
    #[serde(rename = "f14")]
    name: Option<String>,
    #[serde(rename = "f2")]
    latest_price: Option<f64>,
    #[serde(rename = "f3")]
    change_pct: Option<f64>,
    #[serde(rename = "f62")]
    main_net_inflow: Option<f64>,
    #[serde(rename = "f184")]
    main_net_inflow_ratio_pct: Option<f64>,
}

// 4. Datacenter generic (billboard, billboard seats)

#[derive(Debug, Deserialize)]
struct DatacenterEnvelope<T> {
    result: Option<DatacenterResult<T>>,
}

#[derive(Debug, Deserialize)]
struct DatacenterResult<T> {
    data: Option<Vec<T>>,
}

#[derive(Debug, Deserialize)]
struct BillboardItem {
    #[serde(rename = "TRADE_DATE")]
    trade_date: Option<String>,
    #[serde(rename = "SECURITY_CODE")]
    security_code: Option<String>,
    #[serde(rename = "SECURITY_NAME_ABBR")]
    security_name: Option<String>,
    #[serde(rename = "CLOSE_PRICE")]
    close_price: Option<f64>,
    #[serde(rename = "CHANGE_RATE")]
    change_rate: Option<f64>,
    #[serde(rename = "TURNOVERRATE")]
    turnover_rate: Option<f64>,
    #[serde(rename = "BILLBOARD_NET_AMT")]
    net_amount: Option<f64>,
    #[serde(rename = "BILLBOARD_BUY_AMT")]
    buy_amount: Option<f64>,
    #[serde(rename = "BILLBOARD_SELL_AMT")]
    sell_amount: Option<f64>,
    #[serde(rename = "EXPLANATION")]
    explanation: Option<String>,
    #[serde(rename = "EXPLAIN")]
    explain: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BillboardSeatItem {
    #[serde(rename = "TRADE_DATE")]
    trade_date: Option<String>,
    #[serde(rename = "SECURITY_CODE")]
    security_code: Option<String>,
    #[serde(rename = "OPERATEDEPT_NAME")]
    department_name: Option<String>,
    #[serde(rename = "BUY")]
    buy_amount: Option<f64>,
    #[serde(rename = "SELL")]
    sell_amount: Option<f64>,
    #[serde(rename = "NET")]
    net_amount: Option<f64>,
    #[serde(rename = "EXPLANATION")]
    explanation: Option<String>,
}

// 5. Announcements

#[derive(Debug, Deserialize)]
struct AnnouncementEnvelope {
    data: Option<AnnouncementData>,
}

#[derive(Debug, Deserialize)]
struct AnnouncementData {
    list: Option<Vec<AnnouncementWireItem>>,
}

#[derive(Debug, Deserialize)]
struct AnnouncementWireItem {
    art_code: Option<String>,
    notice_date: Option<String>,
    title: Option<String>,
}

// 6. Announcement detail

#[derive(Debug, Deserialize)]
struct AnnouncementContentEnvelope {
    data: Option<AnnouncementContentData>,
}

#[derive(Debug, Deserialize)]
struct AnnouncementContentData {
    art_code: Option<String>,
    notice_title: Option<String>,
    notice_date: Option<String>,
    notice_content: Option<String>,
    attach_url: Option<String>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

const SEARCH_TOKEN: &str = "D43BF722C8E33BDC906FB84D85E326E8";

impl AkShareClient {
    /// Search for stocks via the Eastmoney suggestion API.
    ///
    /// `market` optionally filters results by market label (e.g. "A股", "港股", "美股").
    pub async fn eastmoney_search(
        &self,
        query: &str,
        market: Option<&str>,
        limit: usize,
    ) -> Result<Vec<StockSearchResult>> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Err(Error::invalid_input("search query is empty"));
        }

        let count = limit.clamp(1, 20).to_string();
        let response = crate::util::send_and_check(
            self.get("https://searchapi.eastmoney.com/api/suggest/get")
                .query(&[
                    ("input", trimmed),
                    ("type", "14"),
                    ("token", SEARCH_TOKEN),
                    ("count", count.as_str()),
                ]),
        )
        .await?;

        let payload: SearchEnvelope = response.json().await.map_err(Error::from)?;

        let items = payload
            .quotation_code_table
            .and_then(|table| table.data)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|item| {
                let symbol = item.code?;
                let name = item.name?;
                let exchange = item.exchange.unwrap_or_default();
                let market_name = classify_search_market(
                    item.classify.as_deref(),
                    item.security_type_name.as_deref(),
                )?;
                if let Some(expected) = market
                    && expected != market_name
                {
                    return None;
                }
                Some(StockSearchResult {
                    symbol,
                    name,
                    market: market_name.to_string(),
                    exchange,
                })
            })
            .take(limit)
            .collect::<Vec<_>>();

        Ok(items)
    }

    /// Fetch daily klines from Eastmoney.
    ///
    /// `secid` is the Eastmoney secid format (e.g. "1.600000" for SH, "0.000001" for SZ).
    /// Use `crate::market::eastmoney_secid` to convert a raw symbol.
    ///
    /// `adjust` is one of "qfq" (forward-adjusted), "hfq" (backward-adjusted), "none".
    pub async fn eastmoney_klines(
        &self,
        secid: &str,
        adjust: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let fqt = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "none" => "0",
            other => {
                return Err(Error::invalid_input(format!(
                    "unsupported adjust mode: {other}"
                )));
            }
        };
        let klines = self.kline_fetch(secid, "101", fqt, limit, &[]).await?;

        let items: Vec<CandlePoint> = klines
            .iter()
            .map(|line| parse_candle_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no kline items"));
        }

        Ok(crate::util::sort_and_limit(items, limit))
    }

    /// Fetch sector (industry or concept) rankings from Eastmoney.
    ///
    /// `sector_type` is "industry" or "concept".
    pub async fn eastmoney_sector_rankings(
        &self,
        sector_type: &str,
        limit: usize,
    ) -> Result<Vec<SectorSnapshot>> {
        let fs = match sector_type {
            "industry" => "m:90+t:2",
            "concept" => "m:90+t:3",
            other => {
                return Err(Error::invalid_input(format!(
                    "unsupported sector_type: {other}"
                )));
            }
        };

        let pz = limit.to_string();
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(
                    pz.as_str(),
                    &[
                        ("fid", "f62"),
                        ("fs", fs),
                        ("fields", "f12,f14,f2,f3,f62,f184"),
                    ],
                )),
        )
        .await?;

        let payload: ClistResp = response.json().await.map_err(Error::from)?;
        let items = payload
            .data
            .and_then(|d| d.diff)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| serde_json::from_value::<ClistItem>(v).ok())
            .map(|item| SectorSnapshot {
                sector_code: item.code.unwrap_or_default(),
                sector_name: item.name.unwrap_or_else(|| "未知板块".to_string()),
                latest_index: item.latest_price.unwrap_or_default(),
                change_pct: item.change_pct.unwrap_or_default(),
                main_net_inflow: item.main_net_inflow.unwrap_or_default(),
                main_net_inflow_ratio_pct: item.main_net_inflow_ratio_pct.unwrap_or_default(),
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no sector ranking items",
            ));
        }
        Ok(items)
    }

    /// Fetch the constituent stocks of a given sector from Eastmoney.
    pub async fn eastmoney_sector_constituents(
        &self,
        sector_code: &str,
        limit: usize,
    ) -> Result<Vec<SectorConstituent>> {
        let pz = limit.to_string();
        let fs = format!("b:{sector_code}+f:!50");
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(
                    pz.as_str(),
                    &[
                        ("fid", "f3"),
                        ("fs", fs.as_str()),
                        ("fields", "f12,f14,f2,f3,f62"),
                    ],
                )),
        )
        .await?;

        let payload: ClistResp = response.json().await.map_err(Error::from)?;
        let items = payload
            .data
            .and_then(|d| d.diff)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| serde_json::from_value::<ClistItem>(v).ok())
            .map(|item| SectorConstituent {
                symbol: item.code.unwrap_or_default(),
                name: item.name.unwrap_or_else(|| "未知成分股".to_string()),
                latest_price: item.latest_price.unwrap_or_default(),
                change_pct: item.change_pct.unwrap_or_default(),
                main_net_inflow: item.main_net_inflow,
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no sector constituent items",
            ));
        }
        Ok(items)
    }

    /// Fetch daily capital flow for a sector from Eastmoney.
    pub async fn eastmoney_sector_capital_flow(
        &self,
        sector_code: &str,
        limit: usize,
    ) -> Result<Vec<CapitalFlowPoint>> {
        let secid = format!("90.{}", sector_code.trim().to_uppercase());
        self.fetch_capital_flow(&secid, limit).await
    }

    /// Fetch sector rankings from Eastmoney with a custom `fs` parameter.
    ///
    /// This is the parameterized version of `eastmoney_sector_rankings` that
    /// allows passing market-specific `fs` values (e.g., `m:201+t:2` for HK
    /// industry sectors, `m:202+t:2` for US industry sectors).
    pub async fn eastmoney_sector_rankings_by_fs(
        &self,
        fs: &str,
        limit: usize,
    ) -> Result<Vec<SectorSnapshot>> {
        let pz = limit.to_string();
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(
                    pz.as_str(),
                    &[
                        ("fid", "f62"),
                        ("fs", fs),
                        ("fields", "f12,f14,f2,f3,f62,f184"),
                    ],
                )),
        )
        .await?;

        let payload: ClistResp = response.json().await.map_err(Error::from)?;
        let items = payload
            .data
            .and_then(|d| d.diff)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| serde_json::from_value::<ClistItem>(v).ok())
            .map(|item| SectorSnapshot {
                sector_code: item.code.unwrap_or_default(),
                sector_name: item.name.unwrap_or_else(|| "未知板块".to_string()),
                latest_index: item.latest_price.unwrap_or_default(),
                change_pct: item.change_pct.unwrap_or_default(),
                main_net_inflow: item.main_net_inflow.unwrap_or_default(),
                main_net_inflow_ratio_pct: item.main_net_inflow_ratio_pct.unwrap_or_default(),
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no sector ranking items",
            ));
        }
        Ok(items)
    }

    /// Fetch sector capital flow from Eastmoney with a custom secid prefix.
    ///
    /// For A-share sectors, use prefix `90`. For HK sectors, use `201`.
    /// For US sectors, use `202`.
    pub async fn eastmoney_sector_capital_flow_by_prefix(
        &self,
        prefix: &str,
        sector_code: &str,
        limit: usize,
    ) -> Result<Vec<CapitalFlowPoint>> {
        let secid = format!("{}.{}", prefix, sector_code.trim().to_uppercase());
        self.fetch_capital_flow(&secid, limit).await
    }

    /// Fetch daily capital flow for an individual stock from Eastmoney.
    ///
    /// `secid` is the Eastmoney secid format (e.g. "1.600000").
    pub async fn eastmoney_capital_flow(
        &self,
        secid: &str,
        limit: usize,
    ) -> Result<Vec<CapitalFlowPoint>> {
        self.fetch_capital_flow(secid, limit).await
    }

    /// Fetch billboard (dragon-tiger list) entries for a stock from Eastmoney.
    ///
    /// `symbol` is a raw code like "600000" (exchange suffix is stripped automatically).
    pub async fn eastmoney_billboard(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<BillboardEntry>> {
        let code = strip_exchange_suffix(symbol);
        let filter = format!("(SECURITY_CODE=\"{code}\")");
        let page_size = limit.to_string();

        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("reportName", "RPT_DAILYBILLBOARD_DETAILSNEW"),
                    ("columns", "ALL"),
                    ("filter", filter.as_str()),
                    ("pageNumber", "1"),
                    ("pageSize", page_size.as_str()),
                    ("sortTypes", "-1"),
                    ("sortColumns", "TRADE_DATE"),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: DatacenterEnvelope<BillboardItem> =
            response.json().await.map_err(Error::from)?;
        let items = payload
            .result
            .and_then(|r| r.data)
            .unwrap_or_default()
            .into_iter()
            .map(|item| BillboardEntry {
                trade_date: item.trade_date.unwrap_or_default(),
                symbol: item.security_code.unwrap_or_else(|| code.clone()),
                name: item.security_name.unwrap_or_else(|| "未知股票".to_string()),
                close_price: item.close_price.unwrap_or_default(),
                change_rate_pct: item.change_rate.unwrap_or_default(),
                turnover_rate_pct: item.turnover_rate,
                net_amount: item.net_amount,
                buy_amount: item.buy_amount,
                sell_amount: item.sell_amount,
                explanation: item.explanation,
                reason: item.explain,
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no billboard entries"));
        }
        Ok(items)
    }

    /// Fetch billboard seat (broker department) details from Eastmoney.
    ///
    /// `symbol` is a raw code like "600000".
    /// `side` is "buy" or "sell".
    pub async fn eastmoney_billboard_seats(
        &self,
        symbol: &str,
        side: &str,
        limit: usize,
    ) -> Result<Vec<BillboardSeatDetail>> {
        let report_name = match side {
            "buy" => "RPT_BILLBOARD_DAILYDETAILSBUY",
            "sell" => "RPT_BILLBOARD_DAILYDETAILSSELL",
            other => {
                return Err(Error::invalid_input(format!(
                    "unsupported billboard side: {other}"
                )));
            }
        };
        let code = strip_exchange_suffix(symbol);
        let filter = format!("(SECURITY_CODE=\"{code}\")");
        let page_size = limit.to_string();

        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("reportName", report_name),
                    ("columns", "ALL"),
                    ("filter", filter.as_str()),
                    ("pageNumber", "1"),
                    ("pageSize", page_size.as_str()),
                    ("sortTypes", "-1"),
                    ("sortColumns", "TRADE_DATE"),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: DatacenterEnvelope<BillboardSeatItem> =
            response.json().await.map_err(Error::from)?;
        let items = payload
            .result
            .and_then(|r| r.data)
            .unwrap_or_default()
            .into_iter()
            .map(|item| BillboardSeatDetail {
                trade_date: item.trade_date.unwrap_or_default(),
                symbol: item.security_code.unwrap_or_else(|| code.clone()),
                department_name: item
                    .department_name
                    .unwrap_or_else(|| "未知席位".to_string()),
                buy_amount: item.buy_amount,
                sell_amount: item.sell_amount,
                net_amount: item.net_amount,
                explanation: item.explanation,
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return Err(Error::not_found(
                "eastmoney returned no billboard seat items",
            ));
        }
        Ok(items)
    }

    /// Fetch announcements for a stock from Eastmoney.
    ///
    /// `symbol` is a raw code like "600000" (exchange suffix is stripped automatically).
    pub async fn eastmoney_announcements(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<AnnouncementItem>> {
        let code = strip_exchange_suffix(symbol);
        let page_size = limit.clamp(1, 100).to_string();

        let response = crate::util::send_and_check(
            self.get("https://np-anotice-stock.eastmoney.com/api/security/ann")
                .query(&[
                    ("page_size", page_size.as_str()),
                    ("page_index", "1"),
                    ("ann_type", "A"),
                    ("client_source", "web"),
                    ("stock_list", code.as_str()),
                ]),
        )
        .await?;

        let payload: AnnouncementEnvelope = response.json().await.map_err(Error::from)?;
        let mut items = payload
            .data
            .and_then(|d| d.list)
            .unwrap_or_default()
            .into_iter()
            .map(|item| {
                let art_code = item.art_code.unwrap_or_default();
                AnnouncementItem {
                    url: (!art_code.is_empty()).then(|| {
                        format!("https://data.eastmoney.com/notices/detail/{code}/{art_code}.html")
                    }),
                    art_code,
                    symbol: code.clone(),
                    title: item.title.unwrap_or_else(|| "公司公告".to_string()),
                    published_at: item.notice_date.unwrap_or_default(),
                    source: "Eastmoney 公告".to_string(),
                }
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no announcement items"));
        }
        items.truncate(limit);
        Ok(items)
    }

    /// Fetch the full content of a single announcement by its article code.
    pub async fn eastmoney_announcement_detail(
        &self,
        art_code: &str,
    ) -> Result<AnnouncementDetail> {
        let response = crate::util::send_and_check(
            self.get("https://np-cnotice-stock.eastmoney.com/api/content/ann")
                .query(&[
                    ("art_code", art_code),
                    ("client_source", "web"),
                    ("page_index", "1"),
                ]),
        )
        .await?;

        let payload: AnnouncementContentEnvelope = response.json().await.map_err(Error::from)?;
        let data = payload
            .data
            .ok_or_else(|| Error::upstream("eastmoney announcement detail missing data"))?;

        Ok(AnnouncementDetail {
            art_code: data.art_code.unwrap_or_else(|| art_code.to_string()),
            title: data.notice_title.unwrap_or_else(|| "公司公告".to_string()),
            published_at: data.notice_date.unwrap_or_default(),
            content: data.notice_content.unwrap_or_default(),
            pdf_url: data.attach_url,
            source: "Eastmoney 公告".to_string(),
        })
    }

    // -- shared Eastmoney API helpers ----------------------------------------
    // These are generic Eastmoney datacenter/push API wrappers used by
    // multiple modules (stock, index, futures, bond, fund, reits, forex, etc.).

    /// Generic Eastmoney datacenter fetch with pagination support.
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn dc_fetch_all(
        &self,
        report_name: &str,
        columns: &str,
        filter: &str,
        sort_columns: &str,
        sort_types: &str,
        page_size: i64,
        max_pages: i64,
        extra_params: &[(&str, &str)],
    ) -> Result<Vec<serde_json::Value>> {
        let mut all_data = Vec::new();
        let ps = page_size.to_string();

        let mut page = 1_i64;
        loop {
            let pn = page.to_string();
            let mut builder = self
                .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("reportName", report_name),
                    ("columns", columns),
                    ("filter", filter),
                    ("pageNumber", &pn),
                    ("pageSize", &ps),
                    ("sortTypes", sort_types),
                    ("sortColumns", sort_columns),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]);
            for &(k, v) in extra_params {
                builder = builder.query(&[(k, v)]);
            }

            let resp = crate::util::send_and_check(builder).await?;
            let payload: EmDatacenterResp = resp.json().await.map_err(Error::from)?;
            let result = payload
                .result
                .ok_or_else(|| Error::upstream("eastmoney datacenter missing result"))?;

            all_data.extend(result.data);

            let total_pages = result.pages.unwrap_or(1);
            if page >= total_pages || page >= max_pages {
                break;
            }
            page += 1;
        }

        if all_data.is_empty() {
            return Err(Error::not_found("eastmoney datacenter returned no data"));
        }
        Ok(all_data)
    }

    /// Generic Eastmoney push2ex fetch (for limit-up/down pools, order book changes).
    pub(crate) async fn push2ex_fetch(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        let url = format!("https://push2ex.eastmoney.com/{path}");
        let resp = crate::util::send_and_check(self.get(&url).query(params)).await?;
        let payload: serde_json::Value = resp.json().await.map_err(Error::from)?;
        Ok(payload)
    }

    /// Generic Eastmoney push2 clist fetch (for spot market data).
    pub(crate) async fn clist_spot_fetch(
        &self,
        fs: &str,
        fields: &str,
        page_size: &str,
        sort_field: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let resp = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(
                    page_size,
                    &[("fid", sort_field), ("fs", fs), ("fields", fields)],
                )),
        )
        .await?;

        let payload: ClistResp = resp.json().await.map_err(Error::from)?;
        let items = payload.data.and_then(|d| d.diff).unwrap_or_default();

        if items.is_empty() {
            return Err(Error::not_found("eastmoney clist returned no data"));
        }
        Ok(items)
    }

    /// Generic Eastmoney kline fetch from push2his.
    ///
    /// Returns raw kline strings (comma-separated OHLCV etc.) from the Eastmoney
    /// kline API with Sina fallback. Each string needs to be parsed by the caller with `parse_csv_line`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn kline_fetch(
        &self,
        secid: &str,
        klt: &str,
        fqt: &str,
        limit: usize,
        extra: &[(&str, &str)],
    ) -> Result<Vec<String>> {
        // Try Eastmoney push2his first
        let result = self
            .kline_fetch_eastmoney(secid, klt, fqt, limit, extra)
            .await;
        if result.is_ok() {
            return result;
        }

        // Fallback to Sina for A-share/HK stocks
        if let Ok(klines) = self.kline_fetch_sina(secid, klt, limit).await
            && !klines.is_empty()
        {
            return Ok(klines);
        }

        // Fallback to Tencent for A-share
        if let Ok(klines) = self.kline_fetch_tencent(secid, klt, fqt, limit).await
            && !klines.is_empty()
        {
            return Ok(klines);
        }

        // Fallback to Yahoo for US/HK stocks (market 105, 106, 107, 100)
        let parts: Vec<&str> = secid.split('.').collect();
        if parts.len() == 2
            && matches!(parts[0], "105" | "106" | "107" | "100")
            && let Ok(candles) = self.yahoo_candles(parts[1], limit).await
            && !candles.is_empty()
        {
            let klines: Vec<String> = candles
                .iter()
                .map(|c| {
                    format!(
                        "{},{},{},{},{},{},0,0,0,0,0",
                        c.trade_date, c.open, c.close, c.high, c.low, c.volume
                    )
                })
                .collect();
            return Ok(klines);
        }

        // Return original error
        self.kline_fetch_eastmoney(secid, klt, fqt, limit, extra)
            .await
    }

    /// Eastmoney push2his kline fetch.
    async fn kline_fetch_eastmoney(
        &self,
        secid: &str,
        klt: &str,
        fqt: &str,
        limit: usize,
        extra: &[(&str, &str)],
    ) -> Result<Vec<String>> {
        let lmt = limit.max(1).to_string();
        let mut builder = self
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", klt),
                ("fqt", fqt),
                ("lmt", lmt.as_str()),
                ("end", "20500000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ]);
        for &(k, v) in extra {
            builder = builder.query(&[(k, v)]);
        }

        let resp = crate::util::send_and_check(builder).await?;
        let payload: KlineResp = resp.json().await.map_err(Error::from)?;
        let klines = payload.data.and_then(|d| d.klines).unwrap_or_default();

        if klines.is_empty() {
            return Err(Error::not_found("eastmoney kline returned no data"));
        }
        Ok(klines)
    }

    /// Sina kline fallback for A-share stocks.
    /// Converts Eastmoney secid format (e.g., "1.600000") to Sina symbol (e.g., "sh600000").
    async fn kline_fetch_sina(&self, secid: &str, klt: &str, limit: usize) -> Result<Vec<String>> {
        // Parse secid: "1.600000" -> market=1 (SH), code="600000"
        // "0.000001" -> market=0 (SZ), code="000001"
        // "100.HSI" -> HK index, skip
        let parts: Vec<&str> = secid.split('.').collect();
        if parts.len() != 2 {
            return Err(Error::invalid_input("invalid secid format"));
        }
        let market = parts[0];
        let code = parts[1];

        // Only support A-share (market 0 or 1)
        let prefix = match market {
            "1" => "sh",
            "0" => "sz",
            _ => {
                return Err(Error::unsupported_market(
                    "sina kline only supports A-share",
                ));
            }
        };
        let symbol = format!("{prefix}{code}");

        // Map Eastmoney klt to Sina scale
        let scale = match klt {
            "1" => "5", // 1-min -> 5-min (Sina minimum)
            "5" => "5",
            "15" => "15",
            "30" => "30",
            "60" => "60",
            "101" => "240",  // daily
            "102" => "1680", // weekly
            "103" => "7200", // monthly
            _ => "240",
        };

        let url = format!(
            "https://money.finance.sina.com.cn/quotes_service/api/json_v2.php/CN_MarketData.getKLineData?symbol={symbol}&scale={scale}&ma=no&datalen={limit}"
        );

        let body: serde_json::Value = self
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .json()
            .await?;

        let arr = body.as_array().cloned().unwrap_or_default();
        let mut klines = Vec::with_capacity(arr.len());
        for item in &arr {
            let day = item.get("day").and_then(|v| v.as_str()).unwrap_or("");
            let open = item.get("open").and_then(|v| v.as_str()).unwrap_or("0");
            let high = item.get("high").and_then(|v| v.as_str()).unwrap_or("0");
            let low = item.get("low").and_then(|v| v.as_str()).unwrap_or("0");
            let close = item.get("close").and_then(|v| v.as_str()).unwrap_or("0");
            let volume = item.get("volume").and_then(|v| v.as_str()).unwrap_or("0");
            // Convert to Eastmoney CSV format: date,open,close,high,low,volume,...
            klines.push(format!(
                "{day},{open},{close},{high},{low},{volume},0,0,0,0,0"
            ));
        }

        Ok(klines)
    }

    /// Tencent kline fallback.
    /// Converts Eastmoney secid format to Tencent format.
    async fn kline_fetch_tencent(
        &self,
        secid: &str,
        klt: &str,
        fqt: &str,
        limit: usize,
    ) -> Result<Vec<String>> {
        let parts: Vec<&str> = secid.split('.').collect();
        if parts.len() != 2 {
            return Err(Error::invalid_input("invalid secid format"));
        }
        let market = parts[0];
        let code = parts[1];

        let prefix = match market {
            "1" => "sh",
            "0" => "sz",
            _ => {
                return Err(Error::unsupported_market(
                    "tencent kline only supports A-share",
                ));
            }
        };
        let symbol = format!("{prefix}{code}");

        // Map klt to period
        let period = match klt {
            "101" => "day",
            "102" => "week",
            "103" => "month",
            _ => "day",
        };

        // Map fqt to adjustment type
        let adjust = match fqt {
            "1" => "qfq",
            "2" => "hfq",
            _ => "",
        };

        let url = format!(
            "https://web.ifzq.gtimg.cn/appstock/app/fqkline/get?param={symbol},{period},,,{limit},{adjust}"
        );

        let body: serde_json::Value = self.get(&url).send().await?.json().await?;

        let data = body.get("data").and_then(|d| d.get(&*symbol));
        let klines_array = data
            .and_then(|d| d.get(period).or_else(|| d.get(format!("qfq{period}"))))
            .and_then(|k| k.as_array());

        let arr = match klines_array {
            Some(a) => a.clone(),
            None => return Ok(Vec::new()),
        };

        let mut klines = Vec::with_capacity(arr.len());
        for item in &arr {
            if let Some(a) = item.as_array()
                && a.len() >= 6
            {
                let day = a[0].as_str().unwrap_or("");
                let open = a[1].as_str().unwrap_or("0");
                let close = a[2].as_str().unwrap_or("0");
                let high = a[3].as_str().unwrap_or("0");
                let low = a[4].as_str().unwrap_or("0");
                let volume = a[5].as_str().unwrap_or("0");
                klines.push(format!(
                    "{day},{open},{close},{high},{low},{volume},0,0,0,0,0"
                ));
            }
        }

        Ok(klines)
    }

    /// Generic Eastmoney emweb financial report fetch.
    /// Used for per-stock financial statements (balance sheet, profit, cash flow).
    pub(crate) async fn emweb_financial_fetch(
        &self,
        code: &str,
        report_type: &str,
        date_type: &str,
    ) -> Result<Vec<serde_json::Value>> {
        // First get company type
        let url = "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/Index";
        let resp = self
            .get(url)
            .query(&[("type", "web"), ("code", &code.to_lowercase())])
            .send()
            .await
            .map_err(Error::from)?;
        let html = resp.text().await.map_err(Error::from)?;

        // Extract company type from HTML
        let company_type = if html.contains("hidctype") {
            // Try to extract from hidden input
            if let Some(start) = html.find(r#"id="hidctype""#) {
                if let Some(val_start) = html[start..].find("value=\"") {
                    let val_start = start + val_start + 7;
                    if let Some(val_end) = html[val_start..].find('"') {
                        html[val_start..val_start + val_end].to_string()
                    } else {
                        "4".to_string()
                    }
                } else {
                    "4".to_string()
                }
            } else {
                "4".to_string()
            }
        } else {
            "4".to_string()
        };

        // Get date list
        let date_url = format!(
            "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/{report_type}DateAjaxNew"
        );
        let code_lower = code.to_lowercase();
        let resp = crate::util::send_and_check(self.get(&date_url).query(&[
            ("companyType", company_type.as_str()),
            ("reportDateType", date_type),
            ("code", code_lower.as_str()),
        ]))
        .await?;

        let date_json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let dates = date_json
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        if dates.is_empty() {
            return Err(Error::not_found("no financial report dates available"));
        }

        // Fetch data in batches of 5 dates
        let mut all_data = Vec::new();
        let date_strs: Vec<String> = dates
            .iter()
            .filter_map(|d| {
                d.get("REPORT_DATE")
                    .and_then(|v| v.as_str())
                    .map(std::string::ToString::to_string)
            })
            .collect();

        for chunk in date_strs.chunks(5) {
            let dates_param = chunk.join(",");
            let data_url = format!(
                "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/{report_type}AjaxNew"
            );
            let resp = crate::util::send_and_check(self.get(&data_url).query(&[
                ("companyType", company_type.as_str()),
                ("reportDateType", date_type),
                ("reportType", "1"),
                ("dates", dates_param.as_str()),
                ("code", code_lower.as_str()),
            ]))
            .await?;

            let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
            if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                all_data.extend(data.clone());
            }
        }

        Ok(all_data)
    }

    // -- private helpers ----------------------------------------------------

    /// Shared implementation for individual-stock and sector capital flow.
    async fn fetch_capital_flow(&self, secid: &str, limit: usize) -> Result<Vec<CapitalFlowPoint>> {
        let lmt = limit.to_string();
        let response = crate::util::send_and_check(
            self.get("https://push2his.eastmoney.com/api/qt/stock/fflow/daykline/get")
                .query(&[
                    ("secid", secid),
                    ("lmt", lmt.as_str()),
                    ("fields1", "f1,f2,f3,f7"),
                    (
                        "fields2",
                        "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63",
                    ),
                ]),
        )
        .await?;

        let payload: KlineResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .data
            .ok_or_else(|| Error::upstream("eastmoney capital flow response missing data"))?;
        let klines = data
            .klines
            .ok_or_else(|| Error::upstream("eastmoney capital flow response missing klines"))?;

        let mut items: Vec<CapitalFlowPoint> = klines
            .iter()
            .map(|line| parse_capital_flow_line(line))
            .collect::<Result<Vec<_>>>()?;

        if items.is_empty() {
            return Err(Error::not_found("eastmoney returned no capital flow items"));
        }
        items.truncate(limit);
        Ok(items)
    }
}

// ---------------------------------------------------------------------------
// Parsing helpers (free functions, private)
// ---------------------------------------------------------------------------

/// Parse a single Eastmoney capital-flow CSV line into a `CapitalFlowPoint`.
///
/// Format: `date,main_net,small_net,medium_net,large_net,super_large_net,
///          main_ratio,small_ratio,medium_ratio,large_ratio,super_ratio,close,change_pct`
fn parse_capital_flow_line(line: &str) -> Result<CapitalFlowPoint> {
    let f = parse_csv_line(line);
    if f.len() < 13 {
        return Err(Error::decode(format!(
            "unexpected eastmoney capital flow format: {line}"
        )));
    }
    Ok(CapitalFlowPoint {
        trade_date: f[0].to_string(),
        main_net_inflow: parse_f64_safe(f[1]),
        small_net_inflow: parse_f64_safe(f[2]),
        medium_net_inflow: parse_f64_safe(f[3]),
        large_net_inflow: parse_f64_safe(f[4]),
        super_large_net_inflow: parse_f64_safe(f[5]),
        main_net_inflow_ratio_pct: parse_f64_safe(f[6]),
        small_net_inflow_ratio_pct: parse_f64_safe(f[7]),
        medium_net_inflow_ratio_pct: parse_f64_safe(f[8]),
        large_net_inflow_ratio_pct: parse_f64_safe(f[9]),
        super_large_net_inflow_ratio_pct: parse_f64_safe(f[10]),
        close: parse_f64_safe(f[11]),
        change_pct: parse_f64_safe(f[12]),
    })
}

/// Map Eastmoney search classify / security_type_name to a market label.
fn classify_search_market(
    classify: Option<&str>,
    security_type_name: Option<&str>,
) -> Option<&'static str> {
    match (classify, security_type_name) {
        (Some("AStock" | "Fund" | "OTCFUND" | "BStock" | "NEEQ"), _) | (_, Some("基金")) => {
            Some("A股")
        }
        (Some("Index"), _) => Some("指数"),
        (Some("UsStock"), _) => Some("美股"),
        (Some("HK"), _) => Some("港股"),
        _ => None,
    }
}

/// Strip exchange suffixes like ".SH" or ".SZ" from a symbol.
fn strip_exchange_suffix(symbol: &str) -> String {
    let trimmed = symbol.trim();
    trimmed
        .strip_suffix(".SH")
        .or_else(|| trimmed.strip_suffix(".sh"))
        .or_else(|| trimmed.strip_suffix(".SZ"))
        .or_else(|| trimmed.strip_suffix(".sz"))
        .unwrap_or(trimmed)
        .to_string()
}
