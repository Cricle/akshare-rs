//! News and info (财经快讯) from Eastmoney.

use super::helpers::{json_str, json_str_opt};
use super::types::{DelistInfo, NameChangeInfo, NewsEntry};
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 东方财富-财经早餐
    pub async fn stock_info_cjzc_em(&self) -> Result<Vec<NewsEntry>> {
        let mut all = Vec::new();
        for page in 1..=2 {
            let resp = self
                .get("https://np-listapi.eastmoney.com/comm/web/getNewsByColumns")
                .query(&[
                    ("client", "web"),
                    ("biz", "web_news_col"),
                    ("column", "1207"),
                    ("order", "1"),
                    ("needInteractData", "0"),
                    ("page_index", &page.to_string()),
                    ("page_size", "200"),
                    ("req_trace", "1710314682980"),
                    (
                        "fields",
                        "code,showTime,title,mediaName,summary,image,url,uniqueUrl,Np_dst",
                    ),
                ])
                .send()
                .await?
                .error_for_status()?
                .json::<serde_json::Value>()
                .await?;
            let data = resp
                .get("data")
                .and_then(|d| d.get("list"))
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default();
            if data.is_empty() {
                break;
            }
            for v in &data {
                all.push(NewsEntry {
                    title: json_str(v, "title"),
                    summary: json_str_opt(v, "summary"),
                    time: json_str(v, "showTime"),
                    url: json_str_opt(v, "uniqueUrl"),
                });
            }
        }
        Ok(all)
    }

    /// 东方财富-全球财经快讯
    pub async fn stock_info_global_em(&self) -> Result<Vec<NewsEntry>> {
        let resp = self
            .get("https://np-anotice-stock.eastmoney.com/api/security/ann")
            .query(&[
                ("cb", ""),
                ("sr", "-1"),
                ("page_size", "50"),
                ("page_index", "1"),
                ("ann_type", "A"),
                ("client_source", "web"),
                ("f_node", "0"),
                ("s_node", "0"),
            ])
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let data = resp
            .get("data")
            .and_then(|d| d.get("list"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(data
            .iter()
            .map(|v| NewsEntry {
                title: json_str(v, "title"),
                summary: json_str_opt(v, "digest"),
                time: json_str(v, "notice_date"),
                url: None,
            })
            .collect())
    }

    /// 深交所-股票更名
    pub async fn stock_info_change_name(&self) -> Result<Vec<NameChangeInfo>> {
        let data = self
            .dc_fetch_all(
                "RPT_STK_CHGNAME",
                "ALL",
                "",
                "CHANGE_DATE",
                "-1",
                500,
                5,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| NameChangeInfo { data: v.clone() })
            .collect())
    }

    /// 上交所-退市
    pub async fn stock_info_sh_delist(&self) -> Result<Vec<DelistInfo>> {
        let data = self
            .dc_fetch_all(
                "RPT_DELIST_SSE",
                "ALL",
                "",
                "DELIST_DATE",
                "-1",
                500,
                5,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| DelistInfo { data: v.clone() })
            .collect())
    }

    /// 深交所-股票更名
    pub async fn stock_info_sz_change_name(&self) -> Result<Vec<NameChangeInfo>> {
        let data = self
            .dc_fetch_all(
                "RPT_STK_CHGNAME",
                "ALL",
                "(MARKET=\"SZ\")",
                "CHANGE_DATE",
                "-1",
                500,
                5,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| NameChangeInfo { data: v.clone() })
            .collect())
    }

    /// 深交所-退市
    pub async fn stock_info_sz_delist(&self) -> Result<Vec<DelistInfo>> {
        let data = self
            .dc_fetch_all(
                "RPT_DELIST_SZSE",
                "ALL",
                "",
                "DELIST_DATE",
                "-1",
                500,
                5,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| DelistInfo { data: v.clone() })
            .collect())
    }

    /// 财联社-全球资讯 (Python: stock_info_global_cls)
    pub async fn stock_info_global_cls(&self) -> Result<Vec<NewsEntry>> {
        let resp = self
            .get("https://www.cls.cn/nodeapi/telegraphList")
            .query(&[
                ("app", "CailianpressWeb"),
                ("os", "web"),
                ("sv", "8.4.6"),
                ("rn", "50"),
            ])
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let data = resp
            .get("data")
            .and_then(|d| d.get("roll_data"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(data
            .iter()
            .map(|v| NewsEntry {
                title: json_str(v, "title"),
                summary: json_str_opt(v, "brief"),
                time: json_str(v, "ctime"),
                url: None,
            })
            .collect())
    }

    /// 富途牛牛-全球资讯 (Python: stock_info_global_futu)
    pub async fn stock_info_global_futu(&self) -> Result<Vec<NewsEntry>> {
        let resp = self
            .get("https://news.futunn.com/news-site-api/main/get-flash-list")
            .query(&[("page", "1"), ("size", "50")])
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let data = resp
            .get("data")
            .and_then(|d| d.get("list"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(data
            .iter()
            .map(|v| NewsEntry {
                title: json_str(v, "title"),
                summary: json_str_opt(v, "summary"),
                time: json_str(v, "time"),
                url: None,
            })
            .collect())
    }

    /// 新浪财经-全球资讯 (Python: stock_info_global_sina)
    pub async fn stock_info_global_sina(&self) -> Result<Vec<NewsEntry>> {
        let resp = self
            .get("https://zhibo.sina.com.cn/api/zhibo/feed")
            .query(&[
                ("page", "1"),
                ("page_size", "50"),
                ("zhibo_id", "152"),
                ("tag_id", "0"),
                ("type", "0"),
            ])
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let data = resp
            .get("result")
            .and_then(|r| r.get("data"))
            .and_then(|d| d.get("feed"))
            .and_then(|d| d.get("list"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(data
            .iter()
            .map(|v| NewsEntry {
                title: json_str(v, "rich_text"),
                summary: None,
                time: json_str(v, "create_time"),
                url: None,
            })
            .collect())
    }

    /// 同花顺-全球资讯 (Python: stock_info_global_ths)
    pub async fn stock_info_global_ths(&self) -> Result<Vec<NewsEntry>> {
        let resp = self
            .get("https://news.10jqka.com.cn/tapp/news/push/stock")
            .query(&[
                ("page", "1"),
                ("tag", ""),
                ("track", "website"),
                ("pagesize", "50"),
            ])
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let data = resp
            .get("data")
            .and_then(|d| d.get("list"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(data
            .iter()
            .map(|v| NewsEntry {
                title: json_str(v, "title"),
                summary: json_str_opt(v, "digest"),
                time: json_str(v, "pub_time"),
                url: None,
            })
            .collect())
    }

    /// 上海证券交易所-股票列表 (Python: stock_info_sh_name_code)
    ///
    /// `symbol` is one of: "主板A股", "主板B股", "科创板"
    pub async fn stock_info_sh_name_code(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let indicator_map =
            std::collections::HashMap::from([("主板A股", "1"), ("主板B股", "2"), ("科创板", "8")]);
        let stock_type = indicator_map.get(symbol).unwrap_or(&"1");
        let resp = self
            .get("https://query.sse.com.cn/sseQuery/commonQuery.do")
            .query(&[
                ("STOCK_TYPE", *stock_type),
                ("REG_PROVINCE", ""),
                ("CSRC_CODE", ""),
                ("STOCK_CODE", ""),
                ("sqlId", "COMMON_SSE_CP_GPJCTPZ_GPLB_GP_L"),
                ("COMPANY_STATUS", "2,4,5,7,8"),
                ("type", "inParams"),
                ("isPagination", "true"),
                ("pageHelp.cacheSize", "1"),
                ("pageHelp.beginPage", "1"),
                ("pageHelp.pageSize", "10000"),
                ("pageHelp.pageNo", "1"),
                ("pageHelp.endPage", "1"),
            ])
            .header(
                "Referer",
                "https://www.sse.com.cn/assortment/stock/list/share/",
            )
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let data = resp
            .get("result")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(data)
    }

    /// 深圳证券交易所-股票列表 (Python: stock_info_sz_name_code)
    ///
    /// `symbol` is one of: "A股列表", "B股列表", "CDR列表", "AB股列表"
    pub async fn stock_info_sz_name_code(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let indicator_map = std::collections::HashMap::from([
            ("A股列表", "tab1"),
            ("B股列表", "tab2"),
            ("CDR列表", "tab3"),
            ("AB股列表", "tab4"),
        ]);
        let tab_key = indicator_map.get(symbol).unwrap_or(&"tab1");
        let resp = self
            .get("https://www.szse.cn/api/report/ShowReport")
            .query(&[
                ("SHOWTYPE", "xlsx"),
                ("CATALOGID", "1110"),
                ("TABKEY", *tab_key),
                ("random", "0.6935816432433362"),
            ])
            .send()
            .await?
            .error_for_status()?;
        let bytes = resp.bytes().await?;
        // Parse XLSX - return raw bytes info for now
        // In a full implementation, we'd parse the XLSX file
        Ok(vec![serde_json::json!({"raw_length": bytes.len()})])
    }

    /// 北京证券交易所-股票列表 (Python: stock_info_bj_name_code)
    pub async fn stock_info_bj_name_code(&self) -> Result<Vec<serde_json::Value>> {
        let resp = self
            .post("https://www.bse.cn/nqxxController/nqxxCnzq.do")
            .form(&[
                ("page", "0"),
                ("typejb", "T"),
                ("xxfcbj[]", "2"),
                ("xxzqdm", ""),
                ("sortfield", "xxzqdm"),
                ("sorttype", "asc"),
            ])
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let data = resp
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(data)
    }
}
