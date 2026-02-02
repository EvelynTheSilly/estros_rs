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

use core::panic::PanicInfo;

mod drivers;
mod mem;
mod rng;
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
pub extern "C" fn _kernel_entry() -> ! {
    unsafe {
        println!("booting estros...");

        #[cfg(feature = "qemu")]
        drivers::semihosting::shutdown(0);

        panic!("reached end of init function and didnt find proper shutdown driver");
    };
}
