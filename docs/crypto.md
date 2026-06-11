# Crypto Module

The `crypto` module provides cryptocurrency data.

## Functions (4 total)

- **crypto_bitcoin_cme** — CME Bitcoin futures
- **crypto_bitcoin_hold_report** — Bitcoin hold report
- **crypto_js_spot** — Crypto spot from JS

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// Bitcoin CME data
let btc = client.crypto_bitcoin_cme().await?;
```
