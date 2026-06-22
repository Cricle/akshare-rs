//! Interbank lending rate data from Eastmoney.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::MacroDataPoint;
use crate::types::value_ext::ValueExt;
use crate::types::wire::EmDatacenterResp;

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Interbank lending rate from Eastmoney.
    ///
    /// Returns historical interbank lending rate data for the specified
    /// market, currency, and tenor.
    ///
    /// `market`: "上海银行同业拆借市场", "中国银行同业拆借市场", "伦敦银行同业拆借市场",
    ///           "欧洲银行同业拆借市场", "香港银行同业拆借市场", "新加坡银行同业拆借市场"
    /// `symbol`: "Shibor人民币", "Chibor人民币", "Libor英镑", "Libor欧元", etc.
    /// `indicator`: "隔夜", "1周", "2周", "3周", "1月", "2月", "3月", "6月", "1年"
    pub async fn rate_interbank(
        &self,
        market: &str,
        symbol: &str,
        indicator: &str,
    ) -> Result<Vec<MacroDataPoint>> {
        let market_map = [
            ("上海银行同业拆借市场", "001"),
            ("中国银行同业拆借市场", "002"),
            ("伦敦银行同业拆借市场", "003"),
            ("欧洲银行同业拆借市场", "004"),
            ("香港银行同业拆借市场", "005"),
            ("新加坡银行同业拆借市场", "006"),
        ];
        let currency_map = [
            ("Shibor人民币", "CNY"),
            ("Chibor人民币", "CNY"),
            ("Libor英镑", "GBP"),
            ("Libor欧元", "EUR"),
            ("Libor美元", "USD"),
            ("Libor日元", "JPY"),
            ("Euribor欧元", "EUR"),
            ("Hibor美元", "USD"),
            ("Hibor人民币", "CNH"),
            ("Hibor港币", "HKD"),
            ("Sibor星元", "SGD"),
            ("Sibor美元", "USD"),
        ];
        let indicator_map = [
            ("隔夜", "001"),
            ("1周", "101"),
            ("2周", "102"),
            ("3周", "103"),
            ("1月", "201"),
            ("2月", "202"),
            ("3月", "203"),
            ("4月", "204"),
            ("5月", "205"),
            ("6月", "206"),
            ("7月", "207"),
            ("8月", "208"),
            ("9月", "209"),
            ("10月", "210"),
            ("11月", "211"),
            ("1年", "301"),
        ];

        let market_code = market_map
            .iter()
            .find(|(k, _)| *k == market)
            .map(|(_, v)| *v)
            .ok_or_else(|| Error::invalid_input(format!("unknown market: {market}")))?;

        let currency_code = currency_map
            .iter()
            .find(|(k, _)| *k == symbol)
            .map(|(_, v)| *v)
            .ok_or_else(|| Error::invalid_input(format!("unknown symbol: {symbol}")))?;

        let indicator_code = indicator_map
            .iter()
            .find(|(k, _)| *k == indicator)
            .map(|(_, v)| *v)
            .ok_or_else(|| Error::invalid_input(format!("unknown indicator: {indicator}")))?;

        let filter = format!(
            r#"(MARKET_CODE="{market_code}")(CURRENCY_CODE="{currency_code}")(INDICATOR_ID="{indicator_code}")"#
        );

        let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
        let resp: EmDatacenterResp = self
            .get(url)
            .query(&[
                ("reportName", "RPT_IMP_INTRESTRATEN"),
                ("columns", "REPORT_DATE,REPORT_PERIOD,IR_RATE,CHANGE_RATE"),
                ("filter", &filter),
                ("pageNumber", "1"),
                ("pageSize", "500"),
                ("sortTypes", "-1"),
                ("sortColumns", "REPORT_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = resp.result.map(|r| r.data).unwrap_or_default();
        let mut items = Vec::new();

        for v in &data {
            let date = v.str_or(&["REPORT_DATE"], "");
            if date.is_empty() {
                continue;
            }
            let rate = v.f64_or(&["IR_RATE"], 0.0);
            items.push(MacroDataPoint {
                date: date.get(..10).unwrap_or(&date).to_string(),
                value: rate,
                name: format!("{market} {symbol} {indicator}"),
            });
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_em_datacenter_resp_structure() {
        let json = r#"{
            "result": {
                "data": [
                    {
                        "REPORT_DATE": "2024-01-02 00:00:00",
                        "IR_RATE": 1.5,
                        "CHANGE_RATE": -0.01
                    }
                ],
                "pages": 1
            }
        }"#;
        let resp: EmDatacenterResp = serde_json::from_str(json).unwrap();
        let data = resp.result.unwrap().data;
        assert_eq!(data.len(), 1);
        assert_eq!(
            data[0].get("IR_RATE").and_then(serde_json::Value::as_f64),
            Some(1.5)
        );
    }
}
