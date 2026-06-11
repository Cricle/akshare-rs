//! Commodity options from DCE, CZCE, SHFE, and GFEX.

use crate::client::AkShareClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// A single row of DCE option daily data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DceOptionRow {
    pub variety: String,
    pub contract: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub prev_settlement: f64,
    pub settlement: f64,
    pub change: f64,
    pub change1: f64,
    pub delta: f64,
    pub implied_volatility: f64,
    pub volume: f64,
    pub open_interest: f64,
    pub open_interest_change: f64,
    pub turnover: f64,
    pub exercise_volume: f64,
}

/// A single row of CZCE option daily data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CzceOptionRow {
    pub contract: String,
    pub prev_settlement: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub settlement: f64,
    pub change1: f64,
    pub change2: f64,
    pub volume: f64,
    pub open_interest: f64,
    pub open_interest_change: f64,
    pub turnover: f64,
    pub delta: f64,
    pub implied_volatility: f64,
    pub exercise_volume: f64,
}

/// A single row of SHFE option daily data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShfeOptionRow {
    pub contract: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub prev_settlement: f64,
    pub settlement: f64,
    pub change1: f64,
    pub change2: f64,
    pub volume: f64,
    pub open_interest: f64,
    pub open_interest_change: f64,
    pub turnover: f64,
    pub delta: f64,
    pub exercise_volume: f64,
}

/// A single row of SHFE option volatility data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShfeVolRow {
    pub series: String,
    pub volume: f64,
    pub open_interest: f64,
    pub open_interest_change: f64,
    pub turnover: f64,
    pub exercise_volume: f64,
    pub implied_volatility: f64,
}

/// A single row of GFEX option daily data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GfexOptionRow {
    pub variety: String,
    pub contract: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub prev_settlement: f64,
    pub settlement: f64,
    pub change: f64,
    pub change1: f64,
    pub delta: f64,
    pub volume: f64,
    pub open_interest: f64,
    pub open_interest_change: f64,
    pub turnover: f64,
    pub exercise_volume: f64,
    pub implied_volatility: f64,
}

/// A single row of GFEX option volatility data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GfexVolRow {
    pub series: String,
    pub implied_volatility: f64,
}

// ---------------------------------------------------------------------------
// DCE code map
// ---------------------------------------------------------------------------

fn dce_symbol_to_code(symbol: &str) -> Result<&'static str> {
    match symbol {
        "\u{7389}\u{7c73}\u{9009}\u{6743}" => Ok("c"),
        "\u{8c46}\u{7c95}\u{9009}\u{6743}" => Ok("m"),
        "\u{94c1}\u{77ff}\u{77f3}\u{9009}\u{6743}" => Ok("i"),
        "\u{6db2}\u{5316}\u{77f3}\u{6cb9}\u{6c14}\u{9009}\u{6743}" => Ok("pg"),
        "\u{805a}\u{4e59}\u{70ef}\u{9009}\u{6743}" => Ok("l"),
        "\u{805a}\u{6c2f}\u{4e59}\u{70ef}\u{9009}\u{6743}" => Ok("v"),
        "\u{805a}\u{4e19}\u{70ef}\u{9009}\u{6743}" => Ok("pp"),
        "\u{68d5}\u{6988}\u{6cb9}\u{9009}\u{6743}" => Ok("p"),
        "\u{9ec4}\u{5927}\u{8c46}1\u{53f7}\u{9009}\u{6743}" => Ok("a"),
        "\u{9ec4}\u{5927}\u{8c46}2\u{53f7}\u{9009}\u{6743}" => Ok("b"),
        "\u{8c46}\u{6cb9}\u{9009}\u{6743}" => Ok("y"),
        "\u{4e59}\u{4e8c}\u{9187}\u{9009}\u{6743}" => Ok("eg"),
        "\u{82ef}\u{4e59}\u{70ef}\u{9009}\u{6743}" => Ok("eb"),
        "\u{9e21}\u{86cb}\u{9009}\u{6743}" => Ok("jd"),
        "\u{7389}\u{7c73}\u{6dc0}\u{7c89}\u{9009}\u{6743}" => Ok("cs"),
        "\u{751f}\u{732a}\u{9009}\u{6743}" => Ok("lh"),
        "\u{539f}\u{6728}\u{9009}\u{6743}" => Ok("lg"),
        other => Err(Error::invalid_input(format!(
            "unsupported DCE option symbol: {other}"
        ))),
    }
}

fn czce_symbol_to_code(symbol: &str) -> Result<&'static str> {
    match symbol {
        "\u{767d}\u{7cd6}\u{9009}\u{6743}" => Ok("SR"),
        "\u{68c9}\u{82b1}\u{9009}\u{6743}" => Ok("CF"),
        "\u{7532}\u{9187}\u{9009}\u{6743}" => Ok("MA"),
        "PTA\u{9009}\u{6743}" => Ok("TA"),
        "\u{52a8}\u{529b}\u{7164}\u{9009}\u{6743}" => Ok("ZC"),
        "\u{83dc}\u{7c7d}\u{7c95}\u{9009}\u{6743}" => Ok("RM"),
        "\u{83dc}\u{7c7d}\u{6cb9}\u{9009}\u{6743}" => Ok("OI"),
        "\u{82b1}\u{751f}\u{9009}\u{6743}" => Ok("PK"),
        "\u{5bf9}\u{4e8c}\u{7532}\u{82ef}\u{9009}\u{6743}" => Ok("PX"),
        "\u{70e7}\u{78b1}\u{9009}\u{6743}" => Ok("SH"),
        "\u{7eaf}\u{78b1}\u{9009}\u{6743}" => Ok("SA"),
        "\u{77ed}\u{7ea4}\u{9009}\u{6743}" => Ok("PF"),
        "\u{9530}\u{7845}\u{9009}\u{6743}" => Ok("SM"),
        "\u{7845}\u{94c1}\u{9009}\u{6743}" => Ok("SF"),
        "\u{5c3f}\u{7d20}\u{9009}\u{6743}" => Ok("UR"),
        "\u{82f9}\u{679c}\u{9009}\u{6743}" => Ok("AP"),
        "\u{7ea2}\u{67a3}\u{9009}\u{6743}" => Ok("CJ"),
        "\u{73bb}\u{7483}\u{9009}\u{6743}" => Ok("FG"),
        "\u{74f6}\u{7247}\u{9009}\u{6743}" => Ok("PR"),
        "\u{4e19}\u{70ef}\u{9009}\u{6743}" => Ok("PL"),
        other => Err(Error::invalid_input(format!(
            "unsupported CZCE option symbol: {other}"
        ))),
    }
}

fn czce_code_to_filter(code: &str) -> &str {
    code
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// DCE option daily data.
    ///
    /// `symbol` is the Chinese name, e.g. "\u{805a}\u{4e19}\u{70ef}\u{9009}\u{6743}".
    /// `trade_date` is "YYYYMMDD".
    pub async fn option_hist_dce(
        &self,
        symbol: &str,
        trade_date: &str,
    ) -> Result<Vec<DceOptionRow>> {
        let variety_id = dce_symbol_to_code(symbol)?;
        let url = "http://www.dce.com.cn/dcereport/publicweb/dailystat/dayQuotes";

        let resp: serde_json::Value = self
                        .post(url)
            .json(&serde_json::json!({
                "contractId": "",
                "lang": "zh",
                "optionSeries": "",
                "statisticsType": 0,
                "tradeDate": trade_date,
                "tradeType": "2",
                "varietyId": variety_id,
            }))
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let data = resp
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut rows = Vec::with_capacity(data.len());
        for item in &data {
            rows.push(DceOptionRow {
                variety: json_str(item, "variety"),
                contract: json_str(item, "contractId"),
                open: json_f64(item, "open"),
                high: json_f64(item, "high"),
                low: json_f64(item, "low"),
                close: json_f64(item, "close"),
                prev_settlement: json_f64(item, "lastClear"),
                settlement: json_f64(item, "clearPrice"),
                change: json_f64(item, "diff"),
                change1: json_f64(item, "diff1"),
                delta: json_f64(item, "delta"),
                implied_volatility: json_f64(item, "impliedVolatility"),
                volume: json_f64(item, "volumn"),
                open_interest: json_f64(item, "openInterest"),
                open_interest_change: json_f64(item, "diffI"),
                turnover: json_f64(item, "turnover"),
                exercise_volume: json_f64(item, "matchQtySum"),
            });
        }

        Ok(rows)
    }

    /// CZCE option daily data.
    ///
    /// `symbol` is the Chinese name, e.g. "\u{767d}\u{7cd6}\u{9009}\u{6743}".
    /// `trade_date` is "YYYYMMDD".
    pub async fn option_hist_czce(
        &self,
        symbol: &str,
        trade_date: &str,
    ) -> Result<Vec<CzceOptionRow>> {
        let code = czce_symbol_to_code(symbol)?;
        let year = &trade_date[..4];
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Option/{year}/{trade_date}/OptionDataDaily.txt"
        );

        let body = self
                        .get(&url)
            .send()
            .await
            .map_err(Error::from)?
            .text()
            .await
            .map_err(Error::from)?;

        let lines: Vec<&str> = body.lines().collect();
        if lines.len() < 2 {
            return Ok(vec![]);
        }

        let filter_code = czce_code_to_filter(code);
        let mut rows = Vec::new();

        for line in lines.iter().skip(1) {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("----") {
                continue;
            }
            let fields: Vec<&str> = trimmed.split('|').map(str::trim).collect();
            if fields.len() < 16 {
                continue;
            }
            let contract = fields[0];
            if !contract.contains(filter_code) {
                continue;
            }
            rows.push(CzceOptionRow {
                contract: contract.to_string(),
                prev_settlement: parse_f64_comma(fields[1]),
                open: parse_f64_comma(fields[2]),
                high: parse_f64_comma(fields[3]),
                low: parse_f64_comma(fields[4]),
                close: parse_f64_comma(fields[5]),
                settlement: parse_f64_comma(fields[6]),
                change1: parse_f64_comma(fields[7]),
                change2: parse_f64_comma(fields[8]),
                volume: parse_f64_comma(fields[9]),
                open_interest: parse_f64_comma(fields[10]),
                open_interest_change: parse_f64_comma(fields[11]),
                turnover: parse_f64_comma(fields[12]),
                delta: parse_f64_comma(fields[13]),
                implied_volatility: parse_f64_comma(fields[14]),
                exercise_volume: parse_f64_comma(fields[15]),
            });
        }

        Ok(rows)
    }

    /// SHFE option daily data.
    ///
    /// `symbol` is the Chinese name, e.g. "\u{94dd}\u{9009}\u{6743}".
    /// `trade_date` is "YYYYMMDD".
    pub async fn option_hist_shfe(
        &self,
        symbol: &str,
        trade_date: &str,
    ) -> Result<Vec<ShfeOptionRow>> {
        let url = format!(
            "https://www.shfe.com.cn/data/tradedata/option/dailydata/kx{trade_date}.dat"
        );

        let resp: serde_json::Value = self
                        .get(&url)
            .header("User-Agent", "Mozilla/4.0 (compatible; MSIE 5.5; Windows NT)")
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let instruments = resp
            .get("o_curinstrument")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut rows = Vec::new();
        for item in &instruments {
            let product = json_str(item, "PRODUCTNAME");
            if product.trim() != symbol {
                continue;
            }
            let id = json_str(item, "INSTRUMENTID");
            if id == "\u{5c0f}\u{8ba1}" || id == "\u{5408}\u{8ba1}" || id.is_empty() {
                continue;
            }
            rows.push(ShfeOptionRow {
                contract: id,
                open: json_f64(item, "OPENPRICE"),
                high: json_f64(item, "HIGHESTPRICE"),
                low: json_f64(item, "LOWESTPRICE"),
                close: json_f64(item, "CLOSEPRICE"),
                prev_settlement: json_f64(item, "PRESETTLEMENTPRICE"),
                settlement: json_f64(item, "SETTLEMENTPRICE"),
                change1: json_f64(item, "ZD1_CHG"),
                change2: json_f64(item, "ZD2_CHG"),
                volume: json_f64(item, "VOLUME"),
                open_interest: json_f64(item, "OPENINTEREST"),
                open_interest_change: json_f64(item, "OPENINTERESTCHG"),
                turnover: json_f64(item, "TURNOVER"),
                delta: json_f64(item, "DELTA"),
                exercise_volume: json_f64(item, "EXECVOLUME"),
            });
        }

        Ok(rows)
    }

    /// SHFE option implied volatility data.
    ///
    /// `symbol` is the Chinese name, e.g. "\u{94dd}\u{9009}\u{6743}".
    /// `trade_date` is "YYYYMMDD".
    pub async fn option_vol_shfe(
        &self,
        symbol: &str,
        trade_date: &str,
    ) -> Result<Vec<ShfeVolRow>> {
        let url = format!(
            "https://www.shfe.com.cn/data/tradedata/option/dailydata/kx{trade_date}.dat"
        );

        let resp: serde_json::Value = self
                        .get(&url)
            .header("User-Agent", "Mozilla/4.0 (compatible; MSIE 5.5; Windows NT)")
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let sigma_data = resp
            .get("o_cursigma")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut rows = Vec::new();
        for item in &sigma_data {
            let product = json_str(item, "PRODUCTNAME");
            if product.trim() != symbol {
                continue;
            }
            rows.push(ShfeVolRow {
                series: json_str(item, "INSTRUMENTID"),
                volume: json_f64(item, "VOLUME"),
                open_interest: json_f64(item, "OPENINTEREST"),
                open_interest_change: json_f64(item, "OPENINTERESTCHG"),
                turnover: json_f64(item, "TURNOVER"),
                exercise_volume: json_f64(item, "EXECVOLUME"),
                implied_volatility: json_f64(item, "SIGMA"),
            });
        }

        Ok(rows)
    }

    /// GFEX option daily data.
    ///
    /// `symbol` is the Chinese name, e.g. "\u{5de5}\u{4e1a}\u{7845}".
    /// `trade_date` is "YYYYMMDD".
    pub async fn option_hist_gfex(
        &self,
        symbol: &str,
        trade_date: &str,
    ) -> Result<Vec<GfexOptionRow>> {
        let url = "http://www.gfex.com.cn/u/interfacesWebTiDayQuotes/loadList";
        let body = format!("trade_date={trade_date}&trade_type=1");

        let resp: serde_json::Value = self
                        .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Referer", "http://www.gfex.com.cn/gfex/rihq/hqsj_tjsj.shtml")
            .body(body)
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let data = resp
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut rows = Vec::new();
        for item in &data {
            let variety = json_str(item, "variety");
            if !variety.contains(symbol) {
                continue;
            }
            rows.push(GfexOptionRow {
                variety,
                contract: json_str(item, "delivMonth"),
                open: json_f64(item, "open"),
                high: json_f64(item, "high"),
                low: json_f64(item, "low"),
                close: json_f64(item, "close"),
                prev_settlement: json_f64(item, "lastClear"),
                settlement: json_f64(item, "clearPrice"),
                change: json_f64(item, "diff"),
                change1: json_f64(item, "diff1"),
                delta: json_f64(item, "delta"),
                volume: json_f64(item, "volumn"),
                open_interest: json_f64(item, "openInterest"),
                open_interest_change: json_f64(item, "diffI"),
                turnover: json_f64(item, "turnover"),
                exercise_volume: json_f64(item, "matchQtySum"),
                implied_volatility: json_f64(item, "impliedVolatility"),
            });
        }

        Ok(rows)
    }

    /// GFEX option implied volatility data.
    ///
    /// `symbol` is the Chinese name, e.g. "\u{78b3}\u{9178}\u{9502}".
    /// `trade_date` is "YYYYMMDD".
    pub async fn option_vol_gfex(
        &self,
        symbol: &str,
        trade_date: &str,
    ) -> Result<Vec<GfexVolRow>> {
        let symbol_code = match symbol {
            "\u{5de5}\u{4e1a}\u{7845}" => "si",
            "\u{78b3}\u{9178}\u{9502}" => "lc",
            "\u{591a}\u{6676}\u{7845}" => "ps",
            other => {
                return Err(Error::invalid_input(format!(
                    "unsupported GFEX vol symbol: {other}"
                )))
            }
        };

        let url = "http://www.gfex.com.cn/u/interfacesWebTiDayQuotes/loadListOptVolatility";
        let body = format!("trade_date={trade_date}");

        let resp: serde_json::Value = self
                        .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Referer", "http://www.gfex.com.cn/gfex/rihq/hqsj_tjsj.shtml")
            .body(body)
            .send()
            .await
            .map_err(Error::from)?
            .json()
            .await
            .map_err(Error::from)?;

        let data = resp
            .get("data")
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let mut rows = Vec::new();
        for item in &data {
            let series = json_str(item, "seriesId");
            if !series.to_lowercase().contains(symbol_code) {
                continue;
            }
            rows.push(GfexVolRow {
                series,
                implied_volatility: json_f64(item, "hisVolatility"),
            });
        }

        Ok(rows)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn json_str(v: &serde_json::Value, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string()
}

fn json_f64(v: &serde_json::Value, key: &str) -> f64 {
    match v.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(serde_json::Value::String(s)) => parse_f64_comma(s),
        _ => 0.0,
    }
}

fn parse_f64_comma(s: &str) -> f64 {
    s.trim().replace(',', "").parse::<f64>().unwrap_or(0.0)
}
