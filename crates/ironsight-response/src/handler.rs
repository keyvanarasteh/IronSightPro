//! Response orchestrator — executes the forensic sequence based on threat level.
//!
//! **Forensic Order:** Suspend → Dump → Kill
//!
//! This ensures we preserve evidence before destroying the process.

use serde::{Deserialize, Serialize};

use crate::actions::{self, ActionResult, ActionType};
use crate::exclusions::ExclusionList;

/// A complete response log for an incident.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseLog {
    pub pid: u32,
    pub process_name: String,
    pub threat_score: f64,
    pub actions_taken: Vec<ActionResult>,
    pub was_excluded: bool,
}

/// The response handler orchestrates automated actions.
pub struct ResponseHandler {
    exclusions: ExclusionList,
    dump_dir: String,
    /// When true, log actions but don't actually send signals (STEP 5).
    pub dry_run: bool,
}

impl ResponseHandler {
    pub fn new(dump_dir: &str) -> Self {
        ResponseHandler {
            exclusions: ExclusionList::default(),
            dump_dir: dump_dir.to_string(),
            dry_run: false,
        }
    }

    pub fn with_exclusions(mut self, exclusions: ExclusionList) -> Self {
        self.exclusions = exclusions;
        self
    }

    /// Execute response based on recommended action.
    pub fn respond(
        &self,
        pid: u32,
        process_name: &str,
        threat_score: f64,
        recommended: crate::actions::ActionType,
    ) -> ResponseLog {
        let mut log = ResponseLog {
            pid,
            process_name: process_name.to_string(),
            threat_score,
            actions_taken: Vec::new(),
            was_excluded: false,
        };

        // Check exclusion list
        if self.exclusions.is_excluded(process_name, pid) {
            tracing::info!(
                pid,
                process_name,
                "Process is in exclusion list — skipping response"
            );
            log.was_excluded = true;
            log.actions_taken.push(ActionResult {
                pid,
                action: ActionType::LogOnly,
                success: true,
                message: format!("Excluded: {process_name} (PID {pid})"),
                timestamp: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            });
            return log;
        }

        match recommended {
            ActionType::LogOnly => {
                tracing::info!(pid, process_name, threat_score, "Threat logged (no action)");
            }
            ActionType::Suspend => {
                // Suspend only
                let result = actions::suspend(pid);
                tracing::warn!(pid, process_name, "Process suspended");
                log.actions_taken.push(result);
            }
            ActionType::Kill => {
                // Full forensic sequence: Suspend → Dump → Kill
                let suspend_result = actions::suspend(pid);
                log.actions_taken.push(suspend_result);

                let dump_result = actions::dump_memory(pid, &self.dump_dir);
                log.actions_taken.push(dump_result);

                let kill_result = actions::kill_process(pid);
                tracing::error!(pid, process_name, threat_score, "Process killed after forensic dump");
                log.actions_taken.push(kill_result);
            }
            _ => {}
        }

        log
    }

    /// Execute the full forensic sequence: Suspend → Dump → Kill.
    pub fn forensic_kill(&self, pid: u32, process_name: &str) -> ResponseLog {
        let mut log = ResponseLog {
            pid,
            process_name: process_name.to_string(),
            threat_score: 100.0,
            actions_taken: Vec::new(),
            was_excluded: false,
        };

        tracing::warn!(pid, process_name, "Starting forensic kill sequence");

        // Step 1: Suspend — freeze the process
        let suspend = actions::suspend(pid);
        log.actions_taken.push(suspend);

        // Step 2: Dump — capture memory for analysis
        let dump = actions::dump_memory(pid, &self.dump_dir);
        log.actions_taken.push(dump);

        // Step 3: Kill — terminate the process
        let kill = actions::kill_process(pid);
        log.actions_taken.push(kill);

        log
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclusion_skips_response() {
        let mut exclusions = ExclusionList::default();
        exclusions.add_name("systemd");

        let handler = ResponseHandler::new("/tmp/dumps").with_exclusions(exclusions);
        let log = handler.respond(1, "systemd", 80.0, ActionType::Kill);

        assert!(log.was_excluded);
        assert_eq!(log.actions_taken.len(), 1);
        assert_eq!(log.actions_taken[0].action, ActionType::LogOnly);
    }

    #[test]
    fn log_only_action() {
        let handler = ResponseHandler::new("/tmp/dumps");
        let log = handler.respond(100, "normal_app", 5.0, ActionType::LogOnly);
        assert!(!log.was_excluded);
        assert!(log.actions_taken.is_empty()); // LogOnly just logs, no action result
    }

    #[test]
    fn suspend_action() {
        let handler = ResponseHandler::new("/tmp/dumps");
        // This will fail on non-existent PID, but the action should be attempted
        let log = handler.respond(999999, "test_proc", 60.0, ActionType::Suspend);
        assert_eq!(log.actions_taken.len(), 1);
        assert_eq!(log.actions_taken[0].action, ActionType::Suspend);
    }
}
