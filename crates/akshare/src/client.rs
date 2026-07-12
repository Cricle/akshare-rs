//! HTTP client with mock-server support for testing.
//!
//! [`AkShareClient`] is the main entry point for all API calls.
//! Use [`AkShareClientBuilder`] for custom configuration (proxy, timeout, tokens).

use std::time::Duration;

/// AkShare Rust client — 100% pure Rust, no Python/JS/FFI.
#[derive(Clone)]
pub struct AkShareClient {
    pub(crate) http: reqwest::Client,
    pub(crate) tushare_token: Option<String>,
    /// When set, all HTTP requests are redirected to this base URL (for testing).
    pub mock_uri: Option<String>,
}

impl AkShareClient {
    /// Create a client with default settings (30s timeout, no proxy).
    #[must_use]
    pub fn new() -> Self {
        Self::builder().build()
    }

    /// Create a test client that redirects all HTTP requests to a mock server.
    #[must_use]
    pub fn with_mock(mock_uri: String) -> Self {
        let mut client = Self::new();
        client.mock_uri = Some(mock_uri);
        client
    }

    /// Redirect a URL to the mock server if mock_uri is set.
    /// Also supports PUSH2_BASE_URL env var to override push2.eastmoney.com
    /// (useful when the server IP is blocked by Eastmoney).
    pub(crate) fn redirect_url(&self, url: &str) -> String {
        if let Some(mock) = &self.mock_uri {
            if let Some(proto_end) = url.find("//")
                && let Some(path_start) = url[proto_end + 2..].find('/')
            {
                let path = &url[proto_end + 2 + path_start..];
                return format!("{}{}", mock.trim_end_matches('/'), path);
            }
            mock.clone()
        } else {
            Self::apply_push2_override(url)
        }
    }

    /// Replace push2.eastmoney.com host if PUSH2_BASE_URL env var is set.
    fn apply_push2_override(url: &str) -> String {
        static OVERRIDE: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
        let override_url = OVERRIDE.get_or_init(|| {
            std::env::var("PUSH2_BASE_URL")
                .ok()
                .map(|v| v.trim().trim_end_matches('/').to_string())
        });
        if let Some(base) = override_url
            && (url.contains("push2.eastmoney.com") || url.contains("push2his.eastmoney.com"))
        {
            return url.replace("https://push2.eastmoney.com", base);
        }
        url.to_string()
    }

    /// HTTP GET with automatic mock URL redirection.
    pub(crate) fn get(&self, url: &str) -> reqwest::RequestBuilder {
        let url = self.redirect_url(url);
        let url = Self::apply_push2_ip_override(&url);
        self.http.get(url)
    }

    /// HTTP POST with automatic mock URL redirection.
    pub(crate) fn post(&self, url: &str) -> reqwest::RequestBuilder {
        let url = self.redirect_url(url);
        let url = Self::apply_push2_ip_override(&url);
        self.http.post(url)
    }

    /// When PUSH2_IP env var is set, add a Host header + rewrite URL for push2.eastmoney.com
    /// to work around DNS-based IP blocking.
    fn apply_push2_ip_override(url: &str) -> String {
        static IP_OVERRIDE: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
        let ip = IP_OVERRIDE.get_or_init(|| {
            std::env::var("PUSH2_IP")
                .ok()
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
        });
        if let Some(ip_addr) = ip
            && (url.contains("push2.eastmoney.com") || url.contains("push2his.eastmoney.com"))
        {
            let result = url.replace("push2his.eastmoney.com", ip_addr);
            return result.replace("push2.eastmoney.com", ip_addr);
        }
        url.to_string()
    }

    /// Start building a client with custom configuration.
    #[must_use]
    pub fn builder() -> AkShareClientBuilder {
        AkShareClientBuilder {
            tushare_token: None,
            user_agent: "akshare-rust/0.1".to_string(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            proxy: None,
        }
    }
}

impl Default for AkShareClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for [`AkShareClient`] with custom configuration.
///
/// # Example
///
/// ```rust
/// use akshare::AkShareClient;
/// use std::time::Duration;
///
/// let client = AkShareClient::builder()
///     .timeout(Duration::from_secs(60))
///     .proxy("http://127.0.0.1:7890")
///     .build();
/// ```
pub struct AkShareClientBuilder {
    tushare_token: Option<String>,
    user_agent: String,
    timeout: Duration,
    connect_timeout: Duration,
    proxy: Option<String>,
}

impl AkShareClientBuilder {
    /// Set the Tushare API token for Tushare data sources.
    #[must_use]
    pub fn tushare_token(mut self, token: impl Into<String>) -> Self {
        self.tushare_token = Some(token.into());
        self
    }

    /// Set the User-Agent header for HTTP requests.
    #[must_use]
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }

    /// Set the overall request timeout (default: 30s).
    #[must_use]
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the TCP connect timeout (default: 10s).
    #[must_use]
    pub const fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set an HTTP proxy for all requests.
    #[must_use]
    pub fn proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }

    /// Build the [`AkShareClient`] with the configured settings.
    #[must_use]
    pub fn build(self) -> AkShareClient {
        let mut builder = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .http1_only()
            .connect_timeout(self.connect_timeout)
            .timeout(self.timeout)
            .pool_max_idle_per_host(8);

        if let Some(proxy_url) = &self.proxy
            && let Ok(proxy) = reqwest::Proxy::all(proxy_url)
        {
            builder = builder.proxy(proxy);
        }

        let http = builder.build().unwrap_or_else(|_| reqwest::Client::new());

        AkShareClient {
            http,
            tushare_token: self.tushare_token,
            mock_uri: None,
        }
    }
}
