use core::arch::global_asm;

#[repr(C)]
#[derive(Debug)]
struct State {
    x: [u64; 31],
}

global_asm!(r"

");
