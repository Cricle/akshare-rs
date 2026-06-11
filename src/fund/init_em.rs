//! New fund data from Eastmoney and THS.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::parse_f64_safe;

impl AkShareClient {
    /// Fetch newly established funds (Python: fund_new_found_em).
    pub async fn fund_new_found_em(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .get("https://fund.eastmoney.com/data/FundNewIssue.aspx")
            .query(&[
                ("t", "xcln"), ("sort", "jzrgq,desc"), ("y", ""),
                ("page", "1,50000"), ("isbuy", "1"),
            ])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        let json_str = text
            .strip_prefix("var newfunddata=")
            .unwrap_or(&text);
        let json_start = json_str.find('{').unwrap_or(0);
        let json_end = json_str.rfind('}').map(|i| i + 1).unwrap_or(json_str.len());
        let json_body = &json_str[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_body)
            .map_err(|e| Error::decode(format!("new fund JSON parse: {e}")))?;

        let datas = root.get("datas").and_then(|d| d.as_array())
            .ok_or_else(|| Error::not_found("no new fund data"))?;

        let mut result = Vec::new();
        for item in datas {
            let row = item.as_str().unwrap_or("");
            let fields: Vec<&str> = row.split(',').map(str::trim).collect();
            if fields.len() < 10 { continue; }
            result.push(serde_json::json!({
                "fund_code": fields[0],
                "fund_name": fields[1],
                "company": fields[2],
                "fund_type": fields[4],
                "subscribe_period": fields[9],
                "shares": parse_f64_safe(fields[5]),
                "found_date": fields[6],
                "return_since_found": parse_f64_safe(fields[7]),
                "manager": fields[8],
                "status": fields[9],
            }));
        }
        if result.is_empty() {
            return Err(Error::not_found("no new fund data"));
        }
        Ok(result)
    }

    /// Fetch new funds from THS (Python: fund_new_found_ths).
    pub async fn fund_new_found_ths(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let response = self
            .get("https://fund.10jqka.com.cn/datacenter/xfjj/")
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        // Extract jsonData from the HTML page
        let start_idx = text.find("jsonData=")
            .ok_or_else(|| Error::decode("THS page missing jsonData"))?;
        let start_bracket = text[start_idx..].find('{')
            .ok_or_else(|| Error::decode("THS page missing JSON start"))?;
        let abs_start = start_idx + start_bracket;

        // Find matching closing brace
        let mut count = 0i32;
        let mut end_idx = abs_start;
        for (i, ch) in text[abs_start..].char_indices() {
            if ch == '{' { count += 1; }
            else if ch == '}' {
                count -= 1;
                if count == 0 {
                    end_idx = abs_start + i + 1;
                    break;
                }
            }
        }

        let json_str = &text[abs_start..end_idx];
        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("THS new fund JSON parse: {e}")))?;

        let mut items: Vec<serde_json::Value> = if let Some(obj) = root.as_object() {
            obj.values().cloned().collect()
        } else if let Some(arr) = root.as_array() {
            arr.clone()
        } else {
            return Err(Error::decode("unexpected THS data format"));
        };

        // Filter by symbol
        if symbol == "发行中" {
            items.retain(|v| v.get("zzfx").and_then(|z| z.as_i64()).unwrap_or(0) == 1);
        } else if symbol == "将发行" {
            items.retain(|v| v.get("zzfx").and_then(|z| z.as_i64()).unwrap_or(0) != 1);
        }

        if items.is_empty() {
            return Err(Error::not_found("no THS new fund data"));
        }
        Ok(items)
    }
}
