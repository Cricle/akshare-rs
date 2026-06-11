//! Fund scale data from Sina Finance.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::FundSnapshot;

impl AkShareClient {
    /// Fetch open-end fund scale data from Sina Finance.
    pub async fn fund_scale_open_sina(&self, symbol: &str) -> Result<Vec<FundSnapshot>> {
        let type_map: &[(&str, &str)] = &[
            ("股票型基金", "2"),
            ("混合型基金", "1"),
            ("债券型基金", "3"),
            ("货币型基金", "5"),
            ("QDII基金", "6"),
        ];
        let type_code = type_map
            .iter()
            .find(|(n, _)| *n == symbol)
            .map(|(_, c)| *c)
            .ok_or_else(|| Error::invalid_input(format!("unknown fund type: {symbol}")))?;

        let resp = self
            .get("http://vip.stock.finance.sina.com.cn/fund_center/data/jsonp.php/IO.XSRV2.CallbackList['J2cW8KXheoWKdSHc']/NetValueReturn_Service.NetValueReturnOpen")
            .query(&[
                ("page", "1"), ("num", "10000"), ("sort", "zmjgm"), ("asc", "0"),
                ("ccode", ""), ("type2", type_code), ("type3", ""),
            ])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;

        let text = resp.text().await.map_err(Error::from)?;
        let json_start = text.find("({").map(|i| i + 1).unwrap_or(0);
        let json_end = text.rfind('}').map(|i| i + 1).unwrap_or(text.len());
        let json_str = &text[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("sina fund scale JSON parse: {e}")))?;

        let data = root
            .get("data")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("sina fund scale missing data"))?;

        let snapshots: Vec<FundSnapshot> = data
            .iter()
            .filter_map(|item| {
                let code = item.get("symbol")?.as_str()?.to_string();
                let name = item.get("sname")?.as_str()?.to_string();
                let nav = item.get("dwjz").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let scale = item.get("zmjgm").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let date = item
                    .get("jzrq")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                Some(FundSnapshot {
                    symbol: code,
                    name,
                    date,
                    nav,
                    acc_nav: scale,
                    change_pct: 0.0,
                    fund_type: Some(symbol.to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found(format!("no fund scale data for {symbol}")));
        }
        Ok(snapshots)
    }

    /// Fetch closed-end fund scale data from Sina Finance.
    pub async fn fund_scale_close_sina(&self) -> Result<Vec<FundSnapshot>> {
        let resp = self
            .get("http://vip.stock.finance.sina.com.cn/fund_center/data/jsonp.php/IO.XSRV2.CallbackList['J2cW8KXheoWKdSHc']/NetValueReturn_Service.NetValueReturnClose")
            .query(&[("page", "1"), ("num", "10000"), ("sort", "zmjgm"), ("asc", "0")])
            .send().await.map_err(Error::from)?
            .error_for_status().map_err(Error::from)?;

        let text = resp.text().await.map_err(Error::from)?;
        let json_start = text.find("({").map(|i| i + 1).unwrap_or(0);
        let json_end = text.rfind('}').map(|i| i + 1).unwrap_or(text.len());
        let json_str = &text[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("sina close fund scale JSON parse: {e}")))?;

        let data = root
            .get("data")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::decode("sina close fund scale missing data"))?;

        let snapshots: Vec<FundSnapshot> = data
            .iter()
            .filter_map(|item| {
                let code = item.get("symbol")?.as_str()?.to_string();
                let name = item.get("sname")?.as_str()?.to_string();
                let nav = item.get("dwjz").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let scale = item.get("zmjgm").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let date = item
                    .get("jzrq")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                Some(FundSnapshot {
                    symbol: code,
                    name,
                    date,
                    nav,
                    acc_nav: scale,
                    change_pct: 0.0,
                    fund_type: Some("closed".to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found("no closed fund scale data"));
        }
        Ok(snapshots)
    }

    /// Fetch money market fund scale data from Sina Finance.
    pub async fn fund_scale_money_sina(&self) -> Result<Vec<FundSnapshot>> {
        self.fund_scale_open_sina("货币型基金").await
    }

    /// Fetch structured fund scale data from Sina (Python: fund_scale_structured_sina).
    pub async fn fund_scale_structured_sina(&self) -> Result<Vec<FundSnapshot>> {
        // Structured funds use the close fund endpoint
        self.fund_scale_close_sina().await
    }
}
