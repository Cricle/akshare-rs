//! SGX (Singapore Exchange) futures settlement price data.

use std::io::{Cursor, Read};

use crate::client::AkShareClient;
use crate::error::{Error, Result};
use crate::types::Row;

/// Read the first non-directory entry of a ZIP archive.
fn read_first_zip_entry(bytes: &[u8], source: &str) -> Result<(String, Vec<u8>)> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|e| Error::decode(format!("{source} ZIP: {e}")))?;
    if archive.is_empty() {
        return Err(Error::not_found(format!("{source} ZIP is empty")));
    }
    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|e| Error::decode(format!("{source} ZIP entry: {e}")))?;
        if file.is_dir() {
            continue;
        }
        let name = file
            .name()
            .map_err(|e| Error::decode(format!("{source} ZIP entry name: {e}")))?
            .to_string();
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|e| Error::decode(format!("{source} ZIP read: {e}")))?;
        return Ok((name, data));
    }
    Err(Error::not_found(format!("{source} ZIP has no files")))
}

/// DCE/SGX archives use tab-separated `.txt` files or comma-separated `.csv`.
fn sgx_delimiter(name: &str) -> u8 {
    if name.to_ascii_lowercase().ends_with("txt") {
        b'\t'
    } else {
        b','
    }
}

/// Convert a settlement CSV cell to a JSON value, mirroring pandas inference.
fn cell_to_json(value: &str) -> serde_json::Value {
    let value = value.trim();
    if value.is_empty() {
        return serde_json::Value::Null;
    }
    if let Ok(integer) = value.parse::<i64>() {
        return serde_json::json!(integer);
    }
    if let Ok(float) = value.parse::<f64>() {
        return serde_json::json!(float);
    }
    serde_json::json!(value)
}

/// Parse a header row plus data rows into `Row` maps.
fn parse_delimited_rows(text: &str, delimiter: u8) -> Result<Vec<Row>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(true)
        .flexible(true)
        .from_reader(text.as_bytes());

    let headers = reader
        .headers()
        .map_err(|e| Error::decode(format!("SGX settlement header: {e}")))?
        .clone();
    if headers.is_empty() {
        return Err(Error::decode("SGX settlement file has no header row"));
    }

    let mut items = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|e| Error::decode(format!("SGX settlement row: {e}")))?;
        let mut row = Row::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            row.insert(header.trim().to_string(), cell_to_json(value));
        }
        if !row.is_empty() {
            items.push(row);
        }
    }
    Ok(items)
}

impl AkShareClient {
    /// SGX derivatives historical settlement prices.
    ///
    /// Fetches all futures settlement prices for a given trading date.
    ///
    /// SGX serves daily archives under a sequential id. The id is derived from
    /// the number of FTSE Straits Times Index daily bars available from
    /// Eastmoney up to `date`, plus a fixed offset (mirrors upstream Python:
    /// last kline index + 791).
    pub async fn futures_settlement_price_sgx(&self, date: &str) -> Result<Vec<Row>> {
        let num = self.sgx_archive_number(date).await?;
        let url = format!("https://links.sgx.com/1.0.0/derivatives-daily/{num}/FUTURE.zip");
        let bytes = self
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        if !bytes.starts_with(b"PK") {
            return Err(Error::decode(
                "SGX FUTURE.zip response is not a ZIP archive",
            ));
        }

        let (name, data) = read_first_zip_entry(&bytes, "SGX FUTURE")?;
        let text = crate::util::decode_text(&data, None, Some(encoding_rs::GB18030));
        parse_delimited_rows(&text, sgx_delimiter(&name))
    }

    /// Resolve the SGX archive sequence number for `date`.
    async fn sgx_archive_number(&self, date: &str) -> Result<i64> {
        let url = "https://push2his.eastmoney.com/api/qt/stock/kline/get";
        let body = self
            .get(url)
            .query(&[
                ("secid", "100.STI"),
                ("klt", "101"),
                ("fqt", "0"),
                ("lmt", "10000"),
                ("end", date),
                ("iscca", "1"),
                ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8"),
                (
                    "fields2",
                    "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64",
                ),
                ("ut", "f057cbcbce2a86e2866ab8877db1d059"),
                ("forcect", "1"),
            ])
            .send()
            .await?
            .text()
            .await?;

        let data: serde_json::Value = serde_json::from_str(&body)?;
        let klines = data["data"]["klines"]
            .as_array()
            .ok_or_else(|| Error::not_found("SGX archive number: FTSE kline data missing"))?;
        if klines.is_empty() {
            return Err(Error::not_found(
                "SGX archive number: FTSE kline data is empty",
            ));
        }
        Ok(klines.len() as i64 - 1 + 791)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    fn sgx_fixture_zip(name: &str, body: &str) -> Vec<u8> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        writer.start_file(name, options).unwrap();
        writer.write_all(body.as_bytes()).unwrap();
        writer.finish().unwrap().into_inner()
    }

    #[test]
    fn test_read_first_zip_entry_and_parse_csv() {
        let body = "DATE,COM,COM_MM,OPEN,CLOSE,SETTLE,VOLUME\n\
                    20240109,1MF  ,01,0,0,436.53,10\n\
                    20240109,1MG  ,02,0,0,431.81,0\n";
        let zip = sgx_fixture_zip("0109FUT.csv", body);
        let (name, data) = read_first_zip_entry(&zip, "SGX FUTURE").unwrap();
        assert_eq!(name, "0109FUT.csv");
        assert_eq!(sgx_delimiter(&name), b',');
        let rows = parse_delimited_rows(&String::from_utf8(data).unwrap(), b',').unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["DATE"], serde_json::json!(20240109));
        assert_eq!(rows[0]["COM"], serde_json::json!("1MF"));
        assert_eq!(rows[0]["SETTLE"], serde_json::json!(436.53));
        assert_eq!(rows[1]["VOLUME"], serde_json::json!(0));
    }

    #[test]
    fn test_txt_entries_use_tab_delimiter() {
        assert_eq!(sgx_delimiter("0109FUT.TXT"), b'\t');
        assert_eq!(sgx_delimiter("0109FUT.csv"), b',');
    }

    #[test]
    fn test_reject_non_zip() {
        let err = read_first_zip_entry(b"<html>blocked</html>", "SGX FUTURE").unwrap_err();
        assert!(err.to_string().contains("SGX FUTURE"));
    }
}
