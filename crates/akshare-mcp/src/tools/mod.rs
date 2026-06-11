//! MCP tool definitions for akshare financial data.
//!
//! All 42 tools are defined on [`AkShareMcpService`] and organized by category:
//! stock, fund, bond, futures, option, forex, crypto, index, macro_data, economy, news.

pub mod bond;
pub mod crypto;
pub mod economy;
pub mod fund;
pub mod forex;
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
    model::*,
    tool, tool_handler, tool_router,
};

/// Central MCP service exposing 42 akshare financial data tools.
///
/// Implements [`ServerHandler`] for the MCP protocol and routes tool calls
/// to the appropriate akshare API methods.
#[derive(Clone)]
pub struct AkShareMcpService {
    client: AkShareClient,
    #[allow(dead_code)]
    tool_router: ToolRouter<AkShareMcpService>,
}

#[tool_router]
impl AkShareMcpService {
    /// Create a new service instance with default configuration.
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
    async fn stock_zh_a_spot_em(&self) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_zh_a_spot_em()
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
            .stock_zh_a_hist(&symbol, &period, &start_date, &end_date, &adjust)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get industry board list from Eastmoney")]
    async fn stock_board_industry_name_em(&self) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_board_industry_name_em(100)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&data).unwrap(),
        )]))
    }

    #[tool(description = "Get individual stock fund flow from Eastmoney")]
    async fn stock_individual_fund_flow(
        &self,
        Parameters(stock::FundFlowParams { symbol, market, limit }): Parameters<
            stock::FundFlowParams,
        >,
    ) -> Result<CallToolResult, McpError> {
        let data = self
            .client
            .stock_individual_fund_flow(&symbol, &market, limit)
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
    async fn fund_manager_em(&self) -> Result<CallToolResult, McpError> {
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
    async fn fund_etf_spot_em(&self) -> Result<CallToolResult, McpError> {
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
    async fn fund_lof_spot_em(&self) -> Result<CallToolResult, McpError> {
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
    async fn bond_spot_deal(&self) -> Result<CallToolResult, McpError> {
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
    async fn option_current_day_sse(&self) -> Result<CallToolResult, McpError> {
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
        Parameters(option::OptionHistParams {
            symbol,
            date,
            ..
        }): Parameters<option::OptionHistParams>,
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
    async fn forex_boc_rates(&self) -> Result<CallToolResult, McpError> {
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
    async fn forex_em_rates(&self) -> Result<CallToolResult, McpError> {
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
    async fn forex_spot_em(&self) -> Result<CallToolResult, McpError> {
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
    async fn crypto_bitcoin_hold_report(&self) -> Result<CallToolResult, McpError> {
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
    async fn index_stock_info(&self) -> Result<CallToolResult, McpError> {
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

    // ── Macro Data ─────────────────────────────────────────────

    #[tool(description = "Get China GDP data")]
    async fn macro_china_gdp(&self) -> Result<CallToolResult, McpError> {
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
    async fn macro_usa_cpi_yoy(&self) -> Result<CallToolResult, McpError> {
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
    async fn macro_china_gdp_yearly(&self) -> Result<CallToolResult, McpError> {
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
    async fn macro_usa_cpi_monthly(&self) -> Result<CallToolResult, McpError> {
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
    async fn economy_auto_sales(&self) -> Result<CallToolResult, McpError> {
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
    async fn economy_box_office(&self) -> Result<CallToolResult, McpError> {
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
    async fn movie_boxoffice_realtime(&self) -> Result<CallToolResult, McpError> {
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
        let cloned = service.clone();
        let info = cloned.get_info();
        assert!(info.capabilities.tools.is_some());
    }
}
