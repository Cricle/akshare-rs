use std::collections::HashMap;

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) struct AkshareIndividualInfo {
    pub(crate) stock_name: Option<String>,
    pub(crate) total_share: Option<i64>,
    pub(crate) market_cap: Option<f64>,
    pub(crate) industry: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct SecTickerEntry {
    pub(crate) cik: String,
    pub(crate) title: String,
}

pub(crate) type SecTickerLookup = HashMap<String, SecTickerEntryRaw>;

#[derive(Debug, Deserialize)]
pub(crate) struct TushareResponse {
    pub(crate) code: i32,
    pub(crate) msg: Option<String>,
    pub(crate) data: Option<TushareData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneySearchEnvelope {
    #[serde(rename = "QuotationCodeTable")]
    pub(crate) quotation_code_table: Option<EastmoneySearchTable>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneySearchTable {
    #[serde(rename = "Data")]
    pub(crate) data: Option<Vec<EastmoneySearchItem>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneySearchItem {
    #[serde(rename = "Code")]
    pub(crate) code: Option<String>,
    #[serde(rename = "Name")]
    pub(crate) name: Option<String>,
    #[serde(rename = "SecurityTypeName")]
    pub(crate) security_type_name: Option<String>,
    #[serde(rename = "JYS")]
    pub(crate) exchange: Option<String>,
    #[serde(rename = "Classify")]
    pub(crate) classify: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyKlineEnvelope {
    pub(crate) data: Option<EastmoneyKlineData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyKlineData {
    #[allow(dead_code)]
    pub(crate) code: Option<String>,
    #[allow(dead_code)]
    pub(crate) name: Option<String>,
    pub(crate) klines: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneySectorRankingEnvelope {
    pub(crate) data: Option<EastmoneySectorRankingData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneySectorRankingData {
    #[allow(dead_code)]
    pub(crate) total: Option<i64>,
    pub(crate) diff: Option<Vec<EastmoneySectorRankingItem>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneySectorRankingItem {
    #[serde(rename = "f12")]
    pub(crate) sector_code: Option<String>,
    #[serde(rename = "f14")]
    pub(crate) sector_name: Option<String>,
    #[serde(rename = "f2")]
    pub(crate) latest_index: Option<f64>,
    #[serde(rename = "f3")]
    pub(crate) change_pct: Option<f64>,
    #[serde(rename = "f62")]
    pub(crate) main_net_inflow: Option<f64>,
    #[serde(rename = "f184")]
    pub(crate) main_net_inflow_ratio_pct: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneySectorConstituentEnvelope {
    pub(crate) data: Option<EastmoneySectorConstituentData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneySectorConstituentData {
    #[allow(dead_code)]
    pub(crate) total: Option<i64>,
    pub(crate) diff: Option<Vec<EastmoneySectorConstituentItem>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneySectorConstituentItem {
    #[serde(rename = "f12")]
    pub(crate) symbol: Option<String>,
    #[serde(rename = "f14")]
    pub(crate) name: Option<String>,
    #[serde(rename = "f2")]
    pub(crate) latest_price: Option<f64>,
    #[serde(rename = "f3")]
    pub(crate) change_pct: Option<f64>,
    #[serde(rename = "f62")]
    pub(crate) main_net_inflow: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TushareData {
    pub(crate) fields: Vec<String>,
    pub(crate) items: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyAnnouncementsEnvelope {
    pub(crate) data: Option<EastmoneyAnnouncementsData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyAnnouncementsData {
    pub(crate) list: Option<Vec<EastmoneyAnnouncementItem>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyAnnouncementItem {
    pub(crate) art_code: Option<String>,
    pub(crate) notice_date: Option<String>,
    pub(crate) title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyAnnouncementContentEnvelope {
    pub(crate) data: Option<EastmoneyAnnouncementContentData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyAnnouncementContentData {
    pub(crate) art_code: Option<String>,
    pub(crate) notice_title: Option<String>,
    pub(crate) notice_date: Option<String>,
    pub(crate) notice_content: Option<String>,
    pub(crate) attach_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyDatacenterEnvelope<T> {
    pub(crate) result: Option<EastmoneyDatacenterResult<T>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyDatacenterResult<T> {
    pub(crate) data: Option<Vec<T>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyMainFinanceIndicatorItem {
    #[allow(dead_code)]
    #[serde(rename = "SECUCODE")]
    pub(crate) secucode: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "SECURITY_CODE")]
    pub(crate) security_code: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "SECURITY_NAME_ABBR")]
    pub(crate) security_name_abbr: Option<String>,
    #[serde(rename = "REPORT_DATE")]
    pub(crate) report_date: Option<String>,
    #[serde(rename = "STD_REPORT_DATE")]
    pub(crate) std_report_date: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "REPORT_TYPE")]
    pub(crate) report_type: Option<String>,
    #[serde(rename = "CURRENCY")]
    pub(crate) currency: Option<String>,
    #[serde(rename = "OPERATE_INCOME")]
    pub(crate) operate_income: Option<f64>,
    #[serde(rename = "TOTALOPERATEREVE")]
    pub(crate) total_operate_reve: Option<f64>,
    #[serde(rename = "GROSS_PROFIT")]
    pub(crate) gross_profit: Option<f64>,
    #[serde(rename = "MLR")]
    pub(crate) mlr: Option<f64>,
    #[serde(rename = "HOLDER_PROFIT")]
    pub(crate) holder_profit: Option<f64>,
    #[serde(rename = "PARENTNETPROFIT")]
    pub(crate) parent_net_profit: Option<f64>,
    #[serde(rename = "TOTAL_ASSETS")]
    pub(crate) total_assets: Option<f64>,
    #[serde(rename = "TOTALASSETS")]
    pub(crate) totalassets: Option<f64>,
    #[serde(rename = "TOTAL_LIABILITIES")]
    pub(crate) total_liabilities: Option<f64>,
    #[serde(rename = "TOTLIAB")]
    pub(crate) totliab: Option<f64>,
    #[serde(rename = "TOTAL_PARENT_EQUITY")]
    pub(crate) total_parent_equity: Option<f64>,
    #[serde(rename = "NETCASH_OPERATE")]
    pub(crate) netcash_operate: Option<f64>,
    #[serde(rename = "MGJYXJJE")]
    pub(crate) mgjyxjje: Option<f64>,
    #[serde(rename = "BPS")]
    pub(crate) bps: Option<f64>,
    #[serde(rename = "ZCFZL")]
    pub(crate) zcfzl: Option<f64>,
    #[serde(rename = "CURRENT_LIABILITY")]
    pub(crate) current_liability: Option<f64>,
    #[serde(rename = "CURRENT_LIAB")]
    pub(crate) current_liab: Option<f64>,
    #[serde(rename = "NONCURRENT_LIAB_1YEAR")]
    pub(crate) noncurrent_liab_1year: Option<f64>,
    #[serde(rename = "TOTALNONCLIAB")]
    pub(crate) totalnoncliab: Option<f64>,
    #[serde(rename = "CAPITAL_EXPENDITURE")]
    pub(crate) capital_expenditure: Option<f64>,
    #[serde(rename = "TOTAL_SHARE")]
    pub(crate) total_share: Option<f64>,
    #[allow(dead_code)]
    #[serde(rename = "BASIC_EPS")]
    pub(crate) basic_eps: Option<f64>,
    #[allow(dead_code)]
    #[serde(rename = "DILUTED_EPS")]
    pub(crate) diluted_eps: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyBalanceSheetItem {
    #[allow(dead_code)]
    #[serde(rename = "REPORT_DATE")]
    pub(crate) report_date: Option<String>,
    #[serde(rename = "TOTAL_ASSETS")]
    pub(crate) total_assets: Option<f64>,
    #[serde(rename = "TOTAL_LIABILITIES")]
    pub(crate) total_liabilities: Option<f64>,
    #[serde(rename = "TOTAL_EQUITY")]
    pub(crate) total_equity: Option<f64>,
    #[serde(rename = "MONETARYFUNDS")]
    pub(crate) monetary_funds: Option<f64>,
    #[serde(rename = "CURRENT_LIAB")]
    pub(crate) current_liab: Option<f64>,
    #[serde(rename = "TOTALNONCLIAB")]
    pub(crate) totalnoncliab: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyCashflowItem {
    #[allow(dead_code)]
    #[serde(rename = "REPORT_DATE")]
    pub(crate) report_date: Option<String>,
    #[serde(rename = "NETCASH_OPERATE")]
    pub(crate) netcash_operate: Option<f64>,
    #[serde(rename = "CONSTRUCT_LONG_ASSET")]
    pub(crate) construct_long_asset: Option<f64>,
    #[serde(rename = "END_CCE")]
    pub(crate) end_cce: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyFinancialStatementItem {
    #[serde(rename = "STD_REPORT_DATE")]
    pub(crate) std_report_date: Option<String>,
    #[serde(rename = "ITEM_NAME")]
    pub(crate) item_name: Option<String>,
    #[serde(rename = "AMOUNT")]
    pub(crate) amount: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyBillboardEntryItem {
    #[serde(rename = "TRADE_DATE")]
    pub(crate) trade_date: Option<String>,
    #[serde(rename = "SECURITY_CODE")]
    pub(crate) security_code: Option<String>,
    #[serde(rename = "SECURITY_NAME_ABBR")]
    pub(crate) security_name: Option<String>,
    #[serde(rename = "CLOSE_PRICE")]
    pub(crate) close_price: Option<f64>,
    #[serde(rename = "CHANGE_RATE")]
    pub(crate) change_rate: Option<f64>,
    #[serde(rename = "TURNOVERRATE")]
    pub(crate) turnover_rate: Option<f64>,
    #[serde(rename = "BILLBOARD_NET_AMT")]
    pub(crate) net_amount: Option<f64>,
    #[serde(rename = "BILLBOARD_BUY_AMT")]
    pub(crate) buy_amount: Option<f64>,
    #[serde(rename = "BILLBOARD_SELL_AMT")]
    pub(crate) sell_amount: Option<f64>,
    #[serde(rename = "EXPLAIN")]
    pub(crate) explain: Option<String>,
    #[serde(rename = "EXPLANATION")]
    pub(crate) explanation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EastmoneyBillboardSeatItem {
    #[serde(rename = "TRADE_DATE")]
    pub(crate) trade_date: Option<String>,
    #[serde(rename = "SECURITY_CODE")]
    pub(crate) security_code: Option<String>,
    #[serde(rename = "OPERATEDEPT_NAME")]
    pub(crate) department_name: Option<String>,
    #[serde(rename = "BUY")]
    pub(crate) buy_amount: Option<f64>,
    #[serde(rename = "SELL")]
    pub(crate) sell_amount: Option<f64>,
    #[serde(rename = "NET")]
    pub(crate) net_amount: Option<f64>,
    #[serde(rename = "EXPLANATION")]
    pub(crate) explanation: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct TushareRow {
    values: HashMap<String, serde_json::Value>,
}
impl TushareRow {
    pub(crate) fn new(fields: &[String], items: Vec<serde_json::Value>) -> Self {
        let values = fields.iter().cloned().zip(items).collect::<HashMap<_, _>>();
        Self { values }
    }

    pub(crate) fn string(&self, key: &str) -> anyhow::Result<String> {
        self.optional_string(key)
            .context(format!("missing string field {}", key))
    }

    pub(crate) fn optional_string(&self, key: &str) -> Option<String> {
        self.values.get(key).and_then(|value| match value {
            serde_json::Value::String(value) => Some(value.clone()),
            serde_json::Value::Number(value) => Some(value.to_string()),
            serde_json::Value::Null => None,
            other => Some(other.to_string()),
        })
    }

    pub(crate) fn f64(&self, key: &str) -> anyhow::Result<f64> {
        self.optional_f64(key)
            .context(format!("missing numeric field {}", key))
    }

    pub(crate) fn optional_f64(&self, key: &str) -> Option<f64> {
        self.values.get(key).and_then(|value| match value {
            serde_json::Value::Number(value) => value.as_f64(),
            serde_json::Value::String(value) => value.parse().ok(),
            _ => None,
        })
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct CompanyFactsResponse {
    #[serde(rename = "fiscalYearEnd")]
    pub(crate) fiscal_year_end: Option<String>,
    pub(crate) facts: CompanyFacts,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CompanyFacts {
    pub(crate) dei: Option<DeiFacts>,
    #[serde(rename = "us-gaap")]
    pub(crate) us_gaap: Option<UsGaapFacts>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DeiFacts {
    #[serde(rename = "EntityCommonStockSharesOutstanding")]
    pub(crate) entity_common_stock_shares_outstanding: Option<Metric>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UsGaapFacts {
    #[serde(rename = "NetIncomeLoss")]
    pub(crate) net_income_loss: Option<Metric>,
    #[serde(rename = "ProfitLoss")]
    pub(crate) profit_loss: Option<Metric>,
    #[serde(rename = "Revenues")]
    pub(crate) revenues: Option<Metric>,
    #[serde(rename = "RevenueFromContractWithCustomerExcludingAssessedTax")]
    pub(crate) revenue_from_contract_with_customer_excluding_assessed_tax: Option<Metric>,
    #[serde(rename = "SalesRevenueNet")]
    pub(crate) sales_revenue_net: Option<Metric>,
    #[serde(rename = "Assets")]
    pub(crate) assets: Option<Metric>,
    #[serde(rename = "AssetsCurrent")]
    pub(crate) assets_current: Option<Metric>,
    #[serde(rename = "Liabilities")]
    pub(crate) liabilities: Option<Metric>,
    #[serde(rename = "LiabilitiesCurrent")]
    pub(crate) liabilities_current: Option<Metric>,
    #[serde(rename = "StockholdersEquity")]
    pub(crate) stockholders_equity: Option<Metric>,
    #[serde(rename = "CashAndCashEquivalentsAtCarryingValue")]
    pub(crate) cash_and_cash_equivalents_at_carrying_value: Option<Metric>,
    #[serde(rename = "CashCashEquivalentsRestrictedCashAndRestrictedCashEquivalents")]
    pub(crate) cash_cash_equivalents_restricted_cash_and_restricted_cash_equivalents:
        Option<Metric>,
    #[serde(rename = "GrossProfit")]
    pub(crate) gross_profit: Option<Metric>,
    #[serde(rename = "OperatingIncomeLoss")]
    pub(crate) operating_income_loss: Option<Metric>,
    #[serde(rename = "OperatingExpenses")]
    pub(crate) operating_expenses: Option<Metric>,
    #[serde(rename = "NetCashProvidedByUsedInOperatingActivities")]
    pub(crate) net_cash_provided_by_used_in_operating_activities: Option<Metric>,
    #[serde(rename = "PaymentsToAcquirePropertyPlantAndEquipment")]
    pub(crate) payments_to_acquire_property_plant_and_equipment: Option<Metric>,
    #[serde(rename = "LongTermDebtNoncurrent")]
    pub(crate) long_term_debt_noncurrent: Option<Metric>,
    #[serde(rename = "LongTermDebtCurrent")]
    pub(crate) long_term_debt_current: Option<Metric>,
    #[serde(rename = "LongTermDebt")]
    pub(crate) long_term_debt: Option<Metric>,
    #[serde(rename = "WeightedAverageNumberOfDilutedSharesOutstanding")]
    pub(crate) weighted_average_number_of_diluted_shares_outstanding: Option<Metric>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Metric {
    pub(crate) units: MetricUnits,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MetricUnits {
    #[serde(default)]
    #[serde(rename = "USD")]
    pub(crate) usd: Option<Vec<MetricValue>>,
    #[serde(default)]
    pub(crate) shares: Option<Vec<MetricValue>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MetricValue {
    pub(crate) val: f64,
    pub(crate) filed: String,
    #[serde(default)]
    pub(crate) end: Option<String>,
    #[serde(default)]
    pub(crate) start: Option<String>,
    #[serde(default)]
    pub(crate) fp: Option<String>,
    pub(crate) form: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct AnnualMetricSnapshot {
    pub(crate) start: String,
    pub(crate) end: String,
    pub(crate) fp: Option<String>,
    pub(crate) form: Option<String>,
}

impl UsGaapFacts {
    pub(crate) fn latest_annual_snapshot(&self) -> Option<AnnualMetricSnapshot> {
        self.latest_annual_revenue_metric()
            .or_else(|| {
                [
                    self.net_income_loss.as_ref(),
                    self.profit_loss.as_ref(),
                    self.gross_profit.as_ref(),
                    self.operating_income_loss.as_ref(),
                    self.operating_expenses.as_ref(),
                    self.net_cash_provided_by_used_in_operating_activities
                        .as_ref(),
                ]
                .into_iter()
                .flatten()
                .filter_map(|metric| latest_strict_annual_metric(&metric.units))
                .max_by_key(|item| {
                    (
                        item.end.as_deref().unwrap_or_default(),
                        item.filed.as_str(),
                        item.fp.as_deref().unwrap_or_default(),
                    )
                })
            })
            .map(AnnualMetricSnapshot::from_metric_value)
    }

    pub(crate) fn annual_revenue_aligned(
        &self,
        snapshot: Option<&AnnualMetricSnapshot>,
    ) -> Option<f64> {
        first_metric_value_annual_aligned(
            [
                self.revenue_from_contract_with_customer_excluding_assessed_tax
                    .as_ref(),
                self.sales_revenue_net.as_ref(),
                self.revenues.as_ref(),
            ],
            snapshot,
        )
    }

    pub(crate) fn annual_net_income_aligned(
        &self,
        snapshot: Option<&AnnualMetricSnapshot>,
    ) -> Option<f64> {
        first_metric_value_annual_aligned(
            [self.net_income_loss.as_ref(), self.profit_loss.as_ref()],
            snapshot,
        )
    }

    pub(crate) fn latest_assets(&self) -> Option<f64> {
        first_metric_value_instant([self.assets.as_ref(), self.assets_current.as_ref()])
    }

    pub(crate) fn latest_liabilities(&self) -> Option<f64> {
        first_metric_value_instant([self.liabilities.as_ref(), self.liabilities_current.as_ref()])
    }

    pub(crate) fn latest_stockholders_equity(&self) -> Option<f64> {
        first_metric_value_instant([self.stockholders_equity.as_ref()])
    }

    pub(crate) fn latest_cash_and_equivalents(&self) -> Option<f64> {
        first_metric_value_instant([
            self.cash_and_cash_equivalents_at_carrying_value.as_ref(),
            self.cash_cash_equivalents_restricted_cash_and_restricted_cash_equivalents
                .as_ref(),
        ])
    }

    pub(crate) fn annual_gross_profit_aligned(
        &self,
        snapshot: Option<&AnnualMetricSnapshot>,
    ) -> Option<f64> {
        first_metric_value_annual_aligned([self.gross_profit.as_ref()], snapshot)
    }

    pub(crate) fn annual_operating_income_aligned(
        &self,
        snapshot: Option<&AnnualMetricSnapshot>,
    ) -> Option<f64> {
        first_metric_value_annual_aligned([self.operating_income_loss.as_ref()], snapshot)
    }

    pub(crate) fn annual_operating_expenses_aligned(
        &self,
        snapshot: Option<&AnnualMetricSnapshot>,
    ) -> Option<f64> {
        first_metric_value_annual_aligned([self.operating_expenses.as_ref()], snapshot)
    }

    pub(crate) fn annual_operating_cash_flow_aligned(
        &self,
        snapshot: Option<&AnnualMetricSnapshot>,
    ) -> Option<f64> {
        first_metric_value_annual_aligned(
            [self
                .net_cash_provided_by_used_in_operating_activities
                .as_ref()],
            snapshot,
        )
    }

    pub(crate) fn annual_capital_expenditure_aligned(
        &self,
        snapshot: Option<&AnnualMetricSnapshot>,
    ) -> Option<f64> {
        first_metric_value_annual_aligned(
            [self
                .payments_to_acquire_property_plant_and_equipment
                .as_ref()],
            snapshot,
        )
        .map(f64::abs)
    }

    pub(crate) fn latest_long_term_debt(&self) -> Option<f64> {
        first_metric_value_instant([
            self.long_term_debt_noncurrent.as_ref(),
            self.long_term_debt.as_ref(),
        ])
    }

    pub(crate) fn latest_current_debt(&self) -> Option<f64> {
        first_metric_value_instant([self.long_term_debt_current.as_ref()])
    }

    pub(crate) fn latest_total_debt(&self) -> Option<f64> {
        match (self.latest_long_term_debt(), self.latest_current_debt()) {
            (Some(long_term), Some(current)) => Some(long_term + current),
            (Some(long_term), None) => Some(long_term),
            (None, Some(current)) => Some(current),
            (None, None) => None,
        }
    }

    pub(crate) fn latest_diluted_shares_outstanding(&self) -> Option<i64> {
        first_metric_value_annual([self
            .weighted_average_number_of_diluted_shares_outstanding
            .as_ref()])
        .map(|value| value.round() as i64)
    }

    fn latest_annual_revenue_metric(&self) -> Option<&MetricValue> {
        [
            self.revenues.as_ref(),
            self.sales_revenue_net.as_ref(),
            self.revenue_from_contract_with_customer_excluding_assessed_tax
                .as_ref(),
        ]
        .into_iter()
        .flatten()
        .filter_map(|metric| latest_strict_annual_metric(&metric.units))
        .max_by_key(|item| {
            (
                item.end.as_deref().unwrap_or_default(),
                item.filed.as_str(),
                item.fp.as_deref().unwrap_or_default(),
            )
        })
    }
}

impl AnnualMetricSnapshot {
    fn from_metric_value(item: &MetricValue) -> Self {
        Self {
            start: item.start.clone().unwrap_or_default(),
            end: item.end.clone().unwrap_or_default(),
            fp: item.fp.clone(),
            form: item.form.clone(),
        }
    }
}

fn first_metric_value_annual<const N: usize>(metrics: [Option<&Metric>; N]) -> Option<f64> {
    let available = metrics.into_iter().flatten().collect::<Vec<_>>();
    available
        .iter()
        .find_map(|metric| super::news_filter::latest_strict_annual_metric_value(&metric.units))
        .or_else(|| {
            available
                .iter()
                .find_map(|metric| super::news_filter::latest_annual_metric_value(&metric.units))
        })
}

fn first_metric_value_annual_aligned<const N: usize>(
    metrics: [Option<&Metric>; N],
    snapshot: Option<&AnnualMetricSnapshot>,
) -> Option<f64> {
    let available = metrics.into_iter().flatten().collect::<Vec<_>>();
    if let Some(snapshot) = snapshot {
        available
            .iter()
            .find_map(|metric| annual_metric_value_matching_snapshot(&metric.units, snapshot))
    } else {
        available
            .iter()
            .find_map(|metric| latest_strict_annual_metric(&metric.units).map(|item| item.val))
    }
}

fn first_metric_value_instant<const N: usize>(metrics: [Option<&Metric>; N]) -> Option<f64> {
    metrics
        .into_iter()
        .flatten()
        .find_map(|metric| super::news_filter::latest_instant_metric_value(&metric.units))
}

fn annual_metric_value_matching_snapshot(
    units: &MetricUnits,
    snapshot: &AnnualMetricSnapshot,
) -> Option<f64> {
    units
        .usd
        .as_ref()
        .or(units.shares.as_ref())
        .and_then(|values| {
            values
                .iter()
                .filter(|item| matches!(item.form.as_deref(), Some("10-K")))
                .filter(|item| matches!(item.fp.as_deref(), Some("FY")))
                .find(|item| {
                    item.start.as_deref().unwrap_or_default() == snapshot.start
                        && item.end.as_deref().unwrap_or_default() == snapshot.end
                        && item.form.as_deref() == snapshot.form.as_deref()
                        && item.fp.as_deref() == snapshot.fp.as_deref()
                })
                .or_else(|| {
                    values
                        .iter()
                        .filter(|item| matches!(item.form.as_deref(), Some("10-K")))
                        .filter(|item| matches!(item.fp.as_deref(), Some("FY")))
                        .find(|item| {
                            item.end.as_deref().unwrap_or_default() == snapshot.end
                                && item.form.as_deref() == snapshot.form.as_deref()
                                && item.fp.as_deref() == snapshot.fp.as_deref()
                        })
                })
                .filter(|item| {
                    item.end.as_deref().unwrap_or_default() == snapshot.end
                        && item.form.as_deref() == snapshot.form.as_deref()
                        && item.fp.as_deref() == snapshot.fp.as_deref()
                })
        })
        .map(|item| item.val)
}

fn latest_strict_annual_metric(units: &MetricUnits) -> Option<&MetricValue> {
    units
        .usd
        .as_ref()
        .or(units.shares.as_ref())
        .and_then(|values| {
            values
                .iter()
                .filter(|item| {
                    item.start.is_some()
                        && item.end.is_some()
                        && matches!(item.form.as_deref(), Some("10-K"))
                        && matches!(item.fp.as_deref(), Some("FY"))
                })
                .max_by_key(|item| {
                    (
                        item.end.as_deref().unwrap_or_default(),
                        item.filed.as_str(),
                        item.fp.as_deref().unwrap_or_default(),
                    )
                })
        })
}

#[derive(Debug, Deserialize)]
pub(crate) struct CompanySubmissionsResponse {
    #[serde(default)]
    #[allow(dead_code)]
    pub(crate) sic: Option<String>,
    #[serde(default, rename = "sicDescription")]
    pub(crate) sic_description: Option<String>,
    pub(crate) filings: FilingsSection,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FilingsSection {
    pub(crate) recent: RecentFilings,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RecentFilings {
    #[serde(rename = "filingDate")]
    pub(crate) filing_date: Vec<String>,
    pub(crate) form: Vec<String>,
    #[serde(rename = "accessionNumber")]
    pub(crate) accession_number: Vec<String>,
    #[serde(rename = "primaryDocument")]
    pub(crate) primary_document: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SecTickerEntryRaw {
    pub(crate) cik_str: i64,
    pub(crate) ticker: String,
    pub(crate) title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tushare_row_string_field() {
        let fields = vec!["name".to_string(), "code".to_string()];
        let items = vec![
            serde_json::Value::String("Apple".into()),
            serde_json::Value::String("AAPL".into()),
        ];
        let row = TushareRow::new(&fields, items);
        assert_eq!(row.string("name").unwrap(), "Apple");
        assert_eq!(row.string("code").unwrap(), "AAPL");
    }

    #[test]
    fn tushare_row_string_missing_field() {
        let row = TushareRow::new(&[], vec![]);
        assert!(row.string("missing").is_err());
    }

    #[test]
    fn tushare_row_optional_string_number() {
        let fields = vec!["count".to_string()];
        let items = vec![serde_json::Value::Number(serde_json::Number::from(42))];
        let row = TushareRow::new(&fields, items);
        assert_eq!(row.optional_string("count").unwrap(), "42");
    }

    #[test]
    fn tushare_row_optional_string_null() {
        let fields = vec!["val".to_string()];
        let items = vec![serde_json::Value::Null];
        let row = TushareRow::new(&fields, items);
        assert!(row.optional_string("val").is_none());
    }

    #[test]
    fn tushare_row_optional_string_missing() {
        let row = TushareRow::new(&[], vec![]);
        assert!(row.optional_string("x").is_none());
    }

    #[test]
    fn tushare_row_f64_from_number() {
        let fields = vec!["price".to_string()];
        let items = vec![serde_json::Value::Number(
            serde_json::Number::from_f64(123.45).unwrap(),
        )];
        let row = TushareRow::new(&fields, items);
        assert!((row.f64("price").unwrap() - 123.45).abs() < 0.001);
    }

    #[test]
    fn tushare_row_f64_from_string() {
        let fields = vec!["price".to_string()];
        let items = vec![serde_json::Value::String("99.5".into())];
        let row = TushareRow::new(&fields, items);
        assert!((row.f64("price").unwrap() - 99.5).abs() < 0.001);
    }

    #[test]
    fn tushare_row_f64_missing() {
        let row = TushareRow::new(&[], vec![]);
        assert!(row.f64("x").is_err());
    }

    #[test]
    fn tushare_row_optional_f64_null() {
        let fields = vec!["val".to_string()];
        let items = vec![serde_json::Value::Null];
        let row = TushareRow::new(&fields, items);
        assert!(row.optional_f64("val").is_none());
    }

    #[test]
    fn tushare_row_optional_f64_invalid_string() {
        let fields = vec!["val".to_string()];
        let items = vec![serde_json::Value::String("not_a_number".into())];
        let row = TushareRow::new(&fields, items);
        assert!(row.optional_f64("val").is_none());
    }

    #[test]
    fn tushare_row_bool_value() {
        let fields = vec!["flag".to_string()];
        let items = vec![serde_json::Value::Bool(true)];
        let row = TushareRow::new(&fields, items);
        assert_eq!(row.optional_string("flag").unwrap(), "true");
    }
}
