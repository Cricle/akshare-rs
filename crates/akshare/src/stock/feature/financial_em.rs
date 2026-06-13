//! Financial reports (三大报表) from Eastmoney.

use super::helpers::{json_f64_opt, json_str, json_str_opt};
use super::types::{BalanceSheet, CashFlowSheet, ProfitSheet};
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 资产负债表 (typed version; for raw JSON see `stock_balance_sheet_by_report_em` in three_report_em)
    pub async fn stock_balance_sheet_by_report_em_typed(
        &self,
        symbol: &str,
    ) -> Result<Vec<BalanceSheet>> {
        let data = self.emweb_financial_fetch(symbol, "zcfz", "0").await?;
        Ok(data
            .iter()
            .map(|v| BalanceSheet {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                notice_date: json_str_opt(v, "NOTICE_DATE"),
                total_assets: json_f64_opt(v, "TOTAL_ASSETS"),
                total_liabilities: json_f64_opt(v, "TOTAL_LIABILITIES"),
                equity: json_f64_opt(v, "TOTAL_EQUITY"),
                cash: json_f64_opt(v, "MONETARYFUNDS"),
                accounts_receivable: json_f64_opt(v, "ACCOUNTS_RECE"),
                inventory: json_f64_opt(v, "INVENTORY"),
                accounts_payable: json_f64_opt(v, "ACCOUNTS_PAYABLE"),
                advance_receipts: json_f64_opt(v, "ADVANCE_RECEIVABLES"),
                total_assets_yoy: json_f64_opt(v, "TOTAL_ASSETS_YOY"),
                total_liabilities_yoy: json_f64_opt(v, "TOTAL_LIABILITIES_YOY"),
                debt_ratio: json_f64_opt(v, "DEBT_ASSET_RATIO"),
                long_term_debt: None,
                short_term_debt: None,
            })
            .collect())
    }

    /// 利润表 (typed version)
    pub async fn stock_profit_sheet_by_report_em_typed(
        &self,
        symbol: &str,
    ) -> Result<Vec<ProfitSheet>> {
        let data = self.emweb_financial_fetch(symbol, "lrb", "0").await?;
        Ok(data
            .iter()
            .map(|v| ProfitSheet {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                notice_date: json_str_opt(v, "NOTICE_DATE"),
                total_revenue: json_f64_opt(v, "TOTAL_OPERATE_INCOME"),
                operating_cost: json_f64_opt(v, "OPERATE_COST"),
                operating_profit: json_f64_opt(v, "OPERATE_PROFIT"),
                total_profit: json_f64_opt(v, "TOTAL_PROFIT"),
                net_profit: json_f64_opt(v, "NETPROFIT"),
                net_profit_deducted: json_f64_opt(v, "DEDUCT_PARENT_NETPROFIT"),
                total_revenue_yoy: json_f64_opt(v, "TOTAL_OPERATE_INCOME_YOY"),
                net_profit_yoy: json_f64_opt(v, "PARENT_NETPROFIT_YOY"),
                gross_margin: json_f64_opt(v, "GROSS_PROFIT_RATIO"),
                net_margin: json_f64_opt(v, "NET_PROFIT_RATIO"),
                roe: json_f64_opt(v, "ROE"),
                eps: json_f64_opt(v, "BASIC_EPS"),
                gross_profit: None,
                operating_expenses: None,
            })
            .collect())
    }

    /// 现金流量表 (typed version)
    pub async fn stock_cash_flow_sheet_by_report_em_typed(
        &self,
        symbol: &str,
    ) -> Result<Vec<CashFlowSheet>> {
        let data = self.emweb_financial_fetch(symbol, "xjll", "0").await?;
        Ok(data
            .iter()
            .map(|v| CashFlowSheet {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                notice_date: json_str_opt(v, "NOTICE_DATE"),
                operating_cash_flow: json_f64_opt(v, "SALES_SERVICES"),
                investing_cash_flow: json_f64_opt(v, "INVEST_PAY_CASH"),
                financing_cash_flow: json_f64_opt(v, "ASSIGN_DIVIDEND_PORFIT"),
                cash_increase: json_f64_opt(v, "CCE_ADD"),
                operating_cash_flow_yoy: json_f64_opt(v, "SALES_SERVICES_YOY"),
                capital_expenditure: None,
            })
            .collect())
    }
}
