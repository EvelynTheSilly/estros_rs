#![allow(dead_code)]

use crate::rng::Rng;
use crate::syncronisation::{GlobalSharedLock, Mutex};
use crate::vectors::cpu_state::State;
use aarch64_paging::linearmap::LinearMap;
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use anyhow::bail;
use anyhow::{Result, anyhow};
use elf::ElfBytes;
use elf::abi::PT_LOAD;
use elf::endian::AnyEndian;
use process::Process;
use threads::SchedulerThread;

mod process;
mod threads;

pub struct ProcessManager<Scheduler>
where
    Scheduler: CpuScheduler,
{
    scheduler: Scheduler,
    processes: BTreeMap<u64, Process>,
}
impl<Scheduler> ProcessManager<Scheduler>
where
    Scheduler: CpuScheduler + [const] CpuSchedulerNew,
{
    pub const fn new() -> Self {
        ProcessManager {
            scheduler: Scheduler::new(),
            processes: BTreeMap::new(),
        }
    }
    pub fn schedule(&mut self) -> Result<SchedulerThread> {
        Scheduler::schedule(self)
    }
    pub fn report_thread_state(&mut self, pid: u64, tid: u64, state: State) -> Result<()> {
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
    ///returns a PID
    pub fn launch_process(&mut self, elf: ElfBytes<AnyEndian>) -> Result<u64> {
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
}

pub trait CpuScheduler: Sized {
    fn schedule(manager: &mut ProcessManager<Self>) -> Result<SchedulerThread>;
}

const trait CpuSchedulerNew: Sized {
    fn new() -> Self;
}

#[derive(Default)]
pub struct StupidScheduler;

impl const CpuSchedulerNew for StupidScheduler {
    fn new() -> Self {
        StupidScheduler
    }
}

impl CpuScheduler for StupidScheduler {
    fn schedule(_manager: &mut ProcessManager<Self>) -> Result<SchedulerThread> {
        bail!("unimplemented");
    }
}

static mut PROCESS_MANAGER: GlobalSharedLock<ProcessManager<StupidScheduler>> =
    GlobalSharedLock::new(ProcessManager::new());
