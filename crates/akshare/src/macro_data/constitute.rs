//! Global macro constitue data — OPEC, gold ETF, silver ETF from Jin10.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;

use super::shared::Jin10Resp;

// ---------------------------------------------------------------------------
// Gold / Silver ETF
// ---------------------------------------------------------------------------

/// Fetch ETF holding data from Jin10 (gold or silver).
async fn fetch_etf_holding(
    client: &AkShareClient,
    attr_id: &str,
    name_label: &str,
) -> Result<Vec<MacroDataPoint>> {
    let url = "https://datacenter-api.jin10.com/reports/list_v2";
    let resp: Jin10Resp = client
        .get(url)
        .query(&[("max_date", ""), ("category", "etf"), ("attr_id", attr_id)])
        .header("x-app-id", "rU6QIu7JHe2gOUeR")
        .header("x-csrf-token", "x-csrf-token")
        .header("x-version", "1.0.0")
        .send()
        .await?
        .json()
        .await?;

    let values = resp.data.map(|d| d.values).unwrap_or_default();
    let mut items = Vec::with_capacity(values.len());
    for row in &values {
        if row.len() < 2 {
            continue;
        }
        let date = row[0].as_str().unwrap_or("").to_string();
        // Column 1 is total holdings
        let value = row[1].as_f64().unwrap_or(0.0);
        if date.is_empty() {
            continue;
        }
        items.push(MacroDataPoint {
            date: date.get(..10).unwrap_or(&date).to_string(),
            value,
            name: name_label.to_string(),
        });
    }
    Ok(items)
}

impl AkShareClient {
    /// SPDR Gold Trust ETF holdings (全球最大黄金ETF持仓).
    pub async fn gold_etf_holding(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_etf_holding(self, "1", "Gold ETF Holding").await
    }

    /// iShares Silver Trust ETF holdings (全球最大白银ETF持仓).
    pub async fn silver_etf_holding(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_etf_holding(self, "2", "Silver ETF Holding").await
    }

    // Python-compatible aliases
    /// Fetch cons gold macro data.
    pub async fn macro_cons_gold(&self) -> Result<Vec<MacroDataPoint>> {
        self.gold_etf_holding().await
    }

    /// Fetch cons silver macro data.
    pub async fn macro_cons_silver(&self) -> Result<Vec<MacroDataPoint>> {
        self.silver_etf_holding().await
    }

    /// OPEC monthly report (欧佩克报告-月度).
    pub async fn macro_cons_opec_month(&self) -> Result<Vec<MacroDataPoint>> {
        let url = "https://datacenter-api.jin10.com/reports/dates";
        let dates_resp: serde_json::Value = self
            .get(url)
            .query(&[("category", "opec")])
            .header("x-app-id", "rU6QIu7JHe2gOUeR")
            .header("x-csrf-token", "x-csrf-token")
            .header("x-version", "1.0.0")
            .send()
            .await?
            .json()
            .await?;

        let date_list = dates_resp
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        // Only fetch the most recent 5 dates to avoid excessive API calls
        let dates_to_fetch: Vec<_> = if date_list.len() > 5 {
            date_list[date_list.len() - 5..].to_vec()
        } else {
            date_list
        };

        for date_val in &dates_to_fetch {
            let date_str = date_val.as_str().unwrap_or("");
            if date_str.is_empty() {
                continue;
            }
            let list_url = "https://datacenter-api.jin10.com/reports/list";
            let resp: serde_json::Value = self
                .get(list_url)
                .query(&[("category", "opec"), ("date", date_str)])
                .header("x-app-id", "rU6QIu7JHe2gOUeR")
                .header("x-csrf-token", "x-csrf-token")
                .header("x-version", "1.0.0")
                .send()
                .await?
                .json()
                .await?;

            if let Some(data) = resp.get("data")
                && let Some(values) = data.get("values").and_then(|v| v.as_array())
                && let Some(keys) = data.get("keys").and_then(|k| k.as_array())
            {
                let col_names: Vec<String> = keys
                    .iter()
                    .filter_map(|k| k.str_field(&["name"]).map(String::from))
                    .collect();
                // Get last row (total OPEC production)
                if let Some(last_row) = values.last().and_then(|r| r.as_array()) {
                    for (i, col_name) in col_names.iter().enumerate() {
                        if let Some(val) = last_row.get(i).and_then(serde_json::Value::as_f64) {
                            items.push(MacroDataPoint {
                                date: date_str.to_string(),
                                value: val,
                                name: format!("OPEC {col_name}"),
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
