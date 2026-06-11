use crate::types::MarketKind;

/// Detect market from symbol. Returns None if unrecognized.
pub fn detect_market(symbol: &str) -> MarketKind {
    if normalize_a_share_symbol(symbol).is_some() {
        return MarketKind::AShare;
    }
    if normalize_hk_symbol(symbol).is_some() {
        return MarketKind::HongKong;
    }
    MarketKind::UsEquity
}

/// Normalize A-share symbol to ts_code format (e.g., "600000" -> "600000.SH").
/// Returns None if not an A-share symbol.
pub fn normalize_a_share_symbol(symbol: &str) -> Option<String> {
    let trimmed = symbol.trim();
    // Already normalized: 600000.SH, 000001.SZ, etc.
    if let Some((code, suffix)) = trimmed.split_once('.') {
        let suffix_upper = suffix.to_uppercase();
        if matches!(suffix_upper.as_str(), "SH" | "SZ" | "BJ")
            && code.len() == 6
            && code.chars().all(|c| c.is_ascii_digit())
        {
            return Some(format!("{}.{}", code, suffix_upper));
        }
    }
    // Pure numeric: infer exchange
    if trimmed.len() == 6 && trimmed.chars().all(|c| c.is_ascii_digit()) {
        let suffix = infer_a_share_exchange(trimmed)?;
        return Some(format!("{}.{}", trimmed, suffix));
    }
    // Tencent format: sh600000, sz000001
    let lower = trimmed.to_lowercase();
    if lower.starts_with("sh") && lower.len() == 8 {
        let code = &lower[2..];
        if code.chars().all(|c| c.is_ascii_digit()) && code.len() == 6 {
            return Some(format!("{}.SH", code));
        }
    }
    if lower.starts_with("sz") && lower.len() == 8 {
        let code = &lower[2..];
        if code.chars().all(|c| c.is_ascii_digit()) && code.len() == 6 {
            return Some(format!("{}.SZ", code));
        }
    }
    None
}

fn infer_a_share_exchange(code: &str) -> Option<&'static str> {
    match code {
        // Shanghai: 60xxxx, 68xxxx (STAR Market), 900xxx (B-share)
        s if s.starts_with('6') => Some("SH"),
        // Shenzhen: 00xxxx, 30xxxx (ChiNext), 200xxx (B-share)
        s if s.starts_with('0') || s.starts_with('3') || s.starts_with('2') => Some("SZ"),
        // Beijing: 8xxxxx, 4xxxxx
        s if s.starts_with('8') || s.starts_with('4') => Some("BJ"),
        _ => None,
    }
}

/// Normalize HK symbol. Returns None if not a HK symbol.
pub fn normalize_hk_symbol(symbol: &str) -> Option<String> {
    let trimmed = symbol.trim().trim_start_matches('0').to_string();
    // HK stocks are 1-5 digit numeric codes
    if !trimmed.is_empty() && trimmed.len() <= 5 && trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Some(format!("{:0>5}", trimmed));
    }
    // Check for explicit .HK suffix
    if let Some((code, suffix)) = symbol.trim().split_once('.') {
        if suffix.eq_ignore_ascii_case("HK") {
            let code_trimmed = code.trim_start_matches('0');
            if !code_trimmed.is_empty()
                && code_trimmed.len() <= 5
                && code_trimmed.chars().all(|c| c.is_ascii_digit())
            {
                return Some(format!("{:0>5}", code_trimmed));
            }
        }
    }
    None
}

/// Tencent market symbol format: sh600000, sz000001
pub fn tencent_market_symbol(symbol: &str) -> crate::error::Result<String> {
    let normalized = normalize_a_share_symbol(symbol).ok_or_else(|| {
        crate::error::Error::invalid_input(format!("invalid A-share symbol: {symbol}"))
    })?;
    let (code, suffix) = normalized.split_once('.').ok_or_else(|| {
        crate::error::Error::invalid_input(format!("invalid symbol format: {normalized}"))
    })?;
    let prefix = match suffix {
        "SH" => "sh",
        "SZ" | "BJ" => "sz",
        _ => {
            return Err(crate::error::Error::invalid_input(format!(
                "unsupported suffix: {suffix}"
            )));
        }
    };
    Ok(format!("{prefix}{code}"))
}

/// Eastmoney secid format: 1.600000 (SH), 0.000001 (SZ)
pub fn eastmoney_secid(symbol: &str) -> crate::error::Result<String> {
    let normalized = normalize_a_share_symbol(symbol).ok_or_else(|| {
        crate::error::Error::invalid_input(format!("invalid A-share symbol: {symbol}"))
    })?;
    let (code, suffix) = normalized.split_once('.').ok_or_else(|| {
        crate::error::Error::invalid_input(format!("invalid symbol format: {normalized}"))
    })?;
    let market = match suffix {
        "SH" => "1",
        "SZ" | "BJ" => "0",
        _ => {
            return Err(crate::error::Error::invalid_input(format!(
                "unsupported suffix: {suffix}"
            )));
        }
    };
    Ok(format!("{market}.{code}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_a_share() {
        assert_eq!(
            normalize_a_share_symbol("600000"),
            Some("600000.SH".to_string())
        );
        assert_eq!(
            normalize_a_share_symbol("000001"),
            Some("000001.SZ".to_string())
        );
        assert_eq!(
            normalize_a_share_symbol("600000.SH"),
            Some("600000.SH".to_string())
        );
        assert_eq!(
            normalize_a_share_symbol("sh600000"),
            Some("600000.SH".to_string())
        );
        assert_eq!(normalize_a_share_symbol("AAPL"), None);
    }

    #[test]
    fn test_normalize_hk() {
        assert_eq!(normalize_hk_symbol("00593"), Some("00593".to_string()));
        assert_eq!(normalize_hk_symbol("593"), Some("00593".to_string()));
        assert_eq!(normalize_hk_symbol("00593.HK"), Some("00593".to_string()));
    }

    #[test]
    fn test_detect_market() {
        assert_eq!(detect_market("600000"), MarketKind::AShare);
        assert_eq!(detect_market("00593"), MarketKind::HongKong);
        assert_eq!(detect_market("AAPL"), MarketKind::UsEquity);
    }
}
