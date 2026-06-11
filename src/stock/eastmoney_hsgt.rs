#![allow(dead_code)]
//! Eastmoney HSGT (沪深港通) data — northbound/southbound flows, quota.
//!
//! Covers Python functions:
//! - `stock_hsgt_fund_flow_summary_em` — HSGT fund flow summary
//! - `stock_hsgt_north_net_flow_in_em` — Northbound net flow
//! - `stock_hsgt_south_net_flow_in_em` — Southbound net flow
//! - `stock_hsgt_north_acc_flow_in_em` — Northbound accumulated flow
//! - `stock_hsgt_south_acc_flow_in_em` — Southbound accumulated flow

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct FundFlowEnvelope {
    data: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// HSGT fund flow summary entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsgtFlowEntry {
    pub date: String,
    #[serde(default)]
    pub north_net_flow: Option<f64>,
    #[serde(default)]
    pub south_net_flow: Option<f64>,
    #[serde(default)]
    pub north_acc_flow: Option<f64>,
    #[serde(default)]
    pub south_acc_flow: Option<f64>,
    #[serde(default)]
    pub sh_index: Option<f64>,
    #[serde(default)]
    pub sh_change_pct: Option<f64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get HSGT fund flow summary (KAMT kline) from Eastmoney.
    ///
    /// For the comprehensive datacenter version, see `feature::hsgt_em`.
    pub async fn stock_hsgt_kamt_flow_em(&self, limit: usize) -> Result<Vec<HsgtFlowEntry>> {
        self.fetch_hsgt_flow(
            "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64",
            limit,
        )
        .await
    }

    /// Get northbound net flow data from Eastmoney (KAMT kline).
    pub async fn stock_hsgt_north_net_flow_kamt_em(
        &self,
        limit: usize,
    ) -> Result<Vec<HsgtFlowEntry>> {
        self.fetch_hsgt_flow(
            "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64",
            limit,
        )
        .await
    }

    /// Get southbound net flow data from Eastmoney (KAMT kline).
    pub async fn stock_hsgt_south_net_flow_kamt_em(
        &self,
        limit: usize,
    ) -> Result<Vec<HsgtFlowEntry>> {
        self.fetch_hsgt_flow(
            "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64",
            limit,
        )
        .await
    }

    // -- Private helpers ----------------------------------------------------

    async fn fetch_hsgt_flow(&self, fields: &str, limit: usize) -> Result<Vec<HsgtFlowEntry>> {
        let lmt = limit.to_string();
        let response = self
            .get("https://push2his.eastmoney.com/api/qt/kamt.kline/get")
            .query(&[
                ("fields1", "f1,f2,f3,f7"),
                ("fields2", fields),
                ("klt", "101"),
                ("lmt", lmt.as_str()),
                ("ut", "b2884a393a59ad64002292a3e90d46a5"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;

        let data = payload
            .get("data")
            .ok_or_else(|| Error::upstream("eastmoney HSGT flow missing data"))?;

        let s2n = data
            .get("s2n")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut entries = Vec::new();
        for item in &s2n {
            if let Some(arr) = item.as_array()
                && arr.len() >= 3
            {
                entries.push(HsgtFlowEntry {
                    date: arr[0].as_str().unwrap_or("").to_string(),
                    north_net_flow: arr.get(1).and_then(|v| v.as_f64()),
                    south_net_flow: arr.get(2).and_then(|v| v.as_f64()),
                    north_acc_flow: None,
                    south_acc_flow: None,
                    sh_index: arr.get(3).and_then(|v| v.as_f64()),
                    sh_change_pct: arr.get(4).and_then(|v| v.as_f64()),
                });
            }
        }

        if entries.is_empty() {
            return Err(Error::not_found("eastmoney returned no HSGT flow items"));
        }

        entries.truncate(limit);
        Ok(entries)
    }
}
