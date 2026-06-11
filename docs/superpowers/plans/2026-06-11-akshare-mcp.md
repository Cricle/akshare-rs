# akshare-mcp Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an `akshare-mcp` crate that exposes akshare-rs as an MCP server with stdio and HTTP/SSE transports, authenticated via `X-MCP-KEY` header.

**Architecture:** Cargo workspace with two crates: `akshare` (existing, moved to `crates/akshare`) and `akshare-mcp` (new, at `crates/akshare-mcp`). The MCP server uses `rmcp` crate with `#[tool_router]` macros to define tools. HTTP transport uses axum with `StreamableHttpService` and custom auth middleware.

**Tech Stack:** Rust 2024 edition (MSRV 1.85), rmcp 1.7, axum 0.8, tokio, clap, toml, serde, tracing

---

## File Map

| File | Purpose |
|------|---------|
| `Cargo.toml` (root) | Workspace definition |
| `crates/akshare/Cargo.toml` | Existing akshare crate manifest (moved) |
| `crates/akshare/src/**` | Existing source (moved) |
| `crates/akshare-mcp/Cargo.toml` | New MCP crate manifest |
| `crates/akshare-mcp/config.toml` | Default config template |
| `crates/akshare-mcp/src/main.rs` | CLI entry point (stdio/http subcommands) |
| `crates/akshare-mcp/src/config.rs` | TOML config loader |
| `crates/akshare-mcp/src/auth.rs` | X-MCP-KEY axum middleware |
| `crates/akshare-mcp/src/tools/mod.rs` | Tool router aggregation |
| `crates/akshare-mcp/src/tools/stock.rs` | Stock MCP tools |
| `crates/akshare-mcp/src/tools/fund.rs` | Fund MCP tools |
| `crates/akshare-mcp/src/tools/bond.rs` | Bond MCP tools |
| `crates/akshare-mcp/src/tools/futures.rs` | Futures MCP tools |
| `crates/akshare-mcp/src/tools/option.rs` | Option MCP tools |
| `crates/akshare-mcp/src/tools/forex.rs` | Forex MCP tools |
| `crates/akshare-mcp/src/tools/crypto.rs` | Crypto MCP tools |
| `crates/akshare-mcp/src/tools/index.rs` | Index MCP tools |
| `crates/akshare-mcp/src/tools/macro_data.rs` | Macro data MCP tools |
| `crates/akshare-mcp/src/tools/economy.rs` | Economy MCP tools |
| `crates/akshare-mcp/src/tools/news.rs` | News MCP tools |

---

### Task 1: Convert to Cargo Workspace

Move the existing akshare crate into `crates/akshare/` and create a workspace root `Cargo.toml`.

**Files:**
- Modify: `Cargo.toml` (root — becomes workspace manifest)
- Move: `src/` → `crates/akshare/src/`
- Move: `Cargo.lock` → stays at root
- Move: `tests/` → `crates/akshare/tests/`

- [ ] **Step 1: Create crates directory and move akshare**

```bash
mkdir -p crates
git mv src crates/akshare/src
git mv tests crates/akshare/tests
```

- [ ] **Step 2: Move akshare Cargo.toml**

```bash
git mv Cargo.toml crates/akshare/Cargo.toml
```

- [ ] **Step 3: Create workspace root Cargo.toml**

```toml
[workspace]
members = ["crates/akshare", "crates/akshare-mcp"]
resolver = "2"

[workspace.package]
edition = "2024"
rust-version = "1.85"
license = "MIT OR Apache-2.0"
repository = "https://github.com/Cricle/akshare-rs"
```

- [ ] **Step 4: Update crates/akshare/Cargo.toml — remove fields now in workspace**

Remove `edition`, `rust-version`, `license`, `repository` fields from `[package]` since they're inherited from workspace. Add `version.workspace = true` etc. where appropriate. Keep `name`, `description`, `homepage`, `documentation`, `readme`, `keywords`, `categories`, `exclude`.

- [ ] **Step 5: Verify workspace builds**

```bash
cargo check
```

Expected: Compiles successfully.

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "refactor: convert to cargo workspace, move akshare to crates/akshare"
```

---

### Task 2: Create akshare-mcp Crate Skeleton

**Files:**
- Create: `crates/akshare-mcp/Cargo.toml`
- Create: `crates/akshare-mcp/src/main.rs`
- Create: `crates/akshare-mcp/config.toml`

- [ ] **Step 1: Create Cargo.toml**

```toml
[package]
name = "akshare-mcp"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
description = "MCP server for akshare-rs — financial market data via Model Context Protocol"

[dependencies]
akshare = { path = "../akshare" }
rmcp = { version = "1.7", features = [
    "server",
    "macros",
    "transport-io",
    "transport-streamable-http-server",
    "schemars",
] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "signal"] }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
clap = { version = "4", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1"
```

- [ ] **Step 2: Create minimal main.rs**

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "akshare-mcp", about = "MCP server for akshare financial data")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run as stdio MCP server
    Stdio,
    /// Run as HTTP/SSE MCP server
    Http {
        /// Path to config file
        #[arg(long, default_value = "config.toml")]
        config: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Stdio => {
            tracing::info!("Starting akshare-mcp in stdio mode");
            // TODO: next task
        }
        Commands::Http { config } => {
            tracing::info!("Starting akshare-mcp in HTTP mode with config: {config}");
            // TODO: next task
        }
    }
    Ok(())
}
```

- [ ] **Step 3: Create config.toml template**

```toml
[http]
bind = "127.0.0.1:8080"
mcp_key = ""
```

- [ ] **Step 4: Verify it compiles**

```bash
cargo check -p akshare-mcp
```

Expected: Compiles (with TODO placeholders).

- [ ] **Step 5: Commit**

```bash
git add crates/akshare-mcp/
git commit -m "feat: add akshare-mcp crate skeleton with CLI"
```

---

### Task 3: Config Loader

**Files:**
- Create: `crates/akshare-mcp/src/config.rs`
- Modify: `crates/akshare-mcp/src/main.rs`

- [ ] **Step 1: Create config.rs**

```rust
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub http: HttpConfig,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    #[serde(default = "default_bind")]
    pub bind: String,
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
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
```

- [ ] **Step 2: Add mod config to main.rs**

Add `mod config;` at the top of `main.rs`. Update the `Http` match arm to load config:

```rust
Commands::Http { config } => {
    let cfg = config::Config::load(Path::new(&config))?;
    tracing::info!("Starting akshare-mcp in HTTP mode on {}", cfg.http.bind);
}
```

Add `use std::path::Path;` to imports.

- [ ] **Step 3: Verify compilation**

```bash
cargo check -p akshare-mcp
```

- [ ] **Step 4: Commit**

```bash
git add crates/akshare-mcp/src/config.rs crates/akshare-mcp/src/main.rs
git commit -m "feat: add TOML config loader for akshare-mcp"
```

---

### Task 4: Auth Middleware

**Files:**
- Create: `crates/akshare-mcp/src/auth.rs`
- Modify: `crates/akshare-mcp/src/main.rs`

- [ ] **Step 1: Create auth.rs**

```rust
use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// Axum middleware that validates X-MCP-KEY header.
/// If mcp_key is empty, auth is skipped (dev mode).
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
```

- [ ] **Step 2: Add mod auth to main.rs**

Add `mod auth;` at the top of `main.rs`.

- [ ] **Step 3: Verify compilation**

```bash
cargo check -p akshare-mcp
```

- [ ] **Step 4: Commit**

```bash
git add crates/akshare-mcp/src/auth.rs crates/akshare-mcp/src/main.rs
git commit -m "feat: add X-MCP-KEY auth middleware"
```

---

### Task 5: MCP Service Struct and Tool Aggregation

**Files:**
- Create: `crates/akshare-mcp/src/tools/mod.rs`
- Modify: `crates/akshare-mcp/src/main.rs`

- [ ] **Step 1: Create tools/mod.rs with the central service struct**

```rust
pub mod bond;
pub mod crypto;
pub mod economy;
pub mod fund;
pub mod futures;
pub mod forex;
pub mod index;
pub mod macro_data;
pub mod news;
pub mod option;
pub mod stock;

use akshare::AkShareClient;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::router::tool::ToolRouter,
    model::*,
    tool_handler, tool_router,
};

#[derive(Clone)]
pub struct AkShareMcpService {
    pub client: AkShareClient,
    tool_router: ToolRouter<AkShareMcpService>,
}

#[tool_router]
impl AkShareMcpService {
    pub fn new() -> Self {
        let client = AkShareClient::new();
        let mut service = Self {
            client,
            tool_router: Self::tool_router(),
        };
        // Merge all sub-module routers
        service.tool_router.merge(stock::StockTools::tool_router());
        service.tool_router.merge(fund::FundTools::tool_router());
        service.tool_router.merge(bond::BondTools::tool_router());
        service.tool_router.merge(futures::FuturesTools::tool_router());
        service.tool_router.merge(option::OptionTools::tool_router());
        service.tool_router.merge(forex::ForexTools::tool_router());
        service.tool_router.merge(crypto::CryptoTools::tool_router());
        service.tool_router.merge(index::IndexTools::tool_router());
        service.tool_router.merge(macro_data::MacroTools::tool_router());
        service.tool_router.merge(economy::EconomyTools::tool_router());
        service.tool_router.merge(news::NewsTools::tool_router());
        service
    }
}

#[tool_handler]
impl ServerHandler for AkShareMcpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .build(),
        )
        .with_server_info(Implementation {
            name: "akshare-mcp".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
        .with_instructions(
            "Financial market data MCP server powered by akshare-rs. \
             Provides tools for A-share, HK, US stocks, funds, bonds, \
             futures, options, forex, crypto, macro data, economy, and news."
                .to_string(),
        )
    }
}
```

- [ ] **Step 2: Create stub tool modules**

Create each of these files with a minimal stub pattern. Example for `stock.rs`:

```rust
// crates/akshare-mcp/src/tools/stock.rs
use std::sync::Arc;

use akshare::AkShareClient;
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::*,
    schemars, tool, tool_router,
};

#[derive(Clone)]
pub struct StockTools {
    client: AkShareClient,
    tool_router: ToolRouter<StockTools>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SymbolParams {
    /// Stock symbol, e.g. "600000" (A-share), "00593" (HK), "AAPL" (US)
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CandlesParams {
    /// Stock symbol
    pub symbol: String,
    /// Number of candles to return
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize { 60 }

#[tool_router]
impl StockTools {
    pub fn new(client: AkShareClient) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get A-share real-time quote")]
    async fn a_share_quote(
        &self,
        Parameters(SymbolParams { symbol }): Parameters<SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self.client.a_share_quote(&symbol).await
            .map_err(|e| McpError::internal_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share K-line candles (qfq/hfq/empty adjust)")]
    async fn a_share_candles(
        &self,
        Parameters(CandlesParams { symbol, limit }): Parameters<CandlesParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self.client.a_share_candles(&symbol, "qfq", limit).await
            .map_err(|e| McpError::internal_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock real-time quote")]
    async fn hk_quote(
        &self,
        Parameters(SymbolParams { symbol }): Parameters<SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self.client.hk_quote(&symbol).await
            .map_err(|e| McpError::internal_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock K-line candles")]
    async fn hk_candles(
        &self,
        Parameters(CandlesParams { symbol, limit }): Parameters<CandlesParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self.client.hk_candles(&symbol, limit).await
            .map_err(|e| McpError::internal_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US stock real-time quote")]
    async fn us_quote(
        &self,
        Parameters(SymbolParams { symbol }): Parameters<SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self.client.us_quote(&symbol).await
            .map_err(|e| McpError::internal_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US stock K-line candles")]
    async fn us_candles(
        &self,
        Parameters(CandlesParams { symbol, limit }): Parameters<CandlesParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self.client.us_candles(&symbol, limit).await
            .map_err(|e| McpError::internal_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }
}
```

Create the remaining stub files (`fund.rs`, `bond.rs`, `futures.rs`, `option.rs`, `forex.rs`, `crypto.rs`, `index.rs`, `macro_data.rs`, `economy.rs`, `news.rs`) with the same pattern: a `*Tools` struct holding `AkShareClient`, a `#[tool_router]` impl, and a `new()` constructor. Each file should have at least one tool as a placeholder — use these specific functions:

- **fund.rs**: `FundTools` with `etf_list`, `fund_rank`
- **bond.rs**: `BondTools` with `bond_zh_us_rate`, `bond_corporate_yields`
- **futures.rs**: `FuturesTools` with `futures_spot_prices`, `futures_main_sina`
- **option.rs**: `OptionTools` with `option_sse_greeks`, `option_czce_daily`
- **forex.rs**: `ForexTools` with `forex_boc_rates`, `forex_em_rates`
- **crypto.rs**: `CryptoTools` with `crypto_bitcoin_cme`
- **index.rs**: `IndexTools` with `stock_zh_index_daily_em`, `index_global_em`
- **macro_data.rs**: `MacroTools` with `macro_china_gdp`, `macro_usa_cpi`
- **economy.rs**: `EconomyTools` with `economy_auto_sales`, `economy_box_office`
- **news.rs**: `NewsTools` with `news_cctv`, `news_search`

Each tool follows the same pattern: call the corresponding `self.client.<method>()`, serialize result to JSON string, return as `Content::text`.

- [ ] **Step 3: Update tools/mod.rs to use new constructors**

Update `AkShareMcpService::new()` to pass `client.clone()` to each `*Tools::new(client.clone())`.

- [ ] **Step 4: Verify compilation**

```bash
cargo check -p akshare-mcp
```

Expected: All tool modules compile. Some tools may have incorrect function signatures — fix any compilation errors by checking the actual `AkShareClient` method signatures in `crates/akshare/src/`.

- [ ] **Step 5: Commit**

```bash
git add crates/akshare-mcp/src/tools/
git commit -m "feat: add MCP tool definitions for all akshare modules"
```

---

### Task 6: Wire Up stdio Transport

**Files:**
- Modify: `crates/akshare-mcp/src/main.rs`

- [ ] **Step 1: Update main.rs stdio command**

```rust
use rmcp::ServiceExt;
use rmcp::transport::stdio;

// In the Stdio match arm:
Commands::Stdio => {
    let service = AkShareMcpService::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("stdio serve error: {e:?}");
        })?;
    service.waiting().await?;
}
```

Add `mod tools;` to main.rs imports. Import `tools::AkShareMcpService`.

- [ ] **Step 2: Verify compilation**

```bash
cargo check -p akshare-mcp
```

- [ ] **Step 3: Commit**

```bash
git add crates/akshare-mcp/src/main.rs
git commit -m "feat: wire up stdio transport for akshare-mcp"
```

---

### Task 7: Wire Up HTTP/SSE Transport with Auth

**Files:**
- Modify: `crates/akshare-mcp/src/main.rs`

- [ ] **Step 1: Update main.rs HTTP command**

```rust
use axum::Router;
use axum::middleware;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService,
    session::local::LocalSessionManager,
};

// In the Http match arm:
Commands::Http { config } => {
    let cfg = config::Config::load(Path::new(&config))?;
    let mcp_key = cfg.http.mcp_key.clone();

    let mcp_service: StreamableHttpService<AkShareMcpService, LocalSessionManager> =
        StreamableHttpService::new(
            || Ok(AkShareMcpService::new()),
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig::default(),
        );

    let app = Router::new()
        .nest_service("/mcp", mcp_service)
        .layer(middleware::from_fn_with_state(mcp_key, auth::auth_middleware));

    let listener = tokio::net::TcpListener::bind(&cfg.http.bind).await?;
    tracing::info!("MCP HTTP server listening on {}", cfg.http.bind);

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await?;
}
```

- [ ] **Step 2: Verify compilation**

```bash
cargo check -p akshare-mcp
```

- [ ] **Step 3: Commit**

```bash
git add crates/akshare-mcp/src/main.rs
git commit -m "feat: wire up HTTP/SSE transport with X-MCP-KEY auth"
```

---

### Task 8: Integration Test — stdio Round-Trip

**Files:**
- Create: `crates/akshare-mcp/tests/integration.rs`

- [ ] **Step 1: Create integration test**

```rust
use rmcp::{ClientHandler, ServiceExt};
use akshare_mcp::tools::AkShareMcpService;

#[derive(Default, Clone)]
struct TestClient;
impl ClientHandler for TestClient {}

#[tokio::test]
async fn test_server_info() {
    let service = AkShareMcpService::new();
    let (server_transport, client_transport) = tokio::io::duplex(4096);

    let _server_handle = tokio::spawn(async move {
        let s = service.serve(server_transport).await?;
        s.waiting().await?;
        anyhow::Ok(())
    });

    let client = TestClient::default().serve(client_transport).await?;
    let info = client.peer_info();
    assert!(info.is_some());
    let info = info.unwrap();
    assert_eq!(info.server_info.name, "akshare-mcp");

    client.cancel().await?;
}
```

Note: This test requires exposing `AkShareMcpService` as `pub` from `tools/mod.rs`. Add `pub mod tools;` in `main.rs` or move the service to a `lib.rs`.

- [ ] **Step 2: Run the test**

```bash
cargo test -p akshare-mcp --test integration
```

Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add crates/akshare-mcp/tests/
git commit -m "test: add stdio integration test for akshare-mcp"
```

---

### Task 9: Manual Verification with MCP Inspector

- [ ] **Step 1: Build the binary**

```bash
cargo build -p akshare-mcp
```

- [ ] **Step 2: Test stdio mode with MCP Inspector**

```bash
npx @modelcontextprotocol/inspector ./target/debug/akshare-mcp stdio
```

Expected: Inspector connects, shows tool list with all defined tools.

- [ ] **Step 3: Test HTTP mode**

In one terminal:
```bash
./target/debug/akshare-mcp http --config crates/akshare-mcp/config.toml
```

In another:
```bash
curl -X POST -H "Content-Type: application/json" \
  -H "X-MCP-KEY: " \
  -d '{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1"}},"id":1}' \
  http://127.0.0.1:8080/mcp
```

Expected: JSON-RPC response with server info.

- [ ] **Step 4: Test auth rejection**

```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}' \
  http://127.0.0.1:8080/mcp
```

Expected: 401 Unauthorized (if mcp_key is set in config).

- [ ] **Step 5: Commit any fixes**

```bash
git add -A
git commit -m "fix: address manual testing findings"
```

---

### Task 10: Expand Tool Coverage

Add remaining tools to each module. Follow the same pattern from Task 5.

- [ ] **Step 1: Expand stock.rs**

Add tools for: `stock_hk_spot`, `stock_hk_daily`, `stock_zh_a_spot_em`, `stock_zh_a_hist`, `stock_zh_a_hist_min_em`, `stock_board_industry_name_em`, `stock_board_concept_name_em`, `stock_individual_fund_flow`, `stock_hsgt_north_net_flow_in_em`, `stock_financial_analysis_indicator`.

Each tool: define params struct with `schemars::JsonSchema`, implement `#[tool]` method calling `self.client.<method>()`.

- [ ] **Step 2: Expand fund.rs**

Add: `fund_etf_spot_em`, `fund_lof_spot_em`, `fund_open_fund_rank_em`, `fund_open_fund_info_em`, `fund_portfolio_hold_em`.

- [ ] **Step 3: Expand bond.rs**

Add: `bond_china_yield`, `bond_cash_summary_sse`, `bond_spot_deal`, `bond_spot_rates`.

- [ ] **Step 4: Expand futures.rs**

Add: `futures_daily_cffex`, `futures_daily_shfe`, `futures_spot_stock`, `futures_shfe_position_rank`, `futures_warehouse_receipt_czce`.

- [ ] **Step 5: Expand remaining modules**

Add 2-3 more tools to each of: `option.rs`, `forex.rs`, `crypto.rs`, `index.rs`, `macro_data.rs`, `economy.rs`, `news.rs`.

- [ ] **Step 6: Verify compilation**

```bash
cargo check -p akshare-mcp
```

- [ ] **Step 7: Commit**

```bash
git add crates/akshare-mcp/src/tools/
git commit -m "feat: expand MCP tool coverage across all modules"
```
