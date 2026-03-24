//! # IronSight Kernel
//!
//! Kernel-level real-time monitoring:
//! - Linux: eBPF tracepoints via Aya (mprotect, mmap)
//! - Windows: ETW via ferrisetw (VirtualAlloc, VirtualProtect)
//! - Unified event dispatcher channel

pub mod events;
pub mod dispatcher;

pub use events::{KernelEvent, KernelEventKind, MemoryProtection, SyscallInfo};
pub use dispatcher::{EventDispatcher, EventSubscription};
