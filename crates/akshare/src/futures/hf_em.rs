//! Global futures data from Eastmoney (东方财富网-国际期货).

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{GlobalFuturesKline, Row};

fn parse_f64(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap_or(0.0)
}

impl AkShareClient {
    /// Global futures realtime quotes from Eastmoney.
    ///
    /// Fetches all international futures contracts (COMEX, NYMEX, LME, etc.)
    pub async fn futures_global_spot_em(&self) -> Result<Vec<Row>> {
        let url =
            "https://futsseapi.eastmoney.com/list/COMEX,NYMEX,COBOT,SGX,NYBOT,LME,MDEX,TOCOM,IPE";
        let mut all_items = Vec::new();
        let mut page = 0;

        loop {
            let body = self
                .get(url)
                .query(&[
                    ("orderBy", "dm"),
                    ("sort", "desc"),
                    ("pageSize", "20"),
                    ("pageIndex", &page.to_string()),
                    ("token", "58b2fa8f54638b60b87d69b31969089c"),
                    (
                        "field",
                        "dm,sc,name,p,zsjd,zde,zdf,f152,o,h,l,zjsj,vol,wp,np,ccl",
                    ),
                    ("blockName", "callback"),
                ])
                .send()
                .await?
                .text()
                .await?;

            let data: serde_json::Value = serde_json::from_str(&body)?;
            let total = data["total"].as_i64().unwrap_or(0);
            let list = data["list"].as_array().cloned().unwrap_or_default();

            for item in &list {
                let mut row = Row::new();
                row.insert("code".into(), item["dm"].clone());
                row.insert("name".into(), item["name"].clone());
                row.insert("latest_price".into(), item["p"].clone());
                row.insert("change_amount".into(), item["zde"].clone());
                row.insert("change_pct".into(), item["zdf"].clone());
                row.insert("open".into(), item["o"].clone());
                row.insert("high".into(), item["h"].clone());
                row.insert("low".into(), item["l"].clone());
                row.insert("prev_close".into(), item["zjsj"].clone());
                row.insert("volume".into(), item["vol"].clone());
                row.insert("open_interest".into(), item["ccl"].clone());
                all_items.push(row);
            }

            page += 1;
            if page * 20 >= total as u64 {
                break;
            }
            if page > 50 {
                break; // safety limit
            }
        }

        Ok(all_items)
    }

    /// Global futures historical kline data from Eastmoney.
    ///
    /// `symbol`: Eastmoney symbol code, e.g., "HG00Y", "CL00Y"
    pub async fn futures_global_hist_em(&self, symbol: &str) -> Result<Vec<GlobalFuturesKline>> {
        // Determine market code from symbol prefix
        let base: String = symbol
            .chars()
            .take_while(|c| c.is_ascii_alphabetic())
            .collect();
        let market_code = match base.as_str() {
            "HG" | "GC" | "SI" | "QI" | "QO" | "MGC" | "LTH" => 101,
            "CL" | "NG" | "RB" | "HO" | "PA" | "PL" | "QM" => 102,
            "ZW" | "ZM" | "ZS" | "ZC" | "XC" | "XK" | "XW" | "YM" | "TY" | "US" | "ES" | "NQ" => {
                103
            }
            "SB" | "CT" | "SF" => 108,
            _ => {
                return Err(Error::invalid_input(format!(
                    "unknown symbol prefix: {}",
                    base
                )));
            }
        };

        let url = "https://push2his.eastmoney.com/api/qt/stock/kline/get";
        let secid = format!("{}.{}", market_code, symbol);
        let response = self
            .get(url)
            .query(&[
                ("secid", secid.as_str()),
                ("klt", "101"),
                ("fqt", "1"),
                ("lmt", "6600"),
                ("end", "20500000"),
                ("iscca", "1"),
                ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8"),
                (
                    "fields2",
                    "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64",
                ),
                ("ut", "f057cbcbce2a86e2866ab8877db1d059"),
                ("forcect", "1"),
            ])
            .send()
            .await
            .map_err(Error::from)?;

        let body = response.text().await.map_err(Error::from)?;
        let data: serde_json::Value = serde_json::from_str(&body)?;

        let klines = data["data"]["klines"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        let code = data["data"]["code"].as_str().unwrap_or(symbol);
        let name = data["data"]["name"].as_str().unwrap_or("");

        let mut items = Vec::new();
        for kline in &klines {
            let line = kline.as_str().unwrap_or("");
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 14 {
                continue;
            }
            let open_interest_chg = parse_f64(fields[13]);
            // Fix unsigned -> signed conversion
            let unsigned_max: f64 = 4294967295.0; // 2^32 - 1
            let signed_max: f64 = 2147483647.0; // 2^31 - 1
            let oi_chg = if open_interest_chg > signed_max {
                open_interest_chg - (unsigned_max + 1.0)
            } else {
                open_interest_chg
            };

            items.push(GlobalFuturesKline {
                date: fields[0].to_string(),
                code: code.to_string(),
                name: name.to_string(),
                open: parse_f64(fields[1]),
                latest_price: parse_f64(fields[2]),
                high: parse_f64(fields[3]),
                low: parse_f64(fields[4]),
                volume: parse_f64(fields[5]),
                change_pct: parse_f64(fields[8]),
                open_interest: parse_f64(fields[12]),
                open_interest_chg: oi_chg,
            });
        }
        Ok(items)
    }
}
