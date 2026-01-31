#![allow(dead_code)]

const UART_BASE: u64 = 0x0900_0000;
/// Data register (read/write)
const UART_DATA_REGISTER: *mut u8 = UART_BASE as *mut u8;
/// Flag register
const UART_FLAG_REGISTER: *mut u8 = (UART_BASE + 0x18) as *mut u8;
/// flag register bitmask (00010000)
const UART_FLAG_RXFE: u8 = 1 << 4;
/// flag register bitmask (00010000)
const UART_FLAG_BUSY: u8 = 1 << 3;
/// Interrupt mask register
const UART_INTERRUPT_MASK_REGISTER: *mut u8 = (UART_BASE + 0x38) as *mut u8;
/// Interrupt clear register
const UART_INTERRUPT_CLEAR_REGISTER: *mut u8 = (UART_BASE + 0x44) as *mut u8;

use crate::syncronisation::GlobalSharedLock;

pub struct Uart;

pub static UART: GlobalSharedLock<Uart> = GlobalSharedLock::new(Uart);

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &byte in s.as_bytes() {
            unsafe {
                core::ptr::write_volatile(UART_DATA_REGISTER, byte);
            }
        }
        Ok(())
    }
}

pub unsafe fn write_string(string: &str) {
    for &byte in string.as_bytes() {
        unsafe {
            core::ptr::write_volatile(UART_DATA_REGISTER, byte);
        }
    }
}

pub unsafe fn write_byte(char: u8) {
    unsafe {
        core::ptr::write_volatile(UART_DATA_REGISTER, char);
    }
}

#[macro_export]
macro_rules! println {
    () => {
        let _ = $crate::print!("\n");
        ()
    };
    ($($arg:tt)*) => {{
        let _ = $crate::print!($($arg)*);
        let _ = $crate::print!("\n");
    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use uart::Uart;
        use uart::UART;
        use core::fmt::Write;
        use $crate::syncronisation::Mutex;
        UART.lock(|mut uart|{
            let _ = Uart::write_fmt(&mut uart,core::format_args!($($arg)*));
        })
    }};
}

#[allow(unused)]
pub(crate) use {print, println};
