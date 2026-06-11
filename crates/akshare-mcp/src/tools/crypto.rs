use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CryptoDateParams {
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_date_params() {
        let json = r#"{"date": "20240101"}"#;
        let params: CryptoDateParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.date, "20240101");
    }
}
