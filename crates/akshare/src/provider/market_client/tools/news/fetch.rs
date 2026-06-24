impl TradingToolbox {
    fn news_gap_meta(
        kind: &str,
        message: String,
        attempts: &[crate::types::NewsFetchAttempt],
        cacheable: bool,
    ) -> Value {
        let failed_attempts = attempts.iter().filter(|attempt| !attempt.success).count();
        let successful_attempts = attempts.iter().filter(|attempt| attempt.success).count();
        json!({
            "kind": kind,
            "message": message,
            "failed_attempt_count": failed_attempts,
            "successful_attempt_count": successful_attempts,
            "cacheable": cacheable,
        })
    }

    pub(super) async fn get_news(
        &self,
        symbol: &str,
        scenario_data: Option<&ScenarioData>,
        arguments: &Value,
    ) -> anyhow::Result<ToolExecutionResult> {
        let limit = arguments
            .get("limit")
            .and_then(Value::as_u64)
            .map(|value| value as usize)
            .unwrap_or(12);
        let start_date = arguments
            .get("start_date")
            .and_then(Value::as_str)
            .or_else(|| arguments.get("from").and_then(Value::as_str));
        let end_date = arguments
            .get("end_date")
            .and_then(Value::as_str)
            .or_else(|| arguments.get("to").and_then(Value::as_str));
        let query = arguments
            .get("query")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let (mut items, raw_items, attempts, cacheable, source_tag) = if let Some(prefetched) =
            scenario_data.filter(|data| !data.company_news.is_empty())
        {
            let prefetched_items = self.filter_news_by_date(
                prefetched.company_news.clone(),
                start_date,
                end_date,
            );
            let prefetched_sources: std::collections::HashSet<_> =
                prefetched_items.iter().map(|i| i.source.as_str()).collect();
            // When pre-fetched data is critically sparse (< 5 items or <= 1 source),
            // fall through to live fetch to try to supplement.
            if prefetched_items.len() < 5 || prefetched_sources.len() <= 1 {
                match self
                    .market_data
                    .fetch_news_with_diagnostics_query(
                        symbol,
                        limit * 3,
                        start_date,
                        end_date,
                        query,
                    )
                    .await
                {
                    Ok(diagnostics) if diagnostics.items.len() > prefetched_items.len() => {
                        let existing_titles: std::collections::HashSet<_> =
                            prefetched_items.iter().map(|i| i.title.to_lowercase()).collect();
                        let mut merged = prefetched_items;
                        let mut live_attempts = diagnostics.attempts;
                        for item in diagnostics.items {
                            if !existing_titles.contains(&item.title.to_lowercase()) {
                                merged.push(item);
                            }
                        }
                        live_attempts.insert(
                            0,
                            crate::types::NewsFetchAttempt {
                                source: "prefetched_scenario".to_string(),
                                query: query.map(str::to_string),
                                success: true,
                                item_count: prefetched.company_news.len(),
                                error: None,
                            },
                        );
                        (
                            merged.into_iter().take(limit).collect(),
                            prefetched.company_news.clone(),
                            live_attempts,
                            diagnostics.cacheable,
                            "prefetched_supplemented".to_string(),
                        )
                    }
                    _ => (
                        prefetched_items.into_iter().take(limit).collect(),
                        prefetched.company_news.clone(),
                        vec![crate::types::NewsFetchAttempt {
                            source: "prefetched_scenario".to_string(),
                            query: query.map(str::to_string),
                            success: true,
                            item_count: prefetched.company_news.len(),
                            error: None,
                        }],
                        true,
                        "prefetched_scenario".to_string(),
                    ),
                }
            } else {
                (
                    prefetched_items.into_iter().take(limit).collect(),
                    prefetched.company_news.clone(),
                    vec![crate::types::NewsFetchAttempt {
                        source: "prefetched_scenario".to_string(),
                        query: query.map(str::to_string),
                        success: true,
                        item_count: prefetched.company_news.len(),
                        error: None,
                    }],
                    true,
                    "prefetched_scenario".to_string(),
                )
            }
        } else {
            let diagnostics = self
                .market_data
                .fetch_news_with_diagnostics_query(symbol, limit * 3, start_date, end_date, query)
                .await?;
            (
                self.filter_news_by_date(diagnostics.items.clone(), start_date, end_date)
                    .into_iter()
                    .take(limit)
                    .collect::<Vec<_>>(),
                diagnostics.items,
                diagnostics.attempts,
                diagnostics.cacheable,
                "live_fetch".to_string(),
            )
        };
        if items.is_empty() && raw_items.iter().any(|item| item.source == "hkexnews.hk") {
            items = raw_items.iter().take(limit).cloned().collect();
        }
        let output = Self::format_news_items(&items);
        let sources = Self::unique_news_sources(&items);
        let fallback_kind = attempts
            .iter()
            .find(|attempt| attempt.success)
            .and_then(|attempt| match attempt.source.as_str() {
                "HKEX Recent High-Value" => Some("hkex_recent_high_value"),
                _ => None,
            });
        let fallback_used = fallback_kind.is_some();
        let failed_attempt_count = attempts.iter().filter(|attempt| !attempt.success).count();
        let source_count = sources.len();
        let data_gap = if items.is_empty() {
            Some(Self::news_gap_meta(
                "news_unavailable",
                "no company news items available after filtering".to_string(),
                &attempts,
                cacheable,
            ))
        } else if items.len() < limit.min(3) || source_count <= 1 {
            Some(Self::news_gap_meta(
                "news_sparse_coverage",
                format!(
                    "company news coverage is narrow; returned {} items across {} source(s)",
                    items.len(),
                    source_count
                ),
                &attempts,
                cacheable,
            ))
        } else if failed_attempt_count > 0 {
            Some(Self::news_gap_meta(
                "news_partial_source_failure",
                format!(
                    "company news is only partially available; {} upstream attempts failed",
                    failed_attempt_count
                ),
                &attempts,
                cacheable,
            ))
        } else if !cacheable {
            Some(Self::news_gap_meta(
                "news_not_cacheable",
                "company news result is degraded and not cacheable".to_string(),
                &attempts,
                cacheable,
            ))
        } else {
            None
        };
        Ok(ToolExecutionResult {
            output,
            meta: json!({
                "kind": "company_news",
                "source": source_tag,
                "item_count": items.len(),
                "limit": limit,
                "start_date": start_date,
                "end_date": end_date,
                "query": query,
                "sources": sources,
                "attempts": attempts,
                "cacheable": cacheable,
                "fallback_used": fallback_used,
                "fallback_kind": fallback_kind,
                "data_gap": data_gap.unwrap_or(Value::Null)
            }),
        })
    }

    pub(super) async fn get_global_news(
        &self,
        symbol: &str,
        market_type: &str,
        arguments: &Value,
    ) -> anyhow::Result<ToolExecutionResult> {
        let limit = arguments
            .get("limit")
            .and_then(Value::as_u64)
            .map(|value| value as usize)
            .unwrap_or(5);
        let curr_date = arguments.get("curr_date").and_then(Value::as_str);
        let look_back_days = arguments
            .get("look_back_days")
            .and_then(Value::as_i64)
            .unwrap_or(7);
        let start_date = curr_date.and_then(|date| {
            chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
                .ok()
                .map(|parsed| {
                    (parsed - chrono::Days::new(look_back_days.max(0) as u64))
                        .format("%Y-%m-%d")
                        .to_string()
                })
        });
        let scope = arguments
            .get("scope")
            .and_then(Value::as_str)
            .unwrap_or("macro");
        let curr_date = curr_date.context("curr_date is required for get_global_news")?;
        let diagnostics = self
            .market_data
            .fetch_global_news_with_diagnostics(
                symbol,
                curr_date,
                look_back_days as usize,
                limit * 2,
            )
            .await?;
        let items = self
            .filter_news_by_date(
                diagnostics.items.clone(),
                start_date.as_deref(),
                Some(curr_date),
            )
            .into_iter()
            .take(limit)
            .collect::<Vec<_>>();
        let sources = Self::unique_news_sources(&items);
        let failed_attempt_count = diagnostics
            .attempts
            .iter()
            .filter(|attempt| !attempt.success)
            .count();
        let source_count = sources.len();
        let output = format!(
            "Global/Macro proxy for {market_type} (scope={scope}, curr_date={}, look_back_days={}):\n{}",
            curr_date,
            look_back_days,
            Self::format_news_items(&items)
        );
        let data_gap = if items.is_empty() {
            Some(Self::news_gap_meta(
                "global_news_unavailable",
                "no global/macro news items available after filtering".to_string(),
                &diagnostics.attempts,
                diagnostics.cacheable,
            ))
        } else if items.len() < limit.min(3) || source_count <= 1 {
            Some(Self::news_gap_meta(
                "global_news_sparse_coverage",
                format!(
                    "global/macro news coverage is narrow; returned {} items across {} source(s)",
                    items.len(),
                    source_count
                ),
                &diagnostics.attempts,
                diagnostics.cacheable,
            ))
        } else if failed_attempt_count > 0 {
            Some(Self::news_gap_meta(
                "global_news_partial_source_failure",
                format!(
                    "global/macro news is only partially available; {} upstream attempts failed",
                    failed_attempt_count
                ),
                &diagnostics.attempts,
                diagnostics.cacheable,
            ))
        } else if !diagnostics.cacheable {
            Some(Self::news_gap_meta(
                "global_news_not_cacheable",
                "global/macro news result is degraded and not cacheable".to_string(),
                &diagnostics.attempts,
                diagnostics.cacheable,
            ))
        } else {
            None
        };
        Ok(ToolExecutionResult {
            output,
            meta: json!({
                "kind": "global_news",
                "scope": scope,
                "market_type": market_type,
                "item_count": items.len(),
                "limit": limit,
                "curr_date": curr_date,
                "look_back_days": look_back_days,
                "sources": sources,
                "attempts": diagnostics.attempts,
                "cacheable": diagnostics.cacheable,
                "fallback_used": false,
                "data_gap": data_gap.unwrap_or(Value::Null)
            }),
        })
    }

    pub(super) async fn get_insider_transactions(
        &self,
        symbol: &str,
        _scenario_data: Option<&ScenarioData>,
        arguments: &Value,
    ) -> anyhow::Result<ToolExecutionResult> {
        let limit = arguments
            .get("limit")
            .and_then(Value::as_u64)
            .map(|value| value as usize)
            .unwrap_or(12);
        let start_date = arguments
            .get("start_date")
            .and_then(Value::as_str)
            .or_else(|| arguments.get("from").and_then(Value::as_str));
        let end_date = arguments
            .get("end_date")
            .and_then(Value::as_str)
            .or_else(|| arguments.get("to").and_then(Value::as_str));
        let requested_symbol = arguments
            .get("ticker")
            .or_else(|| arguments.get("symbol"))
            .and_then(Value::as_str)
            .unwrap_or(symbol);
        let fundamentals = self.market_data.fetch_fundamentals(symbol).await?;
        let insider_fetch = self
            .market_data
            .fetch_insider_transactions(requested_symbol)
            .await;
        let (insider_items, data_gap) = match insider_fetch {
            Ok(items) => {
                let filtered = self
                    .filter_news_by_date(items, start_date, end_date)
                    .into_iter()
                    .take(limit)
                    .collect::<Vec<_>>();
                let gap = if filtered.is_empty() {
                    Some(json!({
                        "kind": "insider_transactions_sparse",
                        "message": "no insider-related items remained after date filtering",
                    }))
                } else {
                    None
                };
                (filtered, gap)
            }
            Err(error) => (
                Vec::new(),
                Some(json!({
                    "kind": "insider_transactions_unavailable",
                    "message": error.to_string(),
                })),
            ),
        };
        let output = format!(
            "Requested ticker={}\nCompany snapshot: company={}, market_cap={:?}, shares_outstanding={:?}\nEvent proxy:\n{}",
            requested_symbol,
            fundamentals.company_name,
            fundamentals.market_cap,
            fundamentals.shares_outstanding,
            Self::format_news_items(&insider_items)
        );
        Ok(ToolExecutionResult {
            output,
            meta: json!({
                "kind": "insider_transactions",
                "item_count": insider_items.len(),
                "requested_symbol": requested_symbol,
                "sources": Self::unique_news_sources(&insider_items),
                "data_gap": data_gap.unwrap_or(Value::Null),
            }),
        })
    }
}
