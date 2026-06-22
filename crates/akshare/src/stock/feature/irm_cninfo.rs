//! Investor relations Q&A (互动易) from cninfo.

use crate::client::AkShareClient;
use crate::error::Result;

use super::types::IrmQuestion;

impl AkShareClient {
    /// Fetch org_id for a stock symbol from cninfo keyboard API.
    async fn cninfo_org_id(&self, symbol: &str) -> Result<String> {
        let resp = self
            .post("https://irm.cninfo.com.cn/newircs/index/queryKeyboardInfo")
            .query(&[("_t", "1691144074")])
            .form(&[("keyWord", symbol)])
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let org_id = resp
            .get("data")
            .and_then(|d| d.as_array())
            .and_then(|a| a.first())
            .and_then(|v| v.get("secid"))
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();
        Ok(org_id)
    }

    /// 互动易-提问
    pub async fn stock_irm(&self, symbol: &str) -> Result<Vec<IrmQuestion>> {
        let org_id = self.cninfo_org_id(symbol).await?;
        let mut all = Vec::new();
        for page in 1..=10 {
            let resp = self
                .post("https://irm.cninfo.com.cn/newircs/company/question")
                .query(&[("_t", "1691142650")])
                .form(&[
                    ("stockcode", symbol),
                    ("orgId", &org_id),
                    ("pageSize", "1000"),
                    ("pageNum", &page.to_string()),
                    ("keyWord", ""),
                    ("startDay", ""),
                    ("endDay", ""),
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
            if data.is_empty() {
                break;
            }
            for v in &data {
                all.push(IrmQuestion {
                    code: v
                        .get("stockCode")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string(),
                    company_name: v
                        .get("orgName")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string(),
                    question: v
                        .get("mainContent")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string(),
                    answer: v
                        .get("attachContent")
                        .and_then(|x| x.as_str())
                        .map(std::string::ToString::to_string),
                    questioner: v
                        .get("mainPerson")
                        .and_then(|x| x.as_str())
                        .map(std::string::ToString::to_string),
                    answerer: v
                        .get("attachPerson")
                        .and_then(|x| x.as_str())
                        .map(std::string::ToString::to_string),
                    question_time: v
                        .get("mainDate")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string(),
                    source: Some("cninfo".to_string()),
                });
            }
            let total_pages = resp
                .get("totalPage")
                .and_then(serde_json::Value::as_i64)
                .unwrap_or(1);
            if i64::from(page) >= total_pages {
                break;
            }
        }
        Ok(all)
    }

    /// 互动易-回答详情
    pub async fn stock_irm_ans(&self, question_id: &str) -> Result<Vec<IrmQuestion>> {
        let resp = self
            .post("https://irm.cninfo.com.cn/newircs/company/question/detail")
            .query(&[("_t", "1691142650")])
            .form(&[("questionId", question_id)])
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
        Ok(data
            .iter()
            .map(|v| IrmQuestion {
                code: v
                    .get("stockCode")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string(),
                company_name: v
                    .get("orgName")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string(),
                question: v
                    .get("mainContent")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string(),
                answer: v
                    .get("attachContent")
                    .and_then(|x| x.as_str())
                    .map(std::string::ToString::to_string),
                questioner: v
                    .get("mainPerson")
                    .and_then(|x| x.as_str())
                    .map(std::string::ToString::to_string),
                answerer: v
                    .get("attachPerson")
                    .and_then(|x| x.as_str())
                    .map(std::string::ToString::to_string),
                question_time: v
                    .get("mainDate")
                    .and_then(|x| x.as_str())
                    .unwrap_or("")
                    .to_string(),
                source: Some("cninfo".to_string()),
            })
            .collect())
    }
}
