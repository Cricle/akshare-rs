use rmcp::schemars;

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct MacroDateParams {
    /// Date in YYYYMMDD format
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct MacroDateRangeParams {
    /// Start date in YYYYMMDD format
    pub start_date: String,
    /// End date in YYYYMMDD format
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct MacroSymbolParams {
    /// Symbol name, e.g. "回购定盘利率" or "银银间回购定盘利率"
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct MacroNbsNationParams {
    /// Data kind: "月度数据", "季度数据", or "年度数据"
    pub kind: String,
    /// Indicator code path
    pub path: String,
    /// Period code
    pub period: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct MacroNbsRegionParams {
    /// Data kind: "分省月度数据", "分省季度数据", or "分省年度数据"
    pub kind: String,
    /// Indicator code path
    pub path: String,
    /// Region indicator code
    pub indicator: String,
    /// Period code
    pub period: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct MacroInterbankParams {
    /// Market name, e.g. "上海银行同业拆借市场", "中国银行同业拆借市场", etc.
    pub market: String,
    /// Currency symbol, e.g. "Shibor人民币", "Libor美元", etc.
    pub symbol: String,
    /// Tenor indicator, e.g. "隔夜", "1周", "1月", "1年", etc.
    pub indicator: String,
}
