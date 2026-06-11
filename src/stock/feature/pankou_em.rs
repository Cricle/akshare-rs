//! Order book changes (盘口异动) from Eastmoney.

use super::types::*;
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 盘口异动
    pub async fn stock_changes_em(&self, symbol: &str) -> Result<Vec<PankouChange>> {
        let data = self
            .push2ex_fetch(
                "getChangesList",
                &[
                    ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
                    ("fields1", "f1,f2,f3,f4,f12,f13,f14"),
                    ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58"),
                    ("mpi", "1000"),
                    ("pos", "-1"),
                    ("secid", symbol),
                ],
            )
            .await?;

        let diff = data
            .get("data")
            .and_then(|d| d.get("details"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(diff
            .iter()
            .filter_map(|v| {
                let s = v.as_str()?;
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() >= 5 {
                    Some(PankouChange {
                        time: parts[0].to_string(),
                        code: parts[1].to_string(),
                        name: parts[2].to_string(),
                        board: parts[3].to_string(),
                        info: parts.get(4).map(|s| s.to_string()),
                    })
                } else {
                    None
                }
            })
            .collect())
    }
}
