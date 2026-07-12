use rmcp::schemars;

// ── Existing ────────────────────────────────────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for nlp queries.
pub struct NlpParams {
    pub question: String,
}

// ── New param types ─────────────────────────────────────────────

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for city queries.
pub struct CityParam {
    pub city: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for date city queries.
pub struct DateCityParam {
    pub date: String,
    pub city: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for air hist queries.
pub struct AirHistParam {
    pub city: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol queries.
pub struct SymbolParam {
    pub symbol: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for date queries.
pub struct DateParam {
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for year queries.
pub struct YearParam {
    pub year: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol indicator queries.
pub struct SymbolIndicatorParam {
    pub symbol: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol index queries.
pub struct SymbolIndexParam {
    pub symbol: String,
    pub index: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for migration area queries.
pub struct MigrationAreaParam {
    pub area: String,
    pub indicator: String,
    pub date: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for migration scale queries.
pub struct MigrationScaleParam {
    pub area: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for hurun queries.
pub struct HurunParam {
    pub indicator: String,
    pub year: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for nlp ownthink queries.
pub struct NlpOwnthinkParam {
    pub word: String,
    pub indicator: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
/// MCP tool parameters for symbol date queries.
pub struct SymbolDateParam {
    pub symbol: String,
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nlp_params() {
        let json = r#"{"question": "What is the PE ratio of Apple?"}"#;
        let params: NlpParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.question, "What is the PE ratio of Apple?");
    }

    #[test]
    fn test_city_param() {
        let json = r#"{"city": "Beijing"}"#;
        let params: CityParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.city, "Beijing");
    }

    #[test]
    fn test_date_city_param() {
        let json = r#"{"date": "20240101", "city": "Shanghai"}"#;
        let params: DateCityParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.date, "20240101");
        assert_eq!(params.city, "Shanghai");
    }

    #[test]
    fn test_symbol_param() {
        let json = r#"{"symbol": "China"}"#;
        let params: SymbolParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "China");
    }

    #[test]
    fn test_date_param() {
        let json = r#"{"date": "20240101"}"#;
        let params: DateParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.date, "20240101");
    }

    #[test]
    fn test_year_param() {
        let json = r#"{"year": "2024"}"#;
        let params: YearParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.year, "2024");
    }

    #[test]
    fn test_symbol_indicator_param() {
        let json = r#"{"symbol": "轿车", "indicator": "零售"}"#;
        let params: SymbolIndicatorParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "轿车");
        assert_eq!(params.indicator, "零售");
    }

    #[test]
    fn test_symbol_index_param() {
        let json = r#"{"symbol": "FTSE", "index": "rv5"}"#;
        let params: SymbolIndexParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "FTSE");
        assert_eq!(params.index, "rv5");
    }

    #[test]
    fn test_migration_area_param() {
        let json = r#"{"area": "广州市", "indicator": "move_in", "date": "20240101"}"#;
        let params: MigrationAreaParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.area, "广州市");
        assert_eq!(params.indicator, "move_in");
        assert_eq!(params.date, "20240101");
    }

    #[test]
    fn test_migration_scale_param() {
        let json = r#"{"area": "广州市", "indicator": "move_out"}"#;
        let params: MigrationScaleParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.area, "广州市");
        assert_eq!(params.indicator, "move_out");
    }

    #[test]
    fn test_hurun_param() {
        let json = r#"{"indicator": "胡润百富榜", "year": "2023"}"#;
        let params: HurunParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.indicator, "胡润百富榜");
        assert_eq!(params.year, "2023");
    }

    #[test]
    fn test_nlp_ownthink_param() {
        let json = r#"{"word": "人工智能", "indicator": "desc"}"#;
        let params: NlpOwnthinkParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.word, "人工智能");
        assert_eq!(params.indicator, "desc");
    }

    #[test]
    fn test_symbol_date_param() {
        let json = r#"{"symbol": "车企榜", "date": "202401"}"#;
        let params: SymbolDateParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.symbol, "车企榜");
        assert_eq!(params.date, "202401");
    }

    #[test]
    fn test_air_hist_param() {
        let json = r#"{"city": "Beijing", "start_date": "20240101", "end_date": "20240131"}"#;
        let params: AirHistParam = serde_json::from_str(json).unwrap();
        assert_eq!(params.city, "Beijing");
        assert_eq!(params.start_date, "20240101");
        assert_eq!(params.end_date, "20240131");
    }
}
