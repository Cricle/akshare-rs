impl TradingToolbox {

    pub(super) async fn get_cashflow(
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
            "operating_cash_flow": item.operating_cash_flow_usd,
            "capital_expenditure": item.capital_expenditure_usd,
            "free_cash_flow": item.free_cash_flow_usd,
            "cash_and_equivalents": item.cash_and_equivalents_usd,
            "net_income": item.net_income_usd
        });
        Ok(ToolExecutionResult {
            output: serde_json::to_string_pretty(&payload)?,
            meta: json!({
                "kind": "cashflow",
                "source": if scenario_data.and_then(|data| data.fundamentals.as_ref()).is_some() { "prefetched_scenario" } else { "live_fetch" },
            }),
        })
    }

    pub(super) async fn get_income_statement(
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
            "revenues": item.revenues_usd,
            "net_income": item.net_income_usd,
            "gross_profit": item.gross_profit_usd,
            "operating_income": item.operating_income_usd,
            "operating_expenses": item.operating_expenses_usd,
            "currency": item.currency
        });
        Ok(ToolExecutionResult {
            output: serde_json::to_string_pretty(&payload)?,
            meta: json!({
                "kind": "income_statement",
                "source": if scenario_data.and_then(|data| data.fundamentals.as_ref()).is_some() { "prefetched_scenario" } else { "live_fetch" },
            }),
        })
    }
}
