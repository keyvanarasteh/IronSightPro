//! Error types for IronSight.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SuiteError {
    #[error("ProcessSpy error: {0}")]
    ProcessSpy(String),

    #[error("Process not found: PID {0}")]
    ProcessNotFound(u32),

    #[error("Process action failed: {0}")]
    ProcessAction(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Security error: {0}")]
    Security(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Memory error: {0}")]
    Memory(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Insufficient privileges: {0}")]
    Privilege(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SuiteError>;
