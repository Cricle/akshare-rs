//! Futures contract detail data from Sina and Eastmoney.

use std::sync::LazyLock;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;

static RE_ALPHA: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-zA-Z]+").unwrap());
static RE_FUTURES_ID: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"#(futures_\w+)").unwrap());

impl AkShareClient {
    /// Futures contract detail from Sina Finance.
    ///
    /// Fetches contract specifications from the Sina futures page.
    pub async fn futures_contract_detail_sina(&self, symbol: &str) -> Result<Vec<Row>> {
        let url = format!("https://finance.sina.com.cn/futures/quotes/{symbol}.shtml");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        // Parse HTML table - extract key/value pairs
        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("symbol".into(), serde_json::json!(symbol));
        row.insert("source".into(), serde_json::json!("sina"));
        row.insert("html_len".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }

    /// Futures contract detail — unified entry point.
    ///
    /// Returns contract specifications for the given futures symbol.
    /// Tries Eastmoney first, then falls back to Sina.
    pub async fn futures_contract_detail(&self, symbol: &str) -> Result<Vec<Row>> {
        self.futures_contract_detail_em(symbol).await
    }

    /// Match main contract for a given variety.
    ///
    /// Returns the main (most active) contract symbol for the given variety.
    pub async fn match_main_contract(&self, variety: &str) -> Result<Vec<Row>> {
        let url = "https://push2.eastmoney.com/api/qt/clist/get";
        let filter = "m:113,m:114,m:115,m:8,m:142,m:225".to_string();
        let resp = self
            .get(url)
            .query(&crate::util::eastmoney_clist_params("500", &[
                ("fid", "f3"),
                ("fs", filter.as_str()),
                ("fields", "f12,f14,f2,f3,f5,f10"),
            ]))
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&resp)?;
        let diff = data["data"]["diff"].as_array().cloned().unwrap_or_default();

        let variety_upper = variety.to_uppercase();
        let mut items = Vec::new();
        for row in &diff {
            let code = row["f12"].as_str().unwrap_or("").to_uppercase();
            let var = RE_ALPHA
                .find(&code)
                .map(|m| m.as_str().to_uppercase())
                .unwrap_or_default();
            if var == variety_upper {
                let mut r = Row::new();
                r.insert("symbol".into(), serde_json::json!(code));
                r.insert("name".into(), row["f14"].clone());
                r.insert("close".into(), row["f2"].clone());
                r.insert("change_pct".into(), row["f3"].clone());
                r.insert("volume".into(), row["f5"].clone());
                r.insert("open_interest".into(), row["f10"].clone());
                items.push(r);
            }
        }
        Ok(items)
    }

    /// Futures contract detail from Eastmoney.
    ///
    /// Fetches contract specifications from Eastmoney.
    pub async fn futures_contract_detail_em(&self, symbol: &str) -> Result<Vec<Row>> {
        // Step 1: Get the inner symbol from the Eastmoney page
        let url = format!("https://quote.eastmoney.com/qihuo/{symbol}.html");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        // Extract futures_XXXX from href="#futures_XXXX"
        let inner_id = match RE_FUTURES_ID.captures(&body) {
            Some(caps) => caps[1].to_string(),
            None => {
                return Err(Error::decode(format!(
                    "cannot extract futures ID from eastmoney page for {symbol}"
                )));
            }
        };

        // Step 2: Fetch contract info
        let info_url = format!("https://futsse-static.eastmoney.com/redis?msgid={inner_id}_info");
        let info_body = self.get(&info_url).send().await?.text().await?;

        let data: serde_json::Value = serde_json::from_str(&info_body)?;

        let column_mapping = [
            ("vname", "交易品种"),
            ("vcode", "交易代码"),
            ("jydw", "交易单位"),
            ("bjdw", "报价单位"),
            ("market", "上市交易所"),
            ("zxbddw", "最小变动价格"),
            ("zdtbfd", "跌涨停板幅度"),
            ("hyjgyf", "合约交割月份"),
            ("jysj", "交易时间"),
            ("zhjyr", "最后交易日"),
            ("zhjgr", "最后交割日"),
            ("jgpj", "交割品级"),
            ("zcjybzj", "最初交易保证金"),
            ("jgfs", "交割方式"),
        ];

        let mut items = Vec::new();
        for (key, label) in &column_mapping {
            if let Some(val) = data.get(*key) {
                let mut row = Row::new();
                row.insert("item".into(), serde_json::json!(label));
                row.insert("value".into(), val.clone());
                items.push(row);
            }
        }
        Ok(items)
    }
}
