#![no_std]
#![no_main]
#![feature(macro_metavar_expr_concat)]
#![feature(const_convert)]
#![feature(likely_unlikely)]
#![feature(const_trait_impl)]
#![feature(const_default)]
#![allow(unused_unsafe)]
#![allow(
    clippy::doc_markdown,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc
)]
#![deny(clippy::float_arithmetic)]
#![deny(clippy::float_cmp)]
#![deny(clippy::float_cmp_const)]
#![deny(clippy::float_equality_without_abs)]
#![warn(clippy::missing_const_for_fn)]

use crate::{
    multiprocessor::mp_init,
    scheduler::{CpuScheduler, PROCESS_MANAGER},
    syncronisation::Mutex,
    vectors::cpu_state::State,
};
use aarch64_cpu::asm::wfi;
use core::{arch::asm, panic::PanicInfo};
use elf::{ElfBytes, endian::AnyEndian};
use limine::{
    BaseRevision,
    request::{HhdmRequest, RequestsEndMarker, RequestsStartMarker, StackSizeRequest},
};

mod boot;
mod drivers;
mod dtb;
mod irqs;
mod mem;
mod multiprocessor;
mod rng;
mod scheduler;
mod syncronisation;
mod uart;
mod vectors;
extern crate alloc;

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static STACK: StackSizeRequest = StackSizeRequest::new().with_size(0x100000);

#[used]
#[unsafe(link_section = ".requests")]
static HDDM: HhdmRequest = HhdmRequest::new();

/// Define the stand and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", { info.message() });
    loop {
        wfi();
    }
}

#[unsafe(no_mangle)]
#[allow(unreachable_code)]
pub extern "C" fn kernel_init() {
    unsafe {
        println!("booting estros...");

        mp_init().expect("multiprocessing failed to initialise");

        #[cfg(any())]
        {
            use crate::mem::mmu::NORMAL_CACHEABLE;
            use crate::mem::paging::ArbitraryTranslation;
            use crate::mem::paging::kernel_virtual_to_physical;
            use aarch64_paging::Mapping;
            use aarch64_paging::descriptor::PhysicalAddress;
            use aarch64_paging::paging::PAGE_SIZE;
            use aarch64_paging::paging::{Constraints, MemoryRegion};
            use alloc::alloc::alloc;
            use core::alloc::Layout;

            // debug printing an empty mapping
            let mut memmap = Mapping::new(
                ArbitraryTranslation,
                0,
                0,
                aarch64_paging::paging::TranslationRegime::El1And0,
                aarch64_paging::paging::VaRange::Lower,
            );
            let layout = Layout::from_size_align(10, PAGE_SIZE).unwrap();
            let allocation = alloc(layout);
            memmap
                .map_range(
                    &MemoryRegion::new(0x4000 as usize, 0x8000 as usize),
                    PhysicalAddress(kernel_virtual_to_physical(allocation) as usize),
                    NORMAL_CACHEABLE,
                    Constraints::empty(),
                )
                .unwrap();
            println!("{:?}", memmap);
        }
        #[cfg(all())]
        {
            println!("loading init...");
            let init = include_bytes!("../../build/init.elf");
            let init_elf = ElfBytes::<AnyEndian>::minimal_parse(init).expect("INVALID INIT FILE");
            println!("launching process");
            let init_pid = PROCESS_MANAGER
                .lock(|manager| manager.launch_process(init_elf))
                .expect("failed to launch init");
            println!("launched pid {}", init_pid);
        }
    };
}

extern "C" fn get_init_process(initial_thread_state: *mut State) {
    unsafe {
        let thread = PROCESS_MANAGER.lock(|manager| manager.schedule().unwrap());
        *initial_thread_state = thread.state;
        asm!("    tlbi vmalle1");
        asm!("    dsb sy");
        asm!("    isb");
    }
    println!("loaded init");
}
