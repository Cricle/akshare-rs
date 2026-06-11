//! Central bank interest rate decisions from Jin10 datacenter.

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::MacroDataPoint;

use super::shared::fetch_jin10_interest_rate;

impl AkShareClient {
    /// US Federal Reserve interest rate decision (美联储利率决议).
    pub async fn bank_usa_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "24", "Fed Interest Rate").await
    }

    /// ECB interest rate decision (欧洲央行决议).
    pub async fn bank_euro_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "21", "ECB Interest Rate").await
    }

    /// Reserve Bank of New Zealand interest rate decision (新西兰联储决议).
    pub async fn bank_newzealand_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "23", "RBNZ Interest Rate").await
    }

    /// People's Bank of China interest rate decision (中国央行决议).
    pub async fn bank_china_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "91", "PBOC Interest Rate").await
    }

    /// Swiss National Bank interest rate decision (瑞士央行决议).
    pub async fn bank_switzerland_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "25", "SNB Interest Rate").await
    }

    /// Bank of England interest rate decision (英国央行决议).
    pub async fn bank_england_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "26", "BOE Interest Rate").await
    }

    /// Reserve Bank of Australia interest rate decision (澳洲联储决议).
    pub async fn bank_australia_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "27", "RBA Interest Rate").await
    }

    /// Bank of Japan interest rate decision (日本央行决议).
    pub async fn bank_japan_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "22", "BOJ Interest Rate").await
    }

    /// Central Bank of Russia interest rate decision (俄罗斯央行决议).
    pub async fn bank_russia_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "64", "CBR Interest Rate").await
    }

    /// Reserve Bank of India interest rate decision (印度央行决议).
    pub async fn bank_india_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "68", "RBI Interest Rate").await
    }

    /// Central Bank of Brazil interest rate decision (巴西央行决议).
    pub async fn bank_brazil_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        fetch_jin10_interest_rate(self, "55", "BCB Interest Rate").await
    }
}

// Python-compatible aliases
impl AkShareClient {
    pub async fn macro_bank_australia_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_australia_interest_rate().await
    }

    pub async fn macro_bank_brazil_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_brazil_interest_rate().await
    }

    pub async fn macro_bank_china_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_china_interest_rate().await
    }

    pub async fn macro_bank_euro_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_euro_interest_rate().await
    }

    pub async fn macro_bank_india_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_india_interest_rate().await
    }

    pub async fn macro_bank_japan_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_japan_interest_rate().await
    }

    pub async fn macro_bank_newzealand_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_newzealand_interest_rate().await
    }

    pub async fn macro_bank_russia_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_russia_interest_rate().await
    }

    pub async fn macro_bank_switzerland_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_switzerland_interest_rate().await
    }

    pub async fn macro_bank_usa_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_usa_interest_rate().await
    }

    pub async fn macro_bank_english_interest_rate(&self) -> Result<Vec<MacroDataPoint>> {
        self.bank_england_interest_rate().await
    }
}
