use anyhow::{bail, Context};

use crate::types::{AnnouncementDetail, AnnouncementItem, BillboardEntry, BillboardSeatDetail};
use super::super::MarketDataClient;
use super::super::wire::{
    EastmoneyAnnouncementContentEnvelope, EastmoneyAnnouncementsEnvelope,
    EastmoneyBillboardEntryItem, EastmoneyBillboardSeatItem, EastmoneyDatacenterEnvelope,
};

impl MarketDataClient {
    pub(crate) async fn fetch_a_share_announcement_detail(
        &self,
        art_code: &str,
    ) -> anyhow::Result<AnnouncementDetail> {
        let response = self
            .http
            .get("https://np-cnotice-stock.eastmoney.com/api/content/ann")
            .query(&[
                ("art_code", art_code),
                ("client_source", "web"),
                ("page_index", "1"),
            ])
            .send()
            .await
            .context("failed to fetch Eastmoney announcement detail")?
            .error_for_status()
            .context("eastmoney announcement detail request failed")?;
        let payload: EastmoneyAnnouncementContentEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney announcement detail response")?;
        let data = payload
            .data
            .context("eastmoney announcement detail missing data")?;

        Ok(AnnouncementDetail {
            art_code: data.art_code.unwrap_or_else(|| art_code.to_string()),
            title: data.notice_title.unwrap_or_else(|| "公司公告".to_string()),
            published_at: data.notice_date.unwrap_or_default(),
            content: data.notice_content.unwrap_or_default(),
            pdf_url: data.attach_url,
            source: "Eastmoney 公告".to_string(),
        })
    }

    pub(crate) async fn fetch_a_share_announcements(
        &self,
        ts_code: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<AnnouncementItem>> {
        let symbol = ts_code
            .split_once('.')
            .map(|(code, _)| code)
            .unwrap_or(ts_code);
        let page_size = limit.clamp(1, 100).to_string();
        let response = self
            .http
            .get("https://np-anotice-stock.eastmoney.com/api/security/ann")
            .query(&[
                ("page_size", page_size.as_str()),
                ("page_index", "1"),
                ("ann_type", "A"),
                ("client_source", "web"),
                ("stock_list", symbol),
            ])
            .send()
            .await
            .context("failed to fetch Eastmoney announcements")?
            .error_for_status()
            .context("eastmoney announcements request failed")?;
        let payload: EastmoneyAnnouncementsEnvelope = response
            .json()
            .await
            .context("failed to decode eastmoney announcements response")?;
        let mut items = payload
            .data
            .and_then(|data| data.list)
            .unwrap_or_default()
            .into_iter()
            .map(|item| {
                let art_code = item.art_code.unwrap_or_default();
                AnnouncementItem {
                    url: (!art_code.is_empty()).then(|| {
                        format!(
                            "https://data.eastmoney.com/notices/detail/{symbol}/{art_code}.html"
                        )
                    }),
                    art_code,
                    symbol: symbol.to_string(),
                    title: item.title.unwrap_or_else(|| "公司公告".to_string()),
                    published_at: item.notice_date.unwrap_or_default(),
                    source: "Eastmoney 公告".to_string(),
                }
            })
            .collect::<Vec<_>>();
        if items.is_empty() {
            bail!("eastmoney returned no announcement items");
        }
        items.truncate(limit);
        Ok(items)
    }

    pub(crate) async fn fetch_a_share_billboard_entries(
        &self,
        symbol: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<BillboardEntry>> {
        let symbol = symbol
            .trim()
            .trim_end_matches(".SH")
            .trim_end_matches(".SZ");
        let response = self
            .http
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", "RPT_DAILYBILLBOARD_DETAILSNEW"),
                ("columns", "ALL"),
                ("filter", &format!("(SECURITY_CODE=\"{symbol}\")")),
                ("pageNumber", "1"),
                ("pageSize", &limit.to_string()),
                ("sortTypes", "-1"),
                ("sortColumns", "TRADE_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .context("failed to fetch Eastmoney billboard entries")?
            .error_for_status()
            .context("eastmoney billboard entries request failed")?;
        let payload: EastmoneyDatacenterEnvelope<EastmoneyBillboardEntryItem> = response
            .json()
            .await
            .context("failed to decode eastmoney billboard entries response")?;
        let items = payload
            .result
            .and_then(|result| result.data)
            .unwrap_or_default()
            .into_iter()
            .map(|item| BillboardEntry {
                trade_date: item.trade_date.unwrap_or_default(),
                symbol: item.security_code.unwrap_or_else(|| symbol.to_string()),
                name: item.security_name.unwrap_or_else(|| "未知股票".to_string()),
                close_price: item.close_price.unwrap_or_default(),
                change_rate_pct: item.change_rate.unwrap_or_default(),
                turnover_rate_pct: item.turnover_rate,
                net_amount: item.net_amount,
                buy_amount: item.buy_amount,
                sell_amount: item.sell_amount,
                explanation: item.explanation,
                reason: item.explain,
            })
            .collect::<Vec<_>>();
        if items.is_empty() {
            bail!("eastmoney returned no billboard entries");
        }
        Ok(items)
    }

    pub(crate) async fn fetch_a_share_billboard_seats(
        &self,
        symbol: &str,
        side: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<BillboardSeatDetail>> {
        let report_name = match side {
            "buy" => "RPT_BILLBOARD_DAILYDETAILSBUY",
            "sell" => "RPT_BILLBOARD_DAILYDETAILSSELL",
            other => bail!("unsupported billboard side: {}", other),
        };
        let symbol = symbol
            .trim()
            .trim_end_matches(".SH")
            .trim_end_matches(".SZ");
        let response = self
            .http
            .get("https://datacenter-web.eastmoney.com/api/data/v1/get")
            .query(&[
                ("reportName", report_name),
                ("columns", "ALL"),
                ("filter", &format!("(SECURITY_CODE=\"{symbol}\")")),
                ("pageNumber", "1"),
                ("pageSize", &limit.to_string()),
                ("sortTypes", "-1"),
                ("sortColumns", "TRADE_DATE"),
                ("source", "WEB"),
                ("client", "WEB"),
            ])
            .send()
            .await
            .context("failed to fetch Eastmoney billboard seats")?
            .error_for_status()
            .context("eastmoney billboard seats request failed")?;
        let payload: EastmoneyDatacenterEnvelope<EastmoneyBillboardSeatItem> = response
            .json()
            .await
            .context("failed to decode eastmoney billboard seats response")?;
        let items = payload
            .result
            .and_then(|result| result.data)
            .unwrap_or_default()
            .into_iter()
            .map(|item| BillboardSeatDetail {
                trade_date: item.trade_date.unwrap_or_default(),
                symbol: item.security_code.unwrap_or_else(|| symbol.to_string()),
                department_name: item
                    .department_name
                    .unwrap_or_else(|| "未知席位".to_string()),
                buy_amount: item.buy_amount,
                sell_amount: item.sell_amount,
                net_amount: item.net_amount,
                explanation: item.explanation,
            })
            .collect::<Vec<_>>();
        if items.is_empty() {
            bail!("eastmoney returned no billboard seat items");
        }
        Ok(items)
    }
}
