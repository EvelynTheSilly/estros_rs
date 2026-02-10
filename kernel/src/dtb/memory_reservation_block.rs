use alloc::vec;
use alloc::vec::Vec;

use crate::println;

#[derive(Debug)]
pub struct MemoryReservationBlock {
    entries: Vec<BlockEntry>,
}

#[derive(Debug)]
#[repr(C)]
struct BlockEntry {
    address: u64,
    size: u64,
}

impl MemoryReservationBlock {
    pub fn new(base: *mut u64) -> Self {
        let mut entries = vec![];
        let mut counter = base;
        unsafe {
            loop {
                let entry = BlockEntry {
                    address: u64::from_be(*(counter.byte_add(0))),
                    size: u64::from_be(*(counter.byte_add(8))),
                };
                if entry.address == 0 && entry.size == 0 {
                    println!("reached the end of entries block");
                    break;
                }
                entries.push(entry);
            }
        }
        MemoryReservationBlock { entries: entries }
    }
}
