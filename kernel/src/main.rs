#![no_std]
#![no_main]

#[allow(unused)]
use core::panic::PanicInfo;
core::arch::global_asm!("boot.S");

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _kernel_entry() -> ! {
    loop {}
}
