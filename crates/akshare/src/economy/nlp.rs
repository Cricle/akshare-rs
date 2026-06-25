//! NLP-based financial sentiment data and knowledge graph queries.
//!
//! Provides access to:
//! - Ownthink knowledge graph for entity queries
//! - Ownthink robot for question answering
//! - Eastmoney datacenter for EPU/sentiment indicators (fallback)

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;
use crate::types::wire::EmDatacenterResp;

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct OwnthinkResponse {
    data: Option<OwnthinkData>,
}

#[derive(Debug, Deserialize)]
struct OwnthinkData {
    entity: Option<String>,
    desc: Option<String>,
    avp: Option<Vec<(String, String)>>,
    tag: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct BotResponse {
    data: Option<BotData>,
}

#[derive(Debug, Deserialize)]
struct BotData {
    info: Option<BotInfo>,
}

#[derive(Debug, Deserialize)]
struct BotInfo {
    text: Option<String>,
}

impl AkShareClient {
    /// Financial sentiment / EPU index data.
    ///
    /// Returns Economic Policy Uncertainty index readings or market
    /// sentiment indicators from the Eastmoney datacenter.
    pub async fn economy_sentiment_index(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&crate::util::eastmoney_datacenter_params(
                "REPORT_DATE",
                &[("reportName", "RPT_ECONOMY_SENTIMENT")],
            ))
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        let mut items = Vec::with_capacity(data.len());
        for v in &data {
            let date = v.str_or(&["REPORT_DATE", "DATE", "REPORT_PERIOD"], "");
            if date.is_empty() {
                continue;
            }
            let value = v.f64_or(
                &["EPU_INDEX", "INDICATOR_VALUE", "VALUE", "SENTIMENT_INDEX"],
                0.0,
            );
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value,
                name: "Sentiment Index".to_string(),
            });
        }
        Ok(items)
    }

    /// Ownthink knowledge graph entity query.
    ///
    /// Queries the Ownthink knowledge graph for information about a word/entity.
    /// `word` is the query term (e.g., "人工智能").
    /// `indicator` determines the return type: "entity", "desc", "avp", or "tag".
    pub async fn nlp_ownthink(&self, word: &str, indicator: &str) -> Result<Vec<MacroDataPoint>> {
        let url = "https://api.ownthink.com/kg/knowledge";
        let resp: OwnthinkResponse = self
            .post(url)
            .form(&[("entity", word)])
            .send()
            .await?
            .json()
            .await?;

        let data = resp
            .data
            .ok_or_else(|| Error::not_found(format!("ownthink: no data for '{word}'")))?;

        let mut items = Vec::new();
        match indicator {
            "entity" => {
                if let Some(entity) = data.entity {
                    items.push(MacroDataPoint {
                        date: word.to_string(),
                        value: 1.0,
                        name: entity,
                    });
                }
            }
            "desc" => {
                if let Some(desc) = data.desc {
                    items.push(MacroDataPoint {
                        date: word.to_string(),
                        value: 1.0,
                        name: desc,
                    });
                }
            }
            "avp" => {
                if let Some(avp) = data.avp {
                    for (attr, val) in avp {
                        items.push(MacroDataPoint {
                            date: attr,
                            value: 1.0,
                            name: val,
                        });
                    }
                }
            }
            "tag" => {
                if let Some(tags) = data.tag {
                    for tag in tags {
                        items.push(MacroDataPoint {
                            date: word.to_string(),
                            value: 1.0,
                            name: tag,
                        });
                    }
                }
            }
            _ => {
                return Err(Error::invalid_input(format!(
                    "unknown indicator: {indicator}"
                )));
            }
        }
        Ok(items)
    }

    /// Ownthink intelligent Q&A.
    ///
    /// Sends a question to the Ownthink robot and returns the answer.
    pub async fn nlp_answer(&self, question: &str) -> Result<String> {
        let url = "https://api.ownthink.com/bot";
        let resp: BotResponse = self
            .get(url)
            .query(&[("spoken", question)])
            .send()
            .await?
            .json()
            .await?;

        let answer = resp
            .data
            .and_then(|d| d.info)
            .and_then(|i| i.text)
            .ok_or_else(|| Error::decode("ownthink bot: no answer text"))?;

        Ok(answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_economy_sentiment_index_response_structure() {
        let json = r#"{
            "result": {
                "data": [
                    {"REPORT_DATE": "2024-03-01T00:00:00", "EPU_INDEX": 152.3},
                    {"REPORT_DATE": "2024-02-01T00:00:00", "EPU_INDEX": 148.7}
                ]
            }
        }"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].f64_field(&["EPU_INDEX"]), Some(152.3));
    }

    #[test]
    fn test_economy_sentiment_index_empty_response() {
        let json = r#"{"result": {"data": []}}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert!(data.is_empty());
    }

    #[test]
    fn test_economy_sentiment_index_null_result() {
        let json = r#"{"result": null}"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.map(|r| r.data).unwrap_or_default();
        assert!(data.is_empty());
    }
}
