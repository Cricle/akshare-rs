//! China macro-economic data from Eastmoney datacenter and Jin10.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;

use super::shared::{fetch_em_industry_index, fetch_em_report, fetch_jin10_report};

/// Strip HTML tags and extract text content.
fn extract_html_text(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result.trim().to_string()
}

/// Fetch macro data from Sina Finance JSONP API (新浪财经-中国宏观经济数据).
async fn fetch_sina_macro(
    client: &AkShareClient,
    cate: &str,
    event: &str,
    name_label: &str,
) -> Result<Vec<MacroDataPoint>> {
    let url = "https://quotes.sina.cn/mac/api/jsonp_v3.php/SINAREMOTECALLCALLBACK/MacPage_Service.get_pagedata";
    let resp_text = client
        .get(url)
        .query(&[
            ("cate", cate),
            ("event", event),
            ("from", "0"),
            ("num", "500"),
            ("condition", ""),
        ])
        .send()
        .await?
        .text()
        .await?;

    // Extract JSON from JSONP wrapper: SINAREMOTECALLCALLBACK({...});
    let json_start = resp_text.find('{').unwrap_or(0);
    let json_end = resp_text.rfind('}').unwrap_or(resp_text.len());
    if json_start >= json_end {
        return Ok(Vec::new());
    }
    let json_str = &resp_text[json_start..=json_end];
    let data: serde_json::Value = serde_json::from_str(json_str)?;

    let mut items = Vec::new();
    if let Some(data_arr) = data.get("data").and_then(|d| d.as_array()) {
        for row in data_arr {
            if let Some(row_obj) = row.as_object() {
                // Try to find a date-like key and a numeric value
                let mut date = String::new();
                let mut value = 0.0_f64;
                for (key, val) in row_obj {
                    if key.contains("时间") || key.contains("月份") || key.contains("统计") {
                        date = val.as_str().unwrap_or("").to_string();
                    }
                }
                // Get first numeric value
                for (_key, val) in row_obj {
                    if let Some(v) = val.as_str().and_then(|s| s.parse::<f64>().ok()) {
                        value = v;
                        break;
                    } else if let Some(v) = val.as_f64() {
                        value = v;
                        break;
                    }
                }
                if !date.is_empty() {
                    items.push(MacroDataPoint {
                        date: date.get(..10).unwrap_or(&date).to_string(),
                        value,
                        name: name_label.to_string(),
                    });
                }
            }
        }
    } else if let Some(data_obj) = data.get("data").and_then(|d| d.as_object()) {
        // Handle object-style data (non-cumulative)
        if let Some(arr) = data_obj.get("非累计").and_then(|v| v.as_array()) {
            for row in arr {
                if let Some(row_obj) = row.as_object() {
                    let mut date = String::new();
                    let mut value = 0.0_f64;
                    for (key, val) in row_obj {
                        if key.contains("时间") || key.contains("月份") || key.contains("统计")
                        {
                            date = val.as_str().unwrap_or("").to_string();
                        }
                    }
                    for (_key, val) in row_obj {
                        if let Some(v) = val.as_str().and_then(|s| s.parse::<f64>().ok()) {
                            value = v;
                            break;
                        } else if let Some(v) = val.as_f64() {
                            value = v;
                            break;
                        }
                    }
                    if !date.is_empty() {
                        items.push(MacroDataPoint {
                            date: date.get(..10).unwrap_or(&date).to_string(),
                            value,
                            name: name_label.to_string(),
                        });
                    }
                }
            }
        }
    }
    items.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(items)
}

// ---------------------------------------------------------------------------
// Eastmoney datacenter — direct reportName functions
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// China GDP data.
    pub async fn china_gdp(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(self, "RPT_ECONOMY_GDP", "REPORT_DATE", "China GDP").await
    }

    /// China CPI (Consumer Price Index).
    pub async fn china_cpi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(self, "RPT_ECONOMY_CPI", "REPORT_DATE", "China CPI").await
    }

    /// China PPI (Producer Price Index).
    pub async fn china_ppi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(self, "RPT_ECONOMY_PPI", "REPORT_DATE", "China PPI").await
    }

    /// China PMI (Purchasing Managers' Index).
    pub async fn china_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(self, "RPT_ECONOMY_PMI", "REPORT_DATE", "China PMI").await
    }

    /// China money supply (M0, M1, M2).
    pub async fn china_money_supply(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_CURRENCY_SUPPLY",
            "REPORT_DATE",
            "China Money Supply",
        )
        .await
    }

    /// China trade balance data.
    pub async fn china_trade(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(self, "RPT_ECONOMY_CUSTOMS", "REPORT_DATE", "China Trade").await
    }

    /// Enterprise commodity price index (企业商品价格指数).
    pub async fn china_goods_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_GOODS_INDEX",
            "REPORT_DATE",
            "China Goods Index",
        )
        .await
    }

    /// Foreign direct investment (外商直接投资).
    pub async fn china_fdi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(self, "RPT_ECONOMY_FDI", "REPORT_DATE", "China FDI").await
    }

    /// Loan Prime Rate (LPR, 贷款市场报价利率).
    pub async fn china_lpr(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(self, "RPTA_WEB_RATE", "REPORT_DATE", "China LPR").await
    }

    /// New house price index (新建住宅价格指数).
    pub async fn china_new_house_price(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_HOUSE_PRICE",
            "REPORT_DATE",
            "China New House Price",
        )
        .await
    }

    /// Enterprise boom index (企业景气指数).
    pub async fn china_enterprise_boom_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_BOOM_INDEX",
            "REPORT_DATE",
            "China Enterprise Boom Index",
        )
        .await
    }

    /// National tax receipts (全国税收收入).
    pub async fn china_national_tax_receipts(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(self, "RPT_ECONOMY_TAX", "REPORT_DATE", "China Tax Receipts").await
    }

    /// New financial credit (新增信贷).
    pub async fn china_new_financial_credit(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_RMB_LOAN",
            "REPORT_DATE",
            "China New Credit",
        )
        .await
    }

    /// Foreign exchange and gold reserves (外汇和黄金储备).
    pub async fn china_fx_gold(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_GOLD_CURRENCY",
            "REPORT_DATE",
            "China FX Gold",
        )
        .await
    }

    /// Stock market statistics (股市统计).
    pub async fn china_stock_market_cap(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_STOCK_STATISTICS",
            "REPORT_DATE",
            "China Stock Market Cap",
        )
        .await
    }

    /// Fixed asset investment (固定资产投资).
    pub async fn china_fixed_asset_investment(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_ASSET_INVEST",
            "REPORT_DATE",
            "China Fixed Asset Investment",
        )
        .await
    }

    /// Fiscal revenue (财政收入).
    pub async fn china_fiscal_revenue(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_INCOME",
            "REPORT_DATE",
            "China Fiscal Revenue",
        )
        .await
    }

    /// Foreign exchange loans (外汇贷款).
    pub async fn china_fx_loans(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_FOREX_LOAN",
            "REPORT_DATE",
            "China FX Loans",
        )
        .await
    }

    /// Foreign exchange deposits (外汇存款).
    pub async fn china_fx_deposits(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_FOREX_DEPOSIT",
            "REPORT_DATE",
            "China FX Deposits",
        )
        .await
    }

    /// Consumer confidence index (消费者信心指数).
    pub async fn china_consumer_confidence(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_FAITH_INDEX",
            "REPORT_DATE",
            "China Consumer Confidence",
        )
        .await
    }

    /// Industrial value-added growth (工业增加值).
    pub async fn china_industrial_growth(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_INDUS_GROW",
            "REPORT_DATE",
            "China Industrial Growth",
        )
        .await
    }

    /// Deposit reserve ratio (存款准备金率).
    pub async fn china_reserve_requirement_ratio(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_DEPOSIT_RESERVE",
            "REPORT_DATE",
            "China RRR",
        )
        .await
    }

    /// Total retail sales of consumer goods (社会消费品零售总额).
    pub async fn china_consumer_goods_retail(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_report(
            self,
            "RPT_ECONOMY_TOTAL_RETAIL",
            "REPORT_DATE",
            "China Consumer Goods Retail",
        )
        .await
    }
}

// ---------------------------------------------------------------------------
// Eastmoney datacenter — RPT_INDUSTRY_INDEX with INDICATOR_ID filter
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Bank financing products (银行理财产品发行数量).
    pub async fn china_bank_financing(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI01516267", "China Bank Financing").await
    }

    /// Insurance income (原保险保费收入).
    pub async fn china_insurance_income(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMM00088870", "China Insurance Income").await
    }

    /// Mobile phone shipments (手机出货量).
    pub async fn china_mobile_number(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00225823", "China Mobile Shipments").await
    }

    /// Vegetable basket index (菜篮子指数).
    pub async fn china_vegetable_basket(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00009275", "China Vegetable Basket").await
    }

    /// Agricultural product price index (农产品批发价格指数).
    pub async fn china_agricultural_product(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00009274", "China Agricultural Product").await
    }

    /// Agricultural index (农业指数).
    pub async fn china_agricultural_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00662543", "China Agricultural Index").await
    }

    /// Energy index (能源指数).
    pub async fn china_energy_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00662539", "China Energy Index").await
    }

    /// Commodity price index (大宗商品价格指数).
    pub async fn china_commodity_price_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00662535", "China Commodity Price Index").await
    }

    /// Yiwu small commodity index (义乌小商品指数).
    pub async fn china_yw_electronic_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00055551", "China Yiwu Index").await
    }

    /// Construction industry index (建筑业指数).
    pub async fn china_construction_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00662541", "China Construction Index").await
    }

    /// Construction material price index (建材价格指数).
    pub async fn china_construction_price_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00237146", "China Construction Price Index").await
    }

    /// Logistics prosperity index (物流景气指数).
    pub async fn china_lpi_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00352262", "China LPI Index").await
    }

    /// BDTI (原油运输指数).
    pub async fn china_bdti_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00107668", "China BDTI Index").await
    }

    /// BSI (超灵便型船运价指数).
    pub async fn china_bsi_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMI00107667", "China BSI Index").await
    }

    /// Real estate index (房地产指数).
    pub async fn china_real_estate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_em_industry_index(self, "EMM00121987", "China Real Estate").await
    }
}

// ---------------------------------------------------------------------------
// Jin10 datacenter — macro indicators with attr_id
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// China GDP yearly rate (中国GDP年率报告).
    pub async fn china_gdp_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "57", "China GDP Yearly").await
    }

    /// China CPI yearly rate (中国CPI年率报告).
    pub async fn china_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "56", "China CPI Yearly").await
    }

    /// China CPI monthly rate (中国CPI月率报告).
    pub async fn china_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "72", "China CPI Monthly").await
    }

    /// China PPI yearly rate (中国PPI年率报告).
    pub async fn china_ppi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "60", "China PPI Yearly").await
    }

    /// China exports YoY (以美元计算出口年率).
    pub async fn china_exports_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "66", "China Exports YoY").await
    }

    /// China imports YoY (以美元计算进口年率).
    pub async fn china_imports_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "77", "China Imports YoY").await
    }

    /// China trade balance (以美元计算贸易帐).
    pub async fn china_trade_balance_jin10(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "61", "China Trade Balance").await
    }

    /// China industrial production YoY (规模以上工业增加值年率).
    pub async fn china_industrial_production_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "58", "China Industrial Production YoY").await
    }

    /// China official manufacturing PMI (官方制造业PMI).
    pub async fn china_pmi_jin10(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "65", "China Official PMI").await
    }

    /// China Caixin manufacturing PMI (财新制造业PMI终值).
    pub async fn china_caixin_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "73", "China Caixin PMI").await
    }

    /// China Caixin services PMI (财新服务业PMI).
    pub async fn china_caixin_services_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "67", "China Caixin Services PMI").await
    }

    /// China non-manufacturing PMI (非制造业PMI).
    pub async fn china_non_man_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "75", "China Non-Man PMI").await
    }

    /// China FX reserves yearly (外汇储备).
    pub async fn china_fx_reserves_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "76", "China FX Reserves").await
    }

    /// China M2 yearly (M2年率).
    pub async fn china_m2_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_report(self, "59", "China M2 Yearly").await
    }
}

// ---------------------------------------------------------------------------
// Jin10 CDN — China interbank rates and FX
// ---------------------------------------------------------------------------

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Jin10CdnCdnResp {
    values: Option<serde_json::Value>,
}

impl AkShareClient {
    /// Shibor rates (上海银行业同业拆借报告).
    pub async fn china_shibor(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/il_1.json";
        let resp: Jin10CdnCdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, row) in obj {
                if let Some(cols) = row.as_object() {
                    for (tenor, col_data) in cols {
                        if let Some(arr) = col_data.as_array()
                            && let Some(val) = arr.first().and_then(serde_json::Value::as_f64)
                        {
                            items.push(MacroDataPoint {
                                date: date.clone(),
                                value: val,
                                name: format!("Shibor {tenor}"),
                            });
                        }
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// HIBOR rates (香港同业拆借报告).
    pub async fn china_hibor(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/il_2.json";
        let resp: Jin10CdnCdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, row) in obj {
                if let Some(cols) = row.as_object() {
                    for (tenor, col_data) in cols {
                        if let Some(arr) = col_data.as_array()
                            && let Some(val) = arr.first().and_then(serde_json::Value::as_f64)
                        {
                            items.push(MacroDataPoint {
                                date: date.clone(),
                                value: val,
                                name: format!("HIBOR {tenor}"),
                            });
                        }
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// RMB central parity rates (人民币汇率中间价报告).
    pub async fn china_rmb_central_parity(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/exchange_rate.json";
        let resp: Jin10CdnCdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, row) in obj {
                if let Some(cols) = row.as_object() {
                    for (pair, col_data) in cols {
                        if let Some(arr) = col_data.as_array()
                            && let Some(val) = arr.first().and_then(serde_json::Value::as_f64)
                        {
                            items.push(MacroDataPoint {
                                date: date.clone(),
                                value: val,
                                name: format!("RMB {pair}"),
                            });
                        }
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// Shenzhen margin trading report (深圳融资融券报告).
    pub async fn china_margin_sz(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/fs_2.json";
        let resp: Jin10CdnCdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, row) in obj {
                if let Some(arr) = row.as_array() {
                    // First element is margin buy amount
                    if let Some(val) = arr.first().and_then(serde_json::Value::as_f64) {
                        items.push(MacroDataPoint {
                            date: date.clone(),
                            value: val,
                            name: "SZ Margin Buy".to_string(),
                        });
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// Shanghai margin trading report (上海融资融券报告).
    pub async fn china_margin_sh(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/fs_1.json";
        let resp: Jin10CdnCdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, row) in obj {
                if let Some(arr) = row.as_array()
                    && let Some(val) = arr.first().and_then(serde_json::Value::as_f64)
                {
                    items.push(MacroDataPoint {
                        date: date.clone(),
                        value: val,
                        name: "SH Margin Buy".to_string(),
                    });
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// Shanghai Gold Exchange report (上海黄金交易所报告).
    pub async fn china_sge_report(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://cdn.jin10.com/data_center/reports/sge.json";
        let resp: Jin10CdnCdnResp = self.get(url).send().await?.json().await?;

        let values = resp.values.unwrap_or_default();
        let mut items = Vec::new();
        if let Some(obj) = values.as_object() {
            for (date, products) in obj {
                if let Some(arr) = products.as_array() {
                    for product in arr {
                        if let Some(parr) = product.as_array() {
                            // First element is product name, last numeric is volume
                            let name = parr.first().and_then(|v| v.as_str()).unwrap_or("unknown");
                            let volume = parr
                                .get(8)
                                .and_then(serde_json::Value::as_f64)
                                .unwrap_or(0.0);
                            items.push(MacroDataPoint {
                                date: date.clone(),
                                value: volume,
                                name: format!("SGE {name}"),
                            });
                        }
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }
}

// Python-compatible aliases
impl AkShareClient {
    pub async fn macro_china_agricultural_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_agricultural_index().await
    }

    pub async fn macro_china_agricultural_product(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_agricultural_product().await
    }

    pub async fn macro_china_bank_financing(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_bank_financing().await
    }

    pub async fn macro_china_bdti_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_bdti_index().await
    }

    pub async fn macro_china_bsi_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_bsi_index().await
    }

    pub async fn macro_china_commodity_price_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_commodity_price_index().await
    }

    pub async fn macro_china_construction_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_construction_index().await
    }

    pub async fn macro_china_construction_price_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_construction_price_index().await
    }

    pub async fn macro_china_consumer_goods_retail(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_consumer_goods_retail().await
    }

    pub async fn macro_china_cpi(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_cpi().await
    }

    pub async fn macro_china_cpi_monthly(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_cpi_monthly().await
    }

    pub async fn macro_china_cpi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_cpi_yearly().await
    }

    pub async fn macro_china_energy_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_energy_index().await
    }

    pub async fn macro_china_enterprise_boom_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_enterprise_boom_index().await
    }

    pub async fn macro_china_exports_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_exports_yoy().await
    }

    pub async fn macro_china_fdi(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_fdi().await
    }

    pub async fn macro_china_fx_gold(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_fx_gold().await
    }

    pub async fn macro_china_fx_reserves_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_fx_reserves_yearly().await
    }

    pub async fn macro_china_gdp(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_gdp().await
    }

    pub async fn macro_china_gdp_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_gdp_yearly().await
    }

    pub async fn macro_china_imports_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_imports_yoy().await
    }

    pub async fn macro_china_industrial_production_yoy(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_industrial_production_yoy().await
    }

    pub async fn macro_china_insurance_income(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_insurance_income().await
    }

    pub async fn macro_china_lpi_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_lpi_index().await
    }

    pub async fn macro_china_lpr(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_lpr().await
    }

    pub async fn macro_china_m2_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_m2_yearly().await
    }

    pub async fn macro_china_mobile_number(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_mobile_number().await
    }

    pub async fn macro_china_money_supply(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_money_supply().await
    }

    pub async fn macro_china_national_tax_receipts(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_national_tax_receipts().await
    }

    pub async fn macro_china_new_financial_credit(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_new_financial_credit().await
    }

    pub async fn macro_china_new_house_price(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_new_house_price().await
    }

    pub async fn macro_china_non_man_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_non_man_pmi().await
    }

    pub async fn macro_china_pmi(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_pmi().await
    }

    pub async fn macro_china_ppi(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_ppi().await
    }

    pub async fn macro_china_ppi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_ppi_yearly().await
    }

    pub async fn macro_china_real_estate(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_real_estate().await
    }

    pub async fn macro_china_reserve_requirement_ratio(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_reserve_requirement_ratio().await
    }

    pub async fn macro_china_stock_market_cap(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_stock_market_cap().await
    }

    pub async fn macro_china_vegetable_basket(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_vegetable_basket().await
    }

    pub async fn macro_china_yw_electronic_index(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_yw_electronic_index().await
    }

    pub async fn macro_china_au_report(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_sge_report().await
    }

    pub async fn macro_china_shibor_all(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_shibor().await
    }

    pub async fn macro_china_hk_market_info(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_hibor().await
    }

    pub async fn macro_china_rmb(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_rmb_central_parity().await
    }

    pub async fn macro_china_market_margin_sz(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_margin_sz().await
    }

    pub async fn macro_china_market_margin_sh(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_margin_sh().await
    }

    pub async fn macro_china_trade_balance(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_trade_balance_jin10().await
    }

    pub async fn macro_china_pmi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_pmi_jin10().await
    }

    pub async fn macro_china_cx_pmi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_caixin_pmi().await
    }

    pub async fn macro_china_cx_services_pmi_yearly(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_caixin_services_pmi().await
    }

    pub async fn macro_china_czsr(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_fiscal_revenue().await
    }

    pub async fn macro_china_gdzctz(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_fixed_asset_investment().await
    }

    pub async fn macro_china_gyzjz(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_industrial_growth().await
    }

    pub async fn macro_china_hgjck(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_trade().await
    }

    pub async fn macro_china_whxd(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_fx_loans().await
    }

    pub async fn macro_china_wbck(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_fx_deposits().await
    }

    pub async fn macro_china_xfzxx(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_consumer_confidence().await
    }

    pub async fn macro_china_qyspjg(&self) -> Result<Vec<MacroDataPoint>> {
        self.china_goods_index().await
    }

    /// CNBS macro leverage ratio data (中国宏观杠杆率).
    /// NOTE: The CNBS endpoint returns an Excel (.xlsx) file which requires
    /// a dedicated Excel parsing crate (e.g. calamine). This stub returns
    /// an empty vec until that dependency is added.
    pub async fn macro_cnbs(&self) -> Result<Vec<MacroDataPoint>> {
        Ok(Vec::new())
    }

    /// Sina Finance - central bank balance sheet (央行货币当局资产负债).
    pub async fn macro_china_central_bank_balance(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "fininfo", "8", "China Central Bank Balance").await
    }

    /// Sina Finance - insurance industry data (保险业经营情况).
    pub async fn macro_china_insurance(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "fininfo", "19", "China Insurance").await
    }

    /// Sina Finance - money supply (货币供应量).
    pub async fn macro_china_supply_of_money(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "fininfo", "1", "China Supply of Money").await
    }

    /// Sina Finance - foreign exchange and gold reserves (央行黄金和外汇储备).
    pub async fn macro_china_foreign_exchange_gold(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "fininfo", "5", "China FX Gold Sina").await
    }

    /// Sina Finance - retail price index (商品零售价格指数).
    pub async fn macro_china_retail_price_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "price", "12", "China Retail Price Index").await
    }

    /// Sina Finance - society electricity usage (全社会用电分类情况表).
    pub async fn macro_china_society_electricity(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "industry", "6", "China Society Electricity").await
    }

    /// Sina Finance - society traffic volume (全社会客货运输量).
    pub async fn macro_china_society_traffic_volume(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "industry", "10", "China Society Traffic").await
    }

    /// Sina Finance - postal and telecommunications (邮电业务基本情况).
    pub async fn macro_china_postal_telecommunicational(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "industry", "11", "China Postal Telecom").await
    }

    /// Sina Finance - international tourism FX revenue (国际旅游外汇收入构成).
    pub async fn macro_china_international_tourism_fx(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "industry", "15", "China Intl Tourism FX").await
    }

    /// Sina Finance - passenger load factor (民航客座率及载运率).
    pub async fn macro_china_passenger_load_factor(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "industry", "20", "China Passenger Load Factor").await
    }

    /// Sina Finance - freight index (航贸运价指数).
    pub async fn macro_china_freight_index(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_sina_macro(self, "industry", "22", "China Freight Index").await
    }

    /// Social financing scale (社会融资规模增量统计).
    pub async fn macro_china_shrzgm(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://data.mofcom.gov.cn/datamofcom/front/gnmy/shrzgmQuery";
        let resp: serde_json::Value = self.post(url).send().await?.json().await?;

        let mut items = Vec::new();
        if let Some(arr) = resp.as_array() {
            for row in arr {
                if let Some(arr_inner) = row.as_array() {
                    if arr_inner.len() < 2 {
                        continue;
                    }
                    let date = arr_inner[0].as_str().unwrap_or("").to_string();
                    if date.is_empty() {
                        continue;
                    }
                    if let Some(val) = arr_inner[6].as_f64() {
                        items.push(MacroDataPoint {
                            date,
                            value: val,
                            name: "Social Financing".to_string(),
                        });
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// RMB loan data (新增人民币贷款) from THS.
    pub async fn macro_rmb_loan(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://data.10jqka.com.cn/macro/loan/";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (compatible; akshare-rust/0.1)")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        // Parse HTML table - look for rows with date patterns
        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.contains("<td") && trimmed.contains('-') {
                // Simple extraction from HTML
                let text = extract_html_text(trimmed);
                if text.contains('-') && text.len() <= 10 {
                    // This might be a date cell
                }
            }
        }
        // Fallback: use regex-like approach on the raw HTML
        let re_rows: Vec<&str> = body.split("<tr").collect();
        for row in &re_rows[1..] {
            let cells: Vec<&str> = row.split("<td").collect();
            if cells.len() >= 3 {
                let date_cell = extract_html_text(cells[1]);
                let val_cell = extract_html_text(cells[2]);
                if date_cell.contains('-')
                    && let Ok(val) = val_cell.replace(',', "").parse::<f64>()
                {
                    items.push(MacroDataPoint {
                        date: date_cell,
                        value: val,
                        name: "RMB Loan".to_string(),
                    });
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// RMB deposit data (人民币存款余额) from THS.
    pub async fn macro_rmb_deposit(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://data.10jqka.com.cn/macro/rmb/";
        let body = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (compatible; akshare-rust/0.1)")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        let re_rows: Vec<&str> = body.split("<tr").collect();
        for row in &re_rows[1..] {
            let cells: Vec<&str> = row.split("<td").collect();
            if cells.len() >= 3 {
                let date_cell = extract_html_text(cells[1]);
                let val_cell = extract_html_text(cells[2]);
                if date_cell.contains('-')
                    && let Ok(val) = val_cell.replace(',', "").parse::<f64>()
                {
                    items.push(MacroDataPoint {
                        date: date_cell,
                        value: val,
                        name: "RMB Deposit".to_string(),
                    });
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// Jin10 energy daily report (中国日度沿海六大电库存数据).
    pub async fn macro_china_daily_energy(&self) -> Result<Vec<MacroDataPoint>> {
        // Use Jin10 CDN for energy daily data
        let url = "https://cdn.jin10.com/data_center/reports/energy.json";
        let resp: Jin10CdnCdnResp = self.get(url).send().await?.json().await?;

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
                                name: format!("Energy {col_name}"),
                            });
                        }
                    }
                }
            }
        }
        items.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(items)
    }

    /// 国家统计局-全国数据 (Python: macro_china_nbs_nation)
    pub async fn macro_china_nbs_nation(
        &self,
        kind: &str,
        path: &str,
        period: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let dbcode = match kind {
            "月度数据" => "hgyd",
            "季度数据" => "hgjd",
            "年度数据" => "hgnd",
            _ => {
                return Err(crate::error::Error::invalid_input(format!(
                    "invalid kind: {kind}"
                )));
            }
        };
        let url = "https://data.stats.gov.cn/easyquery.htm";
        let resp = self
            .post(url)
            .form(&[
                ("m", "QueryData"),
                ("dbcode", dbcode),
                ("rowcode", "zb"),
                ("colcode", "sj"),
                ("wds", "[]"),
                (
                    "dfwds",
                    format!("[{{\"wdcode\":\"zb\",\"valuecode\":\"{path}\"}}]").as_str(),
                ),
                ("k1", period),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let items = resp
            .get("returndata")
            .and_then(|r| r.get("datanodes"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(items
            .iter()
            .map(|v| {
                let date = v
                    .nested(&["wds"])
                    .and_then(|w| w.as_array())
                    .and_then(|a| a.get(2))
                    .and_then(|w| w.get("valuecode"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let value = v
                    .nested(&["data", "data"])
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0);
                MacroDataPoint {
                    date: date.to_string(),
                    value,
                    name: path.to_string(),
                }
            })
            .collect())
    }

    /// 国家统计局-地区数据 (Python: macro_china_nbs_region)
    pub async fn macro_china_nbs_region(
        &self,
        kind: &str,
        path: &str,
        indicator: &str,
        period: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let dbcode = match kind {
            "分省月度数据" => "fsyd",
            "分省季度数据" => "fsjd",
            "分省年度数据" => "fsnd",
            _ => {
                return Err(crate::error::Error::invalid_input(format!(
                    "invalid kind: {kind}"
                )));
            }
        };
        let url = "https://data.stats.gov.cn/easyquery.htm";
        let dfwds = format!(
            "[{{\"wdcode\":\"zb\",\"valuecode\":\"{path}\"}},{{\"wdcode\":\"reg\",\"valuecode\":\"{indicator}\"}}]"
        );
        let resp = self
            .post(url)
            .form(&[
                ("m", "QueryData"),
                ("dbcode", dbcode),
                ("rowcode", "reg"),
                ("colcode", "sj"),
                ("wds", "[]"),
                ("dfwds", dfwds.as_str()),
                ("k1", period),
            ])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let items = resp
            .get("returndata")
            .and_then(|r| r.get("datanodes"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(items
            .iter()
            .map(|v| {
                let date = v
                    .nested(&["wds"])
                    .and_then(|w| w.as_array())
                    .and_then(|a| a.get(2))
                    .and_then(|w| w.get("valuecode"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let value = v
                    .nested(&["data", "data"])
                    .and_then(serde_json::Value::as_f64)
                    .unwrap_or(0.0);
                MacroDataPoint {
                    date: date.to_string(),
                    value,
                    name: path.to_string(),
                }
            })
            .collect())
    }

    /// 国家统计局-城镇调查失业率 (Python: macro_china_urban_unemployment)
    pub async fn macro_china_urban_unemployment(&self) -> Result<Vec<MacroDataPoint>> {
        let url =
            "https://data.stats.gov.cn/dg/website/publicrelease/web/external/getEsDataByCidAndDt";
        let payload = serde_json::json!({
            "cid": "ee3b7046b390415b9b7745e3d16f6052",
            "indicatorIds": [
                "3888eac6062945a79c8a27e5f13d4953",
                "1d550f3ec77a463bb607d4a3427e1465",
                "1c1b2d9ab24048bfadc5c7d9510dc663"
            ],
            "daCatalogId": "",
            "das": [{"text": "全国", "value": "000000000000"}],
            "dt": "LAST60"
        });
        let resp = self
            .post(url)
            .json(&payload)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;
        let items = resp
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(items
            .iter()
            .map(|v| {
                let date = v.str_or(&["date"], "");
                let value = v.f64_or(&["value"], 0.0);
                MacroDataPoint {
                    date,
                    value,
                    name: "城镇调查失业率".to_string(),
                }
            })
            .collect())
    }
}
