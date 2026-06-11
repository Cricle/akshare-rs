# akshare-mcp Design Spec

## Overview

Add an `akshare-mcp` crate that wraps akshare-rs as an MCP (Model Context Protocol) server, providing both stdio and HTTP/SSE transports. The HTTP transport supports authentication via a configurable `X-MCP-KEY` header.

## Project Structure

Convert the project to a Cargo workspace:

```
akshare-rs/
├── Cargo.toml              # workspace root
├── crates/
│   ├── akshare/            # existing crate (moved from root)
│   │   ├── Cargo.toml
│   │   └── src/
│   └── akshare-mcp/        # new MCP server crate
│       ├── Cargo.toml
│       ├── config.toml      # default config template
│       └── src/
│           ├── main.rs      # CLI: stdio | http subcommand
│           ├── config.rs    # TOML config loader
│           ├── auth.rs      # X-MCP-KEY middleware
│           └── tools/       # MCP tool definitions
│               ├── mod.rs
│               ├── stock.rs
│               ├── fund.rs
│               ├── bond.rs
│               ├── futures.rs
│               ├── option.rs
│               ├── forex.rs
│               ├── crypto.rs
│               ├── macro_data.rs
│               ├── index.rs
│               ├── economy.rs
│               └── news.rs
```

## Config

**config.toml** (TOML format):

```toml
[http]
bind = "127.0.0.1:8080"
mcp_key = "your-secret-key-here"
```

- Loaded from `./config.toml` by default, or via `--config <path>` flag
- If `mcp_key` is empty or unset, auth is skipped (dev mode)

## Auth

axum middleware for the HTTP transport that:

1. Reads `X-MCP-KEY` header from incoming requests
2. Compares against `config.http.mcp_key`
3. If key is set and header doesn't match → 401 Unauthorized
4. If key is empty/unset → pass through (dev mode)
5. Only applies to `http` subcommand, not `stdio`

## CLI

```bash
# stdio mode (no auth)
akshare-mcp stdio

# HTTP/SSE mode with default config (./config.toml)
akshare-mcp http

# HTTP/SSE mode with custom config
akshare-mcp http --config /path/to/config.toml
```

Uses `clap` for argument parsing.

## Transport

### stdio

Uses `rmcp::transport::stdio()`. Standard MCP stdio protocol — reads JSON-RPC from stdin, writes to stdout.

### HTTP/SSE

Uses `rmcp::transport::streamable_http_server::StreamableHttpService` with axum:

```rust
let mcp_service = StreamableHttpService::new(
    move || Ok(AkShareMcpService::new()),
    LocalSessionManager::default().into(),
    StreamableHttpServerConfig::default(),
);
let app = Router::new()
    .nest_service("/mcp", mcp_service)
    .layer(middleware::from_fn_with_state(mcp_key, auth_middleware));
```

## Service Architecture

Central service struct:

```rust
#[derive(Clone)]
struct AkShareMcpService {
    client: AkShareClient,
    tool_router: ToolRouter<AkShareMcpService>,
}
```

- Holds a shared `AkShareClient` instance
- Merges all `*Tools` routers at construction
- Implements `ServerHandler` via `#[tool_handler]` and `#[prompt_handler]` macros

## MCP Tools

Each akshare-rs module maps to a `*Tools` struct with `#[tool_router]`. Tool names mirror Rust function names (snake_case). Parameters derived from function signatures via `schemars::JsonSchema`.

### Tool pattern

```rust
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct AShareQuoteParams {
    /// Stock code, e.g. "600000"
    pub symbol: String,
}

#[tool_router]
impl StockTools {
    #[tool(description = "Get A-share real-time quote by symbol")]
    async fn a_share_quote(
        &self,
        Parameters(AShareQuoteParams { symbol }): Parameters<AShareQuoteParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self.client.a_share_quote(&symbol).await
            .map_err(|e| McpError::internal_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap()
        )]))
    }
}
```

### Coverage

| Tools struct | Source module | Key functions |
|---|---|---|
| `StockTools` | stock | a_share_quote, a_share_candles, hk_quote, hk_candles, us_candles, billboard, capital_flow, ... |
| `FundTools` | fund | etf_list, etf_realtime, lof_list, fund_rank, fund_holdings, ... |
| `BondTools` | bond | gov_bond_yield, corp_bond, convertible_bond, ... |
| `FuturesTools` | futures | futures_spot, futures_warehouse, futures_cot, ... |
| `OptionTools` | option | sse_options, czce_options, cffex_options, ... |
| `ForexTools` | forex | boc_rates, forex_realtime, ... |
| `CryptoTools` | crypto | bitcoin, crypto_daily, ... |
| `IndexTools` | index | csi_index, sw_index, global_index, vix, ... |
| `MacroTools` | macro_data | china_gdp, us_cpi, eu_pmi, ... |
| `EconomyTools` | economy | auto_sales, box_office, ... |
| `NewsTools` | news | cctv_news, news_search, ... |

## Dependencies

```toml
[dependencies]
akshare = { path = "../akshare" }
rmcp = { version = "1.7", features = ["server", "macros", "transport-io", "transport-streamable-http-server", "schemars"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "signal"] }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
clap = { version = "4", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## Testing

- Unit tests for config parsing
- Integration tests using `rmcp`'s `duplex` transport to test tool calls without real HTTP
- Manual testing with `npx @modelcontextprotocol/inspector`
