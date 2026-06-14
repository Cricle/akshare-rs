//! MCP tool definitions for akshare financial data.
//!
//! All 86 tools are defined on [`AkShareMcpService`] and organized by category:
//! stock, fund, bond, futures, option, forex, crypto, index, macro_data, economy, news.

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

use akshare::AkShareClient;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{
        router::tool::ToolRouter,
        tool::{ToolCallContext, schema_for_type},
        wrapper::Parameters,
    },
    model::{
        CallToolRequestParams, CallToolResult, Content, Implementation, PaginatedRequestParams,
        ServerCapabilities, ServerInfo, Tool,
    },
    schemars,
    service::RequestContext,
    tool, tool_router,
};

use crate::config::ToolsConfig;

/// Central MCP service exposing 86 akshare financial data tools.
///
/// Implements [`ServerHandler`] for the MCP protocol and routes tool calls
/// to the appropriate akshare API methods.
#[derive(Clone)]
pub struct AkShareMcpService {
    client: AkShareClient,
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl AkShareMcpService {
    /// Create a new service instance with the given tool category configuration.
    #[must_use]
    pub fn new(tools_config: ToolsConfig) -> Self {
        let client = AkShareClient::new();
        let mut router = Self::tool_router();
        Self::apply_category_filter(&mut router, &tools_config);
        Self {
            client,
            tool_router: router,
        }
    }

    /// Disable tools whose category is not enabled in the config.
    fn apply_category_filter(router: &mut ToolRouter<Self>, config: &ToolsConfig) {
        // Tool names use snake_case (from #[tool] macro), not kebab-case
        let categories: &[(&str, &[&str])] = &[
            ("stock", &["stock_", "a_share_", "hk_", "us_", "get_us_"]),
            ("bond", &["bond_"]),
            ("index", &["index_"]),
            ("futures", &["futures_"]),
            ("economy", &["economy_", "movie_", "nlp_", "amac_", "car_", "sw_", "fx_", "article_", "air_", "qdii_", "video_", "sunrise_", "repo_", "migration_", "fred_", "xincaifu_", "spot_", "sogou_", "rate_", "online_", "match_", "hurun_", "hf_", "google_", "gdelt_", "game_", "forbes_", "drewry_", "business_", "bing_", "baidu_", "qhkc_", "methods_in_"]),
            ("crypto", &["crypto_"]),
            ("forex", &["forex_", "currency_"]),
            ("option", &["option_"]),
            ("news", &["news_"]),
            ("macro_data", &["macro_"]),
            ("fund", &["fund_"]),
        ];

        for &(category, prefixes) in categories {
            if !config.is_enabled(category) {
                let to_disable: Vec<_> = router
                    .map
                    .keys()
                    .filter(|name| prefixes.iter().any(|p| name.starts_with(p)))
                    .cloned()
                    .collect();
                for name in to_disable {
                    router.disable_route(name);
                }
            }
        }
    }

    // ── Stock ──────────────────────────────────────────────────

    #[tool(description = "Get A-share real-time quote")]
    async fn a_share_quote(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .a_share_quote(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share K-line candles")]
    async fn a_share_candles(
        &self,
        Parameters(stock::CandlesParams { symbol, limit }): Parameters<stock::CandlesParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .a_share_candles(&symbol, "qfq", limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock real-time quote")]
    async fn hk_quote(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .hk_quote(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock K-line candles")]
    async fn hk_candles(
        &self,
        Parameters(stock::CandlesParams { symbol, limit }): Parameters<stock::CandlesParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .hk_candles(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US stock real-time quote")]
    async fn us_quote(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .us_quote(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US stock K-line candles")]
    async fn us_candles(
        &self,
        Parameters(stock::CandlesParams { symbol, limit }): Parameters<stock::CandlesParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .us_candles(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share real-time spot data from Eastmoney")]
    async fn stock_zh_a_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_a_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock real-time spot data from Eastmoney")]
    async fn stock_hk_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US stock real-time spot data from Eastmoney")]
    async fn stock_us_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_us_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share historical data from Eastmoney")]
    async fn stock_zh_a_hist(
        &self,
        Parameters(stock::StockHistParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<stock::StockHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_a_hist(&symbol, &period, &adjust, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock historical data from Eastmoney")]
    async fn stock_hk_hist(
        &self,
        Parameters(stock::StockHistParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<stock::StockHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_hist(&symbol, &period, &adjust, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US stock historical data from Eastmoney")]
    async fn stock_us_hist(
        &self,
        Parameters(stock::StockHistParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<stock::StockHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_us_hist(&symbol, &period, &adjust, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── A-share Minute K-line ─────────────────────────────────────

    #[tool(description = "Get A-share minute-level K-line. Period: '1','5','15','30','60'")]
    async fn stock_zh_a_hist_min(
        &self,
        Parameters(stock::StockHistParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<stock::StockHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_a_hist_min_em(&symbol, &period, &adjust, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── HK/US Minute K-line ─────────────────────────────────────

    #[tool(description = "Get HK stock minute-level K-line. Period: '1','5','15','30','60'")]
    async fn stock_hk_hist_min(
        &self,
        Parameters(stock::StockHistParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<stock::StockHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_hist_min_em(&symbol, &period, &adjust, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US stock minute-level K-line. Period: '1','5','15','30','60'")]
    async fn stock_us_hist_min(
        &self,
        Parameters(stock::StockHistParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<stock::StockHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_us_hist_min_em(&symbol, &period, &adjust, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── A-share Daily K-line (Sina source) ──────────────────────

    #[tool(description = "Get A-share daily K-line from Sina. Adjust: '','qfq','hfq'")]
    async fn stock_zh_a_daily(
        &self,
        Parameters(stock::StockHistParams {
            symbol,
            start_date,
            end_date,
            adjust,
            ..
        }): Parameters<stock::StockHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_a_daily(&symbol, &start_date, &end_date, &adjust)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── HK/US Daily K-line (Sina source) ──────────────────────

    #[tool(description = "Get HK stock daily K-line from Sina. Adjust: '','qfq','hfq'")]
    async fn stock_hk_daily(
        &self,
        Parameters(stock::StockHistParams {
            symbol,
            start_date,
            end_date,
            adjust,
            ..
        }): Parameters<stock::StockHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_daily(&symbol, &start_date, &end_date, &adjust)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US stock daily K-line from Eastmoney")]
    async fn stock_us_daily(
        &self,
        Parameters(stock::StockDailyParams {
            symbol,
            start_date,
            end_date,
        }): Parameters<stock::StockDailyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_us_daily(&symbol, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── HK Special Data ───────────────────────────────────────

    #[tool(description = "Get famous HK stocks from Eastmoney")]
    async fn stock_hk_famous_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_famous_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock hot rank from Eastmoney")]
    async fn stock_hk_hot_rank_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_hot_rank_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock latest hot rank detail")]
    async fn stock_hk_hot_rank_latest_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_hot_rank_latest_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock historical hot rank detail")]
    async fn stock_hk_hot_rank_detail_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_hot_rank_detail_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK index real-time spot data")]
    async fn stock_hk_index_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_index_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK index daily K-line. Symbol: e.g. 'HSTECH', 'HSI'")]
    async fn stock_hk_index_daily_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_index_daily_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(
        description = "Get HK stock valuation from Baidu. Indicator: '总市值','市盈率(TTM)','市净率','市现率'. Period: '近一年','近三年','全部'"
    )]
    async fn stock_hk_valuation_baidu(
        &self,
        Parameters(stock::ValuationParams {
            symbol,
            indicator,
            period,
        }): Parameters<stock::ValuationParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_valuation_baidu(&symbol, &indicator, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock financial indicators from Eastmoney")]
    async fn stock_hk_financial_indicator_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_financial_indicator_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock dividend payout history from Eastmoney")]
    async fn stock_hk_dividend_payout_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_dividend_payout_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock scale comparison from Eastmoney")]
    async fn stock_hk_scale_comparison_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_scale_comparison_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HSI (Hang Seng Index) dividend yield history")]
    async fn stock_hk_gxl_lg(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_gxl_lg()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK Stock Connect (港股通) constituent stocks")]
    async fn stock_hk_ggt_components_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_ggt_components_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK security profile from Eastmoney")]
    async fn stock_hk_security_profile_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_security_profile_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK company profile from Eastmoney")]
    async fn stock_hk_company_profile_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_company_profile_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK growth comparison (peer comparison) from Eastmoney")]
    async fn stock_hk_growth_comparison_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_growth_comparison_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK valuation comparison (peer comparison) from Eastmoney")]
    async fn stock_hk_valuation_comparison_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_valuation_comparison_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get all HK stock spot data from Sina")]
    async fn stock_hk_spot(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_spot()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK index daily K-line from Sina")]
    async fn stock_hk_index_daily_sina(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_index_daily_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK index spot data from Sina")]
    async fn stock_hk_index_spot_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_index_spot_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock realtime hot rank detail")]
    async fn stock_hk_hot_rank_detail_realtime_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_hot_rank_detail_realtime_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock dividend detail from THS (同花顺)")]
    async fn stock_hk_fhpx_detail_ths(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_fhpx_detail_ths(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK stock indicators from Eniu (requires auth, may return error)")]
    async fn stock_hk_indicator_eniu(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hk_indicator_eniu(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── A-share Special Data ───────────────────────────────────────

    #[tool(description = "Get A-share real-time hot rank from Eastmoney")]
    async fn stock_zh_a_hot_rank_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hot_rank_em(100)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share latest hot rank detail")]
    async fn stock_zh_a_hot_rank_latest_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hot_rank_latest_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share historical hot rank detail")]
    async fn stock_zh_a_hot_rank_detail_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hot_rank_detail_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share realtime hot rank detail")]
    async fn stock_zh_a_hot_rank_detail_realtime_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hot_rank_detail_realtime_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share index real-time spot data from Eastmoney")]
    async fn stock_zh_a_index_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_index_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share index daily K-line from Eastmoney")]
    async fn stock_zh_a_index_daily_em(
        &self,
        Parameters(stock::StockDailyParams {
            symbol,
            start_date,
            end_date,
        }): Parameters<stock::StockDailyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_index_daily_em(&symbol, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share index spot data from Sina")]
    async fn stock_zh_a_index_spot_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_index_spot_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(
        description = "Get A-share valuation from Baidu. Indicator: '总市值','市盈率(TTM)','市净率','市现率'. Period: '近一年','近三年','全部'"
    )]
    async fn stock_zh_a_valuation_baidu(
        &self,
        Parameters(stock::ValuationParams {
            symbol,
            indicator,
            period,
        }): Parameters<stock::ValuationParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_valuation_baidu(&symbol, &indicator, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share scale comparison from Eastmoney")]
    async fn stock_zh_a_scale_comparison_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_scale_comparison_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share growth comparison (peer comparison) from Eastmoney")]
    async fn stock_zh_a_growth_comparison_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_growth_comparison_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share valuation comparison (peer comparison) from Eastmoney")]
    async fn stock_zh_a_valuation_comparison_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_valuation_comparison_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share (CSI) dividend yield history from Legulegu")]
    async fn stock_zh_a_gxl_lg(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_a_gxl_lg()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share dividend detail from THS (同花顺)")]
    async fn stock_zh_a_fhps_detail_ths(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_fhps_detail_ths(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share financial indicators from Eastmoney")]
    async fn stock_zh_a_financial_indicator_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_a_financial_indicator_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share dividend payout history from Eastmoney")]
    async fn stock_zh_a_dividend_payout_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_a_dividend_payout_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share company profile from cninfo (巨潮资讯)")]
    async fn stock_zh_a_profile_cninfo(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_profile_cninfo(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share dividend data from cninfo (巨潮资讯)")]
    async fn stock_zh_a_dividend_cninfo(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_dividend_cninfo(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share dividend detail from Eastmoney")]
    async fn stock_zh_a_fhps_detail_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_fhps_detail_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share Dupont comparison from Eastmoney")]
    async fn stock_zh_a_dupont_comparison_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_dupont_comparison_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share related hot rank stocks from Eastmoney")]
    async fn stock_zh_a_hot_rank_relate_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hot_rank_relate_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share hot keywords from Eastmoney")]
    async fn stock_zh_a_hot_keyword_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hot_keyword_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share hot rising stocks from Eastmoney")]
    async fn stock_zh_a_hot_up_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_hot_up_em(100)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share global financial news from Eastmoney")]
    async fn stock_zh_a_info_global_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_info_global_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── US Special Data ───────────────────────────────────────

    #[tool(
        description = "Get famous US stocks by category. Category: '科技类','金融类','医药食品类','媒体类','汽车能源类','制造零售类'"
    )]
    async fn stock_us_famous_spot_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_us_famous_spot_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US pink sheet (OTC) stocks from Eastmoney")]
    async fn stock_us_pink_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_us_pink_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(
        description = "Get US stock valuation from Baidu. Indicator: '总市值','市盈率(TTM)','市净率','市现率'. Period: '近一年','近三年','全部'"
    )]
    async fn stock_us_valuation_baidu(
        &self,
        Parameters(stock::ValuationParams {
            symbol,
            indicator,
            period,
        }): Parameters<stock::ValuationParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_us_valuation_baidu(&symbol, &indicator, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US stock name list from Sina")]
    async fn get_us_stock_name(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_us_stock_name()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get industry board list from Eastmoney")]
    async fn stock_board_industry_name_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_board_industry_name_em(100)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(
        description = "Get individual stock fund flow from Eastmoney (auto-detects A-share/HK/US market)"
    )]
    async fn stock_individual_fund_flow(
        &self,
        Parameters(stock::FundFlowParams {
            symbol,
            market,
            limit,
        }): Parameters<stock::FundFlowParams>,
    ) -> Result<CallToolResult, McpError> {
        let market = if market.is_empty() {
            detect_fund_flow_market(&symbol)
        } else {
            market
        };
        let data = self
            .client
            .stock_individual_fund_flow(&symbol, &market, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(
        description = "Get main fund flow (主力资金流向) for a stock. Supports A-share (e.g. 600000), HK (e.g. 00700), and US (e.g. AAPL)"
    )]
    async fn stock_main_fund_flow(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_main_fund_flow(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(
        description = "Get fund flow ranking for A-share stocks. Indicator: 'today', '3day', '5day', '10day'"
    )]
    async fn stock_individual_fund_flow_rank(
        &self,
        Parameters(stock::FundFlowRankParams { indicator, limit }): Parameters<
            stock::FundFlowRankParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_individual_fund_flow_rank(&indicator, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Fund ───────────────────────────────────────────────────

    #[tool(description = "Get ETF historical K-line candles")]
    async fn fund_etf_hist(
        &self,
        Parameters(fund::FundHistParams { symbol, limit }): Parameters<fund::FundHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_hist(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund manager list from Eastmoney")]
    async fn fund_manager_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_manager_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get ETF real-time spot data from Eastmoney")]
    async fn fund_etf_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get LOF real-time spot data from Eastmoney")]
    async fn fund_lof_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_lof_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get open fund ranking from Eastmoney")]
    async fn fund_open_fund_rank_em(
        &self,
        Parameters(fund::FundRankParams { symbol, limit }): Parameters<fund::FundRankParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_open_fund_rank_em(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK fund ranking from Eastmoney")]
    async fn fund_hk_rank_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_hk_rank_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(
        description = "Get HK fund historical NAV. Query type: '历史净值明细' or '分红送配详情'"
    )]
    async fn fund_hk_fund_hist_em(
        &self,
        Parameters(fund::FundHkHistParams { code, query_type }): Parameters<fund::FundHkHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_hk_fund_hist_em(&code, &query_type)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond ───────────────────────────────────────────────────

    #[tool(description = "Get China/US government bond yield rates")]
    async fn bond_zh_us_rate(
        &self,
        Parameters(bond::BondRateParams { start_date }): Parameters<bond::BondRateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_us_rate(&start_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get corporate bond yields")]
    async fn bond_corporate_yields(
        &self,
        Parameters(bond::BondLimitParams { limit }): Parameters<bond::BondLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_corporate_yields(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China bond yield curve")]
    async fn bond_china_yield(
        &self,
        Parameters(bond::BondYieldParams {
            start_date,
            end_date,
        }): Parameters<bond::BondYieldParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_china_yield(&start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get bond spot deal data")]
    async fn bond_spot_deal(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_spot_deal()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get bond spot rates")]
    async fn bond_spot_rates(
        &self,
        Parameters(bond::BondLimitParams { limit }): Parameters<bond::BondLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_spot_rates(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures ────────────────────────────────────────────────

    #[tool(description = "Get futures spot prices snapshot")]
    async fn futures_spot_prices(
        &self,
        Parameters(futures::FuturesLimitParams { limit }): Parameters<futures::FuturesLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_spot_prices(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get futures main contract K-line from Sina")]
    async fn futures_main_sina(
        &self,
        Parameters(futures::FuturesCandlesParams { symbol, limit }): Parameters<
            futures::FuturesCandlesParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_main_sina(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX daily futures data")]
    async fn futures_daily_cffex(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_daily_cffex(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE daily futures data")]
    async fn futures_daily_shfe(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_daily_shfe(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE futures position rank")]
    async fn futures_shfe_position_rank(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_shfe_position_rank(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Option ─────────────────────────────────────────────────

    #[tool(description = "Get SSE option Greeks from Sina")]
    async fn option_sse_greeks_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_sse_greeks_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option current day data")]
    async fn option_current_day_sse(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_current_day_sse()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CZCE option history data")]
    async fn option_hist_czce(
        &self,
        Parameters(option::OptionHistParams { symbol, date, .. }): Parameters<
            option::OptionHistParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_hist_czce(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option daily statistics")]
    async fn option_daily_stats_sse(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_daily_stats_sse(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Forex ──────────────────────────────────────────────────

    #[tool(description = "Get BOC (Bank of China) forex rates")]
    async fn forex_boc_rates(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .forex_boc_rates()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Eastmoney forex rates")]
    async fn forex_em_rates(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .forex_em_rates()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get real-time forex spot rates from Eastmoney")]
    async fn forex_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .forex_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get BOC currency exchange rates from Sina")]
    async fn currency_boc_sina(
        &self,
        Parameters(forex::CurrencyParams {
            symbol,
            start_date,
            end_date,
        }): Parameters<forex::CurrencyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .currency_boc_sina(&symbol, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Crypto ─────────────────────────────────────────────────

    #[tool(description = "Get Bitcoin CME futures data")]
    async fn crypto_bitcoin_cme(
        &self,
        Parameters(crypto::CryptoDateParams { date }): Parameters<crypto::CryptoDateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .crypto_bitcoin_cme(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Bitcoin holding report")]
    async fn crypto_bitcoin_hold_report(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .crypto_bitcoin_hold_report()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Index ──────────────────────────────────────────────────

    #[tool(description = "Get global index K-line candles from Yahoo")]
    async fn index_global_candles(
        &self,
        Parameters(index::IndexCandlesParams { symbol, limit }): Parameters<
            index::IndexCandlesParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_global_candles(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get index constituent stocks")]
    async fn index_stock_cons(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_stock_cons(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get index stock info list")]
    async fn index_stock_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_stock_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CSIndex A-share index history")]
    async fn stock_zh_index_hist_csindex(
        &self,
        Parameters(index::IndexHistParams {
            symbol,
            start_date,
            end_date,
        }): Parameters<index::IndexHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_index_hist_csindex(&symbol, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK index spot data from Sina (index module)")]
    async fn index_hk_spot_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_hk_spot_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get HK index spot data from Eastmoney (index module)")]
    async fn index_hk_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_hk_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(
        description = "Get HK index daily K-line from Eastmoney. Use index_hk_spot_em to discover internal_id"
    )]
    async fn index_hk_daily_em(
        &self,
        Parameters(index::IndexHkDailyParams {
            symbol,
            internal_id,
            limit,
        }): Parameters<index::IndexHkDailyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_hk_daily_em(&symbol, &internal_id, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US stock index from Sina (requires JS decoding, may return error)")]
    async fn index_us_stock_sina(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_us_stock_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Macro Data ─────────────────────────────────────────────

    #[tool(description = "Get China GDP data")]
    async fn macro_china_gdp(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_gdp()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US CPI year-over-year data")]
    async fn macro_usa_cpi_yoy(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_cpi_yoy()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China GDP yearly data")]
    async fn macro_china_gdp_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_gdp_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US CPI monthly data")]
    async fn macro_usa_cpi_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_cpi_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Economy ────────────────────────────────────────────────

    #[tool(description = "Get China auto sales data")]
    async fn economy_auto_sales(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .economy_auto_sales()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get box office data")]
    async fn economy_box_office(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .economy_box_office()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get real-time movie box office data")]
    async fn movie_boxoffice_realtime(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .movie_boxoffice_realtime()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "NLP question answering for financial queries")]
    async fn nlp_answer(
        &self,
        Parameters(economy::NlpParams { question }): Parameters<economy::NlpParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .nlp_answer(&question)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(data)]))
    }

    // ── News ───────────────────────────────────────────────────

    #[tool(description = "Get CCTV news for a given date")]
    async fn news_cctv(
        &self,
        Parameters(news::NewsDateParams { date }): Parameters<news::NewsDateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .news_cctv(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Search financial news")]
    async fn news_search(
        &self,
        Parameters(news::NewsSearchParams { query, limit }): Parameters<news::NewsSearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .news_search(&query, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(
        description = "Search stock-specific news by symbol. Supports A-share (e.g. 600000), HK (e.g. 00700), and US (e.g. AAPL) stocks"
    )]
    async fn stock_news_em(
        &self,
        Parameters(news::StockNewsSearchParams { symbol, limit }): Parameters<
            news::StockNewsSearchParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        use akshare::market::detect_market;
        use akshare::types::MarketKind;

        let market = detect_market(&symbol);
        let data = match market {
            MarketKind::AShare => self.client.stock_news_em(&symbol).await,
            MarketKind::HongKong => self.client.stock_news_em_hk(&symbol).await,
            MarketKind::UsEquity => self.client.stock_news_em_us(&symbol).await,
        };
        let mut data = data.map_err(|e| McpError::internal_error(e.to_string(), None))?;
        data.truncate(limit);
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get economic news from Baidu")]
    async fn news_economic_baidu(
        &self,
        Parameters(news::NewsSymbolParams { symbol }): Parameters<news::NewsSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .news_economic_baidu(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get dividend trade notifications from Baidu")]
    async fn news_trade_notify_dividend_baidu(
        &self,
        Parameters(news::NewsDateParams { date }): Parameters<news::NewsDateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .news_trade_notify_dividend_baidu(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Auto-generated from agent_bond_tools.rs ──
// ── Bond: Buyback ──────────────────────────────────────────────

    #[tool(description = "Get Shanghai exchange bond buyback list")]
    async fn bond_sh_buy_back_em(
        &self,
        Parameters(bond::BondLimitParams { limit }): Parameters<bond::BondLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_sh_buy_back_em(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenzhen exchange bond buyback list")]
    async fn bond_sz_buy_back_em(
        &self,
        Parameters(bond::BondLimitParams { limit }): Parameters<bond::BondLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_sz_buy_back_em(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get bond buyback historical daily candles from Eastmoney")]
    async fn bond_buy_back_hist_em(
        &self,
        Parameters(bond::BondSymbolLimitParams { symbol, limit }): Parameters<bond::BondSymbolLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_buy_back_hist_em(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: Convertible Bond (Sina) ──────────────────────────

    #[tool(description = "Get convertible bond profile from Sina Finance")]
    async fn bond_cb_profile_sina(
        &self,
        Parameters(bond::BondSymbolParams { symbol }): Parameters<bond::BondSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_cb_profile_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond summary from Sina Finance")]
    async fn bond_cb_summary_sina(
        &self,
        Parameters(bond::BondSymbolParams { symbol }): Parameters<bond::BondSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_cb_summary_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: Convertible Bond (THS) ───────────────────────────

    #[tool(description = "Get convertible bond issue info from THS")]
    async fn bond_zh_cov_info_ths(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_cov_info_ths()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: CBond Index ──────────────────────────────────────

    #[tool(description = "Get CBond general index data by indicator and period")]
    async fn bond_index_general_cbond(
        &self,
        Parameters(bond::BondIndicatorPeriodParams { indicator, period }): Parameters<bond::BondIndicatorPeriodParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_index_general_cbond(&indicator, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CBond treasury index data by indicator and period")]
    async fn bond_treasury_index_cbond(
        &self,
        Parameters(bond::BondIndicatorPeriodParams { indicator, period }): Parameters<bond::BondIndicatorPeriodParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_treasury_index_cbond(&indicator, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CBond new composite index data by indicator and period")]
    async fn bond_new_composite_index_cbond(
        &self,
        Parameters(bond::BondIndicatorPeriodParams { indicator, period }): Parameters<bond::BondIndicatorPeriodParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_new_composite_index_cbond(&indicator, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get list of all available CBond indices")]
    async fn bond_available_index_cbond(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_available_index_cbond()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CBond composite index data by indicator and period")]
    async fn bond_composite_index_cbond(
        &self,
        Parameters(bond::BondIndicatorPeriodParams { indicator, period }): Parameters<bond::BondIndicatorPeriodParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_composite_index_cbond(&indicator, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: ChinaMoney ───────────────────────────────────────

    #[tool(description = "Get ChinaMoney close yield curve historical data")]
    async fn bond_china_close_return(
        &self,
        Parameters(bond::BondCloseReturnParams { symbol, period, start_date, end_date }): Parameters<bond::BondCloseReturnParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_china_close_return(&symbol, &period, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get FR007 interest rate swap curve historical data from ChinaMoney")]
    async fn macro_china_swap_rate(
        &self,
        Parameters(bond::BondYieldParams { start_date, end_date }): Parameters<bond::BondYieldParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_swap_rate(&start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get bond issuance info from ChinaMoney")]
    async fn macro_china_bond_public(
        &self,
        Parameters(bond::BondLimitParams { limit }): Parameters<bond::BondLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_bond_public(limit as u32)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: Convertible Bond (Eastmoney) ─────────────────────

    #[tool(description = "Get convertible bond real-time list from Eastmoney")]
    async fn bond_convertible_list(
        &self,
        Parameters(bond::BondLimitParams { limit }): Parameters<bond::BondLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_convertible_list(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond historical daily klines from Eastmoney")]
    async fn bond_convertible_hist(
        &self,
        Parameters(bond::BondSymbolLimitParams { symbol, limit }): Parameters<bond::BondSymbolLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_convertible_hist(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: Government Bond (Sina) ───────────────────────────

    #[tool(description = "Get China government bond yield data from Sina Finance")]
    async fn bond_gb_zh_sina(
        &self,
        Parameters(bond::BondSymbolParams { symbol }): Parameters<bond::BondSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_gb_zh_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US government bond yield data from Sina Finance")]
    async fn bond_gb_us_sina(
        &self,
        Parameters(bond::BondSymbolParams { symbol }): Parameters<bond::BondSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_gb_us_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: CNINFO Issuance ──────────────────────────────────

    #[tool(description = "Get government bond issuance data from CNINFO")]
    async fn bond_treasure_issue_cninfo(
        &self,
        Parameters(bond::BondYieldParams { start_date, end_date }): Parameters<bond::BondYieldParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_treasure_issue_cninfo(&start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get local government bond issuance data from CNINFO")]
    async fn bond_local_gov_issue_cninfo(
        &self,
        Parameters(bond::BondYieldParams { start_date, end_date }): Parameters<bond::BondYieldParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_local_gov_issue_cninfo(&start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get corporate bond issuance data from CNINFO")]
    async fn bond_corporate_issue_cninfo(
        &self,
        Parameters(bond::BondYieldParams { start_date, end_date }): Parameters<bond::BondYieldParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_corporate_issue_cninfo(&start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond issuance data from CNINFO")]
    async fn bond_cov_issue_cninfo(
        &self,
        Parameters(bond::BondYieldParams { start_date, end_date }): Parameters<bond::BondYieldParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_cov_issue_cninfo(&start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get local government bond issuance data from CNINFO (Python alias)")]
    async fn bond_local_government_issue_cninfo(
        &self,
        Parameters(bond::BondYieldParams { start_date, end_date }): Parameters<bond::BondYieldParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_local_government_issue_cninfo(&start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond stock conversion data from CNINFO")]
    async fn bond_cov_stock_issue_cninfo(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_cov_stock_issue_cninfo()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: JSL (Jisilu) ─────────────────────────────────────

    #[tool(description = "Get convertible bond list with pricing from Jisilu")]
    async fn bond_cb_jsl(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_cb_jsl()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond market index from Jisilu")]
    async fn bond_cb_index_jsl(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_cb_index_jsl()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond conversion price adjustment logs from Jisilu")]
    async fn bond_cb_adj_logs_jsl(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_cb_adj_logs_jsl()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond forced redemption data from Jisilu")]
    async fn bond_cb_redeem_jsl(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_cb_redeem_jsl()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: NAFMII ───────────────────────────────────────────

    #[tool(description = "Get NAFMII bond registration data by page")]
    async fn bond_debt_nafmii(
        &self,
        Parameters(bond::BondPageParams { page }): Parameters<bond::BondPageParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_debt_nafmii(page as u32)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: Spot ─────────────────────────────────────────────

    #[tool(description = "Get bond spot quote data from ChinaMoney interbank market")]
    async fn bond_spot_quote(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_spot_quote()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: SSE Summary ──────────────────────────────────────

    #[tool(description = "Get bond cash market summary from SSE")]
    async fn bond_cash_summary_sse(
        &self,
        Parameters(bond::BondDateParams { date }): Parameters<bond::BondDateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_cash_summary_sse(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get bond deal summary from SSE")]
    async fn bond_deal_summary_sse(
        &self,
        Parameters(bond::BondDateParams { date }): Parameters<bond::BondDateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_deal_summary_sse(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: Convertible Bond (zh_cov) ────────────────────────

    #[tool(description = "Get convertible bond list with pricing from Eastmoney datacenter")]
    async fn bond_zh_cov(
        &self,
        Parameters(bond::BondLimitParams { limit }): Parameters<bond::BondLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_cov(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond vs stock comparison table from Eastmoney")]
    async fn bond_cov_comparison(
        &self,
        Parameters(bond::BondLimitParams { limit }): Parameters<bond::BondLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_cov_comparison(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond detail info by symbol and indicator (basic/ballot/usage/dates)")]
    async fn bond_zh_cov_info(
        &self,
        Parameters(bond::BondSymbolIndicatorParams { symbol, indicator }): Parameters<bond::BondSymbolIndicatorParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_cov_info(&symbol, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond daily OHLCV data from Eastmoney")]
    async fn bond_zh_hs_cov_daily(
        &self,
        Parameters(bond::BondSymbolParams { symbol }): Parameters<bond::BondSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_hs_cov_daily(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond minute-level data from Eastmoney")]
    async fn bond_zh_hs_cov_min(
        &self,
        Parameters(bond::BondSymbolPeriodParams { symbol, period }): Parameters<bond::BondSymbolPeriodParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_hs_cov_min(&symbol, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond pre-market minute data from Eastmoney")]
    async fn bond_zh_hs_cov_pre_min(
        &self,
        Parameters(bond::BondSymbolParams { symbol }): Parameters<bond::BondSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_hs_cov_pre_min(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond real-time spot data from Eastmoney")]
    async fn bond_zh_hs_cov_spot(
        &self,
        Parameters(bond::BondSymbolParams { symbol }): Parameters<bond::BondSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_hs_cov_spot(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get convertible bond value analysis (premium ratio history) from Eastmoney")]
    async fn bond_zh_cov_value_analysis(
        &self,
        Parameters(bond::BondSymbolParams { symbol }): Parameters<bond::BondSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_cov_value_analysis(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bond: Chinese Bond Spot (Sina) ─────────────────────────

    #[tool(description = "Get Chinese bond real-time spot quotes from Sina Finance")]
    async fn bond_zh_hs_spot(
        &self,
        Parameters(bond::BondLimitParams { limit }): Parameters<bond::BondLimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_hs_spot(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Chinese bond historical daily data from Sina Finance")]
    async fn bond_zh_hs_daily(
        &self,
        Parameters(bond::BondSymbolParams { symbol }): Parameters<bond::BondSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bond_zh_hs_daily(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Auto-generated from agent_crypto_tools.rs ──
    #[tool(description = "Get crypto spot prices from Jin10 (JS)")]
    async fn crypto_js_spot(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .crypto_js_spot()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get crypto spot prices from Jin10 data center")]
    async fn crypto_spot(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .crypto_spot()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Auto-generated from agent_economy_tools.rs ──
    // ── Economy: Air Quality ───────────────────────────────────────

    #[tool(description = "Get air quality index data for a given Chinese city")]
    async fn economy_air_quality(
        &self,
        Parameters(economy::CityParam { city }): Parameters<economy::CityParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .economy_air_quality(&city)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Hebei province air quality forecast data")]
    async fn air_quality_hebei(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .air_quality_hebei()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get list of cities available for air quality monitoring")]
    async fn air_city_table(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .air_city_table()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get historical air quality data for a city with date range")]
    async fn air_quality_hist(
        &self,
        Parameters(economy::AirHistParam {
            city,
            start_date,
            end_date,
        }): Parameters<economy::AirHistParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .air_quality_hist(&city, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get air quality ranking across Chinese cities")]
    async fn air_quality_rank(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .air_quality_rank()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get air quality monitoring station data for a specific city")]
    async fn air_quality_watch_point(
        &self,
        Parameters(economy::AirHistParam {
            city,
            start_date,
            end_date,
        }): Parameters<economy::AirHistParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .air_quality_watch_point(&city, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get sunrise and sunset data for a specific date and city")]
    async fn sunrise_daily(
        &self,
        Parameters(economy::DateCityParam { date, city }): Parameters<economy::DateCityParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .sunrise_daily(&date, &city)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get monthly sunrise and sunset data for a city")]
    async fn sunrise_monthly(
        &self,
        Parameters(economy::DateCityParam { date, city }): Parameters<economy::DateCityParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .sunrise_monthly(&date, &city)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Economy: AMAC Fund Industry ────────────────────────────────

    #[tool(description = "Get AMAC fund industry statistics including AUM and product counts")]
    async fn economy_amac_stats(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .economy_amac_stats()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC fund manager information")]
    async fn amac_manager_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_manager_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC fund manager classification information")]
    async fn amac_manager_classify_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_manager_classify_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get information about cancelled AMAC fund managers")]
    async fn amac_manager_cancelled_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_manager_cancelled_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC private fund product information")]
    async fn amac_fund_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_fund_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC asset-backed securities product information")]
    async fn amac_fund_abs(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_fund_abs()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC private fund sub-fund investment information")]
    async fn amac_fund_sub_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_fund_sub_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC fund dedicated account manager product information")]
    async fn amac_fund_account_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_fund_account_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC member institution information")]
    async fn amac_member_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_member_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC member institution subsidiary information")]
    async fn amac_member_sub_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_member_sub_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC securities and futures operating institution information")]
    async fn amac_securities_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_securities_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC futures operating institution information")]
    async fn amac_futures_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_futures_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC practitioner information")]
    async fn amac_aoin_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_aoin_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC bond practitioner institution list")]
    async fn amac_person_bond_org_list(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_person_bond_org_list()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get AMAC fund practitioner institution list")]
    async fn amac_person_fund_org_list(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .amac_person_fund_org_list()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Economy: Academic Articles & Research ──────────────────────

    #[tool(description = "Get Economic Policy Uncertainty index for a country or region")]
    async fn article_epu_index(
        &self,
        Parameters(economy::SymbolParam { symbol }): Parameters<economy::SymbolParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .article_epu_index(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get FRED-MD monthly macroeconomic dataset for a given date")]
    async fn fred_md(
        &self,
        Parameters(economy::DateParam { date }): Parameters<economy::DateParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fred_md(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get FRED-QD quarterly macroeconomic dataset for a given date")]
    async fn fred_qd(
        &self,
        Parameters(economy::DateParam { date }): Parameters<economy::DateParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fred_qd(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Oxford-Man realized volatility data for a symbol and index")]
    async fn article_oman_rv(
        &self,
        Parameters(economy::SymbolIndexParam { symbol, index }): Parameters<
            economy::SymbolIndexParam,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .article_oman_rv(&symbol, &index)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Oxford-Man realized volatility short front-page chart data")]
    async fn article_oman_rv_short(
        &self,
        Parameters(economy::SymbolParam { symbol }): Parameters<economy::SymbolParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .article_oman_rv_short(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Fama-French 3-factor data from Chicago Booth Risk Research Laboratory")]
    async fn article_ff_crr(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .article_ff_crr()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Risk Lab realized volatility data for a stock ticker")]
    async fn article_rlab_rv(
        &self,
        Parameters(economy::SymbolParam { symbol }): Parameters<economy::SymbolParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .article_rlab_rv(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Economy: Car Market ────────────────────────────────────────

    #[tool(description = "Get CPCA country market data for a given country")]
    async fn car_market_country_cpca(
        &self,
        Parameters(economy::SymbolParam { symbol }): Parameters<economy::SymbolParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .car_market_country_cpca(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CPCA segment market data by vehicle type and indicator")]
    async fn car_market_segment_cpca(
        &self,
        Parameters(economy::SymbolIndicatorParam { symbol, indicator }): Parameters<
            economy::SymbolIndicatorParam,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .car_market_segment_cpca(&symbol, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CPCA total passenger car market data by type and indicator")]
    async fn car_market_total_cpca(
        &self,
        Parameters(economy::SymbolIndicatorParam { symbol, indicator }): Parameters<
            economy::SymbolIndicatorParam,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .car_market_total_cpca(&symbol, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CPCA manufacturer ranking data by type and indicator")]
    async fn car_market_man_rank_cpca(
        &self,
        Parameters(economy::SymbolIndicatorParam { symbol, indicator }): Parameters<
            economy::SymbolIndicatorParam,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .car_market_man_rank_cpca(&symbol, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CPCA vehicle category market data by type and indicator")]
    async fn car_market_cate_cpca(
        &self,
        Parameters(economy::SymbolIndicatorParam { symbol, indicator }): Parameters<
            economy::SymbolIndicatorParam,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .car_market_cate_cpca(&symbol, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CPCA new energy vehicle market data by segment")]
    async fn car_market_fuel_cpca(
        &self,
        Parameters(economy::SymbolParam { symbol }): Parameters<economy::SymbolParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .car_market_fuel_cpca(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Gasgoo auto sales ranking by symbol and date")]
    async fn car_sale_rank_gasgoo(
        &self,
        Parameters(economy::SymbolDateParam { symbol, date }): Parameters<
            economy::SymbolDateParam,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .car_sale_rank_gasgoo(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Economy: Migration ─────────────────────────────────────────

    #[tool(description = "Get Baidu migration area-level details for a city or province")]
    async fn migration_area_baidu(
        &self,
        Parameters(economy::MigrationAreaParam {
            area,
            indicator,
            date,
        }): Parameters<economy::MigrationAreaParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .migration_area_baidu(&area, &indicator, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Baidu migration scale index time series for an area")]
    async fn migration_scale_baidu(
        &self,
        Parameters(economy::MigrationScaleParam { area, indicator }): Parameters<
            economy::MigrationScaleParam,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .migration_scale_baidu(&area, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Economy: Fortune & Rankings ────────────────────────────────

    #[tool(description = "Get Bloomberg Billionaires Index current top billionaires")]
    async fn index_bloomberg_billionaires(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_bloomberg_billionaires()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Bloomberg Billionaires Index historical data by year")]
    async fn index_bloomberg_billionaires_hist(
        &self,
        Parameters(economy::YearParam { year }): Parameters<economy::YearParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_bloomberg_billionaires_hist(&year)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Forbes China rankings by list name")]
    async fn forbes_rank(
        &self,
        Parameters(economy::SymbolParam { symbol }): Parameters<economy::SymbolParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .forbes_rank(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Hurun Rich List rankings by indicator and year")]
    async fn hurun_rank(
        &self,
        Parameters(economy::HurunParam { indicator, year }): Parameters<economy::HurunParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .hurun_rank(&indicator, &year)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Xincaifu 500 Rich List rankings by year")]
    async fn xincaifu_rank(
        &self,
        Parameters(economy::YearParam { year }): Parameters<economy::YearParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .xincaifu_rank(&year)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Economy: Movie Box Office ──────────────────────────────────

    #[tool(description = "Get cinema daily box office data for a given date")]
    async fn movie_boxoffice_cinema_daily(
        &self,
        Parameters(economy::DateParam { date }): Parameters<economy::DateParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .movie_boxoffice_cinema_daily(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get cinema weekly box office data for a given date")]
    async fn movie_boxoffice_cinema_weekly(
        &self,
        Parameters(economy::DateParam { date }): Parameters<economy::DateParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .movie_boxoffice_cinema_weekly(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get daily movie box office data for a given date")]
    async fn movie_boxoffice_daily(
        &self,
        Parameters(economy::DateParam { date }): Parameters<economy::DateParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .movie_boxoffice_daily(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get monthly movie box office data for a given year-month")]
    async fn movie_boxoffice_monthly(
        &self,
        Parameters(economy::DateParam { date }): Parameters<economy::DateParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .movie_boxoffice_monthly(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get weekly movie box office data for a given date")]
    async fn movie_boxoffice_weekly(
        &self,
        Parameters(economy::DateParam { date }): Parameters<economy::DateParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .movie_boxoffice_weekly(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get yearly movie box office data for a given year")]
    async fn movie_boxoffice_yearly(
        &self,
        Parameters(economy::YearParam { year }): Parameters<economy::YearParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .movie_boxoffice_yearly(&year)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get first week box office data for movies in a given year")]
    async fn movie_boxoffice_yearly_first_week(
        &self,
        Parameters(economy::YearParam { year }): Parameters<economy::YearParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .movie_boxoffice_yearly_first_week(&year)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Economy: NLP & Sentiment ───────────────────────────────────

    #[tool(description = "Get financial sentiment and EPU index data")]
    async fn economy_sentiment_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .economy_sentiment_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Query Ownthink knowledge graph for entity information")]
    async fn nlp_ownthink(
        &self,
        Parameters(economy::NlpOwnthinkParam { word, indicator }): Parameters<
            economy::NlpOwnthinkParam,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .nlp_ownthink(&word, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Economy: Entertainment & Media ─────────────────────────────

    #[tool(description = "Get business value artist rankings")]
    async fn business_value_artist(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .business_value_artist()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get online influence artist rankings")]
    async fn online_value_artist(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .online_value_artist()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get TV series ranking data")]
    async fn video_tv(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .video_tv()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get variety show ranking data")]
    async fn video_variety_show(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .video_variety_show()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get TapTap game hot ranking by category")]
    async fn game_hot_rank_taptap(
        &self,
        Parameters(economy::SymbolParam { symbol }): Parameters<economy::SymbolParam>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .game_hot_rank_taptap(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Auto-generated from agent_forex_tools.rs ──
    // ── Forex (additional) ────────────────────────────────────────

    #[tool(description = "Get Sina Finance realtime forex rates for major currency pairs against CNY")]
    async fn forex_sina_rates(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .forex_sina_rates()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get historical forex kline data from Eastmoney with full parameter support")]
    async fn forex_hist_em(
        &self,
        Parameters(forex::ForexHistEmParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<forex::ForexHistEmParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .forex_hist_em(&symbol, &period, &start_date, &end_date, &adjust)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get recent forex kline data from Eastmoney by symbol and limit")]
    async fn forex_em_hist(
        &self,
        Parameters(forex::ForexEmHistParams { symbol, limit }): Parameters<forex::ForexEmHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .forex_em_hist(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Currency (CurrencyBeacon) ─────────────────────────────────

    #[tool(description = "Get latest exchange rates from CurrencyBeacon")]
    async fn currency_latest(
        &self,
        Parameters(forex::CurrencyLatestParams {
            base,
            symbols,
            api_key,
        }): Parameters<forex::CurrencyLatestParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .currency_latest(&base, &symbols, &api_key)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get historical exchange rates from CurrencyBeacon for a specific date")]
    async fn currency_history(
        &self,
        Parameters(forex::CurrencyHistoryParams {
            base,
            date,
            symbols,
            api_key,
        }): Parameters<forex::CurrencyHistoryParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .currency_history(&base, &date, &symbols, &api_key)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get time-series exchange rates from CurrencyBeacon for a date range")]
    async fn currency_time_series(
        &self,
        Parameters(forex::CurrencyTimeSeriesParams {
            base,
            start_date,
            end_date,
            symbols,
            api_key,
        }): Parameters<forex::CurrencyTimeSeriesParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .currency_time_series(&base, &start_date, &end_date, &symbols, &api_key)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "List all supported currencies from CurrencyBeacon")]
    async fn currency_currencies(
        &self,
        Parameters(forex::CurrencyCurrenciesParams { c_type, api_key }): Parameters<
            forex::CurrencyCurrenciesParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .currency_currencies(&c_type, &api_key)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Convert currency via CurrencyBeacon")]
    async fn currency_convert(
        &self,
        Parameters(forex::CurrencyConvertParams {
            from,
            to,
            amount,
            api_key,
        }): Parameters<forex::CurrencyConvertParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .currency_convert(&from, &to, amount, &api_key)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SAFE (State Administration of Foreign Exchange) RMB central parity rates")]
    async fn currency_boc_safe(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .currency_boc_safe()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── FX (China Money / Baidu) ──────────────────────────────────

    #[tool(description = "Get CFETS FX currency pair map from Bank of China data")]
    async fn currency_pair_map(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .currency_pair_map()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFETS FX C-Swap (Currency Swap) benchmark rates from China Money")]
    async fn fx_c_swap_cm(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fx_c_swap_cm()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get FX spot/forward quotes for a specific currency pair from China Money")]
    async fn fx_pair_quote(
        &self,
        Parameters(forex::CurrencyPairParams { pair }): Parameters<forex::CurrencyPairParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fx_pair_quote(&pair)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get FX spot market quotes from China Money")]
    async fn fx_spot_quote(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fx_spot_quote()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get FX swap market quotes from China Money")]
    async fn fx_swap_quote(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fx_swap_quote()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get real-time FX quote from Baidu Finance")]
    async fn fx_quote_baidu(
        &self,
        Parameters(forex::CurrencyPairParams { pair }): Parameters<forex::CurrencyPairParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fx_quote_baidu(&pair)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Auto-generated from agent_fund_tools.rs ──
    // ── Fund MCP Tool Wrappers ─────────────────────────────────────────
    // Auto-generated. Existing tools: fund_etf_hist, fund_manager_em,
    // fund_etf_spot_em, fund_lof_spot_em, fund_open_fund_rank_em,
    // fund_hk_rank_em, fund_hk_fund_hist_em

    // ── Announcement ────────────────────────────────────────────────

    #[tool(description = "Get fund dividend announcements from Eastmoney (fund code e.g. '000001')")]
    async fn fund_announcement_dividend_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_announcement_dividend_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund report announcements from Eastmoney (fund code e.g. '000001')")]
    async fn fund_announcement_report_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_announcement_report_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund personnel announcements from Eastmoney (fund code e.g. '000001')")]
    async fn fund_announcement_personnel_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_announcement_personnel_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── AUM ─────────────────────────────────────────────────────────

    #[tool(description = "Get fund company AUM (Assets Under Management) ranking from Eastmoney")]
    async fn fund_aum_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_aum_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund market AUM trend from Eastmoney")]
    async fn fund_aum_trend_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_aum_trend_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund company historical AUM ranking by year (e.g. '2025')")]
    async fn fund_aum_hist_em(
        &self,
        Parameters(stock::DateParams { date }): Parameters<stock::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_aum_hist_em(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Split (cf) ──────────────────────────────────────────────────

    #[tool(description = "Get fund split data by year (e.g. '2025') from Eastmoney")]
    async fn fund_cf_em(
        &self,
        Parameters(stock::DateParams { date }): Parameters<stock::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_cf_em(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── EM general ──────────────────────────────────────────────────

    #[tool(description = "Get fund purchase/redemption status from Eastmoney")]
    async fn fund_purchase_em(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_purchase_em(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get all fund names and types from Eastmoney")]
    async fn fund_name_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_name_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get index fund info from Eastmoney. Symbol: '全部','沪深指数','行业主题','大盘指数','中盘指数','小盘指数'. Indicator: '全部','被动指数型','增强指数型'")]
    async fn fund_info_index_em(
        &self,
        Parameters(fund::FundInfoIndexParams {
            symbol,
            indicator,
            limit,
        }): Parameters<fund::FundInfoIndexParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_info_index_em(&symbol, &indicator, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── ETF (Eastmoney) ─────────────────────────────────────────────

    #[tool(description = "Get ETF fund daily listing data from Eastmoney")]
    async fn fund_etf_fund_daily_em(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_fund_daily_em(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get ETF historical K-line candles from Eastmoney with full parameters. Period: 'daily','weekly','monthly'. Adjust: 'qfq','hfq',''")]
    async fn fund_etf_hist_em(
        &self,
        Parameters(fund::FundEtfHistFullParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<fund::FundEtfHistFullParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_hist_em(&symbol, &period, &start_date, &end_date, &adjust)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get ETF minute-level historical data from Eastmoney. Period: '1','5','15','30','60'. Adjust: '','qfq','hfq'")]
    async fn fund_etf_hist_min_em(
        &self,
        Parameters(fund::FundEtfHistFullParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<fund::FundEtfHistFullParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_hist_min_em(&symbol, &period, &start_date, &end_date, &adjust)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get ETF fund info (historical NAV) from Eastmoney. Date format: YYYYMMDD")]
    async fn fund_etf_fund_info_em(
        &self,
        Parameters(stock::SymbolDateRangeParams {
            symbol,
            start_date,
            end_date,
        }): Parameters<stock::SymbolDateRangeParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_fund_info_em(&symbol, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE ETF scale data by date (format: YYYYMMDD)")]
    async fn fund_etf_scale_sse(
        &self,
        Parameters(stock::DateParams { date }): Parameters<stock::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_scale_sse(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SZSE ETF scale data")]
    async fn fund_etf_scale_szse(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_scale_szse()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get THS fund category data. Symbol: 'ETF','LOF','股票型','债券型','混合型','QDII','保本型','指数型',''. Date format: YYYYMMDD or empty for latest")]
    async fn fund_etf_category_ths(
        &self,
        Parameters(fund::FundEtfCategoryThsParams { symbol, date }): Parameters<
            fund::FundEtfCategoryThsParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_category_ths(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get THS ETF spot data. Date format: YYYYMMDD or empty for latest")]
    async fn fund_etf_spot_ths(
        &self,
        Parameters(stock::DateParams { date }): Parameters<stock::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_spot_ths(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get ETF dividend data from Sina Finance (symbol with exchange prefix e.g. 'sh510050')")]
    async fn fund_etf_dividend_sina(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_dividend_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── ETF (Sina) ──────────────────────────────────────────────────

    #[tool(description = "Get ETF/LOF/closed fund category list from Sina Finance. Symbol: '封闭式基金','ETF基金','LOF基金'")]
    async fn fund_etf_category_sina(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_category_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get ETF historical data from Sina Finance")]
    async fn fund_etf_hist_sina(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_etf_hist_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Fee ─────────────────────────────────────────────────────────

    #[tool(description = "Get fund fee data from Eastmoney")]
    async fn fund_fee_em(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_fee_em(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Dividend (fhsp) ─────────────────────────────────────────────

    #[tool(description = "Get fund dividend data by year (e.g. '2025') from Eastmoney")]
    async fn fund_fh_em(
        &self,
        Parameters(stock::DateParams { date }): Parameters<stock::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_fh_em(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund dividend ranking from Eastmoney")]
    async fn fund_fh_rank_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_fh_rank_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Financial fund ──────────────────────────────────────────────

    #[tool(description = "Get financial fund daily data from Eastmoney")]
    async fn fund_financial_fund_daily_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_financial_fund_daily_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get financial fund info (historical NAV) from Eastmoney")]
    async fn fund_financial_fund_info_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_financial_fund_info_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Graded fund ─────────────────────────────────────────────────

    #[tool(description = "Get graded fund daily snapshot from Eastmoney")]
    async fn fund_graded(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_graded(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get graded fund daily data from Eastmoney")]
    async fn fund_graded_fund_daily_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_graded_fund_daily_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get graded fund info (historical NAV) from Eastmoney")]
    async fn fund_graded_fund_info_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_graded_fund_info_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Holder structure ────────────────────────────────────────────

    #[tool(description = "Get fund holder structure data from Eastmoney")]
    async fn fund_hold_structure_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_hold_structure_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Info THS ────────────────────────────────────────────────────

    #[tool(description = "Get fund info from THS (同花顺)")]
    async fn fund_info_ths(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_info_ths(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── New fund (init) ─────────────────────────────────────────────

    #[tool(description = "Get newly established funds from Eastmoney")]
    async fn fund_new_found_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_new_found_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get new funds from THS (同花顺). Symbol: '发行中' or '将发行'")]
    async fn fund_new_found_ths(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_new_found_ths(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── LCX (financial ranking) ─────────────────────────────────────

    #[tool(description = "Get financial fund ranking from Eastmoney")]
    async fn fund_lcx_rank_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_lcx_rank_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── LOF ─────────────────────────────────────────────────────────

    #[tool(description = "Get LOF fund list from Eastmoney")]
    async fn fund_lof_list(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_lof_list(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get LOF fund historical K-line candles")]
    async fn fund_lof_hist(
        &self,
        Parameters(fund::FundHistParams { symbol, limit }): Parameters<fund::FundHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_lof_hist(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get LOF historical K-line candles from Eastmoney with full parameters. Period: 'daily','weekly','monthly'. Adjust: 'qfq','hfq',''")]
    async fn fund_lof_hist_em(
        &self,
        Parameters(fund::FundEtfHistFullParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<fund::FundEtfHistFullParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_lof_hist_em(&symbol, &period, &start_date, &end_date, &adjust)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get LOF minute-level historical data from Eastmoney. Period: '1','5','15','30','60'. Adjust: '','qfq','hfq'")]
    async fn fund_lof_hist_min_em(
        &self,
        Parameters(fund::FundEtfHistFullParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<fund::FundEtfHistFullParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_lof_hist_min_em(&symbol, &period, &start_date, &end_date, &adjust)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── LOF THS ─────────────────────────────────────────────────────

    #[tool(description = "Get LOF fund data from THS (同花顺)")]
    async fn fund_lof_ths(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_lof_ths(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Money market ────────────────────────────────────────────────

    #[tool(description = "Get money market fund rankings from Eastmoney")]
    async fn fund_money_market(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_money_market(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get money fund daily data from Eastmoney")]
    async fn fund_money_fund_daily_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_money_fund_daily_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get money fund info (historical NAV) from Eastmoney")]
    async fn fund_money_fund_info_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_money_fund_info_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get money fund ranking from Eastmoney")]
    async fn fund_money_rank_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_money_rank_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Open-end fund ───────────────────────────────────────────────

    #[tool(description = "Get daily NAV snapshot for all open-end funds from Eastmoney")]
    async fn fund_open_end_daily(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_open_end_daily(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get NAV history for a specific open-end fund")]
    async fn fund_open_end_nav(
        &self,
        Parameters(fund::FundHistParams { symbol, limit }): Parameters<fund::FundHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_open_end_nav(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get all open-end fund daily NAV data from Eastmoney")]
    async fn fund_open_fund_daily_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_open_fund_daily_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get open fund info (NAV history) from Eastmoney. Indicator: '单位净值走势','累计净值走势',etc.")]
    async fn fund_open_fund_info_em(
        &self,
        Parameters(fund::FundOpenFundInfoParams {
            symbol,
            start_date,
            end_date,
            indicator,
        }): Parameters<fund::FundOpenFundInfoParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_open_fund_info_em(&symbol, &start_date, &end_date, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Overview ────────────────────────────────────────────────────

    #[tool(description = "Get fund overview from Eastmoney")]
    async fn fund_overview_em(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_overview_em(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Portfolio ───────────────────────────────────────────────────

    #[tool(description = "Get fund portfolio holdings from Eastmoney")]
    async fn fund_portfolio_hold_em(
        &self,
        Parameters(stock::SymbolDateParams { symbol, date }): Parameters<stock::SymbolDateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_portfolio_hold_em(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund bond holdings from Eastmoney")]
    async fn fund_portfolio_bond_hold_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_portfolio_bond_hold_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund asset allocation from Eastmoney")]
    async fn fund_portfolio_asset_allocation_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_portfolio_asset_allocation_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund portfolio industry allocation from Eastmoney")]
    async fn fund_portfolio_industry_allocation_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_portfolio_industry_allocation_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund portfolio major changes. Indicator: '累计买入' or '累计卖出'")]
    async fn fund_portfolio_change_em(
        &self,
        Parameters(stock::SymbolIndicatorParams { symbol, indicator }): Parameters<
            stock::SymbolIndicatorParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_portfolio_change_em(&symbol, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Position (Legu) ─────────────────────────────────────────────

    #[tool(description = "Get fund position estimates from Legu (乐咕)")]
    async fn fund_position_lg(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_position_lg(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund position estimate history from Legu (乐咕)")]
    async fn fund_position_hist_lg(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_position_hist_lg(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund position estimate summary from Legu (乐咕)")]
    async fn fund_position_est_lg(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_position_est_lg()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get stock-type fund position from Legu (乐咕)")]
    async fn fund_stock_position_lg(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_stock_position_lg()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get balanced fund position from Legu (乐咕)")]
    async fn fund_balance_position_lg(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_balance_position_lg()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get flexible allocation fund position from Legu (乐咕)")]
    async fn fund_linghuo_position_lg(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_linghuo_position_lg()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── QDII (Jisilu) ───────────────────────────────────────────────

    #[tool(description = "Get Jisilu T+0 QDII Asian market index funds. Cookie is optional for authenticated access")]
    async fn qdii_a_index_jsl(
        &self,
        Parameters(fund::FundQdiiCookieParams { cookie }): Parameters<fund::FundQdiiCookieParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .qdii_a_index_jsl(&cookie)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Jisilu T+0 QDII European/American market index funds. Cookie is optional for authenticated access")]
    async fn qdii_e_index_jsl(
        &self,
        Parameters(fund::FundQdiiCookieParams { cookie }): Parameters<fund::FundQdiiCookieParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .qdii_e_index_jsl(&cookie)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Jisilu T+0 QDII European/American commodity funds. Cookie is optional for authenticated access")]
    async fn qdii_e_comm_jsl(
        &self,
        Parameters(fund::FundQdiiCookieParams { cookie }): Parameters<fund::FundQdiiCookieParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .qdii_e_comm_jsl(&cookie)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Rank (exchange) ─────────────────────────────────────────────

    #[tool(description = "Get exchange fund ranking from Eastmoney")]
    async fn fund_exchange_rank_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_exchange_rank_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Rating ──────────────────────────────────────────────────────

    #[tool(description = "Get fund ratings from Eastmoney")]
    async fn fund_rating_em(
        &self,
        Parameters(stock::LimitParams { limit }): Parameters<stock::LimitParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_rating_em(limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund ratings from Zhaoshang (招商)")]
    async fn fund_rating_zs(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_rating_zs()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund ratings from Tiantian (天天基金)")]
    async fn fund_rating_tiantian(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_rating_tiantian()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund ratings from JiShi (济安金信)")]
    async fn fund_rating_jiashi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_rating_jiashi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get all fund ratings summary")]
    async fn fund_rating_all(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_rating_all()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shanghai Securities fund ratings by date")]
    async fn fund_rating_sh(
        &self,
        Parameters(stock::DateParams { date }): Parameters<stock::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_rating_sh(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Ji'an Jinxin fund ratings by date")]
    async fn fund_rating_ja(
        &self,
        Parameters(stock::DateParams { date }): Parameters<stock::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_rating_ja(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Report (CNINFO) ─────────────────────────────────────────────

    #[tool(description = "Get fund annual report data from CNINFO (巨潮资讯)")]
    async fn fund_report_cninfo(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_report_cninfo(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund semi-annual report data from CNINFO (巨潮资讯)")]
    async fn fund_report_half_year_cninfo(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_report_half_year_cninfo(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund quarterly report data from CNINFO (巨潮资讯)")]
    async fn fund_report_quarter_cninfo(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_report_quarter_cninfo(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund heavy stock holdings from CNINFO by date")]
    async fn fund_report_stock_cninfo(
        &self,
        Parameters(stock::DateParams { date }): Parameters<stock::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_report_stock_cninfo(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund industry allocation from CNINFO by date")]
    async fn fund_report_industry_allocation_cninfo(
        &self,
        Parameters(stock::DateParams { date }): Parameters<stock::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_report_industry_allocation_cninfo(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund asset allocation from CNINFO")]
    async fn fund_report_asset_allocation_cninfo(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_report_asset_allocation_cninfo()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Scale ───────────────────────────────────────────────────────

    #[tool(description = "Get fund scale change data from Eastmoney")]
    async fn fund_scale_change_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_scale_change_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get open-end fund scale data from Sina Finance. Symbol: '股票型基金','混合型基金','债券型基金','货币型基金','QDII基金'")]
    async fn fund_scale_open_sina(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_scale_open_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get closed-end fund scale data from Sina Finance")]
    async fn fund_scale_close_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_scale_close_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get money market fund scale data from Sina Finance")]
    async fn fund_scale_money_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_scale_money_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get structured fund scale data from Sina Finance")]
    async fn fund_scale_structured_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_scale_structured_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SZSE fund scale daily data. Symbol: 'ETF','LOF','REITS'. Date format: YYYYMMDD")]
    async fn fund_scale_daily_szse(
        &self,
        Parameters(stock::SymbolDateRangeParams {
            symbol,
            start_date,
            end_date,
        }): Parameters<stock::SymbolDateRangeParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_scale_daily_szse(&start_date, &end_date, &symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Value estimation ────────────────────────────────────────────

    #[tool(description = "Get fund value estimation from Eastmoney. Symbol: '全部','股票型','混合型','债券型','指数型','QDII','ETF联接','LOF','场内交易基金'")]
    async fn fund_value_estimation_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_value_estimation_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Xueqiu / Danjuan ────────────────────────────────────────────

    #[tool(description = "Get fund info from Xueqiu/Danjuan (蛋卷基金)")]
    async fn fund_xueqiu_info(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_xueqiu_info(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund performance/achievement data from Xueqiu/Danjuan")]
    async fn fund_xueqiu_achievement(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_xueqiu_achievement(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund basic info from Xueqiu/Danjuan (基金基本资料)")]
    async fn fund_individual_basic_info_xq(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_individual_basic_info_xq(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund achievement (annual + stage performance) from Xueqiu/Danjuan")]
    async fn fund_individual_achievement_xq(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_individual_achievement_xq(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund analysis (risk/return metrics) from Xueqiu/Danjuan")]
    async fn fund_individual_analysis_xq(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_individual_analysis_xq(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund profit probability by holding period from Xueqiu/Danjuan")]
    async fn fund_individual_profit_probability_xq(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_individual_profit_probability_xq(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund detail info (trading rules) from Xueqiu/Danjuan")]
    async fn fund_individual_detail_info_xq(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_individual_detail_info_xq(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get fund detail holdings (asset allocation) from Xueqiu/Danjuan. Date format: YYYYMMDD")]
    async fn fund_individual_detail_hold_xq(
        &self,
        Parameters(stock::SymbolDateParams { symbol, date }): Parameters<stock::SymbolDateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .fund_individual_detail_hold_xq(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Auto-generated from agent_futures_tools.rs ──
    // ── Futures (basis) ──────────────────────────────────────────────

    #[tool(description = "Get futures spot price and basis data from 100ppi.com for a given date")]
    async fn futures_spot_price(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_spot_price(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get futures spot price daily range basis data from 100ppi.com")]
    async fn futures_spot_price_daily(
        &self,
        Parameters(futures::FuturesDateRangeParams { start_date, end_date }): Parameters<
            futures::FuturesDateRangeParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_spot_price_daily(&start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get historical spot price and basis from 100ppi.com sf2 format")]
    async fn futures_spot_price_previous(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_spot_price_previous(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (comex) ─────────────────────────────────────────────

    #[tool(description = "Get COMEX gold or silver inventory data from Eastmoney")]
    async fn futures_comex_inventory(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_comex_inventory(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (commission) ────────────────────────────────────────

    #[tool(description = "Get futures fee reference table from openctp")]
    async fn futures_fees_info_openctp(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_fees_info_openctp()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get futures commission data from Jin10 for a given date")]
    async fn futures_comm_js(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_comm_js(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get futures fee information from 9qihuo for a given symbol")]
    async fn futures_fees_info(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_fees_info(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get futures commission info from 9qihuo for a given exchange")]
    async fn futures_comm_info(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_comm_info(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (contract detail) ───────────────────────────────────

    #[tool(description = "Get futures contract detail from Sina Finance")]
    async fn futures_contract_detail_sina(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_contract_detail_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get futures contract detail from Eastmoney")]
    async fn futures_contract_detail_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_contract_detail_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get futures contract detail (unified entry point)")]
    async fn futures_contract_detail(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_contract_detail(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get main contract for a given futures variety")]
    async fn match_main_contract(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .match_main_contract(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (position rank) ─────────────────────────────────────

    #[tool(description = "Get CZCE futures position rank data")]
    async fn futures_czce_position_rank(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_czce_position_rank(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX futures position rank data")]
    async fn futures_cffex_position_rank(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_cffex_position_rank(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE futures position rank data")]
    async fn futures_dce_position_rank(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_dce_position_rank(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get GFEX futures position rank data")]
    async fn futures_gfex_position_rank(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_gfex_position_rank(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE position rank for other categories")]
    async fn futures_dce_position_rank_other(
        &self,
        Parameters(futures::FuturesDateSymbolParams { date, symbol }): Parameters<
            futures::FuturesDateSymbolParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_dce_position_rank_other(&date, &symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Sina futures position data for a single contract")]
    async fn futures_hold_pos_sina(
        &self,
        Parameters(futures::FuturesHoldPosParams { data_type, contract, date }): Parameters<
            futures::FuturesHoldPosParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_hold_pos_sina(&data_type, &contract, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (daily bar) ─────────────────────────────────────────

    #[tool(description = "Get INE daily futures data")]
    async fn futures_daily_ine(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_daily_ine(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE daily futures data")]
    async fn futures_daily_dce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_daily_dce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CZCE daily futures data")]
    async fn futures_daily_czce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_daily_czce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get GFEX daily futures data")]
    async fn futures_daily_gfex(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_daily_gfex(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get futures daily data from a specific exchange")]
    async fn get_futures_daily(
        &self,
        Parameters(futures::FuturesDateMarketParams { date, market }): Parameters<
            futures::FuturesDateMarketParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_futures_daily(&date, &market)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (delivery) ──────────────────────────────────────────

    #[tool(description = "Get SHFE futures-to-spot (期转现) data")]
    async fn futures_to_spot_shfe(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_to_spot_shfe(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE delivery statistics by month")]
    async fn futures_delivery_shfe(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_delivery_shfe(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE delivery statistics")]
    async fn futures_delivery_dce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_delivery_dce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE futures-to-spot (期转现) data")]
    async fn futures_to_spot_dce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_to_spot_dce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CZCE delivery matching data")]
    async fn futures_delivery_match_czce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_delivery_match_czce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CZCE monthly delivery data")]
    async fn futures_delivery_czce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_delivery_czce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE delivery match data")]
    async fn futures_delivery_match_dce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_delivery_match_dce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CZCE futures-to-spot statistics")]
    async fn futures_to_spot_czce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_to_spot_czce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (derivative - contract info) ────────────────────────

    #[tool(description = "Get CFFEX contract trading parameters")]
    async fn futures_contract_info_cffex(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_contract_info_cffex(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CZCE contract reference data")]
    async fn futures_contract_info_czce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_contract_info_czce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE futures contract information")]
    async fn futures_contract_info_dce(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_contract_info_dce()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get GFEX futures contract information")]
    async fn futures_contract_info_gfex(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_contract_info_gfex()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get INE futures contract base info")]
    async fn futures_contract_info_ine(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_contract_info_ine(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE futures contract base info")]
    async fn futures_contract_info_shfe(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_contract_info_shfe(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (derivative - hog data) ─────────────────────────────

    #[tool(description = "Get hog core price data from Zhuwang")]
    async fn futures_hog_core(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_hog_core(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get hog cost dimension data from Zhuwang")]
    async fn futures_hog_cost(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_hog_cost(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get hog supply dimension data from Zhuwang")]
    async fn futures_hog_supply(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_hog_supply(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (derivative - sina main) ────────────────────────────

    #[tool(description = "Get all main continuous contracts from Sina across exchanges")]
    async fn futures_display_main_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_display_main_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get main contract daily data from Sina with date filtering")]
    async fn futures_main_sina_derivative(
        &self,
        Parameters(futures::FuturesMainSinaDerivativeParams {
            symbol,
            start_date,
            end_date,
        }): Parameters<futures::FuturesMainSinaDerivativeParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_main_sina_derivative(&symbol, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get spot-futures comparison data from 100ppi.com")]
    async fn futures_spot_sys(
        &self,
        Parameters(futures::FuturesSymbolIndicatorParams { symbol, indicator }): Parameters<
            futures::FuturesSymbolIndicatorParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_spot_sys(&symbol, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (exchange wrappers) ─────────────────────────────────

    #[tool(description = "Get CFFEX daily bar data (exchange wrapper)")]
    async fn get_cffex_daily(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_cffex_daily(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX rank table (position ranking)")]
    async fn get_cffex_rank_table(
        &self,
        Parameters(futures::FuturesDateSymbolParams { date, symbol }): Parameters<
            futures::FuturesDateSymbolParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_cffex_rank_table(&date, &symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CZCE daily bar data (exchange wrapper)")]
    async fn get_czce_daily(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_czce_daily(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE daily bar data (exchange wrapper)")]
    async fn get_dce_daily(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_dce_daily(&date, None)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE rank table (position ranking)")]
    async fn get_dce_rank_table(
        &self,
        Parameters(futures::FuturesDateSymbolParams { date, symbol }): Parameters<
            futures::FuturesDateSymbolParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_dce_rank_table(&date, &symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get GFEX daily bar data (exchange wrapper)")]
    async fn get_gfex_daily(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_gfex_daily(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get INE daily bar data (exchange wrapper)")]
    async fn get_ine_daily(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_ine_daily(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE daily bar data (exchange wrapper)")]
    async fn get_shfe_daily(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_shfe_daily(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE rank table (position ranking)")]
    async fn get_shfe_rank_table(
        &self,
        Parameters(futures::FuturesDateSymbolParams { date, symbol }): Parameters<
            futures::FuturesDateSymbolParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_shfe_rank_table(&date, &symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CZCE rank table (position ranking)")]
    async fn get_rank_table_czce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_rank_table_czce(&date, None)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get roll yield bar cross-section for all varieties")]
    async fn get_roll_yield_bar(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_roll_yield_bar(&date, None, None, None)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get aggregated position ranking sum across exchanges")]
    async fn get_rank_sum(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_rank_sum(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get daily aggregated position ranking data")]
    async fn get_rank_sum_daily(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_rank_sum_daily(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get warehouse receipt data from DCE and SHFE")]
    async fn get_receipt(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_receipt(&date, None)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (foreign) ───────────────────────────────────────────

    #[tool(description = "Get foreign futures historical daily kline from Sina")]
    async fn futures_foreign_hist(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_foreign_hist(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get QHKC fund buy/sell data")]
    async fn get_qhkc_fund_bs(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_qhkc_fund_bs(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get QHKC fund money change data")]
    async fn get_qhkc_fund_money_change(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_qhkc_fund_money_change(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get QHKC fund position data")]
    async fn get_qhkc_fund_position(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_qhkc_fund_position(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get QHKC index data")]
    async fn get_qhkc_index(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_qhkc_index(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get QHKC index profit/loss data")]
    async fn get_qhkc_index_profit_loss(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_qhkc_index_profit_loss(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get QHKC index trend data")]
    async fn get_qhkc_index_trend(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_qhkc_index_trend(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get QHKC foreign commodity tool data")]
    async fn qhkc_tool_foreign(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .qhkc_tool_foreign()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get QHKC GDP tool data")]
    async fn qhkc_tool_gdp(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .qhkc_tool_gdp()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get foreign futures contract detail from Sina")]
    async fn futures_foreign_detail(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_foreign_detail(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (global - hf_em) ────────────────────────────────────

    #[tool(description = "Get global futures realtime quotes from Eastmoney")]
    async fn futures_global_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_global_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get global futures historical kline data from Eastmoney")]
    async fn futures_global_hist_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_global_hist_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (hist_em) ───────────────────────────────────────────

    #[tool(description = "Get Eastmoney futures market table (品种对照表)")]
    async fn futures_hist_table_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_hist_table_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Eastmoney futures historical kline data")]
    async fn futures_hist_em(
        &self,
        Parameters(futures::FuturesHistEmParams {
            symbol,
            period,
            start_date,
            end_date,
        }): Parameters<futures::FuturesHistEmParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_hist_em(&symbol, &period, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (hq_sina) ───────────────────────────────────────────

    #[tool(description = "Get foreign commodity subscribe exchange symbol list")]
    async fn futures_foreign_commodity_subscribe_exchange_symbol(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_foreign_commodity_subscribe_exchange_symbol();
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get foreign commodity realtime quotes from Sina")]
    async fn futures_foreign_commodity_realtime(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let symbols: Vec<&str> = symbol.split(',').map(str::trim).collect();
        let data = self
            .client
            .futures_foreign_commodity_realtime(&symbols)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (index) ─────────────────────────────────────────────

    #[tool(description = "Get CCIDX futures commodity index from 中证商品指数")]
    async fn futures_index_ccidx(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_index_ccidx(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (inventory) ─────────────────────────────────────────

    #[tool(description = "Get Eastmoney futures inventory data")]
    async fn futures_inventory_em(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_inventory_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get 99qh futures inventory data")]
    async fn futures_inventory_99(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_inventory_99(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (news) ──────────────────────────────────────────────

    #[tool(description = "Get SHMET news flash from Shanghai Metals Market")]
    async fn futures_news_shmet(
        &self,
        Parameters(futures::FuturesCategoryParams { category }): Parameters<
            futures::FuturesCategoryParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_news_shmet(&category)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (receipt) ───────────────────────────────────────────

    #[tool(description = "Get DCE registered warehouse receipt data")]
    async fn get_dce_receipt(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_dce_receipt(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE registered warehouse receipt data")]
    async fn get_shfe_receipt(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_shfe_receipt(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (roll yield) ────────────────────────────────────────

    #[tool(description = "Get roll yield between two futures contracts")]
    async fn get_roll_yield(
        &self,
        Parameters(futures::FuturesDateSymbolParams { date, symbol }): Parameters<
            futures::FuturesDateSymbolParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .get_roll_yield(&date, &symbol, None, None)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get roll yield cross-section for all varieties on a given date")]
    async fn futures_roll_yield_bar(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_roll_yield_bar(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (rule) ──────────────────────────────────────────────

    #[tool(description = "Get Guotai Junan Futures trading calendar and rules")]
    async fn futures_rule_gtja(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_rule_gtja(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get futures trading rules (unified entry point)")]
    async fn futures_rule(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_rule(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Eastmoney futures trading rules")]
    async fn futures_rule_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_rule_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (settle) ────────────────────────────────────────────

    #[tool(description = "Get CFFEX settlement parameters")]
    async fn futures_settle_cffex(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_settle_cffex(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CZCE settlement parameters")]
    async fn futures_settle_czce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_settle_czce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE settlement parameters")]
    async fn futures_settle_shfe(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_settle_shfe(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get INE settlement parameters")]
    async fn futures_settle_ine(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_settle_ine(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get GFEX settlement parameters")]
    async fn futures_settle_gfex(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_settle_gfex(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE settlement parameters for a given symbol")]
    async fn futures_stock_shfe_js(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_stock_shfe_js(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get unified settlement data across exchanges")]
    async fn futures_settle(
        &self,
        Parameters(futures::FuturesDateMarketParams { date, market }): Parameters<
            futures::FuturesDateMarketParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_settle(&date, &market)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (sgx) ───────────────────────────────────────────────

    #[tool(description = "Get SGX derivatives historical settlement prices")]
    async fn futures_settlement_price_sgx(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_settlement_price_sgx(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (sina) ──────────────────────────────────────────────

    #[tool(description = "Get futures symbol-to-mark mapping from Sina")]
    async fn futures_symbol_mark(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_symbol_mark()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get realtime quotes for all contracts of a futures variety from Sina")]
    async fn futures_zh_realtime(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_zh_realtime(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get realtime spot quotes for specific futures contracts from Sina")]
    async fn futures_zh_spot(
        &self,
        Parameters(futures::FuturesSymbolMarketParams { symbols, market }): Parameters<
            futures::FuturesSymbolMarketParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_zh_spot(&symbols, &market)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get minute-frequency kline data for a futures contract from Sina")]
    async fn futures_zh_minute_sina(
        &self,
        Parameters(futures::FuturesSymbolPeriodKlineParams { symbol, period }): Parameters<
            futures::FuturesSymbolPeriodKlineParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_zh_minute_sina(&symbol, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get daily kline data for a specific futures contract from Sina")]
    async fn futures_zh_daily_sina(
        &self,
        Parameters(stock::SymbolParams { symbol }): Parameters<stock::SymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_zh_daily_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (spot_stock) ────────────────────────────────────────

    #[tool(description = "Get futures spot-to-stock mapping data")]
    async fn futures_spot_stock(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_spot_stock(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Eastmoney spot-to-stock mapping data by category")]
    async fn futures_spot_stock_em(
        &self,
        Parameters(futures::FuturesCategoryParams { category }): Parameters<
            futures::FuturesCategoryParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_spot_stock_em(&category)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (warehouse) ─────────────────────────────────────────

    #[tool(description = "Get CZCE warehouse receipt data")]
    async fn futures_warehouse_receipt_czce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_warehouse_receipt_czce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE warehouse receipt data")]
    async fn futures_warehouse_receipt_dce(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_warehouse_receipt_dce(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE warehouse receipt data")]
    async fn futures_shfe_warehouse_receipt(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_shfe_warehouse_receipt(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get GFEX warehouse receipt data")]
    async fn futures_gfex_warehouse_receipt(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_gfex_warehouse_receipt(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Futures (hist - CFFEX historical) ───────────────────────────

    #[tool(description = "Get CFFEX historical daily data")]
    async fn futures_hist_daily_cffex(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .futures_hist_daily_cffex(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Auto-generated from agent_index_tools.rs ──
// ── Index MCP Tool Wrappers ─────────────────────────────────────────
// Generated for all unwrapped pub async fn methods in
// crates/akshare/src/index/

// ── a_share.rs ──────────────────────────────────────────────────────

    #[tool(description = "Get A-share index candles from Eastmoney")]
    async fn index_a_share_candles(
        &self,
        Parameters(index::IndexCandlesParams { symbol, limit }): Parameters<
            index::IndexCandlesParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_a_share_candles(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share index real-time spot from Eastmoney by series")]
    async fn index_stock_zh_spot_em(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_stock_zh_spot_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share index real-time spot from Sina")]
    async fn index_stock_zh_spot_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_stock_zh_spot_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share index daily K-line from Sina (requires JS decoding, may return error)")]
    async fn stock_zh_index_daily(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_index_daily(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── cflp.rs ─────────────────────────────────────────────────────────

    #[tool(description = "Get CFLP road logistics price index. Symbol: '周指数','月指数','季度指数','年度指数'")]
    async fn index_price_cflp(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_price_cflp(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFLP road logistics volume index. Symbol: '月指数','季度指数','年度指数'")]
    async fn index_volume_cflp(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_volume_cflp(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── cni.rs ──────────────────────────────────────────────────────────

    #[tool(description = "Get all CNIndex (国证指数) indices for the latest trading day")]
    async fn index_all_cni(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_all_cni()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CNIndex historical data with date range")]
    async fn index_hist_cni(
        &self,
        Parameters(index::IndexHistParams {
            symbol,
            start_date,
            end_date,
        }): Parameters<index::IndexHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_hist_cni(&symbol, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CNIndex sample detail (constituents). Returns XLS error for most indices")]
    async fn index_detail_cni(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_detail_cni(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CNIndex detailed historical data (alias for index_hist_cni)")]
    async fn index_detail_hist_cni(
        &self,
        Parameters(index::IndexHistParams {
            symbol,
            start_date,
            end_date,
        }): Parameters<index::IndexHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_detail_hist_cni(&symbol, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CNIndex historical adjustment records. Returns XLS error for most indices")]
    async fn index_detail_hist_adjust_cni(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_detail_hist_adjust_cni(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── cons.rs ─────────────────────────────────────────────────────────

    #[tool(description = "Get index constituent stocks from Sina (new API)")]
    async fn index_stock_cons_sina(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_stock_cons_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get index constituent stocks from CSIndex (returns XLS error)")]
    async fn index_stock_cons_csindex(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_stock_cons_csindex(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get index constituent stock weights from CSIndex (returns XLS error)")]
    async fn index_stock_cons_weight_csindex(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_stock_cons_weight_csindex(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── csindex.rs ──────────────────────────────────────────────────────

    #[tool(description = "Get list of all CSIndex (中证指数) indices")]
    async fn index_csindex_all(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_csindex_all()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── cx.rs (19 Caixin innovation indices) ────────────────────────────

    #[tool(description = "Get Caixin composite PMI index")]
    async fn index_pmi_com_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_pmi_com_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin manufacturing PMI index")]
    async fn index_pmi_man_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_pmi_man_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin services PMI index")]
    async fn index_pmi_ser_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_pmi_ser_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin digital economy index")]
    async fn index_dei_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_dei_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin industry index")]
    async fn index_ii_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_ii_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin spillover index")]
    async fn index_si_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_si_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin fusion index")]
    async fn index_fi_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_fi_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin foundation index")]
    async fn index_bi_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_bi_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin China new economy index")]
    async fn index_nei_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_nei_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin labor input index")]
    async fn index_li_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_li_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin capital input index")]
    async fn index_ci_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_ci_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin technology input index")]
    async fn index_ti_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_ti_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin new economy average entry wage index")]
    async fn index_neaw_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_neaw_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin new economy entry wage premium index")]
    async fn index_awpr_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_awpr_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin commodity index")]
    async fn index_cci_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_cci_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin quality factor index")]
    async fn index_qli_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_qli_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin AI strategy index")]
    async fn index_ai_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_ai_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin cornerstone economy index")]
    async fn index_bei_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_bei_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Caixin new momentum index")]
    async fn index_neei_cx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_neei_cx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── drewry.rs ───────────────────────────────────────────────────────

    #[tool(description = "Get Drewry World Container Index (stub, requires HTML scraping)")]
    async fn drewry_wci_index(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .drewry_wci_index(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── eri.rs ──────────────────────────────────────────────────────────

    #[tool(description = "Get Zhejiang emission rights trading index. Symbol: '月度' or '季度'")]
    async fn index_eri(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_eri(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── global.rs ───────────────────────────────────────────────────────

    #[tool(description = "Get global index name-to-symbol mapping from Yahoo Finance")]
    async fn index_global_name_table_yahoo(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_global_name_table_yahoo()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── global_em.rs ────────────────────────────────────────────────────

    #[tool(description = "Get global index real-time spot from Eastmoney")]
    async fn index_global_spot_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_global_spot_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get global index historical K-line from Eastmoney")]
    async fn index_global_hist_em(
        &self,
        Parameters(index::IndexHkDailyParams {
            symbol,
            internal_id,
            limit,
        }): Parameters<index::IndexHkDailyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_global_hist_em(&symbol, &internal_id, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── global_sina.rs ──────────────────────────────────────────────────

    #[tool(description = "Get global index name-to-code mapping from Sina")]
    async fn index_global_name_table(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_global_name_table();
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get global index historical data from Sina")]
    async fn index_global_hist_sina(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_global_hist_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── hf.rs ───────────────────────────────────────────────────────────

    #[tool(description = "Get high-frequency S&P 500 minute data for a given year (2012-2018)")]
    async fn hf_sp_500(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .hf_sp_500(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── hog.rs ──────────────────────────────────────────────────────────

    #[tool(description = "Get hog (生猪) spot price index")]
    async fn index_hog_spot_price(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_hog_spot_price()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── kq_fz.rs ────────────────────────────────────────────────────────

    #[tool(description = "Get Keqiao textile index. Symbol: '价格指数','景气指数','外贸指数'")]
    async fn index_kq_fz(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_kq_fz(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── kq_ss.rs ────────────────────────────────────────────────────────

    #[tool(description = "Get Keqiao fashion index and sub-indices")]
    async fn index_kq_fashion(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_kq_fashion(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── qvix.rs (18 option volatility indices) ──────────────────────────

    #[tool(description = "Get 50ETF option QVIX volatility index (daily)")]
    async fn index_option_50etf_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_50etf_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get 300ETF option QVIX volatility index (daily)")]
    async fn index_option_300etf_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_300etf_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get 500ETF option QVIX volatility index (daily)")]
    async fn index_option_500etf_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_500etf_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get ChiNext (创业板) option QVIX volatility index (daily)")]
    async fn index_option_cyb_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_cyb_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get STAR Market (科创板) option QVIX volatility index (daily)")]
    async fn index_option_kcb_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_kcb_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SZSE 100ETF option QVIX volatility index (daily)")]
    async fn index_option_100etf_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_100etf_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CSI 300 stock index option QVIX volatility index (daily)")]
    async fn index_option_300index_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_300index_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CSI 1000 stock index option QVIX volatility index (daily)")]
    async fn index_option_1000index_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_1000index_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE 50 stock index option QVIX volatility index (daily)")]
    async fn index_option_50index_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_50index_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get 50ETF option QVIX volatility index (intraday)")]
    async fn index_option_50etf_min_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_50etf_min_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get 300ETF option QVIX volatility index (intraday)")]
    async fn index_option_300etf_min_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_300etf_min_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get 500ETF option QVIX volatility index (intraday)")]
    async fn index_option_500etf_min_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_500etf_min_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get ChiNext (创业板) option QVIX volatility index (intraday)")]
    async fn index_option_cyb_min_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_cyb_min_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get STAR Market (科创板) option QVIX volatility index (intraday)")]
    async fn index_option_kcb_min_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_kcb_min_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SZSE 100ETF option QVIX volatility index (intraday)")]
    async fn index_option_100etf_min_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_100etf_min_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CSI 300 stock index option QVIX volatility index (intraday)")]
    async fn index_option_300index_min_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_300index_min_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CSI 1000 stock index option QVIX volatility index (intraday)")]
    async fn index_option_1000index_min_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_1000index_min_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE 50 stock index option QVIX volatility index (intraday)")]
    async fn index_option_50index_min_qvix(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_option_50index_min_qvix()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── scope.rs ────────────────────────────────────────────────────────

    #[tool(description = "Get Chinascope A-share news sentiment index")]
    async fn index_news_sentiment_scope(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_news_sentiment_scope()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── spot.rs ─────────────────────────────────────────────────────────

    #[tool(description = "Get spot goods price index from Sina. Symbol: '波罗的海干散货指数','钢坯价格指数','澳大利亚粉矿价格'")]
    async fn spot_goods(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .spot_goods(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── sugar.rs ────────────────────────────────────────────────────────

    #[tool(description = "Get Msweet China sugar composite price index")]
    async fn index_sugar_msweet(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_sugar_msweet()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Msweet inner-quota import sugar estimate index")]
    async fn index_inner_quote_sugar_msweet(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_inner_quote_sugar_msweet()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Msweet outer-quota import sugar estimate index")]
    async fn index_outer_quote_sugar_msweet(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_outer_quote_sugar_msweet()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── sw.rs ───────────────────────────────────────────────────────────

    #[tool(description = "Get Shenwan industry index daily candles")]
    async fn sw_index_candles(
        &self,
        Parameters(index::IndexCandlesParams { symbol, limit }): Parameters<
            index::IndexCandlesParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .sw_index_candles(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan Level-1 industry index info")]
    async fn sw_index_first_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .sw_index_first_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan Level-2 industry index info")]
    async fn sw_index_second_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .sw_index_second_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan Level-3 industry index constituents")]
    async fn sw_index_third_cons(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .sw_index_third_cons(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan Level-3 industry index info")]
    async fn sw_index_third_info(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .sw_index_third_info(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan Level-1 industry index snapshot with latest prices")]
    async fn sw_index_list(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .sw_index_list()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── sw_fund.rs ──────────────────────────────────────────────────────

    #[tool(description = "Get Shenwan fund index real-time data. Symbol: '基础一级','基础二级','基础三级','特色指数'")]
    async fn index_realtime_fund_sw(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_realtime_fund_sw(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan fund index historical data. Period: 'day','week','month'")]
    async fn index_hist_fund_sw(
        &self,
        Parameters(index::IndexPeriodParams { symbol, period }): Parameters<index::IndexPeriodParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_hist_fund_sw(&symbol, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── sw_research.rs ──────────────────────────────────────────────────

    #[tool(description = "Get Shenwan research index historical data. Period: 'day','week','month'")]
    async fn index_hist_sw(
        &self,
        Parameters(index::IndexPeriodParams { symbol, period }): Parameters<index::IndexPeriodParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_hist_sw(&symbol, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan research index intraday minute data")]
    async fn index_min_sw(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_min_sw(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan research index constituent stocks")]
    async fn index_component_sw(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_component_sw(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan research index real-time data. Symbol: '市场表征','一级行业','二级行业','风格指数'")]
    async fn index_realtime_sw(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_realtime_sw(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan research daily analysis report")]
    async fn index_analysis_daily_sw(
        &self,
        Parameters(index::IndexHistParams {
            symbol,
            start_date,
            end_date,
        }): Parameters<index::IndexHistParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_analysis_daily_sw(&symbol, &start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan research week/month report date series. Symbol: 'WEEK' or 'MONTH'")]
    async fn index_analysis_week_month_sw(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_analysis_week_month_sw(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan research weekly analysis report")]
    async fn index_analysis_weekly_sw(
        &self,
        Parameters(index::SwAnalysisDateParams { symbol, date }): Parameters<
            index::SwAnalysisDateParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_analysis_weekly_sw(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenwan research monthly analysis report")]
    async fn index_analysis_monthly_sw(
        &self,
        Parameters(index::SwAnalysisDateParams { symbol, date }): Parameters<
            index::SwAnalysisDateParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_analysis_monthly_sw(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── yw.rs ───────────────────────────────────────────────────────────

    #[tool(description = "Get Yiwu small commodity index. Symbol: '周价格指数','月价格指数','月景气指数'")]
    async fn index_yw(
        &self,
        Parameters(index::IndexSymbolParams { symbol }): Parameters<index::IndexSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_yw(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

// ── zh_em.rs ────────────────────────────────────────────────────────

    #[tool(description = "Get A-share index historical data from Eastmoney. Period: 'daily','weekly','monthly'")]
    async fn index_zh_a_hist(
        &self,
        Parameters(index::IndexPeriodParams { symbol, period }): Parameters<index::IndexPeriodParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_zh_a_hist(&symbol, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share index intraday data from Eastmoney (Python-compatible)")]
    async fn index_zh_a_hist_min_em(
        &self,
        Parameters(index::IndexZhAHistMinEmParams {
            symbol,
            period,
            start_date,
            end_date,
            adjust,
        }): Parameters<index::IndexZhAHistMinEmParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_zh_a_hist_min_em(&symbol, &period, &start_date, &end_date, &adjust)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get A-share index intraday data from Eastmoney")]
    async fn index_zh_a_hist_min(
        &self,
        Parameters(index::IndexPeriodParams { symbol, period }): Parameters<index::IndexPeriodParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_zh_a_hist_min(&symbol, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Eastmoney stock-to-market-ID code mapping")]
    async fn index_code_id_map_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .index_code_id_map_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Auto-generated from agent_macro_data_tools.rs ──
    // ── Macro Data Tools (generated) ─────────────────────────────────
    // 161 new tools added (165 total - 4 existing)

    // ── China Macro Data ───────────────────────────────────────

    #[tool(description = "Get China CPI (Consumer Price Index) data")]
    async fn macro_china_cpi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_cpi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China CPI monthly rate data")]
    async fn macro_china_cpi_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_cpi_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China CPI yearly rate data")]
    async fn macro_china_cpi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_cpi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China PPI (Producer Price Index) data")]
    async fn macro_china_ppi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_ppi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China PPI yearly rate data")]
    async fn macro_china_ppi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_ppi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China PMI (Purchasing Managers' Index) data")]
    async fn macro_china_pmi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_pmi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China official manufacturing PMI yearly data")]
    async fn macro_china_pmi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_pmi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China Caixin manufacturing PMI yearly data")]
    async fn macro_china_cx_pmi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_cx_pmi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China Caixin services PMI yearly data")]
    async fn macro_china_cx_services_pmi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_cx_services_pmi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China non-manufacturing PMI data")]
    async fn macro_china_non_man_pmi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_non_man_pmi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China money supply (M0, M1, M2) data")]
    async fn macro_china_money_supply(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_money_supply()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China M2 yearly rate data")]
    async fn macro_china_m2_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_m2_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China trade balance data")]
    async fn macro_china_trade_balance(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_trade_balance()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China exports year-over-year data")]
    async fn macro_china_exports_yoy(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_exports_yoy()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China imports year-over-year data")]
    async fn macro_china_imports_yoy(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_imports_yoy()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China industrial production year-over-year data")]
    async fn macro_china_industrial_production_yoy(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_industrial_production_yoy()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China FX reserves yearly data")]
    async fn macro_china_fx_reserves_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_fx_reserves_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China foreign direct investment data")]
    async fn macro_china_fdi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_fdi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China FX and gold reserves data")]
    async fn macro_china_fx_gold(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_fx_gold()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China LPR (Loan Prime Rate) data")]
    async fn macro_china_lpr(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_lpr()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China new house price index data")]
    async fn macro_china_new_house_price(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_new_house_price()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China enterprise boom index data")]
    async fn macro_china_enterprise_boom_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_enterprise_boom_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China national tax receipts data")]
    async fn macro_china_national_tax_receipts(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_national_tax_receipts()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China new financial credit data")]
    async fn macro_china_new_financial_credit(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_new_financial_credit()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China stock market capitalization data")]
    async fn macro_china_stock_market_cap(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_stock_market_cap()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China consumer goods retail data")]
    async fn macro_china_consumer_goods_retail(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_consumer_goods_retail()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China bank financing products data")]
    async fn macro_china_bank_financing(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_bank_financing()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China insurance income data")]
    async fn macro_china_insurance_income(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_insurance_income()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China mobile phone shipments data")]
    async fn macro_china_mobile_number(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_mobile_number()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China vegetable basket index data")]
    async fn macro_china_vegetable_basket(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_vegetable_basket()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China agricultural product price index data")]
    async fn macro_china_agricultural_product(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_agricultural_product()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China agricultural index data")]
    async fn macro_china_agricultural_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_agricultural_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China energy index data")]
    async fn macro_china_energy_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_energy_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China commodity price index data")]
    async fn macro_china_commodity_price_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_commodity_price_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China Yiwu small commodity index data")]
    async fn macro_china_yw_electronic_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_yw_electronic_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China construction industry index data")]
    async fn macro_china_construction_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_construction_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China construction material price index data")]
    async fn macro_china_construction_price_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_construction_price_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China logistics prosperity index data")]
    async fn macro_china_lpi_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_lpi_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China BDTI (crude oil transport index) data")]
    async fn macro_china_bdti_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_bdti_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China BSI (supramax freight index) data")]
    async fn macro_china_bsi_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_bsi_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China real estate index data")]
    async fn macro_china_real_estate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_real_estate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China Shibor (Shanghai interbank offered rates) data")]
    async fn macro_china_shibor_all(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_shibor_all()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China HIBOR (Hong Kong interbank offered rates) data")]
    async fn macro_china_hk_market_info(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hk_market_info()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China RMB central parity rate data")]
    async fn macro_china_rmb(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_rmb()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shenzhen margin trading report data")]
    async fn macro_china_market_margin_sz(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_market_margin_sz()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shanghai margin trading report data")]
    async fn macro_china_market_margin_sh(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_market_margin_sh()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Shanghai Gold Exchange report data")]
    async fn macro_china_au_report(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_au_report()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China fiscal revenue data")]
    async fn macro_china_czsr(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_czsr()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China fixed asset investment data")]
    async fn macro_china_gdzctz(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_gdzctz()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China industrial value-added growth data")]
    async fn macro_china_gyzjz(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_gyzjz()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China customs trade data")]
    async fn macro_china_hgjck(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hgjck()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China FX loans data")]
    async fn macro_china_whxd(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_whxd()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China FX deposits data")]
    async fn macro_china_wbck(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_wbck()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China consumer confidence index data")]
    async fn macro_china_xfzxx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_xfzxx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China enterprise commodity price index data")]
    async fn macro_china_qyspjg(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_qyspjg()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China CNBS macro leverage ratio data")]
    async fn macro_cnbs(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_cnbs()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China central bank balance sheet data")]
    async fn macro_china_central_bank_balance(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_central_bank_balance()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China insurance industry data")]
    async fn macro_china_insurance(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_insurance()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China money supply (Sina Finance) data")]
    async fn macro_china_supply_of_money(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_supply_of_money()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China FX and gold reserves (Sina Finance) data")]
    async fn macro_china_foreign_exchange_gold(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_foreign_exchange_gold()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China retail price index data")]
    async fn macro_china_retail_price_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_retail_price_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China society electricity usage data")]
    async fn macro_china_society_electricity(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_society_electricity()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China society traffic volume data")]
    async fn macro_china_society_traffic_volume(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_society_traffic_volume()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China postal and telecommunications data")]
    async fn macro_china_postal_telecommunicational(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_postal_telecommunicational()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China international tourism FX revenue data")]
    async fn macro_china_international_tourism_fx(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_international_tourism_fx()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China passenger load factor data")]
    async fn macro_china_passenger_load_factor(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_passenger_load_factor()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China freight index data")]
    async fn macro_china_freight_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_freight_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China social financing scale data")]
    async fn macro_china_shrzgm(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_shrzgm()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China RMB loan data")]
    async fn macro_rmb_loan(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_rmb_loan()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China RMB deposit data")]
    async fn macro_rmb_deposit(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_rmb_deposit()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China daily energy data (Jin10)")]
    async fn macro_china_daily_energy(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_daily_energy()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China deposit reserve requirement ratio data")]
    async fn macro_china_reserve_requirement_ratio(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_reserve_requirement_ratio()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China urban survey unemployment rate data")]
    async fn macro_china_urban_unemployment(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_urban_unemployment()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── China NBS (parameterized) ──────────────────────────────

    #[tool(description = "Get China NBS national data (National Bureau of Statistics)")]
    async fn macro_china_nbs_nation(
        &self,
        Parameters(macro_data::MacroNbsNationParams { kind, path, period }): Parameters<macro_data::MacroNbsNationParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_nbs_nation(&kind, &path, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get China NBS regional data (National Bureau of Statistics)")]
    async fn macro_china_nbs_region(
        &self,
        Parameters(macro_data::MacroNbsRegionParams { kind, path, indicator, period }): Parameters<macro_data::MacroNbsRegionParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_nbs_region(&kind, &path, &indicator, &period)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── US Macro Data ──────────────────────────────────────────
    // (macro_usa_cpi_yoy and macro_usa_cpi_monthly already exist in mod.rs)

    #[tool(description = "Get US GDP monthly data")]
    async fn macro_usa_gdp_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_gdp_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US core CPI monthly data")]
    async fn macro_usa_core_cpi_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_core_cpi_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US pending home sales data")]
    async fn macro_usa_pending_home_sales(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_pending_home_sales()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US pending home sales (EM) data")]
    async fn macro_usa_phs(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_phs()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US personal spending data")]
    async fn macro_usa_personal_spending(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_personal_spending()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US retail sales data")]
    async fn macro_usa_retail_sales(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_retail_sales()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US import price index data")]
    async fn macro_usa_import_price(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_import_price()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US export price index data")]
    async fn macro_usa_export_price(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_export_price()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US LMCI (labor market conditions index) data")]
    async fn macro_usa_lmci(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_lmci()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US unemployment rate data")]
    async fn macro_usa_unemployment_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_unemployment_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US job cuts (Challenger) data")]
    async fn macro_usa_job_cuts(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_job_cuts()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US non-farm payrolls data")]
    async fn macro_usa_non_farm(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_non_farm()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US ADP employment data")]
    async fn macro_usa_adp_employment(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_adp_employment()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US core PCE price index data")]
    async fn macro_usa_core_pce_price(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_core_pce_price()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US real consumer spending data")]
    async fn macro_usa_real_consumer_spending(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_real_consumer_spending()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US trade balance data")]
    async fn macro_usa_trade_balance(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_trade_balance()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US current account data")]
    async fn macro_usa_current_account(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_current_account()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US PPI (producer price index) data")]
    async fn macro_usa_ppi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_ppi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US core PPI data")]
    async fn macro_usa_core_ppi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_core_ppi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US API crude oil stock data")]
    async fn macro_usa_api_crude_stock(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_api_crude_stock()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US manufacturing PMI data")]
    async fn macro_usa_pmi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_pmi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US ISM manufacturing PMI data")]
    async fn macro_usa_ism_pmi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_ism_pmi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US industrial production data")]
    async fn macro_usa_industrial_production(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_industrial_production()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US durable goods orders data")]
    async fn macro_usa_durable_goods_orders(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_durable_goods_orders()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US factory orders data")]
    async fn macro_usa_factory_orders(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_factory_orders()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US services PMI data")]
    async fn macro_usa_services_pmi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_services_pmi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US business inventories data")]
    async fn macro_usa_business_inventories(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_business_inventories()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US ISM non-manufacturing PMI data")]
    async fn macro_usa_ism_non_pmi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_ism_non_pmi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US NAHB housing market index data")]
    async fn macro_usa_nahb_house_market_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_nahb_house_market_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US housing starts data")]
    async fn macro_usa_house_starts(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_house_starts()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US new home sales data")]
    async fn macro_usa_new_home_sales(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_new_home_sales()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US building permits data")]
    async fn macro_usa_building_permits(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_building_permits()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US existing home sales data")]
    async fn macro_usa_exist_home_sales(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_exist_home_sales()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US house price index data")]
    async fn macro_usa_house_price_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_house_price_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US S&P/Case-Shiller 20-city house price index data")]
    async fn macro_usa_spcs20(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_spcs20()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US CB consumer confidence data")]
    async fn macro_usa_cb_consumer_confidence(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_cb_consumer_confidence()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US NFIB small business optimism data")]
    async fn macro_usa_nfib_small_business(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_nfib_small_business()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US Michigan consumer sentiment data")]
    async fn macro_usa_michigan_consumer_sentiment(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_michigan_consumer_sentiment()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US EIA crude oil rate data")]
    async fn macro_usa_eia_crude_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_eia_crude_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US initial jobless claims data")]
    async fn macro_usa_initial_jobless(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_initial_jobless()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US Baker Hughes rig count data")]
    async fn macro_usa_rig_count(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_rig_count()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get US crude oil production data")]
    async fn macro_usa_crude_inner(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_crude_inner()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFTC non-commercial holding data")]
    async fn macro_usa_cftc_nc_holding(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_cftc_nc_holding()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFTC commodity non-commercial holding data")]
    async fn macro_usa_cftc_c_holding(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_cftc_c_holding()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFTC merchant currency holding data")]
    async fn macro_usa_cftc_merchant_currency_holding(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_cftc_merchant_currency_holding()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFTC merchant goods holding data")]
    async fn macro_usa_cftc_merchant_goods_holding(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_cftc_merchant_goods_holding()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CME precious metals merchant goods holding data")]
    async fn macro_usa_cme_merchant_goods_holding(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_usa_cme_merchant_goods_holding()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Euro Area Macro Data ───────────────────────────────────

    #[tool(description = "Get Euro area GDP year-over-year data")]
    async fn macro_euro_gdp_yoy(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_gdp_yoy()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area CPI month-over-month data")]
    async fn macro_euro_cpi_mom(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_cpi_mom()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area CPI year-over-year data")]
    async fn macro_euro_cpi_yoy(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_cpi_yoy()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area PPI month-over-month data")]
    async fn macro_euro_ppi_mom(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_ppi_mom()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area retail sales month-over-month data")]
    async fn macro_euro_retail_sales_mom(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_retail_sales_mom()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area employment change quarter-over-quarter data")]
    async fn macro_euro_employment_change_qoq(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_employment_change_qoq()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area unemployment rate data")]
    async fn macro_euro_unemployment_rate_mom(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_unemployment_rate_mom()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area trade balance data")]
    async fn macro_euro_trade_balance(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_trade_balance()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area current account month-over-month data")]
    async fn macro_euro_current_account_mom(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_current_account_mom()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area industrial production month-over-month data")]
    async fn macro_euro_industrial_production_mom(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_industrial_production_mom()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area manufacturing PMI data")]
    async fn macro_euro_manufacturing_pmi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_manufacturing_pmi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area services PMI data")]
    async fn macro_euro_services_pmi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_services_pmi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area ZEW economic sentiment data")]
    async fn macro_euro_zew_economic_sentiment(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_zew_economic_sentiment()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Euro area Sentix investor confidence data")]
    async fn macro_euro_sentix_investor_confidence(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_sentix_investor_confidence()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get LME holdings report data")]
    async fn macro_euro_lme_holding(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_lme_holding()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get LME stock report data")]
    async fn macro_euro_lme_stock(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_euro_lme_stock()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── UK Macro Data ──────────────────────────────────────────

    #[tool(description = "Get UK Halifax house price index monthly data")]
    async fn macro_uk_halifax_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_halifax_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK Halifax house price index yearly data")]
    async fn macro_uk_halifax_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_halifax_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK trade balance data")]
    async fn macro_uk_trade(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_trade()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK bank rate (interest rate) data")]
    async fn macro_uk_bank_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_bank_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK core CPI yearly data")]
    async fn macro_uk_core_cpi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_core_cpi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK core CPI monthly data")]
    async fn macro_uk_core_cpi_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_core_cpi_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK CPI yearly data")]
    async fn macro_uk_cpi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_cpi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK CPI monthly data")]
    async fn macro_uk_cpi_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_cpi_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK retail sales monthly data")]
    async fn macro_uk_retail_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_retail_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK retail sales yearly data")]
    async fn macro_uk_retail_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_retail_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK Rightmove house price index monthly data")]
    async fn macro_uk_rightmove_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_rightmove_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK Rightmove house price index yearly data")]
    async fn macro_uk_rightmove_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_rightmove_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK GDP quarterly data")]
    async fn macro_uk_gdp_quarterly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_gdp_quarterly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK GDP yearly data")]
    async fn macro_uk_gdp_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_gdp_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get UK unemployment rate data")]
    async fn macro_uk_unemployment_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_uk_unemployment_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Bank Interest Rate Decisions ───────────────────────────

    #[tool(description = "Get US Federal Reserve interest rate decision data")]
    async fn macro_bank_usa_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_usa_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get ECB interest rate decision data")]
    async fn macro_bank_euro_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_euro_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get RBNZ (New Zealand) interest rate decision data")]
    async fn macro_bank_newzealand_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_newzealand_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get PBOC (China) interest rate decision data")]
    async fn macro_bank_china_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_china_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SNB (Switzerland) interest rate decision data")]
    async fn macro_bank_switzerland_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_switzerland_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get BOE (England) interest rate decision data")]
    async fn macro_bank_english_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_english_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get RBA (Australia) interest rate decision data")]
    async fn macro_bank_australia_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_australia_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get BOJ (Japan) interest rate decision data")]
    async fn macro_bank_japan_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_japan_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CBR (Russia) interest rate decision data")]
    async fn macro_bank_russia_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_russia_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get RBI (India) interest rate decision data")]
    async fn macro_bank_india_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_india_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get BCB (Brazil) interest rate decision data")]
    async fn macro_bank_brazil_interest_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_bank_brazil_interest_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Canada Macro Data ──────────────────────────────────────

    #[tool(description = "Get Canada new housing starts data")]
    async fn macro_canada_new_house_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_canada_new_house_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Canada unemployment rate data")]
    async fn macro_canada_unemployment_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_canada_unemployment_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Canada trade balance data")]
    async fn macro_canada_trade(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_canada_trade()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Canada retail sales monthly data")]
    async fn macro_canada_retail_rate_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_canada_retail_rate_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Canada bank rate (interest rate) data")]
    async fn macro_canada_bank_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_canada_bank_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Canada core CPI yearly data")]
    async fn macro_canada_core_cpi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_canada_core_cpi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Canada core CPI monthly data")]
    async fn macro_canada_core_cpi_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_canada_core_cpi_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Canada CPI yearly data")]
    async fn macro_canada_cpi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_canada_cpi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Canada CPI monthly data")]
    async fn macro_canada_cpi_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_canada_cpi_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Canada GDP monthly data")]
    async fn macro_canada_gdp_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_canada_gdp_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Germany Macro Data ─────────────────────────────────────

    #[tool(description = "Get Germany IFO business climate index data")]
    async fn macro_germany_ifo(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_germany_ifo()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Germany CPI monthly data")]
    async fn macro_germany_cpi_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_germany_cpi_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Germany CPI yearly data")]
    async fn macro_germany_cpi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_germany_cpi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Germany trade balance adjusted data")]
    async fn macro_germany_trade_adjusted(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_germany_trade_adjusted()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Germany GDP data")]
    async fn macro_germany_gdp(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_germany_gdp()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Germany retail sales monthly data")]
    async fn macro_germany_retail_sale_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_germany_retail_sale_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Germany retail sales yearly data")]
    async fn macro_germany_retail_sale_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_germany_retail_sale_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Germany ZEW economic sentiment data")]
    async fn macro_germany_zew(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_germany_zew()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Hong Kong Macro Data ───────────────────────────────────

    #[tool(description = "Get Hong Kong CPI data")]
    async fn macro_china_hk_cpi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hk_cpi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Hong Kong CPI year-over-year data")]
    async fn macro_china_hk_cpi_ratio(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hk_cpi_ratio()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Hong Kong unemployment rate data")]
    async fn macro_china_hk_rate_of_unemployment(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hk_rate_of_unemployment()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Hong Kong GDP data")]
    async fn macro_china_hk_gbp(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hk_gbp()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Hong Kong GDP year-over-year data")]
    async fn macro_china_hk_gbp_ratio(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hk_gbp_ratio()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Hong Kong building transaction volume data")]
    async fn macro_china_hk_building_volume(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hk_building_volume()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Hong Kong building transaction amount data")]
    async fn macro_china_hk_building_amount(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hk_building_amount()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Hong Kong trade balance year-over-year data")]
    async fn macro_china_hk_trade_diff_ratio(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hk_trade_diff_ratio()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Hong Kong PPI data")]
    async fn macro_china_hk_ppi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_china_hk_ppi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Japan Macro Data ───────────────────────────────────────

    #[tool(description = "Get Japan bank rate (interest rate) data")]
    async fn macro_japan_bank_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_japan_bank_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Japan CPI yearly data")]
    async fn macro_japan_cpi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_japan_cpi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Japan core CPI yearly data")]
    async fn macro_japan_core_cpi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_japan_core_cpi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Japan unemployment rate data")]
    async fn macro_japan_unemployment_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_japan_unemployment_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Japan leading indicator data")]
    async fn macro_japan_head_indicator(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_japan_head_indicator()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Australia Macro Data ───────────────────────────────────

    #[tool(description = "Get Australia bank rate (interest rate) data")]
    async fn macro_australia_bank_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_australia_bank_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Australia CPI quarterly data")]
    async fn macro_australia_cpi_quarterly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_australia_cpi_quarterly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Australia CPI yearly data")]
    async fn macro_australia_cpi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_australia_cpi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Australia PPI quarterly data")]
    async fn macro_australia_ppi_quarterly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_australia_ppi_quarterly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Australia retail sales monthly data")]
    async fn macro_australia_retail_rate_monthly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_australia_retail_rate_monthly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Australia trade balance data")]
    async fn macro_australia_trade(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_australia_trade()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Australia unemployment rate data")]
    async fn macro_australia_unemployment_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_australia_unemployment_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Switzerland Macro Data ─────────────────────────────────

    #[tool(description = "Get Switzerland SVME PMI data")]
    async fn macro_swiss_svme(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_swiss_svme()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Switzerland trade balance data")]
    async fn macro_swiss_trade(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_swiss_trade()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Switzerland CPI yearly data")]
    async fn macro_swiss_cpi_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_swiss_cpi_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Switzerland GDP quarterly data")]
    async fn macro_swiss_gdp_quarterly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_swiss_gdp_quarterly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Switzerland GDP yearly data")]
    async fn macro_swiss_gbd_yearly(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_swiss_gbd_yearly()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Switzerland bank rate (interest rate) data")]
    async fn macro_swiss_gbd_bank_rate(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_swiss_gbd_bank_rate()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Global / Shipping ──────────────────────────────────────

    #[tool(description = "Get Philadelphia Semiconductor Index (SOX) data")]
    async fn macro_global_sox_index(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_global_sox_index()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Baltic Capesize Index (BCI) shipping data")]
    async fn macro_shipping_bci(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_shipping_bci()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Baltic Dry Index (BDI) shipping data")]
    async fn macro_shipping_bdi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_shipping_bdi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Baltic Panamax Index (BPI) shipping data")]
    async fn macro_shipping_bpi(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_shipping_bpi()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Baltic Clean Tanker Index (BCTI) shipping data")]
    async fn macro_shipping_bcti(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_shipping_bcti()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Constitutes (ETF, OPEC) ────────────────────────────────

    #[tool(description = "Get SPDR Gold Trust ETF holdings data")]
    async fn macro_cons_gold(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_cons_gold()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get iShares Silver Trust ETF holdings data")]
    async fn macro_cons_silver(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_cons_silver()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get OPEC monthly report data")]
    async fn macro_cons_opec_month(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_cons_opec_month()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Other (Crypto, FX Sentiment, Wall Street, Finance) ─────

    #[tool(description = "Get crypto spot prices from Jin10")]
    async fn macro_crypto_spot(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_crypto_spot()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get stock finance data from THS")]
    async fn macro_stock_finance(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_stock_finance()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get FX sentiment report from Jin10")]
    async fn macro_fx_sentiment(
        &self,
        Parameters(macro_data::MacroDateRangeParams { start_date, end_date }): Parameters<macro_data::MacroDateRangeParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_fx_sentiment(&start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get Wall Street calendar macro data for a given date")]
    async fn macro_info_ws(
        &self,
        Parameters(macro_data::MacroDateParams { date }): Parameters<macro_data::MacroDateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .macro_info_ws(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Repo Rate (parameterized) ──────────────────────────────

    #[tool(description = "Get China Money repo fixing rate query data")]
    async fn repo_rate_query(
        &self,
        Parameters(macro_data::MacroSymbolParams { symbol }): Parameters<macro_data::MacroSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .repo_rate_query(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get historical repo fixing rates for a date range")]
    async fn repo_rate_hist(
        &self,
        Parameters(macro_data::MacroDateRangeParams { start_date, end_date }): Parameters<macro_data::MacroDateRangeParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .repo_rate_hist(&start_date, &end_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get interbank lending rate data from Eastmoney")]
    async fn rate_interbank(
        &self,
        Parameters(macro_data::MacroInterbankParams { market, symbol, indicator }): Parameters<macro_data::MacroInterbankParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .rate_interbank(&market, &symbol, &indicator)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Auto-generated from agent_news_tools.rs ──
    // ── News: Baidu report time ───────────────────────────────────────────

    #[tool(description = "Get Baidu stock report time data for a given stock symbol")]
    async fn news_report_time_baidu(
        &self,
        Parameters(news::NewsSymbolParams { symbol }): Parameters<news::NewsSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .news_report_time_baidu(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── News: Baidu suspension notifications ─────────────────────────────

    #[tool(description = "Get stock suspension trade notifications from Baidu for a given date (YYYYMMDD)")]
    async fn news_trade_notify_suspend_baidu(
        &self,
        Parameters(news::NewsDateParams { date }): Parameters<news::NewsDateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .news_trade_notify_suspend_baidu(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── News: Eastmoney search with scope ────────────────────────────────

    #[tool(description = "Search financial news from Eastmoney with a specific scope. Use 'default' for A-share focused news, 'global' for broader HK/US coverage")]
    async fn news_search_with_scope(
        &self,
        Parameters(news::NewsSearchWithScopeParams { query, limit, scope }): Parameters<
            news::NewsSearchWithScopeParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .news_search_with_scope(&query, limit, &scope)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── News: Baidu news search ──────────────────────────────────────────

    #[tool(description = "Search news from Baidu News by keyword")]
    async fn baidu_news_search(
        &self,
        Parameters(news::NewsQueryTimeoutParams { query, timeout_secs }): Parameters<
            news::NewsQueryTimeoutParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .baidu_news_search(&query, timeout_secs)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── News: GDELT global news search ───────────────────────────────────

    #[tool(description = "Search global news via GDELT API. Supports language filtering (zh-CN/en-US) and time ranges (day/week/month)")]
    async fn gdelt_news_search(
        &self,
        Parameters(news::GdeltNewsSearchParams {
            query,
            base_url,
            language_hint,
            time_range,
            timeout_secs,
        }): Parameters<news::GdeltNewsSearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .gdelt_news_search(
                &query,
                &base_url,
                language_hint.as_deref(),
                Some(&time_range),
                timeout_secs,
            )
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── News: Sogou news search ──────────────────────────────────────────

    #[tool(description = "Search news from Sogou News by keyword")]
    async fn sogou_news_search(
        &self,
        Parameters(news::NewsQueryTimeoutParams { query, timeout_secs }): Parameters<
            news::NewsQueryTimeoutParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .sogou_news_search(&query, timeout_secs)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── News: Bing RSS news search ───────────────────────────────────────

    #[tool(description = "Search news via Bing RSS feed. Set lang to 'en' for English results, omit for Chinese (default)")]
    async fn bing_news_rss(
        &self,
        Parameters(news::BingNewsRssParams { query, timeout_secs, lang }): Parameters<
            news::BingNewsRssParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .bing_news_rss_with_lang(&query, timeout_secs, lang.as_deref())
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── News: Google News RSS search ─────────────────────────────────────

    #[tool(description = "Search news from Google News RSS feed (English, US edition)")]
    async fn google_news_rss(
        &self,
        Parameters(news::NewsQueryTimeoutParams { query, timeout_secs }): Parameters<
            news::NewsQueryTimeoutParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .google_news_rss(&query, timeout_secs)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    // ── Auto-generated from agent_option_tools.rs ──
    // ── Option (generated) ────────────────────────────────────────────

    #[tool(description = "Get option contract info from openctp")]
    async fn option_contract_info_ctp(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_contract_info_ctp()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SZSE option daily statistics")]
    async fn option_daily_stats_szse(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_daily_stats_szse(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SZSE current day option contracts")]
    async fn option_current_day_szse(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_current_day_szse()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option risk indicators")]
    async fn option_risk_indicator_sse(
        &self,
        Parameters(futures::DateParams { date }): Parameters<futures::DateParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_risk_indicator_sse(&date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option underlying daily data")]
    async fn option_finance_sse_underlying(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_finance_sse_underlying(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get option finance board data")]
    async fn option_finance_board(
        &self,
        Parameters(option::OptionSymbolMonthParams { symbol, end_month }): Parameters<
            option::OptionSymbolMonthParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_finance_board(&symbol, &end_month)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get option billboard (dragon-tiger list) from Eastmoney")]
    async fn option_lhb_em(
        &self,
        Parameters(option::OptionLhbParams {
            symbol,
            indicator,
            trade_date,
        }): Parameters<option::OptionLhbParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_lhb_em(&symbol, &indicator, &trade_date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX SZ50 index option contract list from Sina")]
    async fn option_cffex_sz50_list_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_cffex_sz50_list_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX HS300 index option contract list from Sina")]
    async fn option_cffex_hs300_list_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_cffex_hs300_list_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX ZZ1000 index option contract list from Sina")]
    async fn option_cffex_zz1000_list_sina(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_cffex_zz1000_list_sina()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX SZ50 index option spot data from Sina")]
    async fn option_cffex_sz50_spot_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_cffex_sz50_spot_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX HS300 index option spot data from Sina")]
    async fn option_cffex_hs300_spot_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_cffex_hs300_spot_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX ZZ1000 index option spot data from Sina")]
    async fn option_cffex_zz1000_spot_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_cffex_zz1000_spot_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX SZ50 index option daily kline from Sina")]
    async fn option_cffex_sz50_daily_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_cffex_sz50_daily_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX HS300 index option daily kline from Sina")]
    async fn option_cffex_hs300_daily_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_cffex_hs300_daily_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX ZZ1000 index option daily kline from Sina")]
    async fn option_cffex_zz1000_daily_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_cffex_zz1000_daily_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get commodity option contract list from Sina")]
    async fn option_commodity_contract_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_commodity_contract_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get commodity option contract table (call/put pairs) from Sina")]
    async fn option_commodity_contract_table_sina(
        &self,
        Parameters(option::OptionSymbolContractParams { symbol, contract }): Parameters<
            option::OptionSymbolContractParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_commodity_contract_table_sina(&symbol, &contract)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get commodity option daily history from Sina")]
    async fn option_commodity_hist_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_commodity_hist_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get option premium analysis from Eastmoney")]
    async fn option_premium_analysis_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_premium_analysis_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get option value analysis from Eastmoney")]
    async fn option_value_analysis_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_value_analysis_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get option risk analysis from Eastmoney")]
    async fn option_risk_analysis_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_risk_analysis_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CZCE yearly option history")]
    async fn option_hist_yearly_czce(
        &self,
        Parameters(option::OptionSymbolYearParams { symbol, year }): Parameters<
            option::OptionSymbolYearParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_hist_yearly_czce(&symbol, &year)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get available commodity option symbols from 9qihuo")]
    async fn option_comm_symbol(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_comm_symbol()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get commodity option commission info from 9qihuo")]
    async fn option_comm_info(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_comm_info(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option expiry month list from Sina")]
    async fn option_sse_list_sina(
        &self,
        Parameters(option::OptionSymbolExchangeParams { symbol, exchange }): Parameters<
            option::OptionSymbolExchangeParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_sse_list_sina(&symbol, &exchange)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option expiry day and remaining days from Sina")]
    async fn option_sse_expire_day_sina(
        &self,
        Parameters(option::OptionSseExpireDayParams {
            trade_date,
            symbol,
            exchange,
        }): Parameters<option::OptionSseExpireDayParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_sse_expire_day_sina(&trade_date, &symbol, &exchange)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option codes (call or put) from Sina")]
    async fn option_sse_codes_sina(
        &self,
        Parameters(option::OptionSseCodesParams {
            symbol,
            trade_date,
            underlying,
        }): Parameters<option::OptionSseCodesParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_sse_codes_sina(&symbol, &trade_date, &underlying)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option spot price from Sina")]
    async fn option_sse_spot_price_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_sse_spot_price_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option underlying spot price from Sina")]
    async fn option_sse_underlying_spot_price_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_sse_underlying_spot_price_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option minute data from Sina")]
    async fn option_sse_minute_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_sse_minute_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SSE option daily kline data from Sina")]
    async fn option_sse_daily_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_sse_daily_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get option finance 5-day minute data from Sina")]
    async fn option_finance_minute_sina(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_finance_minute_sina(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get option chain snapshots from Eastmoney")]
    async fn option_chain(
        &self,
        Parameters(option::OptionChainParams { symbol, limit }): Parameters<
            option::OptionChainParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_chain(&symbol, limit)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get all current-day options from Eastmoney (SSE/SZSE + CFFEX)")]
    async fn option_current_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_current_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get CFFEX options from Eastmoney")]
    async fn option_current_cffex_em(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_current_cffex_em()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get option minute (intraday trend) data from Eastmoney")]
    async fn option_minute_em(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_minute_em(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get DCE option daily data")]
    async fn option_hist_dce(
        &self,
        Parameters(option::OptionSymbolDateParams { symbol, date }): Parameters<
            option::OptionSymbolDateParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_hist_dce(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE option daily data")]
    async fn option_hist_shfe(
        &self,
        Parameters(option::OptionSymbolDateParams { symbol, date }): Parameters<
            option::OptionSymbolDateParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_hist_shfe(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get SHFE option implied volatility data")]
    async fn option_vol_shfe(
        &self,
        Parameters(option::OptionSymbolDateParams { symbol, date }): Parameters<
            option::OptionSymbolDateParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_vol_shfe(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get GFEX option daily data")]
    async fn option_hist_gfex(
        &self,
        Parameters(option::OptionSymbolDateParams { symbol, date }): Parameters<
            option::OptionSymbolDateParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_hist_gfex(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get GFEX option implied volatility data")]
    async fn option_vol_gfex(
        &self,
        Parameters(option::OptionSymbolDateParams { symbol, date }): Parameters<
            option::OptionSymbolDateParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_vol_gfex(&symbol, &date)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get available commodity option margin symbols")]
    async fn option_margin_symbol(
        &self,
        Parameters(_): Parameters<stock::EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_margin_symbol()
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get commodity option margin data")]
    async fn option_margin(
        &self,
        Parameters(option::OptionSymbolParams { symbol }): Parameters<option::OptionSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .option_margin(&symbol)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

}

/// Auto-detect market code for fund flow from symbol.
fn detect_fund_flow_market(symbol: &str) -> String {
    use akshare::market::detect_market;
    use akshare::types::MarketKind;

    match detect_market(symbol) {
        MarketKind::HongKong => "hk".to_string(),
        MarketKind::UsEquity => "us".to_string(),
        MarketKind::AShare => {
            let trimmed = symbol.trim();
            if trimmed.starts_with('6') {
                "sh".to_string()
            } else {
                "sz".to_string()
            }
        }
    }
}

impl Default for AkShareMcpService {
    fn default() -> Self {
        Self::new(ToolsConfig::default())
    }
}

// ── Meta-tool param types ──────────────────────────────────────────────

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

// ── ServerHandler implementation ──────────────────────────────────────

const CATEGORY_PREFIXES: &[(&str, &[&str])] = &[
    ("stock", &["stock_", "a_share_", "hk_", "us_", "get_us_"]),
    ("bond", &["bond_"]),
    ("index", &["index_"]),
    ("futures", &["futures_"]),
    (
        "economy",
        &[
            "economy_", "movie_", "nlp_", "amac_", "car_", "sw_", "fx_", "article_", "air_",
            "qdii_", "video_", "sunrise_", "repo_", "migration_", "fred_", "xincaifu_", "spot_",
            "sogou_", "rate_", "online_", "match_", "hurun_", "hf_", "google_", "gdelt_",
            "game_", "forbes_", "drewry_", "business_", "bing_", "baidu_", "qhkc_",
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

impl ServerHandler for AkShareMcpService {
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
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

                let query_lower = args.query.map(|q| q.to_lowercase());
                let category_prefixes: Option<&[&str]> = args
                    .category
                    .as_deref()
                    .and_then(|cat| CATEGORY_PREFIXES.iter().find(|(name, _)| *name == cat))
                    .map(|(_, prefixes)| *prefixes);

                let results: Vec<serde_json::Value> = self
                    .tool_router
                    .map
                    .values()
                    .filter(|route| {
                        let name = route.attr.name.as_ref();
                        // Skip the meta-tools themselves
                        if name == "tools/search" || name == "tools/call" {
                            return false;
                        }
                        // Category filter
                        if let Some(prefixes) = category_prefixes {
                            if !prefixes.iter().any(|p| name.starts_with(p)) {
                                return false;
                            }
                        }
                        // Query filter
                        if let Some(ref q) = query_lower {
                            let name_match = name.to_lowercase().contains(q);
                            let desc_match = route
                                .attr
                                .description
                                .as_ref()
                                .is_some_and(|d| d.to_lowercase().contains(q));
                            if !name_match && !desc_match {
                                return false;
                            }
                        }
                        true
                    })
                    .map(|route| {
                        serde_json::json!({
                            "name": route.attr.name.as_ref(),
                            "description": route.attr.description.as_deref().unwrap_or(""),
                        })
                    })
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

                let inner_request = CallToolRequestParams::new(args.name)
                    .with_arguments(args.arguments.unwrap_or_default());
                let tcc = ToolCallContext::new(self, inner_request, context);
                self.tool_router.call(tcc).await
            }
            _ => {
                let tcc = ToolCallContext::new(self, request, context);
                self.tool_router.call(tcc).await
            }
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, McpError> {
        let tools = vec![
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
        ];

        Ok(rmcp::model::ListToolsResult {
            tools,
            meta: None,
            next_cursor: None,
        })
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        match name {
            "tools/search" => Some(Tool::new_with_raw(
                "tools/search",
                Some("Search available tools by keyword and/or category.".into()),
                schema_for_type::<ToolsSearchParams>(),
            )),
            "tools/call" => Some(Tool::new_with_raw(
                "tools/call",
                Some("Call any tool by name with JSON arguments.".into()),
                schema_for_type::<ToolsCallParams>(),
            )),
            _ => None,
        }
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
    use rmcp::model::{NumberOrString, PaginatedRequestParams};
    use rmcp::service::{AtomicU32RequestIdProvider, Peer};
    use std::sync::Arc;

    fn test_context() -> RequestContext<RoleServer> {
        let id_provider: Arc<dyn rmcp::service::RequestIdProvider> =
            Arc::new(AtomicU32RequestIdProvider::default());
        let (peer, _rx) = Peer::<RoleServer>::new(id_provider, None);
        RequestContext::new(NumberOrString::Number(1), peer)
    }

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
    fn test_service_category_filter() {
        let mut cfg = ToolsConfig::default();
        cfg.bond = true;
        let service = AkShareMcpService::new(cfg);
        let tools = service.tool_router.list_all();
        let has_bond = tools.iter().any(|t| t.name.starts_with("bond_"));
        let has_futures = tools.iter().any(|t| t.name.starts_with("futures_"));
        assert!(has_bond, "bond tools should be enabled");
        assert!(!has_futures, "futures tools should be disabled");
    }

    #[tokio::test]
    async fn test_list_tools_returns_meta_tools_only() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let ctx = test_context();
        let result = service.list_tools(None, ctx).await.unwrap();
        assert_eq!(result.tools.len(), 2, "should return exactly 2 meta-tools");
        let names: Vec<&str> = result.tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(names.contains(&"tools/search"));
        assert!(names.contains(&"tools/call"));
    }

    #[test]
    fn test_get_tool_meta_tools() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        assert!(service.get_tool("tools/search").is_some());
        assert!(service.get_tool("tools/call").is_some());
        assert!(service.get_tool("bond_zh_us_rate").is_none());
    }

    #[tokio::test]
    async fn test_tools_search_by_category() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let ctx = test_context();
        let request = CallToolRequestParams {
            meta: None,
            name: std::borrow::Cow::Borrowed("tools/search"),
            arguments: Some({
                let mut map = serde_json::Map::new();
                map.insert(
                    "category".to_string(),
                    serde_json::Value::String("bond".to_string()),
                );
                map
            }),
            task: None,
        };
        let result = service.call_tool(request, ctx).await.unwrap();
        assert!(!result.is_error.unwrap_or(true));
        let text = result.content[0].as_text().unwrap().text.clone();
        let items: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap();
        assert!(!items.is_empty(), "bond category should have tools");
        for item in &items {
            let name = item["name"].as_str().unwrap();
            assert!(name.starts_with("bond_"), "all results should be bond tools, got: {name}");
        }
    }

    #[tokio::test]
    async fn test_tools_search_excludes_meta_tools() {
        let service = AkShareMcpService::new(ToolsConfig::all());
        let ctx = test_context();
        let request = CallToolRequestParams {
            meta: None,
            name: std::borrow::Cow::Borrowed("tools/search"),
            arguments: Some({
                let mut map = serde_json::Map::new();
                map.insert(
                    "query".to_string(),
                    serde_json::Value::String("tools".to_string()),
                );
                map
            }),
            task: None,
        };
        let result = service.call_tool(request, ctx).await.unwrap();
        let text = result.content[0].as_text().unwrap().text.clone();
        let items: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap();
        for item in &items {
            let name = item["name"].as_str().unwrap();
            assert_ne!(name, "tools/search");
            assert_ne!(name, "tools/call");
        }
    }
}
