//! Industry data (行业数据) from CNINFO and Shenwan.

use super::helpers::{json_str, json_str_opt};
use super::types::{IndustryCategory, IndustryChange, IndustryClfHistSw, IndustryPeRatio};
use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::util::{read_spreadsheet_sheet_rows, zero_pad_code};

impl AkShareClient {
    /// 巨潮-行业分类
    pub async fn stock_industry_category(&self) -> Result<Vec<IndustryCategory>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1125";
        let resp =
            crate::util::send_and_check(self.post(url).header("User-Agent", "Mozilla/5.0")).await?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json
            .get("records")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(records
            .iter()
            .map(|v| IndustryCategory {
                code: json_str(v, "INDUSTRYCODE"),
                name: json_str(v, "INDUSTRYNAME"),
                industry: json_str_opt(v, "CATALOGNAME"),
                extra: None,
            })
            .collect())
    }

    /// 巨潮-行业变动
    pub async fn stock_industry_change(&self, symbol: &str) -> Result<Vec<IndustryChange>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1126";
        let resp = crate::util::send_and_check(
            self.post(url)
                .form(&[("indcode", symbol)])
                .header("User-Agent", "Mozilla/5.0"),
        )
        .await?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json
            .get("records")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(records
            .iter()
            .map(|v| IndustryChange { data: v.clone() })
            .collect())
    }

    /// 申万-行业分类历史
    ///
    /// Upstream downloads `StockClassifyUse_stock.xls`; the optional Rust
    /// symbol filters the parsed workbook while preserving the upstream rows.
    pub async fn stock_industry_clf_hist_sw(&self, symbol: &str) -> Result<Vec<IndustryClfHistSw>> {
        let response = self
            .get("https://www.swsresearch.com/swindex/pdf/SwClass2021/StockClassifyUse_stock.xls")
            .header("User-Agent", "Mozilla/5.0")
            .header(
                "Accept",
                "application/vnd.ms-excel,application/octet-stream;q=0.9,*/*;q=0.8",
            )
            .send()
            .await
            .map_err(|err| {
                Error::unavailable(format!("SW industry workbook request failed: {err}"))
            })?
            .error_for_status()
            .map_err(|err| {
                Error::unavailable(format!("SW industry workbook unavailable: {err}"))
            })?;
        let bytes = response.bytes().await.map_err(Error::from)?;
        let rows = read_spreadsheet_sheet_rows(&bytes, None)?;
        let header_index = rows
            .iter()
            .position(|row| row.iter().any(|cell| cell.trim() == "股票代码"))
            .ok_or_else(|| Error::decode("SW industry workbook has no 股票代码 header"))?;
        let headers = rows[header_index]
            .iter()
            .map(|cell| cell.trim().to_string())
            .collect::<Vec<_>>();
        let column = |name: &str| headers.iter().position(|header| header == name);
        let code_index = column("股票代码")
            .ok_or_else(|| Error::decode("SW industry workbook has no 股票代码 column"))?;
        let start_index = column("计入日期");
        let industry_index = column("行业代码");
        let update_index = column("更新日期");
        let requested = symbol.trim();
        let mut output = Vec::new();
        for row in rows.iter().skip(header_index + 1) {
            let raw_code = row.get(code_index).map(String::as_str).unwrap_or("");
            let code = zero_pad_code(raw_code);
            if code.is_empty() || (!requested.is_empty() && code != zero_pad_code(requested)) {
                continue;
            }
            let mut data = serde_json::Map::new();
            data.insert("symbol".into(), serde_json::json!(code));
            data.insert(
                "start_date".into(),
                serde_json::json!(
                    start_index
                        .and_then(|i| row.get(i))
                        .map(String::as_str)
                        .unwrap_or("")
                ),
            );
            data.insert(
                "industry_code".into(),
                serde_json::json!(
                    industry_index
                        .and_then(|i| row.get(i))
                        .map(String::as_str)
                        .unwrap_or("")
                ),
            );
            data.insert(
                "update_time".into(),
                serde_json::json!(
                    update_index
                        .and_then(|i| row.get(i))
                        .map(String::as_str)
                        .unwrap_or("")
                ),
            );
            output.push(IndustryClfHistSw {
                data: serde_json::Value::Object(data),
            });
        }
        if output.is_empty() {
            return Err(Error::not_found(
                "SW industry workbook returned no matching rows",
            ));
        }
        Ok(output)
    }

    /// 巨潮-行业市盈率
    pub async fn stock_industry_pe_ratio(&self, symbol: &str) -> Result<Vec<IndustryPeRatio>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1131";
        let resp = crate::util::send_and_check(
            self.post(url)
                .form(&[("indcode", symbol)])
                .header("User-Agent", "Mozilla/5.0"),
        )
        .await?;
        let json: serde_json::Value = resp.json().await.map_err(Error::from)?;
        let records = json
            .get("records")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(records
            .iter()
            .map(|v| IndustryPeRatio { data: v.clone() })
            .collect())
    }
}
