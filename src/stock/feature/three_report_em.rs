//! Per-stock financial reports (个股三大报表) from Eastmoney emweb.

use crate::client::AkShareClient;
use crate::error::Result;
use super::types::*;

impl AkShareClient {
    /// Per-stock financial report fetch helper using emweb API.
    async fn emweb_three_report(&self, symbol: &str, report_type: &str, report_date_type: &str, report_type_num: &str) -> Result<Vec<ThreeReportEntry>> {
        let code = symbol.to_lowercase();
        let url = format!(
            "https://emweb.securities.eastmoney.com/PC_HSF10/NewFinanceAnalysis/{}",
            report_type
        );
        let resp = self.get(&url)
            .query(&[
                ("companyType", "4"),
                ("reportDateType", report_date_type),
                ("reportType", report_type_num),
                ("code", &code),
            ])
            .send().await?
            .error_for_status()?
            .json::<serde_json::Value>().await?;

        let data = resp.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(data.iter().map(|v| ThreeReportEntry {
            report_date: v.get("REPORT_DATE").and_then(|d| d.as_str()).unwrap_or("").to_string(),
            data: v.clone(),
        }).collect())
    }

    /// 资产负债表-按报告期
    pub async fn stock_balance_sheet_by_report_em(&self, symbol: &str) -> Result<Vec<ThreeReportEntry>> {
        self.emweb_three_report(symbol, "zcfzbDateAjaxNew", "0", "1").await
    }

    /// 资产负债表-按年度
    pub async fn stock_balance_sheet_by_yearly_em(&self, symbol: &str) -> Result<Vec<ThreeReportEntry>> {
        self.emweb_three_report(symbol, "zcfzbDateAjaxNew", "1", "1").await
    }

    /// 利润表-按报告期
    pub async fn stock_profit_sheet_by_report_em(&self, symbol: &str) -> Result<Vec<ThreeReportEntry>> {
        self.emweb_three_report(symbol, "lrbDateAjaxNew", "0", "1").await
    }

    /// 利润表-按年度
    pub async fn stock_profit_sheet_by_yearly_em(&self, symbol: &str) -> Result<Vec<ThreeReportEntry>> {
        self.emweb_three_report(symbol, "lrbDateAjaxNew", "1", "1").await
    }

    /// 利润表-按单季度
    pub async fn stock_profit_sheet_by_quarterly_em(&self, symbol: &str) -> Result<Vec<ThreeReportEntry>> {
        self.emweb_three_report(symbol, "lrbDateAjaxNew", "0", "2").await
    }

    /// 现金流量表-按报告期
    pub async fn stock_cash_flow_sheet_by_report_em(&self, symbol: &str) -> Result<Vec<ThreeReportEntry>> {
        self.emweb_three_report(symbol, "xjllDateAjaxNew", "0", "1").await
    }

    /// 现金流量表-按年度
    pub async fn stock_cash_flow_sheet_by_yearly_em(&self, symbol: &str) -> Result<Vec<ThreeReportEntry>> {
        self.emweb_three_report(symbol, "xjllDateAjaxNew", "1", "1").await
    }

    /// 现金流量表-按单季度
    pub async fn stock_cash_flow_sheet_by_quarterly_em(&self, symbol: &str) -> Result<Vec<ThreeReportEntry>> {
        self.emweb_three_report(symbol, "xjllDateAjaxNew", "0", "2").await
    }
}
