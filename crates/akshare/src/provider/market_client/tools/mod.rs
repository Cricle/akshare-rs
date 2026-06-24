mod indicators;
mod market_data;
mod news;
mod summarize;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::MarketDataClient;
use super::normalized_news_date;
use crate::types::{CandlePoint, FundamentalsSnapshot, NewsItem, QuoteSnapshot};

/// Pre-fetched scenario data passed into tool execution.
///
/// Contains market data that may have been fetched earlier in the analysis
/// pipeline, allowing tools to skip redundant API calls.
#[derive(Clone, Debug, Default)]
pub struct ScenarioData {
    pub candles: Vec<CandlePoint>,
    pub fundamentals: Option<FundamentalsSnapshot>,
    pub company_news: Vec<NewsItem>,
    pub quote: Option<QuoteSnapshot>,
}

#[derive(Clone)]
pub struct TradingToolbox {
    market_data: MarketDataClient,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PendingToolCall {
    pub tool_name: String,
    #[serde(default)]
    pub arguments: Value,
    pub reason: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ToolObservation {
    pub tool_name: String,
    #[serde(default)]
    pub arguments: Value,
    pub output: String,
    #[serde(default)]
    pub meta: Value,
    pub success: bool,
    pub created_at: String,
}

struct ToolExecutionResult {
    output: String,
    meta: Value,
}

impl TradingToolbox {
    const TECHNICAL_HISTORY_MIN_BARS: usize = 320;

    pub fn new(market_data: MarketDataClient) -> Self {
        Self { market_data }
    }

    fn summarize_success_output(tool_name: &str, output: &str, meta: &Value) -> String {
        match tool_name {
            "get_stock_data" => Self::summarize_stock_data_output(output),
            "get_indicators" => Self::summarize_indicator_output(output, meta),
            "get_fundamentals" | "get_balance_sheet" | "get_cashflow" | "get_income_statement" => {
                Self::summarize_json_object_output(output, 18)
            }
            _ => output.to_string(),
        }
    }

    #[tracing::instrument(skip_all, fields(tool_name = %pending.tool_name, symbol = %symbol))]
    pub async fn execute(
        &self,
        symbol: &str,
        market_type: &str,
        scenario_data: Option<&ScenarioData>,
        pending: &PendingToolCall,
    ) -> ToolObservation {
        let meter = opentelemetry::global::meter("tradingagents");
        let tool_total = meter.u64_counter("tool_execution_total").build();
        let tool_duration = meter.f64_histogram("tool_execution_duration_ms").build();
        let tool_errors = meter.u64_counter("tool_execution_errors_total").build();

        let start = std::time::Instant::now();
        let result = self
            .execute_inner(
                symbol,
                market_type,
                scenario_data,
                &pending.tool_name,
                &pending.arguments,
            )
            .await;
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

        let base_attrs = [
            opentelemetry::KeyValue::new("tool.name", pending.tool_name.clone()),
            opentelemetry::KeyValue::new("tool.symbol", symbol.to_string()),
        ];

        match result {
            Ok(result) => {
                let mut attrs = base_attrs.to_vec();
                attrs.push(opentelemetry::KeyValue::new("tool.outcome", "success"));
                tool_total.add(1, &attrs);
                tool_duration.record(elapsed_ms, &attrs);
                ToolObservation {
                    tool_name: pending.tool_name.clone(),
                    arguments: pending.arguments.clone(),
                    output: Self::summarize_success_output(
                        &pending.tool_name,
                        &result.output,
                        &result.meta,
                    ),
                    meta: result.meta,
                    success: true,
                    created_at: Utc::now().to_rfc3339(),
                }
            }
            Err(error) => {
                let mut attrs = base_attrs.to_vec();
                attrs.push(opentelemetry::KeyValue::new("tool.outcome", "error"));
                tool_total.add(1, &attrs);
                tool_errors.add(1, &attrs);
                tool_duration.record(elapsed_ms, &attrs);
                ToolObservation {
                    tool_name: pending.tool_name.clone(),
                    arguments: pending.arguments.clone(),
                    output: format!("tool execution failed: {error:#}"),
                    meta: json!({
                        "kind": "tool_error",
                        "message": error.to_string(),
                    }),
                    success: false,
                    created_at: Utc::now().to_rfc3339(),
                }
            }
        }
    }

    async fn execute_inner(
        &self,
        symbol: &str,
        market_type: &str,
        scenario_data: Option<&ScenarioData>,
        tool_name: &str,
        arguments: &Value,
    ) -> anyhow::Result<ToolExecutionResult> {
        match tool_name {
            "get_stock_data" => {
                self.get_stock_data(symbol, market_type, scenario_data, arguments)
                    .await
            }
            "get_indicators" => self.get_indicators(symbol, scenario_data, arguments).await,
            "get_fundamentals" => self.get_fundamentals(symbol, scenario_data).await,
            "get_balance_sheet" => self.get_balance_sheet(symbol, scenario_data).await,
            "get_cashflow" => self.get_cashflow(symbol, scenario_data).await,
            "get_income_statement" => self.get_income_statement(symbol, scenario_data).await,
            "get_news" => self.get_news(symbol, scenario_data, arguments).await,
            "get_global_news" => self.get_global_news(symbol, market_type, arguments).await,
            "get_insider_transactions" => {
                self.get_insider_transactions(symbol, scenario_data, arguments)
                    .await
            }
            other => anyhow::bail!("unsupported tool: {other}"),
        }
    }

    fn filter_candles_by_date(
        &self,
        items: Vec<CandlePoint>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Vec<CandlePoint> {
        items
            .into_iter()
            .filter(|item| start_date.is_none_or(|value| item.trade_date.as_str() >= value))
            .filter(|item| end_date.is_none_or(|value| item.trade_date.as_str() <= value))
            .collect()
    }

    fn candle_fetch_limit(
        start_date: Option<&str>,
        end_date: Option<&str>,
        fallback: usize,
    ) -> usize {
        let Some(start_date) = start_date else {
            return fallback.max(Self::TECHNICAL_HISTORY_MIN_BARS);
        };
        let Ok(start) = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d") else {
            return fallback.max(Self::TECHNICAL_HISTORY_MIN_BARS);
        };
        let end = end_date
            .and_then(|value| chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
            .unwrap_or_else(|| chrono::Utc::now().date_naive());
        let today = chrono::Utc::now().date_naive();
        let effective_end = end.max(today);
        let span_days = (effective_end - start).num_days().unsigned_abs() as usize + 1;
        // Daily bars across historical windows need slack for weekends/holidays and indicators.
        fallback.max((span_days * 2).max(Self::TECHNICAL_HISTORY_MIN_BARS))
    }

    fn filter_news_by_date(
        &self,
        items: Vec<NewsItem>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Vec<NewsItem> {
        let mut filtered = items
            .into_iter()
            .filter(|item| {
                if item.published_at.trim().is_empty() {
                    return true;
                }
                let normalized =
                    normalized_news_date(&item.published_at).unwrap_or_else(|| {
                        item.published_at
                            .get(0..10)
                            .unwrap_or(item.published_at.as_str())
                            .to_string()
                    });
                let date = normalized.as_str();
                start_date.is_none_or(|value| date >= value)
                    && end_date.is_none_or(|value| date <= value)
            })
            .collect::<Vec<_>>();
        filtered.sort_by(|left, right| right.published_at.cmp(&left.published_at));
        filtered.dedup_by(|left, right| {
            left.published_at.get(0..10) == right.published_at.get(0..10)
                && left.title == right.title
                && left.source == right.source
        });
        filtered
    }

    fn candle_json(item: &CandlePoint) -> Value {
        json!({
            "trade_date": item.trade_date,
            "open": item.open,
            "close": item.close,
            "high": item.high,
            "low": item.low,
            "volume": item.volume,
            "amount": item.amount,
            "amplitude_pct": item.amplitude_pct,
            "change_pct": item.change_pct,
            "change_amount": item.change_amount,
            "turnover_pct": item.turnover_pct
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_candle(date: &str) -> CandlePoint {
        CandlePoint {
            trade_date: date.to_string(),
            open: 100.0,
            high: 110.0,
            low: 90.0,
            close: 105.0,
            volume: 1000000,
            amount: 100000000.0,
            amplitude_pct: 10.0,
            change_pct: 5.0,
            change_amount: 5.0,
            turnover_pct: 1.0,
        }
    }

    #[test]
    fn candle_fetch_limit_no_dates() {
        let limit = TradingToolbox::candle_fetch_limit(None, None, 100);
        assert_eq!(limit, 320); // TECHNICAL_HISTORY_MIN_BARS
    }

    #[test]
    fn candle_fetch_limit_with_dates() {
        let limit = TradingToolbox::candle_fetch_limit(Some("2026-01-01"), Some("2026-06-01"), 100);
        assert!(limit >= 320);
    }

    #[test]
    fn candle_fetch_limit_invalid_date() {
        let limit = TradingToolbox::candle_fetch_limit(Some("invalid"), None, 100);
        assert_eq!(limit, 320);
    }

    #[test]
    fn candle_fetch_limit_small_fallback() {
        let limit = TradingToolbox::candle_fetch_limit(None, None, 50);
        assert_eq!(limit, 320); // min bars wins
    }

    #[test]
    fn candle_json_serialization() {
        let candle = make_candle("2026-01-01");
        let json = TradingToolbox::candle_json(&candle);
        assert_eq!(json["trade_date"], "2026-01-01");
        assert_eq!(json["open"], 100.0);
        assert_eq!(json["close"], 105.0);
        assert_eq!(json["high"], 110.0);
        assert_eq!(json["low"], 90.0);
        assert_eq!(json["volume"], 1000000);
    }

    #[test]
    fn summarize_success_output_unknown_tool() {
        let result = TradingToolbox::summarize_success_output("unknown_tool", "output", &json!({}));
        assert_eq!(result, "output");
    }

    #[test]
    fn summarize_success_output_stock_data() {
        let result = TradingToolbox::summarize_success_output("get_stock_data", "data output", &json!({}));
        // Should return summarized version
        assert!(!result.is_empty());
    }
}
