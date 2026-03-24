//! DNS enrichment — reverse lookup for IP addresses.

use std::net::IpAddr;

use serde::{Deserialize, Serialize};

/// Result of a reverse DNS lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsEntry {
    pub ip: IpAddr,
    pub hostname: Option<String>,
    pub is_private: bool,
}

impl DnsEntry {
    /// Enrich an IP address with reverse DNS and privacy classification.
    pub fn lookup(ip: IpAddr) -> Self {
        let is_private = Self::is_private_ip(&ip);
        let hostname = Self::reverse_lookup(&ip);

        DnsEntry {
            ip,
            hostname,
            is_private,
        }
    }

    /// Check if an IP is in a private/reserved range.
    fn is_private_ip(ip: &IpAddr) -> bool {
        match ip {
            IpAddr::V4(v4) => {
                v4.is_loopback()
                    || v4.is_private()
                    || v4.is_link_local()
                    || v4.is_broadcast()
                    || v4.is_unspecified()
            }
            IpAddr::V6(v6) => {
                v6.is_loopback()
                    || v6.is_unspecified()
                    // ULA (fc00::/7)
                    || (v6.segments()[0] & 0xfe00) == 0xfc00
                    // Link-local (fe80::/10)
                    || (v6.segments()[0] & 0xffc0) == 0xfe80
            }
        }
    }

    /// Attempt reverse DNS lookup via system resolver.
    fn reverse_lookup(ip: &IpAddr) -> Option<String> {
        // Use std DNS resolution (blocking — fine for audit-time lookups)
        use std::net::ToSocketAddrs;
        let addr = format!("{ip}:0");
        // Try to resolve, timeout naturally handled by OS resolver
        match addr.to_socket_addrs() {
            Ok(_) => {
                // Unfortunately std doesn't do reverse DNS.
                // On Linux, we can try reading /etc/hosts or use `host` command.
                #[cfg(target_os = "linux")]
                {
                    let output = std::process::Command::new("host")
                        .arg(ip.to_string())
                        .output();

                    if let Ok(out) = output {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        // Parse "X.X.X.X.in-addr.arpa domain name pointer hostname."
                        if let Some(line) = stdout.lines().find(|l| l.contains("domain name pointer")) {
                            if let Some(hostname) = line.split("pointer ").nth(1) {
                                let h = hostname.trim_end_matches('.').to_string();
                                if !h.is_empty() {
                                    return Some(h);
                                }
                            }
                        }
                    }
                    None
                }

                #[cfg(not(target_os = "linux"))]
                None
            }
            Err(_) => None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Known suspicious ports
// ─────────────────────────────────────────────────────────────────────────────

/// Well-known ports that may indicate suspicious activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortIntel {
    pub port: u16,
    pub service: String,
    pub risk_note: String,
}

/// Check if a remote port is known-suspicious.
pub fn port_intel(port: u16) -> Option<PortIntel> {
    match port {
        4444 => Some(PortIntel {
            port,
            service: "Metasploit default".into(),
            risk_note: "Common reverse shell port".into(),
        }),
        5555 => Some(PortIntel {
            port,
            service: "Android ADB".into(),
            risk_note: "Remote debug bridge — shouldn't be exposed".into(),
        }),
        1337 => Some(PortIntel {
            port,
            service: "Elite/leet".into(),
            risk_note: "Common backdoor port (cultural convention)".into(),
        }),
        31337 => Some(PortIntel {
            port,
            service: "Back Orifice".into(),
            risk_note: "Classic backdoor trojan port".into(),
        }),
        6667 | 6697 => Some(PortIntel {
            port,
            service: "IRC".into(),
            risk_note: "IRC — used by some C2 frameworks".into(),
        }),
        6666 => Some(PortIntel {
            port,
            service: "IRC alt / DarkComet".into(),
            risk_note: "Common RAT/C2 port".into(),
        }),
        3389 => Some(PortIntel {
            port,
            service: "RDP".into(),
            risk_note: "Remote Desktop — brute force target".into(),
        }),
        2222 => Some(PortIntel {
            port,
            service: "SSH alt".into(),
            risk_note: "Alternative SSH — may indicate unauthorized access".into(),
        }),
        5900..=5910 => Some(PortIntel {
            port,
            service: "VNC".into(),
            risk_note: "VNC remote display — possible unauthorized access".into(),
        }),
        8443 => Some(PortIntel {
            port,
            service: "Alt HTTPS".into(),
            risk_note: "Alternative HTTPS — common for C2 panels".into(),
        }),
        9001 => Some(PortIntel {
            port,
            service: "Tor default".into(),
            risk_note: "Tor relay/SOCKS — may indicate anonymization".into(),
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn private_ip_detection() {
        assert!(DnsEntry::is_private_ip(&IpAddr::V4(Ipv4Addr::LOCALHOST)));
        assert!(DnsEntry::is_private_ip(&IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
        assert!(DnsEntry::is_private_ip(&IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))));
        assert!(DnsEntry::is_private_ip(&IpAddr::V6(Ipv6Addr::LOCALHOST)));
        assert!(!DnsEntry::is_private_ip(&IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
        assert!(!DnsEntry::is_private_ip(&IpAddr::V4(Ipv4Addr::new(93, 184, 216, 34))));
    }

    #[test]
    fn known_suspicious_ports() {
        assert!(port_intel(4444).is_some());
        assert_eq!(port_intel(4444).unwrap().service, "Metasploit default");
        assert!(port_intel(31337).is_some());
        assert!(port_intel(80).is_none()); // HTTP is fine
        assert!(port_intel(443).is_none()); // HTTPS is fine
    }

    #[test]
    fn lookup_localhost() {
        let entry = DnsEntry::lookup(IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert!(entry.is_private);
    }
}
