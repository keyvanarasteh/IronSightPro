//! # IronSight Network
//!
//! Network intelligence — socket-to-PID mapping, listener detection,
//! DNS enrichment, suspicious port detection, network audit.

pub mod audit;
pub mod dns;
pub mod socket_mapper;

pub use audit::NetworkAudit;
pub use dns::{DnsEntry, PortIntel};
pub use socket_mapper::{Protocol, SocketInfo, SocketMapper, TcpState};
