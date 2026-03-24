//! # IronSight Core
//!
//! Temel process monitoring modülü — snapshot, diff, tree, filter, events.
//!
//! Bu crate, tüm IronSight ekosisteminin temeli.
//! Diğer crate'ler (security, memory, heuristic) bu crate'in üzerine inşa edilir.

pub mod diff;
pub mod error;
pub mod filter;
pub mod process_info;
pub mod snapshot;
pub mod spy;
pub mod system_info;

pub use error::{Result, SuiteError};
pub use process_info::{ProcStatus, ProcessInfo};
pub use snapshot::ProcessSnapshot;
pub use diff::{ProcessChange, ProcessDiff};
pub use filter::ProcessFilter;
pub use spy::{ProcessSpy, SpyEvent};
pub use system_info::SystemInfo;

#[cfg(test)]
mod tests;
