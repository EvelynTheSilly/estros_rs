use crate::{
    rng::Rng,
    scheduler::{CpuScheduler, process::Process, threads::SchedulerThread},
    syncronisation::Mutex,
    vectors::cpu_state::State,
};
use aarch64_paging::linearmap::LinearMap;
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use anyhow::{Result, anyhow, bail};
use elf::{ElfBytes, abi::PT_LOAD, endian::AnyEndian};

/// Quick and Dirty Scheduler
/// not meant to truly be functional, rewrite later
pub struct QDScheduler {
    processes: BTreeMap<u64, Process>,
}

impl QDScheduler {
    pub const fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
        }
    }
}

impl CpuScheduler for QDScheduler {
    fn schedule(&mut self) -> Result<SchedulerThread> {
        bail!("TODO")
    }
    ///returns a PID
    fn launch_process(&mut self, elf: ElfBytes<AnyEndian>) -> Result<u64> {
        let pheaders = elf.segments().ok_or(anyhow!("no valid headers"))?;
        let load_headers = pheaders.iter().filter(|header| header.p_type == PT_LOAD);
        let mut segments = Vec::with_capacity(load_headers.count());
        let mut memmap = LinearMap::new(
            1,
            0,
            0,
            aarch64_paging::paging::TranslationRegime::El1And0,
            aarch64_paging::paging::VaRange::Lower,
        );
        let mut pid = crate::rng::RNG.lock(|rng| rng.rand_u64());
        while !self.processes.contains_key(&pid) {
            pid = crate::rng::RNG.lock(|rng| rng.rand_u64());
        }
        self.processes.insert(
            pid,
            Process {
                segments: segments,
                memory_map: memmap,
                threads: BTreeMap::new(),
            },
        );
        Ok(pid)
    }
    fn report_thread_state(&mut self, pid: u64, tid: u64, state: State) -> Result<()> {
        if let Some(process) = self.processes.get_mut(&pid) {
            if let Some(thread) = process.threads.get_mut(&tid) {
                thread.state = state
            } else {
                bail!("invalid tid");
            }
        } else {
            bail!("invalid pid");
        }
        Ok(())
    }
}
