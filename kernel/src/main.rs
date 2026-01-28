#![no_std]
#![no_main]

#[allow(unused)]
use core::panic::PanicInfo;
use cortex_a::{asm, registers::*};
use tock_registers::interfaces::Writeable;
core::arch::global_asm!(include_str!("boot.S"));

mod uart;

//#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        uart::write_string("KERNEL PANIC");
    }
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _kernel_entry() -> ! {
    unsafe {
        uart::println!("test");
        uart::write_string(
            "rust? memory safety? speed? - you want it? it's yours my friend, as long as you have enough patience\n",
        );
        uart::write_string(
            "\nsorry link, i can't give ease of use, come back when youre a little - mmmm - smarter\n",
        );
        loop {}
    };
}
