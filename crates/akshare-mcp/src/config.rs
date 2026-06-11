use serde::Deserialize;
use std::path::Path;

/// Top-level configuration for the MCP server.
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub http: HttpConfig,
}

/// HTTP transport configuration.
#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    /// Bind address (e.g. "127.0.0.1:8080").
    #[serde(default = "default_bind")]
    pub bind: String,
    /// MCP authentication key. Empty string disables auth (dev mode).
    #[serde(default)]
    pub mcp_key: String,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            bind: default_bind(),
            mcp_key: String::new(),
        }
    }
}

fn default_bind() -> String {
    "127.0.0.1:8080".to_string()
}

impl Config {
    /// Load configuration from a TOML file.
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_default_http_config() {
        let cfg = HttpConfig::default();
        assert_eq!(cfg.bind, "127.0.0.1:8080");
        assert_eq!(cfg.mcp_key, "");
    }

    #[test]
    fn test_parse_empty_config() {
        let config: Config = toml::from_str("").unwrap();
        assert_eq!(config.http.bind, "127.0.0.1:8080");
        assert_eq!(config.http.mcp_key, "");
    }

    #[test]
    fn test_parse_full_config() {
        let toml = r#"
[http]
bind = "0.0.0.0:9090"
mcp_key = "secret123"
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.http.bind, "0.0.0.0:9090");
        assert_eq!(config.http.mcp_key, "secret123");
    }

    #[test]
    fn test_parse_partial_config() {
        let toml = r#"
[http]
mcp_key = "mykey"
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.http.bind, "127.0.0.1:8080");
        assert_eq!(config.http.mcp_key, "mykey");
    }

    #[test]
    fn test_load_from_file() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            r#"
[http]
bind = "127.0.0.1:3000"
mcp_key = "test-key"
"#
        )
        .unwrap();

        let config = Config::load(tmp.path()).unwrap();
        assert_eq!(config.http.bind, "127.0.0.1:3000");
        assert_eq!(config.http.mcp_key, "test-key");
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = Config::load(Path::new("/nonexistent/config.toml"));
        assert!(result.is_err());
    }
}
