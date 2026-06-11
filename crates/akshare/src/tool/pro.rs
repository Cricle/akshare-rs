//! Tushare Pro API token management.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

impl AkShareClient {
    /// Set the Tushare Pro API token.
    ///
    /// This token is used for authenticated Tushare Pro API calls.
    pub fn set_token(&mut self, token: &str) {
        self.tushare_token = Some(token.to_string());
    }

    /// Get a Tushare Pro API token by authenticating with username and password.
    ///
    /// `user`: Tushare account email
    /// `password`: Tushare account password
    ///
    /// Returns the API token string.
    pub async fn pro_api(&self, user: &str, password: &str) -> Result<String> {
        let url = "https://api.tushare.pro/auth/token";
        let payload = serde_json::json!({
            "user": user,
            "password": password,
        });

        let body = self
            .post(url)
            .json(&payload)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .text()
            .await?;

        let resp: serde_json::Value = serde_json::from_str(&body)?;
        let token = resp["token"]
            .as_str()
            .or_else(|| resp["data"]["token"].as_str())
            .ok_or_else(|| Error::decode("tushare: no token in response"))?;

        Ok(token.to_string())
    }
}
