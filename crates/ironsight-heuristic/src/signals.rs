//! Signal definitions — individual security indicators with weights.

use serde::{Deserialize, Serialize};

/// A single security signal contributing to the threat score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    pub category: SignalCategory,
    pub weight: f64,
    pub description: String,
    pub evidence: Option<String>,
}

/// Categories of security signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignalCategory {
    /// Binary integrity (hash, entropy, signature)
    StaticAnalysis,
    /// Path-based suspicion
    PathAnalysis,
    /// Network behavior
    NetworkBehavior,
    /// Memory anomalies
    MemoryAnomaly,
    /// Process behavior (CPU spike, rapid spawning)
    ProcessBehavior,
}

impl Signal {
    /// Create a new signal. Weight is clamped to 0.0..=100.0 (STEP 3).
    pub fn new(name: &str, category: SignalCategory, weight: f64, description: &str) -> Self {
        Signal {
            name: name.to_string(),
            category,
            weight: weight.clamp(0.0, 100.0),
            description: description.to_string(),
            evidence: None,
        }
    }

    pub fn with_evidence(mut self, evidence: &str) -> Self {
        self.evidence = Some(evidence.to_string());
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pre-built signal factories
// ─────────────────────────────────────────────────────────────────────────────

pub fn high_entropy(entropy: f64) -> Signal {
    Signal::new(
        "HIGH_ENTROPY",
        SignalCategory::StaticAnalysis,
        if entropy > 7.5 { 30.0 } else { 15.0 },
        "Binary has unusually high entropy — possibly packed/encrypted",
    )
    .with_evidence(&format!("Entropy: {entropy:.2}"))
}

pub fn suspicious_path(path: &str, reason: &str) -> Signal {
    Signal::new(
        "SUSPICIOUS_PATH",
        SignalCategory::PathAnalysis,
        20.0,
        reason,
    )
    .with_evidence(path)
}

pub fn unsigned_binary() -> Signal {
    Signal::new(
        "UNSIGNED_BINARY",
        SignalCategory::StaticAnalysis,
        10.0,
        "Binary is not signed by a trusted authority",
    )
}

pub fn suspicious_port(port: u16, service: &str) -> Signal {
    Signal::new(
        "SUSPICIOUS_PORT",
        SignalCategory::NetworkBehavior,
        25.0,
        &format!("Connection to known malicious port: {port} ({service})"),
    )
    .with_evidence(&format!("Port {port}"))
}

pub fn wx_violation(count: usize) -> Signal {
    Signal::new(
        "WX_VIOLATION",
        SignalCategory::MemoryAnomaly,
        25.0,
        &format!("{count} memory region(s) are both writable and executable"),
    )
}

pub fn anonymous_executable(count: usize) -> Signal {
    Signal::new(
        "ANONYMOUS_EXEC",
        SignalCategory::MemoryAnomaly,
        30.0,
        &format!("{count} anonymous executable region(s) — possible shellcode"),
    )
}

pub fn memory_pattern_match(pattern: &str) -> Signal {
    Signal::new(
        "MEMORY_PATTERN",
        SignalCategory::MemoryAnomaly,
        20.0,
        "Suspicious pattern found in process memory",
    )
    .with_evidence(pattern)
}

pub fn fileless_process() -> Signal {
    Signal::new(
        "FILELESS_PROCESS",
        SignalCategory::ProcessBehavior,
        25.0,
        "Process has no executable path — potential fileless attack",
    )
}

pub fn cpu_spike(cpu: f32) -> Signal {
    Signal::new(
        "CPU_SPIKE",
        SignalCategory::ProcessBehavior,
        10.0,
        &format!("CPU usage abnormally high: {cpu:.1}%"),
    )
}

pub fn external_connections(count: usize) -> Signal {
    Signal::new(
        "EXTERNAL_CONNECTIONS",
        SignalCategory::NetworkBehavior,
        if count > 10 { 15.0 } else { 5.0 },
        &format!("{count} active external connections"),
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests — STEP 3
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_clamped_negative() {
        let s = Signal::new("test", SignalCategory::ProcessBehavior, -10.0, "");
        assert_eq!(s.weight, 0.0);
    }

    #[test]
    fn weight_clamped_excess() {
        let s = Signal::new("test", SignalCategory::ProcessBehavior, 200.0, "");
        assert_eq!(s.weight, 100.0);
    }

    #[test]
    fn weight_normal_passthrough() {
        let s = Signal::new("test", SignalCategory::ProcessBehavior, 42.0, "");
        assert_eq!(s.weight, 42.0);
    }
}
