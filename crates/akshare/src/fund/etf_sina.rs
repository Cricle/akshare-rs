//! ETF fund data from Sina Finance.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundSnapshot;

impl AkShareClient {
    /// Fetch ETF/LOF/closed fund category list from Sina Finance.
    ///
    /// `symbol` is one of: "封闭式基金", "ETF基金", "LOF基金".
    pub async fn fund_etf_category_sina(&self, symbol: &str) -> Result<Vec<FundSnapshot>> {
        let fund_map: &[(&str, &str)] = &[
            ("封闭式基金", "close_fund"),
            ("ETF基金", "etf_hq_fund"),
            ("LOF基金", "lof_hq_fund"),
        ];
        let node = fund_map
            .iter()
            .find(|(n, _)| *n == symbol)
            .map(|(_, c)| *c)
            .ok_or_else(|| Error::invalid_input(format!("unknown fund category: {symbol}")))?;

        let resp = self
            .get("https://vip.stock.finance.sina.com.cn/quotes_service/api/jsonp.php/IO.XSRV2.CallbackList['da_yPT46_Ll7K6WD']/Market_Center.getHQNodeDataSimple")
            .query(&[
                ("page", "1"),
                ("num", "5000"),
                ("sort", "symbol"),
                ("asc", "0"),
                ("node", node),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = resp.text().await.map_err(Error::from)?;
        let json_start = text.find("([").map(|i| i + 1).unwrap_or(0);
        let json_end = text.rfind("])").map(|i| i + 1).unwrap_or(text.len());
        let json_str = &text[json_start..json_end];

        let items: Vec<serde_json::Value> = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("sina ETF category JSON parse: {e}")))?;

        let today = crate::util::today_iso();
        let snapshots: Vec<FundSnapshot> = items
            .into_iter()
            .filter_map(|v| {
                let arr = v.as_array()?;
                if arr.len() < 5 {
                    return None;
                }
                Some(FundSnapshot {
                    symbol: arr[0].as_str().unwrap_or("").to_string(),
                    name: arr[1].as_str().unwrap_or("").to_string(),
                    date: today.clone(),
                    nav: arr[2]
                        .as_f64()
                        .or_else(|| arr[2].as_str().and_then(|s| s.parse().ok()))
                        .unwrap_or(0.0),
                    acc_nav: 0.0,
                    change_pct: arr[4]
                        .as_f64()
                        .or_else(|| arr[4].as_str().and_then(|s| s.parse().ok()))
                        .unwrap_or(0.0),
                    fund_type: Some(symbol.to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found(format!("sina returned no {symbol} data")));
        }
        Ok(snapshots)
    }

    /// Fetch ETF historical data from Sina Finance.
    pub async fn fund_etf_hist_sina(&self, symbol: &str) -> Result<Vec<crate::types::CandlePoint>> {
        Err(Error::decode(format!(
            "sina ETF hist for {symbol} requires JS decryption; use fund_etf_hist instead"
        )))
    }
}
