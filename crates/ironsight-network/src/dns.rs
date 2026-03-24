//! DNS enrichment — reverse lookup for IP addresses.

use std::net::IpAddr;
use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};

use hickory_resolver::TokioResolver;
use hickory_resolver::config::*;

/// Result of a reverse DNS lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsEntry {
    pub ip: IpAddr,
    pub hostname: Option<String>,
    pub is_private: bool,
}

impl DnsEntry {
    /// Enrich an IP address with reverse DNS and privacy classification (blocking fallback).
    pub fn lookup(ip: IpAddr) -> Self {
        let is_private = Self::is_private_ip(&ip);
        DnsEntry {
            ip,
            hostname: None, 
            is_private,
        }
    }

    /// Check if an IP is in a private/reserved range.
    pub fn is_private_ip(ip: &IpAddr) -> bool {
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
                    // IPv4 mapped (::ffff:0:0/96)
                    || (v6.segments()[0] == 0 && v6.segments()[1] == 0 && v6.segments()[2] == 0 && v6.segments()[3] == 0 && v6.segments()[4] == 0 && v6.segments()[5] == 0xffff)
            }
        }
    }
}

pub struct AsyncDnsResolver {
    resolver: TokioResolver,
    cache: HashMap<IpAddr, DnsEntry>,
    timeout: Duration,
}

impl AsyncDnsResolver {
    pub async fn new() -> anyhow::Result<Self> {
        let resolver = hickory_resolver::TokioResolver::builder_with_config(
            ResolverConfig::default(),
            hickory_resolver::name_server::TokioConnectionProvider::default()
        )
        .with_options(ResolverOpts::default())
        .build();
        Ok(Self { 
            resolver, 
            cache: HashMap::new(), 
            timeout: Duration::from_secs(5) 
        })
    }

    pub async fn reverse_lookup(&mut self, ip: IpAddr) -> Option<String> {
        if let Some(cached) = self.cache.get(&ip) {
            return cached.hostname.clone();
        }
        
        match tokio::time::timeout(self.timeout, self.resolver.reverse_lookup(ip)).await {
            Ok(Ok(names)) => {
                let first = names.iter().next().map(|n| n.to_string().trim_end_matches('.').to_string());
                self.cache.insert(ip, DnsEntry {
                    ip,
                    hostname: first.clone(),
                    is_private: DnsEntry::is_private_ip(&ip),
                });
                first
            }
            _ => {
                self.cache.insert(ip, DnsEntry {
                    ip,
                    hostname: None,
                    is_private: DnsEntry::is_private_ip(&ip),
                });
                None
            }
        }
    }
    
    pub async fn lookup_entry(&mut self, ip: IpAddr) -> DnsEntry {
        if let Some(cached) = self.cache.get(&ip) {
            return cached.clone();
        }
        let is_private = DnsEntry::is_private_ip(&ip);
        if is_private {
            return DnsEntry { ip, hostname: None, is_private: true };
        }
        let hostname = self.reverse_lookup(ip).await;
        DnsEntry { ip, hostname, is_private }
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

impl PortIntel {
    pub fn new(port: u16, service: &str, risk_note: &str) -> Self {
        Self {
            port,
            service: service.to_string(),
            risk_note: risk_note.to_string(),
        }
    }
}

pub fn suspicious_ports() -> Vec<PortIntel> {
    vec![
        PortIntel::new(4444, "Metasploit default", "Common reverse shell port"),
        PortIntel::new(5555, "Android ADB", "Remote debug bridge — shouldn't be exposed"),
        PortIntel::new(1337, "Elite/leet", "Common backdoor port (cultural convention)"),
        PortIntel::new(31337, "Back Orifice", "Classic backdoor trojan port"),
        PortIntel::new(6667, "IRC", "IRC — used by some C2 frameworks"),
        PortIntel::new(6666, "IRC alt / DarkComet", "Common RAT/C2 port"),
        PortIntel::new(6668, "IRC Alt", "IRC alt / DDoS botnets"),
        PortIntel::new(6669, "IRC Alt", "IRC alt / DDoS botnets"),
        PortIntel::new(3389, "RDP", "Remote Desktop — brute force target"),
        PortIntel::new(2222, "SSH alt", "Alternative SSH — may indicate unauthorized access"),
        PortIntel::new(8443, "Alt HTTPS", "Alternative HTTPS — common for C2 panels"),
        PortIntel::new(9001, "Tor default", "Tor relay/SOCKS — may indicate anonymization"),
        PortIntel::new(9050, "Tor SOCKS Proxy", "Tor SOCKS Proxy anonymizer"),
        PortIntel::new(1080, "SOCKS Proxy", "Common SOCKS routing"),
        PortIntel::new(8080, "HTTP Proxy / C2", "Secondary HTTP / Cobalt Strike"),
        PortIntel::new(12345, "NetBus Trojan", "Classic Trojan port"),
        PortIntel::new(27374, "SubSeven Trojan", "Classic Trojan port"),
        PortIntel::new(65535, "Overflow Indicator", "Last port usage (anomalous)"),
        PortIntel::new(7777, "Common C2", "Common default for multiple reverse shells"),
        PortIntel::new(9999, "Common Backdoor", "Generic meterpreter binding port"),
        PortIntel::new(1234, "Common Test / Backdoor", "Generic netcat listener"),
    ]
}

pub fn from_config(ports: &[u16], custom: &[(u16, String)]) -> Vec<PortIntel> {
    let mut intel = suspicious_ports();
    intel.retain(|i| ports.contains(&i.port));
    for (p, desc) in custom {
        intel.push(PortIntel::new(*p, desc, "User defined custom rule"));
    }
    intel
}

/// Check if a remote port is known-suspicious based on default rules.
pub fn port_intel(port: u16) -> Option<PortIntel> {
    suspicious_ports().into_iter().find(|i| i.port == port)
}
