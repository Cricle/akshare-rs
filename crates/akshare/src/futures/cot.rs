//! Futures position ranking (持仓排名) data from Chinese exchanges.
//!
//! Covers SHFE, CZCE, DCE, CFFEX, and GFEX position rank data.

use std::io::{Cursor, Read};
use std::sync::LazyLock;

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::{FuturesPositionRank, Row};
use crate::util::{decode_text, parse_f64_safe, parse_html_tables, read_spreadsheet_rows};

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

/// Read every non-directory entry of a ZIP archive.
fn read_zip_entries(bytes: &[u8]) -> Result<Vec<(String, Vec<u8>)>> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|e| Error::decode(format!("DCE position rank ZIP: {e}")))?;
    let mut entries = Vec::with_capacity(archive.len());
    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|e| Error::decode(format!("DCE position rank ZIP entry: {e}")))?;
        if file.is_dir() {
            continue;
        }
        let name = file
            .name()
            .map_err(|e| Error::decode(format!("DCE position rank ZIP entry name: {e}")))?
            .to_string();
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|e| Error::decode(format!("DCE position rank ZIP read: {e}")))?;
        entries.push((name, data));
    }
    Ok(entries)
}

/// Extract the DCE contract code from an archive member such as
/// `20240315_a2601_成交量_买持仓_卖持仓排名.txt`.
fn dce_symbol_from_name(name: &str) -> Option<String> {
    let base = name.rsplit(['/', '\\']).next().unwrap_or(name);
    let contract = base.split('_').nth(1)?;
    if contract.is_empty() {
        None
    } else {
        Some(contract.to_uppercase())
    }
}

/// Parse one DCE position-rank text file.
///
/// DCE publishes a tab-separated file with three vertically stacked sections
/// (volume, long open interest, short open interest). Each section starts with
/// a `名次` header row and ends with a `总计`/`合计` row. Older files use
/// whitespace-aligned columns instead of tabs.
fn parse_dce_rank_text(text: &str, symbol: &str) -> Vec<FuturesPositionRank> {
    let tab_separated = text.contains('\t');
    let mut rows: Vec<Vec<String>> = text
        .lines()
        .map(|line| {
            let fields: Vec<&str> = if tab_separated {
                line.split('\t').collect()
            } else {
                line.split_whitespace().collect()
            };
            fields
                .into_iter()
                .map(|field| {
                    field
                        .trim()
                        .trim_start_matches('\u{feff}')
                        .trim_end_matches('\r')
                        .to_string()
                })
                .collect()
        })
        .collect();

    // A trailing "会员类别" summary block is not part of the rank table.
    if rows
        .iter()
        .any(|row| row.first().is_some_and(|cell| cell.starts_with("会员类别")))
    {
        let keep = rows.len().saturating_sub(6);
        rows.truncate(keep);
    }

    let header_rows: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| row.first().is_some_and(|cell| cell.starts_with("名次")))
        .map(|(index, _)| index)
        .collect();
    if header_rows.len() < 3 {
        return Vec::new();
    }

    let total_rows: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            row.first()
                .is_some_and(|cell| cell.contains("总计") || cell.contains("合计"))
        })
        .map(|(index, _)| index)
        .collect();

    // (member name, quantity, change) for volume, long, and short sections.
    let mut sections: [Vec<(String, f64, f64)>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for (part, &header) in header_rows.iter().take(3).enumerate() {
        let Some(&end) = total_rows.iter().find(|&&total| total > header) else {
            continue;
        };
        for row in &rows[header + 1..end] {
            if row.len() < 4 || row[0].is_empty() {
                continue;
            }
            sections[part].push((
                row[1].trim().to_string(),
                parse_dce_number(&row[2]),
                parse_dce_number(&row[3]),
            ));
        }
    }

    let max_len = sections.iter().map(Vec::len).max().unwrap_or(0).min(20);
    let symbol = symbol.to_uppercase();
    let variety = extract_variety(&symbol);
    let mut items = Vec::with_capacity(max_len);
    for index in 0..max_len {
        let volume = sections[0].get(index);
        let long = sections[1].get(index);
        let short = sections[2].get(index);
        if volume.is_none() && long.is_none() && short.is_none() {
            continue;
        }
        items.push(FuturesPositionRank {
            rank: (index + 1) as i32,
            vol_party_name: volume.map(|entry| entry.0.clone()).unwrap_or_default(),
            vol: volume.map_or(0.0, |entry| entry.1),
            vol_chg: volume.map_or(0.0, |entry| entry.2),
            long_party_name: long.map(|entry| entry.0.clone()).unwrap_or_default(),
            long_open_interest: long.map_or(0.0, |entry| entry.1),
            long_open_interest_chg: long.map_or(0.0, |entry| entry.2),
            short_party_name: short.map(|entry| entry.0.clone()).unwrap_or_default(),
            short_open_interest: short.map_or(0.0, |entry| entry.1),
            short_open_interest_chg: short.map_or(0.0, |entry| entry.2),
            symbol: symbol.clone(),
            variety: variety.clone(),
        });
    }
    items
}

/// Parse a DCE numeric cell, tolerating thousands separators and placeholders.
fn parse_dce_number(value: &str) -> f64 {
    let cleaned = value.trim().replace(',', "");
    if cleaned.is_empty() || cleaned == "-" {
        0.0
    } else {
        cleaned.parse().unwrap_or(0.0)
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
        let bytes = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        let rows = read_spreadsheet_rows(&bytes)?;
        let mut items = Vec::new();
        for row in rows.into_iter().skip(1) {
            if row.len() < 10 {
                continue;
            }
            let rank = row
                .first()
                .and_then(|value| value.parse::<i32>().ok())
                .unwrap_or(0);
            if rank <= 0 || rank > 20 {
                continue;
            }
            let symbol = row.get(1).cloned().unwrap_or_default();
            items.push(FuturesPositionRank {
                rank,
                vol_party_name: row.get(2).cloned().unwrap_or_default(),
                vol: row.get(3).and_then(|v| v.parse().ok()).unwrap_or(0.0),
                vol_chg: row.get(4).and_then(|v| v.parse().ok()).unwrap_or(0.0),
                long_party_name: row.get(5).cloned().unwrap_or_default(),
                long_open_interest: row.get(6).and_then(|v| v.parse().ok()).unwrap_or(0.0),
                long_open_interest_chg: row.get(7).and_then(|v| v.parse().ok()).unwrap_or(0.0),
                short_party_name: row.get(8).cloned().unwrap_or_default(),
                short_open_interest: row.get(9).and_then(|v| v.parse().ok()).unwrap_or(0.0),
                short_open_interest_chg: row.get(10).and_then(|v| v.parse().ok()).unwrap_or(0.0),
                variety: extract_variety(&symbol),
                symbol,
            });
        }
        if items.is_empty() {
            return Err(Error::not_found("CZCE returned no position rank data"));
        }
        Ok(items)
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
    ///
    /// The endpoint returns a ZIP archive with one tab-separated `.txt` file
    /// per contract. Each file stacks the volume, long open interest, and short
    /// open interest top-20 sections.
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

        let bytes = self
            .post(url)
            .header("User-Agent", "Mozilla/5.0")
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        if !bytes.starts_with(b"PK") {
            return Err(Error::decode(
                "DCE position rank response is not a ZIP archive",
            ));
        }

        let mut items = Vec::new();
        for (name, data) in read_zip_entries(&bytes)? {
            if !name.starts_with(date) {
                continue;
            }
            let Some(symbol) = dce_symbol_from_name(&name) else {
                continue;
            };
            let text = crate::util::decode_text(&data, None, Some(encoding_rs::GB18030));
            items.extend(parse_dce_rank_text(&text, &symbol));
        }
        Ok(items)
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
        if !matches!(data_type, "成交量" | "多单持仓" | "空单持仓") {
            return Err(Error::invalid_input(
                "data_type must be 成交量, 多单持仓, or 空单持仓",
            ));
        }
        if date.len() != 8 || !date.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Error::invalid_input("date must use YYYYMMDD"));
        }
        let date_fmt = format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]);
        let url = "https://vip.stock.finance.sina.com.cn/q/view/vFutures_Positions_cjcc.php";
        let response = self
            .get(url)
            .query(&[("t_breed", contract), ("t_date", &date_fmt)])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .error_for_status()?;
        let bytes = response.bytes().await?;
        let body = decode_text(&bytes, None, Some(encoding_rs::GB18030));
        let wanted = match data_type {
            "成交量" => ["名次", "会员简称", "成交量", "比上交易增减"],
            "多单持仓" => ["名次", "会员简称", "多单持仓", "比上交易增减"],
            _ => ["名次", "会员简称", "空单持仓", "比上交易增减"],
        };
        let tables = parse_html_tables(&body);
        let table = tables
            .iter()
            .find(|table| {
                table.iter().any(|row| {
                    wanted
                        .iter()
                        .all(|header| row.iter().any(|cell| cell.trim() == *header))
                })
            })
            .ok_or_else(|| Error::not_found("Sina futures position table not found"))?;
        let header_row = table
            .iter()
            .position(|row| {
                wanted
                    .iter()
                    .all(|header| row.iter().any(|cell| cell.trim() == *header))
            })
            .ok_or_else(|| Error::decode("Sina futures position header not found"))?;
        let headers = &table[header_row];
        let index_of = |name: &str| {
            headers
                .iter()
                .position(|cell| cell.trim() == name)
                .unwrap_or(0)
        };
        let rank_index = index_of("名次");
        let member_index = index_of("会员简称");
        let amount_index = index_of(wanted[2]);
        let change_index = index_of("比上交易增减");
        let mut items = Vec::new();
        for row in table.iter().skip(header_row + 1) {
            let rank = row
                .get(rank_index)
                .map(|v| parse_f64_safe(v))
                .unwrap_or(0.0);
            if rank <= 0.0 || row.get(member_index).is_none() {
                continue;
            }
            let member = row[member_index].trim();
            if member.is_empty() || member == "合计" || member == "总计" {
                continue;
            }
            let mut item = Row::new();
            item.insert("名次".into(), serde_json::json!(rank as i64));
            item.insert("会员简称".into(), serde_json::json!(member));
            item.insert(
                wanted[2].into(),
                serde_json::json!(row.get(amount_index).map_or(0.0, |v| parse_f64_safe(v))),
            );
            item.insert(
                "比上交易增减".into(),
                serde_json::json!(row.get(change_index).map_or(0.0, |v| parse_f64_safe(v))),
            );
            item.insert("合约".into(), serde_json::json!(contract));
            item.insert("日期".into(), serde_json::json!(date));
            items.push(item);
        }
        if items.is_empty() {
            return Err(Error::not_found("Sina futures position returned no rows"));
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    const DCE_FIXTURE: &str = "\
大连商品交易所 持仓排名
20240315
名次\t会员简称\t成交量\t增减
1\t永安期货\t1,000\t10
2\t中信期货\t900\t-5
总计\t-\t1,900\t5
名次\t会员简称\t持买单量\t增减
1\t永安期货\t800\t20
2\t中信期货\t700\t-10
总计\t-\t1,500\t10
名次\t会员简称\t持卖单量\t增减
1\t国泰君安\t600\t5
2\t海通期货\t500\t15
总计\t-\t1,100\t20
";

    fn dce_fixture_zip() -> Vec<u8> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        writer
            .start_file("20240315_a2601_成交量_买持仓_卖持仓排名.txt", options)
            .unwrap();
        writer.write_all(DCE_FIXTURE.as_bytes()).unwrap();
        writer.finish().unwrap().into_inner()
    }

    #[test]
    fn test_parse_dce_rank_text_sections() {
        let items = parse_dce_rank_text(DCE_FIXTURE, "A2601");
        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.rank, 1);
        assert_eq!(first.symbol, "A2601");
        assert_eq!(first.variety, "A");
        assert_eq!(first.vol_party_name, "永安期货");
        assert!((first.vol - 1000.0).abs() < 1e-9);
        assert!((first.vol_chg - 10.0).abs() < 1e-9);
        assert_eq!(first.long_party_name, "永安期货");
        assert!((first.long_open_interest - 800.0).abs() < 1e-9);
        assert_eq!(first.short_party_name, "国泰君安");
        assert!((first.short_open_interest - 600.0).abs() < 1e-9);
        assert!((first.short_open_interest_chg - 5.0).abs() < 1e-9);

        let second = &items[1];
        assert_eq!(second.rank, 2);
        assert!((second.vol - 900.0).abs() < 1e-9);
        assert!((second.long_open_interest - 700.0).abs() < 1e-9);
        assert!((second.short_open_interest - 500.0).abs() < 1e-9);
    }

    #[test]
    fn test_dce_symbol_from_archive_name() {
        assert_eq!(
            dce_symbol_from_name("20240315_a2601_成交量_买持仓_卖持仓排名.txt").as_deref(),
            Some("A2601")
        );
        assert_eq!(dce_symbol_from_name("notes.txt"), None);
    }

    #[test]
    fn test_read_dce_fixture_zip() {
        let zip = dce_fixture_zip();
        let entries = read_zip_entries(&zip).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, "20240315_a2601_成交量_买持仓_卖持仓排名.txt");
        let text = crate::util::decode_text(&entries[0].1, None, Some(encoding_rs::GB18030));
        assert_eq!(parse_dce_rank_text(&text, "A2601").len(), 2);
    }

    #[test]
    fn test_parse_dce_number_placeholders() {
        assert_eq!(parse_dce_number("-"), 0.0);
        assert_eq!(parse_dce_number(""), 0.0);
        assert_eq!(parse_dce_number("1,234.5"), 1234.5);
    }

    #[test]
    fn test_parse_dce_rank_text_without_three_sections() {
        let text = "名次\t会员简称\t成交量\t增减\n1\tA\t1\t0\n";
        assert!(parse_dce_rank_text(text, "A2601").is_empty());
    }
}
