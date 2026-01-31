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

use alloc::string::String;
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
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", { info.message() });
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _kernel_entry() -> ! {
    unsafe {
        println!("test");
        println!(
            "rust? memory safety? speed? - you want it? it's yours my friend, as long as you have enough patience"
        );
        println!(
            "sorry link, i can't give ease of use, come back when youre a little - mmmm - smarter"
        );
        let mut str = String::from("haiii im a string");
        println!("string: {str}");
        str.push_str(" push");
        println!("string: {str}");
        panic!("reached end of init function");
    };
}
