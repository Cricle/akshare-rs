//! Stock fundamental data: financial reports, IPO, shareholders, restricted
//! releases, profit forecasts, and more from Eastmoney, Sina, and THS.

pub mod eastmoney;
pub mod sina;
pub mod ths;

// Re-export all public methods via AkShareClient impl blocks in sub-modules.
