// Auto-generated parameter validation tests — do not edit.
// These tests verify that each tool's JSON schema is valid.

use akshare_mcp::tools::AkShareMcpService;
use akshare_mcp::config::ToolsConfig;

fn get_tool_schema(tool_name: &str) -> serde_json::Value {
let service = AkShareMcpService::new(ToolsConfig::all());
let tool = service.get_tool(tool_name)
.expect(&format!("Tool '{}' not found", tool_name));
serde_json::to_value(tool.input_schema.clone()).unwrap()
}

#[test]
fn test_param_schema_a_share_quote() {
let schema = get_tool_schema("a_share_quote");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'a_share_quote' should have field 'symbol'");
}

#[test]
fn test_param_schema_a_share_candles() {
let schema = get_tool_schema("a_share_candles");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'a_share_candles' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'a_share_candles' should have field 'limit'");
}

#[test]
fn test_param_schema_hk_quote() {
let schema = get_tool_schema("hk_quote");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'hk_quote' should have field 'symbol'");
}

#[test]
fn test_param_schema_hk_candles() {
let schema = get_tool_schema("hk_candles");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'hk_candles' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'hk_candles' should have field 'limit'");
}

#[test]
fn test_param_schema_us_quote() {
let schema = get_tool_schema("us_quote");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'us_quote' should have field 'symbol'");
}

#[test]
fn test_param_schema_us_candles() {
let schema = get_tool_schema("us_candles");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'us_candles' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'us_candles' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_zh_a_hist() {
let schema = get_tool_schema("stock_zh_a_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_hist' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_zh_a_hist' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_a_hist' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_a_hist' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_zh_a_hist' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_hk_hist() {
let schema = get_tool_schema("stock_hk_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_hist' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_hk_hist' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_hk_hist' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_hk_hist' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_hk_hist' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_us_hist() {
let schema = get_tool_schema("stock_us_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_hist' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_us_hist' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_us_hist' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_us_hist' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_us_hist' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_zh_a_hist_min() {
let schema = get_tool_schema("stock_zh_a_hist_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_hist_min' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_zh_a_hist_min' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_a_hist_min' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_a_hist_min' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_zh_a_hist_min' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_hk_hist_min() {
let schema = get_tool_schema("stock_hk_hist_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_hist_min' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_hk_hist_min' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_hk_hist_min' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_hk_hist_min' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_hk_hist_min' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_us_hist_min() {
let schema = get_tool_schema("stock_us_hist_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_hist_min' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_us_hist_min' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_us_hist_min' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_us_hist_min' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_us_hist_min' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_zh_a_daily() {
let schema = get_tool_schema("stock_zh_a_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_daily' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_zh_a_daily' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_a_daily' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_a_daily' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_zh_a_daily' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_hk_daily() {
let schema = get_tool_schema("stock_hk_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_daily' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_hk_daily' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_hk_daily' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_hk_daily' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_hk_daily' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_us_daily() {
let schema = get_tool_schema("stock_us_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_daily' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_us_daily' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_us_daily' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_hk_hot_rank_latest() {
let schema = get_tool_schema("stock_hk_hot_rank_latest");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_hot_rank_latest' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_hot_rank_detail() {
let schema = get_tool_schema("stock_hk_hot_rank_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_hot_rank_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_index_daily_em() {
let schema = get_tool_schema("stock_hk_index_daily_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_index_daily_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_financial_indicator() {
let schema = get_tool_schema("stock_hk_financial_indicator");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_financial_indicator' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_dividend_payout() {
let schema = get_tool_schema("stock_hk_dividend_payout");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_dividend_payout' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_scale_comparison() {
let schema = get_tool_schema("stock_hk_scale_comparison");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_scale_comparison' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_security_profile() {
let schema = get_tool_schema("stock_hk_security_profile");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_security_profile' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_company_profile() {
let schema = get_tool_schema("stock_hk_company_profile");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_company_profile' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_growth_comparison() {
let schema = get_tool_schema("stock_hk_growth_comparison");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_growth_comparison' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_valuation_comparison() {
let schema = get_tool_schema("stock_hk_valuation_comparison");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_valuation_comparison' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_index_daily_sina() {
let schema = get_tool_schema("stock_hk_index_daily_sina");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_index_daily_sina' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_hot_rank_detail_realtime() {
let schema = get_tool_schema("stock_hk_hot_rank_detail_realtime");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_hot_rank_detail_realtime' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_fhpx_detail() {
let schema = get_tool_schema("stock_hk_fhpx_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_fhpx_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hk_indicator_eniu() {
let schema = get_tool_schema("stock_hk_indicator_eniu");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_indicator_eniu' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_hot_rank_latest_em() {
let schema = get_tool_schema("stock_zh_a_hot_rank_latest_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_hot_rank_latest_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_hot_rank_detail_em() {
let schema = get_tool_schema("stock_zh_a_hot_rank_detail_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_hot_rank_detail_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_hot_rank_detail_realtime_em() {
let schema = get_tool_schema("stock_zh_a_hot_rank_detail_realtime_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_hot_rank_detail_realtime_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_index_daily_em() {
let schema = get_tool_schema("stock_zh_a_index_daily_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_index_daily_em' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_a_index_daily_em' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_a_index_daily_em' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_zh_a_scale_comparison_em() {
let schema = get_tool_schema("stock_zh_a_scale_comparison_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_scale_comparison_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_growth_comparison_em() {
let schema = get_tool_schema("stock_zh_a_growth_comparison_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_growth_comparison_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_valuation_comparison_em() {
let schema = get_tool_schema("stock_zh_a_valuation_comparison_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_valuation_comparison_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_fhps_detail_ths() {
let schema = get_tool_schema("stock_zh_a_fhps_detail_ths");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_fhps_detail_ths' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_financial_indicator() {
let schema = get_tool_schema("stock_zh_a_financial_indicator");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_financial_indicator' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_dividend_payout() {
let schema = get_tool_schema("stock_zh_a_dividend_payout");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_dividend_payout' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_profile_cninfo() {
let schema = get_tool_schema("stock_zh_a_profile_cninfo");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_profile_cninfo' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_dividend_cninfo() {
let schema = get_tool_schema("stock_zh_a_dividend_cninfo");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_dividend_cninfo' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_fhps_detail_em() {
let schema = get_tool_schema("stock_zh_a_fhps_detail_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_fhps_detail_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_dupont_comparison_em() {
let schema = get_tool_schema("stock_zh_a_dupont_comparison_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_dupont_comparison_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_hot_rank_relate_em() {
let schema = get_tool_schema("stock_zh_a_hot_rank_relate_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_hot_rank_relate_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_hot_keyword_em() {
let schema = get_tool_schema("stock_zh_a_hot_keyword_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_hot_keyword_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_company_profile_em() {
let schema = get_tool_schema("stock_us_company_profile_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_company_profile_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_key_stats_em() {
let schema = get_tool_schema("stock_us_key_stats_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_key_stats_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_news_em() {
let schema = get_tool_schema("stock_us_news_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_news_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_financial_indicator() {
let schema = get_tool_schema("stock_us_financial_indicator");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_financial_indicator' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_dividend_payout() {
let schema = get_tool_schema("stock_us_dividend_payout");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_dividend_payout' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_hot_rank_latest() {
let schema = get_tool_schema("stock_us_hot_rank_latest");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_hot_rank_latest' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_hot_rank_detail() {
let schema = get_tool_schema("stock_us_hot_rank_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_hot_rank_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_hot_rank_detail_realtime() {
let schema = get_tool_schema("stock_us_hot_rank_detail_realtime");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_hot_rank_detail_realtime' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_growth_comparison() {
let schema = get_tool_schema("stock_us_growth_comparison");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_growth_comparison' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_valuation_comparison() {
let schema = get_tool_schema("stock_us_valuation_comparison");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_valuation_comparison' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_scale_comparison() {
let schema = get_tool_schema("stock_us_scale_comparison");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_scale_comparison' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_index_daily_em() {
let schema = get_tool_schema("stock_us_index_daily_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_index_daily_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_hot_keyword() {
let schema = get_tool_schema("stock_us_hot_keyword");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_hot_keyword' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_etf_hist() {
let schema = get_tool_schema("fund_etf_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_etf_hist' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'fund_etf_hist' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_open_fund_rank() {
let schema = get_tool_schema("fund_open_fund_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_open_fund_rank' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'fund_open_fund_rank' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_zh_us_rate() {
let schema = get_tool_schema("bond_zh_us_rate");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'bond_zh_us_rate' should have field 'start_date'");
}

#[test]
fn test_param_schema_bond_corporate_yields() {
let schema = get_tool_schema("bond_corporate_yields");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'bond_corporate_yields' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_china_yield() {
let schema = get_tool_schema("bond_china_yield");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'bond_china_yield' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'bond_china_yield' should have field 'end_date'");
}

#[test]
fn test_param_schema_bond_spot_rates() {
let schema = get_tool_schema("bond_spot_rates");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'bond_spot_rates' should have field 'limit'");
}

#[test]
fn test_param_schema_futures_spot_prices() {
let schema = get_tool_schema("futures_spot_prices");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'futures_spot_prices' should have field 'limit'");
}

#[test]
fn test_param_schema_futures_main() {
let schema = get_tool_schema("futures_main");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_main' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'futures_main' should have field 'limit'");
}

#[test]
fn test_param_schema_futures_daily_cffex() {
let schema = get_tool_schema("futures_daily_cffex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_daily_cffex' should have field 'date'");
}

#[test]
fn test_param_schema_futures_daily_shfe() {
let schema = get_tool_schema("futures_daily_shfe");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_daily_shfe' should have field 'date'");
}

#[test]
fn test_param_schema_futures_shfe_position_rank() {
let schema = get_tool_schema("futures_shfe_position_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_shfe_position_rank' should have field 'date'");
}

#[test]
fn test_param_schema_option_sse_greeks() {
let schema = get_tool_schema("option_sse_greeks");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_sse_greeks' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_hist_czce() {
let schema = get_tool_schema("option_hist_czce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_hist_czce' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'option_hist_czce' should have field 'date'");
}

#[test]
fn test_param_schema_option_daily_stats_sse() {
let schema = get_tool_schema("option_daily_stats_sse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'option_daily_stats_sse' should have field 'date'");
}

#[test]
fn test_param_schema_currency_boc() {
let schema = get_tool_schema("currency_boc");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'currency_boc' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'currency_boc' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'currency_boc' should have field 'end_date'");
}

#[test]
fn test_param_schema_crypto_bitcoin_cme() {
let schema = get_tool_schema("crypto_bitcoin_cme");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'crypto_bitcoin_cme' should have field 'date'");
}

#[test]
fn test_param_schema_index_global_candles() {
let schema = get_tool_schema("index_global_candles");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_global_candles' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'index_global_candles' should have field 'limit'");
}

#[test]
fn test_param_schema_index_stock_cons() {
let schema = get_tool_schema("index_stock_cons");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_stock_cons' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_index_hist_csindex() {
let schema = get_tool_schema("stock_zh_index_hist_csindex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_index_hist_csindex' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_index_hist_csindex' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_index_hist_csindex' should have field 'end_date'");
}

#[test]
fn test_param_schema_index_us_stock() {
let schema = get_tool_schema("index_us_stock");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_us_stock' should have field 'symbol'");
}

#[test]
fn test_param_schema_nlp_answer() {
let schema = get_tool_schema("nlp_answer");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("question"),
"Schema for 'nlp_answer' should have field 'question'");
}

#[test]
fn test_param_schema_news_cctv() {
let schema = get_tool_schema("news_cctv");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'news_cctv' should have field 'date'");
}

#[test]
fn test_param_schema_news_search() {
let schema = get_tool_schema("news_search");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("query"),
"Schema for 'news_search' should have field 'query'");
        assert!(properties.contains_key("limit"),
"Schema for 'news_search' should have field 'limit'");
}

#[test]
fn test_param_schema_news_economic() {
let schema = get_tool_schema("news_economic");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'news_economic' should have field 'symbol'");
}

#[test]
fn test_param_schema_news_trade_notify_dividend() {
let schema = get_tool_schema("news_trade_notify_dividend");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'news_trade_notify_dividend' should have field 'date'");
}

#[test]
fn test_param_schema_bond_sh_buy_back() {
let schema = get_tool_schema("bond_sh_buy_back");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'bond_sh_buy_back' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_sz_buy_back() {
let schema = get_tool_schema("bond_sz_buy_back");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'bond_sz_buy_back' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_buy_back_hist() {
let schema = get_tool_schema("bond_buy_back_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_buy_back_hist' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'bond_buy_back_hist' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_cb_profile() {
let schema = get_tool_schema("bond_cb_profile");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_cb_profile' should have field 'symbol'");
}

#[test]
fn test_param_schema_bond_cb_summary() {
let schema = get_tool_schema("bond_cb_summary");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_cb_summary' should have field 'symbol'");
}

#[test]
fn test_param_schema_bond_index_general_cbond() {
let schema = get_tool_schema("bond_index_general_cbond");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("indicator"),
"Schema for 'bond_index_general_cbond' should have field 'indicator'");
        assert!(properties.contains_key("period"),
"Schema for 'bond_index_general_cbond' should have field 'period'");
}

#[test]
fn test_param_schema_bond_treasury_index_cbond() {
let schema = get_tool_schema("bond_treasury_index_cbond");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("indicator"),
"Schema for 'bond_treasury_index_cbond' should have field 'indicator'");
        assert!(properties.contains_key("period"),
"Schema for 'bond_treasury_index_cbond' should have field 'period'");
}

#[test]
fn test_param_schema_bond_new_composite_index_cbond() {
let schema = get_tool_schema("bond_new_composite_index_cbond");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("indicator"),
"Schema for 'bond_new_composite_index_cbond' should have field 'indicator'");
        assert!(properties.contains_key("period"),
"Schema for 'bond_new_composite_index_cbond' should have field 'period'");
}

#[test]
fn test_param_schema_bond_composite_index_cbond() {
let schema = get_tool_schema("bond_composite_index_cbond");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("indicator"),
"Schema for 'bond_composite_index_cbond' should have field 'indicator'");
        assert!(properties.contains_key("period"),
"Schema for 'bond_composite_index_cbond' should have field 'period'");
}

#[test]
fn test_param_schema_bond_china_close_return() {
let schema = get_tool_schema("bond_china_close_return");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_china_close_return' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'bond_china_close_return' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'bond_china_close_return' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'bond_china_close_return' should have field 'end_date'");
}

#[test]
fn test_param_schema_macro_china_swap_rate() {
let schema = get_tool_schema("macro_china_swap_rate");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'macro_china_swap_rate' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'macro_china_swap_rate' should have field 'end_date'");
}

#[test]
fn test_param_schema_macro_china_bond_public() {
let schema = get_tool_schema("macro_china_bond_public");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'macro_china_bond_public' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_convertible_list() {
let schema = get_tool_schema("bond_convertible_list");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'bond_convertible_list' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_convertible_hist() {
let schema = get_tool_schema("bond_convertible_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_convertible_hist' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'bond_convertible_hist' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_gb_zh() {
let schema = get_tool_schema("bond_gb_zh");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_gb_zh' should have field 'symbol'");
}

#[test]
fn test_param_schema_bond_gb_us() {
let schema = get_tool_schema("bond_gb_us");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_gb_us' should have field 'symbol'");
}

#[test]
fn test_param_schema_bond_treasure_issue() {
let schema = get_tool_schema("bond_treasure_issue");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'bond_treasure_issue' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'bond_treasure_issue' should have field 'end_date'");
}

#[test]
fn test_param_schema_bond_local_gov_issue() {
let schema = get_tool_schema("bond_local_gov_issue");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'bond_local_gov_issue' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'bond_local_gov_issue' should have field 'end_date'");
}

#[test]
fn test_param_schema_bond_corporate_issue() {
let schema = get_tool_schema("bond_corporate_issue");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'bond_corporate_issue' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'bond_corporate_issue' should have field 'end_date'");
}

#[test]
fn test_param_schema_bond_cov_issue() {
let schema = get_tool_schema("bond_cov_issue");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'bond_cov_issue' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'bond_cov_issue' should have field 'end_date'");
}

#[test]
fn test_param_schema_bond_local_government_issue() {
let schema = get_tool_schema("bond_local_government_issue");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'bond_local_government_issue' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'bond_local_government_issue' should have field 'end_date'");
}

#[test]
fn test_param_schema_bond_debt_nafmii() {
let schema = get_tool_schema("bond_debt_nafmii");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("page"),
"Schema for 'bond_debt_nafmii' should have field 'page'");
}

#[test]
fn test_param_schema_bond_cash_summary() {
let schema = get_tool_schema("bond_cash_summary");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'bond_cash_summary' should have field 'date'");
}

#[test]
fn test_param_schema_bond_deal_summary() {
let schema = get_tool_schema("bond_deal_summary");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'bond_deal_summary' should have field 'date'");
}

#[test]
fn test_param_schema_bond_zh_cov() {
let schema = get_tool_schema("bond_zh_cov");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'bond_zh_cov' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_cov_comparison() {
let schema = get_tool_schema("bond_cov_comparison");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'bond_cov_comparison' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_zh_hs_cov_daily() {
let schema = get_tool_schema("bond_zh_hs_cov_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_zh_hs_cov_daily' should have field 'symbol'");
}

#[test]
fn test_param_schema_bond_zh_hs_cov_min() {
let schema = get_tool_schema("bond_zh_hs_cov_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_zh_hs_cov_min' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'bond_zh_hs_cov_min' should have field 'period'");
}

#[test]
fn test_param_schema_bond_zh_hs_cov_pre_min() {
let schema = get_tool_schema("bond_zh_hs_cov_pre_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_zh_hs_cov_pre_min' should have field 'symbol'");
}

#[test]
fn test_param_schema_bond_zh_hs_cov_spot() {
let schema = get_tool_schema("bond_zh_hs_cov_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_zh_hs_cov_spot' should have field 'symbol'");
}

#[test]
fn test_param_schema_bond_zh_hs_spot() {
let schema = get_tool_schema("bond_zh_hs_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'bond_zh_hs_spot' should have field 'limit'");
}

#[test]
fn test_param_schema_bond_zh_hs_daily() {
let schema = get_tool_schema("bond_zh_hs_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_zh_hs_daily' should have field 'symbol'");
}

#[test]
fn test_param_schema_economy_air_quality() {
let schema = get_tool_schema("economy_air_quality");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("city"),
"Schema for 'economy_air_quality' should have field 'city'");
}

#[test]
fn test_param_schema_air_quality_hist() {
let schema = get_tool_schema("air_quality_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("city"),
"Schema for 'air_quality_hist' should have field 'city'");
        assert!(properties.contains_key("start_date"),
"Schema for 'air_quality_hist' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'air_quality_hist' should have field 'end_date'");
}

#[test]
fn test_param_schema_air_quality_watch_point() {
let schema = get_tool_schema("air_quality_watch_point");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("city"),
"Schema for 'air_quality_watch_point' should have field 'city'");
        assert!(properties.contains_key("start_date"),
"Schema for 'air_quality_watch_point' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'air_quality_watch_point' should have field 'end_date'");
}

#[test]
fn test_param_schema_sunrise_daily() {
let schema = get_tool_schema("sunrise_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'sunrise_daily' should have field 'date'");
        assert!(properties.contains_key("city"),
"Schema for 'sunrise_daily' should have field 'city'");
}

#[test]
fn test_param_schema_sunrise_monthly() {
let schema = get_tool_schema("sunrise_monthly");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'sunrise_monthly' should have field 'date'");
        assert!(properties.contains_key("city"),
"Schema for 'sunrise_monthly' should have field 'city'");
}

#[test]
fn test_param_schema_article_epu_index() {
let schema = get_tool_schema("article_epu_index");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'article_epu_index' should have field 'symbol'");
}

#[test]
fn test_param_schema_fred_md() {
let schema = get_tool_schema("fred_md");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fred_md' should have field 'date'");
}

#[test]
fn test_param_schema_fred_qd() {
let schema = get_tool_schema("fred_qd");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fred_qd' should have field 'date'");
}

#[test]
fn test_param_schema_article_oman_rv() {
let schema = get_tool_schema("article_oman_rv");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'article_oman_rv' should have field 'symbol'");
        assert!(properties.contains_key("index"),
"Schema for 'article_oman_rv' should have field 'index'");
}

#[test]
fn test_param_schema_article_oman_rv_short() {
let schema = get_tool_schema("article_oman_rv_short");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'article_oman_rv_short' should have field 'symbol'");
}

#[test]
fn test_param_schema_article_rlab_rv() {
let schema = get_tool_schema("article_rlab_rv");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'article_rlab_rv' should have field 'symbol'");
}

#[test]
fn test_param_schema_car_market_country_cpca() {
let schema = get_tool_schema("car_market_country_cpca");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'car_market_country_cpca' should have field 'symbol'");
}

#[test]
fn test_param_schema_car_market_segment_cpca() {
let schema = get_tool_schema("car_market_segment_cpca");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'car_market_segment_cpca' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'car_market_segment_cpca' should have field 'indicator'");
}

#[test]
fn test_param_schema_car_market_total_cpca() {
let schema = get_tool_schema("car_market_total_cpca");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'car_market_total_cpca' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'car_market_total_cpca' should have field 'indicator'");
}

#[test]
fn test_param_schema_car_market_man_rank_cpca() {
let schema = get_tool_schema("car_market_man_rank_cpca");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'car_market_man_rank_cpca' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'car_market_man_rank_cpca' should have field 'indicator'");
}

#[test]
fn test_param_schema_car_market_cate_cpca() {
let schema = get_tool_schema("car_market_cate_cpca");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'car_market_cate_cpca' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'car_market_cate_cpca' should have field 'indicator'");
}

#[test]
fn test_param_schema_car_market_fuel_cpca() {
let schema = get_tool_schema("car_market_fuel_cpca");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'car_market_fuel_cpca' should have field 'symbol'");
}

#[test]
fn test_param_schema_car_sale_rank_gasgoo() {
let schema = get_tool_schema("car_sale_rank_gasgoo");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'car_sale_rank_gasgoo' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'car_sale_rank_gasgoo' should have field 'date'");
}

#[test]
fn test_param_schema_migration_area() {
let schema = get_tool_schema("migration_area");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("area"),
"Schema for 'migration_area' should have field 'area'");
        assert!(properties.contains_key("indicator"),
"Schema for 'migration_area' should have field 'indicator'");
        assert!(properties.contains_key("date"),
"Schema for 'migration_area' should have field 'date'");
}

#[test]
fn test_param_schema_migration_scale() {
let schema = get_tool_schema("migration_scale");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("area"),
"Schema for 'migration_scale' should have field 'area'");
        assert!(properties.contains_key("indicator"),
"Schema for 'migration_scale' should have field 'indicator'");
}

#[test]
fn test_param_schema_index_bloomberg_billionaires_hist() {
let schema = get_tool_schema("index_bloomberg_billionaires_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("year"),
"Schema for 'index_bloomberg_billionaires_hist' should have field 'year'");
}

#[test]
fn test_param_schema_forbes_rank() {
let schema = get_tool_schema("forbes_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'forbes_rank' should have field 'symbol'");
}

#[test]
fn test_param_schema_hurun_rank() {
let schema = get_tool_schema("hurun_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("indicator"),
"Schema for 'hurun_rank' should have field 'indicator'");
        assert!(properties.contains_key("year"),
"Schema for 'hurun_rank' should have field 'year'");
}

#[test]
fn test_param_schema_xincaifu_rank() {
let schema = get_tool_schema("xincaifu_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("year"),
"Schema for 'xincaifu_rank' should have field 'year'");
}

#[test]
fn test_param_schema_movie_boxoffice_cinema_daily() {
let schema = get_tool_schema("movie_boxoffice_cinema_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'movie_boxoffice_cinema_daily' should have field 'date'");
}

#[test]
fn test_param_schema_movie_boxoffice_cinema_weekly() {
let schema = get_tool_schema("movie_boxoffice_cinema_weekly");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'movie_boxoffice_cinema_weekly' should have field 'date'");
}

#[test]
fn test_param_schema_movie_boxoffice_daily() {
let schema = get_tool_schema("movie_boxoffice_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'movie_boxoffice_daily' should have field 'date'");
}

#[test]
fn test_param_schema_movie_boxoffice_monthly() {
let schema = get_tool_schema("movie_boxoffice_monthly");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'movie_boxoffice_monthly' should have field 'date'");
}

#[test]
fn test_param_schema_movie_boxoffice_weekly() {
let schema = get_tool_schema("movie_boxoffice_weekly");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'movie_boxoffice_weekly' should have field 'date'");
}

#[test]
fn test_param_schema_movie_boxoffice_yearly() {
let schema = get_tool_schema("movie_boxoffice_yearly");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("year"),
"Schema for 'movie_boxoffice_yearly' should have field 'year'");
}

#[test]
fn test_param_schema_movie_boxoffice_yearly_first_week() {
let schema = get_tool_schema("movie_boxoffice_yearly_first_week");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("year"),
"Schema for 'movie_boxoffice_yearly_first_week' should have field 'year'");
}

#[test]
fn test_param_schema_nlp_ownthink() {
let schema = get_tool_schema("nlp_ownthink");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("word"),
"Schema for 'nlp_ownthink' should have field 'word'");
        assert!(properties.contains_key("indicator"),
"Schema for 'nlp_ownthink' should have field 'indicator'");
}

#[test]
fn test_param_schema_game_hot_rank_taptap() {
let schema = get_tool_schema("game_hot_rank_taptap");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'game_hot_rank_taptap' should have field 'symbol'");
}

#[test]
fn test_param_schema_forex_em_hist() {
let schema = get_tool_schema("forex_em_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'forex_em_hist' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'forex_em_hist' should have field 'limit'");
}

#[test]
fn test_param_schema_currency_latest() {
let schema = get_tool_schema("currency_latest");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("base"),
"Schema for 'currency_latest' should have field 'base'");
        assert!(properties.contains_key("symbols"),
"Schema for 'currency_latest' should have field 'symbols'");
        assert!(properties.contains_key("api_key"),
"Schema for 'currency_latest' should have field 'api_key'");
}

#[test]
fn test_param_schema_currency_history() {
let schema = get_tool_schema("currency_history");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("base"),
"Schema for 'currency_history' should have field 'base'");
        assert!(properties.contains_key("date"),
"Schema for 'currency_history' should have field 'date'");
        assert!(properties.contains_key("symbols"),
"Schema for 'currency_history' should have field 'symbols'");
        assert!(properties.contains_key("api_key"),
"Schema for 'currency_history' should have field 'api_key'");
}

#[test]
fn test_param_schema_currency_time_series() {
let schema = get_tool_schema("currency_time_series");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("base"),
"Schema for 'currency_time_series' should have field 'base'");
        assert!(properties.contains_key("start_date"),
"Schema for 'currency_time_series' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'currency_time_series' should have field 'end_date'");
        assert!(properties.contains_key("symbols"),
"Schema for 'currency_time_series' should have field 'symbols'");
        assert!(properties.contains_key("api_key"),
"Schema for 'currency_time_series' should have field 'api_key'");
}

#[test]
fn test_param_schema_currency_currencies() {
let schema = get_tool_schema("currency_currencies");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("c_type"),
"Schema for 'currency_currencies' should have field 'c_type'");
        assert!(properties.contains_key("api_key"),
"Schema for 'currency_currencies' should have field 'api_key'");
}

#[test]
fn test_param_schema_currency_convert() {
let schema = get_tool_schema("currency_convert");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("from"),
"Schema for 'currency_convert' should have field 'from'");
        assert!(properties.contains_key("to"),
"Schema for 'currency_convert' should have field 'to'");
        assert!(properties.contains_key("amount"),
"Schema for 'currency_convert' should have field 'amount'");
        assert!(properties.contains_key("api_key"),
"Schema for 'currency_convert' should have field 'api_key'");
}

#[test]
fn test_param_schema_fx_quote() {
let schema = get_tool_schema("fx_quote");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("pair"),
"Schema for 'fx_quote' should have field 'pair'");
}

#[test]
fn test_param_schema_fund_announcement_report() {
let schema = get_tool_schema("fund_announcement_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_announcement_report' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_aum_hist() {
let schema = get_tool_schema("fund_aum_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fund_aum_hist' should have field 'date'");
}

#[test]
fn test_param_schema_fund_cf() {
let schema = get_tool_schema("fund_cf");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fund_cf' should have field 'date'");
}

#[test]
fn test_param_schema_fund_purchase() {
let schema = get_tool_schema("fund_purchase");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_purchase' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_etf_fund_daily() {
let schema = get_tool_schema("fund_etf_fund_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_etf_fund_daily' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_etf_scale_sse() {
let schema = get_tool_schema("fund_etf_scale_sse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fund_etf_scale_sse' should have field 'date'");
}

#[test]
fn test_param_schema_fund_etf_spot_ths() {
let schema = get_tool_schema("fund_etf_spot_ths");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fund_etf_spot_ths' should have field 'date'");
}

#[test]
fn test_param_schema_fund_etf_hist_sina() {
let schema = get_tool_schema("fund_etf_hist_sina");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_etf_hist_sina' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_fee() {
let schema = get_tool_schema("fund_fee");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_fee' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_fh() {
let schema = get_tool_schema("fund_fh");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fund_fh' should have field 'date'");
}

#[test]
fn test_param_schema_fund_financial_fund_info() {
let schema = get_tool_schema("fund_financial_fund_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_financial_fund_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_graded() {
let schema = get_tool_schema("fund_graded");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_graded' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_graded_fund_info() {
let schema = get_tool_schema("fund_graded_fund_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_graded_fund_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_info() {
let schema = get_tool_schema("fund_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_info' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_new_found_ths() {
let schema = get_tool_schema("fund_new_found_ths");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_new_found_ths' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_lof_list() {
let schema = get_tool_schema("fund_lof_list");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_lof_list' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_lof_hist() {
let schema = get_tool_schema("fund_lof_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_lof_hist' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'fund_lof_hist' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_lof() {
let schema = get_tool_schema("fund_lof");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_lof' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_money_market() {
let schema = get_tool_schema("fund_money_market");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_money_market' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_money_fund_info() {
let schema = get_tool_schema("fund_money_fund_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_money_fund_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_open_end_daily() {
let schema = get_tool_schema("fund_open_end_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_open_end_daily' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_open_end_nav() {
let schema = get_tool_schema("fund_open_end_nav");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_open_end_nav' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'fund_open_end_nav' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_overview() {
let schema = get_tool_schema("fund_overview");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_overview' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_portfolio_hold() {
let schema = get_tool_schema("fund_portfolio_hold");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_portfolio_hold' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'fund_portfolio_hold' should have field 'date'");
}

#[test]
fn test_param_schema_fund_portfolio_bond_hold() {
let schema = get_tool_schema("fund_portfolio_bond_hold");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_portfolio_bond_hold' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_portfolio_asset_allocation() {
let schema = get_tool_schema("fund_portfolio_asset_allocation");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_portfolio_asset_allocation' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_portfolio_industry_allocation() {
let schema = get_tool_schema("fund_portfolio_industry_allocation");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_portfolio_industry_allocation' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_portfolio_change() {
let schema = get_tool_schema("fund_portfolio_change");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_portfolio_change' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'fund_portfolio_change' should have field 'indicator'");
}

#[test]
fn test_param_schema_fund_position_lg() {
let schema = get_tool_schema("fund_position_lg");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_position_lg' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_position_hist_lg() {
let schema = get_tool_schema("fund_position_hist_lg");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_position_hist_lg' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_rating() {
let schema = get_tool_schema("fund_rating");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'fund_rating' should have field 'limit'");
}

#[test]
fn test_param_schema_fund_rating_sh() {
let schema = get_tool_schema("fund_rating_sh");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fund_rating_sh' should have field 'date'");
}

#[test]
fn test_param_schema_fund_rating_ja() {
let schema = get_tool_schema("fund_rating_ja");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fund_rating_ja' should have field 'date'");
}

#[test]
fn test_param_schema_fund_report() {
let schema = get_tool_schema("fund_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_report' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_report_half_year() {
let schema = get_tool_schema("fund_report_half_year");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_report_half_year' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_report_quarter() {
let schema = get_tool_schema("fund_report_quarter");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_report_quarter' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_report_stock() {
let schema = get_tool_schema("fund_report_stock");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fund_report_stock' should have field 'date'");
}

#[test]
fn test_param_schema_fund_report_industry_allocation() {
let schema = get_tool_schema("fund_report_industry_allocation");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'fund_report_industry_allocation' should have field 'date'");
}

#[test]
fn test_param_schema_fund_xueqiu_info() {
let schema = get_tool_schema("fund_xueqiu_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_xueqiu_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_xueqiu_achievement() {
let schema = get_tool_schema("fund_xueqiu_achievement");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_xueqiu_achievement' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_individual_basic_info_xq() {
let schema = get_tool_schema("fund_individual_basic_info_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_individual_basic_info_xq' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_individual_achievement_xq() {
let schema = get_tool_schema("fund_individual_achievement_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_individual_achievement_xq' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_individual_analysis_xq() {
let schema = get_tool_schema("fund_individual_analysis_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_individual_analysis_xq' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_individual_profit_probability_xq() {
let schema = get_tool_schema("fund_individual_profit_probability_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_individual_profit_probability_xq' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_individual_detail_info_xq() {
let schema = get_tool_schema("fund_individual_detail_info_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_individual_detail_info_xq' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_spot_price() {
let schema = get_tool_schema("futures_spot_price");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_spot_price' should have field 'date'");
}

#[test]
fn test_param_schema_futures_spot_price_daily() {
let schema = get_tool_schema("futures_spot_price_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'futures_spot_price_daily' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'futures_spot_price_daily' should have field 'end_date'");
}

#[test]
fn test_param_schema_futures_spot_price_previous() {
let schema = get_tool_schema("futures_spot_price_previous");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_spot_price_previous' should have field 'date'");
}

#[test]
fn test_param_schema_futures_comex_inventory() {
let schema = get_tool_schema("futures_comex_inventory");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_comex_inventory' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_comm_js() {
let schema = get_tool_schema("futures_comm_js");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_comm_js' should have field 'date'");
}

#[test]
fn test_param_schema_futures_fees_info() {
let schema = get_tool_schema("futures_fees_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_fees_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_comm_info() {
let schema = get_tool_schema("futures_comm_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_comm_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_contract_detail_sina() {
let schema = get_tool_schema("futures_contract_detail_sina");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_contract_detail_sina' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_contract_detail_em() {
let schema = get_tool_schema("futures_contract_detail_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_contract_detail_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_contract_detail() {
let schema = get_tool_schema("futures_contract_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_contract_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_match_main_contract() {
let schema = get_tool_schema("match_main_contract");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'match_main_contract' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_czce_position_rank() {
let schema = get_tool_schema("futures_czce_position_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_czce_position_rank' should have field 'date'");
}

#[test]
fn test_param_schema_futures_cffex_position_rank() {
let schema = get_tool_schema("futures_cffex_position_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_cffex_position_rank' should have field 'date'");
}

#[test]
fn test_param_schema_futures_dce_position_rank() {
let schema = get_tool_schema("futures_dce_position_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_dce_position_rank' should have field 'date'");
}

#[test]
fn test_param_schema_futures_gfex_position_rank() {
let schema = get_tool_schema("futures_gfex_position_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_gfex_position_rank' should have field 'date'");
}

#[test]
fn test_param_schema_futures_dce_position_rank_other() {
let schema = get_tool_schema("futures_dce_position_rank_other");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_dce_position_rank_other' should have field 'date'");
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_dce_position_rank_other' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_hold_pos() {
let schema = get_tool_schema("futures_hold_pos");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("data_type"),
"Schema for 'futures_hold_pos' should have field 'data_type'");
        assert!(properties.contains_key("contract"),
"Schema for 'futures_hold_pos' should have field 'contract'");
        assert!(properties.contains_key("date"),
"Schema for 'futures_hold_pos' should have field 'date'");
}

#[test]
fn test_param_schema_futures_daily_ine() {
let schema = get_tool_schema("futures_daily_ine");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_daily_ine' should have field 'date'");
}

#[test]
fn test_param_schema_futures_daily_dce() {
let schema = get_tool_schema("futures_daily_dce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_daily_dce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_daily_czce() {
let schema = get_tool_schema("futures_daily_czce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_daily_czce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_daily_gfex() {
let schema = get_tool_schema("futures_daily_gfex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_daily_gfex' should have field 'date'");
}

#[test]
fn test_param_schema_get_futures_daily() {
let schema = get_tool_schema("get_futures_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_futures_daily' should have field 'date'");
        assert!(properties.contains_key("market"),
"Schema for 'get_futures_daily' should have field 'market'");
}

#[test]
fn test_param_schema_futures_to_spot_shfe() {
let schema = get_tool_schema("futures_to_spot_shfe");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_to_spot_shfe' should have field 'date'");
}

#[test]
fn test_param_schema_futures_delivery_shfe() {
let schema = get_tool_schema("futures_delivery_shfe");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_delivery_shfe' should have field 'date'");
}

#[test]
fn test_param_schema_futures_delivery_dce() {
let schema = get_tool_schema("futures_delivery_dce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_delivery_dce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_to_spot_dce() {
let schema = get_tool_schema("futures_to_spot_dce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_to_spot_dce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_delivery_match_czce() {
let schema = get_tool_schema("futures_delivery_match_czce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_delivery_match_czce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_delivery_czce() {
let schema = get_tool_schema("futures_delivery_czce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_delivery_czce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_delivery_match_dce() {
let schema = get_tool_schema("futures_delivery_match_dce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_delivery_match_dce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_to_spot_czce() {
let schema = get_tool_schema("futures_to_spot_czce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_to_spot_czce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_contract_info_cffex() {
let schema = get_tool_schema("futures_contract_info_cffex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_contract_info_cffex' should have field 'date'");
}

#[test]
fn test_param_schema_futures_contract_info_czce() {
let schema = get_tool_schema("futures_contract_info_czce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_contract_info_czce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_contract_info_ine() {
let schema = get_tool_schema("futures_contract_info_ine");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_contract_info_ine' should have field 'date'");
}

#[test]
fn test_param_schema_futures_contract_info_shfe() {
let schema = get_tool_schema("futures_contract_info_shfe");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_contract_info_shfe' should have field 'date'");
}

#[test]
fn test_param_schema_futures_hog_core() {
let schema = get_tool_schema("futures_hog_core");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_hog_core' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_hog_cost() {
let schema = get_tool_schema("futures_hog_cost");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_hog_cost' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_hog_supply() {
let schema = get_tool_schema("futures_hog_supply");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_hog_supply' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_main_sina_derivative() {
let schema = get_tool_schema("futures_main_sina_derivative");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_main_sina_derivative' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'futures_main_sina_derivative' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'futures_main_sina_derivative' should have field 'end_date'");
}

#[test]
fn test_param_schema_futures_spot_sys() {
let schema = get_tool_schema("futures_spot_sys");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_spot_sys' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'futures_spot_sys' should have field 'indicator'");
}

#[test]
fn test_param_schema_get_cffex_daily() {
let schema = get_tool_schema("get_cffex_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_cffex_daily' should have field 'date'");
}

#[test]
fn test_param_schema_get_cffex_rank_table() {
let schema = get_tool_schema("get_cffex_rank_table");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_cffex_rank_table' should have field 'date'");
        assert!(properties.contains_key("symbol"),
"Schema for 'get_cffex_rank_table' should have field 'symbol'");
}

#[test]
fn test_param_schema_get_czce_daily() {
let schema = get_tool_schema("get_czce_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_czce_daily' should have field 'date'");
}

#[test]
fn test_param_schema_get_dce_daily() {
let schema = get_tool_schema("get_dce_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_dce_daily' should have field 'date'");
}

#[test]
fn test_param_schema_get_dce_rank_table() {
let schema = get_tool_schema("get_dce_rank_table");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_dce_rank_table' should have field 'date'");
        assert!(properties.contains_key("symbol"),
"Schema for 'get_dce_rank_table' should have field 'symbol'");
}

#[test]
fn test_param_schema_get_gfex_daily() {
let schema = get_tool_schema("get_gfex_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_gfex_daily' should have field 'date'");
}

#[test]
fn test_param_schema_get_ine_daily() {
let schema = get_tool_schema("get_ine_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_ine_daily' should have field 'date'");
}

#[test]
fn test_param_schema_get_shfe_daily() {
let schema = get_tool_schema("get_shfe_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_shfe_daily' should have field 'date'");
}

#[test]
fn test_param_schema_get_shfe_rank_table() {
let schema = get_tool_schema("get_shfe_rank_table");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_shfe_rank_table' should have field 'date'");
        assert!(properties.contains_key("symbol"),
"Schema for 'get_shfe_rank_table' should have field 'symbol'");
}

#[test]
fn test_param_schema_get_rank_table_czce() {
let schema = get_tool_schema("get_rank_table_czce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_rank_table_czce' should have field 'date'");
}

#[test]
fn test_param_schema_get_roll_yield_bar() {
let schema = get_tool_schema("get_roll_yield_bar");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_roll_yield_bar' should have field 'date'");
}

#[test]
fn test_param_schema_get_rank_sum() {
let schema = get_tool_schema("get_rank_sum");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_rank_sum' should have field 'date'");
}

#[test]
fn test_param_schema_get_rank_sum_daily() {
let schema = get_tool_schema("get_rank_sum_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_rank_sum_daily' should have field 'date'");
}

#[test]
fn test_param_schema_get_receipt() {
let schema = get_tool_schema("get_receipt");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_receipt' should have field 'date'");
}

#[test]
fn test_param_schema_futures_foreign_hist() {
let schema = get_tool_schema("futures_foreign_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_foreign_hist' should have field 'symbol'");
}

#[test]
fn test_param_schema_get_qhkc_fund_bs() {
let schema = get_tool_schema("get_qhkc_fund_bs");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_qhkc_fund_bs' should have field 'date'");
}

#[test]
fn test_param_schema_get_qhkc_fund_money_change() {
let schema = get_tool_schema("get_qhkc_fund_money_change");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_qhkc_fund_money_change' should have field 'date'");
}

#[test]
fn test_param_schema_get_qhkc_fund_position() {
let schema = get_tool_schema("get_qhkc_fund_position");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_qhkc_fund_position' should have field 'date'");
}

#[test]
fn test_param_schema_get_qhkc_index() {
let schema = get_tool_schema("get_qhkc_index");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_qhkc_index' should have field 'date'");
}

#[test]
fn test_param_schema_get_qhkc_index_profit_loss() {
let schema = get_tool_schema("get_qhkc_index_profit_loss");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_qhkc_index_profit_loss' should have field 'date'");
}

#[test]
fn test_param_schema_get_qhkc_index_trend() {
let schema = get_tool_schema("get_qhkc_index_trend");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_qhkc_index_trend' should have field 'date'");
}

#[test]
fn test_param_schema_futures_foreign_detail() {
let schema = get_tool_schema("futures_foreign_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_foreign_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_foreign_commodity_realtime() {
let schema = get_tool_schema("futures_foreign_commodity_realtime");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbols"),
"Schema for 'futures_foreign_commodity_realtime' should have field 'symbols'");
}

#[test]
fn test_param_schema_futures_global_hist() {
let schema = get_tool_schema("futures_global_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_global_hist' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_hist() {
let schema = get_tool_schema("futures_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_hist' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'futures_hist' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'futures_hist' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'futures_hist' should have field 'end_date'");
}

#[test]
fn test_param_schema_futures_index_ccidx() {
let schema = get_tool_schema("futures_index_ccidx");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_index_ccidx' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_inventory() {
let schema = get_tool_schema("futures_inventory");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_inventory' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_inventory_99() {
let schema = get_tool_schema("futures_inventory_99");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_inventory_99' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_news_shmet() {
let schema = get_tool_schema("futures_news_shmet");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("category"),
"Schema for 'futures_news_shmet' should have field 'category'");
}

#[test]
fn test_param_schema_get_dce_receipt() {
let schema = get_tool_schema("get_dce_receipt");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_dce_receipt' should have field 'date'");
}

#[test]
fn test_param_schema_get_shfe_receipt() {
let schema = get_tool_schema("get_shfe_receipt");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_shfe_receipt' should have field 'date'");
}

#[test]
fn test_param_schema_get_roll_yield() {
let schema = get_tool_schema("get_roll_yield");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'get_roll_yield' should have field 'date'");
        assert!(properties.contains_key("symbol"),
"Schema for 'get_roll_yield' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_roll_yield_bar() {
let schema = get_tool_schema("futures_roll_yield_bar");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_roll_yield_bar' should have field 'date'");
}

#[test]
fn test_param_schema_futures_rule_gtja() {
let schema = get_tool_schema("futures_rule_gtja");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_rule_gtja' should have field 'date'");
}

#[test]
fn test_param_schema_futures_rule() {
let schema = get_tool_schema("futures_rule");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_rule' should have field 'date'");
}

#[test]
fn test_param_schema_futures_settle_cffex() {
let schema = get_tool_schema("futures_settle_cffex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_settle_cffex' should have field 'date'");
}

#[test]
fn test_param_schema_futures_settle_czce() {
let schema = get_tool_schema("futures_settle_czce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_settle_czce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_settle_shfe() {
let schema = get_tool_schema("futures_settle_shfe");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_settle_shfe' should have field 'date'");
}

#[test]
fn test_param_schema_futures_settle_ine() {
let schema = get_tool_schema("futures_settle_ine");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_settle_ine' should have field 'date'");
}

#[test]
fn test_param_schema_futures_settle_gfex() {
let schema = get_tool_schema("futures_settle_gfex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_settle_gfex' should have field 'date'");
}

#[test]
fn test_param_schema_futures_stock_shfe_js() {
let schema = get_tool_schema("futures_stock_shfe_js");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_stock_shfe_js' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_settle() {
let schema = get_tool_schema("futures_settle");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_settle' should have field 'date'");
        assert!(properties.contains_key("market"),
"Schema for 'futures_settle' should have field 'market'");
}

#[test]
fn test_param_schema_futures_settlement_price_sgx() {
let schema = get_tool_schema("futures_settlement_price_sgx");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_settlement_price_sgx' should have field 'date'");
}

#[test]
fn test_param_schema_futures_zh_realtime() {
let schema = get_tool_schema("futures_zh_realtime");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_zh_realtime' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_zh_spot() {
let schema = get_tool_schema("futures_zh_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbols"),
"Schema for 'futures_zh_spot' should have field 'symbols'");
        assert!(properties.contains_key("market"),
"Schema for 'futures_zh_spot' should have field 'market'");
}

#[test]
fn test_param_schema_futures_zh_minute() {
let schema = get_tool_schema("futures_zh_minute");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_zh_minute' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'futures_zh_minute' should have field 'period'");
}

#[test]
fn test_param_schema_futures_zh_daily() {
let schema = get_tool_schema("futures_zh_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'futures_zh_daily' should have field 'symbol'");
}

#[test]
fn test_param_schema_futures_spot_stock() {
let schema = get_tool_schema("futures_spot_stock");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_spot_stock' should have field 'date'");
}

#[test]
fn test_param_schema_futures_spot_stock_em() {
let schema = get_tool_schema("futures_spot_stock_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("category"),
"Schema for 'futures_spot_stock_em' should have field 'category'");
}

#[test]
fn test_param_schema_futures_warehouse_receipt_czce() {
let schema = get_tool_schema("futures_warehouse_receipt_czce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_warehouse_receipt_czce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_warehouse_receipt_dce() {
let schema = get_tool_schema("futures_warehouse_receipt_dce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_warehouse_receipt_dce' should have field 'date'");
}

#[test]
fn test_param_schema_futures_shfe_warehouse_receipt() {
let schema = get_tool_schema("futures_shfe_warehouse_receipt");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_shfe_warehouse_receipt' should have field 'date'");
}

#[test]
fn test_param_schema_futures_gfex_warehouse_receipt() {
let schema = get_tool_schema("futures_gfex_warehouse_receipt");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_gfex_warehouse_receipt' should have field 'date'");
}

#[test]
fn test_param_schema_futures_hist_daily_cffex() {
let schema = get_tool_schema("futures_hist_daily_cffex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'futures_hist_daily_cffex' should have field 'date'");
}

#[test]
fn test_param_schema_index_a_share_candles() {
let schema = get_tool_schema("index_a_share_candles");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_a_share_candles' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'index_a_share_candles' should have field 'limit'");
}

#[test]
fn test_param_schema_index_stock_zh_spot_em() {
let schema = get_tool_schema("index_stock_zh_spot_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_stock_zh_spot_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_hist_cni() {
let schema = get_tool_schema("index_hist_cni");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_hist_cni' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'index_hist_cni' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'index_hist_cni' should have field 'end_date'");
}

#[test]
fn test_param_schema_index_detail_hist_cni() {
let schema = get_tool_schema("index_detail_hist_cni");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_detail_hist_cni' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'index_detail_hist_cni' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'index_detail_hist_cni' should have field 'end_date'");
}

#[test]
fn test_param_schema_index_stock_cons_sina() {
let schema = get_tool_schema("index_stock_cons_sina");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_stock_cons_sina' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_stock_cons_csindex() {
let schema = get_tool_schema("index_stock_cons_csindex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_stock_cons_csindex' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_stock_cons_weight_csindex() {
let schema = get_tool_schema("index_stock_cons_weight_csindex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_stock_cons_weight_csindex' should have field 'symbol'");
}

#[test]
fn test_param_schema_drewry_wci_index() {
let schema = get_tool_schema("drewry_wci_index");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'drewry_wci_index' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_eri() {
let schema = get_tool_schema("index_eri");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_eri' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_global_hist_em() {
let schema = get_tool_schema("index_global_hist_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_global_hist_em' should have field 'symbol'");
        assert!(properties.contains_key("internal_id"),
"Schema for 'index_global_hist_em' should have field 'internal_id'");
        assert!(properties.contains_key("limit"),
"Schema for 'index_global_hist_em' should have field 'limit'");
}

#[test]
fn test_param_schema_index_global_hist_sina() {
let schema = get_tool_schema("index_global_hist_sina");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_global_hist_sina' should have field 'symbol'");
}

#[test]
fn test_param_schema_hf_sp_500() {
let schema = get_tool_schema("hf_sp_500");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'hf_sp_500' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_kq_fz() {
let schema = get_tool_schema("index_kq_fz");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_kq_fz' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_kq_fashion() {
let schema = get_tool_schema("index_kq_fashion");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_kq_fashion' should have field 'symbol'");
}

#[test]
fn test_param_schema_sw_index_candles() {
let schema = get_tool_schema("sw_index_candles");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'sw_index_candles' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'sw_index_candles' should have field 'limit'");
}

#[test]
fn test_param_schema_sw_index_third_cons() {
let schema = get_tool_schema("sw_index_third_cons");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'sw_index_third_cons' should have field 'symbol'");
}

#[test]
fn test_param_schema_sw_index_third_info() {
let schema = get_tool_schema("sw_index_third_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'sw_index_third_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_hist_fund_sw() {
let schema = get_tool_schema("index_hist_fund_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_hist_fund_sw' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'index_hist_fund_sw' should have field 'period'");
}

#[test]
fn test_param_schema_index_min_sw() {
let schema = get_tool_schema("index_min_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_min_sw' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_component_sw() {
let schema = get_tool_schema("index_component_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_component_sw' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_analysis_daily_sw() {
let schema = get_tool_schema("index_analysis_daily_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_analysis_daily_sw' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'index_analysis_daily_sw' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'index_analysis_daily_sw' should have field 'end_date'");
}

#[test]
fn test_param_schema_index_analysis_weekly_sw() {
let schema = get_tool_schema("index_analysis_weekly_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_analysis_weekly_sw' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'index_analysis_weekly_sw' should have field 'date'");
}

#[test]
fn test_param_schema_index_analysis_monthly_sw() {
let schema = get_tool_schema("index_analysis_monthly_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_analysis_monthly_sw' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'index_analysis_monthly_sw' should have field 'date'");
}

#[test]
fn test_param_schema_index_zh_a_hist_min_em() {
let schema = get_tool_schema("index_zh_a_hist_min_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_zh_a_hist_min_em' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'index_zh_a_hist_min_em' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'index_zh_a_hist_min_em' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'index_zh_a_hist_min_em' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'index_zh_a_hist_min_em' should have field 'adjust'");
}

#[test]
fn test_param_schema_index_zh_a_hist_min() {
let schema = get_tool_schema("index_zh_a_hist_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_zh_a_hist_min' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'index_zh_a_hist_min' should have field 'period'");
}

#[test]
fn test_param_schema_macro_china_nbs_nation() {
let schema = get_tool_schema("macro_china_nbs_nation");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("kind"),
"Schema for 'macro_china_nbs_nation' should have field 'kind'");
        assert!(properties.contains_key("path"),
"Schema for 'macro_china_nbs_nation' should have field 'path'");
        assert!(properties.contains_key("period"),
"Schema for 'macro_china_nbs_nation' should have field 'period'");
}

#[test]
fn test_param_schema_macro_china_nbs_region() {
let schema = get_tool_schema("macro_china_nbs_region");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("kind"),
"Schema for 'macro_china_nbs_region' should have field 'kind'");
        assert!(properties.contains_key("path"),
"Schema for 'macro_china_nbs_region' should have field 'path'");
        assert!(properties.contains_key("indicator"),
"Schema for 'macro_china_nbs_region' should have field 'indicator'");
        assert!(properties.contains_key("period"),
"Schema for 'macro_china_nbs_region' should have field 'period'");
}

#[test]
fn test_param_schema_macro_fx_sentiment() {
let schema = get_tool_schema("macro_fx_sentiment");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'macro_fx_sentiment' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'macro_fx_sentiment' should have field 'end_date'");
}

#[test]
fn test_param_schema_macro_info_ws() {
let schema = get_tool_schema("macro_info_ws");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'macro_info_ws' should have field 'date'");
}

#[test]
fn test_param_schema_repo_rate_query() {
let schema = get_tool_schema("repo_rate_query");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'repo_rate_query' should have field 'symbol'");
}

#[test]
fn test_param_schema_repo_rate_hist() {
let schema = get_tool_schema("repo_rate_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'repo_rate_hist' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'repo_rate_hist' should have field 'end_date'");
}

#[test]
fn test_param_schema_rate_interbank() {
let schema = get_tool_schema("rate_interbank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("market"),
"Schema for 'rate_interbank' should have field 'market'");
        assert!(properties.contains_key("symbol"),
"Schema for 'rate_interbank' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'rate_interbank' should have field 'indicator'");
}

#[test]
fn test_param_schema_news_report_time() {
let schema = get_tool_schema("news_report_time");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'news_report_time' should have field 'symbol'");
}

#[test]
fn test_param_schema_baidu_news_search() {
let schema = get_tool_schema("baidu_news_search");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("query"),
"Schema for 'baidu_news_search' should have field 'query'");
        assert!(properties.contains_key("timeout_secs"),
"Schema for 'baidu_news_search' should have field 'timeout_secs'");
}

#[test]
fn test_param_schema_sogou_news_search() {
let schema = get_tool_schema("sogou_news_search");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("query"),
"Schema for 'sogou_news_search' should have field 'query'");
        assert!(properties.contains_key("timeout_secs"),
"Schema for 'sogou_news_search' should have field 'timeout_secs'");
}

#[test]
fn test_param_schema_google_news() {
let schema = get_tool_schema("google_news");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("query"),
"Schema for 'google_news' should have field 'query'");
        assert!(properties.contains_key("timeout_secs"),
"Schema for 'google_news' should have field 'timeout_secs'");
}

#[test]
fn test_param_schema_finnhub_company_news() {
let schema = get_tool_schema("finnhub_company_news");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'finnhub_company_news' should have field 'symbol'");
        assert!(properties.contains_key("from"),
"Schema for 'finnhub_company_news' should have field 'from'");
        assert!(properties.contains_key("to"),
"Schema for 'finnhub_company_news' should have field 'to'");
        assert!(properties.contains_key("api_key"),
"Schema for 'finnhub_company_news' should have field 'api_key'");
}

#[test]
fn test_param_schema_marketaux_news() {
let schema = get_tool_schema("marketaux_news");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'marketaux_news' should have field 'symbol'");
        assert!(properties.contains_key("api_key"),
"Schema for 'marketaux_news' should have field 'api_key'");
        assert!(properties.contains_key("limit"),
"Schema for 'marketaux_news' should have field 'limit'");
}

#[test]
fn test_param_schema_seeking_alpha_news() {
let schema = get_tool_schema("seeking_alpha_news");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'seeking_alpha_news' should have field 'symbol'");
}

#[test]
fn test_param_schema_gdelt_news_search_owned() {
let schema = get_tool_schema("gdelt_news_search_owned");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("query"),
"Schema for 'gdelt_news_search_owned' should have field 'query'");
        assert!(properties.contains_key("base_url"),
"Schema for 'gdelt_news_search_owned' should have field 'base_url'");
        assert!(properties.contains_key("language_hint"),
"Schema for 'gdelt_news_search_owned' should have field 'language_hint'");
        assert!(properties.contains_key("time_range"),
"Schema for 'gdelt_news_search_owned' should have field 'time_range'");
        assert!(properties.contains_key("timeout_secs"),
"Schema for 'gdelt_news_search_owned' should have field 'timeout_secs'");
}

#[test]
fn test_param_schema_option_daily_stats_szse() {
let schema = get_tool_schema("option_daily_stats_szse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'option_daily_stats_szse' should have field 'date'");
}

#[test]
fn test_param_schema_option_risk_indicator() {
let schema = get_tool_schema("option_risk_indicator");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'option_risk_indicator' should have field 'date'");
}

#[test]
fn test_param_schema_option_finance_sse_underlying() {
let schema = get_tool_schema("option_finance_sse_underlying");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_finance_sse_underlying' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_finance_board() {
let schema = get_tool_schema("option_finance_board");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_finance_board' should have field 'symbol'");
        assert!(properties.contains_key("end_month"),
"Schema for 'option_finance_board' should have field 'end_month'");
}

#[test]
fn test_param_schema_option_lhb() {
let schema = get_tool_schema("option_lhb");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_lhb' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'option_lhb' should have field 'indicator'");
        assert!(properties.contains_key("trade_date"),
"Schema for 'option_lhb' should have field 'trade_date'");
}

#[test]
fn test_param_schema_option_cffex_sz50_spot() {
let schema = get_tool_schema("option_cffex_sz50_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_cffex_sz50_spot' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_cffex_hs300_spot() {
let schema = get_tool_schema("option_cffex_hs300_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_cffex_hs300_spot' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_cffex_zz1000_spot() {
let schema = get_tool_schema("option_cffex_zz1000_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_cffex_zz1000_spot' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_cffex_sz50_daily() {
let schema = get_tool_schema("option_cffex_sz50_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_cffex_sz50_daily' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_cffex_hs300_daily() {
let schema = get_tool_schema("option_cffex_hs300_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_cffex_hs300_daily' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_cffex_zz1000_daily() {
let schema = get_tool_schema("option_cffex_zz1000_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_cffex_zz1000_daily' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_commodity_contract() {
let schema = get_tool_schema("option_commodity_contract");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_commodity_contract' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_commodity_contract_table() {
let schema = get_tool_schema("option_commodity_contract_table");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_commodity_contract_table' should have field 'symbol'");
        assert!(properties.contains_key("contract"),
"Schema for 'option_commodity_contract_table' should have field 'contract'");
}

#[test]
fn test_param_schema_option_commodity_hist() {
let schema = get_tool_schema("option_commodity_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_commodity_hist' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_hist_yearly_czce() {
let schema = get_tool_schema("option_hist_yearly_czce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_hist_yearly_czce' should have field 'symbol'");
        assert!(properties.contains_key("year"),
"Schema for 'option_hist_yearly_czce' should have field 'year'");
}

#[test]
fn test_param_schema_option_comm_info() {
let schema = get_tool_schema("option_comm_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_comm_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_sse_list() {
let schema = get_tool_schema("option_sse_list");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_sse_list' should have field 'symbol'");
        assert!(properties.contains_key("exchange"),
"Schema for 'option_sse_list' should have field 'exchange'");
}

#[test]
fn test_param_schema_option_sse_expire_day() {
let schema = get_tool_schema("option_sse_expire_day");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("trade_date"),
"Schema for 'option_sse_expire_day' should have field 'trade_date'");
        assert!(properties.contains_key("symbol"),
"Schema for 'option_sse_expire_day' should have field 'symbol'");
        assert!(properties.contains_key("exchange"),
"Schema for 'option_sse_expire_day' should have field 'exchange'");
}

#[test]
fn test_param_schema_option_sse_codes() {
let schema = get_tool_schema("option_sse_codes");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_sse_codes' should have field 'symbol'");
        assert!(properties.contains_key("trade_date"),
"Schema for 'option_sse_codes' should have field 'trade_date'");
        assert!(properties.contains_key("underlying"),
"Schema for 'option_sse_codes' should have field 'underlying'");
}

#[test]
fn test_param_schema_option_sse_spot_price() {
let schema = get_tool_schema("option_sse_spot_price");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_sse_spot_price' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_sse_underlying_spot_price() {
let schema = get_tool_schema("option_sse_underlying_spot_price");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_sse_underlying_spot_price' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_sse_minute() {
let schema = get_tool_schema("option_sse_minute");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_sse_minute' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_sse_daily() {
let schema = get_tool_schema("option_sse_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_sse_daily' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_finance_minute() {
let schema = get_tool_schema("option_finance_minute");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_finance_minute' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_chain() {
let schema = get_tool_schema("option_chain");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_chain' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'option_chain' should have field 'limit'");
}

#[test]
fn test_param_schema_option_minute() {
let schema = get_tool_schema("option_minute");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_minute' should have field 'symbol'");
}

#[test]
fn test_param_schema_option_hist_dce() {
let schema = get_tool_schema("option_hist_dce");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_hist_dce' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'option_hist_dce' should have field 'date'");
}

#[test]
fn test_param_schema_option_hist_shfe() {
let schema = get_tool_schema("option_hist_shfe");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_hist_shfe' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'option_hist_shfe' should have field 'date'");
}

#[test]
fn test_param_schema_option_vol_shfe() {
let schema = get_tool_schema("option_vol_shfe");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_vol_shfe' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'option_vol_shfe' should have field 'date'");
}

#[test]
fn test_param_schema_option_hist_gfex() {
let schema = get_tool_schema("option_hist_gfex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_hist_gfex' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'option_hist_gfex' should have field 'date'");
}

#[test]
fn test_param_schema_option_vol_gfex() {
let schema = get_tool_schema("option_vol_gfex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_vol_gfex' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'option_vol_gfex' should have field 'date'");
}

#[test]
fn test_param_schema_option_margin() {
let schema = get_tool_schema("option_margin");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'option_margin' should have field 'symbol'");
}

#[test]
fn test_param_schema_stooq_candles() {
let schema = get_tool_schema("stooq_candles");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stooq_candles' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'stooq_candles' should have field 'limit'");
}

#[test]
fn test_param_schema_sina_a_share_realtime() {
let schema = get_tool_schema("sina_a_share_realtime");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'sina_a_share_realtime' should have field 'symbol'");
}

#[test]
fn test_param_schema_sina_us_daily() {
let schema = get_tool_schema("sina_us_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'sina_us_daily' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'sina_us_daily' should have field 'limit'");
}

#[test]
fn test_param_schema_index_detail_cni() {
let schema = get_tool_schema("index_detail_cni");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_detail_cni' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_realtime_sw() {
let schema = get_tool_schema("index_realtime_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_realtime_sw' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_analysis_week_month_sw() {
let schema = get_tool_schema("index_analysis_week_month_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_analysis_week_month_sw' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_realtime_fund_sw() {
let schema = get_tool_schema("index_realtime_fund_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_realtime_fund_sw' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_yw() {
let schema = get_tool_schema("index_yw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_yw' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_index_daily() {
let schema = get_tool_schema("stock_zh_index_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_index_daily' should have field 'symbol'");
}

#[test]
fn test_param_schema_spot_goods() {
let schema = get_tool_schema("spot_goods");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'spot_goods' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_price_cflp() {
let schema = get_tool_schema("index_price_cflp");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_price_cflp' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_volume_cflp() {
let schema = get_tool_schema("index_volume_cflp");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_volume_cflp' should have field 'symbol'");
}

#[test]
fn test_param_schema_spot_quotations_sge() {
let schema = get_tool_schema("spot_quotations_sge");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'spot_quotations_sge' should have field 'symbol'");
}

#[test]
fn test_param_schema_spot_hist_sge() {
let schema = get_tool_schema("spot_hist_sge");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'spot_hist_sge' should have field 'symbol'");
}

#[test]
fn test_param_schema_spot_price_qh() {
let schema = get_tool_schema("spot_price_qh");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'spot_price_qh' should have field 'symbol'");
}

#[test]
fn test_param_schema_fx_pair_quote() {
let schema = get_tool_schema("fx_pair_quote");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("pair"),
"Schema for 'fx_pair_quote' should have field 'pair'");
}

#[test]
fn test_param_schema_bank_fjcf_table_detail() {
let schema = get_tool_schema("bank_fjcf_table_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bank_fjcf_table_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_reits_list() {
let schema = get_tool_schema("reits_list");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'reits_list' should have field 'limit'");
}

#[test]
fn test_param_schema_reits_hist() {
let schema = get_tool_schema("reits_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'reits_hist' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'reits_hist' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_individual_spot_xq() {
let schema = get_tool_schema("stock_individual_spot_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_individual_spot_xq' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_js_weibo_report() {
let schema = get_tool_schema("stock_js_weibo_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_js_weibo_report' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_js_weibo_report' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_sector_spot() {
let schema = get_tool_schema("stock_sector_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_sector_spot' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_sector_spot' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_hsgt_kamt_flow() {
let schema = get_tool_schema("stock_hsgt_kamt_flow");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_hsgt_kamt_flow' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_board_concept_spot() {
let schema = get_tool_schema("stock_board_concept_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_concept_spot' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_board_industry_spot() {
let schema = get_tool_schema("stock_board_industry_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_industry_spot' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_board_change() {
let schema = get_tool_schema("stock_board_change");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_change' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_board_concept_info() {
let schema = get_tool_schema("stock_board_concept_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_concept_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_board_industry_info() {
let schema = get_tool_schema("stock_board_industry_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_industry_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zyjs() {
let schema = get_tool_schema("stock_zyjs");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zyjs' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_bid_ask() {
let schema = get_tool_schema("stock_bid_ask");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_bid_ask' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_intraday_em() {
let schema = get_tool_schema("stock_intraday_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_intraday_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_individual_info() {
let schema = get_tool_schema("stock_individual_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_individual_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_info_by_secid() {
let schema = get_tool_schema("stock_info_by_secid");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_info_by_secid' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_index_value_csindex() {
let schema = get_tool_schema("stock_zh_index_value_csindex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_index_value_csindex' should have field 'symbol'");
}

#[test]
fn test_param_schema_a_share_announcement_detail() {
let schema = get_tool_schema("a_share_announcement_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'a_share_announcement_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_famous_spot() {
let schema = get_tool_schema("stock_us_famous_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_famous_spot' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_us_index_daily_sina() {
let schema = get_tool_schema("stock_us_index_daily_sina");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_index_daily_sina' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_gdfx_holding_change() {
let schema = get_tool_schema("stock_gdfx_holding_change");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdfx_holding_change' should have field 'date'");
}

#[test]
fn test_param_schema_stock_gdfx_top_10() {
let schema = get_tool_schema("stock_gdfx_top_10");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_gdfx_top_10' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdfx_top_10' should have field 'date'");
}

#[test]
fn test_param_schema_stock_gdfx_holding_teamwork() {
let schema = get_tool_schema("stock_gdfx_holding_teamwork");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_gdfx_holding_teamwork' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hold_change() {
let schema = get_tool_schema("stock_hold_change");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hold_change' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hold_control() {
let schema = get_tool_schema("stock_hold_control");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hold_control' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_info_sh_name_code() {
let schema = get_tool_schema("stock_info_sh_name_code");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_info_sh_name_code' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_info_sz_name_code() {
let schema = get_tool_schema("stock_info_sz_name_code");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_info_sz_name_code' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_irm() {
let schema = get_tool_schema("stock_irm");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_irm' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_irm_ans() {
let schema = get_tool_schema("stock_irm_ans");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_irm_ans' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_changes() {
let schema = get_tool_schema("stock_changes");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_changes' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_dzjy_mrtj() {
let schema = get_tool_schema("stock_dzjy_mrtj");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_dzjy_mrtj' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_dzjy_mrtj' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_yjbb() {
let schema = get_tool_schema("stock_yjbb");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_yjbb' should have field 'date'");
}

#[test]
fn test_param_schema_stock_lhb_jgstatistic() {
let schema = get_tool_schema("stock_lhb_jgstatistic");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_lhb_jgstatistic' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_lhb_yybph() {
let schema = get_tool_schema("stock_lhb_yybph");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_lhb_yybph' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_lhb_yyb_detail() {
let schema = get_tool_schema("stock_lhb_yyb_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_lhb_yyb_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_lhb_detail_daily() {
let schema = get_tool_schema("stock_lhb_detail_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_lhb_detail_daily' should have field 'date'");
}

#[test]
fn test_param_schema_stock_lhb_jgzz() {
let schema = get_tool_schema("stock_lhb_jgzz");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_lhb_jgzz' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_lhb_yytj() {
let schema = get_tool_schema("stock_lhb_yytj");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_lhb_yytj' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_jgdy_tj() {
let schema = get_tool_schema("stock_jgdy_tj");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_jgdy_tj' should have field 'date'");
}

#[test]
fn test_param_schema_stock_jgdy_detail() {
let schema = get_tool_schema("stock_jgdy_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_jgdy_detail' should have field 'date'");
}

#[test]
fn test_param_schema_stock_fund_flow_individual() {
let schema = get_tool_schema("stock_fund_flow_individual");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_fund_flow_individual' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_fund_flow_concept() {
let schema = get_tool_schema("stock_fund_flow_concept");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_fund_flow_concept' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_fund_flow_industry() {
let schema = get_tool_schema("stock_fund_flow_industry");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_fund_flow_industry' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_fund_flow_big_deal() {
let schema = get_tool_schema("stock_fund_flow_big_deal");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_fund_flow_big_deal' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_main_fund_flow() {
let schema = get_tool_schema("stock_main_fund_flow");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_main_fund_flow' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_gdhs() {
let schema = get_tool_schema("stock_gdhs");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdhs' should have field 'date'");
}

#[test]
fn test_param_schema_stock_gdhs_detail() {
let schema = get_tool_schema("stock_gdhs_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_gdhs_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_gdhs() {
let schema = get_tool_schema("stock_zh_a_gdhs");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_gdhs' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_gdhs_detail() {
let schema = get_tool_schema("stock_zh_a_gdhs_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_gdhs_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_industry_change() {
let schema = get_tool_schema("stock_industry_change");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_industry_change' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_industry_clf_hist_sw() {
let schema = get_tool_schema("stock_industry_clf_hist_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_industry_clf_hist_sw' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_fhps() {
let schema = get_tool_schema("stock_fhps");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_fhps' should have field 'date'");
}

#[test]
fn test_param_schema_stock_zcfz() {
let schema = get_tool_schema("stock_zcfz");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_zcfz' should have field 'date'");
}

#[test]
fn test_param_schema_stock_lrb() {
let schema = get_tool_schema("stock_lrb");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_lrb' should have field 'date'");
}

#[test]
fn test_param_schema_stock_xjll() {
let schema = get_tool_schema("stock_xjll");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_xjll' should have field 'date'");
}

#[test]
fn test_param_schema_stock_hsgt_hist() {
let schema = get_tool_schema("stock_hsgt_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hsgt_hist' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hsgt_fund_min() {
let schema = get_tool_schema("stock_hsgt_fund_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hsgt_fund_min' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_margin_detail_sse() {
let schema = get_tool_schema("stock_margin_detail_sse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_margin_detail_sse' should have field 'date'");
}

#[test]
fn test_param_schema_stock_margin_detail_szse() {
let schema = get_tool_schema("stock_margin_detail_szse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_margin_detail_szse' should have field 'date'");
}

#[test]
fn test_param_schema_stock_margin_szse() {
let schema = get_tool_schema("stock_margin_szse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_margin_szse' should have field 'date'");
}

#[test]
fn test_param_schema_stock_xgsglb() {
let schema = get_tool_schema("stock_xgsglb");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_xgsglb' should have field 'date'");
        assert!(properties.contains_key("market"),
"Schema for 'stock_xgsglb' should have field 'market'");
}

#[test]
fn test_param_schema_stock_hot_follow_xq() {
let schema = get_tool_schema("stock_hot_follow_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hot_follow_xq' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hot_tweet_xq() {
let schema = get_tool_schema("stock_hot_tweet_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hot_tweet_xq' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hot_deal_xq() {
let schema = get_tool_schema("stock_hot_deal_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hot_deal_xq' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_yjkb() {
let schema = get_tool_schema("stock_yjkb");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_yjkb' should have field 'date'");
}

#[test]
fn test_param_schema_stock_yjyg() {
let schema = get_tool_schema("stock_yjyg");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_yjyg' should have field 'date'");
}

#[test]
fn test_param_schema_stock_yysj() {
let schema = get_tool_schema("stock_yysj");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_yysj' should have field 'date'");
        assert!(properties.contains_key("market"),
"Schema for 'stock_yysj' should have field 'market'");
}

#[test]
fn test_param_schema_stock_zt_pool() {
let schema = get_tool_schema("stock_zt_pool");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_zt_pool' should have field 'date'");
}

#[test]
fn test_param_schema_stock_zt_pool_dtgc() {
let schema = get_tool_schema("stock_zt_pool_dtgc");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_zt_pool_dtgc' should have field 'date'");
}

#[test]
fn test_param_schema_stock_zt_pool_previous() {
let schema = get_tool_schema("stock_zt_pool_previous");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_zt_pool_previous' should have field 'date'");
}

#[test]
fn test_param_schema_stock_zt_pool_strong() {
let schema = get_tool_schema("stock_zt_pool_strong");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_zt_pool_strong' should have field 'date'");
}

#[test]
fn test_param_schema_stock_zt_pool_sub_new() {
let schema = get_tool_schema("stock_zt_pool_sub_new");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_zt_pool_sub_new' should have field 'date'");
}

#[test]
fn test_param_schema_stock_zt_pool_zbgc() {
let schema = get_tool_schema("stock_zt_pool_zbgc");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_zt_pool_zbgc' should have field 'date'");
}

#[test]
fn test_param_schema_stock_ggcg() {
let schema = get_tool_schema("stock_ggcg");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_ggcg' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_rank_forecast() {
let schema = get_tool_schema("stock_rank_forecast");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_rank_forecast' should have field 'date'");
}

#[test]
fn test_param_schema_stock_sy_yq() {
let schema = get_tool_schema("stock_sy_yq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_sy_yq' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_sy_yq' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_sy_jz() {
let schema = get_tool_schema("stock_sy_jz");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_sy_jz' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_sy() {
let schema = get_tool_schema("stock_sy");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_sy' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_allotment() {
let schema = get_tool_schema("stock_allotment");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_allotment' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_a_code_to_symbol() {
let schema = get_tool_schema("stock_a_code_to_symbol");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_a_code_to_symbol' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_classify() {
let schema = get_tool_schema("stock_classify");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_classify' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_concept_cons_futu() {
let schema = get_tool_schema("stock_concept_cons_futu");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_concept_cons_futu' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_cyq() {
let schema = get_tool_schema("stock_cyq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_cyq' should have field 'symbol'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_cyq' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_esg_rate() {
let schema = get_tool_schema("stock_esg_rate");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_esg_rate' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_news() {
let schema = get_tool_schema("stock_news");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_news' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_news_em_hk() {
let schema = get_tool_schema("stock_news_em_hk");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_news_em_hk' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_news_main_cx() {
let schema = get_tool_schema("stock_news_main_cx");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_news_main_cx' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_price_js() {
let schema = get_tool_schema("stock_price_js");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_price_js' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_qsjy() {
let schema = get_tool_schema("stock_qsjy");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_qsjy' should have field 'date'");
}

#[test]
fn test_param_schema_stock_report_disclosure() {
let schema = get_tool_schema("stock_report_disclosure");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_report_disclosure' should have field 'date'");
}

#[test]
fn test_param_schema_stock_report_fund_hold_detail() {
let schema = get_tool_schema("stock_report_fund_hold_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_report_fund_hold_detail' should have field 'date'");
}

#[test]
fn test_param_schema_stock_research_report() {
let schema = get_tool_schema("stock_research_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_research_report' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_sector_detail() {
let schema = get_tool_schema("stock_sector_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_sector_detail' should have field 'symbol'");
        assert!(properties.contains_key("sector"),
"Schema for 'stock_sector_detail' should have field 'sector'");
}

#[test]
fn test_param_schema_stock_share_change() {
let schema = get_tool_schema("stock_share_change");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_share_change' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_sns_sseinfo() {
let schema = get_tool_schema("stock_sns_sseinfo");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_sns_sseinfo' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_sse_deal_daily() {
let schema = get_tool_schema("stock_sse_deal_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_sse_deal_daily' should have field 'date'");
}

#[test]
fn test_param_schema_stock_szse_area_summary() {
let schema = get_tool_schema("stock_szse_area_summary");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_szse_area_summary' should have field 'date'");
}

#[test]
fn test_param_schema_stock_szse_sector_summary() {
let schema = get_tool_schema("stock_szse_sector_summary");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_szse_sector_summary' should have field 'date'");
}

#[test]
fn test_param_schema_stock_szse_summary() {
let schema = get_tool_schema("stock_szse_summary");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_szse_summary' should have field 'date'");
}

#[test]
fn test_param_schema_stock_tfp() {
let schema = get_tool_schema("stock_tfp");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_tfp' should have field 'date'");
}

#[test]
fn test_param_schema_stock_value() {
let schema = get_tool_schema("stock_value");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_value' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_xgsr() {
let schema = get_tool_schema("stock_xgsr");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_xgsr' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_yzxdr() {
let schema = get_tool_schema("stock_yzxdr");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_yzxdr' should have field 'date'");
}

#[test]
fn test_param_schema_stock_cg_guarantee() {
let schema = get_tool_schema("stock_cg_guarantee");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_cg_guarantee' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_cg_lawsuit() {
let schema = get_tool_schema("stock_cg_lawsuit");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_cg_lawsuit' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zcfz_bj() {
let schema = get_tool_schema("stock_zcfz_bj");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zcfz_bj' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_index_pb_lg() {
let schema = get_tool_schema("stock_index_pb_lg");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_index_pb_lg' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_index_pe_lg() {
let schema = get_tool_schema("stock_index_pe_lg");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_index_pe_lg' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_spot_em_flex() {
let schema = get_tool_schema("stock_zh_a_spot_em_flex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_zh_a_spot_em_flex' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_zh_a_st() {
let schema = get_tool_schema("stock_zh_a_st");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_zh_a_st' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_zh_a_new_em() {
let schema = get_tool_schema("stock_zh_a_new_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_zh_a_new_em' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_staq_net_stop() {
let schema = get_tool_schema("stock_staq_net_stop");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_staq_net_stop' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_hk_spot_em_flex() {
let schema = get_tool_schema("stock_hk_spot_em_flex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_hk_spot_em_flex' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_us_spot_em_flex() {
let schema = get_tool_schema("stock_us_spot_em_flex");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_us_spot_em_flex' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_board_concept_name_em() {
let schema = get_tool_schema("stock_board_concept_name_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_board_concept_name_em' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_zh_ah_spot_em() {
let schema = get_tool_schema("stock_zh_ah_spot_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_zh_ah_spot_em' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_hsgt_sh_hk_spot() {
let schema = get_tool_schema("stock_hsgt_sh_hk_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_hsgt_sh_hk_spot' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_hsgt_sz_hk_spot() {
let schema = get_tool_schema("stock_hsgt_sz_hk_spot");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_hsgt_sz_hk_spot' should have field 'limit'");
}

#[test]
fn test_param_schema_hk_market_cap_from_tencent() {
let schema = get_tool_schema("hk_market_cap_from_tencent");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'hk_market_cap_from_tencent' should have field 'symbol'");
}

#[test]
fn test_param_schema_hk_financial() {
let schema = get_tool_schema("hk_financial");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'hk_financial' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_dzjy_sctj() {
let schema = get_tool_schema("stock_dzjy_sctj");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_dzjy_sctj' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_gsrl_gsdt() {
let schema = get_tool_schema("stock_gsrl_gsdt");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_gsrl_gsdt' should have field 'date'");
}

#[test]
fn test_param_schema_stock_sse_summary() {
let schema = get_tool_schema("stock_sse_summary");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_sse_summary' should have field 'date'");
}

#[test]
fn test_param_schema_us_market_cap_from() {
let schema = get_tool_schema("us_market_cap_from");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'us_market_cap_from' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_etf_dividend() {
let schema = get_tool_schema("fund_etf_dividend");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_etf_dividend' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_scale_open() {
let schema = get_tool_schema("fund_scale_open");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_scale_open' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_etf_category_sina() {
let schema = get_tool_schema("fund_etf_category_sina");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_etf_category_sina' should have field 'symbol'");
}

#[test]
fn test_param_schema_energy_oil_detail() {
let schema = get_tool_schema("energy_oil_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'energy_oil_detail' should have field 'date'");
}

#[test]
fn test_param_schema_commodity_spot_prices() {
let schema = get_tool_schema("commodity_spot_prices");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'commodity_spot_prices' should have field 'limit'");
}

#[test]
fn test_param_schema_news_trade_notify_suspend() {
let schema = get_tool_schema("news_trade_notify_suspend");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'news_trade_notify_suspend' should have field 'date'");
}

#[test]
fn test_param_schema_bing_news() {
let schema = get_tool_schema("bing_news");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("query"),
"Schema for 'bing_news' should have field 'query'");
        assert!(properties.contains_key("timeout_secs"),
"Schema for 'bing_news' should have field 'timeout_secs'");
}

#[test]
fn test_param_schema_bond_zh_cov_value_analysis() {
let schema = get_tool_schema("bond_zh_cov_value_analysis");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_zh_cov_value_analysis' should have field 'symbol'");
}

#[test]
fn test_param_schema_eastmoney_klines() {
let schema = get_tool_schema("eastmoney_klines");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'eastmoney_klines' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'eastmoney_klines' should have field 'limit'");
}

#[test]
fn test_param_schema_eastmoney_sector_rankings() {
let schema = get_tool_schema("eastmoney_sector_rankings");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("sector_type"),
"Schema for 'eastmoney_sector_rankings' should have field 'sector_type'");
        assert!(properties.contains_key("limit"),
"Schema for 'eastmoney_sector_rankings' should have field 'limit'");
}

#[test]
fn test_param_schema_eastmoney_sector_constituents() {
let schema = get_tool_schema("eastmoney_sector_constituents");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("sector_code"),
"Schema for 'eastmoney_sector_constituents' should have field 'sector_code'");
        assert!(properties.contains_key("limit"),
"Schema for 'eastmoney_sector_constituents' should have field 'limit'");
}

#[test]
fn test_param_schema_eastmoney_sector_capital_flow() {
let schema = get_tool_schema("eastmoney_sector_capital_flow");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("sector_code"),
"Schema for 'eastmoney_sector_capital_flow' should have field 'sector_code'");
        assert!(properties.contains_key("limit"),
"Schema for 'eastmoney_sector_capital_flow' should have field 'limit'");
}

#[test]
fn test_param_schema_eastmoney_capital_flow() {
let schema = get_tool_schema("eastmoney_capital_flow");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'eastmoney_capital_flow' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'eastmoney_capital_flow' should have field 'limit'");
}

#[test]
fn test_param_schema_eastmoney_announcement_detail() {
let schema = get_tool_schema("eastmoney_announcement_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'eastmoney_announcement_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_a_share_sector_rankings() {
let schema = get_tool_schema("a_share_sector_rankings");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("sector_type"),
"Schema for 'a_share_sector_rankings' should have field 'sector_type'");
        assert!(properties.contains_key("limit"),
"Schema for 'a_share_sector_rankings' should have field 'limit'");
}

#[test]
fn test_param_schema_a_share_sector_constituents() {
let schema = get_tool_schema("a_share_sector_constituents");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("sector_code"),
"Schema for 'a_share_sector_constituents' should have field 'sector_code'");
        assert!(properties.contains_key("limit"),
"Schema for 'a_share_sector_constituents' should have field 'limit'");
}

#[test]
fn test_param_schema_a_share_sector_capital_flow() {
let schema = get_tool_schema("a_share_sector_capital_flow");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("sector_code"),
"Schema for 'a_share_sector_capital_flow' should have field 'sector_code'");
        assert!(properties.contains_key("limit"),
"Schema for 'a_share_sector_capital_flow' should have field 'limit'");
}

#[test]
fn test_param_schema_a_share_trade_calendar() {
let schema = get_tool_schema("a_share_trade_calendar");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("exchange"),
"Schema for 'a_share_trade_calendar' should have field 'exchange'");
        assert!(properties.contains_key("start_date"),
"Schema for 'a_share_trade_calendar' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'a_share_trade_calendar' should have field 'end_date'");
}

#[test]
fn test_param_schema_a_share_announcements() {
let schema = get_tool_schema("a_share_announcements");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'a_share_announcements' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'a_share_announcements' should have field 'limit'");
}

#[test]
fn test_param_schema_a_share_billboard() {
let schema = get_tool_schema("a_share_billboard");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'a_share_billboard' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'a_share_billboard' should have field 'limit'");
}

#[test]
fn test_param_schema_a_share_billboard_seats() {
let schema = get_tool_schema("a_share_billboard_seats");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'a_share_billboard_seats' should have field 'symbol'");
        assert!(properties.contains_key("side"),
"Schema for 'a_share_billboard_seats' should have field 'side'");
        assert!(properties.contains_key("limit"),
"Schema for 'a_share_billboard_seats' should have field 'limit'");
}

#[test]
fn test_param_schema_a_share_capital_flow() {
let schema = get_tool_schema("a_share_capital_flow");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'a_share_capital_flow' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'a_share_capital_flow' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_financial_hk_balance_sheet_typed() {
let schema = get_tool_schema("stock_financial_hk_balance_sheet_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_hk_balance_sheet_typed' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_hk_balance_sheet_typed' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_hk_income_sheet_typed() {
let schema = get_tool_schema("stock_financial_hk_income_sheet_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_hk_income_sheet_typed' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_hk_income_sheet_typed' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_hk_cashflow_sheet_typed() {
let schema = get_tool_schema("stock_financial_hk_cashflow_sheet_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_hk_cashflow_sheet_typed' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_hk_cashflow_sheet_typed' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_us_balance_sheet_typed() {
let schema = get_tool_schema("stock_financial_us_balance_sheet_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_us_balance_sheet_typed' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_us_balance_sheet_typed' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_us_income_sheet_typed() {
let schema = get_tool_schema("stock_financial_us_income_sheet_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_us_income_sheet_typed' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_us_income_sheet_typed' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_us_cashflow_sheet_typed() {
let schema = get_tool_schema("stock_financial_us_cashflow_sheet_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_us_cashflow_sheet_typed' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_us_cashflow_sheet_typed' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_main_stock_holder() {
let schema = get_tool_schema("stock_main_stock_holder");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_main_stock_holder' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_institute_hold_detail() {
let schema = get_tool_schema("stock_institute_hold_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_institute_hold_detail' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_institute_hold_detail' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_ipo_info() {
let schema = get_tool_schema("stock_ipo_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_ipo_info' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_ipo() {
let schema = get_tool_schema("stock_ipo");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_ipo' should have field 'date'");
}

#[test]
fn test_param_schema_stock_register() {
let schema = get_tool_schema("stock_register");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_register' should have field 'date'");
}

#[test]
fn test_param_schema_stock_profit_forecast_em() {
let schema = get_tool_schema("stock_profit_forecast_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_profit_forecast_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_sector_fund_flow_rank() {
let schema = get_tool_schema("stock_sector_fund_flow_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_sector_fund_flow_rank' should have field 'date'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_sector_fund_flow_rank' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_hsgt_hold_stock() {
let schema = get_tool_schema("stock_hsgt_hold_stock");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_hsgt_hold_stock' should have field 'date'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_hsgt_hold_stock' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_zh_a_disclosure_relation() {
let schema = get_tool_schema("stock_zh_a_disclosure_relation");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_disclosure_relation' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_analyst_rank() {
let schema = get_tool_schema("stock_analyst_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_analyst_rank' should have field 'date'");
}

#[test]
fn test_param_schema_stock_analyst_detail() {
let schema = get_tool_schema("stock_analyst_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_analyst_detail' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_analyst_detail' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_board_concept_cons() {
let schema = get_tool_schema("stock_board_concept_cons");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("sector_code"),
"Schema for 'stock_board_concept_cons' should have field 'sector_code'");
        assert!(properties.contains_key("limit"),
"Schema for 'stock_board_concept_cons' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_board_industry_cons() {
let schema = get_tool_schema("stock_board_industry_cons");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("sector_code"),
"Schema for 'stock_board_industry_cons' should have field 'sector_code'");
        assert!(properties.contains_key("limit"),
"Schema for 'stock_board_industry_cons' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_repurchase() {
let schema = get_tool_schema("stock_repurchase");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_repurchase' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_hsgt_north_net_flow_kamt() {
let schema = get_tool_schema("stock_hsgt_north_net_flow_kamt");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_hsgt_north_net_flow_kamt' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_hsgt_south_net_flow_kamt() {
let schema = get_tool_schema("stock_hsgt_south_net_flow_kamt");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("limit"),
"Schema for 'stock_hsgt_south_net_flow_kamt' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_individual_fund_flow_rank() {
let schema = get_tool_schema("stock_individual_fund_flow_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_individual_fund_flow_rank' should have field 'indicator'");
        assert!(properties.contains_key("limit"),
"Schema for 'stock_individual_fund_flow_rank' should have field 'limit'");
}

#[test]
fn test_param_schema_eastmoney_billboard() {
let schema = get_tool_schema("eastmoney_billboard");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'eastmoney_billboard' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'eastmoney_billboard' should have field 'limit'");
}

#[test]
fn test_param_schema_eastmoney_announcements() {
let schema = get_tool_schema("eastmoney_announcements");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'eastmoney_announcements' should have field 'symbol'");
        assert!(properties.contains_key("limit"),
"Schema for 'eastmoney_announcements' should have field 'limit'");
}

#[test]
fn test_param_schema_index_detail_hist_adjust_cni() {
let schema = get_tool_schema("index_detail_hist_adjust_cni");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_detail_hist_adjust_cni' should have field 'symbol'");
}

#[test]
fn test_param_schema_index_hist_sw() {
let schema = get_tool_schema("index_hist_sw");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_hist_sw' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'index_hist_sw' should have field 'period'");
}

#[test]
fn test_param_schema_index_zh_a_hist() {
let schema = get_tool_schema("index_zh_a_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_zh_a_hist' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'index_zh_a_hist' should have field 'period'");
}

#[test]
fn test_param_schema_reits_hist_min() {
let schema = get_tool_schema("reits_hist_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'reits_hist_min' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'reits_hist_min' should have field 'period'");
}

#[test]
fn test_param_schema_stock_individual_basic_info_xq() {
let schema = get_tool_schema("stock_individual_basic_info_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_individual_basic_info_xq' should have field 'symbol'");
        assert!(properties.contains_key("token"),
"Schema for 'stock_individual_basic_info_xq' should have field 'token'");
}

#[test]
fn test_param_schema_stock_individual_basic_info_us_xq() {
let schema = get_tool_schema("stock_individual_basic_info_us_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_individual_basic_info_us_xq' should have field 'symbol'");
        assert!(properties.contains_key("token"),
"Schema for 'stock_individual_basic_info_us_xq' should have field 'token'");
}

#[test]
fn test_param_schema_stock_individual_basic_info_hk_xq() {
let schema = get_tool_schema("stock_individual_basic_info_hk_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_individual_basic_info_hk_xq' should have field 'symbol'");
        assert!(properties.contains_key("token"),
"Schema for 'stock_individual_basic_info_hk_xq' should have field 'token'");
}

#[test]
fn test_param_schema_stock_zh_kcb_daily() {
let schema = get_tool_schema("stock_zh_kcb_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_kcb_daily' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_zh_kcb_daily' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_kcb_daily' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_kcb_daily' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_zh_kcb_daily' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_board_concept_hist() {
let schema = get_tool_schema("stock_board_concept_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_concept_hist' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_board_concept_hist' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_board_concept_hist' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_board_concept_hist' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_board_concept_hist' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_board_concept_hist_min() {
let schema = get_tool_schema("stock_board_concept_hist_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_concept_hist_min' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_board_concept_hist_min' should have field 'period'");
}

#[test]
fn test_param_schema_stock_board_industry_hist() {
let schema = get_tool_schema("stock_board_industry_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_industry_hist' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_board_industry_hist' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_board_industry_hist' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_board_industry_hist' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_board_industry_hist' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_board_industry_hist_min() {
let schema = get_tool_schema("stock_board_industry_hist_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_industry_hist_min' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_board_industry_hist_min' should have field 'period'");
}

#[test]
fn test_param_schema_stock_zh_b_daily() {
let schema = get_tool_schema("stock_zh_b_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_b_daily' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_zh_b_daily' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_b_daily' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_b_daily' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_zh_b_daily' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_zh_b_minute() {
let schema = get_tool_schema("stock_zh_b_minute");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_b_minute' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_zh_b_minute' should have field 'period'");
}

#[test]
fn test_param_schema_stock_board_concept_index() {
let schema = get_tool_schema("stock_board_concept_index");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_concept_index' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_board_concept_index' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_board_concept_index' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_board_concept_summary() {
let schema = get_tool_schema("stock_board_concept_summary");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_concept_summary' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_board_industry_index() {
let schema = get_tool_schema("stock_board_industry_index");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_industry_index' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_board_industry_index' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_board_industry_index' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_board_industry_summary() {
let schema = get_tool_schema("stock_board_industry_summary");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_board_industry_summary' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zh_a_minute() {
let schema = get_tool_schema("stock_zh_a_minute");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_minute' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_zh_a_minute' should have field 'period'");
}

#[test]
fn test_param_schema_stock_zh_a_cdr_daily() {
let schema = get_tool_schema("stock_zh_a_cdr_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_cdr_daily' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_a_cdr_daily' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_a_cdr_daily' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_zh_a_hist_tx() {
let schema = get_tool_schema("stock_zh_a_hist_tx");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_hist_tx' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_zh_a_hist_tx' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_a_hist_tx' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_a_hist_tx' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_zh_a_hist_tx' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_zh_vote() {
let schema = get_tool_schema("stock_zh_vote");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_vote' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_zh_vote' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_zh_index_daily_tx() {
let schema = get_tool_schema("stock_zh_index_daily_tx");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_index_daily_tx' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_index_daily_tx' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_index_daily_tx' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_financial_abstract_ths() {
let schema = get_tool_schema("stock_financial_abstract_ths");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_abstract_ths' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_abstract_ths' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_debt() {
let schema = get_tool_schema("stock_financial_debt");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_debt' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_debt' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_benefit() {
let schema = get_tool_schema("stock_financial_benefit");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_benefit' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_benefit' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_cash() {
let schema = get_tool_schema("stock_financial_cash");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_cash' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_cash' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_abstract_new() {
let schema = get_tool_schema("stock_financial_abstract_new");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_abstract_new' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_abstract_new' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_debt_new() {
let schema = get_tool_schema("stock_financial_debt_new");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_debt_new' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_debt_new' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_benefit_new() {
let schema = get_tool_schema("stock_financial_benefit_new");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_benefit_new' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_benefit_new' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_cash_new() {
let schema = get_tool_schema("stock_financial_cash_new");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_cash_new' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_cash_new' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_management_change() {
let schema = get_tool_schema("stock_management_change");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_management_change' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_shareholder_change() {
let schema = get_tool_schema("stock_shareholder_change");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_shareholder_change' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_profit_forecast_ths() {
let schema = get_tool_schema("stock_profit_forecast_ths");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_profit_forecast_ths' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_profit_forecast_ths' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_hk_profit_forecast_et() {
let schema = get_tool_schema("stock_hk_profit_forecast_et");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_profit_forecast_et' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_hk_profit_forecast_et' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_analysis_indicator_em() {
let schema = get_tool_schema("stock_financial_analysis_indicator_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_analysis_indicator_em' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_analysis_indicator_em' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_hk_analysis_indicator() {
let schema = get_tool_schema("stock_financial_hk_analysis_indicator");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_hk_analysis_indicator' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_hk_analysis_indicator' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_us_analysis_indicator() {
let schema = get_tool_schema("stock_financial_us_analysis_indicator");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_us_analysis_indicator' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_us_analysis_indicator' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_hk_analysis_indicator_em_typed() {
let schema = get_tool_schema("stock_financial_hk_analysis_indicator_em_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_hk_analysis_indicator_em_typed' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_hk_analysis_indicator_em_typed' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_financial_us_analysis_indicator_em_typed() {
let schema = get_tool_schema("stock_financial_us_analysis_indicator_em_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_us_analysis_indicator_em_typed' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_us_analysis_indicator_em_typed' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_restricted_release_detail() {
let schema = get_tool_schema("stock_restricted_release_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_restricted_release_detail' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_restricted_release_detail' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_restricted_release_queue_em() {
let schema = get_tool_schema("stock_restricted_release_queue_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_restricted_release_queue_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_restricted_release_stockholder() {
let schema = get_tool_schema("stock_restricted_release_stockholder");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_restricted_release_stockholder' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'stock_restricted_release_stockholder' should have field 'date'");
}

#[test]
fn test_param_schema_stock_zh_a_gbjg() {
let schema = get_tool_schema("stock_zh_a_gbjg");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_gbjg' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_zygc() {
let schema = get_tool_schema("stock_zygc");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zygc' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_financial_abstract() {
let schema = get_tool_schema("stock_financial_abstract");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_abstract' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_add_stock() {
let schema = get_tool_schema("stock_add_stock");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_add_stock' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_restricted_release_queue_sina() {
let schema = get_tool_schema("stock_restricted_release_queue_sina");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_restricted_release_queue_sina' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_circulate_stock_holder() {
let schema = get_tool_schema("stock_circulate_stock_holder");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_circulate_stock_holder' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_fund_stock_holder() {
let schema = get_tool_schema("stock_fund_stock_holder");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_fund_stock_holder' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_institute_hold() {
let schema = get_tool_schema("stock_institute_hold");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_institute_hold' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_institute_recommend() {
let schema = get_tool_schema("stock_institute_recommend");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_institute_recommend' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_institute_recommend_detail() {
let schema = get_tool_schema("stock_institute_recommend_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_institute_recommend_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_gdfx_free_holding_statistics() {
let schema = get_tool_schema("stock_gdfx_free_holding_statistics");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdfx_free_holding_statistics' should have field 'date'");
}

#[test]
fn test_param_schema_stock_gdfx_holding_statistics() {
let schema = get_tool_schema("stock_gdfx_holding_statistics");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdfx_holding_statistics' should have field 'date'");
}

#[test]
fn test_param_schema_stock_gdfx_free_holding_change() {
let schema = get_tool_schema("stock_gdfx_free_holding_change");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdfx_free_holding_change' should have field 'date'");
}

#[test]
fn test_param_schema_stock_gdfx_free_top_10() {
let schema = get_tool_schema("stock_gdfx_free_top_10");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_gdfx_free_top_10' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdfx_free_top_10' should have field 'date'");
}

#[test]
fn test_param_schema_stock_gdfx_free_holding_detail() {
let schema = get_tool_schema("stock_gdfx_free_holding_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdfx_free_holding_detail' should have field 'date'");
}

#[test]
fn test_param_schema_stock_gdfx_free_holding_analyse() {
let schema = get_tool_schema("stock_gdfx_free_holding_analyse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdfx_free_holding_analyse' should have field 'date'");
}

#[test]
fn test_param_schema_stock_gdfx_holding_analyse() {
let schema = get_tool_schema("stock_gdfx_holding_analyse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdfx_holding_analyse' should have field 'date'");
}

#[test]
fn test_param_schema_stock_gdfx_free_holding_teamwork() {
let schema = get_tool_schema("stock_gdfx_free_holding_teamwork");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_gdfx_free_holding_teamwork' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hold_management_detail_cninfo() {
let schema = get_tool_schema("stock_hold_management_detail_cninfo");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hold_management_detail_cninfo' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hold_management_detail_em() {
let schema = get_tool_schema("stock_hold_management_detail_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hold_management_detail_em' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hold_management_person() {
let schema = get_tool_schema("stock_hold_management_person");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hold_management_person' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hold_num() {
let schema = get_tool_schema("stock_hold_num");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hold_num' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'stock_hold_num' should have field 'date'");
}

#[test]
fn test_param_schema_stock_comment_detail_zlkp_jgcyd() {
let schema = get_tool_schema("stock_comment_detail_zlkp_jgcyd");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_comment_detail_zlkp_jgcyd' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_comment_detail_zhpj_lspf() {
let schema = get_tool_schema("stock_comment_detail_zhpj_lspf");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_comment_detail_zhpj_lspf' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_comment_detail_scrd_focus() {
let schema = get_tool_schema("stock_comment_detail_scrd_focus");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_comment_detail_scrd_focus' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_comment_detail_scrd_desire() {
let schema = get_tool_schema("stock_comment_detail_scrd_desire");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_comment_detail_scrd_desire' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_balance_sheet_by_report_em_typed() {
let schema = get_tool_schema("stock_balance_sheet_by_report_em_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_balance_sheet_by_report_em_typed' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_profit_sheet_by_report_em_typed() {
let schema = get_tool_schema("stock_profit_sheet_by_report_em_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_profit_sheet_by_report_em_typed' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_cash_flow_sheet_by_report_em_typed() {
let schema = get_tool_schema("stock_cash_flow_sheet_by_report_em_typed");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_cash_flow_sheet_by_report_em_typed' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_dzjy_hygtj() {
let schema = get_tool_schema("stock_dzjy_hygtj");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_dzjy_hygtj' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_dzjy_hygtj' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_dzjy_hyyybtj() {
let schema = get_tool_schema("stock_dzjy_hyyybtj");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_dzjy_hyyybtj' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_dzjy_hyyybtj' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_dzjy_yybph() {
let schema = get_tool_schema("stock_dzjy_yybph");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_dzjy_yybph' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_dzjy_yybph' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_gpzy_individual_pledge_ratio_detail() {
let schema = get_tool_schema("stock_gpzy_individual_pledge_ratio_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_gpzy_individual_pledge_ratio_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_lhb_detail() {
let schema = get_tool_schema("stock_lhb_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_lhb_detail' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_lhb_detail' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_lhb_stock_statistic() {
let schema = get_tool_schema("stock_lhb_stock_statistic");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_lhb_stock_statistic' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_lhb_jgmmtj() {
let schema = get_tool_schema("stock_lhb_jgmmtj");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_lhb_jgmmtj' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_lhb_jgmmtj' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_lhb_hyyyb() {
let schema = get_tool_schema("stock_lhb_hyyyb");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_lhb_hyyyb' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_lhb_hyyyb' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_lhb_traderstatistic() {
let schema = get_tool_schema("stock_lhb_traderstatistic");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_lhb_traderstatistic' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_lhb_stock_detail_date() {
let schema = get_tool_schema("stock_lhb_stock_detail_date");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_lhb_stock_detail_date' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_sector_fund_flow_hist() {
let schema = get_tool_schema("stock_sector_fund_flow_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_sector_fund_flow_hist' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_concept_fund_flow_hist() {
let schema = get_tool_schema("stock_concept_fund_flow_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_concept_fund_flow_hist' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_industry_pe_ratio() {
let schema = get_tool_schema("stock_industry_pe_ratio");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_industry_pe_ratio' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hsgt_stock_statistics() {
let schema = get_tool_schema("stock_hsgt_stock_statistics");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hsgt_stock_statistics' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_hsgt_stock_statistics' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_hsgt_stock_statistics' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_hsgt_board_rank() {
let schema = get_tool_schema("stock_hsgt_board_rank");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hsgt_board_rank' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_hsgt_board_rank' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_hsgt_individual_detail() {
let schema = get_tool_schema("stock_hsgt_individual_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hsgt_individual_detail' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_hsgt_individual_detail' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_hsgt_individual() {
let schema = get_tool_schema("stock_hsgt_individual");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hsgt_individual' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_margin_account_info_em() {
let schema = get_tool_schema("stock_margin_account_info_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_margin_account_info_em' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_margin_account_info_em' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_margin_ratio_pa() {
let schema = get_tool_schema("stock_margin_ratio_pa");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_margin_ratio_pa' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'stock_margin_ratio_pa' should have field 'date'");
}

#[test]
fn test_param_schema_stock_margin_sse() {
let schema = get_tool_schema("stock_margin_sse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_margin_sse' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_margin_sse' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_margin_underlying_info() {
let schema = get_tool_schema("stock_margin_underlying_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_margin_underlying_info' should have field 'date'");
}

#[test]
fn test_param_schema_stock_balance_sheet_by_report() {
let schema = get_tool_schema("stock_balance_sheet_by_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_balance_sheet_by_report' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_balance_sheet_by_yearly() {
let schema = get_tool_schema("stock_balance_sheet_by_yearly");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_balance_sheet_by_yearly' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_profit_sheet_by_report() {
let schema = get_tool_schema("stock_profit_sheet_by_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_profit_sheet_by_report' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_profit_sheet_by_yearly() {
let schema = get_tool_schema("stock_profit_sheet_by_yearly");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_profit_sheet_by_yearly' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_profit_sheet_by_quarterly() {
let schema = get_tool_schema("stock_profit_sheet_by_quarterly");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_profit_sheet_by_quarterly' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_cash_flow_sheet_by_report() {
let schema = get_tool_schema("stock_cash_flow_sheet_by_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_cash_flow_sheet_by_report' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_cash_flow_sheet_by_yearly() {
let schema = get_tool_schema("stock_cash_flow_sheet_by_yearly");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_cash_flow_sheet_by_yearly' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_cash_flow_sheet_by_quarterly() {
let schema = get_tool_schema("stock_cash_flow_sheet_by_quarterly");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_cash_flow_sheet_by_quarterly' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_share_hold_change_bse() {
let schema = get_tool_schema("stock_share_hold_change_bse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_share_hold_change_bse' should have field 'date'");
}

#[test]
fn test_param_schema_stock_share_hold_change_sse() {
let schema = get_tool_schema("stock_share_hold_change_sse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_share_hold_change_sse' should have field 'date'");
}

#[test]
fn test_param_schema_stock_share_hold_change_szse() {
let schema = get_tool_schema("stock_share_hold_change_szse");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_share_hold_change_szse' should have field 'date'");
}

#[test]
fn test_param_schema_stock_zdhtmx() {
let schema = get_tool_schema("stock_zdhtmx");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zdhtmx' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zdhtmx' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_balance_sheet_by_report_delisted() {
let schema = get_tool_schema("stock_balance_sheet_by_report_delisted");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_balance_sheet_by_report_delisted' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_cash_flow_sheet_by_report_delisted() {
let schema = get_tool_schema("stock_cash_flow_sheet_by_report_delisted");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_cash_flow_sheet_by_report_delisted' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_profit_sheet_by_report_delisted() {
let schema = get_tool_schema("stock_profit_sheet_by_report_delisted");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_profit_sheet_by_report_delisted' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_cg_equity_mortgage() {
let schema = get_tool_schema("stock_cg_equity_mortgage");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_cg_equity_mortgage' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_individual_detail_hold_xq() {
let schema = get_tool_schema("fund_individual_detail_hold_xq");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_individual_detail_hold_xq' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'fund_individual_detail_hold_xq' should have field 'date'");
}

#[test]
fn test_param_schema_fund_announcement_dividend() {
let schema = get_tool_schema("fund_announcement_dividend");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_announcement_dividend' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_announcement_personnel() {
let schema = get_tool_schema("fund_announcement_personnel");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_announcement_personnel' should have field 'symbol'");
}

#[test]
fn test_param_schema_fund_lof_hist_em() {
let schema = get_tool_schema("fund_lof_hist_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_lof_hist_em' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'fund_lof_hist_em' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'fund_lof_hist_em' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'fund_lof_hist_em' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'fund_lof_hist_em' should have field 'adjust'");
}

#[test]
fn test_param_schema_fund_lof_hist_min() {
let schema = get_tool_schema("fund_lof_hist_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_lof_hist_min' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'fund_lof_hist_min' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'fund_lof_hist_min' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'fund_lof_hist_min' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'fund_lof_hist_min' should have field 'adjust'");
}

#[test]
fn test_param_schema_fund_etf_hist_em() {
let schema = get_tool_schema("fund_etf_hist_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_etf_hist_em' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'fund_etf_hist_em' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'fund_etf_hist_em' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'fund_etf_hist_em' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'fund_etf_hist_em' should have field 'adjust'");
}

#[test]
fn test_param_schema_fund_etf_hist_min() {
let schema = get_tool_schema("fund_etf_hist_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_etf_hist_min' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'fund_etf_hist_min' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'fund_etf_hist_min' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'fund_etf_hist_min' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'fund_etf_hist_min' should have field 'adjust'");
}

#[test]
fn test_param_schema_fund_etf_category_ths() {
let schema = get_tool_schema("fund_etf_category_ths");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_etf_category_ths' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'fund_etf_category_ths' should have field 'date'");
}

#[test]
fn test_param_schema_fund_value_estimation() {
let schema = get_tool_schema("fund_value_estimation");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_value_estimation' should have field 'symbol'");
}

#[test]
fn test_param_schema_bond_zh_cov_info() {
let schema = get_tool_schema("bond_zh_cov_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'bond_zh_cov_info' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'bond_zh_cov_info' should have field 'indicator'");
}

#[test]
fn test_param_schema_rv_from_futures_zh_minute() {
let schema = get_tool_schema("rv_from_futures_zh_minute");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'rv_from_futures_zh_minute' should have field 'symbol'");
}

#[test]
fn test_param_schema_rv_from_stock_zh_a_hist_min() {
let schema = get_tool_schema("rv_from_stock_zh_a_hist_min");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'rv_from_stock_zh_a_hist_min' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_financial_hk_report() {
let schema = get_tool_schema("stock_financial_hk_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("stock"),
"Schema for 'stock_financial_hk_report' should have field 'stock'");
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_hk_report' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_hk_report' should have field 'indicator'");
}

#[test]
fn test_param_schema_index_hk_daily() {
let schema = get_tool_schema("index_hk_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'index_hk_daily' should have field 'symbol'");
        assert!(properties.contains_key("internal_id"),
"Schema for 'index_hk_daily' should have field 'internal_id'");
        assert!(properties.contains_key("limit"),
"Schema for 'index_hk_daily' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_individual_fund_flow() {
let schema = get_tool_schema("stock_individual_fund_flow");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_individual_fund_flow' should have field 'symbol'");
        assert!(properties.contains_key("market"),
"Schema for 'stock_individual_fund_flow' should have field 'market'");
        assert!(properties.contains_key("limit"),
"Schema for 'stock_individual_fund_flow' should have field 'limit'");
}

#[test]
fn test_param_schema_eastmoney_billboard_seats() {
let schema = get_tool_schema("eastmoney_billboard_seats");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'eastmoney_billboard_seats' should have field 'symbol'");
        assert!(properties.contains_key("side"),
"Schema for 'eastmoney_billboard_seats' should have field 'side'");
        assert!(properties.contains_key("limit"),
"Schema for 'eastmoney_billboard_seats' should have field 'limit'");
}

#[test]
fn test_param_schema_forex_hist() {
let schema = get_tool_schema("forex_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'forex_hist' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'forex_hist' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'forex_hist' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'forex_hist' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'forex_hist' should have field 'adjust'");
}

#[test]
fn test_param_schema_reits_hist_em() {
let schema = get_tool_schema("reits_hist_em");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'reits_hist_em' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'reits_hist_em' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'reits_hist_em' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'reits_hist_em' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'reits_hist_em' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_dzjy_mrmx() {
let schema = get_tool_schema("stock_dzjy_mrmx");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("asset_type"),
"Schema for 'stock_dzjy_mrmx' should have field 'asset_type'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_dzjy_mrmx' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_dzjy_mrmx' should have field 'end_date'");
        assert!(properties.contains_key("limit"),
"Schema for 'stock_dzjy_mrmx' should have field 'limit'");
}

#[test]
fn test_param_schema_stock_financial_analysis_indicator() {
let schema = get_tool_schema("stock_financial_analysis_indicator");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_analysis_indicator' should have field 'symbol'");
        assert!(properties.contains_key("start_year"),
"Schema for 'stock_financial_analysis_indicator' should have field 'start_year'");
}

#[test]
fn test_param_schema_stock_financial_report() {
let schema = get_tool_schema("stock_financial_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("stock"),
"Schema for 'stock_financial_report' should have field 'stock'");
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_report' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_financial_us_report() {
let schema = get_tool_schema("stock_financial_us_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("stock"),
"Schema for 'stock_financial_us_report' should have field 'stock'");
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_financial_us_report' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_financial_us_report' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_gdfx_holding_detail() {
let schema = get_tool_schema("stock_gdfx_holding_detail");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_gdfx_holding_detail' should have field 'date'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_gdfx_holding_detail' should have field 'indicator'");
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_gdfx_holding_detail' should have field 'symbol'");
}

#[test]
fn test_param_schema_stock_hot_search() {
let schema = get_tool_schema("stock_hot_search");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hot_search' should have field 'symbol'");
        assert!(properties.contains_key("date"),
"Schema for 'stock_hot_search' should have field 'date'");
}

#[test]
fn test_param_schema_stock_notice_report() {
let schema = get_tool_schema("stock_notice_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("date"),
"Schema for 'stock_notice_report' should have field 'date'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_notice_report' should have field 'indicator'");
}

#[test]
fn test_param_schema_stock_zh_a_disclosure_report() {
let schema = get_tool_schema("stock_zh_a_disclosure_report");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_a_disclosure_report' should have field 'symbol'");
        assert!(properties.contains_key("category"),
"Schema for 'stock_zh_a_disclosure_report' should have field 'category'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_a_disclosure_report' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_a_disclosure_report' should have field 'end_date'");
}

#[test]
fn test_param_schema_stock_zh_ah_daily() {
let schema = get_tool_schema("stock_zh_ah_daily");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_ah_daily' should have field 'symbol'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_zh_ah_daily' should have field 'period'");
        assert!(properties.contains_key("start_date"),
"Schema for 'stock_zh_ah_daily' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'stock_zh_ah_daily' should have field 'end_date'");
        assert!(properties.contains_key("adjust"),
"Schema for 'stock_zh_ah_daily' should have field 'adjust'");
}

#[test]
fn test_param_schema_stock_zh_valuation() {
let schema = get_tool_schema("stock_zh_valuation");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_zh_valuation' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_zh_valuation' should have field 'indicator'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_zh_valuation' should have field 'period'");
}

#[test]
fn test_param_schema_stock_us_valuation() {
let schema = get_tool_schema("stock_us_valuation");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_us_valuation' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_us_valuation' should have field 'indicator'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_us_valuation' should have field 'period'");
}

#[test]
fn test_param_schema_stock_hk_valuation() {
let schema = get_tool_schema("stock_hk_valuation");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'stock_hk_valuation' should have field 'symbol'");
        assert!(properties.contains_key("indicator"),
"Schema for 'stock_hk_valuation' should have field 'indicator'");
        assert!(properties.contains_key("period"),
"Schema for 'stock_hk_valuation' should have field 'period'");
}

#[test]
fn test_param_schema_fund_etf_fund_info() {
let schema = get_tool_schema("fund_etf_fund_info");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_etf_fund_info' should have field 'symbol'");
        assert!(properties.contains_key("start_date"),
"Schema for 'fund_etf_fund_info' should have field 'start_date'");
        assert!(properties.contains_key("end_date"),
"Schema for 'fund_etf_fund_info' should have field 'end_date'");
}

#[test]
fn test_param_schema_fund_hk_fund_hist() {
let schema = get_tool_schema("fund_hk_fund_hist");
let properties = schema.get("properties")
.and_then(|p| p.as_object())
.expect("Schema should have properties");

// Verify all expected fields exist
        assert!(properties.contains_key("stock"),
"Schema for 'fund_hk_fund_hist' should have field 'stock'");
        assert!(properties.contains_key("symbol"),
"Schema for 'fund_hk_fund_hist' should have field 'symbol'");
}

