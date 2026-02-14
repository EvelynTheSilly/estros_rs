//! mmu implementation using aarch64-paging crate
const KILOBYTE: u64 = 1048;
const MEGABYTE: u64 = KILOBYTE * 1048;
const GIGABYTE: u64 = MEGABYTE * 1048;

use aarch64_paging::{
    descriptor::Attributes,
    idmap::IdMap,
    paging::{MemoryRegion, TranslationRegime},
};
use alloc::vec::Vec;

pub fn init_mmu(_device_ranges: Vec<(MemoryRegion, Attributes)>) {
    let mut idmap = IdMap::new(
        0, //
        0, // 48 bit vadresses
        TranslationRegime::El1And0,
    );
    idmap
        .map_range(
            &MemoryRegion::new(0x40000000, (0x40000000 + GIGABYTE * 8) as usize),
            Attributes::all(),
        )
        .expect("maping memory failed");
    
}
