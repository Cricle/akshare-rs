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
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content, Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

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
    /// Create a new service instance with default configuration.
    #[must_use]
    pub fn new() -> Self {
        let client = AkShareClient::new();
        Self {
            client,
            tool_router: Self::tool_router(),
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
        Self::new()
    }
}

#[tool_handler]
impl ServerHandler for AkShareMcpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "Financial market data MCP server powered by akshare-rs. \
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
        let service = AkShareMcpService::new();
        let info = service.get_info();
        assert!(info.capabilities.tools.is_some());
    }

    #[test]
    fn test_service_info_instructions() {
        let service = AkShareMcpService::new();
        let info = service.get_info();
        let instructions = info.instructions.unwrap();
        assert!(instructions.contains("akshare-rs"));
    }

    #[test]
    fn test_service_clone() {
        let service = AkShareMcpService::new();
        let cloned = service;
        let info = cloned.get_info();
        assert!(info.capabilities.tools.is_some());
    }
}
