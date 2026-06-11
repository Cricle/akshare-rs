//! Disclosure reports (信息披露) from cninfo.

use crate::client::AkShareClient;
use crate::error::Result;

use super::types::*;

impl AkShareClient {
    /// 巨潮资讯-公告查询-信息披露
    pub async fn stock_zh_a_disclosure_report_cninfo(
        &self,
        symbol: &str,
        category: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<DisclosureReport>> {
        let mut all = Vec::new();
        for page in 1..=5 {
            let pn = page.to_string();
            let se_date = format!("{}~{}", start_date, end_date);
            let resp = self.post("https://www.cninfo.com.cn/new/hisAnnouncement/query")
                .form(&[
                    ("pageNum", pn),
                    ("pageSize", "30".to_string()),
                    ("column", "szse".to_string()),
                    ("tabName", "fulltext".to_string()),
                    ("plate", "".to_string()),
                    ("stock", symbol.to_string()),
                    ("searchkey", "".to_string()),
                    ("secid", "".to_string()),
                    ("category", category.to_string()),
                    ("trade", "".to_string()),
                    ("seDate", se_date),
                    ("sortName", "".to_string()),
                    ("sortType", "".to_string()),
                    ("isHLtitle", "true".to_string()),
                ])
                .send().await?
                .error_for_status()?
                .json::<serde_json::Value>().await?;
            let data = resp.get("announcements").and_then(|d| d.as_array()).cloned().unwrap_or_default();
            if data.is_empty() { break; }
            for v in &data {
                let code = v.get("secCode").and_then(|x| x.as_str()).unwrap_or("").to_string();
                let name = v.get("secName").and_then(|x| x.as_str()).unwrap_or("").to_string();
                all.push(DisclosureReport {
                    code: if code.is_empty() { None } else { Some(code) },
                    name: if name.is_empty() { None } else { Some(name) },
                    title: v.get("announcementTitle").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                    publish_date: v.get("announcementTime").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                    url: v.get("adjunctUrl").and_then(|x| x.as_str()).map(|u| format!("https://static.cninfo.com.cn/{}", u)),
                    category: v.get("announcementType").and_then(|x| x.as_str()).map(|s| s.to_string()),
                });
            }
        }
        Ok(all)
    }

    /// 巨潮资讯-公告查询-关联公告
    pub async fn stock_zh_a_disclosure_relation_cninfo(
        &self,
        announcement_id: &str,
    ) -> Result<Vec<DisclosureReport>> {
        let resp = self.post("https://www.cninfo.com.cn/new/announcement/query")
            .form(&[
                ("announcementId", announcement_id.to_string()),
            ])
            .send().await?
            .error_for_status()?
            .json::<serde_json::Value>().await?;
        let data = resp.get("announcements").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(data.iter().map(|v| {
            let code = v.get("secCode").and_then(|x| x.as_str()).unwrap_or("").to_string();
            let name = v.get("secName").and_then(|x| x.as_str()).unwrap_or("").to_string();
            DisclosureReport {
                code: if code.is_empty() { None } else { Some(code) },
                name: if name.is_empty() { None } else { Some(name) },
                title: v.get("announcementTitle").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                publish_date: v.get("announcementTime").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                url: v.get("adjunctUrl").and_then(|x| x.as_str()).map(|u| format!("https://static.cninfo.com.cn/{}", u)),
                category: v.get("announcementType").and_then(|x| x.as_str()).map(|s| s.to_string()),
            }
        }).collect())
    }
}
