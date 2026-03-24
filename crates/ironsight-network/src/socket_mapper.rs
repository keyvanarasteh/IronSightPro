//! Socket-to-PID mapping and connection enumeration.
//!
//! On Linux, parses `/proc/net/tcp`, `/proc/net/tcp6`, `/proc/net/udp`, `/proc/net/udp6`.
//! Maps each socket inode to the owning PID via `/proc/<pid>/fd/`.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TcpState {
    Established,
    SynSent,
    SynRecv,
    FinWait1,
    FinWait2,
    TimeWait,
    Close,
    CloseWait,
    LastAck,
    Listen,
    Closing,
    Unknown(u8),
}

impl TcpState {
    fn from_hex(s: &str) -> Self {
        match u8::from_str_radix(s.trim(), 16).unwrap_or(0) {
            0x01 => Self::Established,
            0x02 => Self::SynSent,
            0x03 => Self::SynRecv,
            0x04 => Self::FinWait1,
            0x05 => Self::FinWait2,
            0x06 => Self::TimeWait,
            0x07 => Self::Close,
            0x08 => Self::CloseWait,
            0x09 => Self::LastAck,
            0x0A => Self::Listen,
            0x0B => Self::Closing,
            other => Self::Unknown(other),
        }
    }
}

/// A single socket connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketInfo {
    pub protocol: Protocol,
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
    pub state: TcpState,
    pub inode: u64,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

impl SocketInfo {
    /// True if this socket is in LISTEN state.
    pub fn is_listener(&self) -> bool {
        self.state == TcpState::Listen
    }

    /// True if this socket has an active connection to a remote host.
    pub fn is_established(&self) -> bool {
        self.state == TcpState::Established
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SocketMapper
// ─────────────────────────────────────────────────────────────────────────────

/// Maps sockets to PIDs by reading /proc on Linux.
pub struct SocketMapper;

impl SocketMapper {
    /// Enumerate all sockets and map them to PIDs.
    #[cfg(target_os = "linux")]
    pub fn scan() -> Vec<SocketInfo> {
        let inode_to_pid = Self::build_inode_pid_map();
        let pid_names = Self::build_pid_name_map();

        let mut sockets = Vec::new();

        // TCP v4
        if let Ok(content) = std::fs::read_to_string("/proc/net/tcp") {
            sockets.extend(Self::parse_proc_net(&content, Protocol::Tcp, false, &inode_to_pid, &pid_names));
        }
        // TCP v6
        if let Ok(content) = std::fs::read_to_string("/proc/net/tcp6") {
            sockets.extend(Self::parse_proc_net(&content, Protocol::Tcp, true, &inode_to_pid, &pid_names));
        }
        // UDP v4
        if let Ok(content) = std::fs::read_to_string("/proc/net/udp") {
            sockets.extend(Self::parse_proc_net(&content, Protocol::Udp, false, &inode_to_pid, &pid_names));
        }
        // UDP v6
        if let Ok(content) = std::fs::read_to_string("/proc/net/udp6") {
            sockets.extend(Self::parse_proc_net(&content, Protocol::Udp, true, &inode_to_pid, &pid_names));
        }

        sockets
    }

    #[cfg(not(target_os = "linux"))]
    pub fn scan() -> Vec<SocketInfo> {
        tracing::warn!("SocketMapper::scan() is only supported on Linux");
        Vec::new()
    }

    /// Build a mapping from socket inode → PID by scanning /proc/<pid>/fd/.
    #[cfg(target_os = "linux")]
    fn build_inode_pid_map() -> HashMap<u64, u32> {
        let mut map = HashMap::new();
        let proc_dir = match std::fs::read_dir("/proc") {
            Ok(d) => d,
            Err(_) => return map,
        };

        for entry in proc_dir.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            let pid: u32 = match name_str.parse() {
                Ok(p) => p,
                Err(_) => continue,
            };

            let fd_path = format!("/proc/{pid}/fd");
            let fd_dir = match std::fs::read_dir(&fd_path) {
                Ok(d) => d,
                Err(_) => continue,
            };

            for fd_entry in fd_dir.flatten() {
                if let Ok(link) = std::fs::read_link(fd_entry.path()) {
                    let link_str = link.to_string_lossy().to_string();
                    // socket:[12345]
                    if let Some(inode_str) = link_str.strip_prefix("socket:[").and_then(|s| s.strip_suffix(']')) {
                        if let Ok(inode) = inode_str.parse::<u64>() {
                            map.insert(inode, pid);
                        }
                    }
                }
            }
        }
        map
    }

    /// Build PID → process name map from /proc/<pid>/comm.
    #[cfg(target_os = "linux")]
    fn build_pid_name_map() -> HashMap<u32, String> {
        let mut map = HashMap::new();
        let proc_dir = match std::fs::read_dir("/proc") {
            Ok(d) => d,
            Err(_) => return map,
        };

        for entry in proc_dir.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            let pid: u32 = match name_str.parse() {
                Ok(p) => p,
                Err(_) => continue,
            };

            let comm_path = format!("/proc/{pid}/comm");
            if let Ok(comm) = std::fs::read_to_string(&comm_path) {
                map.insert(pid, comm.trim().to_string());
            }
        }
        map
    }

    /// Parse a `/proc/net/tcp` or `/proc/net/udp` file content.
    #[cfg(target_os = "linux")]
    fn parse_proc_net(
        content: &str,
        protocol: Protocol,
        is_v6: bool,
        inode_map: &HashMap<u64, u32>,
        pid_names: &HashMap<u32, String>,
    ) -> Vec<SocketInfo> {
        let mut sockets = Vec::new();

        for line in content.lines().skip(1) {
            // Skip header
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() < 10 {
                continue;
            }

            let local = fields[1];
            let remote = fields[2];
            let state_hex = fields[3];
            let inode: u64 = fields[9].parse().unwrap_or(0);

            let (local_addr, local_port) = if is_v6 {
                Self::parse_addr_v6(local)
            } else {
                Self::parse_addr_v4(local)
            };

            let (remote_addr, remote_port) = if is_v6 {
                Self::parse_addr_v6(remote)
            } else {
                Self::parse_addr_v4(remote)
            };

            let state = TcpState::from_hex(state_hex);
            let pid = inode_map.get(&inode).copied();
            let process_name = pid.and_then(|p| pid_names.get(&p).cloned());

            sockets.push(SocketInfo {
                protocol,
                local_addr,
                local_port,
                remote_addr,
                remote_port,
                state,
                inode,
                pid,
                process_name,
            });
        }

        sockets
    }

    /// Parse "0100007F:0050" → (127.0.0.1, 80)
    #[cfg(target_os = "linux")]
    fn parse_addr_v4(s: &str) -> (IpAddr, u16) {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return (IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
        }

        let ip_hex = parts[0];
        let port = u16::from_str_radix(parts[1], 16).unwrap_or(0);

        let ip_u32 = u32::from_str_radix(ip_hex, 16).unwrap_or(0);
        // /proc/net stores IPs in little-endian
        let ip = Ipv4Addr::from(ip_u32.to_be());

        (IpAddr::V4(ip), port)
    }

    /// Parse IPv6 hex address + port.
    #[cfg(target_os = "linux")]
    fn parse_addr_v6(s: &str) -> (IpAddr, u16) {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return (IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);
        }

        let ip_hex = parts[0];
        let port = u16::from_str_radix(parts[1], 16).unwrap_or(0);

        if ip_hex.len() != 32 {
            return (IpAddr::V6(Ipv6Addr::UNSPECIFIED), port);
        }

        // IPv6 in /proc/net is stored as 4 groups of 4 bytes, each group in little-endian
        let mut octets = [0u8; 16];
        for group in 0..4 {
            let offset = group * 8;
            let group_hex = &ip_hex[offset..offset + 8];
            let val = u32::from_str_radix(group_hex, 16).unwrap_or(0);
            let bytes = val.to_be_bytes();
            let base = group * 4;
            octets[base] = bytes[3];
            octets[base + 1] = bytes[2];
            octets[base + 2] = bytes[1];
            octets[base + 3] = bytes[0];
        }

        (IpAddr::V6(Ipv6Addr::from(octets)), port)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Filters
// ─────────────────────────────────────────────────────────────────────────────

/// Get all listening sockets.
pub fn listeners(sockets: &[SocketInfo]) -> Vec<&SocketInfo> {
    sockets.iter().filter(|s| s.is_listener()).collect()
}

/// Get all established connections.
pub fn established(sockets: &[SocketInfo]) -> Vec<&SocketInfo> {
    sockets.iter().filter(|s| s.is_established()).collect()
}

/// Get sockets belonging to a specific PID.
pub fn by_pid(sockets: &[SocketInfo], pid: u32) -> Vec<&SocketInfo> {
    sockets.iter().filter(|s| s.pid == Some(pid)).collect()
}

/// Get sockets connecting to a specific remote port.
pub fn by_remote_port(sockets: &[SocketInfo], port: u16) -> Vec<&SocketInfo> {
    sockets.iter().filter(|s| s.remote_port == port).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tcp_state_parsing() {
        assert_eq!(TcpState::from_hex("0A"), TcpState::Listen);
        assert_eq!(TcpState::from_hex("01"), TcpState::Established);
        assert_eq!(TcpState::from_hex("06"), TcpState::TimeWait);
        assert_eq!(TcpState::from_hex("08"), TcpState::CloseWait);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_ipv4_address() {
        // 0100007F:0050 = 127.0.0.1:80
        let (ip, port) = SocketMapper::parse_addr_v4("0100007F:0050");
        assert_eq!(ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(port, 80);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_zero_address() {
        let (ip, port) = SocketMapper::parse_addr_v4("00000000:0000");
        assert_eq!(ip, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
        assert_eq!(port, 0);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn scan_finds_sockets() {
        let sockets = SocketMapper::scan();
        // Any running system has at least some sockets
        assert!(
            !sockets.is_empty(),
            "Expected at least some sockets on this system"
        );
    }

    #[test]
    fn filter_listeners() {
        let sockets = vec![
            SocketInfo {
                protocol: Protocol::Tcp,
                local_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
                local_port: 8080,
                remote_addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                remote_port: 0,
                state: TcpState::Listen,
                inode: 100,
                pid: Some(1),
                process_name: Some("web".into()),
            },
            SocketInfo {
                protocol: Protocol::Tcp,
                local_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
                local_port: 12345,
                remote_addr: IpAddr::V4(Ipv4Addr::new(93, 184, 216, 34)),
                remote_port: 443,
                state: TcpState::Established,
                inode: 200,
                pid: Some(2),
                process_name: Some("curl".into()),
            },
        ];

        let l = listeners(&sockets);
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].local_port, 8080);

        let e = established(&sockets);
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].remote_port, 443);
    }

    #[test]
    fn filter_by_pid() {
        let sockets = vec![
            SocketInfo {
                protocol: Protocol::Tcp,
                local_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
                local_port: 80,
                remote_addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                remote_port: 0,
                state: TcpState::Listen,
                inode: 1,
                pid: Some(42),
                process_name: Some("nginx".into()),
            },
            SocketInfo {
                protocol: Protocol::Tcp,
                local_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
                local_port: 3000,
                remote_addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                remote_port: 0,
                state: TcpState::Listen,
                inode: 2,
                pid: Some(99),
                process_name: Some("node".into()),
            },
        ];

        let result = by_pid(&sockets, 42);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].process_name.as_deref(), Some("nginx"));
    }

    #[test]
    fn filter_by_remote_port() {
        let sockets = vec![
            SocketInfo {
                protocol: Protocol::Tcp,
                local_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
                local_port: 54321,
                remote_addr: IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)),
                remote_port: 443,
                state: TcpState::Established,
                inode: 1,
                pid: Some(10),
                process_name: None,
            },
            SocketInfo {
                protocol: Protocol::Tcp,
                local_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
                local_port: 54322,
                remote_addr: IpAddr::V4(Ipv4Addr::new(5, 6, 7, 8)),
                remote_port: 80,
                state: TcpState::Established,
                inode: 2,
                pid: Some(11),
                process_name: None,
            },
        ];

        let https = by_remote_port(&sockets, 443);
        assert_eq!(https.len(), 1);
    }
}
