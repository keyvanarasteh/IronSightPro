//! Network audit — combines socket mapping, DNS enrichment, and suspicious port detection
//! into a comprehensive network posture report.

use serde::{Deserialize, Serialize};

use crate::dns::{port_intel, DnsEntry, PortIntel};
use crate::socket_mapper::{self, SocketInfo, SocketMapper};

/// Full network audit result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAudit {
    pub total_sockets: usize,
    pub listeners: Vec<ListenerInfo>,
    pub suspicious_connections: Vec<SuspiciousConnection>,
    pub external_connections: Vec<ExternalConnection>,
    pub flag_count: u32,
    pub flags: Vec<String>,
}

/// A listening socket with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerInfo {
    pub port: u16,
    pub protocol: crate::socket_mapper::Protocol,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub bind_address: String,
}

/// A connection flagged as suspicious based on port intelligence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousConnection {
    pub socket: SocketInfo,
    pub intel: PortIntel,
}

/// An established connection to an external (non-private) IP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalConnection {
    pub socket: SocketInfo,
    pub dns: DnsEntry,
}

impl NetworkAudit {
    /// Run a full network audit.
    pub fn scan() -> Self {
        let all_sockets = SocketMapper::scan();
        let mut flags: Vec<String> = Vec::new();

        // Listeners
        let listeners: Vec<ListenerInfo> = socket_mapper::listeners(&all_sockets)
            .into_iter()
            .map(|s| ListenerInfo {
                port: s.local_port,
                protocol: s.protocol,
                pid: s.pid,
                process_name: s.process_name.clone(),
                bind_address: s.local_addr.to_string(),
            })
            .collect();

        // Suspicious connections (known bad ports)
        let mut suspicious_connections = Vec::new();
        for sock in &all_sockets {
            if let Some(intel) = port_intel(sock.remote_port) {
                flags.push(format!(
                    "Suspicious port {} ({}) — {} [PID: {}]",
                    sock.remote_port,
                    intel.service,
                    intel.risk_note,
                    sock.pid.map_or("?".to_string(), |p| p.to_string())
                ));
                suspicious_connections.push(SuspiciousConnection {
                    socket: sock.clone(),
                    intel,
                });
            }
            // Also check local listening on suspicious ports
            if sock.is_listener() {
                if let Some(intel) = port_intel(sock.local_port) {
                    flags.push(format!(
                        "Listening on suspicious port {} ({}) — {}",
                        sock.local_port, intel.service, intel.risk_note
                    ));
                    suspicious_connections.push(SuspiciousConnection {
                        socket: sock.clone(),
                        intel,
                    });
                }
            }
        }

        // External connections — non-private IPs
        let established = socket_mapper::established(&all_sockets);
        let external_connections: Vec<ExternalConnection> = established
            .into_iter()
            .filter_map(|s| {
                let dns = DnsEntry::lookup(s.remote_addr);
                if !dns.is_private {
                    Some(ExternalConnection {
                        socket: s.clone(),
                        dns,
                    })
                } else {
                    None
                }
            })
            .collect();

        let total = all_sockets.len();

        NetworkAudit {
            total_sockets: total,
            listeners,
            suspicious_connections,
            external_connections,
            flag_count: flags.len() as u32,
            flags,
        }
    }

    /// Run audit for a specific PID only.
    pub fn scan_pid(target_pid: u32) -> Self {
        let all_sockets = SocketMapper::scan();
        let pid_sockets: Vec<SocketInfo> = all_sockets
            .into_iter()
            .filter(|s| s.pid == Some(target_pid))
            .collect();

        let mut flags: Vec<String> = Vec::new();

        let listeners: Vec<ListenerInfo> = socket_mapper::listeners(&pid_sockets)
            .into_iter()
            .map(|s| ListenerInfo {
                port: s.local_port,
                protocol: s.protocol,
                pid: s.pid,
                process_name: s.process_name.clone(),
                bind_address: s.local_addr.to_string(),
            })
            .collect();

        let mut suspicious_connections = Vec::new();
        for sock in &pid_sockets {
            if let Some(intel) = port_intel(sock.remote_port) {
                flags.push(format!(
                    "PID {} connecting to suspicious port {} ({})",
                    target_pid, sock.remote_port, intel.service
                ));
                suspicious_connections.push(SuspiciousConnection {
                    socket: sock.clone(),
                    intel,
                });
            }
        }

        let external_connections: Vec<ExternalConnection> = socket_mapper::established(&pid_sockets)
            .into_iter()
            .filter_map(|s| {
                let dns = DnsEntry::lookup(s.remote_addr);
                if !dns.is_private {
                    Some(ExternalConnection {
                        socket: s.clone(),
                        dns,
                    })
                } else {
                    None
                }
            })
            .collect();

        let total = pid_sockets.len();

        NetworkAudit {
            total_sockets: total,
            listeners,
            suspicious_connections,
            external_connections,
            flag_count: flags.len() as u32,
            flags,
        }
    }
}
