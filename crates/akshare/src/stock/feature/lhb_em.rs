//! Billboard (龙虎榜) data from Eastmoney.

use super::helpers::{json_f64, json_f64_opt, json_i64, json_str, json_str_opt};
use super::types::{
    BillboardActiveBranch, BillboardBranchDetail, BillboardBranchRanking, BillboardDetail,
    BillboardOrgSeatTracking, BillboardOrgTradeSummary, BillboardSinaBranchSummary,
    BillboardSinaDetail, BillboardSinaHkSummary, BillboardSinaOrgDetail, BillboardSinaOrgTracking,
    BillboardStockDetail, BillboardStockDetailDate, BillboardStockStatistic,
    BillboardTraderStatistic,
};
use std::sync::LazyLock;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::{
    decode_text, fmt_date, parse_f64_safe, parse_html_tables, parse_i64_safe, zero_pad_code,
};

fn fmt_date_range(start: &str, end: &str) -> (String, String) {
    (fmt_date(start), fmt_date(end))
}

static RE_PAGE_ELEMENT: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"(?is)<[^>]*class="[^"]*\bpage\b[^"]*"[^>]*>([^<]*)"#).unwrap()
});

const SINA_LHB_DAILY: &str =
    "https://vip.stock.finance.sina.com.cn/q/go.php/vInvestConsult/kind/lhb/index.phtml";
const SINA_LHB_GGTJ: &str =
    "https://vip.stock.finance.sina.com.cn/q/go.php/vLHBData/kind/ggtj/index.phtml";
const SINA_LHB_JGMX: &str =
    "https://vip.stock.finance.sina.com.cn/q/go.php/vLHBData/kind/jgmx/index.phtml";
const SINA_LHB_JGZZ: &str =
    "https://vip.stock.finance.sina.com.cn/q/go.php/vLHBData/kind/jgzz/index.phtml";
const SINA_LHB_YYTJ: &str =
    "https://vip.stock.finance.sina.com.cn/q/go.php/vLHBData/kind/yytj/index.phtml";

/// Fetch a Sina data page as text.
///
/// Sina declares `charset=gbk` in the response header but currently serves
/// UTF-8 bytes. Decode with the raw bytes so valid UTF-8 wins and GB18030 is
/// only used as a genuine fallback.
async fn sina_html(client: &AkShareClient, url: &str, params: &[(&str, String)]) -> Result<String> {
    let bytes = client
        .get(url)
        .query(params)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
        )
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    Ok(decode_text(&bytes, None, Some(encoding_rs::GB18030)))
}

/// Last page number advertised by Sina's `class="page"` pagination.
///
/// Upstream reads the second-to-last page element (the final one is usually a
/// next-page arrow with no numeric text).
fn sina_last_page(html: &str) -> usize {
    let pages: Vec<Option<i64>> = RE_PAGE_ELEMENT
        .captures_iter(html)
        .map(|caps| caps[1].trim().parse::<i64>().ok())
        .collect();
    let value = if pages.len() >= 2 {
        pages[pages.len() - 2]
    } else {
        pages.first().copied().flatten()
    };
    value.unwrap_or(1).clamp(1, 200) as usize
}

/// Return the first table that has a header containing the given label.
fn find_table<'a>(
    tables: &'a [Vec<Vec<String>>],
    header_label: &str,
) -> Option<&'a Vec<Vec<String>>> {
    tables.iter().find(|table| {
        table
            .iter()
            .any(|row| row.iter().any(|c| c.contains(header_label)))
    })
}

fn header_index(table: &[Vec<String>], label: &str) -> Option<usize> {
    table
        .iter()
        .position(|row| row.iter().any(|c| c.contains(label)))
}

impl AkShareClient {
    /// 龙虎榜详情
    pub async fn stock_lhb_detail(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<BillboardDetail>> {
        let (sd, ed) = fmt_date_range(start_date, end_date);
        let filter = format!("(TRADE_DATE<='{ed}')(TRADE_DATE>='{sd}')");
        let data = self
            .dc_fetch_all(
                "RPT_DAILYBILLBOARD_DETAILSNEW",
                "ALL",
                &filter,
                "SECURITY_CODE,TRADE_DATE",
                "1,-1",
                500,
                10,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardDetail {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                trade_date: json_str(v, "TRADE_DATE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                billboard_net_amount: json_f64(v, "BILLBOARD_NET_AMT"),
                billboard_buy_amount: json_f64(v, "BILLBOARD_BUY_AMT"),
                billboard_sell_amount: json_f64(v, "BILLBOARD_SELL_AMT"),
                billboard_deal_amount: json_f64(v, "BILLBOARD_DEAL_AMT"),
                total_amount: json_f64(v, "ACCUM_AMOUNT"),
                net_ratio: json_f64(v, "DEAL_NET_RATIO"),
                deal_ratio: json_f64(v, "DEAL_AMOUNT_RATIO"),
                turnover_rate: json_f64(v, "TURNOVERRATE"),
                circulating_market_cap: json_f64(v, "FREE_MARKET_CAP"),
                reason: json_str(v, "EXPLANATION"),
                explanation: json_str_opt(v, "EXPLAIN"),
                d1_change: json_f64_opt(v, "D1_CLOSE_ADJCHRATE"),
                d2_change: json_f64_opt(v, "D2_CLOSE_ADJCHRATE"),
                d5_change: json_f64_opt(v, "D5_CLOSE_ADJCHRATE"),
                d10_change: json_f64_opt(v, "D10_CLOSE_ADJCHRATE"),
            })
            .collect())
    }

    /// 个股上榜统计
    pub async fn stock_lhb_stock_statistic(
        &self,
        symbol: &str,
    ) -> Result<Vec<BillboardStockStatistic>> {
        let cycle = match symbol {
            "近三月" => "02",
            "近六月" => "03",
            "近一年" => "04",
            _ => "01",
        };
        let filter = format!("(STATISTICS_CYCLE=\"{cycle}\")");
        let data = self
            .dc_fetch_all(
                "RPT_BILLBOARD_TRADEALL",
                "ALL",
                &filter,
                "BILLBOARD_TIMES,LATEST_TDATE,SECURITY_CODE",
                "-1,-1,1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardStockStatistic {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                latest_trade_date: json_str(v, "LATEST_TDATE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                billboard_times: json_i64(v, "BILLBOARD_TIMES"),
                billboard_net_amount: json_f64(v, "BILLBOARD_NET_AMT"),
                billboard_buy_amount: json_f64(v, "BILLBOARD_BUY_AMT"),
                billboard_sell_amount: json_f64(v, "BILLBOARD_SELL_AMT"),
                billboard_total_amount: json_f64(v, "AMOUNT"),
                org_buy_times: json_i64(v, "BUY_TIMES"),
                org_sell_times: json_i64(v, "SELL_TIMES"),
                org_net_buy_amount: json_f64(v, "ORG_BUY_NET_AMT"),
                org_buy_amount: json_f64(v, "ORG_BUY_AMT"),
                org_sell_amount: json_f64(v, "ORG_SELL_AMT"),
                m1_change: json_f64_opt(v, "M1_CLOSE_ADJCHRATE"),
                m3_change: json_f64_opt(v, "M3_CLOSE_ADJCHRATE"),
                m6_change: json_f64_opt(v, "M6_CLOSE_ADJCHRATE"),
                y1_change: json_f64_opt(v, "Y1_CLOSE_ADJCHRATE"),
            })
            .collect())
    }

    /// 机构买卖每日统计
    pub async fn stock_lhb_jgmmtj(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<BillboardOrgTradeSummary>> {
        let (sd, ed) = fmt_date_range(start_date, end_date);
        let filter = format!("(TRADE_DATE>='{sd}')(TRADE_DATE<='{ed}')");
        let data = self
            .dc_fetch_all(
                "RPT_ORGANIZATION_TRADE_DETAILS",
                "ALL",
                &filter,
                "NET_BUY_AMT,TRADE_DATE,SECURITY_CODE",
                "-1,-1,1",
                500,
                10,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardOrgTradeSummary {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                trade_date: json_str(v, "TRADE_DATE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                buy_org_count: json_i64(v, "BUY_ORG_NUM"),
                sell_org_count: json_i64(v, "SELL_ORG_NUM"),
                org_buy_amount: json_f64(v, "ORG_BUY_AMT"),
                org_sell_amount: json_f64(v, "ORG_SELL_AMT"),
                org_net_buy_amount: json_f64(v, "NET_BUY_AMT"),
                total_amount: json_f64(v, "ACCUM_AMOUNT"),
                org_net_ratio: json_f64(v, "NET_BUY_RATIO"),
                turnover_rate: json_f64(v, "TURNOVERRATE"),
                circulating_market_cap: json_f64(v, "FREE_MARKET_CAP"),
                reason: json_str(v, "EXPLANATION"),
            })
            .collect())
    }

    /// 机构席位追踪
    pub async fn stock_lhb_jgstatistic(
        &self,
        symbol: &str,
    ) -> Result<Vec<BillboardOrgSeatTracking>> {
        let cycle = match symbol {
            "近三月" => "02",
            "近六月" => "03",
            "近一年" => "04",
            _ => "01",
        };
        let filter = format!("(STATISTICSCYCLE=\"{cycle}\")");
        let data = self
            .dc_fetch_all(
                "RPT_ORGANIZATION_SEATNEW",
                "ALL",
                &filter,
                "ONLIST_TIMES,SECURITY_CODE",
                "-1,1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardOrgSeatTracking {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                billboard_amount: json_f64(v, "AMOUNT"),
                billboard_times: json_i64(v, "ONLIST_TIMES"),
                org_buy_amount: json_f64(v, "BUY_AMT"),
                org_buy_times: json_i64(v, "BUY_TIMES"),
                org_sell_amount: json_f64(v, "SELL_AMT"),
                org_sell_times: json_i64(v, "SELL_TIMES"),
                org_net_buy_amount: json_f64(v, "NET_BUY_AMT"),
                m1_change: json_f64_opt(v, "M1_CLOSE_ADJCHRATE"),
                m3_change: json_f64_opt(v, "M3_CLOSE_ADJCHRATE"),
                m6_change: json_f64_opt(v, "M6_CLOSE_ADJCHRATE"),
                y1_change: json_f64_opt(v, "Y1_CLOSE_ADJCHRATE"),
            })
            .collect())
    }

    /// 每日活跃营业部
    pub async fn stock_lhb_hyyyb(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<BillboardActiveBranch>> {
        let (sd, ed) = fmt_date_range(start_date, end_date);
        let filter = format!("(ONLIST_DATE>='{sd}')(ONLIST_DATE<='{ed}')");
        let data = self
            .dc_fetch_all(
                "RPT_OPERATEDEPT_ACTIVE",
                "ALL",
                &filter,
                "TOTAL_NETAMT,ONLIST_DATE,OPERATEDEPT_CODE",
                "-1,-1,1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardActiveBranch {
                dept_name: json_str(v, "OPERATEDEPT_NAME"),
                trade_date: json_str(v, "ONLIST_DATE"),
                buy_stock_count: json_i64(v, "BUY_NUM"),
                sell_stock_count: json_i64(v, "SELL_NUM"),
                buy_amount: json_f64(v, "BUY_AMT"),
                sell_amount: json_f64(v, "SELL_AMT"),
                net_amount: json_f64(v, "TOTAL_NETAMT"),
                buy_stocks: json_str_opt(v, "BUY_SECURITIES"),
                dept_code: json_str(v, "OPERATEDEPT_CODE"),
            })
            .collect())
    }

    /// 营业部排行
    pub async fn stock_lhb_yybph(&self, symbol: &str) -> Result<Vec<BillboardBranchRanking>> {
        let cycle = match symbol {
            "近三月" => "02",
            "近六月" => "03",
            "近一年" => "04",
            _ => "01",
        };
        let filter = format!("(STATISTICSCYCLE=\"{cycle}\")");
        let data = self
            .dc_fetch_all(
                "RPT_OPERATEDEPT_RANK",
                "ALL",
                &filter,
                "ONLIST_TIMES,OPERATEDEPT_CODE",
                "-1,1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardBranchRanking {
                dept_name: json_str(v, "OPERATEDEPT_NAME"),
                dept_code: json_str(v, "OPERATEDEPT_CODE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                billboard_amount: json_f64(v, "AMOUNT"),
                billboard_times: json_i64(v, "ONLIST_TIMES"),
                buy_amount: json_f64(v, "BUY_AMT"),
                buy_times: json_i64(v, "BUY_TIMES"),
                sell_amount: json_f64(v, "SELL_AMT"),
                sell_times: json_i64(v, "SELL_TIMES"),
                net_buy_amount: json_f64(v, "NET_BUY_AMT"),
                m1_change: json_f64_opt(v, "M1_CLOSE_ADJCHRATE"),
                m3_change: json_f64_opt(v, "M3_CLOSE_ADJCHRATE"),
                m6_change: json_f64_opt(v, "M6_CLOSE_ADJCHRATE"),
                y1_change: json_f64_opt(v, "Y1_CLOSE_ADJCHRATE"),
            })
            .collect())
    }

    /// 游资统计
    pub async fn stock_lhb_traderstatistic(
        &self,
        symbol: &str,
    ) -> Result<Vec<BillboardTraderStatistic>> {
        let cycle = match symbol {
            "近三月" => "02",
            "近六月" => "03",
            "近一年" => "04",
            _ => "01",
        };
        let filter = format!("(STATISTICSCYCLE=\"{cycle}\")");
        let data = self
            .dc_fetch_all(
                "RPT_BILLBOARD_DAILYDETAILS_TRADER",
                "ALL",
                &filter,
                "ONLIST_TIMES,OPERATEDEPT_CODE",
                "-1,1",
                5000,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardTraderStatistic {
                trader_name: json_str(v, "OPERATEDEPT_NAME"),
                trader_code: json_str(v, "OPERATEDEPT_CODE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                billboard_amount: json_f64(v, "AMOUNT"),
                billboard_times: json_i64(v, "ONLIST_TIMES"),
                buy_amount: json_f64(v, "BUY_AMT"),
                buy_times: json_i64(v, "BUY_TIMES"),
                sell_amount: json_f64(v, "SELL_AMT"),
                sell_times: json_i64(v, "SELL_TIMES"),
                net_buy_amount: json_f64(v, "NET_BUY_AMT"),
                m1_change: json_f64_opt(v, "M1_CLOSE_ADJCHRATE"),
                m3_change: json_f64_opt(v, "M3_CLOSE_ADJCHRATE"),
                m6_change: json_f64_opt(v, "M6_CLOSE_ADJCHRATE"),
                y1_change: json_f64_opt(v, "Y1_CLOSE_ADJCHRATE"),
            })
            .collect())
    }

    /// 个股上榜日期
    pub async fn stock_lhb_stock_detail_date(
        &self,
        symbol: &str,
    ) -> Result<Vec<BillboardStockDetailDate>> {
        let filter = format!("(SECURITY_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_DAILYBILLBOARD_DETAILSNEW",
                "ALL",
                &filter,
                "TRADE_DATE",
                "-1",
                500,
                1,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardStockDetailDate {
                trade_date: json_str(v, "TRADE_DATE"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                reason: json_str(v, "EXPLANATION"),
                billboard_net_amount: json_f64(v, "BILLBOARD_NET_AMT"),
                billboard_buy_amount: json_f64(v, "BILLBOARD_BUY_AMT"),
                billboard_sell_amount: json_f64(v, "BILLBOARD_SELL_AMT"),
            })
            .collect())
    }

    /// 个股龙虎榜详情
    pub async fn stock_lhb_stock_detail(
        &self,
        symbol: &str,
        date: &str,
        flag: &str,
    ) -> Result<Vec<BillboardStockDetail>> {
        let report = match flag {
            "卖出" => "RPT_BILLBOARD_DAILYDETAILSSELL",
            _ => "RPT_BILLBOARD_DAILYDETAILSBUY",
        };
        let filter = format!(
            "(TRADE_DATE='{}')(SECURITY_CODE=\"{symbol}\")",
            fmt_date(date)
        );
        let data = self
            .dc_fetch_all(report, "ALL", &filter, "TRADE_DATE", "-1", 500, 1, &[])
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardStockDetail {
                trade_date: json_str(v, "TRADE_DATE"),
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                dept_name: json_str(v, "OPERATEDEPT_NAME"),
                buy_amount: json_f64(v, "BUY"),
                sell_amount: json_f64(v, "SELL"),
                net_amount: json_f64(v, "NET"),
                reason: json_str(v, "EXPLANATION"),
                side: flag.to_string(),
            })
            .collect())
    }

    /// 营业部龙虎榜详情
    pub async fn stock_lhb_yyb_detail(&self, symbol: &str) -> Result<Vec<BillboardBranchDetail>> {
        let filter = format!("(OPERATEDEPT_CODE=\"{symbol}\")");
        let data = self
            .dc_fetch_all(
                "RPT_BILLBOARD_DAILYDETAILSBUY",
                "ALL",
                &filter,
                "TRADE_DATE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;

        Ok(data
            .iter()
            .map(|v| BillboardBranchDetail {
                trade_date: json_str(v, "TRADE_DATE"),
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                close_price: json_f64(v, "CLOSE_PRICE"),
                change_pct: json_f64(v, "CHANGE_RATE"),
                buy_amount: json_f64(v, "BUY"),
                sell_amount: json_f64(v, "SELL"),
                net_amount: json_f64(v, "NET"),
                reason: json_str(v, "EXPLANATION"),
            })
            .collect())
    }

    /// 新浪-龙虎榜-每日详情
    pub async fn stock_lhb_detail_daily(&self, date: &str) -> Result<Vec<BillboardSinaDetail>> {
        let date = fmt_date(&date.replace('-', ""));
        let html = sina_html(self, SINA_LHB_DAILY, &[("tradedate", date.clone())]).await?;
        let tables = parse_html_tables(&html);
        let mut items = Vec::new();
        for table in &tables {
            let Some(header_idx) = header_index(table, "股票代码") else {
                continue;
            };
            let indicator = table
                .first()
                .and_then(|row| row.iter().find(|c| !c.trim().is_empty()))
                .map(|c| c.trim().to_string())
                .unwrap_or_default();
            for row in table.iter().skip(header_idx + 1) {
                if row.len() < 8 {
                    continue;
                }
                let rank = parse_i64_safe(&row[0]);
                let code = zero_pad_code(&row[1]);
                if rank == 0 || code.len() != 6 {
                    continue;
                }
                items.push(BillboardSinaDetail {
                    rank,
                    code,
                    name: row[2].trim().to_string(),
                    close_price: parse_f64_safe(&row[3]),
                    metric_value: parse_f64_safe(&row[4]),
                    volume: parse_f64_safe(&row[5]),
                    amount: parse_f64_safe(&row[6]),
                    indicator: indicator.clone(),
                });
            }
        }
        if items.is_empty() {
            return Err(Error::not_found(format!(
                "Sina billboard daily returned no rows for {date}"
            )));
        }
        Ok(items)
    }

    /// 新浪-龙虎榜-个股上榜统计
    pub async fn stock_lhb_ggtj(&self) -> Result<Vec<BillboardSinaHkSummary>> {
        // Upstream default symbol is "5" (recent 5 days).
        let last = "5";
        let first = sina_html(
            self,
            SINA_LHB_GGTJ,
            &[("last", last.to_string()), ("p", "1".to_string())],
        )
        .await?;
        let last_page = sina_last_page(&first);
        let mut items = Vec::new();
        for page in 1..=last_page {
            let html = if page == 1 {
                first.clone()
            } else {
                sina_html(
                    self,
                    SINA_LHB_GGTJ,
                    &[("last", last.to_string()), ("p", page.to_string())],
                )
                .await?
            };
            let tables = parse_html_tables(&html);
            let Some(table) = find_table(&tables, "股票代码") else {
                continue;
            };
            let Some(header_idx) = header_index(table, "股票代码") else {
                continue;
            };
            for row in table.iter().skip(header_idx + 1) {
                if row.len() < 8 {
                    continue;
                }
                let code = zero_pad_code(&row[0]);
                if code.len() != 6 {
                    continue;
                }
                items.push(BillboardSinaHkSummary {
                    code,
                    name: row[1].trim().to_string(),
                    billboard_count: parse_i64_safe(&row[2]),
                    accum_buy: parse_f64_safe(&row[3]),
                    accum_sell: parse_f64_safe(&row[4]),
                    net_amount: parse_f64_safe(&row[5]),
                    buy_seats: parse_i64_safe(&row[6]),
                    sell_seats: parse_i64_safe(&row[7]),
                });
            }
        }
        if items.is_empty() {
            return Err(Error::not_found(
                "Sina billboard stock stats returned no rows",
            ));
        }
        Ok(items)
    }

    /// 新浪-龙虎榜-机构席位成交明细
    pub async fn stock_lhb_jgmx(&self) -> Result<Vec<BillboardSinaOrgDetail>> {
        let first = sina_html(self, SINA_LHB_JGMX, &[("p", "1".to_string())]).await?;
        let last_page = sina_last_page(&first);
        let mut items = Vec::new();
        for page in 1..=last_page {
            let html = if page == 1 {
                first.clone()
            } else {
                sina_html(self, SINA_LHB_JGMX, &[("p", page.to_string())]).await?
            };
            let tables = parse_html_tables(&html);
            let Some(table) = find_table(&tables, "股票代码") else {
                continue;
            };
            let Some(header_idx) = header_index(table, "股票代码") else {
                continue;
            };
            for row in table.iter().skip(header_idx + 1) {
                if row.len() < 5 {
                    continue;
                }
                let code = zero_pad_code(&row[0]);
                if code.len() != 6 {
                    continue;
                }
                items.push(BillboardSinaOrgDetail {
                    code,
                    name: row[1].trim().to_string(),
                    trade_date: row[2].trim().to_string(),
                    inst_buy_amount: parse_f64_safe(&row[3]),
                    inst_sell_amount: parse_f64_safe(&row[4]),
                });
            }
        }
        if items.is_empty() {
            return Err(Error::not_found(
                "Sina billboard institution detail returned no rows",
            ));
        }
        Ok(items)
    }

    /// 新浪-龙虎榜-机构席位追踪
    pub async fn stock_lhb_jgzz(&self, symbol: &str) -> Result<Vec<BillboardSinaOrgTracking>> {
        let last = if symbol.trim().is_empty() {
            "5"
        } else {
            symbol
        };
        let first = sina_html(
            self,
            SINA_LHB_JGZZ,
            &[("last", last.to_string()), ("p", "1".to_string())],
        )
        .await?;
        let last_page = sina_last_page(&first);
        let mut items = Vec::new();
        for page in 1..=last_page {
            let html = if page == 1 {
                first.clone()
            } else {
                sina_html(
                    self,
                    SINA_LHB_JGZZ,
                    &[("last", last.to_string()), ("p", page.to_string())],
                )
                .await?
            };
            let tables = parse_html_tables(&html);
            let Some(table) = find_table(&tables, "股票代码") else {
                continue;
            };
            let Some(header_idx) = header_index(table, "股票代码") else {
                continue;
            };
            for row in table.iter().skip(header_idx + 1) {
                // 股票代码, 股票名称, 当前价, 涨跌幅, 累积买入额, 买入次数,
                // 累积卖出额, 卖出次数, 净额
                if row.len() < 9 {
                    continue;
                }
                let code = zero_pad_code(&row[0]);
                if code.len() != 6 {
                    continue;
                }
                items.push(BillboardSinaOrgTracking {
                    code,
                    name: row[1].trim().to_string(),
                    accum_buy: parse_f64_safe(&row[4]),
                    buy_count: parse_i64_safe(&row[5]),
                    accum_sell: parse_f64_safe(&row[6]),
                    sell_count: parse_i64_safe(&row[7]),
                    net_amount: parse_f64_safe(&row[8]),
                });
            }
        }
        if items.is_empty() {
            return Err(Error::not_found(
                "Sina billboard institution tracking returned no rows",
            ));
        }
        Ok(items)
    }

    /// 新浪-龙虎榜-营业部上榜统计
    pub async fn stock_lhb_yytj(&self, symbol: &str) -> Result<Vec<BillboardSinaBranchSummary>> {
        let last = if symbol.trim().is_empty() {
            "5"
        } else {
            symbol
        };
        let first = sina_html(
            self,
            SINA_LHB_YYTJ,
            &[("last", last.to_string()), ("p", "1".to_string())],
        )
        .await?;
        let last_page = sina_last_page(&first);
        let mut items = Vec::new();
        for page in 1..=last_page {
            let html = if page == 1 {
                first.clone()
            } else {
                sina_html(
                    self,
                    SINA_LHB_YYTJ,
                    &[("last", last.to_string()), ("p", page.to_string())],
                )
                .await?
            };
            let tables = parse_html_tables(&html);
            let Some(table) = find_table(&tables, "营业部名称") else {
                continue;
            };
            let Some(header_idx) = header_index(table, "营业部名称") else {
                continue;
            };
            for row in table.iter().skip(header_idx + 1) {
                if row.len() < 7 || row[0].trim().is_empty() {
                    continue;
                }
                items.push(BillboardSinaBranchSummary {
                    dept_name: row[0].trim().to_string(),
                    billboard_count: parse_i64_safe(&row[1]),
                    accum_buy: parse_f64_safe(&row[2]),
                    buy_seats: parse_i64_safe(&row[3]),
                    accum_sell: parse_f64_safe(&row[4]),
                    sell_seats: parse_i64_safe(&row[5]),
                    top_stocks: Some(row[6].trim().to_string()).filter(|value| !value.is_empty()),
                });
            }
        }
        if items.is_empty() {
            return Err(Error::not_found(
                "Sina billboard branch stats returned no rows",
            ));
        }
        Ok(items)
    }
}
