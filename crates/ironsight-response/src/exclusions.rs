//! Exclusion list — processes that should never be automatically responded to.

use serde::{Deserialize, Serialize};

/// Exclusion list to protect critical system processes from automated response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExclusionList {
    /// Process names to exclude (exact match).
    pub names: Vec<String>,
    /// PIDs to exclude.
    pub pids: Vec<u32>,
    /// Path prefixes to exclude (e.g., "/usr/sbin/").
    pub path_prefixes: Vec<String>,
}

impl ExclusionList {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an exclusion list with common system processes pre-populated.
    pub fn system_defaults() -> Self {
        ExclusionList {
            names: vec![
                "init".into(),
                "systemd".into(),
                "systemd-journald".into(),
                "systemd-logind".into(),
                "systemd-udevd".into(),
                "sshd".into(),
                "dbus-daemon".into(),
                "NetworkManager".into(),
                "cron".into(),
                "rsyslogd".into(),
                "kthreadd".into(),
            ],
            pids: vec![1], // PID 1 = init/systemd — never kill
            path_prefixes: vec![
                "/usr/lib/systemd/".into(),
                "/usr/sbin/".into(),
            ],
        }
    }

    /// Check if a process should be excluded from automated response.
    pub fn is_excluded(&self, name: &str, pid: u32) -> bool {
        if self.pids.contains(&pid) {
            return true;
        }
        if self.names.iter().any(|n| n == name) {
            return true;
        }
        false
    }

    /// Check if a process path is excluded.
    pub fn is_path_excluded(&self, path: &str) -> bool {
        self.path_prefixes.iter().any(|p| path.starts_with(p))
    }

    pub fn add_name(&mut self, name: &str) {
        self.names.push(name.to_string());
    }

    pub fn add_pid(&mut self, pid: u32) {
        self.pids.push(pid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_defaults_protect_init() {
        let list = ExclusionList::system_defaults();
        assert!(list.is_excluded("systemd", 1));
        assert!(list.is_excluded("init", 99));
        assert!(list.is_excluded("sshd", 500));
    }

    #[test]
    fn custom_exclusion() {
        let mut list = ExclusionList::new();
        list.add_name("my_service");
        list.add_pid(42);

        assert!(list.is_excluded("my_service", 100));
        assert!(list.is_excluded("unknown", 42));
        assert!(!list.is_excluded("malware", 999));
    }

    #[test]
    fn path_exclusion() {
        let list = ExclusionList::system_defaults();
        assert!(list.is_path_excluded("/usr/sbin/sshd"));
        assert!(list.is_path_excluded("/usr/lib/systemd/systemd-logind"));
        assert!(!list.is_path_excluded("/tmp/evil"));
    }
}
