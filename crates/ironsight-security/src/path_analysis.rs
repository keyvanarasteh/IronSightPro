//! Suspicious path detection — flag processes running from known staging areas.

use std::path::Path;

/// Known suspicious directories where malware stages payloads.
const SUSPICIOUS_DIRS_UNIX: &[&str] = &[
    "/tmp/",
    "/var/tmp/",
    "/dev/shm/",
    "/run/user/",
];

const SUSPICIOUS_DIRS_WINDOWS: &[&str] = &[
    r"AppData\Local\Temp",
    r"AppData\Roaming\Temp",
    r"Windows\Temp",
    r"Users\Public\Downloads",
];

/// Result of path analysis.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PathAnalysis {
    pub is_suspicious: bool,
    pub reason: Option<String>,
    pub path: String,
}

/// Analyze whether a binary path is suspicious.
pub fn analyze_path(exe_path: Option<&Path>) -> PathAnalysis {
    let Some(path) = exe_path else {
        return PathAnalysis {
            is_suspicious: true,
            reason: Some("No executable path — potential fileless process".into()),
            path: "(empty)".into(),
        };
    };

    let path_str = path.to_string_lossy();
    let path_lower = path_str.to_lowercase();

    // Check Unix suspicious dirs
    for dir in SUSPICIOUS_DIRS_UNIX {
        if path_lower.contains(&dir.to_lowercase()) {
            return PathAnalysis {
                is_suspicious: true,
                reason: Some(format!("Binary running from staging area: {dir}")),
                path: path_str.into_owned(),
            };
        }
    }

    // Check Windows suspicious dirs
    for dir in SUSPICIOUS_DIRS_WINDOWS {
        if path_lower.contains(&dir.to_lowercase()) {
            return PathAnalysis {
                is_suspicious: true,
                reason: Some(format!("Binary running from staging area: {dir}")),
                path: path_str.into_owned(),
            };
        }
    }

    // Check if running from Downloads
    if path_lower.contains("downloads") || path_lower.contains("desktop") {
        return PathAnalysis {
            is_suspicious: true,
            reason: Some("Binary running from user download/desktop area".into()),
            path: path_str.into_owned(),
        };
    }

    PathAnalysis {
        is_suspicious: false,
        reason: None,
        path: path_str.into_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn detects_tmp_linux() {
        let path = PathBuf::from("/tmp/evil_binary");
        let result = analyze_path(Some(&path));
        assert!(result.is_suspicious);
        assert!(result.reason.unwrap().contains("/tmp/"));
    }

    #[test]
    fn detects_dev_shm() {
        let path = PathBuf::from("/dev/shm/payload.so");
        let result = analyze_path(Some(&path));
        assert!(result.is_suspicious);
    }

    #[test]
    fn detects_downloads() {
        let path = PathBuf::from("/home/user/Downloads/suspicious.bin");
        let result = analyze_path(Some(&path));
        assert!(result.is_suspicious);
        assert!(result.reason.unwrap().contains("download"));
    }

    #[test]
    fn detects_no_exe_path() {
        let result = analyze_path(None);
        assert!(result.is_suspicious);
        assert!(result.reason.unwrap().contains("fileless"));
    }

    #[test]
    fn normal_path_is_safe() {
        let path = PathBuf::from("/usr/bin/ls");
        let result = analyze_path(Some(&path));
        assert!(!result.is_suspicious);
        assert!(result.reason.is_none());
    }

    #[test]
    fn normal_opt_path_is_safe() {
        let path = PathBuf::from("/opt/myapp/bin/server");
        let result = analyze_path(Some(&path));
        assert!(!result.is_suspicious);
    }
}
