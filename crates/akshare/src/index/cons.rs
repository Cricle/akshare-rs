//! Index constituent data.
//!
//! Some functions (CSIndex XLS, Sina HTML scraping) require special handling
//! that is not feasible in pure Rust without additional libraries.
//! These are implemented as stubs that return a clear error.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// 新浪 — 股票指数成份股 (新版接口).
    ///
    /// Only supports HS300 via paginated JSON.
    pub async fn index_stock_cons_sina(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        if symbol == "000300" {
            // HS300 uses a special endpoint
            let count_resp = self
                                .get("https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeStockCountSimple")
                .query(&[("node", "hs300")])
                .send()
                .await
                .map_err(Error::from)?
                .error_for_status()
                .map_err(Error::from)?;

            let count_text = count_resp.text().await.map_err(Error::from)?;
            let count: i64 = count_text
                .trim_matches(|c: char| !c.is_ascii_digit())
                .parse()
                .unwrap_or(0);
            let pages = (count / 80) + 1;

            let mut all = Vec::new();
            for page in 1..=pages {
                let resp = self
                                        .get("https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData")
                    .query(&[
                        ("page", page.to_string().as_str()),
                        ("num", "80"),
                        ("sort", "symbol"),
                        ("asc", "1"),
                        ("node", "hs300"),
                        ("symbol", ""),
                        ("_s_r_a", "init"),
                    ])
                    .send()
                    .await
                    .map_err(Error::from)?
                    .error_for_status()
                    .map_err(Error::from)?;

                let items: Vec<serde_json::Value> = resp.json().await.map_err(Error::from)?;
                all.extend(items);
            }

            if all.is_empty() {
                return Err(Error::not_found("sina returned no HS300 constituents"));
            }
            return Ok(all);
        }

        // General case
        let response = self
                        .get("https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeDataSimple")
            .query(&[
                ("page", "1"),
                ("num", "3000"),
                ("sort", "symbol"),
                ("asc", "1"),
                ("node", &format!("zhishu_{symbol}")),
                ("_s_r_a", "setlen"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let items: Vec<serde_json::Value> = response.json().await.map_err(Error::from)?;
        if items.is_empty() {
            return Err(Error::not_found(format!(
                "sina returned no constituents for {symbol}"
            )));
        }
        Ok(items)
    }

    /// 聚宽 — 指数列表.
    ///
    /// Returns index code, display name, and publish date.
    pub async fn index_stock_info(&self) -> Result<Vec<IndexInfoItem>> {
        let response = self
            .get("https://www.joinquant.com/data/dict/indexData")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let html = response.text().await.map_err(Error::from)?;

        // Parse HTML table — extract rows between <tr> tags
        let mut items = Vec::new();
        for tr_match in html.split("<tr>").skip(1) {
            let tds: Vec<&str> = tr_match.split("<td>").skip(1).collect();
            if tds.len() >= 3 {
                let code = tds[0].split('<').next().unwrap_or("").trim().to_string();
                let name = tds[1].split('<').next().unwrap_or("").trim().to_string();
                let date = tds[2].split('<').next().unwrap_or("").trim().to_string();
                // Strip exchange suffix from code (e.g. "000300.SH" -> "000300")
                let code_clean = code.split('.').next().unwrap_or(&code).to_string();
                if !code_clean.is_empty() && code_clean.chars().all(|c| c.is_ascii_digit()) {
                    items.push(IndexInfoItem {
                        code: code_clean,
                        name,
                        publish_date: date,
                    });
                }
            }
        }

        if items.is_empty() {
            return Err(Error::not_found("joinquant returned no index info"));
        }
        Ok(items)
    }

    /// 新浪 — 股票指数成份股 (老接口).
    ///
    /// Scrapes HTML pages; returns raw JSON values.
    pub async fn index_stock_cons(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        Err(Error::upstream(format!(
            "index_stock_cons for {symbol} requires HTML scraping with pagination; \
             use index_stock_cons_sina() or a dedicated HTML parser"
        )))
    }

    /// 中证指数网站 — 成份股目录.
    ///
    /// The upstream returns an XLS file which cannot be parsed in pure Rust.
    pub async fn index_stock_cons_csindex(&self, _symbol: &str) -> Result<Vec<serde_json::Value>> {
        Err(Error::upstream(
            "csindex constituents are returned as XLS — not supported in pure Rust",
        ))
    }

    /// 中证指数网站 — 样本权重.
    ///
    /// The upstream returns an XLS file which cannot be parsed in pure Rust.
    pub async fn index_stock_cons_weight_csindex(
        &self,
        _symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        Err(Error::upstream(
            "csindex weights are returned as XLS — not supported in pure Rust",
        ))
    }
}

/// Index info item from JoinQuant.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexInfoItem {
    pub code: String,
    pub name: String,
    pub publish_date: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Cons functions require network access.
    }
}
