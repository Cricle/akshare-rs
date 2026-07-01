//! Build script that generates the MCP tool registry from tool definitions.
//!
//! The tool data is extracted from the existing `#[tool]` methods in the codebase
//! and stored in `tools.txt`. When adding a new akshare function:
//! 1. Add a `pub async fn` with a doc comment to `AkShareClient`
//! 2. Add a line to `tools.txt` with the tool metadata

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=tools.txt");

    let tools = load_tools();
    let code = generate_registry(&tools);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("tool_registry.rs"), code).unwrap();

    // Generate parameter validation tests
    let param_tests = generate_param_tests(&tools);
    let tests_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests");
    fs::create_dir_all(&tests_dir).unwrap();
    fs::write(tests_dir.join("param_validation.rs"), param_tests).unwrap();
}

struct ToolDef {
    name: String,
    param_type: String,
    category: String,
    description: String,
    client_call: String,
}

/// Validate that client call expressions are well-formed.
fn validate_client_call(call: &str, name: &str) -> Result<(), String> {
    // Check that call starts with self.client.
    if !call.starts_with("self.client.") {
        return Err(format!(
            "Tool '{}': client call must start with 'self.client.', got '{}'",
            name, call
        ));
    }

    // Check for balanced parentheses
    let open_count = call.chars().filter(|&c| c == '(').count();
    let close_count = call.chars().filter(|&c| c == ')').count();
    if open_count != close_count {
        return Err(format!(
            "Tool '{}': unbalanced parentheses in client call '{}'",
            name, call
        ));
    }

    // Check for balanced quotes (double quotes)
    let quote_count = call.chars().filter(|&c| c == '"').count();
    if quote_count % 2 != 0 {
        return Err(format!(
            "Tool '{}': unbalanced quotes in client call '{}'",
            name, call
        ));
    }

    Ok(())
}

/// Load tool definitions from `tools.txt`.
/// Format: TOOL_NAME|PARAM_TYPE|CATEGORY|DESCRIPTION|CLIENT_CALL
fn load_tools() -> Vec<ToolDef> {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(manifest).join("tools.txt");
    let content = fs::read_to_string(&path).unwrap_or_default();

    let tools: Vec<ToolDef> = content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(5, '|').collect();
            if parts.len() < 5 {
                return None;
            }
            Some(ToolDef {
                name: parts[0].trim().to_string(),
                param_type: parts[1].trim().to_string(),
                category: parts[2].trim().to_string(),
                description: parts[3].trim().to_string(),
                client_call: parts[4].trim().to_string(),
            })
        })
        .collect();

    // Validate all client calls
    for tool in &tools {
        if let Err(e) = validate_client_call(&tool.client_call, &tool.name) {
            panic!("tools.txt validation error: {}", e);
        }
    }

    tools
}

/// Generate parameter validation tests for each tool.
fn generate_param_tests(tools: &[ToolDef]) -> String {
    let mut out = String::from(
        "// Auto-generated parameter validation tests — do not edit.\n\
         // These tests verify that each tool's JSON schema is valid.\n\n\
         use akshare_mcp::tools::AkShareMcpService;\n\
         use akshare_mcp::config::ToolsConfig;\n\n\
         fn get_tool_schema(tool_name: &str) -> serde_json::Value {\n\
             let service = AkShareMcpService::new(ToolsConfig::all());\n\
             let tool = service.get_tool(tool_name)\n\
                 .expect(&format!(\"Tool '{}' not found\", tool_name));\n\
             serde_json::to_value(tool.input_schema.clone()).unwrap()\n\
         }\n\n",
    );

    for tool in tools {
        let test_name = format!("test_param_schema_{}", tool.name.replace('-', "_"));
        let ptype = normalize_param_type(&tool.param_type);

        // Skip EmptyParams - no validation needed
        if ptype == "stock::EmptyParams" {
            continue;
        }

        // Get fields from param type
        let fields = param_fields(&ptype);
        if fields.is_empty() {
            continue;
        }

        // Verify that the schema has the expected properties
        out.push_str(&format!(
            "#[test]\n\
             fn {}() {{\n\
                 let schema = get_tool_schema(\"{}\");\n\
                 let properties = schema.get(\"properties\")\n\
                     .and_then(|p| p.as_object())\n\
                     .expect(\"Schema should have properties\");\n\
                 \n\
                 // Verify all expected fields exist\n",
            test_name, tool.name
        ));

        for field in &fields {
            out.push_str(&format!(
                "        assert!(properties.contains_key(\"{}\"),\n\
                \"Schema for '{}' should have field '{}'\");\n",
                field, tool.name, field
            ));
        }

        out.push_str("}\n\n");
    }

    out
}

/// Generate `register_tool!` invocations for all tools.
fn generate_registry(tools: &[ToolDef]) -> String {
    let mut out = String::from(
        "// Auto-generated by build.rs from tools.txt — do not edit this file.\n\
         // To add a tool: add a line to tools.txt.\n\n\
         fn load_registry(m: &mut std::collections::HashMap<&'static str, crate::tools::ToolEntry>) {\n",
    );

    let mut current_cat = "";
    for t in tools {
        if t.category != current_cat {
            out.push_str(&format!("    // ── {} ──\n", t.category));
            current_cat = &t.category;
        }

        let handler = generate_handler(t);
        let ptype = normalize_param_type(&t.param_type);

        out.push_str(&format!(
            "register_tool!(m, \"{}\", \"{}\", \"{}\", {}, {});\n",
            t.name,
            escape_str(&t.description),
            t.category,
            ptype,
            handler,
        ));
    }

    out.push_str("}\n");

    // Add count assertion
    out.push_str(&generate_count_assertion(tools));

    out
}

/// Generate a compile-time assertion that verifies tool count.
fn generate_count_assertion(tools: &[ToolDef]) -> String {
    let count = tools.len();
    format!(
        "\n// Compile-time assertion: verify tool count matches tools.txt\n\
         #[cfg(test)]\n\
         mod tool_count_assertion {{\n\
             #[test]\n\
             fn test_tool_count_matches_tools_txt() {{\n\
                 // This number must match the count in tools.txt\n\
                 let expected = {};\n\
                 let actual = crate::tools::TOOL_REGISTRY.len();\n\
                 assert_eq!(actual, expected, \n\
                     \"Tool count mismatch: tools.txt has {{}} tools but registry has {{}}. \
                      Did you forget to update tools.txt?\", expected, actual);\n\
             }}\n\
         }}\n",
        count
    )
}

/// Convert param type from extraction format to Rust path.
fn normalize_param_type(param: &str) -> String {
    match param {
        "_" | "Empty" | "" => "stock::EmptyParams".to_string(),
        other => other.to_string(),
    }
}

/// Generate the handler closure from the client call expression.
fn generate_handler(t: &ToolDef) -> String {
    let ptype = normalize_param_type(&t.param_type);

    // No-param tools: call with no arguments
    if ptype == "stock::EmptyParams" {
        let call = convert_call(&t.client_call, &[]);
        return format!("|client, _p| Box::pin(async move {{ {} }})", call);
    }

    // Extract field names from param type, prefix unused ones with `_`
    let fields = param_fields(&ptype);
    let call = convert_call(&t.client_call, &fields);
    let field_pattern = format!(
        "{{ {} }}",
        fields
            .iter()
            .map(|f| {
                // Check if field is used as &field (reference) or as bare variable (e.g. limit)
                let as_ref = format!("&{}", f);
                let as_sp = format!(" {}", f);
                let as_comma = format!(",{}", f);
                let as_paren = format!("({}", f);
                if call.contains(&as_ref)
                    || call.contains(&as_sp)
                    || call.contains(&as_comma)
                    || call.contains(&as_paren)
                {
                    f.to_string()
                } else {
                    format!("{}: _", f)
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    );

    format!(
        "|client, p| {{ let {}{} = p; Box::pin(async move {{ {} }}) }}",
        ptype, field_pattern, call
    )
}

/// Get the field names for a param type.
fn param_fields(ptype: &str) -> Vec<&'static str> {
    match ptype {
        // stock
        "stock::SymbolParams" => vec!["symbol"],
        "stock::DateParams" => vec!["date"],
        "stock::LimitParams" => vec!["limit"],
        "stock::CandlesParams" => vec!["symbol", "limit"],
        "stock::StockDailyParams" => vec!["symbol", "start_date", "end_date"],
        "stock::StockHistParams" => vec!["symbol", "period", "start_date", "end_date", "adjust"],
        "stock::ValuationParams" => vec!["symbol", "indicator", "period"],
        "stock::SymbolIndicatorParams" => vec!["symbol", "indicator"],
        "stock::SymbolDateParams" => vec!["symbol", "date"],
        "stock::DateIndicatorParams" => vec!["date", "indicator"],
        "stock::DateMarketParams" => vec!["date", "market"],
        "stock::FundFlowParams" => vec!["symbol", "market", "limit"],
        "stock::FundFlowRankParams" => vec!["indicator", "limit"],
        "stock::SymbolStartYearParams" => vec!["symbol", "start_year"],
        "stock::BoardHistMinParams" => vec!["symbol", "period"],
        "stock::XqInfoParams" => vec!["symbol", "token"],
        "stock::CyqParams" => vec!["symbol", "adjust"],
        "stock::KeywordDateParams" => vec!["keyword", "date"],
        "stock::BillboardSeatsParams" => vec!["symbol", "side", "limit"],
        "stock::TradeCalendarParams" => vec!["exchange", "start_date", "end_date"],
        "stock::DateRangeParams" => vec!["start_date", "end_date"],
        "stock::SymbolDateRangeParams" => vec!["symbol", "start_date", "end_date"],
        "stock::DateRangeLimitParams" => vec!["start_date", "end_date", "limit"],
        "stock::SectorRankParams" => vec!["sector_type", "limit"],
        "stock::SectorCodeParams" => vec!["sector_code", "limit"],
        "stock::BoardCodeParams" => vec!["board_code", "limit"],
        "stock::HkReportParams" => vec!["stock", "symbol", "indicator"],
        "stock::StockSymbolParams" => vec!["stock", "symbol"],
        "stock::GdfxDetailParams" => vec!["date", "indicator", "symbol"],
        "stock::DzjyMrmxParams" => vec!["asset_type", "start_date", "end_date", "limit"],
        "stock::HistoryDividendDetailParams" => vec!["symbol", "indicator", "date"],
        "stock::NoticeReportParams" => vec!["security", "report_type", "begin_date", "end_date"],
        "stock::DisclosureParams" => vec!["symbol", "category", "start_date", "end_date"],
        "stock::SymbolYearParams" => vec!["symbol", "year"],
        "stock::SearchParams" => vec!["query", "market", "limit"],
        "stock::SymbolSectorParams" => vec!["symbol", "sector"],

        // bond
        "bond::BondRateParams" => vec!["start_date"],
        "bond::BondLimitParams" => vec!["limit"],
        "bond::BondYieldParams" => vec!["start_date", "end_date"],
        "bond::BondSymbolParams" => vec!["symbol"],
        "bond::BondSymbolLimitParams" => vec!["symbol", "limit"],
        "bond::BondIndicatorPeriodParams" => vec!["indicator", "period"],
        "bond::BondCloseReturnParams" => vec!["symbol", "period", "start_date", "end_date"],
        "bond::BondSymbolIndicatorParams" => vec!["symbol", "indicator"],
        "bond::BondSymbolPeriodParams" => vec!["symbol", "period"],
        "bond::BondPageParams" => vec!["page"],
        "bond::BondDateParams" => vec!["date"],

        // crypto
        "crypto::CryptoDateParams" => vec!["date"],

        // economy
        "economy::NlpParams" => vec!["question"],
        "economy::CityParam" => vec!["city"],
        "economy::DateCityParam" => vec!["date", "city"],
        "economy::AirHistParam" => vec!["city", "start_date", "end_date"],
        "economy::SymbolParam" => vec!["symbol"],
        "economy::DateParam" => vec!["date"],
        "economy::YearParam" => vec!["year"],
        "economy::SymbolIndicatorParam" => vec!["symbol", "indicator"],
        "economy::SymbolIndexParam" => vec!["symbol", "index"],
        "economy::MigrationAreaParam" => vec!["area", "indicator", "date"],
        "economy::MigrationScaleParam" => vec!["area", "indicator"],
        "economy::HurunParam" => vec!["indicator", "year"],
        "economy::NlpOwnthinkParam" => vec!["word", "indicator"],
        "economy::SymbolDateParam" => vec!["symbol", "date"],

        // forex
        "forex::CurrencyParams" => vec!["symbol", "start_date", "end_date"],
        "forex::ForexEmHistParams" => vec!["symbol", "limit"],
        "forex::CurrencyLatestParams" => vec!["base", "symbols", "api_key"],
        "forex::CurrencyHistoryParams" => vec!["base", "date", "symbols", "api_key"],
        "forex::CurrencyTimeSeriesParams" => {
            vec!["base", "start_date", "end_date", "symbols", "api_key"]
        }
        "forex::CurrencyCurrenciesParams" => vec!["c_type", "api_key"],
        "forex::CurrencyConvertParams" => vec!["from", "to", "amount", "api_key"],
        "forex::CurrencyPairParams" => vec!["pair"],

        // fund
        "fund::FundHistParams" => vec!["symbol", "limit"],
        "fund::FundRankParams" => vec!["symbol", "limit"],

        // futures
        "futures::FuturesCandlesParams" => vec!["symbol", "limit"],
        "futures::FuturesLimitParams" => vec!["limit"],
        "futures::DateParams" => vec!["date"],
        "futures::FuturesDateRangeParams" => vec!["start_date", "end_date"],
        "futures::FuturesDateSymbolParams" => vec!["date", "symbol"],
        "futures::FuturesDateMarketParams" => vec!["date", "market"],
        "futures::FuturesSymbolPeriodParams" => vec!["symbol", "period"],
        "futures::FuturesHistEmParams" => vec!["symbol", "period", "start_date", "end_date"],
        "futures::FuturesSymbolMarketParams" => vec!["symbols", "market"],
        "futures::FuturesSymbolPeriodKlineParams" => vec!["symbol", "period"],
        "futures::FuturesMainSinaDerivativeParams" => vec!["symbol", "start_date", "end_date"],
        "futures::FuturesSymbolIndicatorParams" => vec!["symbol", "indicator"],
        "futures::FuturesCategoryParams" => vec!["category"],
        "futures::FuturesHoldPosParams" => vec!["data_type", "contract", "date"],

        // index
        "index::IndexSymbolParams" => vec!["symbol"],
        "index::IndexCandlesParams" => vec!["symbol", "limit"],
        "index::IndexHistParams" => vec!["symbol", "start_date", "end_date"],
        "index::IndexPeriodParams" => vec!["symbol", "period"],
        "index::IndexHkDailyParams" => vec!["symbol", "internal_id", "limit"],
        "index::IndexZhAHistMinEmParams" => {
            vec!["symbol", "period", "start_date", "end_date", "adjust"]
        }
        "index::SwAnalysisDateParams" => vec!["symbol", "date"],

        // macro_data
        "macro_data::MacroDateParams" => vec!["date"],
        "macro_data::MacroDateRangeParams" => vec!["start_date", "end_date"],
        "macro_data::MacroSymbolParams" => vec!["symbol"],
        "macro_data::MacroNbsNationParams" => vec!["kind", "path", "period"],
        "macro_data::MacroNbsRegionParams" => vec!["kind", "path", "indicator", "period"],
        "macro_data::MacroInterbankParams" => vec!["market", "symbol", "indicator"],

        // option
        "option::OptionSymbolParams" => vec!["symbol"],
        "option::OptionHistParams" => vec!["symbol", "date"],
        "option::OptionChainParams" => vec!["symbol", "limit"],
        "option::OptionSymbolDateParams" => vec!["symbol", "date"],
        "option::OptionSymbolMonthParams" => vec!["symbol", "end_month"],
        "option::OptionSymbolYearParams" => vec!["symbol", "year"],
        "option::OptionSymbolContractParams" => vec!["symbol", "contract"],
        "option::OptionSymbolExchangeParams" => vec!["symbol", "exchange"],
        "option::OptionLhbParams" => vec!["symbol", "indicator", "trade_date"],
        "option::OptionSseCodesParams" => vec!["symbol", "trade_date", "underlying"],
        "option::OptionSseExpireDayParams" => vec!["trade_date", "symbol", "exchange"],

        // news
        "news::NewsDateParams" => vec!["date"],
        "news::NewsSearchParams" => vec!["query", "limit"],
        "news::NewsSymbolParams" => vec!["symbol"],
        "news::NewsQueryTimeoutParams" => vec!["query", "timeout_secs"],
        "news::StockNewsSearchParams" => vec!["symbol", "limit"],
        "news::FinnhubNewsParams" => vec!["symbol", "from", "to", "api_key"],
        "news::MarketauxNewsParams" => vec!["symbol", "api_key", "limit"],
        "news::GdeltNewsSearchParams" => vec![
            "query",
            "base_url",
            "language_hint",
            "time_range",
            "timeout_secs",
        ],

        // futures (additional)
        "futures::FuturesForeignCommodityParams" => vec!["symbols"],

        _ => vec![],
    }
}

/// Convert a client call to a complete handler expression.
/// Transforms `self.client.foo(&symbol)` into `client.foo(&symbol).await.map_err(...).and_then(...)`
fn convert_call(call: &str, fields: &[&str]) -> String {
    let call = call.replace("self.client.", "client.");
    let call = fix_var_names(&call, fields);
    // All client functions are async — add .await and serialize
    format!(
        "{}.await.map_err(|e| e.to_string()).and_then(|v| serde_json::to_value(v).map_err(|e| e.to_string()))",
        call
    )
}

/// Fix variable name mismatches between client call args and param fields.
fn fix_var_names(call: &str, fields: &[&str]) -> String {
    let mut result = call.to_string();
    // Common mismatches: singular vs plural, different naming
    let renames: &[(&str, &str)] = &[
        ("&symbols", "&symbol"),
        ("&codes", "&symbol"),
        ("&ticker", "&symbol"),
    ];
    for (from, to) in renames {
        // Only rename if the target field exists in the param
        let to_field = to.trim_start_matches('&');
        if fields.contains(&to_field) {
            result = result.replace(from, to);
        }
    }
    result
}

fn escape_str(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', " ")
}
