//! mmu implementation using aarch64-paging crate
#[allow(dead_code)]
const KILOBYTE: u64 = 1048;
#[allow(dead_code)]
const MEGABYTE: u64 = KILOBYTE * 1048;
#[allow(dead_code)]
const GIGABYTE: u64 = MEGABYTE * 1048;

const NORMAL_CACHEABLE: Attributes =
    Attributes::ATTRIBUTE_INDEX_0.union(Attributes::INNER_SHAREABLE);
use aarch64_paging::{
    descriptor::Attributes,
    idmap::IdMap,
    paging::{MemoryRegion, TranslationRegime},
};
use alloc::vec::Vec;
use core::arch::asm;

use crate::println;

pub fn init_mmu(_device_ranges: Vec<(MemoryRegion, Attributes)>) -> IdMap {
    let mut memmap = IdMap::new(
        1, //
        1, // 48 bit vadresses
        //0,
        TranslationRegime::El1And0,
        //aarch64_paging::paging::VaRange::Lower,
    );
    memmap
        .map_range(
            &MemoryRegion::new(0x40000000, 0x44000000),
            NORMAL_CACHEABLE
                | Attributes::NON_GLOBAL
                | Attributes::VALID
                | Attributes::ACCESSED
                | Attributes::OUTER_SHAREABLE,
        )
        .expect("maping memory failed");
    unsafe {
        memmap.activate();
        asm!(
            "msr tcr_el1, {val}",
            val = in(reg) 0x00000000B5103510 as u64,
        );
        println!("loaded tcr_el1");
        asm!(
            "msr mair_el1, {val}",
            val = in(reg) u64::MAX as u64,
        );
        println!("loaded main_el1");
        let mut sctlr: u64;
        asm!(
            "
            mrs x0, sctlr_el1
            ",
            out("x0") sctlr
        );
        println!("read out sctlr: {:b}", sctlr);
        asm!(
            "
            orr {val}, {val}, #1        // set M
            msr sctlr_el1, {val}
            isb
            ",
            val = inout(reg) sctlr
        );
        println!("read out sctlr: {:b}", sctlr);
    };

    memmap
}
