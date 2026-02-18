#![allow(unused)]

use crate::vectors::cpu_state;
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use anyhow::{Result, bail};

pub struct ProcessManager<Scheduler>
where
    Scheduler: CpuScheduler,
{
    scheduler: Scheduler,
    processes: BTreeMap<u64, Process>,
}
impl<Scheduler> ProcessManager<Scheduler>
where
    Scheduler: CpuScheduler,
{
    pub fn schedule(self: &mut Self) -> Result<SchedulerThread> {
        Scheduler::schedule(self)
    }
    pub fn store_thread(self: &mut Self) {}
}

pub struct Process {
    threads: Vec<SchedulerThread>,
}

#[derive(Clone)]
pub struct SchedulerThread {
    state: cpu_state::State,
}

pub struct StupidScheduler;

pub trait CpuScheduler: Sized {
    fn schedule(manager: &mut ProcessManager<Self>) -> Result<SchedulerThread>;
}

impl CpuScheduler for StupidScheduler {
    fn schedule(manager: &mut ProcessManager<Self>) -> Result<SchedulerThread> {
        Ok(manager
            .processes
            .first_key_value()
            .unwrap_or(return Err(anyhow::anyhow!("no processes")))
            .1
            .threads
            .first()
            .unwrap_or(return Err(anyhow::anyhow!("first process has no threads")))
            .clone())
    }
}
