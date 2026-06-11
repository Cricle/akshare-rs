//! ChinaMoney bond information query (中国外汇交易中心-债券信息).

use crate::error::{Error, Result};

/// 中国外汇交易中心-查询相关指标的参数 (Python: bond_info_cm_query)
///
/// `symbol` is one of: "主承销商", "债券类型", "息票类型", "发行年份", "评级等级"
pub async fn bond_info_cm_query(symbol: &str) -> Result<Vec<serde_json::Value>> {
    let url = match symbol {
        "主承销商" => {
            "https://www.chinamoney.com.cn/ags/ms/cm-u-bond-md/EntyFullNameSearchCondition"
        }
        _ => "https://www.chinamoney.com.cn/ags/ms/cm-u-bond-md/BondBaseInfoSearchCondition",
    };
    let resp = reqwest::Client::new()
        .post(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    if symbol == "主承销商" {
        let data = resp
            .get("data")
            .and_then(|d| d.get("enty"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(data)
    } else {
        let key = match symbol {
            "债券类型" => "bondType",
            "息票类型" => "couponType",
            "发行年份" => "issueYear",
            "评级等级" => "bondRtngShrt",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported query symbol: {symbol}"
                )));
            }
        };
        let data = resp
            .get("data")
            .and_then(|d| d.get(key))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(data)
    }
}

/// 中国外汇交易中心-债券信息查询 (Python: bond_info_cm)
///
#[allow(clippy::too_many_arguments)]
/// Search bonds by various criteria on ChinaMoney.
pub async fn bond_info_cm(
    bond_name: &str,
    bond_code: &str,
    bond_issue: &str,
    bond_type: &str,
    coupon_type: &str,
    issue_year: &str,
    underwriter: &str,
    grade: &str,
) -> Result<Vec<serde_json::Value>> {
    let mut all_results = Vec::new();
    for page in 1..=10 {
        let resp = reqwest::Client::new()
            .post("https://www.chinamoney.com.cn/ags/ms/cm-u-bond-md/BondMarketInfoList2")
            .form(&[
                ("pageNo", page.to_string()),
                ("pageSize", "15".to_string()),
                ("bondName", bond_name.to_string()),
                ("bondCode", bond_code.to_string()),
                ("issueEnty", bond_issue.to_string()),
                ("bondType", bond_type.to_string()),
                ("bondSpclPrjctVrty", "".to_string()),
                ("couponType", coupon_type.to_string()),
                ("issueYear", issue_year.to_string()),
                ("entyDefinedCode", underwriter.to_string()),
                ("rtngShrt", grade.to_string()),
            ])
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let records = resp
            .get("data")
            .and_then(|d| d.get("resultList"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        if records.is_empty() {
            break;
        }
        all_results.extend(records);
    }
    Ok(all_results)
}

/// 中国外汇交易中心-债券详情 (Python: bond_info_detail_cm)
///
/// Get detailed info for a specific bond by name.
pub async fn bond_info_detail_cm(symbol: &str) -> Result<Vec<serde_json::Value>> {
    let resp = reqwest::Client::new()
        .post("https://www.chinamoney.com.cn/ags/ms/cm-u-bond-md/BondDetailInfo")
        .form(&[("bondDefinedCode", symbol)])
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .header("Referer", "https://www.chinamoney.com.cn/chinese/zqjc/")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    let data = resp
        .get("data")
        .and_then(|d| d.get("bondBaseInfo"))
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    if let Some(obj) = data.as_object() {
        let mut results = Vec::new();
        for (key, value) in obj {
            if key == "creditRateEntyList" || key == "exerciseInfoList" {
                continue;
            }
            results.push(serde_json::json!({"name": key, "value": value}));
        }
        Ok(results)
    } else {
        Ok(vec![])
    }
}
