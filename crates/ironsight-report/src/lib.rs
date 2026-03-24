//! # IronSight Report
//!
//! SIEM-ready incident reporting — structured JSON and human-readable text output.

pub mod formatter;
pub mod incident;

pub use formatter::{save_json, to_json, to_json_compact, to_text};
pub use incident::IncidentReport;
