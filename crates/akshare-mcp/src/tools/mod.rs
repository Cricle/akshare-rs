//! MCP tool definitions for akshare financial data.
//!
//! Tools are auto-discovered from `AkShareClient` methods by `build.rs`.
//! Only two meta-tools are exposed via MCP: `tools/search` and `tools/call`.

pub mod bond;
pub mod crypto;
pub mod economy;
pub mod forex;
pub mod fund;
pub mod futures;
pub mod index;
pub mod macro_data;
pub mod news;
pub mod option;
pub mod stock;

use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

use akshare::AkShareClient;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::tool::schema_for_type,
    model::{
        CallToolRequestParams, CallToolResult, Content, Implementation, PaginatedRequestParams,
        ServerCapabilities, ServerInfo, Tool,
    },
    schemars,
    service::RequestContext,
};

use crate::config::ToolsConfig;

// ── Tool registry types ──────────────────────────────────────────────

type Handler = fn(
    &AkShareClient,
    serde_json::Value,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<serde_json::Value, String>> + Send>,
>;

pub(crate) struct ToolEntry {
    pub description: &'static str,
    pub category: &'static str,
    pub schema: Arc<serde_json::Map<String, serde_json::Value>>,
    pub handler: Handler,
}

/// Register a tool in the registry.
/// The closure `|$client, $p| $body` must return `Pin<Box<dyn Future<...> + Send>>`.
/// The macro deserializes `$p` from the JSON args and calls the closure.
macro_rules! register_tool {
    ($registry:expr, $name:expr, $desc:expr, $cat:expr, $ptype:ty, |$client:ident, $p:ident| $body:expr) => {{
        let handler: crate::tools::Handler = |client, args| {
            Box::pin({
                let $client = client.clone();
                async move {
                    let $p: $ptype = serde_json::from_value(args).map_err(|e| e.to_string())?;
                    $body.await
                }
            })
        };
        $registry.insert(
            $name,
            crate::tools::ToolEntry {
                description: $desc,
                category: $cat,
                schema: rmcp::handler::server::tool::schema_for_type::<$ptype>(),
                handler,
            },
        );
    }};
}

// Include the auto-generated tool registry from `build.rs`.
include!(concat!(env!("OUT_DIR"), "/tool_registry.rs"));

static TOOL_REGISTRY: LazyLock<HashMap<&'static str, ToolEntry>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    load_registry(&mut m);
    m
});

// ── Category prefixes for filtering ──────────────────────────────────

const CATEGORY_PREFIXES: &[(&str, &[&str])] = &[
    ("stock", &["stock_", "a_share_", "hk_", "us_", "get_us_"]),
    ("bond", &["bond_"]),
    ("index", &["index_"]),
    ("futures", &["futures_"]),
    (
        "economy",
        &[
            "economy_",
            "movie_",
            "nlp_",
            "amac_",
            "car_",
            "sw_",
            "fx_",
            "article_",
            "air_",
            "qdii_",
            "video_",
            "sunrise_",
            "repo_",
            "migration_",
            "fred_",
            "xincaifu_",
            "spot_",
            "sogou_",
            "rate_",
            "online_",
            "match_",
            "hurun_",
            "hf_",
            "google_",
            "gdelt_",
            "game_",
            "forbes_",
            "drewry_",
            "business_",
            "bing_",
            "baidu_",
            "qhkc_",
            "methods_in_",
        ],
    ),
    ("crypto", &["crypto_"]),
    ("forex", &["forex_", "currency_"]),
    ("option", &["option_"]),
    ("news", &["news_"]),
    ("macro_data", &["macro_"]),
    ("fund", &["fund_"]),
];

// ── Meta-tool param types ──────────────────────────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
struct ToolsSearchParams {
    /// Keyword to match against tool name or description.
    query: Option<String>,
    /// Filter by category: stock, bond, index, futures, economy, crypto, forex, option, news, macro_data, fund.
    category: Option<String>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
struct ToolsCallParams {
    /// Exact tool name (use tools/search to discover).
    name: String,
    /// JSON object of parameters to pass to the tool.
    arguments: Option<serde_json::Map<String, serde_json::Value>>,
}

// ── Service ──────────────────────────────────────────────────────────

/// MCP service exposing akshare financial data via `tools/search` + `tools/call`.
#[derive(Clone)]
pub struct AkShareMcpService {
    client: AkShareClient,
    enabled_categories: Vec<String>,
}

impl AkShareMcpService {
    #[must_use]
    pub fn new(tools_config: ToolsConfig) -> Self {
        Self {
            client: AkShareClient::new(),
            enabled_categories: Self::enabled_categories(&tools_config),
        }
    }

    fn enabled_categories(config: &ToolsConfig) -> Vec<String> {
        CATEGORY_PREFIXES
            .iter()
            .filter(|(cat, _)| config.is_enabled(cat))
            .map(|(cat, _)| (*cat).to_string())
            .collect()
    }

    fn is_category_enabled(&self, category: &str) -> bool {
        self.enabled_categories.iter().any(|c| c == category)
    }

    /// Build the 2 meta-tools for MCP `list_tools`.
    fn meta_tools() -> Vec<Tool> {
        vec![
            Tool::new_with_raw(
                "tools/search",
                Some("Search available tools by keyword and/or category. Returns matching tool names and descriptions. Use this before tools/call to discover the exact tool name.".into()),
                schema_for_type::<ToolsSearchParams>(),
            ),
            Tool::new_with_raw(
                "tools/call",
                Some("Call any tool by name with JSON arguments. Use tools/search first to discover available tools and their parameter schemas.".into()),
                schema_for_type::<ToolsCallParams>(),
            ),
        ]
    }

    /// Search the tool registry by optional category and/or keyword.
    fn search_registry(
        &self,
        category: Option<&str>,
        query: Option<&str>,
    ) -> Vec<(String, String)> {
        let query_lower = query.map(|q| q.to_lowercase());
        let category_prefixes: Option<&[&str]> = category
            .and_then(|cat| CATEGORY_PREFIXES.iter().find(|(name, _)| *name == cat))
            .map(|(_, prefixes)| *prefixes);

        TOOL_REGISTRY
            .iter()
            .filter(|(name, entry)| {
                if !self.is_category_enabled(entry.category) {
                    return false;
                }
                if let Some(prefixes) = category_prefixes
                    && !prefixes.iter().any(|p| name.starts_with(p))
                {
                    return false;
                }
                if let Some(ref q) = query_lower {
                    let name_match = name.to_lowercase().contains(q.as_str());
                    let desc_match = entry.description.to_lowercase().contains(q.as_str());
                    if !name_match && !desc_match {
                        return false;
                    }
                }
                true
            })
            .map(|(name, entry)| (name.to_string(), entry.description.to_string()))
            .collect()
    }

    /// Get a single tool by name (public API for tests and external callers).
    pub fn get_tool(&self, name: &str) -> Option<Tool> {
        if name == "tools/search" || name == "tools/call" {
            return Self::meta_tools()
                .into_iter()
                .find(|t| t.name.as_ref() == name);
        }
        TOOL_REGISTRY.get(name).and_then(|entry| {
            if self.is_category_enabled(entry.category) {
                Some(Tool::new_with_raw(
                    name.to_string(),
                    Some(entry.description.into()),
                    entry.schema.clone(),
                ))
            } else {
                None
            }
        })
    }

    /// List all tools synchronously (including meta-tools).
    pub fn list_tools_sync(&self) -> Vec<Tool> {
        let mut tools = Self::meta_tools();
        for (name, entry) in TOOL_REGISTRY.iter() {
            if self.is_category_enabled(entry.category) {
                tools.push(Tool::new_with_raw(
                    *name,
                    Some(entry.description.into()),
                    entry.schema.clone(),
                ));
            }
        }
        tools
    }
}

// ── MCP handler ──────────────────────────────────────────────────────

impl ServerHandler for AkShareMcpService {
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        match request.name.as_ref() {
            "tools/search" => {
                let args: ToolsSearchParams = request
                    .arguments
                    .map(|obj| {
                        serde_json::from_value(serde_json::Value::Object(obj)).map_err(|e| {
                            McpError::invalid_params(
                                format!("failed to deserialize parameters: {e}"),
                                None,
                            )
                        })
                    })
                    .transpose()?
                    .unwrap_or(ToolsSearchParams {
                        query: None,
                        category: None,
                    });

                let results: Vec<serde_json::Value> = self
                    .search_registry(args.category.as_deref(), args.query.as_deref())
                    .into_iter()
                    .map(|(name, desc)| serde_json::json!({ "name": name, "description": desc }))
                    .collect();

                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&results).unwrap(),
                )]))
            }
            "tools/call" => {
                let args: ToolsCallParams = request
                    .arguments
                    .map(|obj| {
                        serde_json::from_value(serde_json::Value::Object(obj)).map_err(|e| {
                            McpError::invalid_params(
                                format!("failed to deserialize parameters: {e}"),
                                None,
                            )
                        })
                    })
                    .transpose()?
                    .ok_or_else(|| McpError::invalid_params("missing arguments", None))?;

                let entry = TOOL_REGISTRY.get(args.name.as_str()).ok_or_else(|| {
                    McpError::invalid_params(format!("unknown tool: {}", args.name), None)
                })?;

                if !self.is_category_enabled(entry.category) {
                    return Err(McpError::invalid_params(
                        format!("tool category '{}' is disabled", entry.category),
                        None,
                    ));
                }

                let args_value = args
                    .arguments
                    .map(serde_json::Value::Object)
                    .unwrap_or(serde_json::Value::Null);

                let handler = entry.handler;
                let result = handler(&self.client, args_value)
                    .await
                    .map_err(|e| McpError::internal_error(e, None))?;

                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&result).unwrap(),
                )]))
            }
            _ => Err(McpError::invalid_params(
                format!(
                    "unknown tool: {}. Use tools/search to discover available tools.",
                    request.name
                ),
                None,
            )),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, McpError> {
        let mut tools = Self::meta_tools();

        // Add all registry tools whose category is enabled
        for (name, entry) in TOOL_REGISTRY.iter() {
            if self.is_category_enabled(entry.category) {
                tools.push(Tool::new_with_raw(
                    *name,
                    Some(entry.description.into()),
                    entry.schema.clone(),
                ));
            }
        }

        Ok(rmcp::model::ListToolsResult {
            tools,
            meta: None,
            next_cursor: None,
        })
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        if name == "tools/search" || name == "tools/call" {
            return Self::meta_tools()
                .into_iter()
                .find(|t| t.name.as_ref() == name);
        }
        TOOL_REGISTRY.get(name).and_then(|entry| {
            if self.is_category_enabled(entry.category) {
                Some(Tool::new_with_raw(
                    name.to_string(),
                    Some(entry.description.into()),
                    entry.schema.clone(),
                ))
            } else {
                None
            }
        })
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "Financial market data MCP server powered by akshare-rs. \
                 Use tools/search to discover available tools, then tools/call to invoke them. \
                 Provides tools for A-share, HK, US stocks, funds, bonds, \
                 futures, options, forex, crypto, macro data, economy, and news."
                    .to_string(),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creates() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let info = service.get_info();
        assert!(info.capabilities.tools.is_some());
    }

    #[test]
    fn test_service_info_instructions() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let info = service.get_info();
        let instructions = info.instructions.unwrap();
        assert!(instructions.contains("akshare-rs"));
    }

    #[test]
    fn test_service_clone() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let cloned = service;
        let info = cloned.get_info();
        assert!(info.capabilities.tools.is_some());
    }

    #[test]
    fn test_list_tools_has_meta_tools() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let tools = service.list_tools_sync();
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(names.contains(&"tools/search"));
        assert!(names.contains(&"tools/call"));
    }

    #[test]
    fn test_list_tools_has_registry_tools() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let tools = service.list_tools_sync();
        // Should have at least the 2 meta-tools + some registry tools
        assert!(tools.len() > 2, "expected registry tools to be listed");
    }

    #[test]
    fn test_get_tool_meta_tools() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        assert!(service.get_tool("tools/search").is_some());
        assert!(service.get_tool("tools/call").is_some());
    }

    #[test]
    fn test_category_filter() {
        let cfg = ToolsConfig {
            bond: true,
            ..ToolsConfig::default()
        };
        let service = AkShareMcpService::new(cfg);
        let results = service.search_registry(None, None);
        let has_bond = results.iter().any(|(n, _)| n.starts_with("bond_"));
        let has_futures = results.iter().any(|(n, _)| n.starts_with("futures_"));
        assert!(has_bond, "bond tools should be enabled");
        assert!(!has_futures, "futures tools should be disabled");
    }

    #[test]
    fn test_search_by_keyword() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let results = service.search_registry(None, Some("quote"));
        assert!(!results.is_empty(), "searching 'quote' should find tools");
        for (name, desc) in &results {
            assert!(
                name.to_lowercase().contains("quote") || desc.to_lowercase().contains("quote"),
                "result should match query: {}",
                name
            );
        }
    }

    #[test]
    fn test_search_by_category() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let results = service.search_registry(Some("stock"), None);
        assert!(!results.is_empty(), "stock category should have tools");
        for (name, _) in &results {
            assert!(
                name.starts_with("stock_")
                    || name.starts_with("a_share_")
                    || name.starts_with("hk_")
                    || name.starts_with("us_")
                    || name.starts_with("get_us_"),
                "all results should be stock tools, got: {}",
                name
            );
        }
    }

    #[test]
    fn test_registry_has_minimum_tools() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let tools = service.list_tools_sync();
        // Subtract 2 for meta-tools (tools/search and tools/call)
        let registry_count = tools.len() - 2;
        assert!(
            registry_count >= 1400,
            "Expected at least 1400 registry tools, got {}",
            registry_count
        );
    }
}
