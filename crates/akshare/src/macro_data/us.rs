//! US macro-economic data from Eastmoney datacenter and Jin10.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::{fetch_em_indicator, fetch_jin10_report};

// ---------------------------------------------------------------------------
// Eastmoney datacenter — RPT_ECONOMICVALUE_USA with INDICATOR_ID
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// US pending home sales monthly rate (未决房屋销售月率).
    pub async fn us_pending_home_sales(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(
            self,
            "RPT_ECONOMICVALUE_USA",
            "EMG00342249",
            "US Pending Home Sales",
        )
        .await
    }

    /// US CPI YoY (CPI年率).
    pub async fn us_cpi_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_indicator(self, "RPT_ECONOMICVALUE_USA", "EMG00000733", "US CPI YoY").await
    }
}

// ---------------------------------------------------------------------------
// Jin10 datacenter — US macro indicators with attr_id
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// US GDP monthly (美国国内生产总值).
    pub async fn us_gdp_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "53", "US GDP Monthly").await
    }

    /// US CPI monthly (美国CPI月率).
    pub async fn us_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "9", "US CPI Monthly").await
    }

    /// US core CPI monthly (美国核心CPI月率).
    pub async fn us_core_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "6", "US Core CPI Monthly").await
    }

    /// US personal spending (美国个人支出月率).
    pub async fn us_personal_spending(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "35", "US Personal Spending").await
    }

    /// US retail sales (美国零售销售月率).
    pub async fn us_retail_sales(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "39", "US Retail Sales").await
    }

    /// US import price (美国进口物价指数).
    pub async fn us_import_price(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "18", "US Import Price").await
    }

    /// US export price (美国出口价格指数).
    pub async fn us_export_price(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "79", "US Export Price").await
    }

    /// US LMCI (美联储劳动力市场状况指数).
    pub async fn us_lmci(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "93", "US LMCI").await
    }

    /// US unemployment rate (美国失业率).
    pub async fn us_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "47", "US Unemployment Rate").await
    }

    /// US job cuts (挑战者企业裁员人数).
    pub async fn us_job_cuts(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "78", "US Job Cuts").await
    }

    /// US non-farm payrolls (非农就业人数).
    pub async fn us_non_farm(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "33", "US Non-Farm Payrolls").await
    }

    /// US ADP employment (ADP就业人数).
    pub async fn us_adp_employment(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "1", "US ADP Employment").await
    }

    /// US core PCE price (核心PCE物价指数).
    pub async fn us_core_pce_price(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "80", "US Core PCE Price").await
    }

    /// US real consumer spending (实际个人消费支出).
    pub async fn us_real_consumer_spending(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "81", "US Real Consumer Spending").await
    }

    /// US trade balance (贸易帐).
    pub async fn us_trade_balance(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "42", "US Trade Balance").await
    }

    /// US current account (经常帐).
    pub async fn us_current_account(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "12", "US Current Account").await
    }

    /// US PPI (生产者物价指数).
    pub async fn us_ppi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "37", "US PPI").await
    }

    /// US core PPI (核心生产者物价指数).
    pub async fn us_core_ppi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "7", "US Core PPI").await
    }

    /// US API crude stock (API原油库存).
    pub async fn us_api_crude_stock(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "69", "US API Crude Stock").await
    }

    /// US PMI (制造业PMI).
    pub async fn us_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "74", "US PMI").await
    }

    /// US ISM PMI (ISM制造业PMI).
    pub async fn us_ism_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "28", "US ISM PMI").await
    }

    /// US industrial production (工业产出月率).
    pub async fn us_industrial_production(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "20", "US Industrial Production").await
    }

    /// US durable goods orders (耐用品订单月率).
    pub async fn us_durable_goods_orders(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "13", "US Durable Goods Orders").await
    }

    /// US factory orders (工厂订单月率).
    pub async fn us_factory_orders(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "16", "US Factory Orders").await
    }

    /// US services PMI (服务业PMI).
    pub async fn us_services_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "89", "US Services PMI").await
    }

    /// US business inventories (商业库存月率).
    pub async fn us_business_inventories(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "4", "US Business Inventories").await
    }

    /// US ISM non-manufacturing PMI (ISM非制造业PMI).
    pub async fn us_ism_non_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "29", "US ISM Non-PMI").await
    }

    /// US NAHB housing market index (NAHB房产市场指数).
    pub async fn us_nahb_house_market_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "31", "US NAHB House Market").await
    }

    /// US housing starts (新屋开工月率).
    pub async fn us_house_starts(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "17", "US House Starts").await
    }

    /// US new home sales (新屋销售月率).
    pub async fn us_new_home_sales(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "32", "US New Home Sales").await
    }

    /// US building permits (营建许可月率).
    pub async fn us_building_permits(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "3", "US Building Permits").await
    }

    /// US existing home sales (成屋销售月率).
    pub async fn us_exist_home_sales(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "15", "US Existing Home Sales").await
    }

    /// US house price index (房价指数月率).
    pub async fn us_house_price_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "51", "US House Price Index").await
    }

    /// US S&P/Case-Shiller 20-city house price index.
    pub async fn us_spcs20(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "52", "US SPCS20").await
    }

    /// US pending home sales (未决房屋销售月率 Jin10).
    pub async fn us_pending_home_sales_jin10(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "34", "US Pending Home Sales").await
    }

    /// US CB consumer confidence (谘商会消费者信心指数).
    pub async fn us_cb_consumer_confidence(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "5", "US CB Consumer Confidence").await
    }

    /// US NFIB small business optimism (NFIB小型企业乐观指数).
    pub async fn us_nfib_small_business(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "63", "US NFIB Small Business").await
    }

    /// US Michigan consumer sentiment (密歇根大学消费者信心指数).
    pub async fn us_michigan_consumer_sentiment(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "50", "US Michigan Consumer Sentiment").await
    }

    /// US EIA crude oil rate (EIA原油库存变化率).
    pub async fn us_eia_crude_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "10", "US EIA Crude Rate").await
    }

    /// US initial jobless claims (初请失业金人数).
    pub async fn us_initial_jobless(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "44", "US Initial Jobless Claims").await
    }
}

// ---------------------------------------------------------------------------
// Jin10 CDN — US macro indicators
// ---------------------------------------------------------------------------

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Jin10CdnResp {
    values: Option<serde_json::Value>,
}

impl AkShareClient {
    /// Baker Hughes US rig count (贝克休斯钻井报告).
    pub async fn us_rig_count(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/baker.json";
        let resp: Jin10CdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, row) in obj {
                if let Some(arr) = row.as_array()
                    && arr.len() >= 2
                {
                    let count = arr[0].as_f64().unwrap_or(0.0);
                    items.push(MacroDataPoint {
                        date: date.clone(),
                        value: count,
                        name: "US Rig Count".to_string(),
                    });
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// US crude oil production (美国原油产量).
    pub async fn us_crude_production(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/usa_oil.json";
        let resp: Jin10CdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, row) in obj {
                if let Some(arr) = row.as_array() {
                    // First key is total US crude production
                    if let Some(inner) = arr.first().and_then(|v| v.as_array())
                        && let Some(val) = inner.first().and_then(serde_json::Value::as_f64)
                    {
                        items.push(MacroDataPoint {
                            date: date.clone(),
                            value: val,
                            name: "US Crude Production".to_string(),
                        });
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// US crude oil production (美国原油产量报告) - alias.
    pub async fn macro_usa_crude_inner(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_crude_production().await
    }
}

// ---------------------------------------------------------------------------
// Python-compatible macro_usa_* aliases
// ---------------------------------------------------------------------------

impl AkShareClient {
    pub async fn macro_usa_phs(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_pending_home_sales().await
    }

    pub async fn macro_usa_cpi_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_cpi_yoy().await
    }

    pub async fn macro_usa_gdp_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_gdp_monthly().await
    }

    pub async fn macro_usa_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_cpi_monthly().await
    }

    pub async fn macro_usa_core_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_core_cpi_monthly().await
    }

    pub async fn macro_usa_personal_spending(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_personal_spending().await
    }

    pub async fn macro_usa_retail_sales(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_retail_sales().await
    }

    pub async fn macro_usa_import_price(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_import_price().await
    }

    pub async fn macro_usa_export_price(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_export_price().await
    }

    pub async fn macro_usa_lmci(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_lmci().await
    }

    pub async fn macro_usa_unemployment_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_unemployment_rate().await
    }

    pub async fn macro_usa_job_cuts(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_job_cuts().await
    }

    pub async fn macro_usa_non_farm(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_non_farm().await
    }

    pub async fn macro_usa_adp_employment(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_adp_employment().await
    }

    pub async fn macro_usa_core_pce_price(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_core_pce_price().await
    }

    pub async fn macro_usa_real_consumer_spending(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_real_consumer_spending().await
    }

    pub async fn macro_usa_trade_balance(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_trade_balance().await
    }

    pub async fn macro_usa_current_account(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_current_account().await
    }

    pub async fn macro_usa_ppi(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_ppi().await
    }

    pub async fn macro_usa_core_ppi(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_core_ppi().await
    }

    pub async fn macro_usa_api_crude_stock(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_api_crude_stock().await
    }

    pub async fn macro_usa_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_pmi().await
    }

    pub async fn macro_usa_ism_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_ism_pmi().await
    }

    pub async fn macro_usa_industrial_production(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_industrial_production().await
    }

    pub async fn macro_usa_durable_goods_orders(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_durable_goods_orders().await
    }

    pub async fn macro_usa_factory_orders(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_factory_orders().await
    }

    pub async fn macro_usa_services_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_services_pmi().await
    }

    pub async fn macro_usa_business_inventories(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_business_inventories().await
    }

    pub async fn macro_usa_ism_non_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_ism_non_pmi().await
    }

    pub async fn macro_usa_nahb_house_market_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_nahb_house_market_index().await
    }

    pub async fn macro_usa_house_starts(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_house_starts().await
    }

    pub async fn macro_usa_new_home_sales(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_new_home_sales().await
    }

    pub async fn macro_usa_building_permits(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_building_permits().await
    }

    pub async fn macro_usa_exist_home_sales(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_exist_home_sales().await
    }

    pub async fn macro_usa_house_price_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_house_price_index().await
    }

    pub async fn macro_usa_spcs20(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_spcs20().await
    }

    pub async fn macro_usa_pending_home_sales(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_pending_home_sales().await
    }

    pub async fn macro_usa_cb_consumer_confidence(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_cb_consumer_confidence().await
    }

    pub async fn macro_usa_nfib_small_business(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_nfib_small_business().await
    }

    pub async fn macro_usa_michigan_consumer_sentiment(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_michigan_consumer_sentiment().await
    }

    pub async fn macro_usa_eia_crude_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_eia_crude_rate().await
    }

    pub async fn macro_usa_initial_jobless(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_initial_jobless().await
    }

    pub async fn macro_usa_rig_count(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_rig_count().await
    }

    pub async fn macro_usa_cftc_nc_holding(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_cftc_nc_holding().await
    }

    pub async fn macro_usa_cftc_c_holding(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_cftc_c_holding().await
    }

    pub async fn macro_usa_cftc_merchant_currency_holding(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_cftc_merchant_currency_holding().await
    }

    pub async fn macro_usa_cftc_merchant_goods_holding(&self) -> Result<Vec<MacroDataPoint>> {
        self.us_cftc_merchant_goods_holding().await
    }

    /// CME precious metals holding report (CME贵金属).
    pub async fn macro_usa_cme_merchant_goods_holding(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/cme_3.json";
        let resp: Jin10CdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, products) in obj {
                if let Some(arr) = products.as_array() {
                    for product in arr {
                        if let Some(parr) = product.as_array() {
                            let pz = parr.first().and_then(|v| v.as_str()).unwrap_or("");
                            let tc = parr.get(1).and_then(|v| v.as_str()).unwrap_or("");
                            // Column index 5 is volume
                            if let Some(vol) = parr.get(5).and_then(serde_json::Value::as_f64) {
                                items.push(MacroDataPoint {
                                    date: date.clone(),
                                    value: vol,
                                    name: format!("CME {pz}-{tc}"),
                                });
                            }
                        }
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }
}

// ---------------------------------------------------------------------------
// Jin10 CDN — CFTC data
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct Jin10CftcResp {
    values: Option<serde_json::Value>,
}

/// Helper to fetch CFTC holding data from Jin10 CDN.
async fn fetch_cftc_holding(
    client: &AkShareClient,
    url: &str,
    name_label: &str,
) -> Result<Vec<MacroDataPoint>> {
    let resp: Jin10CftcResp = client.get(url).send().await?.json().await?;

    let values = resp.values.unwrap_or_default();
    let mut items = Vec::new();
    if let Some(obj) = values.as_object() {
        for (date, row) in obj {
            if let Some(cols) = row.as_object() {
                for (col_name, col_data) in cols {
                    if let Some(arr) = col_data.as_array()
                        && let Some(val) = arr.first().and_then(serde_json::Value::as_f64)
                    {
                        items.push(MacroDataPoint {
                            date: date.clone(),
                            value: val,
                            name: format!("{name_label} {col_name}"),
                        });
                    }
                }
            }
        }
    }
    items.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(items)
}

impl AkShareClient {
    /// CFTC non-commercial holding report (CFTC外汇类非商业持仓报告).
    pub async fn us_cftc_nc_holding(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_cftc_holding(
            self,
            "https://cdn.jin10.com/data_center/reports/cftc_4.json",
            "CFTC NC",
        )
        .await
    }

    /// CFTC commodity non-commercial holding report (CFTC商品类非商业持仓报告).
    pub async fn us_cftc_c_holding(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_cftc_holding(
            self,
            "https://cdn.jin10.com/data_center/reports/cftc_2.json",
            "CFTC C",
        )
        .await
    }

    /// CFTC merchant currency holding report (CFTC外汇类商业持仓报告).
    pub async fn us_cftc_merchant_currency_holding(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_cftc_holding(
            self,
            "https://cdn.jin10.com/data_center/reports/cftc_3.json",
            "CFTC Merchant Currency",
        )
        .await
    }

    /// CFTC merchant goods holding report (CFTC商品类商业持仓报告).
    pub async fn us_cftc_merchant_goods_holding(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_cftc_holding(
            self,
            "https://cdn.jin10.com/data_center/reports/cftc_1.json",
            "CFTC Merchant Goods",
        )
        .await
    }
}
