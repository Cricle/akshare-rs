//! US index data from Sina.
//!
//! Upstream `index_us_daily_sina` decodes Sina's compressed payload by
//! evaluating the obfuscated `zh_js_decode` JavaScript
//! (`akshare/stock/cons.py`) with `py_mini_racer`. This module ports the
//! deterministic `KLC_K2` variant of that decoder (`_k2_run`) to Rust, so no JS
//! runtime is required. Live payloads for `.INX`, `.IXIC`, `.DJI` and `.NDX`
//! all select this variant; other variants return [`Error::unavailable`].
//!
//! The decoder is a bit-packed format: records are delta-encoded against the
//! previous record, with variable-width bit fields and decimal exponent scaling.

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use serde_json::{Map, Value};

impl AkShareClient {
    /// 新浪财经 — 美股指数行情 (upstream `index_us_daily_sina`).
    ///
    /// Supported symbols include `.INX`, `.IXIC`, `.DJI` and `.NDX`.
    pub async fn index_us_stock(&self, symbol: &str) -> Result<Vec<serde_json::Value>> {
        let url = format!("https://finance.sina.com.cn/staticdata/us/{symbol}");
        let resp =
            crate::util::send_and_check(self.get(&url).header("User-Agent", "Mozilla/5.0")).await?;
        let text = resp.text().await.map_err(Error::from)?;
        let encoded = parse_sina_payload(&text)?;
        decode_sina_payload(&encoded)
    }
}

/// Extract the encoded string from `var KLC_K2_$INX="...";`.
fn parse_sina_payload(text: &str) -> Result<String> {
    let rest = text
        .split_once('=')
        .map(|(_, rest)| rest)
        .ok_or_else(|| Error::decode("index_us_stock: malformed Sina payload (missing '=')"))?;
    let encoded = rest.split_once(';').map_or(rest, |(head, _)| head);
    let encoded = encoded.trim().replace('"', "");
    if encoded.is_empty() {
        return Err(Error::decode("index_us_stock: empty Sina payload"));
    }
    Ok(encoded)
}

/// Decode the `zh_js_decode` dispatch header, then the K2 record stream.
fn decode_sina_payload(encoded: &str) -> Result<Vec<Value>> {
    let mut reader = SinaBitReader::new(encoded)
        .ok_or_else(|| Error::decode("index_us_stock: payload is not valid Sina base64"))?;
    let header = reader.n(&[12, 6], &[false, false]);
    if header.len() < 2 {
        return Err(Error::decode(
            "index_us_stock: truncated Sina decoder header",
        ));
    }
    let variant = header[0] as i64;
    // Upstream: s = 63 ^ c[1]
    let s = 63 ^ (header[1] as i64);
    if variant != 3466 {
        return Err(Error::unavailable(format!(
            "index_us_stock: unsupported Sina decoder variant {variant}; \
             only the KLC_K2 variant (`_k2_run`) is ported"
        )));
    }
    if s > 0 {
        // `_k2_run` returns [] when s > 0.
        return Ok(Vec::new());
    }
    decode_k2(&mut reader)
}

// ---------------------------------------------------------------------------
// Bit reader
// ---------------------------------------------------------------------------

const SINA_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

struct SinaBitReader {
    vals: Vec<u8>,
    /// Index of the current 6-bit character.
    e: usize,
    /// Bit offset inside the current character (0..5).
    a: u32,
    len: usize,
}

impl SinaBitReader {
    fn new(text: &str) -> Option<Self> {
        let mut vals = Vec::with_capacity(text.len());
        for ch in text.bytes() {
            let idx = SINA_ALPHABET.iter().position(|&c| c == ch)?;
            vals.push(idx as u8);
        }
        let len = vals.len();
        Some(Self {
            vals,
            e: 0,
            a: 0,
            len,
        })
    }

    fn bit(&mut self) -> bool {
        if self.e >= self.len {
            return false;
        }
        let t = self.vals[self.e] & (1u8 << self.a);
        self.a += 1;
        if self.a >= 6 {
            self.a -= 6;
            self.e += 1;
        }
        t != 0
    }

    /// Port of JS `N(widths, signed)`.
    ///
    /// Reads `widths[i]` bits, LSB-first, and two's-complement sign-extends
    /// fields flagged in `signed`. Widths above 30 are split into a 30-bit low
    /// chunk plus the remainder (matching upstream's recursive branch).
    fn n(&mut self, widths: &[i64], signed: &[bool]) -> Vec<f64> {
        let mut out = Vec::with_capacity(widths.len());
        for (s, &w) in widths.iter().enumerate() {
            if w == 0 {
                out.push(0.0);
                continue;
            }
            if self.e >= self.len {
                return out;
            }
            if w < 0 {
                out.push(0.0);
                continue;
            }
            let signed_flag = signed.get(s).copied().unwrap_or(false);
            if w <= 30 {
                let original = w;
                let mut remaining = w;
                let mut c: i64 = 0;
                loop {
                    let d = std::cmp::min(6 - self.a as i64, remaining);
                    let mask: i64 = (1i64 << d) - 1;
                    let cur = ((self.vals[self.e] as i64) >> self.a) & mask;
                    c |= cur << (original - remaining);
                    self.a += d as u32;
                    if self.a >= 6 {
                        self.a -= 6;
                        self.e += 1;
                    }
                    remaining -= d;
                    if remaining <= 0 {
                        break;
                    }
                }
                if signed_flag && c >= (1i64 << (original - 1)) {
                    c -= 1i64 << original;
                }
                out.push(c as f64);
            } else {
                let parts = self.n(&[30, w - 30], &[false, signed_flag]);
                let c0 = parts.first().copied().unwrap_or(f64::NAN);
                let c1 = parts.get(1).copied().unwrap_or(f64::NAN);
                out.push(c0 + c1 * (1i64 << 30) as f64);
            }
        }
        out
    }

    fn one(&mut self, widths: &[i64], signed: &[bool]) -> f64 {
        self.n(widths, signed).first().copied().unwrap_or(f64::NAN)
    }

    /// Port of JS `y()`: signed run-length value.
    fn y(&mut self) -> f64 {
        let t = self.bit();
        let mut e = 1i64;
        loop {
            if !self.bit() {
                return (e * (2 * t as i64 - 1)) as f64;
            }
            e += 1;
        }
    }

    /// Port of JS `x()`: day delta, with absolute-date escape.
    fn x(&mut self, state: &mut SinaK2State) -> i64 {
        let mut t = self.one(&[3], &[]) as i64;
        if t == 1 {
            state.d = self.one(&[18], &[true]) as i64;
            t = 0;
        } else if t == 0 {
            t = self.one(&[6], &[]) as i64;
        }
        t
    }
}

// ---------------------------------------------------------------------------
// Decimal exponent scaling (JS `T` / `C` / `_decnum`)
// ---------------------------------------------------------------------------

/// Port of JS `T(t)`.
fn t_pair(t: i64) -> [i64; 2] {
    if t == 0 {
        return [0, 0];
    }
    if t < 0 {
        let p = t_pair(-t);
        return [-p[0], -p[1]];
    }
    let e = t % 3;
    let n = (t - e) / 3;
    let mut r = [n, n];
    if e != 0 {
        r[(e - 1) as usize] += 1;
    }
    r
}

/// Port of JS `_decnum(t)` for integer-valued inputs.
fn decnum(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    if v.is_nan() {
        // JS `(t || 0)` treats NaN as falsy.
        return "0".into();
    }
    if v.is_infinite() {
        return if v > 0.0 {
            "Infinity".into()
        } else {
            "-Infinity".into()
        };
    }
    if v.fract() == 0.0 && v.abs() < 1e21 {
        return format!("{v:.0}");
    }
    let s = format!("{v}");
    if let Some(pos) = s.find(['e', 'E']) {
        let mantissa = &s[..pos];
        let exp: i32 = s[pos + 1..].parse().unwrap_or(0);
        if exp >= 0 {
            let mut digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
            let total = digits.len() as i32 + exp;
            while (digits.len() as i32) < total {
                digits.push('0');
            }
            return digits;
        }
        return String::new();
    }
    s
}

/// Port of JS `C(t, e, n)`.
fn c_convert(t: f64, e: [i64; 2], n: Option<[i64; 2]>) -> f64 {
    let rp = n.unwrap_or([0, 0]);
    let mut i0 = rp[0] - e[0];
    let mut i1 = rp[1] - e[1];
    let mut o = 1.0f64;
    while i0 < i1 {
        o *= 5.0;
        i1 -= 1;
    }
    while i1 < i0 {
        o *= 2.0;
        i0 -= 1;
    }
    let mut tv = t;
    if o > 1.0 {
        tv *= o;
    }
    let mut i = i0;
    let mut s = decnum(tv);
    if i < 0 {
        while s.len() as i64 + i <= 0 {
            s.insert(0, '0');
        }
        i += s.len() as i64;
        let idx = i as usize;
        let intpart: f64 = s[..idx].parse().unwrap_or(f64::NAN);
        match n {
            None => {
                let joined = if idx >= s.len() {
                    format!("{intpart}")
                } else {
                    format!("{intpart}.{}", &s[idx..])
                };
                joined.parse::<f64>().unwrap_or(f64::NAN)
            }
            Some(_) => {
                let first = s.as_bytes()[idx].wrapping_sub(b'0') as i32;
                let mut out = intpart;
                if first > 5 {
                    out += 1.0;
                } else if first == 5 {
                    let rest = &s[idx + 1..];
                    let restv: f64 = if rest.is_empty() {
                        0.0
                    } else {
                        rest.parse().unwrap_or(0.0)
                    };
                    if restv > 0.0 {
                        out += 1.0;
                    } else {
                        out += ((out as i64) & 1) as f64;
                    }
                }
                out
            }
        }
    } else {
        for _ in 0..i {
            s.push('0');
        }
        s.parse::<f64>().unwrap_or(0.0)
    }
}

// ---------------------------------------------------------------------------
// K2 record decoder
// ---------------------------------------------------------------------------

struct SinaK2State {
    b_avp: bool,
    b_ph: bool,
    b_phx: bool,
    b_sep: bool,
    p_p: i64,
    p_v: i64,
    p_a: i64,
    p_e: i64,
    p_t: i64,
    l_o: i64,
    l_h: i64,
    l_l: i64,
    l_c: i64,
    l_v: i64,
    l_a: i64,
    l_e: i64,
    l_t: i64,
    u_p: f64,
    u_v: f64,
    u_a: f64,
    u_o: f64,
    u_h: f64,
    u_l: f64,
    u_c: f64,
    wd: i64,
    d: i64,
}

impl SinaK2State {
    fn new() -> Self {
        Self {
            b_avp: true,
            b_ph: false,
            b_phx: false,
            b_sep: false,
            p_p: 6,
            p_v: 0,
            p_a: 0,
            p_e: 0,
            p_t: 0,
            l_o: 3,
            l_h: 3,
            l_l: 3,
            l_c: 3,
            l_v: 5,
            l_a: 5,
            l_e: 3,
            l_t: 0,
            u_p: 0.0,
            u_v: 0.0,
            u_a: 0.0,
            u_o: f64::NAN,
            u_h: f64::NAN,
            u_l: f64::NAN,
            u_c: f64::NAN,
            wd: 62,
            d: 0,
        }
    }

    fn p(&self, ch: char) -> i64 {
        match ch {
            'p' => self.p_p,
            'v' => self.p_v,
            'a' => self.p_a,
            'e' => self.p_e,
            't' => self.p_t,
            _ => 0,
        }
    }

    fn set_p(&mut self, ch: char, v: i64) {
        match ch {
            'p' => self.p_p = v,
            'v' => self.p_v = v,
            'a' => self.p_a = v,
            'e' => self.p_e = v,
            't' => self.p_t = v,
            _ => {}
        }
    }

    fn l(&self, ch: char) -> i64 {
        match ch {
            'o' => self.l_o,
            'h' => self.l_h,
            'l' => self.l_l,
            'c' => self.l_c,
            'v' => self.l_v,
            'a' => self.l_a,
            'e' => self.l_e,
            't' => self.l_t,
            _ => 0,
        }
    }

    fn add_l(&mut self, ch: char, d: i64) {
        match ch {
            'o' => self.l_o += d,
            'h' => self.l_h += d,
            'l' => self.l_l += d,
            'c' => self.l_c += d,
            'v' => self.l_v += d,
            'a' => self.l_a += d,
            'e' => self.l_e += d,
            't' => self.l_t += d,
            _ => {}
        }
    }

    fn u(&self, ch: char) -> f64 {
        match ch {
            'o' => self.u_o,
            'h' => self.u_h,
            'l' => self.u_l,
            'c' => self.u_c,
            'p' => self.u_p,
            'v' => self.u_v,
            'a' => self.u_a,
            _ => f64::NAN,
        }
    }

    fn set_u(&mut self, ch: char, v: f64) {
        match ch {
            'o' => self.u_o = v,
            'h' => self.u_h = v,
            'l' => self.u_l = v,
            'c' => self.u_c = v,
            'p' => self.u_p = v,
            'v' => self.u_v = v,
            'a' => self.u_a = v,
            _ => {}
        }
    }
}

#[derive(Default)]
struct SinaRecord {
    date: String,
    prevclose: Option<f64>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    amount: f64,
    post_vol: Option<f64>,
    post_amt: Option<f64>,
}

fn sina_date(days: i64) -> Result<String> {
    let base = chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
        .ok_or_else(|| Error::decode("index_us_stock: invalid epoch date"))?;
    let date = base
        .checked_add_signed(chrono::Duration::days(days))
        .ok_or_else(|| Error::decode("index_us_stock: date out of range"))?;
    Ok(date.format("%Y-%m-%d").to_string())
}

/// Port of JS `S(t)`: advance `state.d` by `t` trading days.
fn advance_date(state: &mut SinaK2State, t: i64) -> Result<String> {
    let wd = if state.wd != 0 { state.wd } else { 62 };
    for _ in 0..t {
        let mut guard = 0;
        loop {
            state.d += 1;
            let bitpos = ((state.d % 7 + 10) % 7) as u32;
            if (wd & (1i64 << bitpos)) != 0 {
                break;
            }
            guard += 1;
            if guard > 100 {
                return Err(Error::decode(
                    "index_us_stock: invalid weekday mask in payload",
                ));
            }
        }
    }
    sina_date(7657 + state.d)
}

fn jnum(value: f64) -> Value {
    serde_json::Number::from_f64(value).map_or(Value::Null, Value::Number)
}

fn record_to_value(rec: &SinaRecord) -> Value {
    let mut map = Map::new();
    map.insert("date".into(), Value::String(rec.date.clone()));
    if let Some(v) = rec.prevclose {
        map.insert("prevclose".into(), jnum(v));
    }
    map.insert("open".into(), jnum(rec.open));
    map.insert("high".into(), jnum(rec.high));
    map.insert("low".into(), jnum(rec.low));
    map.insert("close".into(), jnum(rec.close));
    map.insert("volume".into(), jnum(rec.volume));
    map.insert("amount".into(), jnum(rec.amount));
    if let Some(v) = rec.post_vol {
        map.insert("postVol".into(), jnum(v));
    }
    if let Some(v) = rec.post_amt {
        map.insert("postAmt".into(), jnum(v));
    }
    Value::Object(map)
}

/// Port of the `_k2_run` function inside `zh_js_decode`.
fn decode_k2(reader: &mut SinaBitReader) -> Result<Vec<Value>> {
    let mut st = SinaK2State::new();
    let mut out: Vec<SinaRecord> = Vec::new();

    loop {
        if reader.e >= reader.len {
            break;
        }
        let mut rd: i64 = 1;
        let mut rc: i64 = 0;
        let mut rp: f64 = 0.0;

        if reader.bit() {
            if reader.bit() {
                if reader.bit() {
                    rc += 1;
                    let ra = st.b_avp;
                    if reader.bit() {
                        st.b_avp ^= reader.bit();
                        st.b_ph ^= reader.bit();
                        st.b_phx ^= reader.bit();
                        let old_sep = st.b_sep;
                        st.b_sep ^= reader.bit();
                        if reader.bit() {
                            st.wd = reader.one(&[7], &[]) as i64;
                        }
                        if old_sep != st.b_sep {
                            if old_sep {
                                st.u_p = st.u_c;
                            } else {
                                st.u_o = st.u_p;
                                st.u_h = st.u_p;
                                st.u_l = st.u_p;
                                st.u_c = st.u_p;
                            }
                        }
                    }
                    let lim = 3 + 2 * (st.b_ph as i64);
                    for c in 0..lim {
                        if reader.bit() {
                            let ch = ['p', 'v', 'a', 'e', 't'][c as usize];
                            let a = st.p(ch);
                            let delta = reader.y() as i64;
                            st.set_p(ch, a + delta);
                            let newp = st.p(ch);
                            let oldu = st.u(ch);
                            st.set_u(ch, c_convert(oldu, t_pair(a), Some(t_pair(newp))));
                            if st.b_sep && c == 0 {
                                for uch in ['o', 'h', 'l', 'c'] {
                                    let old = st.u(uch);
                                    st.set_u(uch, c_convert(old, t_pair(a), Some(t_pair(st.p_p))));
                                }
                            }
                        }
                    }
                    // JS: !o.b_avp && r.a && (o.u_a = C(n && n.amount || 0, 0, o.p_a))
                    if !st.b_avp && ra {
                        let prev_amount = out.last().map_or(0.0, |rec| rec.amount);
                        let prev_amount = if prev_amount.is_nan() {
                            0.0
                        } else {
                            prev_amount
                        };
                        st.u_a = c_convert(prev_amount, [0, 0], Some(t_pair(st.p_a)));
                    }
                }
                if reader.bit() {
                    rc += 1;
                    let lim = 7 + (st.b_ph as i64) + (st.b_phx as i64);
                    for c in 0..lim {
                        if reader.bit() {
                            if c == 6 {
                                rd = reader.x(&mut st);
                            } else {
                                let ch = "ohlcva*et".chars().nth(c as usize).unwrap_or('o');
                                let delta = reader.y() as i64;
                                st.add_l(ch, delta);
                            }
                        }
                    }
                }
                if reader.bit() {
                    rc += 1;
                    let extra = if reader.bit() { reader.y() as i64 } else { 0 };
                    let len = st.l_o + extra;
                    let a = reader.one(&[3 * len], &[true]);
                    rp = if st.b_sep {
                        st.u_c + a
                    } else {
                        st.u_p += a;
                        st.u_p
                    };
                }
                if rc == 0 {
                    break;
                }
            } else if reader.bit() {
                if reader.bit() {
                    if reader.bit() {
                        rd = reader.x(&mut st);
                    } else {
                        let d = reader.y() as i64;
                        st.l_v += d;
                    }
                } else if st.b_ph && reader.bit() {
                    let ch = if st.b_phx && reader.bit() { 't' } else { 'e' };
                    let d = reader.y() as i64;
                    st.add_l(ch, d);
                } else {
                    let d = reader.y() as i64;
                    st.l_a += d;
                }
            } else {
                let idx = reader.one(&[2], &[]) as usize;
                let ch = ['o', 'h', 'l', 'c'][idx.min(3)];
                let d = reader.y() as i64;
                st.add_l(ch, d);
            }
        }

        let vlim = 6 + (st.b_ph as i64) + (st.b_phx as i64);
        let mut v = [0.0f64; 8];
        for c in 0..vlim {
            let ch = ['o', 'h', 'l', 'c', 'v', 'a', 'e', 't'][c as usize];
            let signed = ((if st.b_sep { 191 } else { 185 }) >> c) & 1 != 0;
            let width = 3 * st.l(ch);
            v[c as usize] = reader.one(&[width], &[signed]);
        }
        let (v_o, v_h, v_l, v_c) = (v[0], v[1], v[2], v[3]);
        let (v_v, v_a, v_e, v_t) = (v[4], v[5], v[6], v[7]);

        let mut rec = SinaRecord {
            date: advance_date(&mut st, rd)?,
            ..SinaRecord::default()
        };
        if rp != 0.0 {
            rec.prevclose = Some(c_convert(rp, t_pair(st.p_p), None));
        }

        let r_o = if st.b_sep {
            st.u_o += v_o;
            rec.open = c_convert(st.u_o, t_pair(st.p_p), None);
            st.u_h += v_h;
            rec.high = c_convert(st.u_h, t_pair(st.p_p), None);
            st.u_l += v_l;
            rec.low = c_convert(st.u_l, t_pair(st.p_p), None);
            st.u_c += v_c;
            rec.close = c_convert(st.u_c, t_pair(st.p_p), None);
            0.0
        } else {
            let r_o = st.u_p + v_o;
            rec.open = c_convert(r_o, t_pair(st.p_p), None);
            rec.high = c_convert(r_o + v_h, t_pair(st.p_p), None);
            rec.low = c_convert(r_o - v_l, t_pair(st.p_p), None);
            let new_close = r_o + v_c;
            rec.close = c_convert(new_close, t_pair(st.p_p), None);
            st.u_p = new_close;
            r_o
        };
        st.u_v += v_v;
        rec.volume = c_convert(st.u_v, t_pair(st.p_v), None);

        if st.b_avp {
            let tp = t_pair(st.p_p);
            let tv = t_pair(st.p_v);
            let base = if st.b_sep {
                (st.u_o + st.u_h + st.u_l + st.u_c) / 4.0
            } else {
                r_o + (v_h - v_l + v_c) / 4.0
            };
            let inner = c_convert(
                (base * st.u_v + 0.5).floor(),
                [tp[0] + tv[0], tp[1] + tv[1]],
                Some(t_pair(st.p_a)),
            );
            rec.amount = c_convert(inner + v_a, t_pair(st.p_a), None);
        } else {
            st.u_a += v_a;
            rec.amount = c_convert(st.u_a, t_pair(st.p_a), None);
        }
        if st.b_ph {
            let post_vol = c_convert(v_e, t_pair(st.p_e), None);
            rec.post_vol = Some(post_vol);
            let extra = if st.b_phx {
                c_convert(v_t, t_pair(st.p_t), None)
            } else {
                0.0
            };
            rec.post_amt = Some(c_convert(
                (post_vol * rec.close + extra + 0.5).floor(),
                [0, 0],
                None,
            ));
        }

        out.push(rec);
    }

    Ok(out.iter().map(record_to_value).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorKind;

    #[test]
    fn test_k2_dispatch_header() {
        let mut reader = SinaBitReader::new("K2/").unwrap();
        let header = reader.n(&[12, 6], &[false, false]);
        assert_eq!(header[0] as i64, 3466);
        assert_eq!(63 ^ header[1] as i64, 0);
    }

    #[test]
    fn test_unsupported_variant_is_explicit_error() {
        let err = decode_sina_payload("AAAA").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Unavailable);
    }

    #[test]
    fn test_k2_nonzero_s_returns_empty() {
        // Same K2 variant, but the 6-bit selector is not '/', so upstream
        // `_k2_run` returns an empty list.
        assert!(decode_sina_payload("K2A").unwrap().is_empty());
    }

    #[test]
    fn test_sina_date_epoch() {
        assert_eq!(sina_date(7658).unwrap(), "1990-12-20");
    }
}
