//! Runtime configuration — TOML-based settings for IronSight.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Default config search paths (in priority order).
const CONFIG_PATHS: &[&str] = &[
    "./ironsight.toml",
    "/etc/ironsight/config.toml",
    "~/.config/ironsight/config.toml",
];

/// Root configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub scan: ScanConfig,
    pub thresholds: ThresholdConfig,
    pub exclusions: ExclusionConfig,
    pub report: ReportConfig,
    pub watchdog: WatchdogConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    /// Scan interval in seconds.
    pub interval_secs: u64,
    /// Log level (trace, debug, info, warn, error).
    pub log_level: String,
    /// Directory for output reports.
    pub report_dir: String,
    /// Enable colored terminal output.
    pub color: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ScanConfig {
    /// Maximum processes to scan per cycle (0 = unlimited).
    pub max_processes: usize,
    /// Enable network analysis.
    pub network: bool,
    /// Enable memory analysis.
    pub memory: bool,
    /// Enable security analysis (hash, entropy, signature).
    pub security: bool,
    /// Top N threats to display.
    pub top_n: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThresholdConfig {
    /// Entropy threshold for flagging (0.0–8.0).
    pub entropy_alert: f64,
    /// CPU percentage threshold for spike detection.
    pub cpu_spike: f32,
    /// Minimum threat score for JSON export.
    pub export_min_score: f64,
    /// Auto-response enabled.
    pub auto_response: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ExclusionConfig {
    /// Process names to exclude from scanning.
    pub names: Vec<String>,
    /// Path prefixes to exclude.
    pub paths: Vec<String>,
    /// PIDs to exclude.
    pub pids: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ReportConfig {
    /// Export JSON reports for threats at or above this level.
    pub min_level: String,
    /// Pretty-print JSON (false = compact/SIEM-ready).
    pub pretty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WatchdogConfig {
    /// Enable peer watchdog.
    pub enabled: bool,
    /// Watchdog check interval in seconds.
    pub interval_secs: u64,
    /// Auto-restart on peer death.
    pub auto_restart: bool,
}

// ─── Defaults ────────────────────────────────────────────────────────────────

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            scan: ScanConfig::default(),
            thresholds: ThresholdConfig::default(),
            exclusions: ExclusionConfig::default(),
            report: ReportConfig::default(),
            watchdog: WatchdogConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            interval_secs: 5,
            log_level: "info".into(),
            report_dir: "/tmp/ironsight-reports".into(),
            color: true,
        }
    }
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            max_processes: 0,
            network: true,
            memory: true,
            security: true,
            top_n: 10,
        }
    }
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        Self {
            entropy_alert: 7.0,
            cpu_spike: 90.0,
            export_min_score: 30.0,
            auto_response: false,
        }
    }
}

impl Default for ExclusionConfig {
    fn default() -> Self {
        Self {
            names: vec![
                "systemd".into(),
                "init".into(),
                "sshd".into(),
                "dbus-daemon".into(),
            ],
            paths: vec!["/usr/lib/systemd/".into(), "/usr/sbin/".into()],
            pids: vec![1],
        }
    }
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            min_level: "Medium".into(),
            pretty: true,
        }
    }
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_secs: 10,
            auto_restart: true,
        }
    }
}

// ─── Loading ─────────────────────────────────────────────────────────────────

impl Config {
    /// Load config from a specific file path.
    pub fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::ReadError(path.to_path_buf(), e))?;
        let config: Config =
            toml::from_str(&content).map_err(|e| ConfigError::ParseError(path.to_path_buf(), e))?;
        Ok(config)
    }

    /// Auto-discover config from default paths, falling back to defaults.
    pub fn discover() -> Self {
        for path_str in CONFIG_PATHS {
            let expanded = shellexpand(path_str);
            let path = Path::new(&expanded);
            if path.exists() {
                match Self::from_file(path) {
                    Ok(config) => {
                        tracing::info!("Loaded config from {}", path.display());
                        return config;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load {}: {e}", path.display());
                    }
                }
            }
        }
        tracing::info!("No config file found, using defaults");
        Self::default()
    }

    /// Generate a default config file as TOML string.
    pub fn default_toml() -> String {
        toml::to_string_pretty(&Config::default()).unwrap_or_default()
    }
}

/// Minimal ~ expansion.
fn shellexpand(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{home}/{rest}");
        }
    }
    path.to_string()
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Cannot read config file {0}: {1}")]
    ReadError(PathBuf, std::io::Error),
    #[error("Invalid TOML in {0}: {1}")]
    ParseError(PathBuf, toml::de::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_sane() {
        let c = Config::default();
        assert_eq!(c.general.interval_secs, 5);
        assert_eq!(c.scan.top_n, 10);
        assert!(c.thresholds.entropy_alert > 6.0);
        assert!(!c.exclusions.names.is_empty());
    }

    #[test]
    fn parse_toml_string() {
        let toml_str = r#"
[general]
interval_secs = 10
log_level = "debug"

[scan]
network = false
top_n = 20

[thresholds]
entropy_alert = 7.5
auto_response = true

[exclusions]
names = ["code", "rust-analyzer"]
pids = [1, 2]
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.general.interval_secs, 10);
        assert_eq!(config.general.log_level, "debug");
        assert!(!config.scan.network);
        assert_eq!(config.scan.top_n, 20);
        assert_eq!(config.thresholds.entropy_alert, 7.5);
        assert!(config.thresholds.auto_response);
        assert_eq!(config.exclusions.names, vec!["code", "rust-analyzer"]);
    }

    #[test]
    fn partial_toml_uses_defaults() {
        let toml_str = r#"
[general]
interval_secs = 30
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.general.interval_secs, 30);
        // Rest should be defaults
        assert_eq!(config.scan.top_n, 10);
        assert!(config.scan.network);
    }

    #[test]
    fn default_toml_roundtrips() {
        let toml_str = Config::default_toml();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.general.interval_secs, 5);
    }

    #[test]
    fn discover_returns_defaults_when_no_file() {
        let config = Config::discover();
        assert_eq!(config.general.interval_secs, 5);
    }

    #[test]
    fn shellexpand_tilde() {
        let expanded = shellexpand("~/test/path");
        assert!(!expanded.starts_with("~/") || std::env::var("HOME").is_err());
    }
}
