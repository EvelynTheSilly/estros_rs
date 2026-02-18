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

use aarch64_paging::paging::MemoryRegion;
use alloc::vec;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use elf::{endian::AnyEndian, segment::ProgramHeader};

mod drivers;
mod dtb;
mod mem;
mod rng;
mod scheduler;
mod syncronisation;
mod uart;
mod vectors;
extern crate alloc;

core::arch::global_asm!(include_str!("boot.S"));

#[panic_handler]
#[allow(unreachable_code)] // rustc complains code isnt reachable when it very much is when qemu isnt enabled
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", { info.message() });
    #[cfg(feature = "qemu")]
    drivers::semihosting::shutdown(1);
    loop {}
}

#[unsafe(no_mangle)]
#[allow(unreachable_code)] // rustc complains code isnt reachable when it very much is when qemu isnt enabled
pub extern "C" fn _kernel_entry(_dtb_addr: *mut u64) -> ! {
    unsafe {
        println!("booting estros...");

        println!("initialising mmu");
        // apparently we need to keep hold of it?
        let _map = mem::mmu::init_mmu(vec![&MemoryRegion::new(0x0900_0000, 0x0900_1000)]);

        println!("loading init process elf");
        let init = include_bytes!("../../build/init.elf");
        let init_elf = elf::ElfBytes::<AnyEndian>::minimal_parse(init).unwrap();
        println!("init elf {} bytes in size", init.len());
        let headers: Vec<ProgramHeader> = init_elf
            .segments() // actually gets the headers
            .expect("init elf should have segments")
            .iter()
            .filter(|segment_header| segment_header.p_type == elf::abi::PT_LOAD) // filter to only the ones that should be loaded
            .collect();
        println!("{} load segments found", headers.len());
        headers
            .iter()
            .for_each(|header| println!("{:x?} all segment data", header));
        #[cfg(feature = "qemu")]
        drivers::semihosting::shutdown(0);

        panic!("reached end of init function and didnt find proper shutdown driver");
    };
}
