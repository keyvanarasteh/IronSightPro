//! SecurityAudit — orchestrates all security checks for a single process.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::entropy::{EntropyResult, EntropyRisk};
use crate::hash::HashResult;
use crate::path_analysis::PathAnalysis;
use crate::signature::SignatureResult;

// ─────────────────────────────────────────────────────────────────────────────
// AuditResult
// ─────────────────────────────────────────────────────────────────────────────

/// Complete security audit result for a single process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    pub pid: u32,
    pub name: String,
    pub exe_path: Option<PathBuf>,
    pub hash: Option<HashResult>,
    pub entropy: Option<EntropyResult>,
    pub path_analysis: PathAnalysis,
    pub signature: Option<SignatureResult>,
    /// Number of security flags raised (0 = clean).
    pub flag_count: u32,
    /// Human-readable summary of issues found.
    pub flags: Vec<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// SecurityAudit
// ─────────────────────────────────────────────────────────────────────────────

/// Orchestrator that runs all security checks on a process.
pub struct SecurityAudit;

impl SecurityAudit {
    /// Run full audit on a process given its PID, name, and exe path.
    pub fn audit(pid: u32, name: &str, exe_path: Option<&PathBuf>) -> AuditResult {
        let mut flags: Vec<String> = Vec::new();

        // 1. Path analysis (works even without exe)
        let path_analysis = crate::path_analysis::analyze_path(exe_path.map(|p| p.as_path()));
        if path_analysis.is_suspicious {
            if let Some(ref reason) = path_analysis.reason {
                flags.push(reason.clone());
            }
        }

        // Only proceed with file-based checks if exe exists
        let (hash, entropy, signature) = if let Some(path) = exe_path {
            if path.exists() {
                let h = crate::hash::compute_sha256(path).ok();
                let e = crate::entropy::compute_entropy(path).ok();
                let s = Some(crate::signature::verify_signature(path));

                // Flag high entropy
                if let Some(ref ent) = e {
                    match ent.risk_level {
                        EntropyRisk::High => {
                            flags.push(format!(
                                "High entropy ({:.2}) — possibly packed binary",
                                ent.entropy
                            ));
                        }
                        EntropyRisk::Critical => {
                            flags.push(format!(
                                "Critical entropy ({:.2}) — likely encrypted/packed",
                                ent.entropy
                            ));
                        }
                        _ => {}
                    }
                }

                // Flag unsigned binary
                if let Some(ref sig) = s {
                    if sig.is_signed == Some(false) {
                        flags.push("Unsigned binary".into());
                    }
                }

                (h, e, s)
            } else {
                flags.push("Exe path exists but file not found on disk".into());
                (None, None, None)
            }
        } else {
            // No exe path = potentially fileless
            (None, None, None)
        };

        AuditResult {
            pid,
            name: name.to_string(),
            exe_path: exe_path.cloned(),
            hash,
            entropy,
            path_analysis,
            signature,
            flag_count: flags.len() as u32,
            flags,
        }
    }
}
