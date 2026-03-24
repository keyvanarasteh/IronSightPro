//! # IronSight Memory
//!
//! Memory forensics — region mapping, pattern scanning, W^X detection,
//! permission change tracking, and suspicious content analysis.

pub mod maps;
pub mod scanner;
pub mod watcher;

pub use maps::{MemoryRegion, MemorySummary, Permissions};
pub use scanner::{PatternMatch, ScanResult, SuspiciousPatterns, entropy};
pub use watcher::MemoryChange;
