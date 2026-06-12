//! # akshare-mcp
//!
//! MCP (Model Context Protocol) server for [akshare-rs](https://github.com/akfamily/akshare-rs).
//! Provides 42 tools covering A-share, HK, US stocks, funds, bonds, futures,
//! options, forex, crypto, macro data, economy, and news.
//!
//! ## Quick Start
//!
//! ```bash
//! # Run as stdio MCP server
//! cargo run -p akshare-mcp -- stdio
//!
//! # Run as HTTP/SSE MCP server
//! cargo run -p akshare-mcp -- http --config config.toml
//! ```
//!
//! ## Transport Modes
//!
//! - **stdio**: Standard input/output, ideal for local integration with AI assistants
//! - **HTTP/SSE**: Streamable HTTP with optional `X-MCP-KEY` header authentication
//!
//! ## Configuration
//!
//! HTTP mode reads `config.toml`:
//!
//! ```toml
//! [http]
//! bind = "127.0.0.1:8080"
//! mcp_key = "your-secret-key"
//! ```

#![allow(clippy::missing_errors_doc, clippy::doc_markdown)]

pub mod auth;
pub mod config;
pub mod tools;
