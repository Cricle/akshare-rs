use rmcp::{ClientHandler, ServiceExt};

#[derive(Default, Clone)]
struct TestClient;
impl ClientHandler for TestClient {}

#[tokio::test]
async fn test_all_tools_comprehensive() {
    let service =
        akshare_mcp::tools::AkShareMcpService::new(akshare_mcp::config::ToolsConfig::all());

    // Get all tools
    let tools = service.list_tools_sync();
    let registry_tools: Vec<_> = tools
        .iter()
        .filter(|t| t.name.as_ref() != "tools/search" && t.name.as_ref() != "tools/call")
        .collect();

    // Verify minimum count
    assert!(
        registry_tools.len() >= 1400,
        "Expected at least 1400 tools, got {}",
        registry_tools.len()
    );

    // Verify each tool has required fields
    for tool in &registry_tools {
        // Check name is not empty
        assert!(!tool.name.as_ref().is_empty(), "Tool should have a name");

        // Check description exists
        assert!(
            tool.description.is_some(),
            "Tool '{}' should have a description",
            tool.name
        );

        // Check schema exists (not empty)
        assert!(
            !tool.input_schema.is_empty(),
            "Tool '{}' should have a schema",
            tool.name
        );

        // Verify schema is valid JSON
        let schema_json = serde_json::to_value(&*tool.input_schema);
        assert!(
            schema_json.is_ok(),
            "Tool '{}' schema should be valid JSON",
            tool.name
        );
    }

    // Verify all categories are represented
    let categories: Vec<&str> = registry_tools
        .iter()
        .filter_map(|t| {
            // Extract category from tool name prefix
            let name = t.name.as_ref();
            if name.starts_with("stock_")
                || name.starts_with("a_share_")
                || name.starts_with("hk_")
                || name.starts_with("us_")
            {
                Some("stock")
            } else if name.starts_with("bond_") {
                Some("bond")
            } else if name.starts_with("fund_") {
                Some("fund")
            } else if name.starts_with("futures_") {
                Some("futures")
            } else if name.starts_with("option_") {
                Some("option")
            } else if name.starts_with("forex_") || name.starts_with("currency_") {
                Some("forex")
            } else if name.starts_with("crypto_") {
                Some("crypto")
            } else if name.starts_with("index_") {
                Some("index")
            } else if name.starts_with("macro_") {
                Some("macro_data")
            } else if name.starts_with("news_") {
                Some("news")
            } else if name.starts_with("economy_") {
                Some("economy")
            } else {
                None
            }
        })
        .collect();

    // Verify we have tools in each expected category
    let expected_categories = [
        "stock",
        "bond",
        "fund",
        "futures",
        "option",
        "forex",
        "crypto",
        "index",
        "macro_data",
        "news",
        "economy",
    ];
    for cat in &expected_categories {
        assert!(
            categories.contains(cat),
            "Should have tools in category '{}'",
            cat
        );
    }
}

#[tokio::test]
async fn test_all_tools_have_valid_handlers() {
    let service =
        akshare_mcp::tools::AkShareMcpService::new(akshare_mcp::config::ToolsConfig::all());

    // Get all tools
    let tools = service.list_tools_sync();
    let registry_tools: Vec<_> = tools
        .iter()
        .filter(|t| t.name.as_ref() != "tools/search" && t.name.as_ref() != "tools/call")
        .collect();

    // Verify each tool has a description
    for tool in &registry_tools {
        assert!(
            tool.description.is_some(),
            "Tool '{}' should have a description",
            tool.name
        );
    }

    assert!(
        registry_tools.len() >= 1400,
        "Expected at least 1400 tools, got {}",
        registry_tools.len()
    );
}

fn make_init_request() -> rmcp::model::ClientRequest {
    rmcp::model::ClientRequest::InitializeRequest(rmcp::model::Request::new(
        rmcp::model::InitializeRequestParams::new(
            rmcp::model::ClientCapabilities::default(),
            rmcp::model::Implementation::from_build_env(),
        ),
    ))
}

#[tokio::test]
async fn test_stdio_roundtrip_server_info() {
    let server =
        akshare_mcp::tools::AkShareMcpService::new(akshare_mcp::config::ToolsConfig::all());
    let client = TestClient;

    let (server_transport, client_transport) = tokio::io::duplex(4096);

    let server_handle = tokio::spawn(async move {
        let service = server.serve(server_transport).await.unwrap();
        service.waiting().await.unwrap();
    });

    let client_service = client.serve(client_transport).await.unwrap();

    let info = client_service
        .send_request(make_init_request())
        .await
        .unwrap();

    match info {
        rmcp::model::ServerResult::InitializeResult(result) => {
            assert!(
                result.capabilities.tools.is_some(),
                "server should advertise tool capabilities"
            );
            let instructions = result.instructions.unwrap_or_default();
            assert!(
                instructions.contains("akshare-rs"),
                "instructions should mention akshare-rs, got: {instructions}"
            );
        }
        other => panic!("unexpected result: {other:?}"),
    }

    client_service.cancel().await.unwrap();
    let _ = server_handle.await;
}

#[tokio::test]
async fn test_stdio_list_meta_tools() {
    let server =
        akshare_mcp::tools::AkShareMcpService::new(akshare_mcp::config::ToolsConfig::all());
    let client = TestClient;

    let (server_transport, client_transport) = tokio::io::duplex(4096);

    let server_handle = tokio::spawn(async move {
        let service = server.serve(server_transport).await.unwrap();
        service.waiting().await.unwrap();
    });

    let client_service = client.serve(client_transport).await.unwrap();

    let _ = client_service
        .send_request(make_init_request())
        .await
        .unwrap();

    let tools_result = client_service
        .send_request(rmcp::model::ClientRequest::ListToolsRequest(
            rmcp::model::RequestOptionalParam::default(),
        ))
        .await
        .unwrap();

    match tools_result {
        rmcp::model::ServerResult::ListToolsResult(result) => {
            let tool_names: Vec<&str> = result.tools.iter().map(|t| t.name.as_ref()).collect();

            // Search + Call meta-tools plus all registry tools
            assert!(
                tool_names.contains(&"tools/search"),
                "should have tools/search"
            );
            assert!(tool_names.contains(&"tools/call"), "should have tools/call");
            assert!(
                tool_names.len() > 2,
                "should have registry tools beyond the 2 meta-tools"
            );

            // Both tools must have descriptions
            for tool in &result.tools {
                assert!(
                    tool.description.is_some(),
                    "tool '{}' should have a description",
                    tool.name
                );
            }
        }
        other => panic!("unexpected result: {other:?}"),
    }

    client_service.cancel().await.unwrap();
    let _ = server_handle.await;
}

#[tokio::test]
async fn test_stdio_tool_has_description() {
    let server =
        akshare_mcp::tools::AkShareMcpService::new(akshare_mcp::config::ToolsConfig::all());
    let client = TestClient;

    let (server_transport, client_transport) = tokio::io::duplex(4096);

    let server_handle = tokio::spawn(async move {
        let service = server.serve(server_transport).await.unwrap();
        service.waiting().await.unwrap();
    });

    let client_service = client.serve(client_transport).await.unwrap();

    let _ = client_service
        .send_request(make_init_request())
        .await
        .unwrap();

    let tools_result = client_service
        .send_request(rmcp::model::ClientRequest::ListToolsRequest(
            rmcp::model::RequestOptionalParam::default(),
        ))
        .await
        .unwrap();

    match tools_result {
        rmcp::model::ServerResult::ListToolsResult(result) => {
            for tool in &result.tools {
                assert!(
                    tool.description.is_some(),
                    "tool '{}' should have a description",
                    tool.name
                );
            }
        }
        other => panic!("unexpected result: {other:?}"),
    }

    client_service.cancel().await.unwrap();
    let _ = server_handle.await;
}
