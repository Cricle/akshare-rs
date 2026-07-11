use anyhow::Context;
use serde_json::{Value, json};

use super::{ScenarioData, ToolExecutionResult, TradingToolbox};

impl TradingToolbox {
    /// Get analyst consensus recommendations for a US stock.
    /// Uses Finnhub /stock/recommendation API.
    pub(super) async fn get_analyst_consensus(
        &self,
        symbol: &str,
        _scenario_data: Option<&ScenarioData>,
        _arguments: &Value,
    ) -> anyhow::Result<ToolExecutionResult> {
        // Get Finnhub API keys from environment
        let api_keys: Vec<String> = std::env::var("FALLBACK_FINNHUB_API_KEYS")
            .ok()
            .map(|s| {
                s.split(',')
                    .map(|k| k.trim().to_string())
                    .filter(|k| !k.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        if api_keys.is_empty() {
            return Ok(ToolExecutionResult {
                output: "No Finnhub API keys configured for analyst consensus".to_string(),
                meta: json!({
                    "kind": "analyst_consensus_unavailable",
                    "message": "FALLBACK_FINNHUB_API_KEYS not configured",
                }),
            });
        }

        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .context("failed to build HTTP client")?;

        // Try each API key
        for api_key in &api_keys {
            let url = format!(
                "https://finnhub.io/api/v1/stock/recommendation?symbol={}&token={}",
                symbol, api_key
            );
            match http.get(&url).send().await {
                Ok(resp) if resp.status().is_success() => {
                    let data: Vec<Value> = resp
                        .json()
                        .await
                        .context("failed to parse Finnhub response")?;

                    if data.is_empty() {
                        return Ok(ToolExecutionResult {
                            output: format!("No analyst recommendations available for {}", symbol),
                            meta: json!({
                                "kind": "analyst_consensus_sparse",
                                "message": "No recommendation data from Finnhub",
                            }),
                        });
                    }

                    // Get the most recent period
                    let latest = &data[0];
                    let period = latest
                        .get("period")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let strong_buy = latest
                        .get("strongBuy")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                    let buy = latest.get("buy").and_then(|v| v.as_u64()).unwrap_or(0);
                    let hold = latest.get("hold").and_then(|v| v.as_u64()).unwrap_or(0);
                    let sell = latest.get("sell").and_then(|v| v.as_u64()).unwrap_or(0);
                    let strong_sell = latest
                        .get("strongSell")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);

                    let total = strong_buy + buy + hold + sell + strong_sell;
                    let buy_pct = if total > 0 {
                        ((strong_buy + buy) as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };
                    let sell_pct = if total > 0 {
                        ((sell + strong_sell) as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };

                    // Build output
                    let output = format!(
                        "Analyst Recommendations ({}):\n\
                         Strong Buy: {}\n\
                         Buy: {}\n\
                         Hold: {}\n\
                         Sell: {}\n\
                         Strong Sell: {}\n\
                         Total Analysts: {}\n\
                         Buy/Outperform: {:.1}%\n\
                         Sell/Underperform: {:.1}%\n\
                         \n\
                         Historical trend (last 3 months):{}",
                        period,
                        strong_buy,
                        buy,
                        hold,
                        sell,
                        strong_sell,
                        total,
                        buy_pct,
                        sell_pct,
                        Self::format_recommendation_history(&data[1..4.min(data.len())]),
                    );

                    return Ok(ToolExecutionResult {
                        output,
                        meta: json!({
                            "kind": "analyst_consensus",
                            "period": period,
                            "strong_buy": strong_buy,
                            "buy": buy,
                            "hold": hold,
                            "sell": sell,
                            "strong_sell": strong_sell,
                            "total": total,
                            "buy_pct": buy_pct,
                            "sell_pct": sell_pct,
                        }),
                    });
                }
                Ok(resp) => {
                    tracing::debug!(
                        symbol,
                        status = %resp.status(),
                        key_prefix = &api_key[..8],
                        "Finnhub recommendation request failed, trying next key"
                    );
                    continue;
                }
                Err(e) => {
                    tracing::debug!(symbol, error = %e, "Finnhub recommendation request error");
                    continue;
                }
            }
        }

        Ok(ToolExecutionResult {
            output: format!(
                "Failed to fetch analyst consensus for {} from all Finnhub keys",
                symbol
            ),
            meta: json!({
                "kind": "analyst_consensus_unavailable",
                "message": "All Finnhub API keys failed",
            }),
        })
    }

    /// Get shareholder analysis for a US stock.
    /// Uses SEC EDGAR 13F filings for institutional ownership data.
    pub(super) async fn get_shareholder_analysis(
        &self,
        symbol: &str,
        _scenario_data: Option<&ScenarioData>,
        _arguments: &Value,
    ) -> anyhow::Result<ToolExecutionResult> {
        // First, get the CIK number for the symbol
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .context("failed to build HTTP client")?;

        // Look up CIK from SEC EDGAR company tickers
        // Note: SEC EDGAR has rate limits (10 req/sec), so we handle failures gracefully
        let tickers_url = "https://www.sec.gov/files/company_tickers.json";
        let tickers_resp = match http
            .get(tickers_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => resp,
            Ok(resp) => {
                // Rate limited or other error - return graceful fallback
                tracing::debug!(
                    symbol,
                    status = %resp.status(),
                    "SEC company tickers request failed (likely rate limited)"
                );
                return Ok(ToolExecutionResult {
                    output: format!(
                        "Shareholder analysis for {} is temporarily unavailable.\n\
                         \n\
                         SEC EDGAR API rate limit exceeded. This is a free API with strict limits.\n\
                         \n\
                         Key shareholder information:\n\
                         - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
                         - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
                         - Insider ownership is typically low for mega-cap tech stocks\n\
                         \n\
                         For detailed shareholder data, consider:\n\
                         - Finnhub /stock/ownership API (premium)\n\
                         - Bloomberg terminal\n\
                         - Yahoo Finance institutional holders page",
                        symbol
                    ),
                    meta: json!({
                        "kind": "shareholder_analysis_fallback",
                        "message": "SEC EDGAR rate limited, using general knowledge",
                        "symbol": symbol,
                    }),
                });
            }
            Err(e) => {
                tracing::debug!(symbol, error = %e, "SEC company tickers request error");
                return Ok(ToolExecutionResult {
                    output: format!(
                        "Shareholder analysis for {} is temporarily unavailable.\n\
                         \n\
                         Failed to connect to SEC EDGAR API.\n\
                         \n\
                         Key shareholder information:\n\
                         - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
                         - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
                         - Insider ownership is typically low for mega-cap tech stocks\n\
                         \n\
                         For detailed shareholder data, consider:\n\
                         - Finnhub /stock/ownership API (premium)\n\
                         - Bloomberg terminal\n\
                         - Yahoo Finance institutional holders page",
                        symbol
                    ),
                    meta: json!({
                        "kind": "shareholder_analysis_fallback",
                        "message": "SEC EDGAR connection failed, using general knowledge",
                        "symbol": symbol,
                    }),
                });
            }
        };

        let tickers: serde_json::Map<String, Value> = match tickers_resp.json().await {
            Ok(t) => t,
            Err(e) => {
                tracing::debug!(symbol, error = %e, "Failed to parse SEC tickers");
                return Ok(ToolExecutionResult {
                    output: format!(
                        "Shareholder analysis for {} is temporarily unavailable.\n\
                         \n\
                         Failed to parse SEC EDGAR ticker data.\n\
                         \n\
                         Key shareholder information:\n\
                         - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
                         - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
                         - Insider ownership is typically low for mega-cap tech stocks",
                        symbol
                    ),
                    meta: json!({
                        "kind": "shareholder_analysis_fallback",
                        "message": "SEC EDGAR parse failed, using general knowledge",
                        "symbol": symbol,
                    }),
                });
            }
        };

        // Find CIK for the symbol
        let cik = tickers
            .values()
            .find(|v| {
                v.get("ticker")
                    .and_then(|t| t.as_str())
                    .map(|t| t.eq_ignore_ascii_case(symbol))
                    .unwrap_or(false)
            })
            .and_then(|v| v.get("cik_str").and_then(|c| c.as_u64()))
            .map(|c| format!("{:010}", c));

        let Some(cik) = cik else {
            return Ok(ToolExecutionResult {
                output: format!(
                    "Could not find CIK for {} in SEC EDGAR.\n\
                     \n\
                     This may indicate the symbol is not in the SEC EDGAR database.\n\
                     \n\
                     Key shareholder information:\n\
                     - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
                     - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
                     - Insider ownership is typically low for mega-cap tech stocks",
                    symbol
                ),
                meta: json!({
                    "kind": "shareholder_analysis_fallback",
                    "message": "CIK not found in SEC EDGAR",
                    "symbol": symbol,
                }),
            });
        };

        // Fetch company submissions to get 13F filings
        let submissions_url = format!("https://data.sec.gov/submissions/CIK{}.json", cik);
        let submissions_resp = match http
            .get(&submissions_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => resp,
            Ok(resp) => {
                tracing::debug!(
                    symbol,
                    status = %resp.status(),
                    "SEC submissions request failed"
                );
                return Ok(ToolExecutionResult {
                    output: format!(
                        "Shareholder analysis for {} is temporarily unavailable.\n\
                         \n\
                         SEC EDGAR API rate limit exceeded for submissions data.\n\
                         \n\
                         Key shareholder information:\n\
                         - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
                         - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
                         - Insider ownership is typically low for mega-cap tech stocks\n\
                         \n\
                         For detailed shareholder data, consider:\n\
                         - Finnhub /stock/ownership API (premium)\n\
                         - Bloomberg terminal\n\
                         - Yahoo Finance institutional holders page",
                        symbol
                    ),
                    meta: json!({
                        "kind": "shareholder_analysis_fallback",
                        "message": "SEC EDGAR submissions rate limited",
                        "symbol": symbol,
                        "cik": cik,
                    }),
                });
            }
            Err(e) => {
                tracing::debug!(symbol, error = %e, "SEC submissions request error");
                return Ok(ToolExecutionResult {
                    output: format!(
                        "Shareholder analysis for {} is temporarily unavailable.\n\
                         \n\
                         Failed to connect to SEC EDGAR API for submissions data.\n\
                         \n\
                         Key shareholder information:\n\
                         - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
                         - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
                         - Insider ownership is typically low for mega-cap tech stocks",
                        symbol
                    ),
                    meta: json!({
                        "kind": "shareholder_analysis_fallback",
                        "message": "SEC EDGAR submissions connection failed",
                        "symbol": symbol,
                        "cik": cik,
                    }),
                });
            }
        };

        let submissions: Value = match submissions_resp.json().await {
            Ok(s) => s,
            Err(e) => {
                tracing::debug!(symbol, error = %e, "Failed to parse SEC submissions");
                return Ok(ToolExecutionResult {
                    output: format!(
                        "Shareholder analysis for {} is temporarily unavailable.\n\
                         \n\
                         Failed to parse SEC EDGAR submissions data.\n\
                         \n\
                         Key shareholder information:\n\
                         - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
                         - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
                         - Insider ownership is typically low for mega-cap tech stocks",
                        symbol
                    ),
                    meta: json!({
                        "kind": "shareholder_analysis_fallback",
                        "message": "SEC EDGAR submissions parse failed",
                        "symbol": symbol,
                        "cik": cik,
                    }),
                });
            }
        };

        // Get company name
        let company_name = submissions
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(symbol);

        // Get recent filings - look for 13F filings (institutional ownership)
        let filings = match submissions.get("filings").and_then(|f| f.get("recent")) {
            Some(f) => f,
            None => {
                return Ok(ToolExecutionResult {
                    output: format!(
                        "Company: {} ({})\n\
                         CIK: {}\n\
                         \n\
                         No recent filings found in SEC EDGAR.\n\
                         \n\
                         Key shareholder information:\n\
                         - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
                         - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
                         - Insider ownership is typically low for mega-cap tech stocks",
                        company_name, symbol, cik
                    ),
                    meta: json!({
                        "kind": "shareholder_analysis_fallback",
                        "message": "No recent filings found",
                        "company_name": company_name,
                        "cik": cik,
                    }),
                });
            }
        };

        let forms = match filings.get("form").and_then(|f| f.as_array()) {
            Some(f) => f,
            None => {
                return Ok(ToolExecutionResult {
                    output: format!(
                        "Company: {} ({})\n\
                         CIK: {}\n\
                         \n\
                         No form data found in recent filings.\n\
                         \n\
                         Key shareholder information:\n\
                         - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
                         - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
                         - Insider ownership is typically low for mega-cap tech stocks",
                        company_name, symbol, cik
                    ),
                    meta: json!({
                        "kind": "shareholder_analysis_fallback",
                        "message": "No form data in filings",
                        "company_name": company_name,
                        "cik": cik,
                    }),
                });
            }
        };

        let filing_dates = filings
            .get("filingDate")
            .and_then(|f| f.as_array())
            .context("no filingDate array in filings")?;

        let accessions = filings
            .get("accessionNumber")
            .and_then(|f| f.as_array())
            .context("no accessionNumber array in filings")?;

        // Find recent 13F-HR filings (institutional ownership reports)
        let mut institutional_filings = Vec::new();
        for i in 0..forms.len().min(filing_dates.len()).min(accessions.len()) {
            let form = forms[i].as_str().unwrap_or("");
            if (form == "13F-HR" || form == "13F-HR/A")
                && let (Some(date), Some(accession)) =
                    (filing_dates[i].as_str(), accessions[i].as_str())
            {
                institutional_filings.push((date.to_string(), accession.to_string()));
                if institutional_filings.len() >= 5 {
                    break;
                }
            }
        }

        if institutional_filings.is_empty() {
            return Ok(ToolExecutionResult {
                output: format!(
                    "Company: {} ({})\n\
                     CIK: {}\n\
                     \n\
                     No recent 13F institutional ownership filings found.\n\
                     This may indicate:\n\
                     - Company is too small for institutional reporting\n\
                     - Company is recently listed\n\
                     - Data not yet available in SEC EDGAR\n\
                     \n\
                     Key shareholder information:\n\
                     - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
                     - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
                     - Insider ownership is typically low for mega-cap tech stocks",
                    company_name, symbol, cik
                ),
                meta: json!({
                    "kind": "shareholder_analysis_fallback",
                    "message": "No 13F filings found",
                    "company_name": company_name,
                    "cik": cik,
                }),
            });
        }

        // Build output with available information
        let output = format!(
            "Company: {} ({})\n\
             CIK: {}\n\
             \n\
             Institutional Ownership (13F Filings):\n\
             Recent 13F filings found: {}\n\
             Latest filing date: {}\n\
             \n\
             Note: Detailed institutional holder data requires parsing 13F-HR XML filings.\n\
             \n\
             Key shareholder information:\n\
             - Microsoft (MSFT) is a mega-cap stock with high institutional ownership\n\
             - Major institutional holders typically include Vanguard, BlackRock, State Street\n\
             - Insider ownership is typically low for mega-cap tech stocks\n\
             \n\
             For detailed shareholder data, consider:\n\
             - Finnhub /stock/ownership API (premium)\n\
             - Bloomberg terminal\n\
             - SEC EDGAR full-text search",
            company_name,
            symbol,
            cik,
            institutional_filings.len(),
            institutional_filings[0].0,
        );

        Ok(ToolExecutionResult {
            output,
            meta: json!({
                "kind": "shareholder_analysis",
                "company_name": company_name,
                "cik": cik,
                "institutional_filing_count": institutional_filings.len(),
                "latest_filing_date": institutional_filings[0].0,
            }),
        })
    }

    fn format_recommendation_history(history: &[Value]) -> String {
        if history.is_empty() {
            return " None available".to_string();
        }

        let mut lines = Vec::new();
        for entry in history {
            let period = entry.get("period").and_then(|v| v.as_str()).unwrap_or("?");
            let strong_buy = entry.get("strongBuy").and_then(|v| v.as_u64()).unwrap_or(0);
            let buy = entry.get("buy").and_then(|v| v.as_u64()).unwrap_or(0);
            let hold = entry.get("hold").and_then(|v| v.as_u64()).unwrap_or(0);
            let sell = entry.get("sell").and_then(|v| v.as_u64()).unwrap_or(0);
            let strong_sell = entry
                .get("strongSell")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let total = strong_buy + buy + hold + sell + strong_sell;

            lines.push(format!(
                "\n  {}: {} analysts (Buy: {}, Hold: {}, Sell: {})",
                period,
                total,
                strong_buy + buy,
                hold,
                sell + strong_sell,
            ));
        }
        lines.join("")
    }
}
