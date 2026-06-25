//! ETF data from Eastmoney: daily listing, hist, spot, and fund info.
//! Also includes THS and Sina ETF functions.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::value_ext::ValueExt;
use crate::types::{CandlePoint, EtfScaleItem, EtfSpotItem, FundNavHistory, FundSnapshot};
use crate::util::parse_f64_safe;

/// Determine market id for an ETF symbol: 1=Shanghai, 0=Shenzhen.
fn etf_market_id(symbol: &str) -> &str {
    if symbol.starts_with('5') || symbol.starts_with('6') {
        "1"
    } else {
        "0"
    }
}

impl AkShareClient {
    /// Fetch ETF fund daily listing data from Eastmoney.
    ///
    /// Returns up to `limit` ETF funds with latest NAV and change percentage.
    pub async fn fund_etf_fund_daily(&self, limit: usize) -> Result<Vec<FundSnapshot>> {
        let pz = limit.max(1).to_string();
        let response = crate::util::send_and_check(
            self.get("https://push2.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params(pz.as_str(), &[
                ("fs", "b:MK0021,b:MK0022,b:MK0023,b:MK0024,b:MK0025"),
                ("fields", "f2,f3,f12,f14"),
                ]))
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let items = payload
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let today = crate::util::today_iso();
        let snapshots: Vec<FundSnapshot> = items
            .into_iter()
            .take(limit)
            .filter_map(|v| {
                let code = v.get("f12")?.as_str()?.to_string();
                let name = v.str_or(&["f14"], "");
                let price = v.f64_or(&["f2"], 0.0);
                let change_pct = v.f64_or(&["f3"], 0.0);
                Some(FundSnapshot {
                    symbol: code,
                    name,
                    date: today.clone(),
                    nav: price,
                    acc_nav: price,
                    change_pct,
                    fund_type: Some("etf".to_string()),
                })
            })
            .collect();

        if snapshots.is_empty() {
            return Err(Error::not_found("no ETF fund daily data"));
        }
        Ok(snapshots)
    }

    /// Fetch ETF historical candles from Eastmoney.
    ///
    /// `symbol`: 6-digit ETF code (e.g. "159707").
    /// `period`: "daily", "weekly", or "monthly".
    /// `start_date` / `end_date`: format "YYYYMMDD".
    /// `adjust`: "qfq" (forward), "hfq" (backward), or "" (no adjust).
    pub async fn fund_etf_hist_em(
        &self,
        symbol: &str,
        period: &str,
        _start_date: &str,
        _end_date: &str,
        adjust: &str,
    ) -> Result<Vec<CandlePoint>> {
        let period_map = match period {
            "daily" => "101",
            "weekly" => "102",
            "monthly" => "103",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported period: {period}"
                )));
            }
        };
        let adjust_map = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "" | "none" => "0",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported adjust: {adjust}"
                )));
            }
        };
        let market_id = etf_market_id(symbol);
        let secid = format!("{market_id}.{symbol}");

        let klines = self
            .kline_fetch(&secid, period_map, adjust_map, usize::MAX, &[])
            .await?;

        let mut candles = Vec::new();
        for s in &klines {
            let fields: Vec<&str> = s.split(',').collect();
            if fields.len() < 11 {
                continue;
            }
            candles.push(CandlePoint {
                trade_date: fields[0].to_string(),
                open: parse_f64_safe(fields[1]),
                close: parse_f64_safe(fields[2]),
                high: parse_f64_safe(fields[3]),
                low: parse_f64_safe(fields[4]),
                volume: fields[5].parse().unwrap_or(0),
                amount: parse_f64_safe(fields[6]),
                amplitude_pct: parse_f64_safe(fields[7]),
                change_pct: parse_f64_safe(fields[8]),
                change_amount: parse_f64_safe(fields[9]),
                turnover_pct: parse_f64_safe(fields[10]),
            });
        }
        Ok(candles)
    }

    /// Fetch ETF minute-level historical data from Eastmoney.
    ///
    /// `symbol`: 6-digit ETF code.
    /// `period`: "1", "5", "15", "30", or "60".
    /// `start_date` / `end_date`: format "YYYY-MM-DD HH:MM:SS".
    /// `adjust`: "", "qfq", or "hfq".
    pub async fn fund_etf_hist_min(
        &self,
        symbol: &str,
        period: &str,
        start_date: &str,
        end_date: &str,
        adjust: &str,
    ) -> Result<Vec<CandlePoint>> {
        let adjust_map = match adjust {
            "qfq" => "1",
            "hfq" => "2",
            "" | "none" => "0",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported adjust: {adjust}"
                )));
            }
        };
        let market_id = etf_market_id(symbol);

        if period == "1" {
            // 1-minute uses a different endpoint (trends2)
            let response = crate::util::send_and_check(
                self.get("https://push2his.eastmoney.com/api/qt/stock/trends2/get")
                    .query(&[
                    ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13"),
                    ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58"),
                    ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
                    ("ndays", "5"),
                    ("iscr", "0"),
                    ("secid", &format!("{market_id}.{symbol}")),
                    ])
            )
            .await?;

            let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
            let trends = payload
                .get("data")
                .and_then(|d| d.get("trends"))
                .and_then(|t| t.as_array())
                .ok_or_else(|| Error::not_found(format!("no trend data for ETF {symbol}")))?;

            let mut candles = Vec::new();
            for trend in trends {
                let s = trend.as_str().unwrap_or("");
                let fields: Vec<&str> = s.split(',').collect();
                if fields.len() < 8 {
                    continue;
                }
                let dt = fields[0];
                if dt < start_date || dt > end_date {
                    continue;
                }
                candles.push(CandlePoint {
                    trade_date: dt.to_string(),
                    open: parse_f64_safe(fields[1]),
                    close: parse_f64_safe(fields[2]),
                    high: parse_f64_safe(fields[3]),
                    low: parse_f64_safe(fields[4]),
                    volume: fields[5].parse().unwrap_or(0),
                    amount: parse_f64_safe(fields[6]),
                    amplitude_pct: 0.0,
                    change_pct: 0.0,
                    change_amount: 0.0,
                    turnover_pct: 0.0,
                });
            }
            Ok(candles)
        } else {
            // 5/15/30/60-minute uses kline endpoint
            let secid = format!("{market_id}.{symbol}");
            let klines = self
                .kline_fetch(&secid, period, adjust_map, usize::MAX, &[])
                .await?;

            let mut candles = Vec::new();
            for s in &klines {
                let fields: Vec<&str> = s.split(',').collect();
                if fields.len() < 11 {
                    continue;
                }
                let dt = fields[0];
                if dt < start_date || dt > end_date {
                    continue;
                }
                candles.push(CandlePoint {
                    trade_date: dt.to_string(),
                    open: parse_f64_safe(fields[1]),
                    close: parse_f64_safe(fields[2]),
                    high: parse_f64_safe(fields[3]),
                    low: parse_f64_safe(fields[4]),
                    volume: fields[5].parse().unwrap_or(0),
                    amount: parse_f64_safe(fields[6]),
                    amplitude_pct: parse_f64_safe(fields[7]),
                    change_pct: parse_f64_safe(fields[8]),
                    change_amount: parse_f64_safe(fields[9]),
                    turnover_pct: parse_f64_safe(fields[10]),
                });
            }
            Ok(candles)
        }
    }

    /// Fetch ETF spot (real-time) data from Eastmoney.
    ///
    /// Returns all ETFs with real-time quotes.
    pub async fn fund_etf_spot_em(&self) -> Result<Vec<EtfSpotItem>> {
        let response = crate::util::send_and_check(
            self.get("https://push2delay.eastmoney.com/api/qt/clist/get")
                .query(&crate::util::eastmoney_clist_params("10000", &[
                ("fid", "f12"),
                ("fs", "b:MK0021,b:MK0022,b:MK0023,b:MK0024,b:MK0827"),
                (
                "fields",
                "f2,f3,f4,f5,f6,f7,f12,f14,f15,f16,f17,f18,f20,f21,f38,f62,f184,f402,f441,f297",
                ),
                ]))
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let items = payload
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();

        let result: Vec<EtfSpotItem> = items
            .into_iter()
            .filter_map(|v| {
                Some(EtfSpotItem {
                    code: v.get("f12")?.as_str()?.to_string(),
                    name: v.str_or(&["f14"], ""),
                    latest_price: v.f64_or(&["f2"], 0.0),
                    change_pct: v.f64_or(&["f3"], 0.0),
                    change_amount: v.f64_or(&["f4"], 0.0),
                    volume: v.f64_or(&["f5"], 0.0),
                    amount: v.f64_or(&["f6"], 0.0),
                    open: v.f64_or(&["f17"], 0.0),
                    high: v.f64_or(&["f15"], 0.0),
                    low: v.f64_or(&["f16"], 0.0),
                    prev_close: v.f64_or(&["f18"], 0.0),
                    amplitude: v.f64_or(&["f7"], 0.0),
                    turnover_rate: v.f64_or(&["f38"], 0.0),
                    iopv: v.f64_or(&["f441"], 0.0),
                    discount_rate: v.f64_or(&["f402"], 0.0),
                    shares: v.f64_or(&["f38"], 0.0),
                    circ_mv: v.f64_or(&["f21"], 0.0),
                    total_mv: v.f64_or(&["f20"], 0.0),
                    data_date: v.str_or(&["f297"], ""),
                })
            })
            .collect();

        if result.is_empty() {
            return Err(Error::not_found("no ETF spot data"));
        }
        Ok(result)
    }

    /// Fetch ETF fund info (historical NAV) from Eastmoney.
    ///
    /// `fund`: ETF fund code (e.g. "511280").
    /// `start_date` / `end_date`: format "YYYYMMDD".
    pub async fn fund_etf_fund_info(
        &self,
        fund: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<FundNavHistory>> {
        let sd = format!(
            "{}-{}-{}",
            &start_date[0..4],
            &start_date[4..6],
            &start_date[6..8]
        );
        let ed = format!(
            "{}-{}-{}",
            &end_date[0..4],
            &end_date[4..6],
            &end_date[6..8]
        );

        let response = crate::util::send_and_check(
            self.get("https://api.fund.eastmoney.com/f10/lsjz")
                .header(
                "Referer",
                format!("https://fundf10.eastmoney.com/jjjz_{fund}.html"),
                )
                .query(&[
                ("fundCode", fund),
                ("pageIndex", "1"),
                ("pageSize", "10000"),
                ("startDate", sd.as_str()),
                ("endDate", ed.as_str()),
                ])
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let list = payload
            .get("Data")
            .and_then(|d| d.get("LSJZList"))
            .and_then(|l| l.as_array())
            .ok_or_else(|| Error::not_found(format!("no NAV data for fund {fund}")))?;

        let mut result = Vec::new();
        for item in list {
            let arr = item.as_array();
            if let Some(arr) = arr {
                if arr.len() < 10 {
                    continue;
                }
                result.push(FundNavHistory {
                    date: arr[0].as_str().unwrap_or("").to_string(),
                    nav: arr[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    acc_nav: arr[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    change_pct: arr[6].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    subscribe_status: arr[7].as_str().unwrap_or("").to_string(),
                    redeem_status: arr[8].as_str().unwrap_or("").to_string(),
                });
            }
        }

        if result.is_empty() {
            return Err(Error::not_found(format!("no NAV data for fund {fund}")));
        }
        result.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(result)
    }

    /// Fetch SSE ETF scale data.
    ///
    /// `date`: format "YYYYMMDD" (e.g. "20250115").
    pub async fn fund_etf_scale_sse(&self, date: &str) -> Result<Vec<EtfScaleItem>> {
        let data_str = format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8]);
        let response = crate::util::send_and_check(
            self.get("https://query.sse.com.cn/commonQuery.do")
                .header("Referer", "https://www.sse.com.cn/")
                .query(&[
                ("isPagination", "true"),
                ("pageHelp.pageSize", "10000"),
                ("pageHelp.pageNo", "1"),
                ("pageHelp.beginPage", "1"),
                ("pageHelp.cacheSize", "1"),
                ("pageHelp.endPage", "1"),
                ("sqlId", "COMMON_SSE_ZQPZ_ETFZL_XXPL_ETFGM_SEARCH_L"),
                ("STAT_DATE", data_str.as_str()),
                ])
        )
        .await?;

        let payload: serde_json::Value = response.json().await.map_err(Error::from)?;
        let items = payload
            .get("result")
            .and_then(|r| r.as_array())
            .ok_or_else(|| Error::not_found("no SSE ETF scale data"))?;

        let mut result = Vec::new();
        for (i, item) in items.iter().enumerate() {
            result.push(EtfScaleItem {
                rank: (i + 1) as i32,
                fund_code: item.str_or(&["SEC_CODE"], ""),
                fund_name: item.str_or(&["SEC_NAME"], ""),
                etf_type: item.str_or(&["ETF_TYPE"], ""),
                stat_date: item.str_or(&["STAT_DATE"], ""),
                shares: item.f64_or(&["TOT_VOL"], 0.0) * 10000.0,
            });
        }

        if result.is_empty() {
            return Err(Error::not_found("no SSE ETF scale data"));
        }
        Ok(result)
    }

    /// Fetch SZSE ETF scale data.
    ///
    /// Returns ETF fund shares data from Shenzhen Stock Exchange.
    pub async fn fund_etf_scale_szse(&self) -> Result<Vec<serde_json::Value>> {
        // SZSE returns xlsx content; cannot parse in pure Rust without a library.
        Err(Error::decode(
            "SZSE ETF scale data requires xlsx parsing (not supported in pure Rust)",
        ))
    }

    // --- THS functions ---

    /// Fetch THS fund category data (ETF, LOF, stock, bond, etc.).
    ///
    /// `symbol`: "ETF", "LOF", "股票型", "债券型", "混合型", "QDII", "保本型", "指数型", or "" for all.
    /// `date`: format "YYYYMMDD" or "" for latest.
    pub async fn fund_etf_category_ths(
        &self,
        symbol: &str,
        date: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let symbol_map: &[(&str, &str)] = &[
            ("股票型", "gpx"),
            ("债券型", "zqx"),
            ("混合型", "hhx"),
            ("ETF", "ETF"),
            ("LOF", "LOF"),
            ("QDII", "QDII"),
            ("保本型", "bbx"),
            ("指数型", "zsx"),
            ("", "all"),
        ];
        let inner_symbol = symbol_map
            .iter()
            .find(|(n, _)| *n == symbol)
            .map_or("ETF", |(_, c)| *c);

        let inner_date = if !date.is_empty() && date.len() >= 8 {
            format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
        } else {
            "0".to_string()
        };

        let url = format!(
            "https://fund.10jqka.com.cn/data/Net/info/{inner_symbol}_rate_desc_{inner_date}_0_1_9999_0_0_0_jsonp_g.html"
        );

        let response = crate::util::send_and_check(
            self.get(&url)
        )
        .await?;

        let text = response.text().await.map_err(Error::from)?;
        // THS returns JSONP: g({...})
        let json_str = text
            .strip_prefix("g(")
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(&text);

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("THS JSON parse: {e}")))?;

        let data = root
            .get("data")
            .and_then(|d| d.get("data"))
            .ok_or_else(|| Error::not_found("THS fund category data missing"))?;

        // data is an object with numeric keys; convert to array
        let items: Vec<serde_json::Value> = if let Some(arr) = data.as_array() {
            arr.clone()
        } else if let Some(obj) = data.as_object() {
            obj.values().cloned().collect()
        } else {
            return Err(Error::decode("unexpected THS data format"));
        };

        if items.is_empty() {
            return Err(Error::not_found("no THS fund category data"));
        }
        Ok(items)
    }

    /// Fetch THS ETF spot data.
    ///
    /// `date`: format "YYYYMMDD" or "" for latest.
    pub async fn fund_etf_spot_ths(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        self.fund_etf_category_ths("ETF", date).await
    }

    // --- Sina functions ---

    /// Fetch Sina ETF dividend data.
    ///
    /// `symbol`: e.g. "sh510050" (with exchange prefix).
    pub async fn fund_etf_dividend(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let factor_url = format!("https://finance.sina.com.cn/realstock/company/{symbol}/hfq.js");
        let response = crate::util::send_and_check(
            self.get(&factor_url)
        )
        .await?;

        let text = response.text().await.map_err(Error::from)?;
        if !text.starts_with("var") {
            return Err(Error::not_found(format!("no dividend data for {symbol}")));
        }

        // Parse the JS variable assignment: var xxx={...}
        let json_start = text.find('{').unwrap_or(0);
        let json_end = text.rfind('}').map_or(text.len(), |i| i + 1);
        let json_str = &text[json_start..json_end];

        let root: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::decode(format!("Sina dividend JSON parse: {e}")))?;

        let data = root
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| Error::not_found(format!("no dividend data for {symbol}")))?;

        let mut result = Vec::new();
        for item in data {
            if let Some(arr) = item.as_array()
                && arr.len() >= 4
            {
                let date = arr[0].as_str().unwrap_or("");
                if date == "1900-01-01" {
                    continue;
                }
                result.push(serde_json::json!({
                    "date": date,
                    "cumulative_dividend": arr[3].as_f64().unwrap_or(0.0),
                }));
            }
        }

        if result.is_empty() {
            return Err(Error::not_found(format!("no dividend data for {symbol}")));
        }
        Ok(result)
    }
}
