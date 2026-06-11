//! Limit-up/down pools (涨停/跌停股池) from Eastmoney.

use super::helpers::*;
use super::types::*;
use crate::client::AkShareClient;
use crate::error::Result;

impl AkShareClient {
    /// 涨停股池
    pub async fn stock_zt_pool_em(&self, date: &str) -> Result<Vec<ZtPool>> {
        let date_fmt = date.to_string();
        let data = self.push2ex_fetch("gettopicZTPool", &[
            ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
            ("dpt", "wz.ztzt"),
            ("Ession", &date_fmt),
            ("sort", "fbt:asc"),
            ("fields", "f1,f2,f3,f4,f6,f8,f10,f12,f13,f14,f15,f17,f18,f20,f152,f221,f222,f223,f224,f225,f226,f227,f228,f229,f230,f231,f232,f233,f234,f235"),
            ("pn", "1"),
            ("pz", "5000"),
        ]).await?;

        let diff = data
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(diff
            .iter()
            .map(|v| ZtPool {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                change_pct: json_f64(v, "f3"),
                latest_price: json_f64(v, "f2"),
                amount: json_f64(v, "f6"),
                circulating_market_cap: json_f64(v, "f20"),
                total_market_cap: json_f64(v, "f21"),
                turnover_rate: json_f64(v, "f8"),
                seal_amount: json_f64(v, "f222"),
                first_seal_time: json_str(v, "f223"),
                last_seal_time: json_str(v, "f224"),
                break_count: json_i64(v, "f225"),
                zt_statistics: json_str(v, "f233"),
                consecutive_count: json_i64(v, "f226"),
                industry: json_str(v, "f104"),
            })
            .collect())
    }

    /// 跌停股池
    pub async fn stock_zt_pool_dtgc_em(&self, date: &str) -> Result<Vec<ZtPoolDtgc>> {
        let date_fmt = date.to_string();
        let data = self.push2ex_fetch("gettopicDTPool", &[
            ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
            ("dpt", "wz.ztzt"),
            ("Ession", &date_fmt),
            ("sort", "fbt:asc"),
            ("fields", "f1,f2,f3,f4,f6,f8,f10,f12,f13,f14,f15,f17,f18,f20,f152,f221,f222,f223,f224,f225,f226,f227,f228,f229,f230,f231,f232,f233,f234,f235"),
            ("pn", "1"),
            ("pz", "5000"),
        ]).await?;

        let diff = data
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(diff
            .iter()
            .map(|v| ZtPoolDtgc {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                change_pct: json_f64(v, "f3"),
                latest_price: json_f64(v, "f2"),
                amount: json_f64(v, "f6"),
                circulating_market_cap: json_f64(v, "f20"),
                total_market_cap: json_f64(v, "f21"),
                turnover_rate: json_f64(v, "f8"),
                seal_amount: json_f64(v, "f222"),
                first_seal_time: json_str(v, "f223"),
                last_seal_time: json_str(v, "f224"),
                open_count: json_i64(v, "f225"),
                industry: json_str(v, "f104"),
            })
            .collect())
    }

    /// 昨日涨停股池
    pub async fn stock_zt_pool_previous_em(&self, date: &str) -> Result<Vec<ZtPoolPrevious>> {
        let date_fmt = date.to_string();
        let data = self.push2ex_fetch("gettopicZTPool", &[
            ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
            ("dpt", "wz.ztzt"),
            ("Ession", &date_fmt),
            ("sort", "fbt:asc"),
            ("fields", "f1,f2,f3,f4,f6,f8,f10,f12,f13,f14,f15,f17,f18,f20,f152,f221,f222,f223,f224,f225,f226,f227,f228,f229,f230,f231,f232,f233,f234,f235"),
            ("pn", "1"),
            ("pz", "5000"),
            ("tp", "1"),
        ]).await?;

        let diff = data
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(diff
            .iter()
            .map(|v| ZtPoolPrevious {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                change_pct: json_f64(v, "f3"),
                latest_price: json_f64(v, "f2"),
                limit_price: json_f64(v, "f221"),
                amount: json_f64(v, "f6"),
                circulating_market_cap: json_f64(v, "f20"),
                total_market_cap: json_f64(v, "f21"),
                turnover_rate: json_f64(v, "f8"),
                speed: json_f64(v, "f227"),
                amplitude: json_f64(v, "f228"),
                prev_seal_time: json_str(v, "f223"),
                prev_consecutive_count: json_i64(v, "f226"),
                zt_statistics: json_str(v, "f233"),
                industry: json_str(v, "f104"),
            })
            .collect())
    }

    /// 强势股池
    pub async fn stock_zt_pool_strong_em(&self, date: &str) -> Result<Vec<ZtPoolStrong>> {
        let date_fmt = date.to_string();
        let data = self.push2ex_fetch("gettopicQSPool", &[
            ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
            ("dpt", "wz.ztzt"),
            ("Ession", &date_fmt),
            ("sort", "fbt:asc"),
            ("fields", "f1,f2,f3,f4,f6,f8,f10,f12,f13,f14,f15,f17,f18,f20,f152,f221,f222,f223,f224,f225,f226,f227,f228,f229,f230,f231,f232,f233,f234,f235"),
            ("pn", "1"),
            ("pz", "5000"),
        ]).await?;

        let diff = data
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(diff
            .iter()
            .map(|v| ZtPoolStrong {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                change_pct: json_f64(v, "f3"),
                latest_price: json_f64(v, "f2"),
                amount: json_f64(v, "f6"),
                circulating_market_cap: json_f64(v, "f20"),
                total_market_cap: json_f64(v, "f21"),
                turnover_rate: json_f64(v, "f8"),
                seal_amount: json_f64(v, "f222"),
                first_seal_time: json_str(v, "f223"),
                last_seal_time: json_str(v, "f224"),
                break_count: json_i64(v, "f225"),
                zt_statistics: json_str(v, "f233"),
                consecutive_count: json_i64(v, "f226"),
                industry: json_str(v, "f104"),
            })
            .collect())
    }

    /// 次新股池
    pub async fn stock_zt_pool_sub_new_em(&self, date: &str) -> Result<Vec<ZtPoolSubNew>> {
        let date_fmt = date.to_string();
        let data = self.push2ex_fetch("gettopicCXPool", &[
            ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
            ("dpt", "wz.ztzt"),
            ("Ession", &date_fmt),
            ("sort", "fbt:asc"),
            ("fields", "f1,f2,f3,f4,f6,f8,f10,f12,f13,f14,f15,f17,f18,f20,f152,f221,f222,f223,f224,f225,f226,f227,f228,f229,f230,f231,f232,f233,f234,f235"),
            ("pn", "1"),
            ("pz", "5000"),
        ]).await?;

        let diff = data
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(diff
            .iter()
            .map(|v| ZtPoolSubNew {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                change_pct: json_f64(v, "f3"),
                latest_price: json_f64(v, "f2"),
                amount: json_f64(v, "f6"),
                circulating_market_cap: json_f64(v, "f20"),
                total_market_cap: json_f64(v, "f21"),
                turnover_rate: json_f64(v, "f8"),
                ipo_date: json_str(v, "f229"),
                industry: json_str(v, "f104"),
            })
            .collect())
    }

    /// 炸板股池
    pub async fn stock_zt_pool_zbgc_em(&self, date: &str) -> Result<Vec<ZtPoolZbgc>> {
        let date_fmt = date.to_string();
        let data = self.push2ex_fetch("gettopicZBPool", &[
            ("ut", "7eea3edcaed734bea9cbfc24409ed989"),
            ("dpt", "wz.ztzt"),
            ("Ession", &date_fmt),
            ("sort", "fbt:asc"),
            ("fields", "f1,f2,f3,f4,f6,f8,f10,f12,f13,f14,f15,f17,f18,f20,f152,f221,f222,f223,f224,f225,f226,f227,f228,f229,f230,f231,f232,f233,f234,f235"),
            ("pn", "1"),
            ("pz", "5000"),
        ]).await?;

        let diff = data
            .get("data")
            .and_then(|d| d.get("diff"))
            .and_then(|d| d.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(diff
            .iter()
            .map(|v| ZtPoolZbgc {
                code: json_str(v, "f12"),
                name: json_str(v, "f14"),
                change_pct: json_f64(v, "f3"),
                latest_price: json_f64(v, "f2"),
                amount: json_f64(v, "f6"),
                circulating_market_cap: json_f64(v, "f20"),
                total_market_cap: json_f64(v, "f21"),
                turnover_rate: json_f64(v, "f8"),
                seal_amount: json_f64(v, "f222"),
                first_seal_time: json_str(v, "f223"),
                last_seal_time: json_str(v, "f224"),
                break_count: json_i64(v, "f225"),
                zt_statistics: json_str(v, "f233"),
                industry: json_str(v, "f104"),
            })
            .collect())
    }
}
