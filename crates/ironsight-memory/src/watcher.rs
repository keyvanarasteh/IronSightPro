//! Memory watcher — track memory region changes between snapshots.
//!
//! Detects: new regions, removed regions, permission changes, size changes.

use serde::{Deserialize, Serialize};

use crate::maps::MemoryRegion;

/// A change detected in memory layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryChange {
    /// A new memory region appeared.
    RegionAdded {
        start: u64,
        end: u64,
        permissions: String,
        pathname: Option<String>,
    },
    /// A memory region was removed.
    RegionRemoved {
        start: u64,
        end: u64,
        permissions: String,
        pathname: Option<String>,
    },
    /// Permissions changed on an existing region.
    PermissionChanged {
        start: u64,
        old_perms: String,
        new_perms: String,
        pathname: Option<String>,
    },
    /// Region size changed (grew or shrunk).
    SizeChanged {
        start: u64,
        old_size: u64,
        new_size: u64,
        pathname: Option<String>,
    },
}

impl MemoryChange {
    /// True if this change is security-relevant.
    pub fn is_suspicious(&self) -> bool {
        match self {
            MemoryChange::PermissionChanged { new_perms, .. } => {
                // Becoming executable is suspicious
                new_perms.contains('x')
            }
            MemoryChange::RegionAdded { permissions, pathname, .. } => {
                // New anonymous executable region is suspicious
                permissions.contains('x') && pathname.is_none()
            }
            _ => false,
        }
    }
}

/// Compute differences between two memory region snapshots.
pub fn diff_regions(old: &[MemoryRegion], new: &[MemoryRegion]) -> Vec<MemoryChange> {
    let mut changes = Vec::new();

    // Index old regions by start address
    let old_map: std::collections::HashMap<u64, &MemoryRegion> =
        old.iter().map(|r| (r.start, r)).collect();
    let new_map: std::collections::HashMap<u64, &MemoryRegion> =
        new.iter().map(|r| (r.start, r)).collect();

    // Detect added regions
    for (start, region) in &new_map {
        if !old_map.contains_key(start) {
            changes.push(MemoryChange::RegionAdded {
                start: region.start,
                end: region.end,
                permissions: region.permissions.as_string(),
                pathname: region.pathname.clone(),
            });
        }
    }

    // Detect removed regions
    for (start, region) in &old_map {
        if !new_map.contains_key(start) {
            changes.push(MemoryChange::RegionRemoved {
                start: region.start,
                end: region.end,
                permissions: region.permissions.as_string(),
                pathname: region.pathname.clone(),
            });
        }
    }

    // Detect changes in existing regions
    for (start, old_region) in &old_map {
        if let Some(new_region) = new_map.get(start) {
            let old_perms = old_region.permissions.as_string();
            let new_perms = new_region.permissions.as_string();

            if old_perms != new_perms {
                changes.push(MemoryChange::PermissionChanged {
                    start: *start,
                    old_perms,
                    new_perms,
                    pathname: new_region.pathname.clone(),
                });
            }

            if old_region.size() != new_region.size() {
                changes.push(MemoryChange::SizeChanged {
                    start: *start,
                    old_size: old_region.size(),
                    new_size: new_region.size(),
                    pathname: new_region.pathname.clone(),
                });
            }
        }
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maps::Permissions;

    fn make_region(start: u64, end: u64, perms: &str, name: Option<&str>) -> MemoryRegion {
        MemoryRegion {
            start,
            end,
            permissions: Permissions {
                read: perms.contains('r'),
                write: perms.contains('w'),
                execute: perms.contains('x'),
                shared: perms.contains('s'),
            },
            offset: 0,
            device: "00:00".into(),
            inode: 0,
            pathname: name.map(|s| s.to_string()),
        }
    }

    #[test]
    fn detect_new_region() {
        let old = vec![make_region(0x1000, 0x2000, "r--p", Some("/usr/bin/ls"))];
        let new = vec![
            make_region(0x1000, 0x2000, "r--p", Some("/usr/bin/ls")),
            make_region(0x5000, 0x6000, "rwxp", None), // New anonymous rwx!
        ];

        let changes = diff_regions(&old, &new);
        assert!(changes.iter().any(|c| matches!(c, MemoryChange::RegionAdded { .. })));
        assert!(changes.iter().any(|c| c.is_suspicious()));
    }

    #[test]
    fn detect_removed_region() {
        let old = vec![
            make_region(0x1000, 0x2000, "r--p", Some("/lib/libc.so")),
            make_region(0x3000, 0x4000, "r-xp", Some("/lib/libc.so")),
        ];
        let new = vec![make_region(0x1000, 0x2000, "r--p", Some("/lib/libc.so"))];

        let changes = diff_regions(&old, &new);
        assert!(changes.iter().any(|c| matches!(c, MemoryChange::RegionRemoved { .. })));
    }

    #[test]
    fn detect_permission_change() {
        let old = vec![make_region(0x1000, 0x2000, "rw-p", None)];
        let new = vec![make_region(0x1000, 0x2000, "rwxp", None)]; // Became executable!

        let changes = diff_regions(&old, &new);
        let perm_change = changes.iter().find(|c| matches!(c, MemoryChange::PermissionChanged { .. }));
        assert!(perm_change.is_some());
        assert!(perm_change.unwrap().is_suspicious());
    }

    #[test]
    fn detect_size_change() {
        let old = vec![make_region(0x1000, 0x2000, "rw-p", Some("[heap]"))];
        let new = vec![make_region(0x1000, 0x5000, "rw-p", Some("[heap]"))]; // Grew

        let changes = diff_regions(&old, &new);
        assert!(changes.iter().any(|c| matches!(c, MemoryChange::SizeChanged { .. })));
    }

    #[test]
    fn no_changes_when_identical() {
        let regions = vec![
            make_region(0x1000, 0x2000, "r--p", Some("/usr/bin/ls")),
            make_region(0x3000, 0x4000, "r-xp", Some("/usr/bin/ls")),
        ];

        let changes = diff_regions(&regions, &regions);
        assert!(changes.is_empty());
    }
}
