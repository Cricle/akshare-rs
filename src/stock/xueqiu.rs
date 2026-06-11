//! Xueqiu (雪球) stock data — individual stock spot data.
//!
//! Covers Python functions:
//! - `stock_individual_spot_xq` — Individual stock spot from Xueqiu

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct XqQuoteEnvelope {
    data: Option<XqQuoteData>,
}

#[derive(Debug, Deserialize)]
struct XqQuoteData {
    quote: Option<XqQuote>,
}

#[derive(Debug, Deserialize)]
struct XqQuote {
    #[serde(rename = "symbol")]
    symbol: Option<String>,
    #[serde(rename = "name")]
    name: Option<String>,
    #[serde(rename = "current")]
    current: Option<f64>,
    #[serde(rename = "percent")]
    percent: Option<f64>,
    #[serde(rename = "chg")]
    chg: Option<f64>,
    #[serde(rename = "open")]
    open: Option<f64>,
    #[serde(rename = "high")]
    high: Option<f64>,
    #[serde(rename = "low")]
    low: Option<f64>,
    #[serde(rename = "last_close")]
    last_close: Option<f64>,
    #[serde(rename = "volume")]
    volume: Option<f64>,
    #[serde(rename = "amount")]
    amount: Option<f64>,
    #[serde(rename = "amplitude")]
    amplitude: Option<f64>,
    #[serde(rename = "avg_price")]
    avg_price: Option<f64>,
    #[serde(rename = "turnover_rate")]
    turnover_rate: Option<f64>,
    #[serde(rename = "pe_ttm")]
    pe_ttm: Option<f64>,
    #[serde(rename = "pe_lyr")]
    pe_lyr: Option<f64>,
    #[serde(rename = "pb")]
    pb: Option<f64>,
    #[serde(rename = "psr")]
    psr: Option<f64>,
    #[serde(rename = "market_capital")]
    market_capital: Option<f64>,
    #[serde(rename = "float_market_capital")]
    float_market_capital: Option<f64>,
    #[serde(rename = "total_shares")]
    total_shares: Option<f64>,
    #[serde(rename = "float_shares")]
    float_shares: Option<f64>,
    #[serde(rename = "limit_up")]
    limit_up: Option<f64>,
    #[serde(rename = "limit_down")]
    limit_down: Option<f64>,
    #[serde(rename = "eps")]
    eps: Option<f64>,
    #[serde(rename = "navps")]
    navps: Option<f64>,
    #[serde(rename = "dividend")]
    dividend: Option<f64>,
    #[serde(rename = "dividend_yield")]
    dividend_yield: Option<f64>,
    #[serde(rename = "high52w")]
    high52w: Option<f64>,
    #[serde(rename = "low52w")]
    low52w: Option<f64>,
    #[serde(rename = "currency")]
    currency: Option<String>,
    #[serde(rename = "exchange")]
    exchange: Option<String>,
    #[serde(rename = "lot_size")]
    lot_size: Option<i64>,
    #[serde(rename = "time")]
    time: Option<i64>,
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Xueqiu individual stock spot data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XqStockSpot {
    pub symbol: String,
    pub name: String,
    #[serde(default)]
    pub current: Option<f64>,
    #[serde(default)]
    pub percent: Option<f64>,
    #[serde(default)]
    pub chg: Option<f64>,
    #[serde(default)]
    pub open: Option<f64>,
    #[serde(default)]
    pub high: Option<f64>,
    #[serde(default)]
    pub low: Option<f64>,
    #[serde(default)]
    pub last_close: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub amplitude: Option<f64>,
    #[serde(default)]
    pub avg_price: Option<f64>,
    #[serde(default)]
    pub turnover_rate: Option<f64>,
    #[serde(default)]
    pub pe_ttm: Option<f64>,
    #[serde(default)]
    pub pe_lyr: Option<f64>,
    #[serde(default)]
    pub pb: Option<f64>,
    #[serde(default)]
    pub psr: Option<f64>,
    #[serde(default)]
    pub market_capital: Option<f64>,
    #[serde(default)]
    pub float_market_capital: Option<f64>,
    #[serde(default)]
    pub total_shares: Option<f64>,
    #[serde(default)]
    pub float_shares: Option<f64>,
    #[serde(default)]
    pub limit_up: Option<f64>,
    #[serde(default)]
    pub limit_down: Option<f64>,
    #[serde(default)]
    pub eps: Option<f64>,
    #[serde(default)]
    pub navps: Option<f64>,
    #[serde(default)]
    pub dividend: Option<f64>,
    #[serde(default)]
    pub dividend_yield: Option<f64>,
    #[serde(default)]
    pub high52w: Option<f64>,
    #[serde(default)]
    pub low52w: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub exchange: Option<String>,
    #[serde(default)]
    pub lot_size: Option<i64>,
    #[serde(default)]
    pub timestamp: Option<i64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get individual stock spot data from Xueqiu.
    ///
    /// Python equivalent: `stock_individual_spot_xq(symbol)`
    ///
    /// `symbol` uses Xueqiu format like "SH600000", "SZ000001", "SPY".
    ///
    /// Note: This endpoint may require a valid `xq_a_token` cookie.
    /// The client does not automatically provide one — you may need to
    /// set cookies manually via a custom reqwest client.
    pub async fn stock_individual_spot_xq(&self, symbol: &str) -> Result<XqStockSpot> {
        let url = format!(
            "https://stock.xueqiu.com/v5/stock/quote.json?symbol={}&extend=detail",
            symbol
        );

        let response = self
                        .get(&url)
            .header("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1")
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: XqQuoteEnvelope = response.json().await.map_err(Error::from)?;
        let quote = payload
            .data
            .and_then(|d| d.quote)
            .ok_or_else(|| Error::upstream("xueqiu quote missing data"))?;

        Ok(XqStockSpot {
            symbol: quote.symbol.unwrap_or_default(),
            name: quote.name.unwrap_or_default(),
            current: quote.current,
            percent: quote.percent,
            chg: quote.chg,
            open: quote.open,
            high: quote.high,
            low: quote.low,
            last_close: quote.last_close,
            volume: quote.volume,
            amount: quote.amount,
            amplitude: quote.amplitude,
            avg_price: quote.avg_price,
            turnover_rate: quote.turnover_rate,
            pe_ttm: quote.pe_ttm,
            pe_lyr: quote.pe_lyr,
            pb: quote.pb,
            psr: quote.psr,
            market_capital: quote.market_capital,
            float_market_capital: quote.float_market_capital,
            total_shares: quote.total_shares,
            float_shares: quote.float_shares,
            limit_up: quote.limit_up,
            limit_down: quote.limit_down,
            eps: quote.eps,
            navps: quote.navps,
            dividend: quote.dividend,
            dividend_yield: quote.dividend_yield,
            high52w: quote.high52w,
            low52w: quote.low52w,
            currency: quote.currency,
            exchange: quote.exchange,
            lot_size: quote.lot_size,
            timestamp: quote.time,
        })
    }

    /// Get individual stock basic info from Xueqiu (A-share).
    ///
    /// Python equivalent: `stock_individual_basic_info_xq(symbol, token)`
    ///
    /// `symbol` uses Xueqiu format like "SH601127".
    /// `token` is the Xueqiu `xq_a_token` cookie value.
    pub async fn stock_individual_basic_info_xq(
        &self,
        symbol: &str,
        token: &str,
    ) -> Result<Vec<(String, String)>> {
        self.fetch_xq_basic_info("cn", symbol, token).await
    }

    /// Get individual stock basic info from Xueqiu (US).
    ///
    /// `symbol` is the US stock code, e.g. "NVDA".
    /// `token` is the Xueqiu `xq_a_token` cookie value.
    pub async fn stock_individual_basic_info_us_xq(
        &self,
        symbol: &str,
        token: &str,
    ) -> Result<Vec<(String, String)>> {
        self.fetch_xq_basic_info("us", symbol, token).await
    }

    /// Get individual stock basic info from Xueqiu (HK).
    ///
    /// `symbol` is the HK stock code, e.g. "02097".
    /// `token` is the Xueqiu `xq_a_token` cookie value.
    pub async fn stock_individual_basic_info_hk_xq(
        &self,
        symbol: &str,
        token: &str,
    ) -> Result<Vec<(String, String)>> {
        self.fetch_xq_basic_info("hk", symbol, token).await
    }

    async fn fetch_xq_basic_info(
        &self,
        market: &str,
        symbol: &str,
        token: &str,
    ) -> Result<Vec<(String, String)>> {
        let url = format!("https://stock.xueqiu.com/v5/stock/f10/{market}/company.json");
        let response = self
            .get(&url)
            .query(&[("symbol", symbol)])
            .header("Cookie", format!("xq_a_token={token};"))
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?
            .error_for_status()?;

        let payload: serde_json::Value = response.json().await?;
        let data = payload
            .get("data")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        Ok(data
            .into_iter()
            .map(|(k, v)| {
                let val = match v {
                    serde_json::Value::String(s) => s,
                    other => other.to_string(),
                };
                (k, val)
            })
            .collect())
    }
}
