# Bank Module

Banking supervision data from CBIRC (China Banking and Insurance Regulatory Commission).

## Functions

- **bank_fjcf_table_detail** — 银保监会行政处罚详情

Fetches detailed penalty/sanction records from the CBIRC website.

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();
let data = client.bank_fjcf_table_detail().await?;
```
