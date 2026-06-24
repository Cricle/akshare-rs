use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct YahooChartResponse {
    pub(super) chart: YahooChartRoot,
}

#[derive(Debug, Deserialize)]
pub(super) struct YahooChartRoot {
    pub(super) result: Option<Vec<YahooChartResult>>,
}

#[derive(Debug, Deserialize)]
pub(super) struct YahooChartResult {
    pub(super) timestamp: Option<Vec<i64>>,
    pub(super) indicators: YahooChartIndicators,
}

#[derive(Debug, Deserialize)]
pub(super) struct YahooChartIndicators {
    pub(super) quote: Option<Vec<YahooChartQuote>>,
}

#[derive(Debug, Deserialize)]
pub(super) struct YahooChartQuote {
    pub(super) open: Option<Vec<Option<f64>>>,
    pub(super) high: Option<Vec<Option<f64>>>,
    pub(super) low: Option<Vec<Option<f64>>>,
    pub(super) close: Option<Vec<Option<f64>>>,
    pub(super) volume: Option<Vec<Option<i64>>>,
}
