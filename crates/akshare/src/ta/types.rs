/// Bollinger Bands: (middle, upper, lower).
#[derive(Debug, Clone, Copy)]
pub struct BollingerBands {
    pub middle: f64,
    pub upper: f64,
    pub lower: f64,
}

/// MACD values: (macd_line, signal_line, histogram).
#[derive(Debug, Clone, Copy)]
pub struct MacdValues {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

/// KDJ values: (K, D, J).
#[derive(Debug, Clone, Copy)]
pub struct KdjValues {
    pub k: f64,
    pub d: f64,
    pub j: f64,
}

/// OBV values: (current_obv, obv_change).
#[derive(Debug, Clone, Copy)]
pub struct ObvValues {
    pub obv: f64,
    pub change: f64,
}
