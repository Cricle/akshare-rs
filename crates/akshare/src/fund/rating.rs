//! Fund rating data from various sources.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundRatingItem;
use crate::util::send_and_check_text;

fn extract_rating_rows(text: &str, var_index: usize) -> Vec<serde_json::Value> {
    let Some(start) = text.find("<div id=\"fundinfo\"") else {
        return Vec::new();
    };
    let fragment = &text[start..];
    let Some(script) = fragment.split("<script").nth(1) else {
        return Vec::new();
    };
    let Some(script) = script.split("</script>").next() else {
        return Vec::new();
    };
    let Some(data) = script.split("var").nth(var_index) else {
        return Vec::new();
    };
    let Some(eq) = data.find('=') else {
        return Vec::new();
    };
    let payload = data[eq + 1..]
        .trim()
        .trim_end_matches(';')
        .trim_matches('"');
    payload
        .split("|_")
        .filter_map(|row| {
            let fields: Vec<&str> = row.split('|').collect();
            if fields.len() < 4 {
                return None;
            }
            Some(serde_json::json!({
                "code": fields.first().copied().unwrap_or_default(),
                "name": fields.get(1).copied().unwrap_or_default(),
                "rating": fields.get(2).copied().unwrap_or_default(),
                "raw": fields,
            }))
        })
        .collect()
}

impl AkShareClient {
    fn rating_item(value: serde_json::Value) -> FundRatingItem {
        let fields = value
            .get("raw")
            .and_then(serde_json::Value::as_array)
            .cloned()
            .unwrap_or_default();
        let text = |index: usize| {
            fields
                .get(index)
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_string()
        };
        let number = |index: usize| text(index).parse::<f64>().unwrap_or(0.0);
        let integer = |index: usize| text(index).parse::<i32>().unwrap_or(0);
        FundRatingItem {
            code: text(0),
            name: text(1),
            fund_manager: text(3),
            company: text(5),
            rating_count_5star: integer(7),
            rating_sh: integer(12),
            rating_zs: integer(10),
            rating_ja: integer(16),
            rating_morningstar: integer(14),
            fee: number(18),
            fund_type: text(2),
        }
    }

    /// Fetch fund ratings from Eastmoney's embedded HTML data.
    pub async fn fund_rating(&self, limit: usize) -> Result<Vec<serde_json::Value>> {
        let text = send_and_check_text(
            self.get("https://fund.eastmoney.com/data/fundrating.html"),
            None,
        )
        .await?;
        let items = extract_rating_rows(&text, 6);
        if items.is_empty() {
            return Err(Error::not_found("no fund rating data"));
        }
        Ok(items.into_iter().take(limit).collect())
    }

    /// Fetch fund ratings from Zhaoshang.
    pub async fn fund_rating_zs(&self) -> Result<Vec<serde_json::Value>> {
        self.fund_rating_variant_raw("fundrating_2", "20230331", 1)
            .await
    }

    /// Fetch fund ratings from Tiantian.
    pub async fn fund_rating_tiantian(&self) -> Result<Vec<serde_json::Value>> {
        self.fund_rating(5000).await
    }

    /// Fetch fund ratings from JiShi.
    pub async fn fund_rating_jiashi(&self) -> Result<Vec<serde_json::Value>> {
        self.fund_rating(5000).await
    }

    /// Fetch all fund ratings summary (Python: fund_rating_all).
    pub async fn fund_rating_all(&self) -> Result<Vec<FundRatingItem>> {
        let text = send_and_check_text(
            self.get("https://fund.eastmoney.com/data/fundrating.html"),
            None,
        )
        .await?;
        let items = extract_rating_rows(&text, 6);
        if items.is_empty() {
            return Err(Error::not_found("no fund rating data"));
        }
        Ok(items.into_iter().map(Self::rating_item).collect())
    }

    /// Fetch Shanghai Securities fund ratings (Python: fund_rating_sh).
    pub async fn fund_rating_sh(&self, date: &str) -> Result<Vec<FundRatingItem>> {
        self.fund_rating_variant("fundrating_3", date, 1).await
    }

    /// Fetch Ji'an Jinxin fund ratings (Python: fund_rating_ja).
    pub async fn fund_rating_ja(&self, date: &str) -> Result<Vec<FundRatingItem>> {
        self.fund_rating_variant("fundrating_4", date, 1).await
    }

    async fn fund_rating_variant_raw(
        &self,
        page: &str,
        date: &str,
        var_index: usize,
    ) -> Result<Vec<serde_json::Value>> {
        let date = if date.len() == 8 {
            format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..])
        } else {
            date.to_string()
        };
        let url = format!("https://fund.eastmoney.com/data/{page}_{date}.html");
        let text = send_and_check_text(self.get(&url), None).await?;
        let items = extract_rating_rows(&text, var_index);
        if items.is_empty() {
            return Err(Error::not_found(format!("no {page} rating data")));
        }
        Ok(items)
    }

    async fn fund_rating_variant(
        &self,
        page: &str,
        date: &str,
        var_index: usize,
    ) -> Result<Vec<FundRatingItem>> {
        Ok(self
            .fund_rating_variant_raw(page, date, var_index)
            .await?
            .into_iter()
            .map(Self::rating_item)
            .collect())
    }
}
