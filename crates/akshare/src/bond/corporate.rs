//! Corporate bond issuance data from Eastmoney datacenter.
//!
//! Uses `RPT_BOND_ISSUE` report to fetch corporate bond issuance records.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::BondSnapshot;
use crate::types::value_ext::ValueExt;

impl AkShareClient {
    /// Fetch corporate bond issuance data.
    ///
    /// Returns bond issuance records with yield information from the
    /// Eastmoney datacenter (`RPT_BOND_ISSUE` report).
    pub async fn bond_corporate_yields(&self, limit: usize) -> Result<Vec<BondSnapshot>> {
        let data = self
            .dc_fetch_all(
                "RPT_BOND_ISSUE",
                "ALL",
                "",
                "ISSUE_DATE",
                "-1",
                limit.max(1) as i64,
                1,
                &[],
            )
            .await?;

        let today = crate::util::today_iso();
        let items: Vec<BondSnapshot> = data
            .into_iter()
            .take(limit)
            .filter_map(|v| {
                let symbol = v
                    .str_field(&["SECURITY_CODE", "BOND_CODE", "SECCODE"])?
                    .to_string();
                if symbol.is_empty() {
                    return None;
                }
                let name = v.str_or(
                    &["SECURITY_NAME_ABBR", "BOND_NAME", "SECNAME", "SHORT_NAME"],
                    &symbol,
                );

                let date = v
                    .str_field(&["ISSUE_DATE", "DECLAREDATE"])
                    .unwrap_or(&today);

                let close = v
                    .f64_field(&[
                        "ISSUE_PRICE",
                        "PAR_VALUE",
                        "FACE_VALUE",
                        "CURRENT_BOND_PRICE",
                    ])
                    .unwrap_or(100.0);

                let yield_rate =
                    v.f64_field(&["COUPON_RATE", "INTEREST_RATE", "YIELD_RATE", "ACTUAL_RATE"]);

                Some(BondSnapshot {
                    symbol,
                    name,
                    date: date.get(..10).unwrap_or(date).to_string(),
                    close,
                    change_pct: 0.0,
                    yield_rate,
                    credit_rating: None,
                })
            })
            .collect();

        Ok(items)
    }
}
