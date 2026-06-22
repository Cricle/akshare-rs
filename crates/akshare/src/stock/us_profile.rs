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
                .map(std::string::ToString::to_string),
            industry: profile
                .get("industry")
                .and_then(|v| v.as_str())
                .map(std::string::ToString::to_string),
            full_time_employees: profile
                .get("fullTimeEmployees")
                .and_then(serde_json::Value::as_i64),
            long_business_summary: profile
                .get("longBusinessSummary")
                .and_then(|v| v.as_str())
                .map(std::string::ToString::to_string),
            website: profile
                .get("website")
                .and_then(|v| v.as_str())
                .map(std::string::ToString::to_string),
            city: profile
                .get("city")
                .and_then(|v| v.as_str())
                .map(std::string::ToString::to_string),
            state: profile
                .get("state")
                .and_then(|v| v.as_str())
                .map(std::string::ToString::to_string),
            country: profile
                .get("country")
                .and_then(|v| v.as_str())
                .map(std::string::ToString::to_string),
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
        let raw = |v: &serde_json::Value| -> Option<f64> {
            v.get("raw").and_then(serde_json::Value::as_f64)
        };

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
        // Technology — Consumer Electronics
        "AAPL" => Some(("Technology", "Consumer Electronics")),
        // Technology — Software—Infrastructure
        "MSFT" | "ORCL" | "SNPS" | "PANW" | "CRWD" | "ZS" | "NET" | "SQ" | "TWLO" => {
            Some(("Technology", "Software—Infrastructure"))
        }
        // Technology — Software—Application
        "CRM" | "ADBE" | "NOW" | "INTU" | "CDNS" | "DDOG" | "SNOW" | "PLTR" | "SHOP" | "UBER"
        | "ABNB" | "GRAB" => Some(("Technology", "Software—Application")),
        // Technology — Semiconductors
        "NVDA" | "AMD" | "INTC" | "TSM" | "AVGO" | "QCOM" | "TXN" | "MU" | "MRVL" => {
            Some(("Technology", "Semiconductors"))
        }
        // Technology — Semiconductor Equipment & Materials
        "AMAT" | "LRCX" | "KLAC" => Some(("Technology", "Semiconductor Equipment & Materials")),
        // Technology — Communication Equipment
        "CSCO" => Some(("Technology", "Communication Equipment")),
        // Technology — Information Technology Services
        "IBM" => Some(("Technology", "Information Technology Services")),
        // Communication Services — Internet Content & Information
        "GOOG" | "GOOGL" | "META" | "SPOT" | "PINS" | "SNAP" | "BIDU" | "BILI" | "ZH" => {
            Some(("Communication Services", "Internet Content & Information"))
        }
        // Communication Services — Entertainment
        "NFLX" | "DIS" | "CMCSA" | "CHTR" | "IQ" | "TME" => {
            Some(("Communication Services", "Entertainment"))
        }
        // Communication Services — Telecom Services
        "T" | "VZ" | "TMUS" => Some(("Communication Services", "Telecom Services")),
        // Communication Services — Electronic Gaming & Multimedia
        "EA" | "TTWO" | "ATVI" | "NTES" => {
            Some(("Communication Services", "Electronic Gaming & Multimedia"))
        }
        // Communication Services — Consumer Electronics
        "ROKU" => Some(("Communication Services", "Consumer Electronics")),
        // Consumer Cyclical — Internet Retail
        "AMZN" | "BABA" | "JD" | "PDD" | "VIPS" | "MELI" | "SE" | "CPNG" => {
            Some(("Consumer Cyclical", "Internet Retail"))
        }
        // Consumer Cyclical — Auto Manufacturers
        "TSLA" | "GM" | "F" | "NIO" | "XPEV" | "LI" => {
            Some(("Consumer Cyclical", "Auto Manufacturers"))
        }
        // Consumer Cyclical — Home Improvement Retail
        "HD" | "LOW" => Some(("Consumer Cyclical", "Home Improvement Retail")),
        // Consumer Cyclical — Restaurants
        "MCD" | "SBUX" => Some(("Consumer Cyclical", "Restaurants")),
        // Consumer Cyclical — Apparel Retail
        "TJX" | "LULU" => Some(("Consumer Cyclical", "Apparel Retail")),
        // Consumer Cyclical — Footwear & Accessories
        "NKE" => Some(("Consumer Cyclical", "Footwear & Accessories")),
        // Consumer Cyclical — Travel Services
        "BKNG" => Some(("Consumer Cyclical", "Travel Services")),
        // Consumer Cyclical — Gambling
        "DKNG" => Some(("Consumer Cyclical", "Gambling")),
        // Consumer Defensive — Discount Stores
        "WMT" | "COST" | "TGT" | "MNSO" => Some(("Consumer Defensive", "Discount Stores")),
        // Consumer Defensive — Beverages—Brewers
        "STZ" => Some(("Consumer Defensive", "Beverages—Brewers")),
        // Consumer Defensive — Beverages—Non-Alcoholic
        "KO" | "PEP" => Some(("Consumer Defensive", "Beverages—Non-Alcoholic")),
        // Consumer Defensive — Tobacco
        "PM" | "MO" => Some(("Consumer Defensive", "Tobacco")),
        // Consumer Defensive — Household & Personal Products
        "CL" | "PG" | "EL" => Some(("Consumer Defensive", "Household & Personal Products")),
        // Healthcare — Drug Manufacturers—General
        "JNJ" | "PFE" | "ABBV" | "MRK" | "LLY" | "BMY" | "AMGN" | "GILD" => {
            Some(("Healthcare", "Drug Manufacturers—General"))
        }
        // Healthcare — Drug Manufacturers—Specialty & Generic
        "ZTS" => Some(("Healthcare", "Drug Manufacturers—Specialty & Generic")),
        // Healthcare — Healthcare Plans
        "UNH" => Some(("Healthcare", "Healthcare Plans")),
        // Healthcare — Diagnostics & Research
        "TMO" => Some(("Healthcare", "Diagnostics & Research")),
        // Healthcare — Medical Devices
        "ABT" | "MDT" | "SYK" | "BSX" => Some(("Healthcare", "Medical Devices")),
        // Healthcare — Medical Instruments & Supplies
        "ISRG" => Some(("Healthcare", "Medical Instruments & Supplies")),
        // Healthcare — Biotechnology
        "REGN" | "VRTX" | "MRNA" | "BIIB" => Some(("Healthcare", "Biotechnology")),
        // Healthcare — Medical Care Facilities
        "HCA" => Some(("Healthcare", "Medical Care Facilities")),
        // Financials — Insurance—Diversified
        "BRK-B" | "BRK.A" | "BRK_A" | "AIG" => Some(("Financials", "Insurance—Diversified")),
        // Financials — Banks—Diversified
        "JPM" | "BAC" | "WFC" | "C" => Some(("Financials", "Banks—Diversified")),
        // Financials — Credit Services
        "V" | "MA" | "AXP" => Some(("Financials", "Credit Services")),
        // Financials — Capital Markets
        "GS" | "MS" | "SCHW" | "FUTU" | "TIGR" => Some(("Financials", "Capital Markets")),
        // Financials — Asset Management
        "BLK" => Some(("Financials", "Asset Management")),
        // Financials — Insurance—Property & Casualty
        "CB" | "PGR" | "TRV" => Some(("Financials", "Insurance—Property & Casualty")),
        // Financials — Insurance—Life
        "MET" | "AFL" => Some(("Financials", "Insurance—Life")),
        // Financials — Financial Data & Stock Exchanges
        "CME" | "ICE" | "MCO" | "SPGI" => Some(("Financials", "Financial Data & Stock Exchanges")),
        // Energy — Oil & Gas Integrated
        "XOM" | "CVX" => Some(("Energy", "Oil & Gas Integrated")),
        // Energy — Oil & Gas E&P
        "COP" | "EOG" | "OXY" | "DVN" | "FANG" => Some(("Energy", "Oil & Gas E&P")),
        // Energy — Oil & Gas Equipment & Services
        "SLB" | "HAL" => Some(("Energy", "Oil & Gas Equipment & Services")),
        // Energy — Oil & Gas Refining & Marketing
        "MPC" | "PSX" | "VLO" => Some(("Energy", "Oil & Gas Refining & Marketing")),
        // Industrials — Farm & Heavy Construction Machinery
        "CAT" | "DE" => Some(("Industrials", "Farm & Heavy Construction Machinery")),
        // Industrials — Aerospace & Defense
        "BA" | "RTX" | "LMT" | "GD" | "NOC" | "TDG" => Some(("Industrials", "Aerospace & Defense")),
        // Industrials — Specialty Industrial Machinery
        "HON" | "GE" | "MMM" | "ETN" | "EMR" | "ITW" | "ROK" | "PH" => {
            Some(("Industrials", "Specialty Industrial Machinery"))
        }
        // Industrials — Integrated Freight & Logistics
        "UPS" | "FDX" => Some(("Industrials", "Integrated Freight & Logistics")),
        // Industrials — Waste Management
        "WM" => Some(("Industrials", "Waste Management")),
        // Real Estate — REIT—Industrial
        "PLD" => Some(("Real Estate", "REIT—Industrial")),
        // Real Estate — REIT—Specialty
        "AMT" | "CCI" => Some(("Real Estate", "REIT—Specialty")),
        // Real Estate — REIT—Data Center
        "EQIX" | "DLR" => Some(("Real Estate", "REIT—Data Center")),
        // Real Estate — REIT—Retail
        "SPG" | "O" => Some(("Real Estate", "REIT—Retail")),
        // Real Estate — REIT—Healthcare Facilities
        "WELL" => Some(("Real Estate", "REIT—Healthcare Facilities")),
        // Utilities — Utilities—Renewable
        "NEE" => Some(("Utilities", "Utilities—Renewable")),
        // Utilities — Utilities—Regulated Electric
        "DUK" | "SO" | "D" | "AEP" | "EXC" | "XEL" => {
            Some(("Utilities", "Utilities—Regulated Electric"))
        }
        // Utilities — Utilities—Diversified
        "SRE" => Some(("Utilities", "Utilities—Diversified")),
        // Basic Materials — Specialty Chemicals
        "LIN" | "APD" | "SHW" | "ECL" | "DOW" | "DD" | "PPG" => {
            Some(("Basic Materials", "Specialty Chemicals"))
        }
        // Basic Materials — Copper
        "FCX" => Some(("Basic Materials", "Copper")),
        // Basic Materials — Gold
        "NEM" => Some(("Basic Materials", "Gold")),
        // Basic Materials — Steel
        "NUE" => Some(("Basic Materials", "Steel")),

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
