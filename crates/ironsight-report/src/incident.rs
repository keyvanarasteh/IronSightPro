//! Incident report — structured data model for a full security assessment.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A complete incident report for a process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentReport {
    /// Schema version for SIEM parsing
    pub schema_version: String,
    /// Unique identifier for this incident report
    pub id: Uuid,
    /// When the incident was generated
    pub timestamp: DateTime<Utc>,
    pub hostname: String,
    pub process: ProcessInfo,
    pub threat: ThreatInfo,
    pub security: SecurityInfo,
    pub network: NetworkInfo,
    pub memory: MemoryInfo,
    pub actions: Vec<ActionInfo>,
}

impl IncidentReport {
    pub fn new() -> Self {
        IncidentReport {
            schema_version: "1.0.0".to_string(),
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            hostname: whoami::hostname().unwrap_or_else(|_| "Unknown".to_string()),
            process: ProcessInfo::default(),
            threat: ThreatInfo::default(),
            security: SecurityInfo::default(),
            network: NetworkInfo::default(),
            memory: MemoryInfo::default(),
            actions: Vec::new(),
        }
    }
}

impl Default for IncidentReport {
    fn default() -> Self {
        Self {
            schema_version: "1.0.0".to_string(),
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            hostname: whoami::hostname().unwrap_or_else(|_| "Unknown".to_string()),
            process: ProcessInfo::default(),
            threat: ThreatInfo::default(),
            security: SecurityInfo::default(),
            network: NetworkInfo::default(),
            memory: MemoryInfo::default(),
            actions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub exe_path: Option<String>,
    pub cmdline: Option<String>,
    pub parent_pid: Option<u32>,
    pub user: Option<String>,
    pub cpu_percent: f32,
    pub memory_bytes: u64,
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Default)]
pub enum ThreatLevel {
    #[default]
    Clean,
    Low,
    Medium,
    High,
    Critical,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Default)]
pub enum RecommendedAction {
    #[default]
    None,
    Log,
    Monitor,
    Suspend,
    SuspendDumpKill,
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThreatInfo {
    pub score: f64,
    pub level: ThreatLevel,
    pub signals: Vec<SignalInfo>,
    pub recommended_action: Box<RecommendedAction>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignalInfo {
    pub name: String,
    pub category: String,
    pub weight: f64,
    pub description: String,
    pub evidence: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityInfo {
    pub sha256: Option<String>,
    pub entropy: Option<f64>,
    pub entropy_risk: Option<String>,
    pub is_signed: Option<bool>,
    pub suspicious_path: bool,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub total_sockets: usize,
    pub listeners: usize,
    pub external_connections: usize,
    pub suspicious_connections: usize,
    pub suspicious_ports: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_regions: usize,
    pub wx_violations: usize,
    pub anonymous_executable: usize,
    pub pattern_matches: usize,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionInfo {
    pub action_type: String,
    pub success: bool,
    pub message: String,
    pub timestamp: String,
}

impl From<&ironsight_core::ProcessInfo> for ProcessInfo {
    fn from(p: &ironsight_core::ProcessInfo) -> Self {
        ProcessInfo {
            pid: p.pid,
            name: p.name.clone(),
            exe_path: p.exe.as_ref().map(|x| x.to_string_lossy().to_string()),
            cmdline: if p.cmd.is_empty() { None } else { Some(p.cmd.join(" ")) },
            parent_pid: p.parent_pid,
            user: p.uid.map(|x| x.to_string()),
            cpu_percent: p.cpu_percent,
            memory_bytes: p.memory_bytes,
            start_time: None,
        }
    }
}
