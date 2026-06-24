impl TradingToolbox {
    pub(super) async fn get_stock_data(
        &self,
        symbol: &str,
        market_type: &str,
        scenario_data: Option<&ScenarioData>,
        arguments: &Value,
    ) -> anyhow::Result<ToolExecutionResult> {
        let start_date = arguments
            .get("start_date")
            .or_else(|| arguments.get("from"))
            .and_then(Value::as_str);
        let end_date = arguments
            .get("end_date")
            .or_else(|| arguments.get("to"))
            .and_then(Value::as_str);
        let limit = arguments
            .get("limit")
            .and_then(Value::as_u64)
            .map(|value| value as usize)
            .unwrap_or(Self::TECHNICAL_HISTORY_MIN_BARS);
        let fetch_limit = Self::candle_fetch_limit(start_date, end_date, limit);
        let candles = if let Some(prefetched) = scenario_data
            .filter(|data| !data.candles.is_empty())
            .map(|data| data.candles.clone())
        {
            prefetched
        } else {
            match self
                .market_data
                .fetch_candles(symbol, "qfq", fetch_limit)
                .await
            {
                Ok(items) => items,
                Err(error) => {
                    let body = json!({
                        "symbol": symbol,
                        "market_type": market_type,
                        "start_date": start_date,
                        "end_date": end_date,
                        "rows": [],
                        "data_gap": {
                            "kind": "technical_data_unavailable",
                            "message": error.to_string(),
                        }
                    });
                    return Ok(ToolExecutionResult {
                        output: serde_json::to_string_pretty(&body)?,
                        meta: json!({
                            "kind": "stock_data",
                            "source": "live_fetch_failed",
                            "data_gap": {
                                "kind": "technical_data_unavailable",
                                "message": error.to_string(),
                            }
                        }),
                    });
                }
            }
        };
        let body = json!({
            "symbol": symbol,
            "market_type": market_type,
            "start_date": start_date,
            "end_date": end_date,
            "fetched_history_count": candles.len(),
            "rows": self
                .filter_candles_by_date(
                    candles,
                    start_date,
                    end_date,
                )
                .into_iter()
                .map(|item| Self::candle_json(&item))
                .collect::<Vec<_>>()
        });
        Ok(ToolExecutionResult {
            output: serde_json::to_string_pretty(&body)?,
            meta: json!({
                "kind": "stock_data",
                "source": if scenario_data.is_some_and(|data| !data.candles.is_empty()) { "prefetched_scenario" } else { "live_fetch" },
                "row_count": body["rows"].as_array().map(|rows| rows.len()).unwrap_or_default(),
            }),
        })
    }

    pub(super) async fn get_indicators(
        &self,
        symbol: &str,
        scenario_data: Option<&ScenarioData>,
        arguments: &Value,
    ) -> anyhow::Result<ToolExecutionResult> {
        let start_date = arguments
            .get("start_date")
            .or_else(|| arguments.get("from"))
            .and_then(Value::as_str);
        let end_date = arguments
            .get("end_date")
            .or_else(|| arguments.get("to"))
            .and_then(Value::as_str);
        let candles = if let Some(prefetched) = scenario_data
            .filter(|data| !data.candles.is_empty())
            .map(|data| data.candles.clone())
        {
            prefetched
        } else {
            match self
                .market_data
                .fetch_candles(
                    symbol,
                    "qfq",
                    Self::candle_fetch_limit(
                        start_date,
                        end_date,
                        Self::TECHNICAL_HISTORY_MIN_BARS,
                    ),
                )
                .await
            {
                Ok(items) => items,
                Err(error) => {
                    let body = json!({
                        "symbol": symbol,
                        "start_date": start_date,
                        "end_date": end_date,
                        "indicators": [],
                        "history_candle_count": 0,
                        "requested_window_candle_count": 0,
                        "data_gap": {
                            "kind": "technical_indicators_unavailable",
                            "message": error.to_string(),
                        }
                    });
                    return Ok(ToolExecutionResult {
                        output: serde_json::to_string_pretty(&body)?,
                        meta: json!({
                            "kind": "indicators",
                            "source": "live_fetch_failed",
                            "data_gap": {
                                "kind": "technical_indicators_unavailable",
                                "message": error.to_string(),
                            }
                        }),
                    });
                }
            }
        };
        let requested_window_candles =
            self.filter_candles_by_date(candles.clone(), start_date, end_date);
        let indicators = arguments
            .get("indicators")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .filter(|items| !items.is_empty())
            .unwrap_or_else(|| {
                vec![
                    "close_50_sma".to_string(),
                    "close_200_sma".to_string(),
                    "close_10_ema".to_string(),
                    "rsi".to_string(),
                    "atr".to_string(),
                ]
            });
        let indicator_items = indicators
            .iter()
            .map(|name| {
                let value = Self::compute_indicator(name, &candles);
                json!({
                    "key": name,
                    "value": value,
                    "available": value.is_some(),
                })
            })
            .collect::<Vec<_>>();
        let unavailable = indicator_items
            .iter()
            .filter(|item| {
                !item
                    .get("available")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
            })
            .filter_map(|item| item.get("key").and_then(Value::as_str))
            .map(str::to_string)
            .collect::<Vec<_>>();
        let body = json!({
            "symbol": symbol,
            "start_date": start_date,
            "end_date": end_date,
            "history_candle_count": candles.len(),
            "requested_window_candle_count": requested_window_candles.len(),
            "history_start_date": candles.first().map(|item| item.trade_date.clone()),
            "history_end_date": candles.last().map(|item| item.trade_date.clone()),
            "requested_indicators": indicators,
            "indicators": indicator_items,
            "data_gap": if unavailable.is_empty() {
                Value::Null
            } else {
                json!({
                    "kind": "indicator_values_unavailable",
                    "message": "some technical indicators could not be computed from the fetched history",
                    "unavailable_indicators": unavailable,
                })
            }
        });
        Ok(ToolExecutionResult {
            output: serde_json::to_string_pretty(&body)?,
            meta: json!({
                "kind": "indicators",
                "source": if scenario_data.is_some_and(|data| !data.candles.is_empty()) { "prefetched_scenario" } else { "live_fetch" },
                "indicator_count": body["indicators"].as_array().map(|items| items.len()).unwrap_or_default(),
                "candle_count": candles.len(),
                "requested_window_candle_count": requested_window_candles.len(),
                "unavailable_indicator_count": unavailable.len(),
                "data_gap": body["data_gap"].clone(),
            }),
        })
    }

    pub(super) async fn get_fundamentals(
        &self,
        symbol: &str,
        scenario_data: Option<&ScenarioData>,
    ) -> anyhow::Result<ToolExecutionResult> {
        let item =
            if let Some(prefetched) = scenario_data.and_then(|data| data.fundamentals.clone()) {
                prefetched
            } else {
                self.market_data.fetch_fundamentals(symbol).await?
            };
        let payload = json!({
            "company_name": item.company_name,
            "industry": item.industry,
            "currency": item.currency,
            "fiscal_year_end": item.fiscal_year_end,
            "shares_outstanding": item.shares_outstanding,
            "diluted_shares_outstanding": item.diluted_shares_outstanding,
            "market_cap": item.market_cap,
            "net_income": item.net_income_usd,
            "revenues": item.revenues_usd,
            "assets": item.assets_usd,
            "liabilities": item.liabilities_usd,
            "stockholders_equity": item.stockholders_equity_usd,
            "cash_and_equivalents": item.cash_and_equivalents_usd,
            "gross_profit": item.gross_profit_usd,
            "operating_income": item.operating_income_usd,
            "operating_expenses": item.operating_expenses_usd,
            "operating_cash_flow": item.operating_cash_flow_usd,
            "capital_expenditure": item.capital_expenditure_usd,
            "free_cash_flow": item.free_cash_flow_usd,
            "long_term_debt": item.long_term_debt_usd,
            "current_debt": item.current_debt_usd,
            "total_debt": item.total_debt_usd
        });
        Ok(ToolExecutionResult {
            output: serde_json::to_string_pretty(&payload)?,
            meta: json!({
                "kind": "fundamentals",
                "source": if scenario_data.and_then(|data| data.fundamentals.as_ref()).is_some() { "prefetched_scenario" } else { "live_fetch" },
                "currency": payload["currency"],
            }),
        })
    }

    pub(super) async fn get_balance_sheet(
        &self,
        symbol: &str,
        scenario_data: Option<&ScenarioData>,
    ) -> anyhow::Result<ToolExecutionResult> {
        let item =
            if let Some(prefetched) = scenario_data.and_then(|data| data.fundamentals.clone()) {
                prefetched
            } else {
                self.market_data.fetch_fundamentals(symbol).await?
            };
        let payload = json!({
            "assets": item.assets_usd,
            "liabilities": item.liabilities_usd,
            "stockholders_equity": item.stockholders_equity_usd,
            "cash_and_equivalents": item.cash_and_equivalents_usd,
            "long_term_debt": item.long_term_debt_usd,
            "current_debt": item.current_debt_usd,
            "total_debt": item.total_debt_usd,
            "shares_outstanding": item.shares_outstanding,
            "diluted_shares_outstanding": item.diluted_shares_outstanding,
            "fiscal_year_end": item.fiscal_year_end
        });
        Ok(ToolExecutionResult {
            output: serde_json::to_string_pretty(&payload)?,
            meta: json!({
                "kind": "balance_sheet",
                "source": if scenario_data.and_then(|data| data.fundamentals.as_ref()).is_some() { "prefetched_scenario" } else { "live_fetch" },
            }),
        })
    }
}
