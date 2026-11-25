//! Futures spot-to-stock (现货与股票) data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;
use crate::util::send_and_check_text;

impl AkShareClient {
    /// Futures spot-to-stock mapping — unified entry point.
    ///
    /// `date`: date string for the data
    /// Returns spot-to-stock mapping data.
    pub async fn futures_spot_stock(&self, category_or_date: &str) -> Result<Vec<Row>> {
        let category = match category_or_date {
            "能源" | "化工" | "塑料" | "纺织" | "有色" | "钢铁" | "建材" | "农副" => {
                category_or_date
            }
            value if value.len() == 8 && value.bytes().all(|byte| byte.is_ascii_digit()) => "能源",
            _ => category_or_date,
        };
        self.futures_spot_stock_em(category).await
    }

    /// Eastmoney spot-to-stock mapping data.
    ///
    /// `category`: "能源", "化工", "塑料", "纺织", "有色", "钢铁", "建材", "农副"
    pub async fn futures_spot_stock_em(&self, category: &str) -> Result<Vec<Row>> {
        let index = match category {
            "能源" => 0,
            "化工" => 1,
            "塑料" => 2,
            "纺织" => 3,
            "有色" => 4,
            "钢铁" => 5,
            "建材" => 6,
            "农副" => 7,
            _ => {
                return Err(Error::invalid_input(
                    "category must be one of 能源/化工/塑料/纺织/有色/钢铁/建材/农副",
                ));
            }
        };
        let body = send_and_check_text(
            self.get("https://data.eastmoney.com/ifdata/xhgp.html")
                .header(
                    "Accept",
                    "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
                )
                .header("User-Agent", "Mozilla/5.0"),
            Some(encoding_rs::GB18030),
        )
        .await?;
        let start = body
            .find("pagedata")
            .and_then(|position| {
                body[position..]
                    .find('=')
                    .map(|offset| position + offset + 1)
            })
            .ok_or_else(|| Error::decode("Eastmoney spot-stock pagedata not found"))?;
        let end = body[start..]
            .find("/newstatic/js/common/emdataview.js")
            .map(|offset| start + offset)
            .ok_or_else(|| Error::decode("Eastmoney spot-stock pagedata terminator not found"))?;
        let mut json_text = body[start..end].trim().trim_end_matches(';').trim();
        if let Some(script_end) = json_text.find("</script>") {
            json_text = json_text[..script_end].trim().trim_end_matches(';').trim();
        }
        let root: serde_json::Value = serde_json::from_str(json_text)
            .map_err(|err| Error::decode(format!("Eastmoney spot-stock JSON: {err}")))?;
        let dates = root["dates"]
            .as_object()
            .map(|dates| {
                let mut values = dates
                    .values()
                    .filter_map(serde_json::Value::as_str)
                    .map(str::to_string)
                    .collect::<Vec<_>>();
                values.sort();
                values
            })
            .unwrap_or_default();
        let list = root["datas"]
            .get(index)
            .and_then(|value| value["list"].as_array())
            .ok_or_else(|| {
                Error::not_found(format!(
                    "Eastmoney spot-stock category {category} has no data"
                ))
            })?;
        let mut result = Vec::new();
        for entry in list {
            let mut row = Row::new();
            let fields = entry.as_object().cloned().unwrap_or_default();
            for (key, value) in fields {
                let normalized = match key.as_str() {
                    "name" => "商品名称",
                    "scss" => "生产商",
                    "xyyhs" => "下游用户",
                    "price" => "最新价格",
                    "zf" => "近半年涨跌幅",
                    other => other,
                };
                let value = if normalized == "生产商" || normalized == "下游用户" {
                    if let Some(items) = value.as_array() {
                        if items.is_empty() {
                            serde_json::json!("-")
                        } else {
                            serde_json::json!(
                                items
                                    .iter()
                                    .filter_map(|item| item["name"].as_str())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )
                        }
                    } else {
                        value
                    }
                } else {
                    value
                };
                row.insert(normalized.to_string(), value);
            }
            for (position, date) in dates.iter().enumerate() {
                if let Some(value) = entry.get(format!("d{}", position + 1)) {
                    row.insert(date.clone(), value.clone());
                }
            }
            result.push(row);
        }
        Ok(result)
    }
}
