//! Privilege escalation and capability checks.
//!
//! IronSight needs elevated privileges to:
//! - Read `/proc/<pid>/mem` (memory analysis)
//! - Send signals to other processes (response handler)
//! - Read `/proc/<pid>/maps` and `/proc/net/tcp` (network/memory)

use serde::{Deserialize, Serialize};

/// Result of privilege checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeReport {
    pub is_root: bool,
    pub effective_uid: u32,
    pub capabilities: Vec<CapabilityCheck>,
    pub overall_level: PrivilegeLevel,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivilegeLevel {
    /// Root / full admin — all features available.
    Full,
    /// Partial — some features may be degraded.
    Partial,
    /// Unprivileged — limited to own processes only.
    Limited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityCheck {
    pub name: String,
    pub description: String,
    pub available: bool,
    pub required_for: String,
}

impl PrivilegeReport {
    /// Run all privilege checks for the current platform.
    pub fn check() -> Self {
        #[cfg(unix)]
        {
            check_unix()
        }
        #[cfg(not(unix))]
        {
            PrivilegeReport {
                is_root: false,
                effective_uid: 0,
                capabilities: vec![],
                overall_level: PrivilegeLevel::Limited,
                warnings: vec!["Privilege checks not implemented for this platform".into()],
            }
        }
    }

    /// Print a human-readable summary.
    pub fn display(&self) {
        let level_icon = match self.overall_level {
            PrivilegeLevel::Full => "🟢",
            PrivilegeLevel::Partial => "🟡",
            PrivilegeLevel::Limited => "🔴",
        };

        println!("── Privilege Status ────────────────────────────────────────────");
        println!(
            "  Level:  {level_icon} {:?} (UID: {})",
            self.overall_level, self.effective_uid
        );
        println!(
            "  Root:   {}",
            if self.is_root { "✅ Yes" } else { "❌ No" }
        );
        println!();

        for cap in &self.capabilities {
            let icon = if cap.available { "✅" } else { "❌" };
            println!("  {icon} {} — {}", cap.name, cap.description);
            if !cap.available {
                println!("      Required for: {}", cap.required_for);
            }
        }

        if !self.warnings.is_empty() {
            println!();
            for w in &self.warnings {
                println!("  ⚠ {w}");
            }
        }
        println!();
    }
}

// ─── Unix Implementation ─────────────────────────────────────────────────────

#[cfg(unix)]
fn check_unix() -> PrivilegeReport {
    let euid = nix::unistd::geteuid().as_raw();
    let is_root = euid == 0;
    let mut warnings = Vec::new();
    let mut capabilities = Vec::new();

    // Check /proc readability (proxy for CAP_SYS_PTRACE)
    let can_read_proc_mem = check_proc_mem_access();
    capabilities.push(CapabilityCheck {
        name: "CAP_SYS_PTRACE".into(),
        description: "Read other processes' memory via /proc/<pid>/mem".into(),
        available: can_read_proc_mem || is_root,
        required_for: "Memory analysis, pattern scanning".into(),
    });

    // Check signal sending (proxy for CAP_KILL)
    capabilities.push(CapabilityCheck {
        name: "CAP_KILL".into(),
        description: "Send signals (SIGSTOP/SIGKILL) to other processes".into(),
        available: is_root,
        required_for: "Response handler (suspend/kill)".into(),
    });

    // Check /proc/net/tcp readability
    let can_read_net = std::fs::read_to_string("/proc/net/tcp").is_ok();
    capabilities.push(CapabilityCheck {
        name: "NET_READ".into(),
        description: "Read /proc/net/tcp for socket enumeration".into(),
        available: can_read_net,
        required_for: "Network analysis".into(),
    });

    // Check /proc/<pid>/maps readability for PID 1
    let can_read_maps = std::fs::read_to_string("/proc/1/maps").is_ok();
    capabilities.push(CapabilityCheck {
        name: "PROC_MAPS".into(),
        description: "Read /proc/<pid>/maps for memory region enumeration".into(),
        available: can_read_maps || is_root,
        required_for: "Memory watcher, W^X detection".into(),
    });

    if !is_root {
        warnings.push("Running without root — memory analysis and response actions will be limited".into());
        warnings.push("Run with: sudo ironsight  or  sudo setcap cap_sys_ptrace+ep ./ironsight".into());
    }

    let available_count = capabilities.iter().filter(|c| c.available).count();
    let overall_level = if is_root || available_count == capabilities.len() {
        PrivilegeLevel::Full
    } else if available_count > 0 {
        PrivilegeLevel::Partial
    } else {
        PrivilegeLevel::Limited
    };

    PrivilegeReport {
        is_root,
        effective_uid: euid,
        capabilities,
        overall_level,
        warnings,
    }
}

/// Try reading /proc/self/mem as a proxy for ptrace capability.
#[cfg(unix)]
fn check_proc_mem_access() -> bool {
    // Try reading a non-own process mem — use PID 1 as test
    std::fs::metadata("/proc/1/mem")
        .map(|m| !m.permissions().readonly() || nix::unistd::geteuid().as_raw() == 0)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn privilege_check_runs() {
        let report = PrivilegeReport::check();
        // Should at least have some capabilities listed
        assert!(!report.capabilities.is_empty());
    }

    #[test]
    fn privilege_level_ordering() {
        // Basic enum validation
        assert_ne!(PrivilegeLevel::Full, PrivilegeLevel::Limited);
        assert_ne!(PrivilegeLevel::Partial, PrivilegeLevel::Full);
    }

    #[test]
    fn net_read_usually_available() {
        // /proc/net/tcp is world-readable on most Linux systems
        let report = PrivilegeReport::check();
        let net = report.capabilities.iter().find(|c| c.name == "NET_READ");
        if let Some(cap) = net {
            // Should be readable for any user
            assert!(cap.available, "/proc/net/tcp should be world-readable");
        }
    }
}
