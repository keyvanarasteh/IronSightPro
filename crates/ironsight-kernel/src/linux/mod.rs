#![allow(unused_imports, dead_code)]
use anyhow::Result;
use aya::Ebpf;
use aya::programs::TracePoint;
use std::sync::Arc;
use crate::dispatcher::EventDispatcher;

pub struct EbpfMonitor {
    bpf: Ebpf,
    dispatcher: Arc<EventDispatcher>,
}

impl EbpfMonitor {
    pub async fn new(dispatcher: Arc<EventDispatcher>) -> Result<Self> {
        // In a real application, you'd load the compiled BPF object:
        // let mut bpf = Bpf::load(include_bytes_aligned!("../../bpf/monitor.o"))?;
        let mut bpf = Ebpf::load(&[])?; 

        let prog: &mut TracePoint = bpf.program_mut("monitor_mprotect")
            .ok_or_else(|| anyhow::anyhow!("Program monitor_mprotect not found"))?
            .try_into()?;
        prog.load()?;
        prog.attach("syscalls", "sys_enter_mprotect")?;
        
        Ok(Self { bpf, dispatcher })
    }

    pub async fn run(&self) -> Result<()> {
        // Setup Bpf array and poll events...
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}
