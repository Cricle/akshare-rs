use serde::Deserialize;
use std::path::Path;

/// Top-level configuration for the MCP server.
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub http: HttpConfig,
    #[serde(default)]
    pub tools: ToolsConfig,
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

fn default_true() -> bool {
    true
}

/// Tool category enable/disable configuration.
///
/// Only enabled categories will have their tools registered with the MCP server.
/// By default only `stock` is enabled.
#[derive(Debug, Clone, Deserialize)]
pub struct ToolsConfig {
    #[serde(default = "default_true")]
    pub stock: bool,
    #[serde(default)]
    pub bond: bool,
    #[serde(default)]
    pub index: bool,
    #[serde(default)]
    pub futures: bool,
    #[serde(default)]
    pub economy: bool,
    #[serde(default)]
    pub crypto: bool,
    #[serde(default)]
    pub forex: bool,
    #[serde(default)]
    pub option: bool,
    #[serde(default)]
    pub news: bool,
    #[serde(default)]
    pub macro_data: bool,
    #[serde(default)]
    pub fund: bool,
}

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            stock: true,
            bond: false,
            index: false,
            futures: false,
            economy: false,
            crypto: false,
            forex: false,
            option: false,
            news: false,
            macro_data: false,
            fund: false,
        }
    }
}

impl ToolsConfig {
    /// Create a config with all categories enabled.
    #[must_use]
    pub fn all() -> Self {
        Self {
            stock: true,
            bond: true,
            index: true,
            futures: true,
            economy: true,
            crypto: true,
            forex: true,
            option: true,
            news: true,
            macro_data: true,
            fund: true,
        }
    }

    /// Enable a category by name.
    pub fn enable(&mut self, name: &str) -> bool {
        match name {
            "stock" => { self.stock = true; true }
            "bond" => { self.bond = true; true }
            "index" => { self.index = true; true }
            "futures" => { self.futures = true; true }
            "economy" => { self.economy = true; true }
            "crypto" => { self.crypto = true; true }
            "forex" => { self.forex = true; true }
            "option" => { self.option = true; true }
            "news" => { self.news = true; true }
            "macro_data" => { self.macro_data = true; true }
            "fund" => { self.fund = true; true }
            _ => false,
        }
    }

    /// Disable a category by name.
    pub fn disable(&mut self, name: &str) -> bool {
        match name {
            "stock" => { self.stock = false; true }
            "bond" => { self.bond = false; true }
            "index" => { self.index = false; true }
            "futures" => { self.futures = false; true }
            "economy" => { self.economy = false; true }
            "crypto" => { self.crypto = false; true }
            "forex" => { self.forex = false; true }
            "option" => { self.option = false; true }
            "news" => { self.news = false; true }
            "macro_data" => { self.macro_data = false; true }
            "fund" => { self.fund = false; true }
            _ => false,
        }
    }

    /// Check if a category is enabled by name.
    pub fn is_enabled(&self, name: &str) -> bool {
        match name {
            "stock" => self.stock,
            "bond" => self.bond,
            "index" => self.index,
            "futures" => self.futures,
            "economy" => self.economy,
            "crypto" => self.crypto,
            "forex" => self.forex,
            "option" => self.option,
            "news" => self.news,
            "macro_data" => self.macro_data,
            "fund" => self.fund,
            _ => false,
        }
    }
}

impl Config {
    /// Load configuration from a TOML file.
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
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

    #[test]
    fn test_default_tools_config() {
        let cfg = ToolsConfig::default();
        assert!(cfg.stock);
        assert!(!cfg.bond);
        assert!(!cfg.index);
        assert!(!cfg.futures);
        assert!(!cfg.economy);
        assert!(!cfg.crypto);
        assert!(!cfg.forex);
        assert!(!cfg.option);
        assert!(!cfg.news);
        assert!(!cfg.macro_data);
        assert!(!cfg.fund);
    }

    #[test]
    fn test_tools_config_all() {
        let cfg = ToolsConfig::all();
        assert!(cfg.stock);
        assert!(cfg.bond);
        assert!(cfg.index);
        assert!(cfg.futures);
        assert!(cfg.economy);
        assert!(cfg.crypto);
        assert!(cfg.forex);
        assert!(cfg.option);
        assert!(cfg.news);
        assert!(cfg.macro_data);
        assert!(cfg.fund);
    }

    #[test]
    fn test_tools_config_enable_disable() {
        let mut cfg = ToolsConfig::default();
        assert!(!cfg.bond);
        assert!(cfg.enable("bond"));
        assert!(cfg.bond);
        assert!(cfg.disable("stock"));
        assert!(!cfg.stock);
        assert!(!cfg.enable("nonexistent"));
    }

    #[test]
    fn test_tools_config_is_enabled() {
        let cfg = ToolsConfig::default();
        assert!(cfg.is_enabled("stock"));
        assert!(!cfg.is_enabled("bond"));
        assert!(!cfg.is_enabled("nonexistent"));
    }

    #[test]
    fn test_parse_tools_config() {
        let toml = r#"
[tools]
stock = true
bond = true
index = true
futures = false
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert!(config.tools.stock);
        assert!(config.tools.bond);
        assert!(config.tools.index);
        assert!(!config.tools.futures);
    }

    #[test]
    fn test_parse_empty_tools_config() {
        let config: Config = toml::from_str("").unwrap();
        assert!(config.tools.stock);
        assert!(!config.tools.bond);
    }

    #[test]
    fn test_tools_config_clone() {
        let cfg = ToolsConfig::all();
        let cloned = cfg.clone();
        assert!(cloned.stock);
        assert!(cloned.bond);
    }
}
