#![allow(dead_code)]

const UART_BASE: u64 = 0x09000000;
/// Data register (read/write)
const UART_DATA_REGISTER: *mut u8 = (UART_BASE + 0x00) as *mut u8;
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

pub unsafe fn write_string(string: &str) {
    for &byte in string.as_bytes() {
        unsafe {
            *UART_DATA_REGISTER = byte;
        }
    }
}

pub unsafe fn write_byte(char: u8) {
    unsafe {
        *UART_DATA_REGISTER = char;
    }
}
