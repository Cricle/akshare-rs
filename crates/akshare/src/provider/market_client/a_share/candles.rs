use anyhow::{bail, Context};
use rust_decimal::prelude::ToPrimitive;
use serde_json::Value;

use crate::types::{CandlePoint, CandlesWithProvider};
use super::super::MarketDataClient;

impl MarketDataClient {
    pub(crate) async fn fetch_a_share_candles(
        &self,
        symbol: &str,
        adjust: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CandlePoint>> {
        self.fetch_a_share_candles_with_provider(symbol, adjust, limit)
            .await
            .map(|cp| cp.candles)
    }

    pub(crate) async fn fetch_a_share_candles_with_provider(
        &self,
        symbol: &str,
        adjust: &str,
        limit: usize,
    ) -> anyhow::Result<CandlesWithProvider> {
        match self
            .fetch_a_share_tencent_candles(symbol, adjust, limit)
            .await
        {
            Ok(items) => Ok(CandlesWithProvider { candles: items, provider: "tencent_kline".to_string() }),
            Err(tencent_error) => {
                tracing::info!(
                    symbol = %symbol,
                    adjust = %adjust,
                    error = ?tencent_error,
                    "Tencent A-share candles failed, falling back to Eastmoney"
                );
                Ok(CandlesWithProvider {
                    candles: self.fetch_a_share_eastmoney_candles(symbol, adjust, limit)
                        .await
                        .with_context(|| {
                            format!(
                                "failed to fetch A-share candles for {symbol} from Tencent and Eastmoney fallback"
                            )
                        })?,
                    provider: "eastmoney_kline".to_string(),
                })
            }
        }
    }

    pub(crate) async fn fetch_a_share_tencent_candles(
        &self,
        symbol: &str,
        adjust: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CandlePoint>> {
        let market_symbol = self.tencent_market_symbol(symbol)?;
        let fq = match adjust {
            "none" => "",
            "qfq" => "qfq",
            "hfq" => "hfq",
            other => bail!("unsupported adjust mode: {}", other),
        };
        // Tencent kline API rejects requests with limit > 2000 ("param error").
        // Cap to 2000; the API returns all available data up to the limit.
        let capped_limit = limit.clamp(5, 2000);
        let response = self
            .http
            .get("https://web.ifzq.gtimg.cn/appstock/app/fqkline/get")
            .query(&[(
                "param",
                format!("{market_symbol},day,,,{},{}", capped_limit, fq),
            )])
            .send()
            .await
            .context("failed to fetch A-share candles from Tencent")?
            .error_for_status()
            .context("tencent candle request failed")?
            .json::<Value>()
            .await
            .context("failed to decode tencent candle response")?;
        let series = response
            .get("data")
            .and_then(|data| data.get(&market_symbol))
            .and_then(|item| item.get(if fq == "hfq" { "hfqday" } else { "qfqday" }))
            .or_else(|| {
                response
                    .get("data")
                    .and_then(|data| data.get(&market_symbol))
                    .and_then(|item| item.get("day"))
            })
            .and_then(|value| value.as_array())
            .context("tencent candle response missing day series")?;
        let items = series
            .iter()
            .map(Self::parse_tencent_candle_row)
            .collect::<anyhow::Result<Vec<_>>>()?;
        if items.is_empty() {
            bail!("tencent returned no candle items");
        }
        Self::finalize_a_share_candles(items, limit)
    }

    pub(crate) async fn fetch_a_share_eastmoney_candles(
        &self,
        symbol: &str,
        adjust: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<CandlePoint>> {
        let secid = self.eastmoney_secid(symbol)?;
        let fqt = match adjust {
            "none" => "0",
            "qfq" => "1",
            "hfq" => "2",
            other => bail!("unsupported adjust mode: {}", other),
        };
        let response = self
            .http
            .get("https://push2his.eastmoney.com/api/qt/stock/kline/get")
            .query(&[
                ("secid", secid.as_str()),
                ("ut", "fa5fd1943c7b386f172d6893dbfba10b"),
                ("klt", "101"),
                ("fqt", fqt),
                ("lmt", &limit.max(5).to_string()),
                ("end", "20500000"),
                ("fields1", "f1,f2,f3,f4,f5,f6"),
                ("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61"),
            ])
            .send()
            .await
            .context("failed to fetch A-share candles from Eastmoney")?
            .error_for_status()
            .context("eastmoney A-share candle request failed")?;
        let payload: super::super::wire::EastmoneyKlineEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney A-share candle response")?;
        let data = payload
            .data
            .context("eastmoney A-share candle response missing data")?;
        let klines = data
            .klines
            .context("eastmoney A-share candle response missing klines")?;
        let items = klines
            .into_iter()
            .map(|line| Self::parse_eastmoney_a_share_candle_line(&line))
            .collect::<anyhow::Result<Vec<_>>>()?;
        if items.is_empty() {
            bail!("eastmoney A-share candle response returned no rows");
        }
        Self::finalize_a_share_candles(items, limit)
    }

    fn parse_eastmoney_a_share_candle_line(line: &str) -> anyhow::Result<CandlePoint> {
        let fields = line.split(',').collect::<Vec<_>>();
        if fields.len() < 11 {
            bail!("unexpected eastmoney A-share candle format: {}", line);
        }
        Ok(CandlePoint {
            trade_date: fields[0].to_string(),
            open: fields[1]
                .parse()
                .context("invalid eastmoney A-share candle open")?,
            close: fields[2]
                .parse()
                .context("invalid eastmoney A-share candle close")?,
            high: fields[3]
                .parse()
                .context("invalid eastmoney A-share candle high")?,
            low: fields[4]
                .parse()
                .context("invalid eastmoney A-share candle low")?,
            volume: fields[5].parse::<f64>().unwrap_or_default().round() as i64,
            amount: fields[6].parse().unwrap_or_default(),
            amplitude_pct: fields[7].parse().unwrap_or_default(),
            change_pct: fields[8].parse().unwrap_or_default(),
            change_amount: fields[9].parse().unwrap_or_default(),
            turnover_pct: fields[10].parse().unwrap_or_default(),
        })
    }

    fn finalize_a_share_candles(
        mut items: Vec<CandlePoint>,
        limit: usize,
    ) -> anyhow::Result<Vec<CandlePoint>> {
        if items.is_empty() {
            bail!("A-share candle response returned no rows");
        }
        items.sort_by(|left, right| left.trade_date.cmp(&right.trade_date));
        for index in 1..items.len() {
            let previous_close = items[index - 1].close;
            if previous_close > 0.0
                && (items[index].change_amount.abs() <= 1e-10
                    || items[index].change_pct.abs() <= f64::EPSILON)
            {
                items[index].change_amount = items[index].close - previous_close;
                items[index].change_pct = (items[index].change_amount / previous_close * 100.0)
                    .to_f64()
                    .unwrap_or_default();
            }
        }
        if items.len() > limit {
            let start_index = items.len() - limit;
            items = items[start_index..].to_vec();
        }
        Ok(items)
    }

    pub(super) fn parse_tencent_candle_row(value: &Value) -> anyhow::Result<CandlePoint> {
        let row = value.as_array().context("unexpected tencent candle row")?;
        if row.len() < 6 {
            bail!("unexpected tencent candle row length");
        }
        let trade_date = row[0]
            .as_str()
            .context("missing tencent candle date")?
            .to_string();
        let open = row[1]
            .as_str()
            .context("missing tencent candle open")?
            .parse()
            .context("invalid tencent candle open")?;
        let close = row[2]
            .as_str()
            .context("missing tencent candle close")?
            .parse()
            .context("invalid tencent candle close")?;
        let high = row[3]
            .as_str()
            .context("missing tencent candle high")?
            .parse()
            .context("invalid tencent candle high")?;
        let low = row[4]
            .as_str()
            .context("missing tencent candle low")?
            .parse()
            .context("invalid tencent candle low")?;
        let volume = row[5]
            .as_str()
            .context("missing tencent candle volume")?
            .parse::<f64>()
            .context("invalid tencent candle volume")?;
        Ok(CandlePoint {
            trade_date,
            open,
            close,
            high,
            low,
            volume: volume.round() as i64,
            amount: 0.0,
            amplitude_pct: if low > 0.0 {
                ((high - low) / low * 100.0).to_f64().unwrap_or_default()
            } else {
                0.0
            },
            change_pct: 0.0,
            change_amount: 0.0,
            turnover_pct: 0.0,
        })
    }
}
