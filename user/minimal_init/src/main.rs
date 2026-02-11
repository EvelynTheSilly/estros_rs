#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern "C" fn _init() {
    loop {}
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
