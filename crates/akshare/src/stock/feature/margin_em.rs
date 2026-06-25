//! Margin trading (融资融券) from Eastmoney, SSE, SZSE, PA.

use super::helpers::{json_f64, json_f64_opt, json_i64_opt, json_str};
use super::types::{
    MarginAccountInfo, MarginRatioPa, MarginSseDetail, MarginSseSummary, MarginSzseDetail,
    MarginSzseSummary, MarginUnderlyingInfoSzse,
};
use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::fmt_date;

impl AkShareClient {
    /// 融资融券账户统计
    pub async fn stock_margin_account_info_em(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<MarginAccountInfo>> {
        let sd = fmt_date(start_date);
        let ed = fmt_date(end_date);
        let filter = format!("(TRADE_DATE>='{sd}')(TRADE_DATE<='{ed}')");
        let data = self
            .dc_fetch_all(
                "RPTA_WEB_RZRQ_GK",
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
            .map(|v| MarginAccountInfo {
                date: json_str(v, "TRADE_DATE"),
                fin_balance: json_f64(v, "RZYE"),
                loan_balance: json_f64(v, "RQYE"),
                fin_buy_amount: json_f64(v, "RZMRE"),
                loan_sell_amount: json_f64(v, "RQMCL"),
                security_org_count: json_i64_opt(v, "RZRQ_JG_NUM"),
                dept_count: json_i64_opt(v, "RZRQ_JGJG_NUM"),
                personal_investor_count: json_i64_opt(v, "RZRQ_GR_NUM"),
                org_investor_count: json_i64_opt(v, "RZRQ_JG_NUM2"),
                active_investor_count: json_i64_opt(v, "RZRQ_ACTIVE_NUM"),
                margin_liability_investor_count: json_i64_opt(v, "RZRQ_DEBT_NUM"),
                total_guarantee: json_f64_opt(v, "RZRQ_GUA_TOTAL"),
                avg_guarantee_ratio: json_f64_opt(v, "RZRQ_GUA_RATIO"),
            })
            .collect())
    }

    /// 融资融券-账户统计信息（东方财富）
    pub async fn stock_margin_account_info(&self) -> Result<Vec<MarginAccountInfo>> {
        let data = self
            .dc_fetch_all(
                "RPTA_WEB_MARGIN_DAILYTRADE",
                "ALL",
                "",
                "STATISTICS_DATE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| MarginAccountInfo {
                date: json_str(v, "STATISTICS_DATE"),
                fin_balance: json_f64(v, "FIN_BALANCE"),
                loan_balance: json_f64(v, "LOAN_BALANCE"),
                fin_buy_amount: json_f64(v, "FIN_BUY_AMT"),
                loan_sell_amount: json_f64(v, "LOAN_SELL_AMT"),
                security_org_count: json_i64_opt(v, "SECURITY_ORG_NUM"),
                dept_count: json_i64_opt(v, "OPERATEDEPT_NUM"),
                personal_investor_count: json_i64_opt(v, "PERSONAL_INVESTOR_NUM"),
                org_investor_count: json_i64_opt(v, "ORG_INVESTOR_NUM"),
                active_investor_count: json_i64_opt(v, "INVESTOR_NUM"),
                margin_liability_investor_count: json_i64_opt(v, "MARGINLIAB_INVESTOR_NUM"),
                total_guarantee: json_f64_opt(v, "TOTAL_GUARANTEE"),
                avg_guarantee_ratio: json_f64_opt(v, "AVG_GUARANTEE_RATIO"),
            })
            .collect())
    }

    /// 上海证券交易所-融资融券明细
    pub async fn stock_margin_detail_sse(&self, date: &str) -> Result<Vec<MarginSseDetail>> {
        let url = "https://query.sse.com.cn/marketdata/tradedata/queryMargin.do";
        let resp = self.get(url)
            .query(&[
                ("isPagination", "true"),
                ("tabType", "mxtype"),
                ("detailsDate", date),
                ("stockCode", ""),
                ("beginDate", ""),
                ("endDate", ""),
                ("pageHelp.pageSize", "5000"),
                ("pageHelp.pageCount", "50"),
                ("pageHelp.pageNo", "1"),
                ("pageHelp.beginPage", "1"),
                ("pageHelp.cacheSize", "1"),
                ("pageHelp.endPage", "21"),
            ])
            .header("Referer", "https://www.sse.com.cn/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.150 Safari/537.36");
        let resp = crate::util::send_and_check(resp).await?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json
            .get("result")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();
        // SSE result is an array of arrays
        Ok(arr
            .iter()
            .filter_map(|row| {
                let cols = row.as_array()?;
                if cols.len() < 13 {
                    return None;
                }
                Some(MarginSseDetail {
                    trade_date: cols[1].as_str().unwrap_or("").to_string(),
                    code: cols[12].as_str().unwrap_or("").to_string(),
                    name: cols[11].as_str().unwrap_or("").to_string(),
                    fin_balance: cols[10].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    fin_buy_amount: cols[8].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    fin_repay_amount: cols[7].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    loan_volume: cols[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    loan_sell_amount: cols[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    loan_repay_amount: cols[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                })
            })
            .collect())
    }

    /// 深圳证券交易所-融资融券明细
    pub async fn stock_margin_detail_szse(&self, date: &str) -> Result<Vec<MarginSzseDetail>> {
        let url = "https://www.szse.cn/api/report/ShowReport";
        let formatted_date = fmt_date(date);
        let resp = self.get(url)
            .query(&[
                ("SHOWTYPE", "JSON"),
                ("CATALOGID", "1837_xxpl"),
                ("txtDate", formatted_date.as_str()),
                ("tab2PAGENO", "1"),
                ("random", "0.24279342734085696"),
                ("TABKEY", "tab2"),
            ])
            .header("Referer", "https://www.szse.cn/disclosure/margin/margin/index.html")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.150 Safari/537.36");
        let resp = crate::util::send_and_check(resp).await?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json
            .as_array()
            .and_then(|a| a.first())
            .and_then(|v| v.get("data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(arr
            .iter()
            .filter_map(|row| {
                let cols = row.get("data").and_then(|d| d.as_array())?;
                if cols.len() < 8 {
                    return None;
                }
                let clean = |s: &str| s.replace(',', "").replace("&nbsp;", "").trim().to_string();
                Some(MarginSzseDetail {
                    code: cols[0].as_str().unwrap_or("").to_string(),
                    name: clean(cols[1].as_str().unwrap_or("")),
                    fin_buy_amount: clean(cols[2].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                    fin_balance: clean(cols[3].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                    loan_sell_amount: clean(cols[4].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                    loan_volume: clean(cols[5].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                    loan_balance: clean(cols[6].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                    margin_balance: clean(cols[7].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                })
            })
            .collect())
    }

    /// 平安-融资融券保证金比例
    pub async fn stock_margin_ratio_pa(
        &self,
        symbol: &str,
        date: &str,
    ) -> Result<Vec<MarginRatioPa>> {
        let market_code = match symbol {
            "深市" => "00",
            "北交所" => "30",
            _ => "10",
        };
        let url = "https://stock.pingan.com/fss/servlet/fsscoreapp/stockSource/mrgRatio";
        let formatted = fmt_date(date);
        let parts: Vec<&str> = formatted.split('-').collect();
        let setdate = if parts.len() == 3 {
            format!("{}-{}-{}", parts[0], parts[1], parts[2])
        } else {
            date.to_string()
        };
        let resp = self
            .post(url)
            .json(&serde_json::json!({
                "currentPage": 1,
                "pageSize": 50000,
                "type": "bdzq",
                "setdate": setdate,
                "stockMes": "",
                "market": market_code,
                "appName": "AYLCH5",
                "tokenId": "",
                "appChannel": "LRSP",
                "requestId": "194055910e2075c03e25fabf6ffc5a7f",
                "channel": "pa18",
            }))
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json
            .get("data")
            .and_then(|d| d.get("list"))
            .and_then(|l| l.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(arr
            .iter()
            .map(|v| MarginRatioPa {
                code: json_str(v, "secuCode"),
                name: json_str(v, "secuName"),
                fin_ratio: json_f64(v, "fiMarginRatio"),
                loan_ratio: json_f64(v, "slMarginRatio"),
            })
            .collect())
    }

    /// 上海证券交易所-融资融券汇总
    pub async fn stock_margin_sse(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<MarginSseSummary>> {
        let url = "https://query.sse.com.cn/marketdata/tradedata/queryMargin.do";
        let resp = self.get(url)
            .query(&[
                ("isPagination", "true"),
                ("beginDate", start_date),
                ("endDate", end_date),
                ("tabType", ""),
                ("stockCode", ""),
                ("pageHelp.pageSize", "5000"),
                ("pageHelp.pageNo", "1"),
                ("pageHelp.beginPage", "1"),
                ("pageHelp.cacheSize", "1"),
                ("pageHelp.endPage", "5"),
            ])
            .header("Referer", "https://www.sse.com.cn/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.150 Safari/537.36");
        let resp = crate::util::send_and_check(resp).await?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json
            .get("result")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(arr
            .iter()
            .filter_map(|row| {
                let cols = row.as_array()?;
                if cols.len() < 13 {
                    return None;
                }
                Some(MarginSseSummary {
                    trade_date: cols[1].as_str().unwrap_or("").to_string(),
                    fin_balance: cols[10].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    fin_buy_amount: cols[8].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    loan_volume: cols[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    loan_balance: cols[5].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    loan_sell_amount: cols[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    margin_balance: cols[9].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                })
            })
            .collect())
    }

    /// 深圳证券交易所-融资融券汇总
    pub async fn stock_margin_szse(&self, date: &str) -> Result<Vec<MarginSzseSummary>> {
        let url = "https://www.szse.cn/api/report/ShowReport/data";
        let formatted_date = fmt_date(date);
        let resp = self.get(url)
            .query(&[
                ("SHOWTYPE", "JSON"),
                ("CATALOGID", "1837_xxpl"),
                ("txtDate", formatted_date.as_str()),
                ("tab1PAGENO", "1"),
                ("random", "0.7425245522795993"),
            ])
            .header("Referer", "https://www.szse.cn/disclosure/margin/object/index.html")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.150 Safari/537.36");
        let resp = crate::util::send_and_check(resp).await?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json
            .as_array()
            .and_then(|a| a.first())
            .and_then(|v| v.get("data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(arr
            .iter()
            .filter_map(|row| {
                let cols = row.get("data").and_then(|d| d.as_array())?;
                if cols.len() < 6 {
                    return None;
                }
                let clean = |s: &str| s.replace(',', "").trim().to_string();
                Some(MarginSzseSummary {
                    fin_buy_amount: clean(cols[0].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                    fin_balance: clean(cols[1].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                    loan_sell_amount: clean(cols[2].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                    loan_volume: clean(cols[3].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                    loan_balance: clean(cols[4].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                    margin_balance: clean(cols[5].as_str().unwrap_or("0"))
                        .parse()
                        .unwrap_or(0.0),
                })
            })
            .collect())
    }

    /// 深圳证券交易所-标的证券信息
    pub async fn stock_margin_underlying_info(
        &self,
        date: &str,
    ) -> Result<Vec<MarginUnderlyingInfoSzse>> {
        let url = "https://www.szse.cn/api/report/ShowReport";
        let formatted_date = fmt_date(date);
        let resp = self.get(url)
            .query(&[
                ("SHOWTYPE", "JSON"),
                ("CATALOGID", "1834_xxpl"),
                ("txtDate", formatted_date.as_str()),
                ("tab1PAGENO", "1"),
                ("random", "0.7425245522795993"),
                ("TABKEY", "tab1"),
            ])
            .header("Referer", "https://www.szse.cn/disclosure/margin/object/index.html")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.150 Safari/537.36");
        let resp = crate::util::send_and_check(resp).await?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json
            .as_array()
            .and_then(|a| a.first())
            .and_then(|v| v.get("data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(arr
            .iter()
            .filter_map(|row| {
                let cols = row.get("data").and_then(|d| d.as_array())?;
                if cols.len() < 2 {
                    return None;
                }
                Some(MarginUnderlyingInfoSzse {
                    code: cols[0].as_str().unwrap_or("").to_string(),
                    name: cols[1].as_str().unwrap_or("").to_string(),
                    data: row.clone(),
                })
            })
            .collect())
    }
}
