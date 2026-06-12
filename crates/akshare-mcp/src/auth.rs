use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// Axum middleware that validates `X-MCP-KEY` header.
/// If `mcp_key` is empty, auth is skipped (dev mode).
pub async fn auth_middleware(
    State(mcp_key): State<String>,
    headers: HeaderMap,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if mcp_key.is_empty() {
        return Ok(next.run(request).await);
    }

    match headers.get("X-MCP-KEY").and_then(|v| v.to_str().ok()) {
        Some(key) if key == mcp_key => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_headers(key: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(k) = key {
            headers.insert("X-MCP-KEY", k.parse().unwrap());
        }
        headers
    }

    fn check_auth(headers: &HeaderMap, mcp_key: &str) -> bool {
        if mcp_key.is_empty() {
            return true;
        }
        headers
            .get("X-MCP-KEY")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|k| k == mcp_key)
    }

    #[test]
    fn test_empty_key_skips_auth() {
        let headers = make_headers(None);
        assert!(check_auth(&headers, ""));
    }

    #[test]
    fn test_valid_key_passes() {
        let headers = make_headers(Some("my-secret"));
        assert!(check_auth(&headers, "my-secret"));
    }

    #[test]
    fn test_wrong_key_rejected() {
        let headers = make_headers(Some("wrong-key"));
        assert!(!check_auth(&headers, "my-secret"));
    }

    #[test]
    fn test_missing_key_rejected() {
        let headers = make_headers(None);
        assert!(!check_auth(&headers, "my-secret"));
    }
}
