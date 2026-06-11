//! Global index candlestick data via Yahoo Finance.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::CandlePoint;

/// Map common global index names to Yahoo Finance symbols.
fn yahoo_index_symbol(symbol: &str) -> String {
    let s = symbol.trim().to_uppercase();
    match s.as_str() {
        "SP500" | "SPX" | "S&P500" | "S&P_500" | "标普500" => "^GSPC".to_string(),
        "NASDAQ" | "IXIC" | "COMP" | "纳斯达克" => "^IXIC".to_string(),
        "DOW" | "DJI" | "道琼斯" => "^DJI".to_string(),
        "NIKKEI" | "N225" | "日经225" => "^N225".to_string(),
        "HANGSENG" | "HSI" | "恒生指数" => "^HSI".to_string(),
        "DAX" | "德国DAX" => "^GDAXI".to_string(),
        "FTSE" | "富时100" => "^FTSE".to_string(),
        _ => {
            // If it already starts with ^, use as-is; otherwise treat as yahoo symbol
            if s.starts_with('^') || s.contains('.') {
                symbol.to_string()
            } else {
                format!("^{s}")
            }
        }
    }
}

impl AkShareClient {
    /// Global index name table (Yahoo Finance version).
    ///
    /// Returns the mapping of common global index names to Yahoo Finance symbols.
    pub async fn index_global_name_table_yahoo(&self) -> Result<Vec<crate::types::Row>> {
        let pairs = [
            ("SP500", "^GSPC", "标普500"),
            ("NASDAQ", "^IXIC", "纳斯达克"),
            ("DOW", "^DJI", "道琼斯"),
            ("NIKKEI", "^N225", "日经225"),
            ("HANGSENG", "^HSI", "恒生指数"),
            ("DAX", "^GDAXI", "德国DAX"),
            ("FTSE", "^FTSE", "富时100"),
            ("CAC40", "^FCHI", "法国CAC40"),
            ("KOSPI", "^KS11", "韩国KOSPI"),
            ("ASX200", "^AXJO", "澳大利亚ASX200"),
            ("SENSEX", "^BSE_Sensex", "印度SENSEX"),
            ("TSX", "^GSPTSE", "加拿大TSX"),
            ("IBOVESPA", "^BVSP", "巴西IBOVESPA"),
            ("MOEX", "^IMOEX", "俄罗斯MOEX"),
            ("STOXX50E", "^STOXX50E", "欧洲STOXX50"),
        ];

        let mut items = Vec::new();
        for (name, symbol, cn_name) in &pairs {
            let mut row = crate::types::Row::new();
            row.insert("name".into(), serde_json::json!(name));
            row.insert("symbol".into(), serde_json::json!(symbol));
            row.insert("cn_name".into(), serde_json::json!(cn_name));
            items.push(row);
        }
        Ok(items)
    }

    /// Get global index candles from Yahoo Finance.
    ///
    /// `symbol` can be a Yahoo symbol like "^GSPC", a friendly name like
    /// "SP500" / "标普500", or any valid Yahoo Finance ticker.
    pub async fn index_global_candles(
        &self,
        symbol: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let yahoo_sym = yahoo_index_symbol(symbol);
        self.yahoo_candles(&yahoo_sym, limit).await
    }
}
