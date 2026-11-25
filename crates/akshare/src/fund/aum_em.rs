//! Fund AUM (Assets Under Management) data from Eastmoney.

use super::html_tables;
use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FundAumCompanyItem, FundAumHistItem, FundAumTrendPoint};

impl AkShareClient {
    /// Fetch fund company AUM ranking (Python: fund_aum).
    pub async fn fund_aum(&self) -> Result<Vec<FundAumCompanyItem>> {
        let response = crate::util::send_and_check(
            self.get("https://fund.eastmoney.com/Company/home/gspmlist")
                .query(&[("fundType", "0")]),
        )
        .await?;

        let text = response.text().await.map_err(Error::from)?;
        let table = html_tables(&text)
            .into_iter()
            .find(|table| {
                table
                    .iter()
                    .any(|row| row.iter().any(|cell| cell.contains("基金公司")))
            })
            .ok_or_else(|| Error::decode("fund AUM company table not found"))?;
        let header = table
            .iter()
            .position(|row| row.iter().any(|cell| cell.contains("基金公司")))
            .ok_or_else(|| Error::decode("fund AUM company header not found"))?;
        let headers = &table[header];
        let at = |needle: &str| headers.iter().position(|cell| cell.contains(needle));
        let rank_i = at("序号");
        let company_i =
            at("基金公司").ok_or_else(|| Error::decode("fund AUM company column missing"))?;
        let found_i = at("成立时间");
        let scale_i = at("全部管理规模");
        let fund_i = at("全部基金数");
        let manager_i = at("全部经理数");
        let mut result = Vec::new();
        for (offset, row) in table.iter().skip(header + 1).enumerate() {
            let company = row.get(company_i).map(String::as_str).unwrap_or("").trim();
            if company.is_empty() {
                continue;
            }
            let scale_raw = scale_i
                .and_then(|i| row.get(i))
                .map(String::as_str)
                .unwrap_or("");
            let mut scale_parts = scale_raw.split_whitespace();
            let total_scale = scale_parts
                .next()
                .unwrap_or("")
                .replace(',', "")
                .parse()
                .unwrap_or(0.0);
            let update_date = scale_parts.next().unwrap_or("").to_string();
            let parse_i32 = |idx: Option<usize>| {
                idx.and_then(|index| row.get(index))
                    .and_then(|value| value.replace(',', "").parse().ok())
                    .unwrap_or(0)
            };
            result.push(FundAumCompanyItem {
                rank: rank_i
                    .and_then(|i| row.get(i))
                    .and_then(|v| v.parse().ok())
                    .unwrap_or((offset + 1) as i32),
                company: company.to_string(),
                found_date: found_i
                    .and_then(|i| row.get(i))
                    .cloned()
                    .unwrap_or_default(),
                total_scale,
                fund_count: parse_i32(fund_i),
                manager_count: parse_i32(manager_i),
                update_date,
            });
        }
        if result.is_empty() {
            return Err(Error::not_found("no fund AUM company rows"));
        }
        Ok(result)
    }

    /// Fetch fund market AUM trend (Python: fund_aum_trend).
    pub async fn fund_aum_trend(&self) -> Result<Vec<FundAumTrendPoint>> {
        let response = crate::util::send_and_check(
            self.get("https://fund.eastmoney.com/Company/home/GetFundTotalScaleForChart"),
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let x = payload
            .get("x")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::not_found("no AUM trend data"))?;
        let y = payload
            .get("y")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::not_found("no AUM trend values"))?;

        let mut result = Vec::new();
        for (date, value) in x.iter().zip(y.iter()) {
            result.push(FundAumTrendPoint {
                date: date.as_str().unwrap_or("").to_string(),
                value: value.as_f64().unwrap_or(0.0),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found("no AUM trend data"));
        }
        Ok(result)
    }

    /// Fetch fund company historical AUM ranking (Python: fund_aum_hist).
    pub async fn fund_aum_hist(&self, year: &str) -> Result<Vec<FundAumHistItem>> {
        let response = crate::util::send_and_check(
            self.get("https://fund.eastmoney.com/Company/home/HistoryScaleTable")
                .query(&[("year", year)]),
        )
        .await?;

        let text = response.text().await.map_err(Error::from)?;
        let table = html_tables(&text)
            .into_iter()
            .find(|table| {
                table.iter().any(|row| {
                    row.iter().any(|cell| cell.contains("基金公司"))
                        && row.iter().any(|cell| cell.contains("总规模"))
                })
            })
            .ok_or_else(|| Error::decode("fund AUM history table not found"))?;
        let header = table
            .iter()
            .position(|row| {
                row.iter().any(|cell| cell.contains("基金公司"))
                    && row.iter().any(|cell| cell.contains("总规模"))
            })
            .ok_or_else(|| Error::decode("fund AUM history header not found"))?;
        let headers = &table[header];
        let at = |needle: &str| headers.iter().position(|cell| cell.contains(needle));
        let company_i = at("基金公司")
            .ok_or_else(|| Error::decode("fund AUM history company column missing"))?;
        let parse = |row: &[String], idx: Option<usize>| {
            idx.and_then(|index| row.get(index))
                .map(|v| v.replace(',', "").parse().unwrap_or(0.0))
                .unwrap_or(0.0)
        };
        let mut result = Vec::new();
        for (offset, row) in table.iter().skip(header + 1).enumerate() {
            let company = row.get(company_i).map(String::as_str).unwrap_or("").trim();
            if company.is_empty() {
                continue;
            }
            result.push(FundAumHistItem {
                rank: at("序号")
                    .and_then(|i| row.get(i))
                    .and_then(|v| v.parse().ok())
                    .unwrap_or((offset + 1) as i32),
                company: company.to_string(),
                total: parse(row, at("总规模")),
                equity: parse(row, at("股票")),
                mixed: parse(row, at("混合")),
                bond: parse(row, at("债券")),
                index: parse(row, at("指数")),
                qdii: parse(row, at("QDII")),
                money: parse(row, at("货币")),
            });
        }
        if result.is_empty() {
            return Err(Error::not_found(format!("no AUM history rows for {year}")));
        }
        Ok(result)
    }
}
