use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct NlpParams {
    pub question: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nlp_params() {
        let json = r#"{"question": "What is the PE ratio of Apple?"}"#;
        let params: NlpParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.question, "What is the PE ratio of Apple?");
    }
}
