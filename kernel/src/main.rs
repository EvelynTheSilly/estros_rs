#![no_std]
#![no_main]
#![allow(unused_unsafe)]

use core::panic::PanicInfo;

use alloc::string::String;

mod mem;
mod uart;
extern crate alloc;

core::arch::global_asm!(include_str!("boot.S"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", { _info.message() });
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
