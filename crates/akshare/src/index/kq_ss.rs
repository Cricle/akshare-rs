//! 柯桥时尚指数.

use serde::Deserialize;

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// 柯桥时尚指数及其子项数据.
    ///
    /// `symbol` is one of many sub-indices, e.g. "柯桥时尚指数", "时尚创意指数",
    /// "时尚设计人才数", etc.
    pub async fn index_kq_fashion(&self, symbol: &str) -> Result<Vec<KqFashionPoint>> {
        #[derive(Deserialize)]
        struct Envelope {
            data: Option<Vec<FashionItem>>,
        }

        #[derive(Deserialize)]
        struct FashionItem {
            #[serde(default, rename = "publishTime")]
            publish_time: String,
            #[serde(default, rename = "indexValue")]
            index_value: Option<f64>,
        }

        let struct_code = match symbol {
            "柯桥时尚指数" => "root",
            "时尚创意指数" => "01",
            "时尚设计人才数" => "0101",
            "新花型推出数" => "0102",
            "创意产品成交数" => "0103",
            "创意企业数量" => "0104",
            "时尚活跃度指数" => "02",
            "电商运行数"
            | "时尚平台拓展数"
            | "新产品销售额占比"
            | "企业合作占比"
            | "品牌传播费用" => "0201",
            "时尚推广度指数" => "03",
            "国际交流合作次数" => "0301",
            "企业参展次数" | "外商驻点数量变化" => "0302",
            "时尚评价指数" => "04",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unknown KQ fashion index: {symbol}"
                )));
            }
        };

        let response = crate::util::send_and_check(
            self.get("http://api.idx365.com/index/project/34/data")
                .query(&[("structCode", struct_code)]),
        )
        .await?;

        let payload: Envelope = response.json().await.map_err(Error::from)?;
        let items = payload.data.unwrap_or_default();

        let mut points: Vec<KqFashionPoint> = items
            .into_iter()
            .map(|item| KqFashionPoint {
                date: item.publish_time,
                index: item.index_value.unwrap_or(0.0),
                change_value: 0.0,
                change_pct: 0.0,
            })
            .collect();

        points.sort_by(|a, b| a.date.cmp(&b.date));

        // Compute diff and pct_change
        for i in 1..points.len() {
            let prev = points[i - 1].index;
            points[i].change_value = points[i].index - prev;
            if prev != 0.0 {
                points[i].change_pct = (points[i].index - prev) / prev;
            }
        }

        if points.is_empty() {
            return Err(Error::not_found("idx365 returned no KQ fashion data"));
        }
        Ok(points)
    }
}

/// KQ fashion index point.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KqFashionPoint {
    pub date: String,
    pub index: f64,
    pub change_value: f64,
    pub change_pct: f64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // KQ SS functions require network access.
    }
}
