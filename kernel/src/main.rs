#![no_std]
#![no_main]
#![feature(macro_metavar_expr_concat)]
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

use aarch64_cpu::asm::wfi;
use aarch64_paging::paging::MemoryRegion;
use alloc::vec;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use elf::{endian::AnyEndian, segment::ProgramHeader};
use limine::{
    BaseRevision,
    request::{RequestsEndMarker, RequestsStartMarker, StackSizeRequest},
};

mod drivers;
mod dtb;
mod mem;
mod rng;
mod scheduler;
mod syncronisation;
mod uart;
mod vectors;
extern crate alloc;

#[used]
#[unsafe(link_section = ".requests")]
static STACK: StackSizeRequest = StackSizeRequest::new().with_size(0x100000);

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

/// Define the stand and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

core::arch::global_asm!(include_str!("boot.S"));

#[panic_handler]
#[allow(unreachable_code)] // rustc complains code isnt reachable when it very much is when qemu isnt enabled
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", { info.message() });
    loop {
        wfi();
    }
}

#[unsafe(no_mangle)]
#[allow(unreachable_code)] // rustc complains code isnt reachable when it very much is when qemu isnt enabled
pub extern "C" fn _kernel_entry(_dtb_addr: *mut u64) -> ! {
    unsafe {
        println!("booting estros...");

        let mut sctlr: u64;
        core::arch::asm!(
            "
            mrs x0, sctlr_el1
            ",
            out("x0") sctlr
        );
        println!("{:b}", sctlr);

        panic!("reached end of init function");
    };
}
