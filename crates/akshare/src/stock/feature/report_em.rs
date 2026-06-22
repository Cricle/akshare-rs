//! Datacenter financial reports (三大报表-数据中心) from Eastmoney.

use super::helpers::{json_f64_opt, json_str, json_str_opt};
use crate::util::fmt_date;
use super::types::{BalanceSheet, CashFlowSheet, ProfitSheet};
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 资产负债表(数据中心)
    pub async fn stock_zcfz(&self, date: &str) -> Result<Vec<BalanceSheet>> {
        let date_fmt = fmt_date(date);
        let filter = format!(
            "(SECURITY_TYPE_CODE in (\"058001001\",\"058001008\"))(TRADE_MARKET_CODE!=\"069001017\")(REPORT_DATE='{date_fmt}')"
        );
        let data = self
            .dc_fetch_all(
                "RPT_DMSK_FN_BALANCE",
                "ALL",
                &filter,
                "NOTICE_DATE,SECURITY_CODE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
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

    /// 利润表(数据中心)
    pub async fn stock_lrb(&self, date: &str) -> Result<Vec<ProfitSheet>> {
        let date_fmt = fmt_date(date);
        let filter = format!(
            "(SECURITY_TYPE_CODE in (\"058001001\",\"058001008\"))(TRADE_MARKET_CODE!=\"069001017\")(REPORT_DATE='{date_fmt}')"
        );
        let data = self
            .dc_fetch_all(
                "RPT_DMSK_FN_INCOME",
                "ALL",
                &filter,
                "NOTICE_DATE,SECURITY_CODE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
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

    /// 现金流量表(数据中心)
    pub async fn stock_xjll(&self, date: &str) -> Result<Vec<CashFlowSheet>> {
        let date_fmt = fmt_date(date);
        let filter = format!(
            "(SECURITY_TYPE_CODE in (\"058001001\",\"058001008\"))(TRADE_MARKET_CODE!=\"069001017\")(REPORT_DATE='{date_fmt}')"
        );
        let data = self
            .dc_fetch_all(
                "RPT_DMSK_FN_CASHFLOW",
                "ALL",
                &filter,
                "NOTICE_DATE,SECURITY_CODE",
                "-1",
                500,
                10,
                &[],
            )
            .await?;
        Ok(data
            .iter()
            .map(|v| CashFlowSheet {
                code: json_str(v, "SECURITY_CODE"),
                name: json_str(v, "SECURITY_NAME_ABBR"),
                notice_date: json_str_opt(v, "NOTICE_DATE"),
                operating_cash_flow: json_f64_opt(v, "NETCASH_OPERATE"),
                investing_cash_flow: json_f64_opt(v, "NETCASH_INVEST"),
                financing_cash_flow: json_f64_opt(v, "NETCASH_FINANCE"),
                cash_increase: json_f64_opt(v, "CCE_ADD"),
                operating_cash_flow_yoy: json_f64_opt(v, "NETCASH_OPERATE_YOY"),
                capital_expenditure: None,
            })
            .collect())
    }
}
