//! Memory region mapping — parse `/proc/<pid>/maps` to enumerate virtual memory regions.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A single virtual memory region from /proc/<pid>/maps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub permissions: Permissions,
    pub offset: u64,
    pub device: String,
    pub inode: u64,
    pub pathname: Option<String>,
}

impl MemoryRegion {
    /// Size of this memory region in bytes.
    pub fn size(&self) -> u64 {
        self.end - self.start
    }

    /// Size in MiB.
    pub fn size_mib(&self) -> f64 {
        self.size() as f64 / (1024.0 * 1024.0)
    }

    /// True if this region is executable.
    pub fn is_executable(&self) -> bool {
        self.permissions.execute
    }

    /// True if this region is writable AND executable (W^X violation).
    pub fn is_wx(&self) -> bool {
        self.permissions.write && self.permissions.execute
    }

    /// True if this is an anonymous mapping (no file backing).
    pub fn is_anonymous(&self) -> bool {
        self.pathname.is_none()
    }

    /// True if this is a stack region.
    pub fn is_stack(&self) -> bool {
        self.pathname.as_deref() == Some("[stack]")
    }

    /// True if this is a heap region.
    pub fn is_heap(&self) -> bool {
        self.pathname.as_deref() == Some("[heap]")
    }
}

/// UNIX-style permission bits for a memory region.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub shared: bool,
}

impl Permissions {
    fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        Permissions {
            read: bytes.first() == Some(&b'r'),
            write: bytes.get(1) == Some(&b'w'),
            execute: bytes.get(2) == Some(&b'x'),
            shared: bytes.get(3) == Some(&b's'),
        }
    }

    pub fn as_string(&self) -> String {
        format!(
            "{}{}{}{}",
            if self.read { 'r' } else { '-' },
            if self.write { 'w' } else { '-' },
            if self.execute { 'x' } else { '-' },
            if self.shared { 's' } else { 'p' },
        )
    }
}

/// Parse /proc/<pid>/maps for a given PID.
#[cfg(target_os = "linux")]
pub fn read_maps(pid: u32) -> Result<Vec<MemoryRegion>, std::io::Error> {
    let path = PathBuf::from(format!("/proc/{pid}/maps"));
    let content = std::fs::read_to_string(&path)?;
    Ok(parse_maps(&content))
}

#[cfg(not(target_os = "linux"))]
pub fn read_maps(_pid: u32) -> Result<Vec<MemoryRegion>, std::io::Error> {
    Ok(Vec::new())
}

/// Parse the content of a /proc/<pid>/maps file.
pub fn parse_maps(content: &str) -> Vec<MemoryRegion> {
    let mut regions = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(6, char::is_whitespace).collect();
        if parts.len() < 5 {
            continue;
        }

        // Address range: "7f1234000000-7f1234001000"
        let addr_parts: Vec<&str> = parts[0].split('-').collect();
        if addr_parts.len() != 2 {
            continue;
        }

        let start = u64::from_str_radix(addr_parts[0], 16).unwrap_or(0);
        let end = u64::from_str_radix(addr_parts[1], 16).unwrap_or(0);
        let permissions = Permissions::from_str(parts[1]);
        let offset = u64::from_str_radix(parts[2], 16).unwrap_or(0);
        let device = parts[3].to_string();
        let inode = parts[4].trim().parse::<u64>().unwrap_or(0);

        let pathname = if parts.len() > 5 {
            let p = parts[5].trim();
            if p.is_empty() {
                None
            } else {
                Some(p.to_string())
            }
        } else {
            None
        };

        regions.push(MemoryRegion {
            start,
            end,
            permissions,
            offset,
            device,
            inode,
            pathname,
        });
    }

    regions
}

/// Summary statistics for a process's memory regions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySummary {
    pub pid: u32,
    pub total_regions: usize,
    pub total_size_bytes: u64,
    pub executable_regions: usize,
    pub writable_executable_regions: usize,
    pub anonymous_executable_regions: usize,
    pub heap_size_bytes: u64,
    pub stack_size_bytes: u64,
    pub flags: Vec<String>,
}

/// Analyze memory regions and produce a summary with flags.
pub fn summarize(pid: u32, regions: &[MemoryRegion]) -> MemorySummary {
    let mut flags = Vec::new();

    let wx_regions: Vec<&MemoryRegion> = regions.iter().filter(|r| r.is_wx()).collect();
    let anon_exec: Vec<&MemoryRegion> = regions
        .iter()
        .filter(|r| r.is_anonymous() && r.is_executable())
        .collect();

    if !wx_regions.is_empty() {
        flags.push(format!(
            "W^X violation: {} regions are both writable and executable",
            wx_regions.len()
        ));
    }

    if !anon_exec.is_empty() {
        flags.push(format!(
            "Anonymous executable regions: {} — possible shellcode injection",
            anon_exec.len()
        ));
    }

    let heap_size: u64 = regions.iter().filter(|r| r.is_heap()).map(|r| r.size()).sum();
    let stack_size: u64 = regions.iter().filter(|r| r.is_stack()).map(|r| r.size()).sum();

    MemorySummary {
        pid,
        total_regions: regions.len(),
        total_size_bytes: regions.iter().map(|r| r.size()).sum(),
        executable_regions: regions.iter().filter(|r| r.is_executable()).count(),
        writable_executable_regions: wx_regions.len(),
        anonymous_executable_regions: anon_exec.len(),
        heap_size_bytes: heap_size,
        stack_size_bytes: stack_size,
        flags,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_MAPS: &str = "\
55a2e3400000-55a2e3402000 r--p 00000000 08:02 1234567  /usr/bin/ls
55a2e3402000-55a2e3418000 r-xp 00002000 08:02 1234567  /usr/bin/ls
55a2e3418000-55a2e341f000 r--p 00018000 08:02 1234567  /usr/bin/ls
55a2e3420000-55a2e3421000 rw-p 0001f000 08:02 1234567  /usr/bin/ls
55a2e4c00000-55a2e4c21000 rw-p 00000000 00:00 0        [heap]
7f1234000000-7f1234002000 rwxp 00000000 00:00 0
7ffd12340000-7ffd12361000 rw-p 00000000 00:00 0        [stack]";

    #[test]
    fn parse_maps_region_count() {
        let regions = parse_maps(SAMPLE_MAPS);
        assert_eq!(regions.len(), 7);
    }

    #[test]
    fn parse_maps_addresses() {
        let regions = parse_maps(SAMPLE_MAPS);
        assert_eq!(regions[0].start, 0x55a2e3400000);
        assert_eq!(regions[0].end, 0x55a2e3402000);
        assert_eq!(regions[0].size(), 0x2000);
    }

    #[test]
    fn parse_maps_permissions() {
        let regions = parse_maps(SAMPLE_MAPS);
        // r--p
        assert!(regions[0].permissions.read);
        assert!(!regions[0].permissions.write);
        assert!(!regions[0].permissions.execute);
        // r-xp
        assert!(regions[1].permissions.execute);
        assert!(!regions[1].permissions.write);
    }

    #[test]
    fn detect_wx_region() {
        let regions = parse_maps(SAMPLE_MAPS);
        // rwxp region at index 5
        let wx: Vec<&MemoryRegion> = regions.iter().filter(|r| r.is_wx()).collect();
        assert_eq!(wx.len(), 1, "Should detect exactly 1 W^X violation");
    }

    #[test]
    fn detect_anonymous_executable() {
        let regions = parse_maps(SAMPLE_MAPS);
        let anon_exec: Vec<&MemoryRegion> = regions
            .iter()
            .filter(|r| r.is_anonymous() && r.is_executable())
            .collect();
        assert_eq!(anon_exec.len(), 1, "Should detect 1 anonymous executable region");
    }

    #[test]
    fn detect_heap_and_stack() {
        let regions = parse_maps(SAMPLE_MAPS);
        assert!(regions.iter().any(|r| r.is_heap()));
        assert!(regions.iter().any(|r| r.is_stack()));
    }

    #[test]
    fn summarize_flags_wx() {
        let regions = parse_maps(SAMPLE_MAPS);
        let summary = summarize(1234, &regions);
        assert!(summary.writable_executable_regions > 0);
        assert!(summary.flags.iter().any(|f| f.contains("W^X")));
    }

    #[test]
    fn permissions_string() {
        let p = Permissions {
            read: true,
            write: false,
            execute: true,
            shared: false,
        };
        assert_eq!(p.as_string(), "r-xp");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn read_self_maps() {
        let pid = std::process::id();
        let regions = read_maps(pid).unwrap();
        assert!(!regions.is_empty(), "Our process should have memory regions");
        assert!(
            regions.iter().any(|r| r.is_stack()),
            "Our process should have a stack"
        );
    }
}
