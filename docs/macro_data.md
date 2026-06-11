# Macro Data Module

The `macro_data` module provides economic indicators and macro data from around the world.

## Functions (423 total)

### China (macro_china_*)
- **macro_china_gdp** — GDP data
- **macro_china_cpi** — CPI data
- **macro_china_ppi** — PPI data
- **macro_china_pmi** — PMI data
- **macro_china_money_supply** — Money supply
- **macro_china_cx_pmi** — Caixin PMI
- **macro_china_cx_services_pmi** — Caixin Services PMI
- **macro_china_non_man_pmi** — Non-manufacturing PMI
- **macro_china_shrzgm** — Social financing
- **macro_china_lpr** — LPR rates
- **macro_china_swap_rate** — Swap rates
- **macro_china_bond_public** — Bond issuance
- **macro_china_new_financial_credit** — New loans
- **macro_china_urban_unemployment** — Unemployment
- **macro_china_nbs_region** — Regional data
- ... and many more

### US (macro_usa_*)
- **macro_usa_gdp** — US GDP
- **macro_usa_cpi** — US CPI
- **macro_usa_pmi** — US PMI
- **macro_usa_ism_pmi** — ISM PMI
- **macro_usa_non_farm** — Non-farm payrolls
- **macro_usa_unemployment_rate** — Unemployment
- **macro_usa_interest_rate** — Fed interest rate
- **macro_usa_adp_employment** — ADP employment
- ... and many more

### EU (macro_euro_*)
- **macro_euro_gdp** — EU GDP
- **macro_euro_cpi** — EU CPI
- **macro_euro_pmi** — EU PMI
- ... and more

### Japan (macro_japan_*)
- **macro_japan_gdp** — Japan GDP
- **macro_japan_cpi** — Japan CPI
- ... and more

### Australia (macro_australia_*)
- **macro_australia_gdp** — Australia GDP
- **macro_australia_cpi** — Australia CPI
- ... and more

### UK (macro_uk_*)
- **macro_uk_gdp** — UK GDP
- **macro_uk_cpi** — UK CPI
- ... and more

### Other Countries
- **macro_canada_*** — Canada data
- **macro_germany_*** — Germany data
- **macro_swiss_*** — Switzerland data
- **macro_hk_*** — Hong Kong data

### Bank Rates
- **macro_bank_china_interest_rate** — China bank rates
- **macro_bank_usa_interest_rate** — US bank rates
- **macro_bank_euro_interest_rate** — EU bank rates
- **macro_bank_japan_interest_rate** — Japan bank rates
- **macro_bank_uk_interest_rate** — UK bank rates
- **macro_bank_australia_interest_rate** — Australia bank rates
- **macro_bank_brazil_interest_rate** — Brazil bank rates

### Interest Rates
- **macro_china_shibor_all** — SHIBOR rates
- **macro_china_hibor_all** — HIBOR rates

## Usage Examples

```rust
use akshare::AkShareClient;

let client = AkShareClient::new();

// China GDP
let gdp = client.macro_china_gdp().await?;

// US CPI
let cpi = client.macro_china_cpi().await?;

// China LPR rates
let lpr = client.macro_china_lpr().await?;
```
