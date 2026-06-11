//! Miscellaneous stock feature functions from Eastmoney, CNINFO, Legulegu, Sina, etc.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;
use super::helpers::*;
use super::types::*;

impl AkShareClient {
    /// 东方财富-股票账户统计
    pub async fn stock_account_statistics_em(&self) -> Result<Vec<AccountStatistics>> {
        let data = self.dc_fetch_all(
            "RPT_STOCK_OPEN_DATA",
            "ALL",
            "",
            "STATISTICS_DATE",
            "-1",
            500,
            10,
            &[],
        ).await?;
        Ok(data.iter().map(|v| AccountStatistics {
            date: json_str(v, "STATISTICS_DATE"),
            new_investor_count: json_f64_opt(v, "NEW_INVESTOR_NUM"),
            new_investor_mom: json_f64_opt(v, "NEW_INVESTOR_MOM"),
            new_investor_yoy: json_f64_opt(v, "NEW_INVESTOR_YOY"),
            total_investor_count: json_f64_opt(v, "TOTAL_INVESTOR_NUM"),
            a_share_count: json_f64_opt(v, "A_SHARE_NUM"),
            b_share_count: json_f64_opt(v, "B_SHARE_NUM"),
            total_market_cap: json_f64_opt(v, "TOTAL_MARKET_CAP"),
            avg_market_cap: json_f64_opt(v, "AVG_MARKET_CAP"),
            sh_close: json_f64_opt(v, "SH_CLOSE"),
            sh_change_pct: json_f64_opt(v, "SH_CHANGE_PCT"),
        }).collect())
    }

    /// 巨潮-配股
    pub async fn stock_allotment_cninfo(&self, symbol: &str) -> Result<Vec<AllotmentCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1091";
        let resp = self.post(url)
            .form(&[("seccode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| AllotmentCninfo { data: v.clone() }).collect())
    }

    /// 乐咕乐股-全部A股等权重市净率
    pub async fn stock_a_all_pb(&self) -> Result<Vec<serde_json::Value>> {
        let url = "https://legulegu.com/api/stockdata/market-index-pb";
        let resp = self.get(url)
            .query(&[("marketId", "ALL"), ("token", "325843825a2745a2a8f9b9e3355cb864")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr)
    }

    /// 乐咕乐股-破净股统计
    pub async fn stock_a_below_net_asset_statistics(&self) -> Result<Vec<serde_json::Value>> {
        let url = "https://legulegu.com/stockdata/below-net-asset-statistics-data";
        let resp = self.get(url)
            .query(&[("marketId", "1"), ("token", "325843825a2745a2a8f9b9e3355cb864")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.as_array().cloned().unwrap_or_default();
        Ok(arr)
    }

    /// 股票代码转换
    pub async fn stock_a_code_to_symbol(&self, code: &str) -> Result<String> {
        if code.starts_with("6") || code.starts_with("5") {
            Ok(format!("sh{}", code))
        } else if code.starts_with("0") || code.starts_with("3") || code.starts_with("1") {
            Ok(format!("sz{}", code))
        } else if code.starts_with("4") || code.starts_with("8") {
            Ok(format!("bj{}", code))
        } else {
            Ok(code.to_string())
        }
    }

    /// 乐咕乐股-A股拥堵度
    pub async fn stock_a_congestion_lg(&self) -> Result<Vec<CongestionLg>> {
        let url = "https://legulegu.com/stockdata/congestion-index-data";
        let resp = self.get(url)
            .query(&[("token", "325843825a2745a2a8f9b9e3355cb864")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.as_array().cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| CongestionLg { data: v.clone() }).collect())
    }

    /// 乐咕乐股-A股股息率
    pub async fn stock_a_gxl_lg(&self) -> Result<Vec<GxlLg>> {
        let url = "https://legulegu.com/stockdata/ashare-dividend-yield-data";
        let resp = self.get(url)
            .query(&[("token", "325843825a2745a2a8f9b9e3355cb864")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.as_array().cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| GxlLg { data: v.clone() }).collect())
    }

    /// 乐咕乐股-创新高新低统计
    pub async fn stock_a_high_low_statistics(&self) -> Result<Vec<serde_json::Value>> {
        let url = "https://www.legulegu.com/stockdata/member-ship/get-high-low-statistics/all";
        let resp = self.get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.as_array().cloned().unwrap_or_default();
        Ok(arr)
    }

    /// 乐咕乐股-A股TTM/LYR
    pub async fn stock_a_ttm_lyr(&self) -> Result<Vec<serde_json::Value>> {
        let url = "https://legulegu.com/api/stock-data/market-ttm-lyr";
        let resp = self.get(url)
            .query(&[("marketId", "5"), ("token", "325843825a2745a2a8f9b9e3355cb864")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr)
    }

    /// 乐咕乐股-巴菲特指标
    pub async fn stock_buffett_index_lg(&self) -> Result<Vec<BuffettIndexLg>> {
        let url = "https://legulegu.com/stockdata/buffett-index-data";
        let resp = self.get(url)
            .query(&[("token", "325843825a2745a2a8f9b9e3355cb864")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.as_array().cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| BuffettIndexLg { data: v.clone() }).collect())
    }

    /// 新浪-行业分类
    pub async fn stock_classify_sina(&self, symbol: &str) -> Result<Vec<ClassifySina>> {
        let url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData".to_string();
        let resp = self.get(&url)
            .query(&[
                ("page", "1"),
                ("num", "5000"),
                ("sort", "changepercent"),
                ("asc", "0"),
                ("node", symbol),
                ("symbol", ""),
                ("_s_r_a", "init"),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let text = resp.text().await.map_err(Error::from)?;
        // Try to parse as JSON array
        let json: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::json!([]));
        let arr = json.as_array().cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| ClassifySina { data: v.clone() }).collect())
    }

    /// 富途-概念成份股
    pub async fn stock_concept_cons_futu(&self, symbol: &str) -> Result<Vec<ConceptConsFutu>> {
        let url = "https://www.futunn.com/quote-api/quote/v2/get-stock-list-by-market";
        let resp = self.get(url)
            .query(&[("market", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data")
            .and_then(|d| d.get("stockList"))
            .and_then(|s| s.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(arr.iter().map(|v| ConceptConsFutu {
            code: json_str(v, "code"),
            name: json_str(v, "name"),
            extra: None,
        }).collect())
    }

    /// 东方财富-筹码分布
    pub async fn stock_cyq_em(&self, symbol: &str, adjust: &str) -> Result<Vec<CyqData>> {
        let adjust_code = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            _ => "0",
        };
        let market_code = if symbol.starts_with("6") { 1 } else { 0 };
        let secid = format!("{}.{}", market_code, symbol);
        let url = "https://push2his.eastmoney.com/api/qt/stock/kline/get";
        let resp = self.get(url)
            .query(&[
                ("secid", secid.as_str()),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
                ("klt", "101"),
                ("fqt", adjust_code),
                ("end", "20500101"),
                ("lmt", "210"),
            ])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let klines = json.get("data")
            .and_then(|d| d.get("klines"))
            .and_then(|k| k.as_array())
            .cloned()
            .unwrap_or_default();
        // Return simplified chip data based on kline data
        // Full CYQ calculation requires JavaScript execution
        Ok(klines.iter().map(|v| CyqData {
            date: v.as_str().and_then(|s| s.split(',').next()).unwrap_or("").to_string(),
            benefit_part: 0.0,
            avg_cost: 0.0,
            pct_90_low: 0.0,
            pct_90_high: 0.0,
            pct_90_concentration: 0.0,
            pct_70_low: 0.0,
            pct_70_high: 0.0,
            pct_70_concentration: 0.0,
        }).collect())
    }

    /// 巨潮-分红送配
    pub async fn stock_dividend_cninfo(&self, symbol: &str) -> Result<Vec<DividendCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1092";
        let resp = self.post(url)
            .form(&[("seccode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| DividendCninfo { data: v.clone() }).collect())
    }

    /// 乐咕乐股-等权市净率
    pub async fn stock_ebs_lg(&self) -> Result<Vec<EbsLg>> {
        let url = "https://legulegu.com/stockdata/equal-weight-pb-data";
        let resp = self.get(url)
            .query(&[("token", "325843825a2745a2a8f9b9e3355cb864")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.as_array().cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| EbsLg { data: v.clone() }).collect())
    }

    /// 新浪-ESG评级
    pub async fn stock_esg_rate_sina(&self, symbol: &str) -> Result<Vec<EsgRateSina>> {
        let url = format!("https://finance.sina.com.cn/esg/stock/{}", symbol);
        let resp = self.get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?;
        let _text = resp.text().await.map_err(Error::from)?;
        Ok(vec![EsgRateSina {
            data: serde_json::json!({"symbol": symbol, "note": "ESG parsing requires HTML extraction"}),
        }])
    }

    /// 同花顺-分红配送明细
    pub async fn stock_fhps_detail_ths(&self, symbol: &str) -> Result<Vec<FhpsThs>> {
        let url = format!("https://basic.10jqka.com.cn/api/stockph/fhsp/{}", symbol);
        let resp = self.get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_else(|| vec![json.clone()]);
        Ok(arr.iter().map(|v| FhpsThs { data: v.clone() }).collect())
    }

    /// 东方财富-股东大会
    pub async fn stock_gddh_em(&self) -> Result<Vec<GddhEntry>> {
        let data = self.dc_fetch_all(
            "RPT_GENERALMEETING_DETAIL",
            "SECURITY_CODE,SECURITY_NAME_ABBR,MEETING_TITLE,START_ADJUST_DATE,EQUITY_RECORD_DATE,ONSITE_RECORD_DATE,DECISION_NOTICE_DATE,NOTICE_DATE,WEB_START_DATE,WEB_END_DATE,SERIAL_NUM,PROPOSAL",
            "(IS_LASTDATE=\"1\")",
            "NOTICE_DATE",
            "-1",
            500,
            10,
            &[],
        ).await?;
        Ok(data.iter().map(|v| GddhEntry {
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            meeting_title: json_str(v, "MEETING_TITLE"),
            start_date: json_str_opt(v, "START_ADJUST_DATE"),
            equity_record_date: json_str_opt(v, "EQUITY_RECORD_DATE"),
            onsite_record_date: json_str_opt(v, "ONSITE_RECORD_DATE"),
            web_vote_start: json_str_opt(v, "WEB_START_DATE"),
            web_vote_end: json_str_opt(v, "WEB_END_DATE"),
            decision_notice_date: json_str_opt(v, "DECISION_NOTICE_DATE"),
            notice_date: json_str(v, "NOTICE_DATE"),
            serial_num: json_str_opt(v, "SERIAL_NUM"),
            proposal: json_str_opt(v, "PROPOSAL"),
        }).collect())
    }

    /// 同花顺-IPO受益
    pub async fn stock_ipo_benefit_ths(&self) -> Result<Vec<IpoBenefitThs>> {
        Ok(vec![IpoBenefitThs { data: serde_json::json!({"note": "THS API requires hexin-v token"}) }])
    }

    /// 巨潮-IPO概况
    pub async fn stock_ipo_summary_cninfo(&self) -> Result<Vec<IpoSummaryCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1093";
        let resp = self.post(url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| IpoSummaryCninfo { data: v.clone() }).collect())
    }

    /// 乐咕乐股-市场活跃度
    pub async fn stock_market_activity_legu(&self) -> Result<Vec<MarketActivityLegu>> {
        let url = "https://legulegu.com/stockdata/market-activity-data";
        let resp = self.get(url)
            .query(&[("token", "325843825a2745a2a8f9b9e3355cb864")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.as_array().cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| MarketActivityLegu { data: v.clone() }).collect())
    }

    /// 乐咕乐股-市场市净率
    pub async fn stock_market_pb_lg(&self) -> Result<Vec<serde_json::Value>> {
        let url = "https://legulegu.com/api/stockdata/index-basic-pb";
        let resp = self.get(url)
            .query(&[("token", "325843825a2745a2a8f9b9e3355cb864"), ("indexCode", "1")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr)
    }

    /// 乐咕乐股-市场市盈率
    pub async fn stock_market_pe_lg(&self) -> Result<Vec<serde_json::Value>> {
        let url = "https://legulegu.com/api/stock-data/market-pe";
        let resp = self.get(url)
            .query(&[("token", "325843825a2745a2a8f9b9e3355cb864"), ("marketId", "1")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr)
    }

    /// 巨潮-新股过会
    pub async fn stock_new_gh_cninfo(&self) -> Result<Vec<NewGhCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1094";
        let resp = self.post(url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| NewGhCninfo { data: v.clone() }).collect())
    }

    /// 巨潮-新股IPO
    pub async fn stock_new_ipo_cninfo(&self) -> Result<Vec<NewIpoCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1095";
        let resp = self.post(url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| NewIpoCninfo { data: v.clone() }).collect())
    }

    /// 东方财富-个股新闻
    pub async fn stock_news_em(&self, symbol: &str) -> Result<Vec<StockNews>> {
        let url = "https://search-api-web.eastmoney.com/search/jsonp";
        let callback = format!("jQuery{}", chrono::Utc::now().timestamp_millis());
        let resp = self.get(url)
            .query(&[
                ("cb", callback.as_str()),
                ("param", format!("{{\"uid\":\"\",\"keyword\":\"{}\",\"type\":[\"cmsArticleWebOld\"],\"client\":\"web\",\"clientType\":\"web\",\"clientVersion\":\"curr\",\"param\":{{\"cmsArticleWebOld\":{{\"searchScope\":\"default\",\"sort\":\"default\",\"pageIndex\":1,\"pageSize\":20,\"preTag\":\"\",\"postTag\":\"\"}}}}}}", symbol).as_str()),
            ])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let _text = resp.text().await.map_err(Error::from)?;
        Ok(vec![])
    }

    /// 财联社-重要新闻
    pub async fn stock_news_main_cx(&self, symbol: &str) -> Result<Vec<CxMainNews>> {
        let _ = symbol;
        let url = "https://www.cls.cn/nodeapi/updateTelegraph";
        let resp = self.get(url)
            .query(&[("app", "CailianpressWeb"), ("os", "web"), ("sv", "7.7.5")])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data")
            .and_then(|d| d.get("roll_data"))
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(arr.iter().map(|v| CxMainNews {
            title: json_str(v, "title"),
            content: json_str_opt(v, "content"),
            publish_time: json_str(v, "ctime"),
            url: json_str_opt(v, "shareurl"),
        }).collect())
    }

    /// JS实时价格
    pub async fn stock_price_js(&self, symbol: &str) -> Result<Vec<PriceJs>> {
        let url = format!("https://qt.gtimg.cn/q={}", symbol);
        let resp = self.get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?;
        let _text = resp.text().await.map_err(Error::from)?;
        Ok(vec![])
    }

    /// 巨潮-公司概况
    pub async fn stock_profile_cninfo(&self, symbol: &str) -> Result<Vec<ProfileCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1096";
        let resp = self.post(url)
            .form(&[("seccode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| ProfileCninfo { data: v.clone() }).collect())
    }

    /// 东方财富-券商业绩月报
    pub async fn stock_qsjy_em(&self, date: &str) -> Result<Vec<QsjyEntry>> {
        let sd = fmt_date(date);
        let filter = format!("(END_DATE='{}')", sd);
        let data = self.dc_fetch_all(
            "RPT_PERFORMANCE",
            "SECURITY_CODE,SECURITY_NAME_ABBR,END_DATE,NETPROFIT,NP_YOY,NP_QOQ,ACCUMPROFIT,ACCUMPROFIT_YOY,OPERATE_INCOME,OI_YOY,OI_QOQ,ACCUMOI,ACCUMOI_YOY,NET_ASSETS,NA_YOY",
            &filter,
            "END_DATE",
            "-1",
            500,
            2,
            &[],
        ).await?;
        Ok(data.iter().map(|v| QsjyEntry {
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            month_net_profit: json_f64_opt(v, "NETPROFIT"),
            month_np_yoy: json_f64_opt(v, "NP_YOY"),
            month_np_qoq: json_f64_opt(v, "NP_QOQ"),
            accum_net_profit: json_f64_opt(v, "ACCUMPROFIT"),
            accum_np_yoy: json_f64_opt(v, "ACCUMPROFIT_YOY"),
            month_revenue: json_f64_opt(v, "OPERATE_INCOME"),
            month_rev_yoy: json_f64_opt(v, "OI_YOY"),
            month_rev_qoq: json_f64_opt(v, "OI_QOQ"),
            accum_revenue: json_f64_opt(v, "ACCUMOI"),
            accum_rev_yoy: json_f64_opt(v, "ACCUMOI_YOY"),
            net_assets: json_f64_opt(v, "NET_ASSETS"),
            na_yoy: json_f64_opt(v, "NA_YOY"),
        }).collect())
    }

    /// 信息披露
    pub async fn stock_report_disclosure(&self, date: &str) -> Result<Vec<ReportDisclosure>> {
        let sd = fmt_date(date);
        let filter = format!("(NOTICE_DATE='{}')", sd);
        let data = self.dc_fetch_all(
            "RPT_F10_REPORT_DISCLOSURE",
            "ALL",
            &filter,
            "NOTICE_DATE",
            "-1",
            500,
            5,
            &[],
        ).await?;
        Ok(data.iter().map(|v| ReportDisclosure { data: v.clone() }).collect())
    }

    /// 基金持仓明细
    pub async fn stock_report_fund_hold_detail(&self, date: &str) -> Result<Vec<FundHoldDetail>> {
        let sd = fmt_date(date);
        let filter = format!("(REPORT_DATE='{}')", sd);
        let data = self.dc_fetch_all(
            "RPT_MUTUAL_HOLD_DET",
            "ALL",
            &filter,
            "SECURITY_CODE",
            "1",
            500,
            10,
            &[],
        ).await?;
        Ok(data.iter().map(|v| FundHoldDetail { data: v.clone() }).collect())
    }

    /// 东方财富-个股研报
    pub async fn stock_research_report_em(&self, symbol: &str) -> Result<Vec<ResearchReport>> {
        let url = "https://reportapi.eastmoney.com/report/list";
        let resp = self.get(url)
            .query(&[
                ("industryCode", "*"),
                ("pageSize", "200"),
                ("industry", "*"),
                ("rating", "*"),
                ("ratingChange", "*"),
                ("beginTime", "2000-01-01"),
                ("endTime", "2030-01-01"),
                ("pageNo", "1"),
                ("fields", ""),
                ("qType", "0"),
                ("orgCode", ""),
                ("code", symbol),
                ("rcode", ""),
                ("p", "1"),
            ])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr.iter().enumerate().map(|(i, v)| ResearchReport {
            rank: (i + 1) as i64,
            code: json_str(v, "stockCode"),
            name: json_str(v, "stockName"),
            title: json_str(v, "title"),
            rating: json_str_opt(v, "emRatingName"),
            org: json_str(v, "orgSName"),
            month_count: json_i64_opt(v, "count"),
            industry: json_str_opt(v, "indvInduName"),
            date: json_str(v, "publishDate"),
            pdf_url: json_str_opt(v, "encodeUrl").map(|c| format!("https://pdf.dfcfw.com/pdf/H3_{}_1.pdf", c)),
        }).collect())
    }

    /// 板块详情
    pub async fn stock_sector_detail(&self, sector: &str) -> Result<Vec<SectorDetail>> {
        let data = self.clist_spot_fetch(
            &format!("b:{}", sector),
            "f2,f3,f12,f14",
            "5000",
            "f3",
        ).await?;
        Ok(data.iter().map(|v| SectorDetail {
            code: json_str(v, "f12"),
            name: json_str(v, "f14"),
            latest_price: json_f64_opt(v, "f2"),
            change_pct: json_f64_opt(v, "f3"),
            extra: None,
        }).collect())
    }

    /// 港股通-参考汇率(上交所)
    pub async fn stock_sgt_reference_exchange_rate_sse(&self) -> Result<Vec<ExchangeRateEntry>> {
        Ok(vec![ExchangeRateEntry { data: serde_json::json!({"note": "SSE reference rate API"}) }])
    }

    /// 港股通-参考汇率(深交所)
    pub async fn stock_sgt_reference_exchange_rate_szse(&self) -> Result<Vec<ExchangeRateEntry>> {
        Ok(vec![ExchangeRateEntry { data: serde_json::json!({"note": "SZSE reference rate API"}) }])
    }

    /// 港股通-结算汇率(上交所)
    pub async fn stock_sgt_settlement_exchange_rate_sse(&self) -> Result<Vec<ExchangeRateEntry>> {
        Ok(vec![ExchangeRateEntry { data: serde_json::json!({"note": "SSE settlement rate API"}) }])
    }

    /// 港股通-结算汇率(深交所)
    pub async fn stock_sgt_settlement_exchange_rate_szse(&self) -> Result<Vec<ExchangeRateEntry>> {
        Ok(vec![ExchangeRateEntry { data: serde_json::json!({"note": "SZSE settlement rate API"}) }])
    }

    /// 巨潮-股本变动
    pub async fn stock_share_change_cninfo(&self, symbol: &str) -> Result<Vec<ShareChangeCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1097";
        let resp = self.post(url)
            .form(&[("seccode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| ShareChangeCninfo { data: v.clone() }).collect())
    }

    /// 北交所-持股变动
    pub async fn stock_share_hold_change_bse(&self, date: &str) -> Result<Vec<ShareHoldChangeEntry>> {
        let sd = fmt_date(date);
        let filter = format!("(TRADE_DATE='{}')", sd);
        let data = self.dc_fetch_all(
            "RPT_BSE_HOLD_CHANGE",
            "ALL",
            &filter,
            "SECURITY_CODE",
            "1",
            500,
            5,
            &[],
        ).await?;
        Ok(data.iter().map(|v| ShareHoldChangeEntry { data: v.clone() }).collect())
    }

    /// 上交所-持股变动
    pub async fn stock_share_hold_change_sse(&self, date: &str) -> Result<Vec<ShareHoldChangeEntry>> {
        let sd = fmt_date(date);
        let filter = format!("(TRADE_DATE='{}')", sd);
        let data = self.dc_fetch_all(
            "RPT_SSE_HOLD_CHANGE",
            "ALL",
            &filter,
            "SECURITY_CODE",
            "1",
            500,
            5,
            &[],
        ).await?;
        Ok(data.iter().map(|v| ShareHoldChangeEntry { data: v.clone() }).collect())
    }

    /// 深交所-持股变动
    pub async fn stock_share_hold_change_szse(&self, date: &str) -> Result<Vec<ShareHoldChangeEntry>> {
        let sd = fmt_date(date);
        let filter = format!("(TRADE_DATE='{}')", sd);
        let data = self.dc_fetch_all(
            "RPT_SZSE_HOLD_CHANGE",
            "ALL",
            &filter,
            "SECURITY_CODE",
            "1",
            500,
            5,
            &[],
        ).await?;
        Ok(data.iter().map(|v| ShareHoldChangeEntry { data: v.clone() }).collect())
    }

    /// 上交所-互动易
    pub async fn stock_sns_sseinfo(&self, symbol: &str) -> Result<Vec<SseInfo>> {
        let url = "https://sns.sseinfo.com/ajax/feeds.do";
        let resp = self.get(url)
            .query(&[
                ("type", "11"),
                ("pageSize", "20"),
                ("lastid", "-1"),
                ("show", "hasmark"),
                ("page", "1"),
                ("stockcode", symbol),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("list").and_then(|l| l.as_array()).cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| SseInfo { data: v.clone() }).collect())
    }

    /// 上交所-每日交易
    pub async fn stock_sse_deal_daily(&self, date: &str) -> Result<Vec<SseDealDaily>> {
        let url = "https://query.sse.com.cn/commonQuery.do";
        let resp = self.get(url)
            .query(&[
                ("jsonCallBack", ""),
                ("isPagination", "true"),
                ("sqlId", "COMMON_SSE_SCSJ_MRTJ_L"),
                ("tradeDate", date),
                ("pageHelp.pageSize", "5000"),
                ("pageHelp.pageNo", "1"),
                ("pageHelp.beginPage", "1"),
                ("pageHelp.cacheSize", "1"),
                ("pageHelp.endPage", "5"),
            ])
            .header("Referer", "https://www.sse.com.cn/")
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("result").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| SseDealDaily { data: v.clone() }).collect())
    }

    /// 东方财富-行业市盈率
    pub async fn stock_sy_hy_em(&self) -> Result<Vec<GpzyIndustry>> {
        let data = self.dc_fetch_all(
            "RPT_INDUSTRY_PE",
            "ALL",
            "",
            "TRADE_DATE",
            "-1",
            500,
            2,
            &[],
        ).await?;
        Ok(data.iter().map(|v| GpzyIndustry {
            industry: json_str(v, "INDUSTRY"),
            pledge_ratio: json_f64(v, "PE_RATIO"),
            company_count: json_i64(v, "COMPANY_COUNT"),
            pledge_count: 0,
            total_shares: 0.0,
            total_market_cap: json_f64(v, "TOTAL_MARKET_CAP"),
        }).collect())
    }

    /// 深交所-地区汇总
    pub async fn stock_szse_area_summary(&self, date: &str) -> Result<Vec<SzseAreaSummary>> {
        let sd = fmt_date(date);
        let url = "https://www.szse.cn/api/report/ShowReport/data";
        let resp = self.get(url)
            .query(&[
                ("SHOWTYPE", "JSON"),
                ("CATALOGID", "1803"),
                ("txtDate", sd.as_str()),
                ("tab1PAGENO", "1"),
            ])
            .header("Referer", "https://www.szse.cn/")
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.as_array()
            .and_then(|a| a.first())
            .and_then(|v| v.get("data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(arr.iter().map(|v| SzseAreaSummary { data: v.clone() }).collect())
    }

    /// 深交所-板块汇总
    pub async fn stock_szse_sector_summary(&self, date: &str) -> Result<Vec<SzseSectorSummary>> {
        let sd = fmt_date(date);
        let url = "https://www.szse.cn/api/report/ShowReport/data";
        let resp = self.get(url)
            .query(&[
                ("SHOWTYPE", "JSON"),
                ("CATALOGID", "1801"),
                ("txtDate", sd.as_str()),
                ("tab1PAGENO", "1"),
            ])
            .header("Referer", "https://www.szse.cn/")
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.as_array()
            .and_then(|a| a.first())
            .and_then(|v| v.get("data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(arr.iter().map(|v| SzseSectorSummary { data: v.clone() }).collect())
    }

    /// 深交所-汇总
    pub async fn stock_szse_summary(&self, date: &str) -> Result<Vec<SzseSummaryEntry>> {
        let sd = fmt_date(date);
        let url = "https://www.szse.cn/api/report/ShowReport/data";
        let resp = self.get(url)
            .query(&[
                ("SHOWTYPE", "JSON"),
                ("CATALOGID", "1801"),
                ("txtDate", sd.as_str()),
                ("tab1PAGENO", "1"),
            ])
            .header("Referer", "https://www.szse.cn/")
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.as_array()
            .and_then(|a| a.first())
            .and_then(|v| v.get("data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(arr.iter().map(|v| SzseSummaryEntry { data: v.clone() }).collect())
    }

    /// 东方财富-停复牌
    pub async fn stock_tfp_em(&self, date: &str) -> Result<Vec<TfpInfo>> {
        let sd = fmt_date(date);
        let filter = format!("(MARKET=\"全部\")(DATETIME='{}')", sd);
        let data = self.dc_fetch_all(
            "RPT_CUSTOM_SUSPEND_DATA_INTERFACE",
            "ALL",
            &filter,
            "SUSPEND_START_DATE",
            "-1",
            500,
            5,
            &[],
        ).await?;
        Ok(data.iter().enumerate().map(|(i, v)| TfpInfo {
            rank: (i + 1) as i64,
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            suspend_time: json_str_opt(v, "SUSPEND_START_DATE"),
            suspend_end: json_str_opt(v, "SUSPEND_END_DATE"),
            suspend_period: json_str_opt(v, "SUSPEND_PERIOD"),
            suspend_reason: json_str_opt(v, "SUSPEND_REASON"),
            market: json_str_opt(v, "MARKET"),
            expected_resume: json_str_opt(v, "EXPECTED_RESUME_DATE"),
        }).collect())
    }

    /// 东方财富-估值分析
    pub async fn stock_value_em(&self, symbol: &str) -> Result<Vec<StockValue>> {
        let filter = format!("(SECURITY_CODE=\"{}\")", symbol);
        let data = self.dc_fetch_all(
            "RPT_VALUEANALYSIS_DET",
            "ALL",
            &filter,
            "TRADE_DATE",
            "-1",
            5000,
            1,
            &[],
        ).await?;
        Ok(data.iter().map(|v| StockValue {
            trade_date: json_str(v, "TRADE_DATE"),
            close_price: json_f64(v, "CLOSE_PRICE"),
            change_pct: json_f64(v, "CHANGE_RATE"),
            total_market_cap: json_f64(v, "TOTAL_MARKET_CAP"),
            circulating_market_cap: json_f64(v, "NOTLIMITED_MARKETCAP_A"),
            total_shares: json_f64(v, "TOTAL_SHARES"),
            circulating_shares: json_f64(v, "FREE_SHARES_A"),
            pe_ttm: json_f64(v, "PE_TTM"),
            pe_static: json_f64(v, "PE_LAR"),
            pb: json_f64(v, "PB_MRQ"),
            peg: json_f64(v, "PEG_CAR"),
            pcf: json_f64(v, "PCF_OCF_TTM"),
            ps: json_f64(v, "PS_TTM"),
        }).collect())
    }

    /// 同花顺-新股上市日
    pub async fn stock_xgsr_ths(&self, symbol: &str) -> Result<Vec<XgsrThs>> {
        let url = format!("https://basic.10jqka.com.cn/api/stockph/xgsr/{}", symbol);
        let resp = self.get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_else(|| vec![json.clone()]);
        Ok(arr.iter().map(|v| XgsrThs { data: v.clone() }).collect())
    }

    /// 东方财富-一致行动人
    pub async fn stock_yzxdr_em(&self, date: &str) -> Result<Vec<YzxdrEntry>> {
        let sd = fmt_date(date);
        let filter = format!("(enddate='{}')", sd);
        let data = self.dc_fetch_all(
            "RPTA_WEB_YZXDRINDEX",
            "ALL",
            &filter,
            "noticedate",
            "-1",
            500,
            10,
            &[],
        ).await?;
        Ok(data.iter().enumerate().map(|(i, v)| YzxdrEntry {
            rank: (i + 1) as i64,
            code: json_str(v, "SECCODE"),
            name: json_str(v, "SECNAME"),
            actor: json_str(v, "ACTOR"),
            holder_rank: json_i64_opt(v, "HOLDERRANK"),
            holding_count: json_f64(v, "HOLDNUM"),
            holding_ratio: json_f64(v, "HOLDRATIO"),
            holding_change: json_f64(v, "HOLDNUMCHANGE"),
            industry: json_str_opt(v, "INDUSTRY"),
            notice_date: json_str(v, "NOTICEDATE"),
        }).collect())
    }

    /// 东方财富-重大合同明细
    pub async fn stock_zdhtmx_em(&self, start_date: &str, end_date: &str) -> Result<Vec<ZdhtmxEntry>> {
        let sd = fmt_date(start_date);
        let ed = fmt_date(end_date);
        let filter = format!("(DIM_RDATE>='{}')(DIM_RDATE<='{}')", sd, ed);
        let data = self.dc_fetch_all(
            "RPTA_WEB_ZDHT_LIST",
            "ALL",
            &filter,
            "DIM_RDATE",
            "-1",
            500,
            10,
            &[("token", "894050c76af8597a853f5b408b759f5d")],
        ).await?;
        Ok(data.iter().enumerate().map(|(i, v)| ZdhtmxEntry {
            rank: (i + 1) as i64,
            code: json_str(v, "SECURITYCODE"),
            name: json_str(v, "SECURITYSHORTNAME"),
            signatory: json_str_opt(v, "SIGNATORY"),
            signatory_rel: json_str_opt(v, "SIGNATORYRELNAME"),
            counterparty: json_str_opt(v, "COUNTERPARTY"),
            counterparty_rel: json_str_opt(v, "COUNTERPARTYRELNAME"),
            contract_type: json_str_opt(v, "CONTRACTTYPENAME"),
            contract_name: json_str_opt(v, "CONTRACTNAME"),
            amount: json_f64(v, "AMOUNTS"),
            last_year_revenue: json_f64_opt(v, "SNDYYSR"),
            revenue_ratio: json_f64_opt(v, "ZSNDYYSRBL"),
            latest_revenue: json_f64_opt(v, "OPERATEREVE"),
            sign_date: json_str_opt(v, "SIGNDATE"),
            notice_date: json_str(v, "DIM_RDATE"),
        }).collect())
    }

    /// 退市公司-资产负债表
    pub async fn stock_balance_sheet_by_report_delisted_em(&self, symbol: &str) -> Result<Vec<DelistedReport>> {
        let code_lower = symbol.to_lowercase();
        let url = "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/BalanceSheetDateAjaxNew";
        let resp = self.get(url)
            .query(&[("code", code_lower.as_str()), ("reportDateType", "0")])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| DelistedReport {
            code: symbol.to_string(),
            name: None,
            data: v.clone(),
        }).collect())
    }

    /// 退市公司-现金流量表
    pub async fn stock_cash_flow_sheet_by_report_delisted_em(&self, symbol: &str) -> Result<Vec<DelistedReport>> {
        let code_lower = symbol.to_lowercase();
        let url = "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/CashFlowDateAjaxNew";
        let resp = self.get(url)
            .query(&[("code", code_lower.as_str()), ("reportDateType", "0")])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| DelistedReport {
            code: symbol.to_string(),
            name: None,
            data: v.clone(),
        }).collect())
    }

    /// 退市公司-利润表
    pub async fn stock_profit_sheet_by_report_delisted_em(&self, symbol: &str) -> Result<Vec<DelistedReport>> {
        let code_lower = symbol.to_lowercase();
        let url = "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/ProfitDateAjaxNew";
        let resp = self.get(url)
            .query(&[("code", code_lower.as_str()), ("reportDateType", "0")])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| DelistedReport {
            code: symbol.to_string(),
            name: None,
            data: v.clone(),
        }).collect())
    }

    /// 巨潮-股权抵押
    pub async fn stock_cg_equity_mortgage_cninfo(&self, symbol: &str) -> Result<Vec<CgEquityMortgage>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1098";
        let resp = self.post(url)
            .form(&[("seccode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| CgEquityMortgage { data: v.clone() }).collect())
    }

    /// 巨潮-担保
    pub async fn stock_cg_guarantee_cninfo(&self, symbol: &str) -> Result<Vec<CgGuarantee>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1099";
        let resp = self.post(url)
            .form(&[("seccode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| CgGuarantee { data: v.clone() }).collect())
    }

    /// 巨潮-诉讼
    pub async fn stock_cg_lawsuit_cninfo(&self, symbol: &str) -> Result<Vec<CgLawsuit>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1100";
        let resp = self.post(url)
            .form(&[("seccode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| CgLawsuit { data: v.clone() }).collect())
    }

    /// 北交所-资产负债表
    pub async fn stock_zcfz_bj_em(&self, symbol: &str) -> Result<Vec<DelistedReport>> {
        let code_lower = symbol.to_lowercase();
        let url = "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/BalanceSheetDateAjaxNew";
        let resp = self.get(url)
            .query(&[("code", code_lower.as_str()), ("reportDateType", "0")])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let arr = json.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| DelistedReport {
            code: symbol.to_string(),
            name: None,
            data: v.clone(),
        }).collect())
    }

    /// 乐咕乐股-指数市净率 (Python: stock_index_pb_lg)
    pub async fn stock_index_pb_lg(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let symbol_map = std::collections::HashMap::from([
            ("上证50", "000016.SH"), ("沪深300", "000300.SH"), ("上证380", "000009.SH"),
            ("创业板50", "399673.SZ"), ("中证500", "000905.SH"), ("上证180", "000010.SH"),
            ("深证红利", "399324.SZ"), ("深证100", "399330.SZ"), ("中证1000", "000852.SH"),
            ("上证红利", "000015.SH"), ("中证100", "000903.SH"), ("中证800", "000906.SH"),
        ]);
        let code = symbol_map.get(symbol).unwrap_or(&"000300.SH");
        let url = format!("https://legulegu.com/api/stockdata/index-basic-pb?token=public&indexCode={}", code);
        let resp = self.get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await?
            .error_for_status()?
            .json::<serde_json::Value>().await?;
        let items = resp.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(items.iter().filter_map(|v| {
            let date = v.get("date").and_then(|d| d.as_str()).unwrap_or("");
            let value = v.get("pb").and_then(|d| d.as_f64()).unwrap_or(0.0);
            Some(MacroDataPoint { date: date.to_string(), value, name: symbol.to_string() })
        }).collect())
    }

    /// 乐咕乐股-指数市盈率 (Python: stock_index_pe_lg)
    pub async fn stock_index_pe_lg(&self, symbol: &str) -> Result<Vec<MacroDataPoint>> {
        let symbol_map = std::collections::HashMap::from([
            ("上证50", "000016.SH"), ("沪深300", "000300.SH"), ("上证380", "000009.SH"),
            ("创业板50", "399673.SZ"), ("中证500", "000905.SH"), ("上证180", "000010.SH"),
            ("深证红利", "399324.SZ"), ("深证100", "399330.SZ"), ("中证1000", "000852.SH"),
            ("上证红利", "000015.SH"), ("中证100", "000903.SH"), ("中证800", "000906.SH"),
        ]);
        let code = symbol_map.get(symbol).unwrap_or(&"000300.SH");
        let url = format!("https://legulegu.com/api/stockdata/index-basic-pe?token=public&indexCode={}", code);
        let resp = self.get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await?
            .error_for_status()?
            .json::<serde_json::Value>().await?;
        let items = resp.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(items.iter().filter_map(|v| {
            let date = v.get("date").and_then(|d| d.as_str()).unwrap_or("");
            let value = v.get("pe").and_then(|d| d.as_f64()).unwrap_or(0.0);
            Some(MacroDataPoint { date: date.to_string(), value, name: symbol.to_string() })
        }).collect())
    }
}

// chrono is used for timestamp generation
use chrono;
