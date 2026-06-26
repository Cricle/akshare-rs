//! Eastmoney global index data — spot quotes and daily history.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::CandlePoint;

impl AkShareClient {
    /// Global index real-time quotes.
    ///
    /// Uses Sina's `hq.sinajs.cn` API as primary source since the Eastmoney
    /// push2 endpoint may be unavailable in certain network environments.
    pub async fn index_global_spot(&self) -> Result<Vec<GlobalEmSpotItem>> {
        // Sina global index symbols
        let sina_symbols = [
            ("int_dji", "DJIA", "道琼斯"),
            ("int_nasdaq", "NDX", "纳斯达克"),
            ("int_sp500", "SPX", "标普500"),
            ("int_hangseng", "HSI", "恒生指数"),
            ("int_nikkei", "N225", "日经225"),
            ("int_ftse", "FTSE", "英国富时100"),
            ("int_dax", "GDAXI", "德国DAX"),
            ("int_cac", "FCHI", "法国CAC40"),
            ("int_kospi", "KS11", "韩国KOSPI"),
            ("int_twii", "TWII", "台湾加权"),
            ("b_TWSE", "STI", "富时新加坡海峡时报"),
        ];

        let symbols_csv: Vec<&str> = sina_symbols.iter().map(|(s, _, _)| *s).collect();
        let url = format!("https://hq.sinajs.cn/list={}", symbols_csv.join(","));

        let body = self
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .send()
            .await?
            .text()
            .await?;

        let mut items = Vec::new();
        for (i, line) in body.lines().enumerate() {
            let data = line
                .split_once('=')
                .and_then(|(_, r)| r.trim_matches('"').split_once(';'))
                .map_or("", |(s, _)| s);
            if data.is_empty() || i >= sina_symbols.len() {
                continue;
            }
            let fields: Vec<&str> = data.split(',').collect();
            if fields.len() < 4 {
                continue;
            }
            let (_, code, name) = sina_symbols[i];
            let close = fields[1].parse::<f64>().unwrap_or(0.0);
            let change_amount = fields[2].parse::<f64>().unwrap_or(0.0);
            let change_pct = fields[3].parse::<f64>().unwrap_or(0.0);
            if close == 0.0 {
                continue;
            }
            items.push(GlobalEmSpotItem {
                code: code.to_string(),
                name: name.to_string(),
                close,
                change_pct,
                change_amount,
                amplitude_pct: 0.0,
                high: 0.0,
                low: 0.0,
                open: 0.0,
                prev_close: close - change_amount,
                timestamp: 0,
            });
        }

        if items.is_empty() {
            return Err(Error::not_found("sina returned no global index data"));
        }
        Ok(items)
    }

    /// 东方财富 — 全球指数历史行情.
    ///
    /// `market` and `code` come from the `index_global_em_symbol_map` mapping,
    /// e.g. market="100", code="SPX" for S&P 500.
    pub async fn index_global_hist_em(
        &self,
        market: &str,
        code: &str,
        limit: usize,
    ) -> Result<Vec<CandlePoint>> {
        let secid = format!("{market}.{code}");
        self.eastmoney_klines(&secid, "qfq", limit).await
    }
}

/// Eastmoney global index spot item.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalEmSpotItem {
    pub code: String,
    pub name: String,
    pub close: f64,
    pub change_pct: f64,
    pub change_amount: f64,
    pub amplitude_pct: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub prev_close: f64,
    pub timestamp: i64,
}

/// Well-known global index symbol map (Eastmoney).
///
/// Maps Chinese name -> (market_id, code).
#[must_use]
pub fn global_em_symbol_map(name: &str) -> Option<(&str, &str)> {
    match name {
        "上证指数" => Some(("1", "000001")),
        "深证成指" => Some(("0", "399001")),
        "中小100" => Some(("0", "399005")),
        "创业板指" => Some(("0", "399006")),
        "沪深300" => Some(("1", "000300")),
        "恒生指数" => Some(("100", "HSI")),
        "国企指数" => Some(("100", "HSCEI")),
        "红筹指数" => Some(("124", "HSCCI")),
        "台湾加权" => Some(("100", "TWII")),
        "日经225" => Some(("100", "N225")),
        "韩国KOSPI200" => Some(("100", "KOSPI200")),
        "韩国KOSPI" => Some(("100", "KS11")),
        "富时新加坡海峡时报" => Some(("100", "STI")),
        "印度孟买SENSEX" => Some(("100", "SENSEX")),
        "富时马来西亚KLCI" => Some(("100", "KLSE")),
        "泰国SET" => Some(("100", "SET")),
        "菲律宾马尼拉" => Some(("100", "PSI")),
        "巴基斯坦卡拉奇" => Some(("100", "KSE100")),
        "越南胡志明" => Some(("100", "VNINDEX")),
        "印尼雅加达综合" => Some(("100", "JKSE")),
        "斯里兰卡科伦坡" => Some(("100", "CSEALL")),
        "欧洲斯托克50" => Some(("100", "SX5E")),
        "英国富时100" => Some(("100", "FTSE")),
        "英国富时250" => Some(("100", "MCX")),
        "富时AIM全股" => Some(("100", "AXX")),
        "法国CAC40" => Some(("100", "FCHI")),
        "德国DAX30" => Some(("100", "GDAXI")),
        "俄罗斯RTS" => Some(("100", "RTS")),
        "西班牙IBEX35" => Some(("100", "IBEX")),
        "葡萄牙PSI20" => Some(("100", "PSI20")),
        "OMX哥本哈根20" => Some(("100", "OMXC20")),
        "比利时BFX" => Some(("100", "BFX")),
        "荷兰AEX" => Some(("100", "AEX")),
        "波兰WIG" => Some(("100", "WIG")),
        "瑞典OMXSPI" => Some(("100", "OMXSPI")),
        "瑞士SMI" => Some(("100", "SSMI")),
        "芬兰赫尔辛基" => Some(("100", "HEX")),
        "挪威OSEBX" => Some(("100", "OSEBX")),
        "奥地利ATX" => Some(("100", "ATX")),
        "富时意大利MIB" => Some(("100", "MIB")),
        "希腊雅典ASE" => Some(("100", "ASE")),
        "冰岛ICEX" => Some(("100", "ICEXI")),
        "布拉格指数" => Some(("100", "PX")),
        "爱尔兰综合" => Some(("100", "ISEQ")),
        "道琼斯" => Some(("100", "DJIA")),
        "标普500" => Some(("100", "SPX")),
        "纳斯达克" => Some(("100", "NDX")),
        "加拿大S&P/TSX" => Some(("100", "TSX")),
        "巴西BOVESPA" => Some(("100", "BVSP")),
        "墨西哥BOLSA" => Some(("100", "MXX")),
        "澳大利亚标普200" => Some(("100", "AS51")),
        "澳大利亚普通股" => Some(("100", "AORD")),
        "新西兰50" => Some(("100", "NZ50")),
        "美元指数" => Some(("100", "UDI")),
        "波罗的海BDI指数" => Some(("100", "BDI")),
        "路透CRB商品指数" => Some(("100", "CRB")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_em_symbol_map() {
        assert_eq!(global_em_symbol_map("恒生指数"), Some(("100", "HSI")));
        assert_eq!(global_em_symbol_map("标普500"), Some(("100", "SPX")));
        assert_eq!(global_em_symbol_map("不存在"), None);
    }
}
