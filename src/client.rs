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
    pub fn new() -> Self {
        Self::builder().build()
    }

    /// Create a test client that redirects all HTTP requests to a mock server.
    pub fn with_mock(mock_uri: String) -> Self {
        let mut client = Self::new();
        client.mock_uri = Some(mock_uri);
        client
    }

    /// Redirect a URL to the mock server if mock_uri is set.
    pub(crate) fn redirect_url(&self, url: &str) -> String {
        if let Some(mock) = &self.mock_uri {
            if let Some(proto_end) = url.find("//") {
                if let Some(path_start) = url[proto_end + 2..].find('/') {
                    let path = &url[proto_end + 2 + path_start..];
                    return format!("{}{}", mock.trim_end_matches('/'), path);
                }
            }
            mock.clone()
        } else {
            url.to_string()
        }
    }

    /// HTTP GET with automatic mock URL redirection.
    pub(crate) fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.http.get(self.redirect_url(url))
    }

    /// HTTP POST with automatic mock URL redirection.
    pub(crate) fn post(&self, url: &str) -> reqwest::RequestBuilder {
        self.http.post(self.redirect_url(url))
    }

    pub fn builder() -> AkShareClientBuilder {
        AkShareClientBuilder {
            tushare_token: None,
            user_agent: "akshare-rust/0.1".to_string(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            proxy: None,
        }
    }

    pub fn with_tushare_token(token: impl Into<String>) -> Self {
        Self::builder().tushare_token(token).build()
    }
}

impl Default for AkShareClient {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AkShareClientBuilder {
    tushare_token: Option<String>,
    user_agent: String,
    timeout: Duration,
    connect_timeout: Duration,
    proxy: Option<String>,
}

impl AkShareClientBuilder {
    pub fn tushare_token(mut self, token: impl Into<String>) -> Self {
        self.tushare_token = Some(token.into());
        self
    }

    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    pub fn proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }

    pub fn build(self) -> AkShareClient {
        let mut builder = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .http1_only()
            .connect_timeout(self.connect_timeout)
            .timeout(self.timeout)
            .pool_max_idle_per_host(8);

        if let Some(proxy_url) = &self.proxy {
            if let Ok(proxy) = reqwest::Proxy::all(proxy_url) {
                builder = builder.proxy(proxy);
            }
        }

        let http = builder.build().unwrap_or_else(|_| reqwest::Client::new());

        AkShareClient {
            http,
            tushare_token: self.tushare_token,
            mock_uri: None,
        }
    }
}
