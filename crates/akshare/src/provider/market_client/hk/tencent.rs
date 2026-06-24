use super::super::{CandlePoint, MarketDataClient, QuoteSnapshot};
use anyhow::{Context, bail};
use chrono::NaiveDate;
impl MarketDataClient {
    pub(crate) async fn fetch_hk_tencent_quote(
        &self,
        symbol: &str,
        compact_code: &str,
    ) -> anyhow::Result<QuoteSnapshot> {
        let market_symbol = format!("hk{compact_code}");
        let response = self
            .http
            .get("https://qt.gtimg.cn/q")
            .query(&[("q", market_symbol.as_str())])
            .header(reqwest::header::REFERER, "https://gu.qq.com/")
            .send()
            .await
            .context("failed to fetch HK quote from Tencent")?
            .error_for_status()
            .context("tencent HK quote request failed")?
            .text()
            .await
            .context("failed to read tencent HK quote response")?;
        Self::parse_hk_tencent_quote(symbol, &response)
    }

    fn parse_hk_tencent_quote(symbol: &str, raw: &str) -> anyhow::Result<QuoteSnapshot> {
        let fields = raw
            .split_once('"')
            .and_then(|(_, rest)| rest.rsplit_once('"').map(|(body, _)| body))
            .context("unexpected tencent HK quote response format")?
            .split('~')
            .collect::<Vec<_>>();
        if fields.len() < 40 {
            bail!("unexpected tencent HK quote field count");
        }
        let trade_date = normalize_hk_trade_date(fields.get(30).copied())?;
        QuoteResponseBuilder::new(symbol.to_uppercase(), trade_date)
            .open(fields.get(5).copied())
            .close(fields.get(3).copied())
            .high(fields.get(33).copied())
            .low(fields.get(34).copied())
            .volume(fields.get(6).copied())
            .build()
    }

    pub(crate) async fn fetch_hk_tencent_candles(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CandlePoint>> {
        let market_symbol = format!("hk{}", self.hk_standard_code(symbol)?);
        let request_limit = limit.clamp(5, Self::HK_TENCENT_MAX_CANDLE_LIMIT);
        let response = self
            .http
            .get("https://web.ifzq.gtimg.cn/appstock/app/fqkline/get")
            .query(&[("param", format!("{market_symbol},day,,,{},", request_limit))])
            .send()
            .await
            .context("failed to fetch HK candles from Tencent")?
            .error_for_status()
            .context("tencent HK candle request failed")?
            .json::<serde_json::Value>()
            .await
            .context("failed to decode tencent HK candle response")?;

        let series = response
            .get("data")
            .and_then(|data| data.get(&market_symbol))
            .and_then(|item| item.get("day"))
            .and_then(|value| value.as_array())
            .context("tencent HK candle response missing day series")?;

        let mut items = series
            .iter()
            .map(Self::parse_hk_tencent_candle_row)
            .collect::<anyhow::Result<Vec<_>>>()?;
        if items.is_empty() {
            bail!("tencent HK candle response returned no rows");
        }
        items.sort_by(|left, right| left.trade_date.cmp(&right.trade_date));
        for index in 1..items.len() {
            let previous_close = items[index - 1].close;
            if previous_close > 0.0 {
                items[index].change_amount = items[index].close - previous_close;
                items[index].change_pct = (items[index].change_amount / previous_close) * 100.0;
            }
        }
        if items.len() > limit {
            let start_index = items.len() - limit;
            items = items[start_index..].to_vec();
        }
        Ok(items)
    }

    pub(crate) async fn fetch_hk_yahoo_quote(&self, symbol: &str) -> anyhow::Result<QuoteSnapshot> {
        let mut items = self.fetch_hk_yahoo_candles(symbol, 2).await?;
        let last = items
            .pop()
            .context("Yahoo Finance HK quote returned no rows")?;
        Ok(QuoteSnapshot {
            symbol: symbol.trim().to_uppercase(),
            date: last.trade_date,
            open: last.open,
            high: last.high,
            low: last.low,
            close: last.close,
            volume: last.volume,
        })
    }

    pub(crate) async fn fetch_hk_yahoo_candles(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CandlePoint>> {
        let yahoo_symbol = self.hk_yahoo_symbol(symbol)?;
        let end = chrono::Utc::now().date_naive() + chrono::Days::new(1);
        let start = end - chrono::Days::new((limit.max(60) + 10) as u64);
        let mut items = self
            .fetch_us_chart_candles(&yahoo_symbol, start, end)
            .await?;
        if items.is_empty() {
            bail!("Yahoo Finance HK candles returned no rows");
        }
        if items.len() > limit {
            let start_index = items.len() - limit;
            items = items[start_index..].to_vec();
        }
        Ok(items)
    }

    pub(super) fn hk_yahoo_symbol(&self, symbol: &str) -> anyhow::Result<String> {
        let normalized = self
            .normalize_hk_symbol(symbol)
            .context("invalid HK symbol for candles")?;
        let code = normalized.trim_end_matches(".HK");
        let compact = code.trim_start_matches('0');
        let yahoo_code = if compact.is_empty() {
            "0000".to_string()
        } else {
            format!("{compact:0>4}")
        };
        Ok(format!("{yahoo_code}.HK"))
    }

    pub(super) async fn fetch_hk_tencent_snapshot(
        &self,
        symbol: &str,
    ) -> anyhow::Result<HkTencentSnapshot> {
        let compact = self.hk_standard_code(symbol)?;
        let market_symbol = format!("hk{compact}");
        let response = self
            .http
            .get("https://qt.gtimg.cn/q")
            .query(&[("q", market_symbol.as_str())])
            .header(reqwest::header::REFERER, "https://gu.qq.com/")
            .send()
            .await
            .context("failed to fetch HK quote snapshot from Tencent")?
            .error_for_status()
            .context("tencent HK snapshot request failed")?
            .text()
            .await
            .context("failed to read tencent HK snapshot response")?;
        Self::parse_hk_tencent_snapshot(&response)
    }

    pub(super) fn parse_hk_tencent_snapshot(raw: &str) -> anyhow::Result<HkTencentSnapshot> {
        let fields = raw
            .split_once('"')
            .and_then(|(_, rest)| rest.rsplit_once('"').map(|(body, _)| body))
            .context("unexpected tencent HK quote response format")?
            .split('~')
            .collect::<Vec<_>>();
        if fields.len() < 76 {
            bail!("unexpected tencent HK quote field count");
        }
        Ok(HkTencentSnapshot {
            name: fields[1].trim().to_string(),
            market_cap_hkd: fields
                .get(45)
                .and_then(|value| value.parse::<f64>().ok())
                .map(|value| value * 100_000_000.0),
            shares_outstanding: match (
                fields.get(3).and_then(|value| value.parse::<f64>().ok()),
                fields
                    .get(45)
                    .and_then(|value| value.parse::<f64>().ok())
                    .map(|value| value * 100_000_000.0),
            ) {
                (Some(close), Some(market_cap)) if close > 0.0 => {
                    Some((market_cap / close).round() as i64)
                }
                _ => None,
            },
            currency: fields
                .get(75)
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
        })
    }

    fn parse_hk_tencent_candle_row(value: &serde_json::Value) -> anyhow::Result<CandlePoint> {
        let row = value
            .as_array()
            .context("unexpected tencent HK candle row format")?;
        if row.len() < 6 {
            bail!("unexpected tencent HK candle field count");
        }
        let trade_date = row[0]
            .as_str()
            .context("missing tencent HK candle trade_date")?
            .to_string();
        let open = row[1]
            .as_str()
            .context("missing tencent HK candle open")?
            .parse::<f64>()
            .context("invalid tencent HK candle open")?;
        let close = row[2]
            .as_str()
            .context("missing tencent HK candle close")?
            .parse::<f64>()
            .context("invalid tencent HK candle close")?;
        let high = row[3]
            .as_str()
            .context("missing tencent HK candle high")?
            .parse::<f64>()
            .context("invalid tencent HK candle high")?;
        let low = row[4]
            .as_str()
            .context("missing tencent HK candle low")?
            .parse::<f64>()
            .context("invalid tencent HK candle low")?;
        let volume = row[5]
            .as_str()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or_default()
            .round() as i64;
        Ok(CandlePoint {
            trade_date,
            open,
            close,
            high,
            low,
            volume,
            amount: 0.0,
            amplitude_pct: if low > 0.0 {
                ((high - low) / low) * 100.0
            } else {
                0.0
            },
            change_pct: 0.0,
            change_amount: 0.0,
            turnover_pct: 0.0,
        })
    }
}

#[derive(Debug, Clone)]
pub(super) struct HkTencentSnapshot {
    pub(super) name: String,
    pub(super) market_cap_hkd: Option<f64>,
    pub(super) shares_outstanding: Option<i64>,
    pub(super) currency: Option<String>,
}

struct QuoteResponseBuilder {
    symbol: String,
    date: String,
    open: Option<f64>,
    close: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    volume: Option<i64>,
}

impl QuoteResponseBuilder {
    fn new(symbol: String, date: String) -> Self {
        Self {
            symbol,
            date,
            open: None,
            close: None,
            high: None,
            low: None,
            volume: None,
        }
    }

    fn open(mut self, raw: Option<&str>) -> Self {
        self.open = raw.and_then(|value| value.parse::<f64>().ok());
        self
    }

    fn close(mut self, raw: Option<&str>) -> Self {
        self.close = raw.and_then(|value| value.parse::<f64>().ok());
        self
    }

    fn high(mut self, raw: Option<&str>) -> Self {
        self.high = raw.and_then(|value| value.parse::<f64>().ok());
        self
    }

    fn low(mut self, raw: Option<&str>) -> Self {
        self.low = raw.and_then(|value| value.parse::<f64>().ok());
        self
    }

    fn volume(mut self, raw: Option<&str>) -> Self {
        self.volume = raw
            .and_then(|value| value.parse::<f64>().ok())
            .map(|value| value.round() as i64);
        self
    }

    fn build(self) -> anyhow::Result<QuoteSnapshot> {
        Ok(QuoteSnapshot {
            symbol: self.symbol,
            date: self.date,
            open: self.open.context("missing HK quote open")?,
            high: self.high.context("missing HK quote high")?,
            low: self.low.context("missing HK quote low")?,
            close: self.close.context("missing HK quote close")?,
            volume: self.volume.unwrap_or_default(),
        })
    }
}

pub(super) fn normalize_hk_trade_date(raw: Option<&str>) -> anyhow::Result<String> {
    let raw = raw
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .context("missing tencent HK trade date")?;
    let date = raw.get(0..10).unwrap_or(raw).replace('/', "-");
    NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map(|value| value.format("%Y-%m-%d").to_string())
        .context("invalid tencent HK trade date")
}
