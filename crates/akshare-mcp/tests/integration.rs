use rmcp::{ClientHandler, ServiceExt};

#[derive(Default, Clone)]
struct TestClient;
impl ClientHandler for TestClient {}

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

            // Search + Call pattern: only 2 meta-tools exposed via MCP protocol
            assert_eq!(tool_names.len(), 2, "should expose exactly 2 meta-tools");
            assert!(tool_names.contains(&"tools/search"));
            assert!(tool_names.contains(&"tools/call"));

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
