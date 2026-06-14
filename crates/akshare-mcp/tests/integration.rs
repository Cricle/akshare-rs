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
async fn test_stdio_list_all_tools() {
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

            // Stock tools (10)
            assert!(tool_names.contains(&"a_share_quote"));
            assert!(tool_names.contains(&"a_share_candles"));
            assert!(tool_names.contains(&"hk_quote"));
            assert!(tool_names.contains(&"hk_candles"));
            assert!(tool_names.contains(&"us_quote"));
            assert!(tool_names.contains(&"us_candles"));
            assert!(tool_names.contains(&"stock_zh_a_spot_em"));
            assert!(tool_names.contains(&"stock_zh_a_hist"));
            assert!(tool_names.contains(&"stock_board_industry_name_em"));
            assert!(tool_names.contains(&"stock_individual_fund_flow"));

            // Fund tools (5)
            assert!(tool_names.contains(&"fund_etf_hist"));
            assert!(tool_names.contains(&"fund_manager_em"));
            assert!(tool_names.contains(&"fund_etf_spot_em"));
            assert!(tool_names.contains(&"fund_lof_spot_em"));
            assert!(tool_names.contains(&"fund_open_fund_rank_em"));

            // Bond tools (5)
            assert!(tool_names.contains(&"bond_zh_us_rate"));
            assert!(tool_names.contains(&"bond_corporate_yields"));
            assert!(tool_names.contains(&"bond_china_yield"));
            assert!(tool_names.contains(&"bond_spot_deal"));
            assert!(tool_names.contains(&"bond_spot_rates"));

            // Futures tools (5)
            assert!(tool_names.contains(&"futures_spot_prices"));
            assert!(tool_names.contains(&"futures_main_sina"));
            assert!(tool_names.contains(&"futures_daily_cffex"));
            assert!(tool_names.contains(&"futures_daily_shfe"));
            assert!(tool_names.contains(&"futures_shfe_position_rank"));

            // Option tools (4)
            assert!(tool_names.contains(&"option_sse_greeks_sina"));
            assert!(tool_names.contains(&"option_current_day_sse"));
            assert!(tool_names.contains(&"option_hist_czce"));
            assert!(tool_names.contains(&"option_daily_stats_sse"));

            // Forex tools (4)
            assert!(tool_names.contains(&"forex_boc_rates"));
            assert!(tool_names.contains(&"forex_em_rates"));
            assert!(tool_names.contains(&"forex_spot_em"));
            assert!(tool_names.contains(&"currency_boc_sina"));

            // Crypto tools (2)
            assert!(tool_names.contains(&"crypto_bitcoin_cme"));
            assert!(tool_names.contains(&"crypto_bitcoin_hold_report"));

            // Index tools (4)
            assert!(tool_names.contains(&"index_global_candles"));
            assert!(tool_names.contains(&"index_stock_cons"));
            assert!(tool_names.contains(&"index_stock_info"));
            assert!(tool_names.contains(&"stock_zh_index_hist_csindex"));

            // Macro data tools (4)
            assert!(tool_names.contains(&"macro_china_gdp"));
            assert!(tool_names.contains(&"macro_usa_cpi_yoy"));
            assert!(tool_names.contains(&"macro_china_gdp_yearly"));
            assert!(tool_names.contains(&"macro_usa_cpi_monthly"));

            // Economy tools (4)
            assert!(tool_names.contains(&"economy_auto_sales"));
            assert!(tool_names.contains(&"economy_box_office"));
            assert!(tool_names.contains(&"movie_boxoffice_realtime"));
            assert!(tool_names.contains(&"nlp_answer"));

            // News tools (4)
            assert!(tool_names.contains(&"news_cctv"));
            assert!(tool_names.contains(&"news_search"));
            assert!(tool_names.contains(&"news_economic_baidu"));
            assert!(tool_names.contains(&"news_trade_notify_dividend_baidu"));

            assert!(
                tool_names.len() >= 42,
                "should have at least 42 tools, got {}",
                tool_names.len()
            );
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
