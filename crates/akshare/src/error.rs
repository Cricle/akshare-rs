//! Error types for akshare-rs.
//!
//! All API methods return [`Result<T>`](crate::Result) which wraps [`Error`].

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    UnsupportedMarket,
    PermissionDenied,
    Restricted,
    MissingCredentials,
    NotFound,
    Upstream,
    InvalidInput,
    Decode,
}

impl ErrorKind {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedMarket => "unsupported_market",
            Self::PermissionDenied => "permission_denied",
            Self::Restricted => "restricted",
            Self::MissingCredentials => "missing_credentials",
            Self::NotFound => "not_found",
            Self::Upstream => "upstream_error",
            Self::InvalidInput => "invalid_input",
            Self::Decode => "decode_error",
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn unsupported_market(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::UnsupportedMarket, msg)
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::NotFound, msg)
    }

    pub fn upstream(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::Upstream, msg)
    }

    pub fn decode(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::Decode, msg)
    }

    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::InvalidInput, msg)
    }

    pub fn missing_credentials(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::MissingCredentials, msg)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.kind.as_str(), self.message)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        let kind = if err.is_status() {
            match err.status() {
                Some(s)
                    if s == reqwest::StatusCode::FORBIDDEN
                        || s == reqwest::StatusCode::UNAUTHORIZED =>
                {
                    ErrorKind::Restricted
                }
                Some(s) if s == reqwest::StatusCode::NOT_FOUND => ErrorKind::NotFound,
                _ => ErrorKind::Upstream,
            }
        } else {
            ErrorKind::Upstream
        };
        Self::new(kind, format!("HTTP error: {err}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::new(ErrorKind::Decode, format!("JSON error: {err}"))
    }
}
