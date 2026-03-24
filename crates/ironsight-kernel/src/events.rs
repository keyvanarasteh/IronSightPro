//! KernelEvent — unified event types from eBPF and ETW.
//!
//! STEP 1: Comprehensive KernelEvent enum with all syscall/ETW event variants.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────────
// KernelEvent
// ─────────────────────────────────────────────────────────────────────────────

/// A unified kernel event from eBPF (Linux) or ETW (Windows).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelEvent {
    pub timestamp: DateTime<Utc>,
    pub pid: u32,
    pub tid: u32,
    pub comm: String,
    pub kind: KernelEventKind,
}

/// The kind of kernel event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KernelEventKind {
    /// mprotect / VirtualProtect — memory protection change
    MemoryProtect(MemoryProtection),
    /// mmap / VirtualAlloc — new memory mapping
    MemoryMap {
        addr: u64,
        length: u64,
        prot: u32,
        flags: u32,
    },
    /// execve / CreateProcess — process execution
    ProcessExec {
        filename: String,
        argv: Vec<String>,
    },
    /// open/openat / CreateFile — file access
    FileOpen {
        path: String,
        flags: u32,
    },
    /// connect / WSAConnect — network connection attempt
    NetworkConnect {
        addr: String,
        port: u16,
        protocol: String,
    },
    /// ptrace / ReadProcessMemory — debugging attach
    PtraceAttach {
        target_pid: u32,
    },
    /// Custom syscall event for future extensions
    Syscall(SyscallInfo),
}

/// Memory protection change event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProtection {
    pub addr: u64,
    pub length: u64,
    pub old_prot: u32,
    pub new_prot: u32,
}

impl MemoryProtection {
    /// True if the new protection makes memory executable.
    pub fn becomes_executable(&self) -> bool {
        // PROT_EXEC = 0x4 on Linux, PAGE_EXECUTE* on Windows
        (self.new_prot & 0x4) != 0 && (self.old_prot & 0x4) == 0
    }

    /// True if this is a W^X violation (both writable and executable).
    pub fn is_wx_violation(&self) -> bool {
        // Writable (0x2) AND executable (0x4)
        (self.new_prot & 0x2) != 0 && (self.new_prot & 0x4) != 0
    }
}

/// Generic syscall information for extensibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyscallInfo {
    pub number: u32,
    pub name: String,
    pub args: Vec<u64>,
    pub return_value: i64,
}

impl KernelEvent {
    /// Create a new kernel event with current timestamp.
    pub fn new(pid: u32, tid: u32, comm: &str, kind: KernelEventKind) -> Self {
        Self {
            timestamp: Utc::now(),
            pid,
            tid,
            comm: comm.to_string(),
            kind,
        }
    }

    /// True if this event is security-relevant.
    pub fn is_suspicious(&self) -> bool {
        match &self.kind {
            KernelEventKind::MemoryProtect(mp) => mp.becomes_executable() || mp.is_wx_violation(),
            KernelEventKind::PtraceAttach { .. } => true,
            KernelEventKind::NetworkConnect { port, .. } => {
                matches!(port, 4444 | 5555 | 6666 | 1337 | 31337 | 8443)
            }
            KernelEventKind::FileOpen { path, .. } => {
                path.contains("/etc/shadow")
                    || path.contains("/etc/passwd")
                    || path.contains("/proc/kcore")
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mprotect_wx_detection() {
        let mp = MemoryProtection {
            addr: 0x7f000000,
            length: 4096,
            old_prot: 0x3, // RW
            new_prot: 0x7, // RWX
        };
        assert!(mp.becomes_executable());
        assert!(mp.is_wx_violation());
    }

    #[test]
    fn mprotect_readonly_not_suspicious() {
        let mp = MemoryProtection {
            addr: 0x7f000000,
            length: 4096,
            old_prot: 0x3,
            new_prot: 0x1, // Read-only
        };
        assert!(!mp.becomes_executable());
        assert!(!mp.is_wx_violation());
    }

    #[test]
    fn suspicious_ptrace_event() {
        let event = KernelEvent::new(
            42, 42, "gdb",
            KernelEventKind::PtraceAttach { target_pid: 1 },
        );
        assert!(event.is_suspicious());
    }

    #[test]
    fn suspicious_port_event() {
        let event = KernelEvent::new(
            100, 100, "backdoor",
            KernelEventKind::NetworkConnect {
                addr: "10.0.0.1".into(),
                port: 4444,
                protocol: "tcp".into(),
            },
        );
        assert!(event.is_suspicious());
    }

    #[test]
    fn normal_port_not_suspicious() {
        let event = KernelEvent::new(
            100, 100, "curl",
            KernelEventKind::NetworkConnect {
                addr: "google.com".into(),
                port: 443,
                protocol: "tcp".into(),
            },
        );
        assert!(!event.is_suspicious());
    }

    #[test]
    fn shadow_file_is_suspicious() {
        let event = KernelEvent::new(
            200, 200, "cat",
            KernelEventKind::FileOpen {
                path: "/etc/shadow".into(),
                flags: 0,
            },
        );
        assert!(event.is_suspicious());
    }
}
