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
    Stdio {
        /// Comma-separated list of tool categories to enable (e.g. "stock,bond,index")
        #[arg(long, value_delimiter = ',')]
        categories: Vec<String>,
        /// Additional categories to enable (keeps defaults)
        #[arg(long, value_delimiter = ',')]
        enable: Vec<String>,
        /// Categories to disable
        #[arg(long, value_delimiter = ',')]
        disable: Vec<String>,
    },
    /// Run as HTTP/SSE MCP server
    Http {
        /// Path to config file
        #[arg(long, default_value = "config.toml")]
        config: String,
        /// Comma-separated list of tool categories to enable (overrides config file)
        #[arg(long, value_delimiter = ',')]
        categories: Vec<String>,
        /// Additional categories to enable (keeps config defaults)
        #[arg(long, value_delimiter = ',')]
        enable: Vec<String>,
        /// Categories to disable
        #[arg(long, value_delimiter = ',')]
        disable: Vec<String>,
    },
}

fn build_tools_config(
    categories: &[String],
    enable: &[String],
    disable: &[String],
    base: config::ToolsConfig,
) -> config::ToolsConfig {
    let mut cfg = if categories.is_empty() {
        base
    } else {
        // --categories overrides everything: start with all disabled
        let mut c = config::ToolsConfig {
            stock: false,
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
        };
        for name in categories {
            c.enable(name);
        }
        c
    };

    for name in enable {
        cfg.enable(name);
    }
    for name in disable {
        cfg.disable(name);
    }
    cfg
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Stdio {
            categories,
            enable,
            disable,
        } => {
            tracing::info!("Starting akshare-mcp in stdio mode");
            let tools_cfg = build_tools_config(
                &categories,
                &enable,
                &disable,
                config::ToolsConfig::default(),
            );
            let service = AkShareMcpService::new(tools_cfg)
                .serve(rmcp::transport::stdio())
                .await
                .inspect_err(|e| {
                    tracing::error!("stdio serve error: {e:?}");
                })?;
            service.waiting().await?;
        }
        Commands::Http {
            config,
            categories,
            enable,
            disable,
        } => {
            let cfg = config::Config::load(Path::new(&config))?;
            let mcp_key = cfg.http.mcp_key.clone();
            let tools_cfg = build_tools_config(&categories, &enable, &disable, cfg.tools);

            let mcp_service: rmcp::transport::streamable_http_server::StreamableHttpService<
                AkShareMcpService,
                rmcp::transport::streamable_http_server::session::local::LocalSessionManager,
            > = rmcp::transport::streamable_http_server::StreamableHttpService::new(
                move || Ok(AkShareMcpService::new(tools_cfg.clone())),
                rmcp::transport::streamable_http_server::session::local::LocalSessionManager::default().into(),
                rmcp::transport::streamable_http_server::StreamableHttpServerConfig::default(),
            );

            let app = axum::Router::new().nest_service("/mcp", mcp_service).layer(
                axum::middleware::from_fn_with_state(mcp_key, auth::auth_middleware),
            );

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
