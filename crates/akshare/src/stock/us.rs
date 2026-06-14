use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{CandlePoint, QuoteSnapshot};

impl AkShareClient {
    /// Get US stock quote. Derived from the latest candle of `us_candles`.
    pub async fn us_quote(&self, symbol: &str) -> Result<QuoteSnapshot> {
        let mut candles = self.us_candles(symbol, 2).await?;
        let last = candles
            .pop()
            .ok_or_else(|| Error::upstream("no US quote data"))?;
        Ok(QuoteSnapshot {
            symbol: symbol.to_uppercase(),
            date: last.trade_date,
            open: last.open,
            high: last.high,
            low: last.low,
            close: last.close,
            volume: last.volume,
        })
    }

    /// Get US stock candles with fallback: Sina -> Yahoo -> Stooq
    pub async fn us_candles(&self, symbol: &str, limit: usize) -> Result<Vec<CandlePoint>> {
        // Try Sina first
        match self.sina_us_daily(symbol, limit).await {
            Ok(items) if !items.is_empty() => return Ok(items),
            _ => {}
        }

        // Try Yahoo
        match self.yahoo_candles(symbol, limit).await {
            Ok(items) if !items.is_empty() => return Ok(items),
            _ => {}
        }

        // Fallback to Stooq
        self.stooq_candles(symbol, limit).await
    }

    /// Get market cap for a specific US stock from Sina Finance.
    pub async fn us_market_cap_from_sina(&self, symbol: &str) -> Result<Option<f64>> {
        let url = "https://stock.finance.sina.com.cn/usstock/api/jsonp.php/callback/US_CategoryService.getList?page=1&num=200&sort=&asc=0&market=&id=";
        let resp = self
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;
        // Parse JSONP response: callback({...});
        if let Some(start) = resp.find("({")
            && let Some(end) = resp.rfind("})")
        {
            let json_str = &resp[start + 1..=end];
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(json_str)
                && let Some(arr) = data.get("data").and_then(|d| d.as_array())
            {
                for item in arr {
                    if let Some(code) = item.get("symbol").and_then(|v| v.as_str())
                        && code.eq_ignore_ascii_case(symbol)
                    {
                        // Try string first, then number
                        let mktcap = item.get("mktcap").and_then(|v| {
                            v.as_str()
                                .and_then(|s| s.parse::<f64>().ok())
                                .or_else(|| v.as_f64())
                        });
                        return Ok(mktcap);
                    }
                }
            }
        }
        Ok(None)
    }

    /// 获取美国股票名称列表 (Python: get_us_stock_name)
    ///
    /// Fetches US stock names from Sina Finance.
    /// Returns stock name, Chinese name, and symbol.
    pub async fn get_us_stock_name(&self) -> Result<Vec<serde_json::Value>> {
        let mut all_stocks = Vec::new();
        for page in 1..=5 {
            let url = format!(
                "https://stock.finance.sina.com.cn/usstock/api/jsonp.php/callback/US_CategoryService.getList?page={page}&num=20&sort=&asc=0&market=&id="
            );
            let resp = self
                .get(&url)
                .header("User-Agent", "Mozilla/5.0")
                .send()
                .await?
                .text()
                .await?;
            // Parse JSONP response
            if let Some(start) = resp.find("({")
                && let Some(end) = resp.rfind(");")
            {
                let json_str = &resp[start + 1..=end];
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(json_str)
                    && let Some(arr) = data.get("data").and_then(|d| d.as_array())
                {
                    if arr.is_empty() {
                        break;
                    }
                    all_stocks.extend(arr.clone());
                }
            }
        }
        Ok(all_stocks)
    }
}
