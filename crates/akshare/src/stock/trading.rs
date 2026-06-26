//! Stock trading data — billboard (龙虎榜), margin trading, block trades.
//!
//! Covers Python functions:
//! - `stock_billboard_details_em` — Billboard (龙虎榜) details
//! - `stock_billboard_statistic_em` — Billboard statistics
//! - `stock_billboard_org_statistic_em` — Organization trading statistics
//! - `stock_billboard_org_detail_em` — Organization seat tracking
//! - `stock_margin_detail_sse` — Margin trading details (SSE)
//! - `stock_margin_detail_szse` — Margin trading details (SZSE)
//! - `stock_margin_underlying_info_szse` — Margin trading underlying info
//! - `stock_dzjy_mdetail` — Block trade daily details
//! - `stock_dzjy_detail_cls` — Block trade daily statistics
//! - `stock_dzjy_hygt` — Active A-share block trade statistics

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::wire::EmDatacenterResp;

// ---------------------------------------------------------------------------
// Helper: format YYYYMMDD -> YYYY-MM-DD
// ---------------------------------------------------------------------------

fn fmt_date(date: &str) -> String {
    if date.len() >= 8 {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        date.to_string()
    }
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl AkShareClient {
    /// Get billboard (龙虎榜) details from Eastmoney.
    ///
    /// Python equivalent: `stock_billboard_details_em(date)`
    pub async fn stock_billboard_details_em(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let date_fmt = fmt_date(date);
        let filter = format!("(TRADE_DATE='{date_fmt}')");

        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("sortColumns", "SECURITY_CODE"),
                    ("sortTypes", "1"),
                    ("pageSize", "500"),
                    ("pageNumber", "1"),
                    ("reportName", "RPT_DAILYBILLBOARD_DETAILSNEW"),
                    ("columns", "ALL"),
                    ("filter", filter.as_str()),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: EmDatacenterResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .ok_or_else(|| Error::upstream("billboard details missing result"))?
            .data;

        if data.is_empty() {
            return Err(Error::not_found("billboard details returned no data"));
        }
        Ok(data)
    }

    /// Get billboard statistics from Eastmoney.
    ///
    /// Python equivalent: `stock_billboard_statistic_em(period)`
    ///
    /// `period` is one of: "daily", "5days", "10days", "month", "year".
    pub async fn stock_billboard_statistic_em(
        &self,
        period: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let period_code = match period {
            "daily" => "daily",
            "5days" => "5days",
            "10days" => "10days",
            "month" => "month",
            "year" => "year",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported period: {period}"
                )));
            }
        };

        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("sortColumns", "SECURITY_CODE"),
                    ("sortTypes", "1"),
                    ("pageSize", "500"),
                    ("pageNumber", "1"),
                    ("reportName", "RPT_DAILYBILLBOARD_STATISTIC"),
                    ("columns", "ALL"),
                    ("filter", format!("(PERIOD=\"{period_code}\")").as_str()),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: EmDatacenterResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .ok_or_else(|| Error::upstream("billboard statistic missing result"))?
            .data;

        if data.is_empty() {
            return Err(Error::not_found("billboard statistic returned no data"));
        }
        Ok(data)
    }

    /// Get organization (institutional) trading statistics from Eastmoney.
    ///
    /// Python equivalent: `stock_billboard_org_statistic_em(period)`
    ///
    /// `period` is one of: "daily", "5days", "10days", "month", "year".
    pub async fn stock_billboard_org_statistic_em(
        &self,
        period: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let period_code = match period {
            "daily" => "daily",
            "5days" => "5days",
            "10days" => "10days",
            "month" => "month",
            "year" => "year",
            _ => {
                return Err(Error::invalid_input(format!(
                    "unsupported period: {period}"
                )));
            }
        };

        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("sortColumns", "SECURITY_CODE"),
                    ("sortTypes", "1"),
                    ("pageSize", "500"),
                    ("pageNumber", "1"),
                    ("reportName", "RPT_DAILYBILLBOARD_ORG_STATISTIC"),
                    ("columns", "ALL"),
                    ("filter", format!("(PERIOD=\"{period_code}\")").as_str()),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: EmDatacenterResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .ok_or_else(|| Error::upstream("billboard org statistic missing result"))?
            .data;

        if data.is_empty() {
            return Err(Error::not_found("billboard org statistic returned no data"));
        }
        Ok(data)
    }

    /// Get organization (seat) tracking details from Eastmoney.
    ///
    /// Python equivalent: `stock_billboard_org_detail_em(trade_date, symbol)`
    ///
    /// `symbol` is the institution/organization name to track.
    pub async fn stock_billboard_org_detail_em(
        &self,
        trade_date: &str,
        symbol: &str,
    ) -> Result<Vec<serde_json::Value>> {
        let date_fmt = fmt_date(trade_date);
        let filter =
            format!("(TRADE_DATE>='{date_fmt}')(TRADE_DATE<='{date_fmt}')(ORG_NAME=\"{symbol}\")");

        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("sortColumns", "SECURITY_CODE"),
                    ("sortTypes", "1"),
                    ("pageSize", "500"),
                    ("pageNumber", "1"),
                    ("reportName", "RPT_DAILYBILLBOARD_ORGDETAIL"),
                    ("columns", "ALL"),
                    ("filter", filter.as_str()),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: EmDatacenterResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .ok_or_else(|| Error::upstream("billboard org detail missing result"))?
            .data;

        if data.is_empty() {
            return Err(Error::not_found("billboard org detail returned no data"));
        }
        Ok(data)
    }

    /// Get margin trading details from SSE (Shanghai Stock Exchange) via Eastmoney.
    ///
    /// Python equivalent: `stock_margin_detail_sse(date)`
    ///
    /// This is a raw JSON variant that fetches from the Eastmoney datacenter.
    /// For typed SSE margin data, use the `stock_margin_detail_sse` in `feature::margin_em`.
    pub async fn stock_margin_detail_sse_dc(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let date_fmt = fmt_date(date);

        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("sortColumns", "SECURITY_CODE"),
                    ("sortTypes", "1"),
                    ("pageSize", "500"),
                    ("pageNumber", "1"),
                    ("reportName", "RPTA_WEB_RZRQ_GGMX"),
                    ("columns", "ALL"),
                    ("filter", format!("(TRADE_DATE='{date_fmt}')").as_str()),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: EmDatacenterResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .ok_or_else(|| Error::upstream("SSE margin detail missing result"))?
            .data;

        if data.is_empty() {
            return Err(Error::not_found("SSE margin detail returned no data"));
        }
        Ok(data)
    }

    /// Get margin trading details from SZSE (Shenzhen Stock Exchange) via Eastmoney.
    ///
    /// Python equivalent: `stock_margin_detail_szse(date)`
    ///
    /// This is a raw JSON variant that fetches from the Eastmoney datacenter.
    /// For typed SZSE margin data, use the `stock_margin_detail_szse` in `feature::margin_em`.
    pub async fn stock_margin_detail_szse_dc(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let date_fmt = fmt_date(date);

        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("sortColumns", "SECURITY_CODE"),
                    ("sortTypes", "1"),
                    ("pageSize", "500"),
                    ("pageNumber", "1"),
                    ("reportName", "RPTA_WEB_RZRQ_GGMX"),
                    ("columns", "ALL"),
                    ("filter", format!("(TRADE_DATE='{date_fmt}')").as_str()),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: EmDatacenterResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .ok_or_else(|| Error::upstream("SZSE margin detail missing result"))?
            .data;

        if data.is_empty() {
            return Err(Error::not_found("SZSE margin detail returned no data"));
        }
        Ok(data)
    }

    /// Get margin trading underlying info from SZSE via Eastmoney.
    ///
    /// Python equivalent: `stock_margin_underlying_info_szse()`
    ///
    /// This is a raw JSON variant from the Eastmoney datacenter.
    pub async fn stock_margin_underlying_info_szse_dc(&self) -> Result<Vec<serde_json::Value>> {
        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("sortColumns", "SECURITY_CODE"),
                    ("sortTypes", "1"),
                    ("pageSize", "500"),
                    ("pageNumber", "1"),
                    ("reportName", "RPTA_WEB_RZRQ_MLGS"),
                    ("columns", "ALL"),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: EmDatacenterResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .ok_or_else(|| Error::upstream("margin underlying info missing result"))?
            .data;

        if data.is_empty() {
            return Err(Error::not_found("margin underlying info returned no data"));
        }
        Ok(data)
    }

    /// Get block trade daily details from Eastmoney.
    ///
    /// Python equivalent: `stock_dzjy_mdetail(date)`
    pub async fn stock_dzjy_mdetail(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let date_fmt = fmt_date(date);
        let filter = format!("(TRADE_DATE='{date_fmt}')");

        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("sortColumns", "SECURITY_CODE"),
                    ("sortTypes", "1"),
                    ("pageSize", "500"),
                    ("pageNumber", "1"),
                    ("reportName", "RPT_DATA_BLOCKTRADE"),
                    ("columns", "ALL"),
                    ("filter", filter.as_str()),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: EmDatacenterResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .ok_or_else(|| Error::upstream("block trade daily detail missing result"))?
            .data;

        if data.is_empty() {
            return Err(Error::not_found(
                "block trade daily detail returned no data",
            ));
        }
        Ok(data)
    }

    /// Get block trade daily statistics from Eastmoney.
    ///
    /// Python equivalent: `stock_dzjy_detail_cls(date)`
    pub async fn stock_dzjy_detail_cls(&self, date: &str) -> Result<Vec<serde_json::Value>> {
        let date_fmt = fmt_date(date);
        let filter = format!("(TRADE_DATE='{date_fmt}')");

        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("sortColumns", "SECURITY_CODE"),
                    ("sortTypes", "1"),
                    ("pageSize", "500"),
                    ("pageNumber", "1"),
                    ("reportName", "RPT_BLOCKTRADE_DETAILSNEW"),
                    ("columns", "ALL"),
                    ("filter", filter.as_str()),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: EmDatacenterResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .ok_or_else(|| Error::upstream("block trade daily statistics missing result"))?
            .data;

        if data.is_empty() {
            return Err(Error::not_found(
                "block trade daily statistics returned no data",
            ));
        }
        Ok(data)
    }

    /// Get active A-share block trade statistics from Eastmoney.
    ///
    /// Python equivalent: `stock_dzjy_hygt()`
    pub async fn stock_dzjy_hygt(&self) -> Result<Vec<serde_json::Value>> {
        let response = crate::util::send_and_check(
            self.get("https://datacenter-web.eastmoney.com/api/data/v1/get")
                .query(&[
                    ("sortColumns", "SECURITY_CODE"),
                    ("sortTypes", "1"),
                    ("pageSize", "500"),
                    ("pageNumber", "1"),
                    ("reportName", "RPT_DZRZ_HY_GT"),
                    ("columns", "ALL"),
                    ("source", "WEB"),
                    ("client", "WEB"),
                ]),
        )
        .await?;

        let payload: EmDatacenterResp = response.json().await.map_err(Error::from)?;
        let data = payload
            .result
            .ok_or_else(|| Error::upstream("A-share block trade statistics missing result"))?
            .data;

        if data.is_empty() {
            return Err(Error::not_found(
                "A-share block trade statistics returned no data",
            ));
        }
        Ok(data)
    }
}
