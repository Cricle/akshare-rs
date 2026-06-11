use std::path::Path;

use akshare_mcp::{auth, config, tools::AkShareMcpService};
use clap::{Parser, Subcommand};
use rmcp::ServiceExt;

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
            let service = AkShareMcpService::new()
                .serve(rmcp::transport::stdio())
                .await
                .inspect_err(|e| {
                    tracing::error!("stdio serve error: {e:?}");
                })?;
            service.waiting().await?;
        }
        Commands::Http { config } => {
            let cfg = config::Config::load(Path::new(&config))?;
            let mcp_key = cfg.http.mcp_key.clone();

            let mcp_service: rmcp::transport::streamable_http_server::StreamableHttpService<
                AkShareMcpService,
                rmcp::transport::streamable_http_server::session::local::LocalSessionManager,
            > = rmcp::transport::streamable_http_server::StreamableHttpService::new(
                || Ok(AkShareMcpService::new()),
                rmcp::transport::streamable_http_server::session::local::LocalSessionManager::default().into(),
                rmcp::transport::streamable_http_server::StreamableHttpServerConfig::default(),
            );

            let app = axum::Router::new()
                .nest_service("/mcp", mcp_service)
                .layer(axum::middleware::from_fn_with_state(
                    mcp_key,
                    auth::auth_middleware,
                ));

            let listener = tokio::net::TcpListener::bind(&cfg.http.bind).await?;
            tracing::info!("MCP HTTP server listening on {}", cfg.http.bind);

            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    tokio::signal::ctrl_c().await.ok();
                })
                .await?;
        }
    }
    Ok(())
}
