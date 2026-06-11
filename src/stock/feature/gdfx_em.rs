//! Shareholder analysis (股东分析) from Eastmoney.

use crate::client::AkShareClient;
use crate::error::Result;
use super::helpers::*;
use super::types::*;

impl AkShareClient {
    /// 股东持股统计-十大流通股东
    pub async fn stock_gdfx_free_holding_statistics_em(&self, date: &str) -> Result<Vec<GdfxHoldingStatistic>> {
        let filter = format!("(HOLDNUM_CHANGE_TYPE=\"001\")(END_DATE='{}')", fmt_date(date));
        let data = self.dc_fetch_all("RPT_COOPFREEHOLDERS_ANALYSIS", "ALL", &filter,
            "STATISTICS_TIMES,COOPERATION_HOLDER_MARK", "-1,-1", 500, 10, &[]).await?;
        Ok(data.iter().map(|v| GdfxHoldingStatistic {
            holder_name: json_str(v, "HOLDER_NAME"),
            holder_type: json_str_opt(v, "HOLDER_TYPE"),
            statistics_count: json_i64_opt(v, "STATISTICS_TIMES"),
            avg_change_10d: json_f64_opt(v, "HOLD_10_DAYS_AVG_CHR"),
            max_change_10d: json_f64_opt(v, "HOLD_10_DAYS_MAX_CHR"),
            min_change_10d: json_f64_opt(v, "HOLD_10_DAYS_MIN_CHR"),
            avg_change_30d: json_f64_opt(v, "HOLD_30_DAYS_AVG_CHR"),
            max_change_30d: json_f64_opt(v, "HOLD_30_DAYS_MAX_CHR"),
            min_change_30d: json_f64_opt(v, "HOLD_30_DAYS_MIN_CHR"),
            avg_change_60d: json_f64_opt(v, "HOLD_60_DAYS_AVG_CHR"),
            max_change_60d: json_f64_opt(v, "HOLD_60_DAYS_MAX_CHR"),
            min_change_60d: json_f64_opt(v, "HOLD_60_DAYS_MIN_CHR"),
            holding_stocks: json_str_opt(v, "HOLD_SECURITIES"),
        }).collect())
    }

    /// 股东持股统计-十大股东
    pub async fn stock_gdfx_holding_statistics_em(&self, date: &str) -> Result<Vec<GdfxHoldingStatistic>> {
        let filter = format!("(HOLDNUM_CHANGE_TYPE=\"001\")(END_DATE='{}')", fmt_date(date));
        let data = self.dc_fetch_all("RPT_COOPHOLDERS_ANALYSIS", "ALL", &filter,
            "STATISTICS_TIMES,COOPERATION_HOLDER_MARK", "-1,-1", 500, 10, &[]).await?;
        Ok(data.iter().map(|v| GdfxHoldingStatistic {
            holder_name: json_str(v, "HOLDER_NAME"),
            holder_type: json_str_opt(v, "HOLDER_TYPE"),
            statistics_count: json_i64_opt(v, "STATISTICS_TIMES"),
            avg_change_10d: json_f64_opt(v, "HOLD_10_DAYS_AVG_CHR"),
            max_change_10d: json_f64_opt(v, "HOLD_10_DAYS_MAX_CHR"),
            min_change_10d: json_f64_opt(v, "HOLD_10_DAYS_MIN_CHR"),
            avg_change_30d: json_f64_opt(v, "HOLD_30_DAYS_AVG_CHR"),
            max_change_30d: json_f64_opt(v, "HOLD_30_DAYS_MAX_CHR"),
            min_change_30d: json_f64_opt(v, "HOLD_30_DAYS_MIN_CHR"),
            avg_change_60d: json_f64_opt(v, "HOLD_60_DAYS_AVG_CHR"),
            max_change_60d: json_f64_opt(v, "HOLD_60_DAYS_MAX_CHR"),
            min_change_60d: json_f64_opt(v, "HOLD_60_DAYS_MIN_CHR"),
            holding_stocks: json_str_opt(v, "HOLD_SECURITIES"),
        }).collect())
    }

    /// 股东持股变动统计-十大流通股东
    pub async fn stock_gdfx_free_holding_change_em(&self, date: &str) -> Result<Vec<GdfxHoldingChange>> {
        let filter = format!("(END_DATE='{}')", fmt_date(date));
        let data = self.dc_fetch_all("RPT_FREEHOLDERS_BASIC_INFO", "ALL", &filter,
            "HOLDER_NUM,HOLDER_NEW", "-1,-1", 500, 10, &[]).await?;
        Ok(data.iter().map(|v| GdfxHoldingChange {
            holder_name: json_str(v, "HOLDER_NAME"),
            holder_type: json_str_opt(v, "HOLDER_TYPE"),
            total_holdings: json_i64(v, "HOLDER_NUM"),
            new_positions: json_i64(v, "HOLDER_NEW"),
            increased: json_i64(v, "HOLDER_INCREASE"),
            unchanged: json_i64(v, "HOLDER_UNCHANGED"),
            decreased: json_i64(v, "HOLDER_DECREASE"),
            circulating_market_cap: json_f64_opt(v, "FREE_MARKET_CAP"),
            holding_stocks: json_str_opt(v, "HOLD_SECURITIES"),
        }).collect())
    }

    /// 股东持股变动统计-十大股东
    pub async fn stock_gdfx_holding_change_em(&self, date: &str) -> Result<Vec<GdfxHoldingChange>> {
        let filter = format!("(END_DATE='{}')", fmt_date(date));
        let data = self.dc_fetch_all("RPT_HOLDERS_BASIC_INFO", "ALL", &filter,
            "HOLDER_NUM,HOLDER_NEW", "-1,-1", 500, 10, &[]).await?;
        Ok(data.iter().map(|v| GdfxHoldingChange {
            holder_name: json_str(v, "HOLDER_NAME"),
            holder_type: json_str_opt(v, "HOLDER_TYPE"),
            total_holdings: json_i64(v, "HOLDER_NUM"),
            new_positions: json_i64(v, "HOLDER_NEW"),
            increased: json_i64(v, "HOLDER_INCREASE"),
            unchanged: json_i64(v, "HOLDER_UNCHANGED"),
            decreased: json_i64(v, "HOLDER_DECREASE"),
            circulating_market_cap: json_f64_opt(v, "FREE_MARKET_CAP"),
            holding_stocks: json_str_opt(v, "HOLD_SECURITIES"),
        }).collect())
    }

    /// 个股-十大流通股东
    pub async fn stock_gdfx_free_top_10_em(&self, symbol: &str, date: &str) -> Result<Vec<GdfxTop10>> {
        let url = "https://emweb.securities.eastmoney.com/PC_HSF10/ShareholderResearch/PageSDLTGD";
        let resp = self.get(url)
            .query(&[("code", symbol.to_uppercase().as_str()), ("date", fmt_date(date).as_str())])
            .send().await.map_err(crate::error::Error::from)?
            .error_for_status().map_err(crate::error::Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(crate::error::Error::from)?;
        let arr = json.get("sdltgd").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr.iter().enumerate().map(|(i, v)| GdfxTop10 {
            rank: (i + 1) as i64,
            holder_name: json_str(v, "HOLDER_NAME"),
            holder_nature: json_str_opt(v, "HOLDER_NATURE"),
            share_type: json_str_opt(v, "SHARES_TYPE"),
            holding_count: json_f64(v, "HOLD_NUM"),
            holding_ratio: json_f64(v, "FREE_HOLDNUM_RATIO"),
            change: json_str_opt(v, "XZCHANGE"),
            change_ratio: json_f64_opt(v, "CHANGE_RATIO"),
        }).collect())
    }

    /// 个股-十大股东
    pub async fn stock_gdfx_top_10_em(&self, symbol: &str, date: &str) -> Result<Vec<GdfxTop10>> {
        let url = "https://emweb.securities.eastmoney.com/PC_HSF10/ShareholderResearch/PageSDGD";
        let resp = self.get(url)
            .query(&[("code", symbol.to_uppercase().as_str()), ("date", fmt_date(date).as_str())])
            .send().await.map_err(crate::error::Error::from)?
            .error_for_status().map_err(crate::error::Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(crate::error::Error::from)?;
        let arr = json.get("sdgd").and_then(|d| d.as_array()).cloned().unwrap_or_default();
        Ok(arr.iter().enumerate().map(|(i, v)| GdfxTop10 {
            rank: (i + 1) as i64,
            holder_name: json_str(v, "HOLDER_NAME"),
            holder_nature: None,
            share_type: json_str_opt(v, "SHARES_TYPE"),
            holding_count: json_f64(v, "HOLD_NUM"),
            holding_ratio: json_f64(v, "HOLD_RATIO"),
            change: json_str_opt(v, "XZCHANGE"),
            change_ratio: json_f64_opt(v, "CHANGE_RATIO"),
        }).collect())
    }

    /// 股东持股明细-十大流通股东
    pub async fn stock_gdfx_free_holding_detail_em(&self, date: &str) -> Result<Vec<GdfxHoldingDetail>> {
        let filter = format!("(END_DATE='{}')", fmt_date(date));
        let data = self.dc_fetch_all("RPT_F10_EH_FREEHOLDERS", "ALL", &filter,
            "UPDATE_DATE,SECURITY_CODE,HOLDER_RANK", "-1,1,1", 500, 10, &[]).await?;
        Ok(data.iter().map(|v| GdfxHoldingDetail {
            holder_name: json_str(v, "HOLDER_NAME"),
            holder_type: json_str_opt(v, "HOLDER_TYPE"),
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            report_date: json_str(v, "END_DATE"),
            holding_count: json_f64(v, "HOLD_NUM"),
            holding_change: json_f64_opt(v, "XZCHANGE"),
            holding_change_ratio: json_f64_opt(v, "CHANGE_RATIO"),
            holding_change_name: json_str_opt(v, "HOLDNUM_CHANGE_NAME"),
            circulating_market_cap: json_f64_opt(v, "HOLDER_MARKET_CAP"),
            notice_date: json_str(v, "UPDATE_DATE"),
        }).collect())
    }

    /// 股东持股明细-十大股东
    pub async fn stock_gdfx_holding_detail_em(
        &self, date: &str, indicator: &str, symbol: &str,
    ) -> Result<Vec<GdfxHoldingDetail>> {
        let filter = format!("(END_DATE='{}')(HOLDNUM_CHANGE_TYPE=\"{}\")", fmt_date(date), symbol);
        let report = match indicator {
            "个人" => "RPT_F10_EH_HOLDERS",
            _ => "RPT_F10_EH_HOLDERS",
        };
        let data = self.dc_fetch_all(report, "ALL", &filter,
            "UPDATE_DATE,SECURITY_CODE,HOLDER_RANK", "-1,1,1", 500, 10, &[]).await?;
        Ok(data.iter().map(|v| GdfxHoldingDetail {
            holder_name: json_str(v, "HOLDER_NAME"),
            holder_type: json_str_opt(v, "HOLDER_TYPE"),
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            report_date: json_str(v, "END_DATE"),
            holding_count: json_f64(v, "HOLD_NUM"),
            holding_change: json_f64_opt(v, "XZCHANGE"),
            holding_change_ratio: json_f64_opt(v, "CHANGE_RATIO"),
            holding_change_name: json_str_opt(v, "HOLDNUM_CHANGE_NAME"),
            circulating_market_cap: json_f64_opt(v, "HOLDER_MARKET_CAP"),
            notice_date: json_str(v, "UPDATE_DATE"),
        }).collect())
    }

    /// 股东持股分析-十大流通股东
    pub async fn stock_gdfx_free_holding_analyse_em(&self, date: &str) -> Result<Vec<GdfxHoldingAnalyse>> {
        let filter = format!("(END_DATE='{}')", fmt_date(date));
        let data = self.dc_fetch_all("RPT_F10_EH_FREEHOLDERS_ANALYSE", "ALL", &filter,
            "SECURITY_CODE", "1", 500, 10, &[]).await?;
        Ok(data.iter().map(|v| GdfxHoldingAnalyse {
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            report_date: json_str(v, "END_DATE"),
            holder_count: json_i64_opt(v, "HOLDER_NUM"),
            holder_count_change: json_i64_opt(v, "HOLDER_NUM_CHANGE"),
            holder_count_ratio: json_f64_opt(v, "HOLDER_NUM_RATIO"),
            avg_holding_amount: json_f64_opt(v, "AVG_HOLD_NUM"),
            avg_market_cap: json_f64_opt(v, "AVG_MARKET_CAP"),
            total_market_cap: json_f64_opt(v, "TOTAL_MARKET_CAP"),
            total_shares: json_f64_opt(v, "TOTAL_A_SHARES"),
            price: json_f64_opt(v, "CLOSE_PRICE"),
            change_pct: json_f64_opt(v, "CHANGE_RATE"),
        }).collect())
    }

    /// 股东持股分析-十大股东
    pub async fn stock_gdfx_holding_analyse_em(&self, date: &str) -> Result<Vec<GdfxHoldingAnalyse>> {
        let filter = format!("(END_DATE='{}')", fmt_date(date));
        let data = self.dc_fetch_all("RPT_F10_EH_HOLDERS_ANALYSE", "ALL", &filter,
            "SECURITY_CODE", "1", 500, 10, &[]).await?;
        Ok(data.iter().map(|v| GdfxHoldingAnalyse {
            code: json_str(v, "SECURITY_CODE"),
            name: json_str(v, "SECURITY_NAME_ABBR"),
            report_date: json_str(v, "END_DATE"),
            holder_count: json_i64_opt(v, "HOLDER_NUM"),
            holder_count_change: json_i64_opt(v, "HOLDER_NUM_CHANGE"),
            holder_count_ratio: json_f64_opt(v, "HOLDER_NUM_RATIO"),
            avg_holding_amount: json_f64_opt(v, "AVG_HOLD_NUM"),
            avg_market_cap: json_f64_opt(v, "AVG_MARKET_CAP"),
            total_market_cap: json_f64_opt(v, "TOTAL_MARKET_CAP"),
            total_shares: json_f64_opt(v, "TOTAL_A_SHARES"),
            price: json_f64_opt(v, "CLOSE_PRICE"),
            change_pct: json_f64_opt(v, "CHANGE_RATE"),
        }).collect())
    }

    /// 股东协同-十大流通股东
    pub async fn stock_gdfx_free_holding_teamwork_em(&self, symbol: &str) -> Result<Vec<GdfxTeamwork>> {
        let filter = format!("(HOLDER_TYPE=\"{symbol}\")");
        let data = self.dc_fetch_all("RPT_FREEHOLDERS_TEAMWORK", "ALL", &filter,
            "HOLDER_NUM", "-1", 500, 1, &[]).await?;
        Ok(data.iter().map(|v| GdfxTeamwork {
            holder_name: json_str(v, "HOLDER_NAME"),
            holder_type: json_str_opt(v, "HOLDER_TYPE"),
            holding_count: json_i64(v, "HOLDER_NUM"),
            total_market_cap: json_f64_opt(v, "FREE_MARKET_CAP"),
            holding_stocks: json_str_opt(v, "HOLD_SECURITIES"),
        }).collect())
    }

    /// 股东协同-十大股东
    pub async fn stock_gdfx_holding_teamwork_em(&self, symbol: &str) -> Result<Vec<GdfxTeamwork>> {
        let filter = format!("(HOLDER_TYPE=\"{symbol}\")");
        let data = self.dc_fetch_all("RPT_HOLDERS_TEAMWORK", "ALL", &filter,
            "HOLDER_NUM", "-1", 500, 1, &[]).await?;
        Ok(data.iter().map(|v| GdfxTeamwork {
            holder_name: json_str(v, "HOLDER_NAME"),
            holder_type: json_str_opt(v, "HOLDER_TYPE"),
            holding_count: json_i64(v, "HOLDER_NUM"),
            total_market_cap: json_f64_opt(v, "FREE_MARKET_CAP"),
            holding_stocks: json_str_opt(v, "HOLD_SECURITIES"),
        }).collect())
    }

    /// 巨潮-股东持股变动
    pub async fn stock_hold_change_cninfo(&self, symbol: &str) -> Result<Vec<HoldChangeCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1087";
        let resp = self.post(url)
            .form(&[("seccode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(crate::error::Error::from)?
            .error_for_status().map_err(crate::error::Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(crate::error::Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| HoldChangeCninfo {
            data: v.clone(),
        }).collect())
    }

    /// 巨潮-实际控制人
    pub async fn stock_hold_control_cninfo(&self, symbol: &str) -> Result<Vec<HoldControlCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1088";
        let resp = self.post(url)
            .form(&[("seccode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(crate::error::Error::from)?
            .error_for_status().map_err(crate::error::Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(crate::error::Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| HoldControlCninfo {
            data: v.clone(),
        }).collect())
    }

    /// 巨潮-管理层明细
    pub async fn stock_hold_management_detail_cninfo(&self, symbol: &str) -> Result<Vec<ManagementDetail>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1089";
        let resp = self.post(url)
            .form(&[("seccode", symbol)])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(crate::error::Error::from)?
            .error_for_status().map_err(crate::error::Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(crate::error::Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| ManagementDetail {
            data: v.clone(),
        }).collect())
    }

    /// 东方财富-管理层明细
    pub async fn stock_hold_management_detail_em(&self, symbol: &str) -> Result<Vec<ManagementDetail>> {
        let url = "https://emweb.securities.eastmoney.com/PC_HSF10/CompanyManagement/PageAjax";
        let resp = self.get(url)
            .query(&[("code", symbol.to_uppercase().as_str())])
            .send().await.map_err(crate::error::Error::from)?
            .error_for_status().map_err(crate::error::Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(crate::error::Error::from)?;
        let arr = json.as_array().cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| ManagementDetail {
            data: v.clone(),
        }).collect())
    }

    /// 东方财富-管理层人员
    pub async fn stock_hold_management_person_em(&self, symbol: &str) -> Result<Vec<ManagementDetail>> {
        let url = "https://emweb.securities.eastmoney.com/PC_HSF10/CompanyManagement/PageAjax";
        let resp = self.get(url)
            .query(&[("code", symbol.to_uppercase().as_str())])
            .send().await.map_err(crate::error::Error::from)?
            .error_for_status().map_err(crate::error::Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(crate::error::Error::from)?;
        let arr = json.as_array().cloned().unwrap_or_default();
        Ok(arr.iter().map(|v| ManagementDetail {
            data: v.clone(),
        }).collect())
    }

    /// 巨潮-股东户数
    pub async fn stock_hold_num_cninfo(&self, symbol: &str, date: &str) -> Result<Vec<HoldChangeCninfo>> {
        let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1090";
        let sd = fmt_date(date);
        let resp = self.post(url)
            .form(&[("seccode", symbol), ("sdate", sd.as_str())])
            .header("User-Agent", "Mozilla/5.0")
            .send().await.map_err(crate::error::Error::from)?
            .error_for_status().map_err(crate::error::Error::from)?;
        let json: serde_json::Value = resp.json().await.map_err(crate::error::Error::from)?;
        let records = json.get("records").and_then(|r| r.as_array()).cloned().unwrap_or_default();
        Ok(records.iter().map(|v| HoldChangeCninfo {
            data: v.clone(),
        }).collect())
    }
}
