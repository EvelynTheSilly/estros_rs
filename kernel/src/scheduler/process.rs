use crate::scheduler::threads::SchedulerThread;
use aarch64_paging::linearmap::LinearMap;
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use elf::segment::ProgramHeader;

pub struct Process {
    pub segments: Vec<SegmentAllocation>,
    pub memory_map: LinearMap,
    pub threads: BTreeMap<u64, SchedulerThread>,
}
pub struct SegmentAllocation {
    header: ProgramHeader,
    allocation: *mut u8,
}
