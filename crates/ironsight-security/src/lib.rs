//! # IronSight Security
//!
//! Binary integrity analysis — SHA-256 hash, Shannon entropy,
//! signature verification, suspicious path detection.
//!
//! The `SecurityAudit` struct orchestrates all checks and returns
//! a unified `AuditResult` with flags and risk indicators.

pub mod audit;
pub mod entropy;
pub mod hash;
pub mod path_analysis;
pub mod signature;

pub use audit::{AuditResult, SecurityAudit};
pub use entropy::{EntropyResult, EntropyRisk};
pub use hash::HashResult;
pub use path_analysis::PathAnalysis;
pub use signature::SignatureResult;
