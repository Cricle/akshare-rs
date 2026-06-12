//! Futures derivative data: contract info, hog data, position rankings.
//!
//! Ports of functions from `akshare/futures_derivative/`.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;

impl AkShareClient {
    // =========================================================================
    // Contract info per exchange
    // =========================================================================

    /// CFFEX contract trading parameters (交易参数).
    ///
    /// Fetches XML data from CFFEX for the given date (YYYYMMDD format).
    pub async fn futures_contract_info_cffex(&self, date: &str) -> Result<Vec<Row>> {
        let year = &date[..6];
        let day = &date[6..];
        let url = format!("http://www.cffex.com.cn/sj/jycs/{year}/{day}/index.xml");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        // Simple XML parsing: extract <INDEX> records
        let mut items = Vec::new();
        let records: Vec<&str> = body.split("<INDEX>").collect();
        for record in &records[1..] {
            let end = record.find("</INDEX>").unwrap_or(record.len());
            let record_text = &record[..end];
            let mut r = Row::new();
            // Extract each XML tag
            for tag in &[
                "TRADING_DAY",
                "PRODUCT_ID",
                "INSTRUMENT_ID",
                "INSTRUMENT_MONTH",
                "BASIS_PRICE",
                "OPEN_DATE",
                "END_TRADING_DAY",
                "UPPER_VALUE",
                "LOWER_VALUE",
                "UPPERLIMITPRICE",
                "LOWERLIMITPRICE",
                "LONG_LIMIT",
            ] {
                let open_tag = format!("<{tag}>");
                let close_tag = format!("</{tag}>");
                if let Some(start) = record_text.find(&open_tag) {
                    let value_start = start + open_tag.len();
                    if let Some(end_pos) = record_text[value_start..].find(&close_tag) {
                        let value = &record_text[value_start..value_start + end_pos];
                        let key = match *tag {
                            "TRADING_DAY" => "trading_day",
                            "PRODUCT_ID" => "product_id",
                            "INSTRUMENT_ID" => "instrument_id",
                            "INSTRUMENT_MONTH" => "instrument_month",
                            "BASIS_PRICE" => "basis_price",
                            "OPEN_DATE" => "open_date",
                            "END_TRADING_DAY" => "end_trading_day",
                            "UPPER_VALUE" => "upper_value",
                            "LOWER_VALUE" => "lower_value",
                            "UPPERLIMITPRICE" => "upper_limit_price",
                            "LOWERLIMITPRICE" => "lower_limit_price",
                            "LONG_LIMIT" => "long_limit",
                            _ => continue,
                        };
                        r.insert(key.into(), serde_json::json!(value));
                    }
                }
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        Ok(items)
    }

    /// CZCE contract reference data (参考数据).
    ///
    /// Fetches XML data from CZCE for the given date (YYYYMMDD format).
    pub async fn futures_contract_info_czce(&self, date: &str) -> Result<Vec<Row>> {
        let url = format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{date}/FutureDataReferenceData.xml"
        );
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .header("Host", "www.czce.com.cn")
            .send()
            .await?
            .text()
            .await?;

        // Parse XML: extract <Contract> records
        let mut items = Vec::new();
        let records: Vec<&str> = body.split("<Contract>").collect();
        for record in &records[1..] {
            let end = record.find("</Contract>").unwrap_or(record.len());
            let record_text = &record[..end];
            let mut r = Row::new();
            // Extract each XML tag
            for tag in &[
                "Name",
                "CtrCd",
                "PrdCd",
                "PrdTp",
                "ExchCd",
                "SegTp",
                "TrdHrs",
                "TrdCtyCd",
                "TrdCcyCd",
                "ClrngCcyCd",
                "ExpiryTime",
                "SettleTp",
                "Duration",
                "TckSz",
                "TckVal",
                "CtrSz",
                "MsrmntUnt",
                "MaxOrdSz",
                "MnthPosLmt",
                "MinBlckTrdSz",
                "CesrEaaFl",
                "FlexElgblFl",
                "ListCy",
                "DlvryNtcDt",
                "FrstTrdDt",
                "LstTrdDt",
                "DlvrySettleDt",
                "MnthCd",
                "YrCd",
                "LstDlvryDt",
                "LstDlvryDtBoard",
                "DlvryMnth",
                "Margin",
                "PxLim",
                "FeeCcy",
                "TrdFee",
                "FeeCollectionType",
                "DlvryFee",
                "IntraDayTrdFee",
                "TradingLimit",
            ] {
                let open_tag = format!("<{tag}>");
                let close_tag = format!("</{tag}>");
                if let Some(start) = record_text.find(&open_tag) {
                    let value_start = start + open_tag.len();
                    if let Some(end_pos) = record_text[value_start..].find(&close_tag) {
                        let value = &record_text[value_start..value_start + end_pos];
                        r.insert(tag.to_string(), serde_json::json!(value));
                    }
                }
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        Ok(items)
    }

    /// DCE contract information (合约信息查询).
    pub async fn futures_contract_info_dce(&self) -> Result<Vec<Row>> {
        let url = "http://www.dce.com.cn/dcereport/publicweb/tradepara/contractInfo";
        let payload = serde_json::json!({
            "lang": "zh",
            "tradeType": "1",
            "varietyId": "all",
        });

        let body = self.post(url).json(&payload).send().await?.text().await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for mut row in rows {
            let mut r = Row::new();
            r.insert(
                "variety".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("variety"))
                    .unwrap_or_default(),
            );
            r.insert(
                "contract_id".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("contractId"))
                    .unwrap_or_default(),
            );
            r.insert(
                "unit".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("unit"))
                    .unwrap_or_default(),
            );
            r.insert(
                "tick".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("tick"))
                    .unwrap_or_default(),
            );
            r.insert(
                "start_trade_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("startTradeDate"))
                    .unwrap_or_default(),
            );
            r.insert(
                "end_trade_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("endTradeDate"))
                    .unwrap_or_default(),
            );
            r.insert(
                "end_delivery_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("endDeliveryDate"))
                    .unwrap_or_default(),
            );
            items.push(r);
        }
        Ok(items)
    }

    /// GFEX contract information (合约信息).
    pub async fn futures_contract_info_gfex(&self) -> Result<Vec<Row>> {
        let url = "http://www.gfex.com.cn/u/interfacesWebTtQueryContractInfo/loadList";
        let body = self
            .post(url)
            .query(&[("variety", ""), ("trade_type", "0")])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for mut row in rows {
            let mut r = Row::new();
            r.insert(
                "variety".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("variety"))
                    .unwrap_or_default(),
            );
            r.insert(
                "contract_id".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("contractId"))
                    .unwrap_or_default(),
            );
            r.insert(
                "unit".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("unit"))
                    .unwrap_or_default(),
            );
            r.insert(
                "tick".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("tick"))
                    .unwrap_or_default(),
            );
            r.insert(
                "start_trade_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("startTradeDate"))
                    .unwrap_or_default(),
            );
            r.insert(
                "end_trade_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("endTradeDate"))
                    .unwrap_or_default(),
            );
            r.insert(
                "end_delivery_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("endDeliveryDate0"))
                    .unwrap_or_default(),
            );
            items.push(r);
        }
        Ok(items)
    }

    /// INE contract base info (上海国际能源交易中心交易参数).
    pub async fn futures_contract_info_ine(&self, date: &str) -> Result<Vec<Row>> {
        let url =
            format!("https://www.ine.cn/data/busiparamdata/future/ContractBaseInfo{date}.dat");
        let body = self
            .get(&url)
            .query(&[("rnd", "0.8312696798757147")])
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["ContractBaseInfo"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for mut row in rows {
            let mut r = Row::new();
            r.insert(
                "instrument_id".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("INSTRUMENTID"))
                    .unwrap_or_default(),
            );
            r.insert(
                "open_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("OPENDATE"))
                    .unwrap_or_default(),
            );
            r.insert(
                "expire_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("EXPIREDATE"))
                    .unwrap_or_default(),
            );
            r.insert(
                "start_delivery_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("STARTDELIVDATE"))
                    .unwrap_or_default(),
            );
            r.insert(
                "end_delivery_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("ENDDELIVDATE"))
                    .unwrap_or_default(),
            );
            r.insert(
                "basis_price".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("BASISPRICE"))
                    .unwrap_or_default(),
            );
            r.insert(
                "trading_day".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("TRADINGDAY"))
                    .unwrap_or_default(),
            );
            items.push(r);
        }
        Ok(items)
    }

    /// SHFE contract base info (上海期货交易所交易参数).
    pub async fn futures_contract_info_shfe(&self, date: &str) -> Result<Vec<Row>> {
        let url =
            format!("https://www.shfe.com.cn/data/busiparamdata/future/ContractBaseInfo{date}.dat");
        let body = self
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["ContractBaseInfo"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        let mut items = Vec::new();
        for mut row in rows {
            let mut r = Row::new();
            r.insert(
                "instrument_id".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("INSTRUMENTID"))
                    .unwrap_or_default(),
            );
            r.insert(
                "open_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("OPENDATE"))
                    .unwrap_or_default(),
            );
            r.insert(
                "expire_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("EXPIREDATE"))
                    .unwrap_or_default(),
            );
            r.insert(
                "start_delivery_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("STARTDELIVDATE"))
                    .unwrap_or_default(),
            );
            r.insert(
                "end_delivery_date".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("ENDDELIVDATE"))
                    .unwrap_or_default(),
            );
            r.insert(
                "basis_price".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("BASISPRICE"))
                    .unwrap_or_default(),
            );
            r.insert(
                "trading_day".into(),
                row.as_object_mut()
                    .and_then(|m| m.remove("TRADINGDAY"))
                    .unwrap_or_default(),
            );
            if let Some(update_date) = data.get("update_date") {
                r.insert("update_date".into(), update_date.clone());
            }
            items.push(r);
        }
        Ok(items)
    }

    // =========================================================================
    // Hog data (生猪数据)
    // =========================================================================

    /// Hog core price data from Zhuwang (玄田数据).
    ///
    /// `symbol`: "外三元", "内三元", "土杂猪"
    pub async fn futures_hog_core(&self, symbol: &str) -> Result<Vec<Row>> {
        let ptype = match symbol {
            "外三元" => "1",
            "内三元" => "2",
            "土杂猪" => "3",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported hog core symbol: {symbol}"
                )));
            }
        };

        let url = "https://xt.yangzhu.vip/data/getzhujiahitsdata";
        let body = self
            .post(url)
            .query(&[("ptype", ptype), ("areano", "-1"), ("datetype", "0")])
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in rows {
            let Some(arr) = row.as_array() else {
                continue;
            };
            if arr.len() < 2 {
                continue;
            }
            let mut r = Row::new();
            r.insert("date".into(), arr[1].clone());
            r.insert("value".into(), arr[0].clone());
            items.push(r);
        }
        Ok(items)
    }

    /// Hog cost dimension data from Zhuwang (玄田数据-成本维度).
    ///
    /// `symbol`: "玉米", "豆粕", "二元母猪价格", "仔猪价格"
    pub async fn futures_hog_cost(&self, symbol: &str) -> Result<Vec<Row>> {
        let (ptype, url) = match symbol {
            "玉米" => ("4", "https://xt.yangzhu.vip/data/getzhujiahitsdata"),
            "豆粕" => ("5", "https://xt.yangzhu.vip/data/getzhujiahitsdata"),
            "二元母猪价格" => ("1", "https://xt.yangzhu.vip/data/getmapdata"),
            "仔猪价格" => ("2", "https://xt.yangzhu.vip/data/getmapdata"),
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported hog cost symbol: {symbol}"
                )));
            }
        };

        let body = self
            .post(url)
            .query(&[("ptype", ptype), ("areano", "-1")])
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in rows {
            let Some(arr) = row.as_array() else {
                continue;
            };
            let mut r = Row::new();
            if symbol == "玉米" || symbol == "豆粕" {
                // getzhujiahitsdata returns [value, date]
                if arr.len() >= 2 {
                    r.insert("date".into(), arr[1].clone());
                    r.insert("value".into(), arr[0].clone());
                }
            } else {
                // getmapdata returns [date, value]
                if arr.len() >= 2 {
                    r.insert("date".into(), arr[0].clone());
                    r.insert("value".into(), arr[1].clone());
                }
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        Ok(items)
    }

    /// Hog supply dimension data from Zhuwang (玄田数据-供应维度).
    ///
    /// `symbol`: "猪肉批发价", "储备冻猪肉", "饲料原料数据", "白条肉",
    ///          "生猪产能", "育肥猪", "肉类价格指数", "猪粮比价"
    pub async fn futures_hog_supply(&self, symbol: &str) -> Result<Vec<Row>> {
        let ptype = match symbol {
            "猪肉批发价" => "3",
            "储备冻猪肉" => "4",
            "饲料原料数据" => "5",
            "白条肉" => "6",
            "生猪产能" => "7",
            "育肥猪" => "9",
            "肉类价格指数" => "10",
            "猪粮比价" => "11",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported hog supply symbol: {symbol}"
                )));
            }
        };

        let url = "https://xt.yangzhu.vip/data/getmapdata";
        let body = self
            .post(url)
            .query(&[("ptype", ptype), ("areano", "-1")])
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let rows = data["data"].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for row in rows {
            let Some(arr) = row.as_array() else {
                continue;
            };
            let mut r = Row::new();
            match symbol {
                "猪肉批发价" | "肉类价格指数" => {
                    // Returns [date, item, value]
                    if arr.len() >= 3 {
                        r.insert("date".into(), arr[0].clone());
                        r.insert("value".into(), arr[2].clone());
                    }
                }
                "饲料原料数据" => {
                    // Returns complex data
                    if arr.len() >= 5 {
                        r.insert("period".into(), arr[0].clone());
                        r.insert("soybean_import".into(), arr[1].clone());
                        r.insert("soybean_area".into(), arr[2].clone());
                        r.insert("corn_import".into(), arr[3].clone());
                        r.insert("corn_area".into(), arr[4].clone());
                    }
                }
                "白条肉" => {
                    // Returns [period, price, mom, yoy]
                    if arr.len() >= 4 {
                        r.insert("period".into(), arr[0].clone());
                        r.insert("price".into(), arr[1].clone());
                        r.insert("mom".into(), arr[2].clone());
                        r.insert("yoy".into(), arr[3].clone());
                    }
                }
                "生猪产能" => {
                    // Returns [period, sow_inventory, pork_output, hog_inventory, hog_output]
                    if arr.len() >= 5 {
                        r.insert("period".into(), arr[0].clone());
                        r.insert("sow_inventory".into(), arr[1].clone());
                        r.insert("pork_output".into(), arr[2].clone());
                        r.insert("hog_inventory".into(), arr[3].clone());
                        r.insert("hog_output".into(), arr[4].clone());
                    }
                }
                "育肥猪" => {
                    // Returns [date, benzhou]
                    if arr.len() >= 2 {
                        r.insert("date".into(), arr[0].clone());
                        r.insert("value".into(), arr[1].clone());
                    }
                }
                _ => {
                    // 储备冻猪肉, 猪粮比价: Returns [date, value]
                    if arr.len() >= 2 {
                        r.insert("date".into(), arr[0].clone());
                        r.insert("value".into(), arr[1].clone());
                    }
                }
            }
            if !r.is_empty() {
                items.push(r);
            }
        }
        Ok(items)
    }

    // =========================================================================
    // Sina main contract display and data
    // =========================================================================

    /// Display all main continuous contracts from Sina across all exchanges.
    ///
    /// Iterates over dce, czce, shfe, cffex, gfex to find main contracts.
    /// Returns (exchange, symbol, name) for each main contract.
    pub async fn futures_display_main_sina(&self) -> Result<Vec<Row>> {
        let mut all_items = Vec::new();
        for exchange in &["dce", "czce", "shfe", "cffex", "gfex"] {
            let items = self.match_main_contract_sina(exchange).await?;
            all_items.extend(items);
        }
        Ok(all_items)
    }

    /// Helper: find main contracts for a given exchange from Sina.
    async fn match_main_contract_sina(&self, exchange: &str) -> Result<Vec<Row>> {
        // Get exchange symbol list
        let symbols = self.zh_subscribe_exchange_symbol(exchange).await?;
        let mut items = Vec::new();

        for sym_entry in &symbols {
            let node = match sym_entry.get("mark").and_then(|v| v.as_str()) {
                Some(n) => n,
                None => continue,
            };

            let url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQFuturesData";
            let body = self
                .get(url)
                .query(&[
                    ("page", "1"),
                    ("sort", "position"),
                    ("asc", "0"),
                    ("node", node),
                    ("base", "futures"),
                ])
                .header("Referer", "https://vip.stock.finance.sina.com.cn/")
                .send()
                .await?
                .text()
                .await?;

            let data: Vec<serde_json::Value> = match serde_json::from_str(&body) {
                Ok(d) => d,
                Err(_) => continue,
            };

            // Find main contract: typically the one with "连续" in name and "0" in symbol
            for entry in &data {
                let name = entry["name"].as_str().unwrap_or("");
                let symbol = entry["symbol"].as_str().unwrap_or("");
                if name.contains("连续") && symbol.ends_with('0') {
                    let mut r = Row::new();
                    r.insert("exchange".into(), serde_json::json!(exchange));
                    r.insert("symbol".into(), serde_json::json!(symbol));
                    r.insert("name".into(), serde_json::json!(name));
                    items.push(r);
                    break;
                }
            }
        }
        Ok(items)
    }

    /// Get exchange symbol list from Sina.
    async fn zh_subscribe_exchange_symbol(&self, exchange: &str) -> Result<Vec<Row>> {
        let url = "https://vip.stock.finance.sina.com.cn/quotes_service/view/js/qihuohangqing.js";
        let body = self
            .get(url)
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .send()
            .await?
            .text()
            .await?;

        let json_str = body
            .find('{')
            .and_then(|start| {
                body[start..]
                    .find('}')
                    .map(|end| &body[start..start + end + 1])
            })
            .ok_or_else(|| Error::decode("sina exchange symbols: invalid JS response"))?;

        let data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|_| Error::decode("sina exchange symbols: JSON parse error"))?;

        let arr = data[exchange].as_array().cloned().unwrap_or_default();

        let mut items = Vec::new();
        for entry in arr.iter().skip(1) {
            if let Some(arr_entry) = entry.as_array()
                && arr_entry.len() >= 2
            {
                let mut r = Row::new();
                r.insert("symbol".into(), arr_entry[0].clone());
                r.insert("mark".into(), arr_entry[1].clone());
                items.push(r);
            }
        }
        Ok(items)
    }

    /// Main contract daily data from Sina (derivative version).
    ///
    /// Fetches daily kline data for a given symbol with date filtering.
    pub async fn futures_main_sina_derivative(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<Row>> {
        let date = "20210817";
        let date_formatted = format!("{}_{}_{}", &date[..4], &date[4..6], &date[6..8]);
        let url = format!(
            "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_{symbol}{date_formatted}=/InnerFuturesNewService.getDailyKLine?symbol={symbol}&_={date_formatted}"
        );
        let body = self
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        let json_str = body
            .find("[[")
            .and_then(|start| {
                body[start..]
                    .rfind("]]")
                    .map(|end| &body[start..start + end + 2])
            })
            .ok_or_else(|| Error::decode("sina main derivative: invalid JSONP response"))?;

        let rows: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)
            .map_err(|_| Error::decode("sina main derivative: JSON parse error"))?;

        let start_dt = chrono::NaiveDate::parse_from_str(start_date, "%Y%m%d")
            .unwrap_or(chrono::NaiveDate::MIN);
        let end_dt =
            chrono::NaiveDate::parse_from_str(end_date, "%Y%m%d").unwrap_or(chrono::NaiveDate::MAX);

        let mut items = Vec::new();
        for row in rows {
            if row.len() < 8 {
                continue;
            }
            let date_str = row[0].as_str().unwrap_or("");
            let dt = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d");
            if let Ok(dt) = dt
                && (dt < start_dt || dt > end_dt)
            {
                continue;
            }
            let mut r = Row::new();
            r.insert("date".into(), row[0].clone());
            r.insert("open".into(), row[1].clone());
            r.insert("high".into(), row[2].clone());
            r.insert("low".into(), row[3].clone());
            r.insert("close".into(), row[4].clone());
            r.insert("volume".into(), row[5].clone());
            r.insert("hold".into(), row[6].clone());
            r.insert("settle".into(), row[7].clone());
            items.push(r);
        }
        Ok(items)
    }

    /// Spot-futures comparison data from 100ppi.com (生意社).
    ///
    /// Returns a placeholder noting HTML scraping is required.
    pub async fn futures_spot_sys(&self, symbol: &str, indicator: &str) -> Result<Vec<Row>> {
        let mut items = Vec::new();
        let mut r = Row::new();
        r.insert("symbol".into(), serde_json::json!(symbol));
        r.insert("indicator".into(), serde_json::json!(indicator));
        r.insert("source".into(), serde_json::json!("100ppi.com"));
        r.insert(
            "note".into(),
            serde_json::json!("requires HTML scraping from 100ppi.com"),
        );
        items.push(r);
        Ok(items)
    }
}
