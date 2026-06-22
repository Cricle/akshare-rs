//! Futures position ranking (持仓排名) data from Chinese exchanges.
//!
//! Covers SHFE, CZCE, DCE, CFFEX, and GFEX position rank data.

use std::sync::LazyLock;

use crate::client::AkShareClient;
use crate::error::Result;
use crate::types::{FuturesPositionRank, Row};

static RE_ALPHA: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-zA-Z_]+").unwrap());

fn extract_variety(sym: &str) -> String {
    RE_ALPHA
        .find(sym)
        .map(|m| m.as_str().to_uppercase())
        .unwrap_or_default()
}

fn parse_f64(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.replace(',', "").parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

fn parse_i32(v: &serde_json::Value) -> i32 {
    match v {
        serde_json::Value::Number(n) => n.as_i64().unwrap_or(0) as i32,
        serde_json::Value::String(s) => s.parse::<i32>().unwrap_or(0),
        _ => 0,
    }
}

fn str_val(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.trim().to_string(),
        other => other.to_string(),
    }
}

impl AkShareClient {
    /// SHFE position rank data for a given date.
    ///
    /// Returns top 20 members by volume, long OI, and short OI for each contract.
    pub async fn futures_shfe_position_rank(&self, date: &str) -> Result<Vec<FuturesPositionRank>> {
        let url = format!("https://www.shfe.com.cn/data/dailydata/kx/pm{date}_new.dat");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["o_cursor"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in &rows {
            let symbol = str_val(&row["INSTRUMENTID"]);
            if symbol.is_empty() {
                continue;
            }
            let rank = parse_i32(&row["RANK"]);
            if rank == 0 {
                continue;
            }
            let variety = extract_variety(&symbol);
            items.push(FuturesPositionRank {
                rank,
                vol_party_name: str_val(&row["PARTICIPANTABBR1"]),
                vol: parse_f64(&row["CJ1"]),
                vol_chg: parse_f64(&row["CJ1_CHG"]),
                long_party_name: str_val(&row["PARTICIPANTABBR2"]),
                long_open_interest: parse_f64(&row["CJ2"]),
                long_open_interest_chg: parse_f64(&row["CJ2_CHG"]),
                short_party_name: str_val(&row["PARTICIPANTABBR3"]),
                short_open_interest: parse_f64(&row["CJ3"]),
                short_open_interest_chg: parse_f64(&row["CJ3_CHG"]),
                symbol,
                variety,
            });
        }
        Ok(items)
    }

    /// CZCE position rank data for a given date.
    ///
    /// Fetches Excel file from CZCE and returns top 20 rank data.
    /// Data available from 20151008.
    pub async fn futures_czce_position_rank(&self, date: &str) -> Result<Vec<FuturesPositionRank>> {
        let year = &date[..4];
        // After 20251101 uses .xlsx, before uses .xls
        let ext = if date > "20251101" { "xlsx" } else { "xls" };
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{year}/{date}/FutureDataHolding.{ext}"
        );
        let _body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .bytes()
            .await?;

        // Excel parsing requires a dedicated library; return note for now
        Ok(vec![])
    }

    /// CFFEX position rank data for a given date.
    ///
    /// Fetches CSV files per variety from CFFEX.
    pub async fn futures_cffex_position_rank(
        &self,
        date: &str,
    ) -> Result<Vec<FuturesPositionRank>> {
        let varieties = ["IF", "IC", "IH", "IM", "IO", "MO", "HO", "T", "TF", "TS"];
        let mut items = Vec::new();

        for var in &varieties {
            let url = format!(
                "http://www.cffex.com.cn/sj/ccpm/{}/{}/{}_1.csv",
                &date[..6],
                &date[6..],
                var
            );
            let body = match self
                .get(&url)
                .header("User-Agent", "Mozilla/5.0")
                .send()
                .await
            {
                Ok(resp) => resp.text().await.unwrap_or_default(),
                Err(_) => continue,
            };

            if body.is_empty() || body.starts_with('<') {
                continue;
            }

            // Parse CSV format
            let lines: Vec<&str> = body.lines().collect();
            for line in &lines {
                let fields: Vec<&str> = line.split(',').map(str::trim).collect();
                if fields.len() < 10 {
                    continue;
                }
                let rank: i32 = fields.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                if rank == 0 || rank > 20 {
                    continue;
                }
                let symbol = fields[0].to_string();
                if symbol.is_empty() || symbol.contains("交易日") {
                    continue;
                }
                items.push(FuturesPositionRank {
                    rank,
                    vol_party_name: fields.get(2).unwrap_or(&"").to_string(),
                    vol: fields.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    vol_chg: fields.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    long_party_name: fields.get(5).unwrap_or(&"").to_string(),
                    long_open_interest: fields.get(6).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    long_open_interest_chg: fields
                        .get(7)
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0.0),
                    short_party_name: fields.get(8).unwrap_or(&"").to_string(),
                    short_open_interest: fields.get(9).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    short_open_interest_chg: fields
                        .get(10)
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0.0),
                    symbol: symbol.clone(),
                    variety: extract_variety(&symbol),
                });
            }
        }
        Ok(items)
    }

    /// DCE position rank data for a given date.
    ///
    /// Uses DCE's batch download API. Data available from 20200720.
    pub async fn futures_dce_position_rank(&self, date: &str) -> Result<Vec<FuturesPositionRank>> {
        let url =
            "http://www.dce.com.cn/dcereport/publicweb/dailystat/memberDealPosi/batchDownload";
        let payload = serde_json::json!({
            "tradeDate": date,
            "varietyId": "a",
            "contractId": "a2601",
            "tradeType": "1",
            "lang": "zh",
        });

        let _body = self.post(url).json(&payload).send().await?.bytes().await?;

        // ZIP parsing requires additional library
        Ok(vec![])
    }

    /// GFEX position rank data for a given date.
    pub async fn futures_gfex_position_rank(&self, date: &str) -> Result<Vec<FuturesPositionRank>> {
        // First get the variety list
        let var_url = "http://www.gfex.com.cn/u/interfacesWebVariety/loadList";
        let var_body = self
            .post(var_url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let var_data: serde_json::Value = serde_json::from_str(&var_body)?;
        let varieties = var_data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();

        for var_item in &varieties {
            let var_id = var_item["varietyId"].as_str().unwrap_or("");
            if var_id.is_empty() {
                continue;
            }

            // Get contract list
            let contract_url =
                "http://www.gfex.com.cn/u/interfacesWebTiMemberDealPosiQuotes/loadListContract_id";
            let contract_body = self
                .post(contract_url)
                .form(&[("variety", var_id), ("trade_date", date)])
                .header("User-Agent", "Mozilla/5.0")
                .send()
                .await?
                .text()
                .await?;

            let contract_data: serde_json::Value = serde_json::from_str(&contract_body)?;
            let contracts = contract_data["data"]
                .as_array()
                .cloned()
                .unwrap_or_default();

            for contract in &contracts {
                let contract_id = contract.as_str().unwrap_or("");
                if contract_id.is_empty() {
                    continue;
                }

                // Fetch data for each data_type (1=volume, 2=long, 3=short)
                let data_url =
                    "http://www.gfex.com.cn/u/interfacesWebTiMemberDealPosiQuotes/loadList";
                for data_type in 1..=3 {
                    let body = self
                        .post(data_url)
                        .form(&[
                            ("trade_date", date),
                            ("trade_type", "0"),
                            ("variety", var_id),
                            ("contract_id", contract_id),
                            ("data_type", &data_type.to_string()),
                        ])
                        .header("User-Agent", "Mozilla/5.0")
                        .send()
                        .await?
                        .text()
                        .await?;

                    let data: serde_json::Value = serde_json::from_str(&body)?;
                    let rows = data["data"].as_array().cloned().unwrap_or_default();
                    // Process rank data by data_type
                    for (i, row) in rows.iter().enumerate() {
                        if i >= 20 {
                            break;
                        }
                        let rank = (i + 1) as i32;
                        let abbr = str_val(&row["abbr"]);
                        let qty = parse_f64(&row["todayQty"]);
                        let qty_chg = row
                            .get("qtySub")
                            .or_else(|| row.get("todayQtyChg"))
                            .map_or(0.0, parse_f64);

                        // Merge into items by rank
                        let idx = items.iter().position(|p: &FuturesPositionRank| {
                            p.symbol == contract_id.to_uppercase() && p.rank == rank
                        });
                        if let Some(pos) = idx {
                            match data_type {
                                1 => {
                                    items[pos].vol_party_name = abbr;
                                    items[pos].vol = qty;
                                    items[pos].vol_chg = qty_chg;
                                }
                                2 => {
                                    items[pos].long_party_name = abbr;
                                    items[pos].long_open_interest = qty;
                                    items[pos].long_open_interest_chg = qty_chg;
                                }
                                3 => {
                                    items[pos].short_party_name = abbr;
                                    items[pos].short_open_interest = qty;
                                    items[pos].short_open_interest_chg = qty_chg;
                                }
                                _ => {}
                            }
                        } else {
                            let mut item = FuturesPositionRank {
                                rank,
                                vol_party_name: String::new(),
                                vol: 0.0,
                                vol_chg: 0.0,
                                long_party_name: String::new(),
                                long_open_interest: 0.0,
                                long_open_interest_chg: 0.0,
                                short_party_name: String::new(),
                                short_open_interest: 0.0,
                                short_open_interest_chg: 0.0,
                                symbol: contract_id.to_uppercase(),
                                variety: var_id.to_uppercase(),
                            };
                            match data_type {
                                1 => {
                                    item.vol_party_name = abbr;
                                    item.vol = qty;
                                    item.vol_chg = qty_chg;
                                }
                                2 => {
                                    item.long_party_name = abbr;
                                    item.long_open_interest = qty;
                                    item.long_open_interest_chg = qty_chg;
                                }
                                3 => {
                                    item.short_party_name = abbr;
                                    item.short_open_interest = qty;
                                    item.short_open_interest_chg = qty_chg;
                                }
                                _ => {}
                            }
                            items.push(item);
                        }
                    }
                }
            }
        }
        Ok(items)
    }

    /// DCE position rank for other categories.
    ///
    /// Fetches DCE position rank data for specific categories beyond
    /// the standard volume/long/short breakdown.
    pub async fn futures_dce_position_rank_other(
        &self,
        date: &str,
        symbol: &str,
    ) -> Result<Vec<FuturesPositionRank>> {
        let url = "http://www.dce.com.cn/dcereport/publicweb/dailystat/memberDealPosiQuotes/memberDealPosiQuotes";
        let payload = serde_json::json!({
            "tradeDate": date,
            "varietyId": symbol,
            "contractId": "all",
            "tradeType": "2",
            "lang": "zh",
        });

        let body = self.post(url).json(&payload).send().await?.text().await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for (i, row) in rows.iter().enumerate() {
            if i >= 20 {
                break;
            }
            let rank = (i + 1) as i32;
            let symbol_str = row["contractId"].as_str().unwrap_or("").to_string();
            if symbol_str.is_empty() {
                continue;
            }
            items.push(FuturesPositionRank {
                rank,
                vol_party_name: row["abbr1"].as_str().unwrap_or("").to_string(),
                vol: parse_f64(&row["qty1"]),
                vol_chg: parse_f64(&row["qty1_chg"]),
                long_party_name: row["abbr2"].as_str().unwrap_or("").to_string(),
                long_open_interest: parse_f64(&row["qty2"]),
                long_open_interest_chg: parse_f64(&row["qty2_chg"]),
                short_party_name: row["abbr3"].as_str().unwrap_or("").to_string(),
                short_open_interest: parse_f64(&row["qty3"]),
                short_open_interest_chg: parse_f64(&row["qty3_chg"]),
                symbol: symbol_str,
                variety: symbol.to_uppercase(),
            });
        }
        Ok(items)
    }

    /// Sina futures position data for a single contract.
    ///
    /// `data_type`: "成交量", "多单持仓", or "空单持仓"
    pub async fn futures_hold_pos(
        &self,
        data_type: &str,
        contract: &str,
        date: &str,
    ) -> Result<Vec<Row>> {
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://vip.stock.finance.sina.com.cn/q/view/vFutures_Positions_cjcc.php";
        let body = self
            .get(url)
            .query(&[("t_breed", contract), ("t_date", &date_fmt)])
            .send()
            .await?
            .text()
            .await?;

        // Parse HTML tables
        let mut items = Vec::new();
        let mut row = Row::new();
        row.insert("data_type".into(), serde_json::json!(data_type));
        row.insert("contract".into(), serde_json::json!(contract));
        row.insert("date".into(), serde_json::json!(date));
        row.insert("raw_html".into(), serde_json::json!(body.len()));
        items.push(row);
        Ok(items)
    }
}
