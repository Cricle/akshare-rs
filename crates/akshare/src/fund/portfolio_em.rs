//! Fund portfolio data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Fetch fund portfolio holdings from Eastmoney.
    pub async fn fund_portfolio_hold_em(
        &self,
        symbol: &str,
        _date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let resp = self
            .get("https://fundf10.eastmoney.com/FundArchivesDatas.aspx")
            .query(&[
                ("type", "jjcc"),
                ("code", symbol),
                ("topline", "10"),
                ("year", ""),
                ("month", ""),
                ("rt", "0.123"),
            ])
            .header(
                "Referer",
                format!("https://fundf10.eastmoney.com/ccmx_{symbol}.html").as_str(),
            )
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let _text = resp.text().await.map_err(Error::from)?;
        Err(Error::decode(format!(
            "fund portfolio requires HTML parsing for {symbol}"
        )))
    }

    /// Fetch fund bond holdings from Eastmoney.
    pub async fn fund_portfolio_bond_hold_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.fund_portfolio_hold_em(symbol, "").await
    }

    /// Fetch fund asset allocation from Eastmoney.
    pub async fn fund_portfolio_asset_allocation_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.fund_portfolio_hold_em(symbol, "").await
    }

    /// Fetch fund portfolio industry allocation (Python: fund_portfolio_industry_allocation_em).
    pub async fn fund_portfolio_industry_allocation_em(
        &self,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let response = self
            .get("https://api.fund.eastmoney.com/f10/HYPZ/")
            .header("Referer", "https://fundf10.eastmoney.com/")
            .query(&[
                ("fundCode", symbol),
                ("year", ""),
                ("callback", "jQuery183006997159478989867_1648016188499"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        // Response is JSONP; extract JSON
        let json_start = text.find('{').unwrap_or(0);
        let json_end = text.rfind('}').map_or(text.len(), |i| i + 1);
        let json_str = &text[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("industry alloc JSON parse: {e}")))?;

        let quarters = root
            .get("Data")
            .and_then(|d| d.get("QuarterInfos"))
            .and_then(|q| q.as_array())
            .ok_or_else(|| Error::not_found(format!("no industry allocation for {symbol}")))?;

        let mut result = Vec::new();
        for quarter in quarters {
            let items = quarter
                .get("HYPZInfo")
                .and_then(|h| h.as_array())
                .cloned()
                .unwrap_or_default();
            for item in items {
                result.push(item);
            }
        }
        if result.is_empty() {
            return Err(Error::not_found(format!(
                "no industry allocation for {symbol}"
            )));
        }
        Ok(result)
    }

    /// Fetch fund portfolio major changes (Python: fund_portfolio_change_em).
    ///
    /// `symbol`: fund code.
    /// `indicator`: "累计买入" or "累计卖出".
    pub async fn fund_portfolio_change_em(
        &self,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let zdbd = match indicator {
            "累计买入" => "1",
            "累计卖出" => "2",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported indicator: {indicator}"
                )));
            }
        };

        let response = self
            .get("https://fundf10.eastmoney.com/FundArchivesDatas.aspx")
            .query(&[
                ("type", "zdbd"),
                ("code", symbol),
                ("zdbd", zdbd),
                ("year", ""),
                ("rt", "0.913877030254846"),
            ])
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let text = response.text().await.map_err(Error::from)?;
        let json_start = text.find('{').unwrap_or(0);
        let json_end = text.rfind('}').map_or(text.len(), |i| i + 1);
        let json_str = &text[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("portfolio change JSON parse: {e}")))?;

        let content = root
            .get("content")
            .and_then(|c| c.as_str())
            .ok_or_else(|| Error::not_found(format!("no portfolio change data for {symbol}")))?;

        // Content is HTML; we return it as a raw value for now.
        Ok(vec![serde_json::json!({
            "fund_code": symbol,
            "indicator": indicator,
            "html_content": content,
        })])
    }
}
