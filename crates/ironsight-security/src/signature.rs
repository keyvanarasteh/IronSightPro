//! Binary signature verification (placeholder for cross-platform).
//!
//! - Windows: Authenticode signature verification
//! - macOS: Developer ID / notarization
//! - Linux: PGP / distro signature (best-effort)

use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignatureResult {
    pub is_signed: Option<bool>,
    pub signer: Option<String>,
    pub platform_note: String,
}

/// Check if a binary is signed (platform-specific).
pub fn verify_signature(path: &Path) -> SignatureResult {
    #[cfg(target_os = "linux")]
    {
        // Linux: No universal code signing. Check if file is from a known package.
        let path_str = path.to_string_lossy();
        let is_system = path_str.starts_with("/usr/")
            || path_str.starts_with("/bin/")
            || path_str.starts_with("/sbin/");

        SignatureResult {
            is_signed: Some(is_system),
            signer: if is_system {
                Some("System package (heuristic)".into())
            } else {
                None
            },
            platform_note: "Linux: Using path-based heuristic (system paths = trusted)".into(),
        }
    }

    #[cfg(target_os = "windows")]
    {
        // TODO: Implement Authenticode verification via cross-authenticode crate
        SignatureResult {
            is_signed: None,
            signer: None,
            platform_note: "Windows: Authenticode verification not yet implemented".into(),
        }
    }

    #[cfg(target_os = "macos")]
    {
        // TODO: Check codesign via command-line invocation
        SignatureResult {
            is_signed: None,
            signer: None,
            platform_note: "macOS: Code signing verification not yet implemented".into(),
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        SignatureResult {
            is_signed: None,
            signer: None,
            platform_note: "Unsupported platform for signature verification".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[cfg(target_os = "linux")]
    #[test]
    fn system_binary_is_trusted() {
        let result = verify_signature(&PathBuf::from("/usr/bin/ls"));
        assert_eq!(result.is_signed, Some(true));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn tmp_binary_is_untrusted() {
        let result = verify_signature(&PathBuf::from("/tmp/suspicious"));
        assert_eq!(result.is_signed, Some(false));
    }
}
