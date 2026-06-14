//! US stock profile data — sector, industry, company info.
//!
//! Provides industry classification for US stocks using multiple sources:
//! 1. Yahoo Finance assetProfile API (primary)
//! 2. Static sector mapping for major stocks (fallback)

use crate::client::AkShareClient;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// US stock profile with industry/sector classification.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsStockProfile {
    pub symbol: String,
    #[serde(default)]
    pub sector: Option<String>,
    #[serde(default)]
    pub industry: Option<String>,
    #[serde(default)]
    pub full_time_employees: Option<i64>,
    #[serde(default)]
    pub long_business_summary: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
}

/// Key financial statistics for a US stock from Yahoo Finance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsKeyStats {
    pub symbol: String,
    /// Shares outstanding (total).
    #[serde(default)]
    pub shares_outstanding: Option<f64>,
    /// Float shares.
    #[serde(default)]
    pub float_shares: Option<f64>,
    /// Market cap.
    #[serde(default)]
    pub market_cap: Option<f64>,
    /// Trailing PE ratio.
    #[serde(default)]
    pub trailing_pe: Option<f64>,
    /// Forward PE ratio.
    #[serde(default)]
    pub forward_pe: Option<f64>,
    /// Price to book.
    #[serde(default)]
    pub price_to_book: Option<f64>,
    /// Enterprise value.
    #[serde(default)]
    pub enterprise_value: Option<f64>,
    /// Trailing EPS.
    #[serde(default)]
    pub trailing_eps: Option<f64>,
    /// Forward EPS.
    #[serde(default)]
    pub forward_eps: Option<f64>,
    /// Book value per share.
    #[serde(default)]
    pub book_value: Option<f64>,
    /// Revenue (TTM).
    #[serde(default)]
    pub revenue: Option<f64>,
    /// Net income (TTM).
    #[serde(default)]
    pub net_income: Option<f64>,
    /// Gross margin.
    #[serde(default)]
    pub gross_margin: Option<f64>,
    /// Operating margin.
    #[serde(default)]
    pub operating_margin: Option<f64>,
    /// Profit margin.
    #[serde(default)]
    pub profit_margin: Option<f64>,
    /// Return on equity.
    #[serde(default)]
    pub roe: Option<f64>,
    /// Return on assets.
    #[serde(default)]
    pub roa: Option<f64>,
    /// Debt to equity.
    #[serde(default)]
    pub debt_to_equity: Option<f64>,
    /// Current ratio.
    #[serde(default)]
    pub current_ratio: Option<f64>,
    /// Beta.
    #[serde(default)]
    pub beta: Option<f64>,
    /// 52-week high.
    #[serde(default)]
    pub week52_high: Option<f64>,
    /// 52-week low.
    #[serde(default)]
    pub week52_low: Option<f64>,
    /// Dividend yield.
    #[serde(default)]
    pub dividend_yield: Option<f64>,
    /// Payout ratio.
    #[serde(default)]
    pub payout_ratio: Option<f64>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get US stock profile from Yahoo Finance.
    ///
    /// Returns sector, industry, and company info for a US stock.
    /// Uses the Yahoo Finance v10 quoteSummary API with assetProfile module.
    pub async fn us_stock_profile(&self, symbol: &str) -> Result<UsStockProfile> {
        let url = format!(
            "https://query1.finance.yahoo.com/v10/finance/quoteSummary/{}",
            symbol.to_uppercase()
        );

        let response = self
            .get(&url)
            .query(&[("modules", "assetProfile")])
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;

        let profile = payload
            .get("quoteSummary")
            .and_then(|qs| qs.get("result"))
            .and_then(|r| r.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("assetProfile"))
            .ok_or_else(|| Error::upstream("Yahoo Finance assetProfile missing"))?;

        Ok(UsStockProfile {
            symbol: symbol.to_uppercase(),
            sector: profile
                .get("sector")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            industry: profile
                .get("industry")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            full_time_employees: profile.get("fullTimeEmployees").and_then(|v| v.as_i64()),
            long_business_summary: profile
                .get("longBusinessSummary")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            website: profile
                .get("website")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            city: profile
                .get("city")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            state: profile
                .get("state")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            country: profile
                .get("country")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        })
    }

    /// Get US stock industry using Yahoo Finance with static fallback.
    ///
    /// Returns (sector, industry) tuple. Tries Yahoo Finance first,
    /// falls back to static mapping for well-known stocks.
    pub async fn us_stock_industry(&self, symbol: &str) -> (Option<String>, Option<String>) {
        // Try Yahoo Finance first
        if let Ok(profile) = self.us_stock_profile(symbol).await
            && (profile.sector.is_some() || profile.industry.is_some())
        {
            return (profile.sector, profile.industry);
        }

        // Fallback to static mapping
        let sym = symbol.to_uppercase();
        if let Some((sector, industry)) = static_us_sector(&sym) {
            return (Some(sector.to_string()), Some(industry.to_string()));
        }

        (None, None)
    }

    /// Get key financial statistics for a US stock from Yahoo Finance.
    ///
    /// Fetches `defaultKeyStatistics`, `summaryDetail`, and `financialData` modules.
    /// Returns shares outstanding, PE, market cap, margins, and other key metrics.
    pub async fn us_stock_key_stats(&self, symbol: &str) -> Result<UsKeyStats> {
        let url = format!(
            "https://query1.finance.yahoo.com/v10/finance/quoteSummary/{}",
            symbol.to_uppercase()
        );

        let response = self
            .get(&url)
            .query(&[(
                "modules",
                "defaultKeyStatistics,summaryDetail,financialData",
            )])
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
            .map_err(Error::from)?
            .error_for_status()
            .map_err(Error::from)?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;

        let result = payload
            .get("quoteSummary")
            .and_then(|qs| qs.get("result"))
            .and_then(|r| r.as_array())
            .and_then(|arr| arr.first())
            .ok_or_else(|| Error::upstream("Yahoo Finance quoteSummary missing"))?;

        let stats = result
            .get("defaultKeyStatistics")
            .unwrap_or(&serde_json::Value::Null);
        let detail = result
            .get("summaryDetail")
            .unwrap_or(&serde_json::Value::Null);
        let financial = result
            .get("financialData")
            .unwrap_or(&serde_json::Value::Null);

        // Helper to extract raw value from Yahoo Finance nested format
        // Yahoo returns {"raw": 123.45, "fmt": "123.45"} for numeric fields
        let raw = |v: &serde_json::Value| -> Option<f64> { v.get("raw").and_then(|r| r.as_f64()) };

        Ok(UsKeyStats {
            symbol: symbol.to_uppercase(),
            shares_outstanding: raw(stats
                .get("sharesOutstanding")
                .unwrap_or(&serde_json::Value::Null)),
            float_shares: raw(stats.get("floatShares").unwrap_or(&serde_json::Value::Null)),
            market_cap: raw(detail.get("marketCap").unwrap_or(&serde_json::Value::Null)),
            trailing_pe: raw(detail.get("trailingPE").unwrap_or(&serde_json::Value::Null)),
            forward_pe: raw(detail.get("forwardPE").unwrap_or(&serde_json::Value::Null)),
            price_to_book: raw(stats.get("priceToBook").unwrap_or(&serde_json::Value::Null)),
            enterprise_value: raw(stats
                .get("enterpriseValue")
                .unwrap_or(&serde_json::Value::Null)),
            trailing_eps: raw(stats.get("trailingEps").unwrap_or(&serde_json::Value::Null)),
            forward_eps: raw(stats.get("forwardEps").unwrap_or(&serde_json::Value::Null)),
            book_value: raw(stats.get("bookValue").unwrap_or(&serde_json::Value::Null)),
            revenue: raw(financial
                .get("totalRevenue")
                .unwrap_or(&serde_json::Value::Null)),
            net_income: raw(financial
                .get("netIncomeToCommon")
                .unwrap_or(&serde_json::Value::Null)),
            gross_margin: raw(financial
                .get("grossMargins")
                .unwrap_or(&serde_json::Value::Null)),
            operating_margin: raw(financial
                .get("operatingMargins")
                .unwrap_or(&serde_json::Value::Null)),
            profit_margin: raw(financial
                .get("profitMargins")
                .unwrap_or(&serde_json::Value::Null)),
            roe: raw(financial
                .get("returnOnEquity")
                .unwrap_or(&serde_json::Value::Null)),
            roa: raw(financial
                .get("returnOnAssets")
                .unwrap_or(&serde_json::Value::Null)),
            debt_to_equity: raw(financial
                .get("debtToEquity")
                .unwrap_or(&serde_json::Value::Null)),
            current_ratio: raw(financial
                .get("currentRatio")
                .unwrap_or(&serde_json::Value::Null)),
            beta: raw(stats.get("beta").unwrap_or(&serde_json::Value::Null)),
            week52_high: raw(detail
                .get("fiftyTwoWeekHigh")
                .unwrap_or(&serde_json::Value::Null)),
            week52_low: raw(detail
                .get("fiftyTwoWeekLow")
                .unwrap_or(&serde_json::Value::Null)),
            dividend_yield: raw(detail
                .get("dividendYield")
                .unwrap_or(&serde_json::Value::Null)),
            payout_ratio: raw(detail
                .get("payoutRatio")
                .unwrap_or(&serde_json::Value::Null)),
        })
    }
}

/// Static sector/industry mapping for major US stocks.
/// Used as fallback when Yahoo Finance API fails.
fn static_us_sector(symbol: &str) -> Option<(&str, &str)> {
    let upper = symbol.to_uppercase();
    match upper.as_str() {
        // Technology
        "AAPL" => Some(("Technology", "Consumer Electronics")),
        "MSFT" => Some(("Technology", "Software—Infrastructure")),
        "GOOG" | "GOOGL" => Some(("Communication Services", "Internet Content & Information")),
        "META" => Some(("Communication Services", "Internet Content & Information")),
        "NVDA" => Some(("Technology", "Semiconductors")),
        "AMD" => Some(("Technology", "Semiconductors")),
        "INTC" => Some(("Technology", "Semiconductors")),
        "TSM" => Some(("Technology", "Semiconductors")),
        "AVGO" => Some(("Technology", "Semiconductors")),
        "QCOM" => Some(("Technology", "Semiconductors")),
        "ORCL" => Some(("Technology", "Software—Infrastructure")),
        "CRM" => Some(("Technology", "Software—Application")),
        "ADBE" => Some(("Technology", "Software—Application")),
        "NOW" => Some(("Technology", "Software—Application")),
        "INTU" => Some(("Technology", "Software—Application")),
        "CSCO" => Some(("Technology", "Communication Equipment")),
        "IBM" => Some(("Technology", "Information Technology Services")),
        "TXN" => Some(("Technology", "Semiconductors")),
        "MU" => Some(("Technology", "Semiconductors")),
        "AMAT" => Some(("Technology", "Semiconductor Equipment & Materials")),
        "LRCX" => Some(("Technology", "Semiconductor Equipment & Materials")),
        "KLAC" => Some(("Technology", "Semiconductor Equipment & Materials")),
        "MRVL" => Some(("Technology", "Semiconductors")),
        "SNPS" => Some(("Technology", "Software—Infrastructure")),
        "CDNS" => Some(("Technology", "Software—Application")),
        "PANW" => Some(("Technology", "Software—Infrastructure")),
        "CRWD" => Some(("Technology", "Software—Infrastructure")),
        "ZS" => Some(("Technology", "Software—Infrastructure")),
        "NET" => Some(("Technology", "Software—Infrastructure")),
        "DDOG" => Some(("Technology", "Software—Application")),
        "SNOW" => Some(("Technology", "Software—Application")),
        "PLTR" => Some(("Technology", "Software—Application")),
        "SQ" => Some(("Technology", "Software—Infrastructure")),
        "SHOP" => Some(("Technology", "Software—Application")),
        "UBER" => Some(("Technology", "Software—Application")),
        "ABNB" => Some(("Technology", "Software—Application")),
        "NFLX" => Some(("Communication Services", "Entertainment")),
        "DIS" => Some(("Communication Services", "Entertainment")),
        "SPOT" => Some(("Communication Services", "Internet Content & Information")),

        // Consumer
        "AMZN" => Some(("Consumer Cyclical", "Internet Retail")),
        "TSLA" => Some(("Consumer Cyclical", "Auto Manufacturers")),
        "WMT" => Some(("Consumer Defensive", "Discount Stores")),
        "COST" => Some(("Consumer Defensive", "Discount Stores")),
        "HD" => Some(("Consumer Cyclical", "Home Improvement Retail")),
        "NKE" => Some(("Consumer Cyclical", "Footwear & Accessories")),
        "MCD" => Some(("Consumer Cyclical", "Restaurants")),
        "SBUX" => Some(("Consumer Cyclical", "Restaurants")),
        "TGT" => Some(("Consumer Defensive", "Discount Stores")),
        "LOW" => Some(("Consumer Cyclical", "Home Improvement Retail")),
        "TJX" => Some(("Consumer Cyclical", "Apparel Retail")),
        "BKNG" => Some(("Consumer Cyclical", "Travel Services")),
        "GM" => Some(("Consumer Cyclical", "Auto Manufacturers")),
        "F" => Some(("Consumer Cyclical", "Auto Manufacturers")),
        "STZ" => Some(("Consumer Defensive", "Beverages—Brewers")),
        "KO" => Some(("Consumer Defensive", "Beverages—Non-Alcoholic")),
        "PEP" => Some(("Consumer Defensive", "Beverages—Non-Alcoholic")),
        "PM" => Some(("Consumer Defensive", "Tobacco")),
        "MO" => Some(("Consumer Defensive", "Tobacco")),
        "CL" => Some(("Consumer Defensive", "Household & Personal Products")),
        "PG" => Some(("Consumer Defensive", "Household & Personal Products")),
        "EL" => Some(("Consumer Defensive", "Household & Personal Products")),

        // Healthcare
        "JNJ" => Some(("Healthcare", "Drug Manufacturers—General")),
        "UNH" => Some(("Healthcare", "Healthcare Plans")),
        "PFE" => Some(("Healthcare", "Drug Manufacturers—General")),
        "ABBV" => Some(("Healthcare", "Drug Manufacturers—General")),
        "MRK" => Some(("Healthcare", "Drug Manufacturers—General")),
        "TMO" => Some(("Healthcare", "Diagnostics & Research")),
        "ABT" => Some(("Healthcare", "Medical Devices")),
        "LLY" => Some(("Healthcare", "Drug Manufacturers—General")),
        "BMY" => Some(("Healthcare", "Drug Manufacturers—General")),
        "AMGN" => Some(("Healthcare", "Drug Manufacturers—General")),
        "GILD" => Some(("Healthcare", "Drug Manufacturers—General")),
        "ISRG" => Some(("Healthcare", "Medical Instruments & Supplies")),
        "MDT" => Some(("Healthcare", "Medical Devices")),
        "SYK" => Some(("Healthcare", "Medical Devices")),
        "BSX" => Some(("Healthcare", "Medical Devices")),
        "ZTS" => Some(("Healthcare", "Drug Manufacturers—Specialty & Generic")),
        "REGN" => Some(("Healthcare", "Biotechnology")),
        "VRTX" => Some(("Healthcare", "Biotechnology")),
        "MRNA" => Some(("Healthcare", "Biotechnology")),
        "BIIB" => Some(("Healthcare", "Biotechnology")),
        "HCA" => Some(("Healthcare", "Medical Care Facilities")),

        // Financials
        "BRK-B" | "BRK.A" | "BRK_A" => Some(("Financials", "Insurance—Diversified")),
        "JPM" => Some(("Financials", "Banks—Diversified")),
        "V" => Some(("Financials", "Credit Services")),
        "MA" => Some(("Financials", "Credit Services")),
        "BAC" => Some(("Financials", "Banks—Diversified")),
        "WFC" => Some(("Financials", "Banks—Diversified")),
        "GS" => Some(("Financials", "Capital Markets")),
        "MS" => Some(("Financials", "Capital Markets")),
        "C" => Some(("Financials", "Banks—Diversified")),
        "AXP" => Some(("Financials", "Credit Services")),
        "BLK" => Some(("Financials", "Asset Management")),
        "SCHW" => Some(("Financials", "Capital Markets")),
        "CB" => Some(("Financials", "Insurance—Property & Casualty")),
        "AIG" => Some(("Financials", "Insurance—Diversified")),
        "MET" => Some(("Financials", "Insurance—Life")),
        "AFL" => Some(("Financials", "Insurance—Life")),
        "PGR" => Some(("Financials", "Insurance—Property & Casualty")),
        "TRV" => Some(("Financials", "Insurance—Property & Casualty")),
        "CME" => Some(("Financials", "Financial Data & Stock Exchanges")),
        "ICE" => Some(("Financials", "Financial Data & Stock Exchanges")),
        "MCO" => Some(("Financials", "Financial Data & Stock Exchanges")),
        "SPGI" => Some(("Financials", "Financial Data & Stock Exchanges")),

        // Energy
        "XOM" => Some(("Energy", "Oil & Gas Integrated")),
        "CVX" => Some(("Energy", "Oil & Gas Integrated")),
        "COP" => Some(("Energy", "Oil & Gas E&P")),
        "EOG" => Some(("Energy", "Oil & Gas E&P")),
        "SLB" => Some(("Energy", "Oil & Gas Equipment & Services")),
        "MPC" => Some(("Energy", "Oil & Gas Refining & Marketing")),
        "PSX" => Some(("Energy", "Oil & Gas Refining & Marketing")),
        "VLO" => Some(("Energy", "Oil & Gas Refining & Marketing")),
        "OXY" => Some(("Energy", "Oil & Gas E&P")),
        "HAL" => Some(("Energy", "Oil & Gas Equipment & Services")),
        "DVN" => Some(("Energy", "Oil & Gas E&P")),
        "FANG" => Some(("Energy", "Oil & Gas E&P")),

        // Industrials
        "CAT" => Some(("Industrials", "Farm & Heavy Construction Machinery")),
        "BA" => Some(("Industrials", "Aerospace & Defense")),
        "HON" => Some(("Industrials", "Specialty Industrial Machinery")),
        "UPS" => Some(("Industrials", "Integrated Freight & Logistics")),
        "RTX" => Some(("Industrials", "Aerospace & Defense")),
        "DE" => Some(("Industrials", "Farm & Heavy Construction Machinery")),
        "LMT" => Some(("Industrials", "Aerospace & Defense")),
        "GE" => Some(("Industrials", "Specialty Industrial Machinery")),
        "MMM" => Some(("Industrials", "Specialty Industrial Machinery")),
        "GD" => Some(("Industrials", "Aerospace & Defense")),
        "NOC" => Some(("Industrials", "Aerospace & Defense")),
        "FDX" => Some(("Industrials", "Integrated Freight & Logistics")),
        "WM" => Some(("Industrials", "Waste Management")),
        "ETN" => Some(("Industrials", "Specialty Industrial Machinery")),
        "EMR" => Some(("Industrials", "Specialty Industrial Machinery")),
        "ITW" => Some(("Industrials", "Specialty Industrial Machinery")),
        "ROK" => Some(("Industrials", "Specialty Industrial Machinery")),
        "PH" => Some(("Industrials", "Specialty Industrial Machinery")),
        "TDG" => Some(("Industrials", "Aerospace & Defense")),

        // Communication Services
        "T" => Some(("Communication Services", "Telecom Services")),
        "VZ" => Some(("Communication Services", "Telecom Services")),
        "CMCSA" => Some(("Communication Services", "Entertainment")),
        "TMUS" => Some(("Communication Services", "Telecom Services")),
        "CHTR" => Some(("Communication Services", "Entertainment")),
        "EA" => Some(("Communication Services", "Electronic Gaming & Multimedia")),
        "TTWO" => Some(("Communication Services", "Electronic Gaming & Multimedia")),
        "ATVI" => Some(("Communication Services", "Electronic Gaming & Multimedia")),
        "ROKU" => Some(("Communication Services", "Consumer Electronics")),
        "PINS" => Some(("Communication Services", "Internet Content & Information")),
        "SNAP" => Some(("Communication Services", "Internet Content & Information")),
        "TWLO" => Some(("Technology", "Software—Infrastructure")),

        // Real Estate
        "PLD" => Some(("Real Estate", "REIT—Industrial")),
        "AMT" => Some(("Real Estate", "REIT—Specialty")),
        "CCI" => Some(("Real Estate", "REIT—Specialty")),
        "EQIX" => Some(("Real Estate", "REIT—Data Center")),
        "SPG" => Some(("Real Estate", "REIT—Retail")),
        "O" => Some(("Real Estate", "REIT—Retail")),
        "WELL" => Some(("Real Estate", "REIT—Healthcare Facilities")),
        "DLR" => Some(("Real Estate", "REIT—Data Center")),

        // Utilities
        "NEE" => Some(("Utilities", "Utilities—Renewable")),
        "DUK" => Some(("Utilities", "Utilities—Regulated Electric")),
        "SO" => Some(("Utilities", "Utilities—Regulated Electric")),
        "D" => Some(("Utilities", "Utilities—Regulated Electric")),
        "AEP" => Some(("Utilities", "Utilities—Regulated Electric")),
        "SRE" => Some(("Utilities", "Utilities—Diversified")),
        "EXC" => Some(("Utilities", "Utilities—Regulated Electric")),
        "XEL" => Some(("Utilities", "Utilities—Regulated Electric")),

        // Materials
        "LIN" => Some(("Basic Materials", "Specialty Chemicals")),
        "APD" => Some(("Basic Materials", "Specialty Chemicals")),
        "SHW" => Some(("Basic Materials", "Specialty Chemicals")),
        "ECL" => Some(("Basic Materials", "Specialty Chemicals")),
        "FCX" => Some(("Basic Materials", "Copper")),
        "NEM" => Some(("Basic Materials", "Gold")),
        "NUE" => Some(("Basic Materials", "Steel")),
        "DOW" => Some(("Basic Materials", "Specialty Chemicals")),
        "DD" => Some(("Basic Materials", "Specialty Chemicals")),
        "PPG" => Some(("Basic Materials", "Specialty Chemicals")),

        // Chinese ADRs (common US-listed)
        "BABA" => Some(("Consumer Cyclical", "Internet Retail")),
        "JD" => Some(("Consumer Cyclical", "Internet Retail")),
        "PDD" => Some(("Consumer Cyclical", "Internet Retail")),
        "BIDU" => Some(("Communication Services", "Internet Content & Information")),
        "NIO" => Some(("Consumer Cyclical", "Auto Manufacturers")),
        "XPEV" => Some(("Consumer Cyclical", "Auto Manufacturers")),
        "LI" => Some(("Consumer Cyclical", "Auto Manufacturers")),
        "NTES" => Some(("Communication Services", "Electronic Gaming & Multimedia")),
        "BILI" => Some(("Communication Services", "Internet Content & Information")),
        "IQ" => Some(("Communication Services", "Entertainment")),
        "TME" => Some(("Communication Services", "Entertainment")),
        "VIPS" => Some(("Consumer Cyclical", "Internet Retail")),
        "MNSO" => Some(("Consumer Cyclical", "Discount Stores")),
        "FUTU" => Some(("Financials", "Capital Markets")),
        "TIGR" => Some(("Financials", "Capital Markets")),
        "ZH" => Some(("Communication Services", "Internet Content & Information")),
        "LULU" => Some(("Consumer Cyclical", "Apparel Retail")),
        "MELI" => Some(("Consumer Cyclical", "Internet Retail")),
        "SE" => Some(("Consumer Cyclical", "Internet Retail")),
        "GRAB" => Some(("Technology", "Software—Application")),
        "CPNG" => Some(("Consumer Cyclical", "Internet Retail")),
        "DKNG" => Some(("Consumer Cyclical", "Gambling")),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_sector_known() {
        assert_eq!(
            static_us_sector("AAPL"),
            Some(("Technology", "Consumer Electronics"))
        );
        assert_eq!(
            static_us_sector("MSFT"),
            Some(("Technology", "Software—Infrastructure"))
        );
        assert_eq!(
            static_us_sector("NVDA"),
            Some(("Technology", "Semiconductors"))
        );
        assert_eq!(
            static_us_sector("JPM"),
            Some(("Financials", "Banks—Diversified"))
        );
        assert_eq!(
            static_us_sector("XOM"),
            Some(("Energy", "Oil & Gas Integrated"))
        );
        assert_eq!(
            static_us_sector("BABA"),
            Some(("Consumer Cyclical", "Internet Retail"))
        );
    }

    #[test]
    fn test_static_sector_unknown() {
        assert_eq!(static_us_sector("UNKNOWN_SYMBOL"), None);
    }

    #[test]
    fn test_static_sector_case_insensitive() {
        assert_eq!(
            static_us_sector("aapl"),
            Some(("Technology", "Consumer Electronics"))
        );
        assert_eq!(
            static_us_sector("Nvda"),
            Some(("Technology", "Semiconductors"))
        );
    }
}
