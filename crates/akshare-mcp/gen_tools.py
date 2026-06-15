#!/usr/bin/env python3
"""Auto-generate tools.txt from AkShareClient pub async fn methods.

Scans the akshare crate for all pub async fn methods on AkShareClient,
maps their parameters to existing param types, and generates tools.txt.

Usage:
    python3 gen_tools.py > tools.txt
    python3 gen_tools.py --check   # verify tools.txt is up-to-date
"""

import re
import os
import sys
import subprocess

AKSHARE_SRC = os.path.join(os.path.dirname(__file__), '..', 'akshare', 'src')
TOOLS_TXT = os.path.join(os.path.dirname(__file__), 'tools.txt')

# Parameter signature -> (param_type, call_args)
PARAM_MAP = {
    '&self': ('_', ''),
    '&self, symbol: &str': ('stock::SymbolParams', '&symbol'),
    '&self, date: &str': ('stock::DateParams', '&date'),
    '&self, limit: usize': ('stock::LimitParams', 'limit'),
    '&self, symbol: &str, limit: usize': ('stock::CandlesParams', '&symbol, limit'),
    '&self, symbol: &str, period: &str': ('stock::BoardHistMinParams', '&symbol, &period'),
    '&self, symbol: &str, date: &str': ('stock::SymbolDateParams', '&symbol, &date'),
    '&self, symbol: &str, adjust: &str': ('stock::CyqParams', '&symbol, &adjust'),
    '&self, start_date: &str, end_date: &str': ('stock::DateRangeParams', '&start_date, &end_date'),
    '&self, market: &str': ('stock::DateMarketParams', '&market'),
    '&self, indicator: &str': ('stock::SymbolIndicatorParams', '&indicator'),
    '&self, year: &str': ('stock::DateParams', '&date'),
    '&self, sector: &str': ('stock::SymbolSectorParams', '&sector'),
    '&self, symbol: &str, indicator: &str': ('stock::SymbolIndicatorParams', '&symbol, &indicator'),
    '&self, symbol: &str, period: &str, start_date: &str, end_date: &str, adjust: &str': ('stock::StockHistParams', '&symbol, &period, &start_date, &end_date, &adjust'),
    '&self, symbol: &str, start_date: &str, end_date: &str': ('stock::SymbolDateRangeParams', '&symbol, &start_date, &end_date'),
    '&self, symbol: &str, start_date: &str, end_date: &str, adjust: &str': ('stock::StockHistParams', '&symbol, &period, &start_date, &end_date, &adjust'),
    '&self, symbol: &str, token: &str': ('stock::XqInfoParams', '&symbol, &token'),
    '&self, query: &str, timeout_secs: u64': ('news::NewsQueryTimeoutParams', '&query, timeout_secs'),
    '&self, pair: &str': ('forex::CurrencyPairParams', '&pair'),
    '&self, exchange: &str, start: &str, end: &str': ('stock::TradeCalendarParams', '&exchange, &start_date, &end_date'),
    '&self, market: &str, date: &str': ('stock::DateMarketParams', '&date, &market'),
    '&self, symbol: &str, market: Option<&str>, limit: usize': ('stock::SearchParams', '&query, market.as_deref(), limit'),
    '&self, stock: &str, symbol: &str, indicator: &str': ('stock::HkReportParams', '&symbol, &symbol, &indicator'),
    '&self, symbol: &str, side: &str, limit: usize': ('stock::BillboardSeatsParams', '&symbol, &side, limit'),
    '&self, symbol: &str, internal_id: &str, limit: usize': ('index::IndexHkDailyParams', '&symbol, &internal_id, limit'),
    '&self, symbol: &str, market: &str, limit: usize': ('stock::FundFlowParams', '&symbol, &market, limit'),
    '&self, from_page: i32, to_page: i32': ('stock::LimitParams', 'limit'),
    '&self, symbol: &str, _start_date: &str, _end_date: &str, _adjust: &str': ('stock::StockHistParams', '&symbol, &period, &start_date, &end_date, &adjust'),
    # eastmoney variants
    '&self, secid: &str, adjust: &str, limit: usize': ('stock::CandlesParams', '&symbol, "", limit'),
    '&self, sector_type: &str, limit: usize': ('stock::SectorRankParams', '&sector_type, limit'),
    '&self, sector_code: &str, limit: usize': ('stock::SectorCodeParams', '&sector_code, limit'),
    '&self, code: &str': ('stock::SymbolParams', '&symbol'),
    '&self, item: &str': ('stock::SymbolParams', '&symbol'),
    '&self, art_code: &str': ('stock::SymbolParams', '&symbol'),
    '&self, secid: &str': ('stock::SymbolParams', '&symbol'),
    '&self, question_id: &str': ('stock::SymbolParams', '&symbol'),
    '&self, _symbol: &str': ('stock::SymbolParams', '&symbol'),
    # macro data
    '&self, kind: &str, path: &str, period: &str': ('macro_data::MacroNbsNationParams', '&kind, &path, &period'),
    '&self, kind: &str, path: &str, indicator: &str, period: &str': ('macro_data::MacroNbsRegionParams', '&kind, &path, &indicator, &period'),
    '&self, market: &str, symbol: &str, indicator: &str': ('macro_data::MacroInterbankParams', '&market, &symbol, &indicator'),
    # forex
    '&self, base: &str, symbols: &str, api_key: &str': ('forex::CurrencyLatestParams', '&base, &symbols, &api_key'),
    '&self, base: &str, date: &str, symbols: &str, api_key: &str': ('forex::CurrencyHistoryParams', '&base, &date, &symbols, &api_key'),
    '&self, base: &str, start_date: &str, end_date: &str, symbols: &str, api_key: &str': ('forex::CurrencyTimeSeriesParams', '&base, &start_date, &end_date, &symbols, &api_key'),
    '&self, c_type: &str, api_key: &str': ('forex::CurrencyCurrenciesParams', '&c_type, &api_key'),
    '&self, from: &str, to: &str, amount: f64, api_key: &str': ('forex::CurrencyConvertParams', '&from, &to, amount, &api_key'),
    # stock-specific
    '&self, indicator: &str, limit: usize': ('stock::FundFlowRankParams', '&indicator, limit'),
    '&self, symbol: &str, date: &str, limit: usize': ('stock::SymbolDateParams', '&symbol, &date'),
    '&self, market: &str, code: &str, limit: usize': ('stock::FundFlowParams', '&symbol, &market, limit'),
    '&self, symbol: &str, quarter: &str': ('stock::SymbolIndicatorParams', '&symbol, &indicator'),
    '&self, analyst_id: &str, indicator: &str': ('stock::SymbolIndicatorParams', '&symbol, &indicator'),
    '&self, announcement_id: &str': ('stock::SymbolParams', '&symbol'),
}

# Single-param with non-standard name -> SymbolParams
SINGLE_PARAM_RE = re.compile(r'^&self, (\w+): &str$')

# Auth-required params
AUTH_PARAMS = {'cookie', 'user', 'password', 'token'}


def guess_category(method: str) -> str:
    """Guess tool category from method name prefix."""
    PREFIXES = [
        ('stock_', 'stock'), ('a_share_', 'stock'), ('hk_', 'stock'), ('us_', 'stock'),
        ('bond_', 'bond'), ('index_', 'index'), ('futures_', 'futures'), ('crypto_', 'crypto'),
        ('forex_', 'forex'), ('currency_', 'forex'), ('fx_', 'forex'),
        ('option_', 'option'), ('fund_', 'fund'), ('reits_', 'economy'),
        ('news_', 'news'), ('bing_', 'news'), ('google_', 'news'), ('sogou_', 'news'),
        ('baidu_', 'news'), ('gdelt_', 'news'),
        ('macro_', 'macro_data'), ('china_', 'macro_data'), ('usa_', 'macro_data'),
        ('euro_', 'macro_data'), ('japan_', 'macro_data'), ('uk_', 'macro_data'),
        ('germany_', 'macro_data'), ('canada_', 'macro_data'), ('australia_', 'macro_data'),
        ('swiss_', 'macro_data'), ('bank_', 'macro_data'),
        ('economy_', 'economy'), ('movie_', 'economy'), ('nlp_', 'economy'),
        ('amac_', 'economy'), ('car_', 'economy'), ('sw_', 'economy'),
        ('article_', 'economy'), ('air_', 'economy'), ('migration_', 'economy'),
        ('fred_', 'economy'), ('xincaifu_', 'economy'), ('rate_', 'economy'),
        ('online_', 'economy'), ('match_', 'economy'), ('hurun_', 'economy'),
        ('hf_', 'economy'), ('game_', 'economy'), ('forbes_', 'economy'),
        ('drewry_', 'economy'), ('business_', 'economy'), ('qhkc_', 'economy'),
        ('sunrise_', 'economy'), ('video_', 'economy'), ('repo_', 'economy'),
    ]
    for prefix, cat in PREFIXES:
        if method.startswith(prefix):
            return cat
    return 'economy'


def extract_methods(src_dir: str) -> list[tuple[str, str]]:
    """Extract all (method_name, normalized_params) from AkShareClient impl blocks."""
    results = []
    for root, dirs, files in os.walk(src_dir):
        for fname in files:
            if not fname.endswith('.rs'):
                continue
            fpath = os.path.join(root, fname)
            try:
                with open(fpath) as f:
                    content = f.read()
            except Exception:
                continue

            lines = content.split('\n')
            i = 0
            while i < len(lines):
                line = lines[i]
                if 'pub async fn ' in line and '(' in line:
                    sig_lines = [line]
                    paren_depth = line.count('(') - line.count(')')
                    while paren_depth > 0 and i + 1 < len(lines):
                        i += 1
                        sig_lines.append(lines[i])
                        paren_depth += lines[i].count('(') - lines[i].count(')')

                    sig_text = ' '.join(sig_lines)
                    m = re.search(r'pub async fn (\w+)\((.*?)\)\s*->', sig_text, re.DOTALL)
                    if m:
                        method = m.group(1)
                        raw_params = m.group(2).strip()
                        norm = re.sub(r'\s+', ' ', raw_params).strip().rstrip(',').strip()
                        results.append((method, norm))
                i += 1
    return results


def main():
    check_mode = '--check' in sys.argv

    methods = extract_methods(AKSHARE_SRC)

    # If in check mode, read existing tools.txt and compare
    if check_mode:
        existing = set()
        with open(TOOLS_TXT) as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#'):
                    parts = line.split('|')
                    if len(parts) >= 1:
                        existing.add(parts[0])

    entries = []
    skipped = []

    for method, norm in methods:
        # Skip auth-required
        if any(p in norm for p in AUTH_PARAMS):
            continue

        # Try to match param pattern
        if norm in PARAM_MAP:
            ptype, call_args = PARAM_MAP[norm]
        else:
            # Single param with any name -> SymbolParams
            single = SINGLE_PARAM_RE.match(norm)
            if single:
                ptype, call_args = 'stock::SymbolParams', '&symbol'
            else:
                skipped.append((method, norm))
                continue

        cat = guess_category(method)
        if call_args:
            call = f'self.client.{method}({call_args})'
        else:
            call = f'self.client.{method}()'

        entries.append((method, ptype, cat, method, call))

    # In check mode, report missing
    if check_mode:
        new_names = {e[0] for e in entries}
        missing = new_names - existing
        extra = existing - new_names
        if missing:
            print(f"Missing {len(missing)} tools: {sorted(missing)[:10]}...")
            sys.exit(1)
        if extra:
            print(f"Note: {len(extra)} tools in tools.txt not found in source (may be manually added)")
        print(f"tools.txt is up-to-date ({len(entries)} auto-discoverable tools)")
        return

    # Generate output
    print("# Auto-generated by gen_tools.py — do not edit manually.")
    print("# To regenerate: python3 gen_tools.py > tools.txt")
    print(f"# Total: {len(entries)} tools, {len(skipped)} skipped (complex params)")
    print()

    current_cat = ''
    for name, ptype, cat, desc, call in entries:
        if cat != current_cat:
            print(f"# ── {cat} ──")
            current_cat = cat
        print(f"{name}|{ptype}|{cat}|{desc}|{call}")

    if skipped:
        print()
        print(f"# Skipped {len(skipped)} methods with complex params:")
        for method, norm in skipped[:20]:
            print(f"#   {method}: {norm[:80]}")
        if len(skipped) > 20:
            print(f"#   ... and {len(skipped) - 20} more")


if __name__ == '__main__':
    main()
