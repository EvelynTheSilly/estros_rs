#![allow(dead_code)]

use crate::scheduler::implementations::GlobalScheduler;
use crate::syncronisation::GlobalSharedLock;
use crate::vectors::cpu_state::State;
use anyhow::Result;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use threads::SchedulerThread;

mod implementations;
mod process;
mod threads;

pub trait CpuScheduler: Sized {
    fn report_thread_state(&mut self, pid: u64, tid: u64, state: State) -> Result<()>;
    fn launch_process(&mut self, elf: ElfBytes<AnyEndian>) -> Result<u64>;
    fn schedule(&mut self) -> Result<SchedulerThread>;
}

pub static PROCESS_MANAGER: GlobalSharedLock<GlobalScheduler> =
    GlobalSharedLock::new(GlobalScheduler::new());
